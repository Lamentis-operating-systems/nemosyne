use nemosyne_core::activation::{
    ActivationChannel, ActivationProfile, CandidateId, EvidenceChannel, InhibitionChannel,
    RankedActivation, UnitInterval, rank_activations,
};

use super::{
    ActivationParameter, ActivationParameters, EvaluationError, EvaluationReport,
    EvaluationScenario, EvaluationSuite, PreferenceEvaluation, PreferenceOutcome,
    ScenarioEvaluation,
};

/// Evaluates one fixed activation parameter set over a numeric scenario suite.
///
/// The function combines situation-independent parameters with each scenario's
/// evidence gates, delegates candidate scoring to `nemosyne-core`, and evaluates
/// only the strict preferences declared by the scenario. No parameters are
/// selected or modified.
///
/// # Errors
///
/// Returns an error when a scenario's gate or candidate schema does not match
/// the parameters, or when the activation kernel rejects a scenario. No partial
/// report is returned.
pub fn evaluate_parameters(
    parameters: &ActivationParameters,
    suite: &EvaluationSuite,
) -> Result<EvaluationReport, EvaluationError> {
    let mut scenario_reports = Vec::with_capacity(suite.scenarios().len());
    let mut preference_count = 0;
    let mut satisfied_count = 0;
    let mut tied_count = 0;
    let mut violated_count = 0;
    let mut passed_scenario_count = 0;
    let mut scenario_accuracy_sum = 0.0;

    for scenario in suite.scenarios() {
        let report = evaluate_scenario(parameters, scenario)?;
        preference_count += report.preference_count();
        satisfied_count += report.satisfied_count();
        tied_count += report.tied_count();
        violated_count += report.violated_count();
        passed_scenario_count += usize::from(report.passed());
        scenario_accuracy_sum += report.accuracy();
        scenario_reports.push(report);
    }

    let macro_accuracy = scenario_accuracy_sum / scenario_reports.len() as f64;
    Ok(EvaluationReport::new(
        scenario_reports.into_boxed_slice(),
        preference_count,
        satisfied_count,
        tied_count,
        violated_count,
        passed_scenario_count,
        macro_accuracy,
    ))
}

fn evaluate_scenario(
    parameters: &ActivationParameters,
    scenario: &EvaluationScenario,
) -> Result<ScenarioEvaluation, EvaluationError> {
    let profile = build_profile(parameters, scenario)?;
    let ranking = rank_activations(&profile, scenario.candidates()).map_err(|source| {
        EvaluationError::Activation {
            scenario_id: scenario.scenario_id(),
            source,
        }
    })?;

    let mut scores_by_candidate = ranking.clone();
    scores_by_candidate.sort_unstable_by_key(RankedActivation::candidate_id);

    let mut preference_reports = Vec::with_capacity(scenario.preferences().len());
    let mut satisfied_count = 0;
    let mut tied_count = 0;
    let mut violated_count = 0;
    for &expectation in scenario.preferences() {
        let preferred_score = find_score(&scores_by_candidate, expectation.preferred());
        let other_score = find_score(&scores_by_candidate, expectation.other());
        let outcome = match preferred_score.get().total_cmp(&other_score.get()) {
            std::cmp::Ordering::Greater => {
                satisfied_count += 1;
                PreferenceOutcome::Satisfied
            }
            std::cmp::Ordering::Equal => {
                tied_count += 1;
                PreferenceOutcome::Tied
            }
            std::cmp::Ordering::Less => {
                violated_count += 1;
                PreferenceOutcome::Violated
            }
        };
        preference_reports.push(PreferenceEvaluation::new(
            expectation,
            preferred_score,
            other_score,
            outcome,
        ));
    }

    Ok(ScenarioEvaluation::new(
        scenario.scenario_id(),
        ranking.into_boxed_slice(),
        preference_reports.into_boxed_slice(),
        satisfied_count,
        tied_count,
        violated_count,
    ))
}

fn build_profile(
    parameters: &ActivationParameters,
    scenario: &EvaluationScenario,
) -> Result<ActivationProfile, EvaluationError> {
    let mut channels = Vec::with_capacity(parameters.parameters().len());
    let mut gate_index = 0;

    for parameter in parameters.parameters() {
        match parameter {
            ActivationParameter::Evidence(parameter) => {
                let Some(gate) = scenario.gates().get(gate_index) else {
                    return Err(EvaluationError::MissingEvidenceGate {
                        scenario_id: scenario.scenario_id(),
                        channel_id: parameter.channel_id(),
                    });
                };
                if gate.channel_id() < parameter.channel_id() {
                    return Err(EvaluationError::UnexpectedEvidenceGate {
                        scenario_id: scenario.scenario_id(),
                        channel_id: gate.channel_id(),
                    });
                }
                if gate.channel_id() > parameter.channel_id() {
                    return Err(EvaluationError::MissingEvidenceGate {
                        scenario_id: scenario.scenario_id(),
                        channel_id: parameter.channel_id(),
                    });
                }
                channels.push(ActivationChannel::Evidence(EvidenceChannel::new(
                    parameter.channel_id(),
                    parameter.weight(),
                    gate.gate(),
                )));
                gate_index += 1;
            }
            ActivationParameter::Inhibition(parameter) => {
                channels.push(ActivationChannel::Inhibition(InhibitionChannel::new(
                    parameter.channel_id(),
                    parameter.strength(),
                )));
            }
        }
    }

    if let Some(gate) = scenario.gates().get(gate_index) {
        return Err(EvaluationError::UnexpectedEvidenceGate {
            scenario_id: scenario.scenario_id(),
            channel_id: gate.channel_id(),
        });
    }

    ActivationProfile::new(channels).map_err(|source| EvaluationError::Activation {
        scenario_id: scenario.scenario_id(),
        source,
    })
}

fn find_score(ranking: &[RankedActivation], candidate_id: CandidateId) -> UnitInterval {
    ranking
        .binary_search_by_key(&candidate_id, RankedActivation::candidate_id)
        .map(|index| ranking[index].score())
        .expect("preference candidates were validated and ranked")
}
