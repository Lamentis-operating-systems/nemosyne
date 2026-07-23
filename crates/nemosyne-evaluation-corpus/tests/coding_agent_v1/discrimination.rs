use nemosyne_core::activation::{ChannelId, UnitInterval};
use nemosyne_evaluation::activation::{
    ActivationParameter, ActivationParameters, EvaluationScenario, EvaluationSuite,
    EvidenceParameter, PreferenceOutcome, ScenarioId, evaluate_parameters,
};
use nemosyne_evaluation_corpus::activation::ActivationEvidenceCorpus;

use super::corpus;

#[test]
fn fixed_constraint_ablation_has_the_documented_local_effect() {
    let corpus = corpus();
    let uniform = corpus
        .references()
        .iter()
        .find(|reference| reference.key() == "uniform_evidence")
        .expect("uniform reference exists")
        .parameters();
    let without_constraint = ActivationParameters::new(
        corpus
            .channels()
            .iter()
            .map(|channel| {
                let weight = if channel.channel_id() == ChannelId::new(50) {
                    0.0
                } else {
                    1.0
                };
                ActivationParameter::from(EvidenceParameter::new(
                    channel.channel_id(),
                    UnitInterval::new(weight).expect("probe weights are valid"),
                ))
            })
            .collect(),
    )
    .expect("constraint-ablation parameters are valid");

    assert_eq!(
        outcome(&corpus, ScenarioId::new(2201), uniform),
        PreferenceOutcome::Satisfied
    );
    assert_eq!(
        outcome(&corpus, ScenarioId::new(2201), &without_constraint),
        PreferenceOutcome::Violated
    );
    assert_eq!(
        outcome(&corpus, ScenarioId::new(2202), uniform),
        PreferenceOutcome::Satisfied
    );
    assert_eq!(
        outcome(&corpus, ScenarioId::new(2202), &without_constraint),
        PreferenceOutcome::Satisfied
    );
}

fn outcome(
    corpus: &ActivationEvidenceCorpus,
    scenario_id: ScenarioId,
    parameters: &ActivationParameters,
) -> PreferenceOutcome {
    let scenario = corpus
        .held_out()
        .suite()
        .scenarios()
        .iter()
        .find(|scenario| scenario.scenario_id() == scenario_id)
        .cloned()
        .expect("declared ablation scenario exists");
    let suite = single_scenario_suite(scenario);
    evaluate_parameters(parameters, &suite)
        .expect("declared ablation scenario must evaluate")
        .scenarios()[0]
        .preferences()[0]
        .outcome()
}

fn single_scenario_suite(scenario: EvaluationScenario) -> EvaluationSuite {
    EvaluationSuite::new(vec![scenario]).expect("one valid scenario forms a suite")
}
