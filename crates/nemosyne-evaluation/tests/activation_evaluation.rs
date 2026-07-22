//! Integration tests for the public activation-evaluation contract.

use nemosyne_core::activation::{
    ActivationCandidate, CandidateId, ChannelId, ChannelSignal, UnitInterval,
};
use nemosyne_evaluation::activation::{
    ActivationParameter, ActivationParameters, EvaluationScenario, EvaluationSuite, EvidenceGate,
    EvidenceParameter, ExpectedPreference, InhibitionParameter, ScenarioId,
};

#[path = "activation_evaluation/determinism.rs"]
mod determinism;
#[path = "activation_evaluation/metrics.rs"]
mod metrics;
#[path = "activation_evaluation/validation.rs"]
mod validation;

fn unit(value: f64) -> UnitInterval {
    UnitInterval::new(value).expect("fixture value must be in the unit interval")
}

fn evidence(id: u64, weight: f64) -> ActivationParameter {
    EvidenceParameter::new(ChannelId::new(id), unit(weight)).into()
}

fn inhibition(id: u64, strength: f64) -> ActivationParameter {
    InhibitionParameter::new(ChannelId::new(id), unit(strength)).into()
}

fn parameters(entries: Vec<ActivationParameter>) -> ActivationParameters {
    ActivationParameters::new(entries).expect("parameter fixture must be valid")
}

fn gate(id: u64, value: f64) -> EvidenceGate {
    EvidenceGate::new(ChannelId::new(id), unit(value))
}

fn candidate(id: u64, signals: &[(u64, f64)]) -> ActivationCandidate {
    ActivationCandidate::new(
        CandidateId::new(id),
        signals
            .iter()
            .map(|(channel_id, value)| {
                ChannelSignal::new(ChannelId::new(*channel_id), unit(*value))
            })
            .collect(),
    )
    .expect("candidate fixture must be valid")
}

fn preference(preferred: u64, other: u64) -> ExpectedPreference {
    ExpectedPreference::new(CandidateId::new(preferred), CandidateId::new(other))
}

fn scenario(
    id: u64,
    gates: Vec<EvidenceGate>,
    candidates: Vec<ActivationCandidate>,
    preferences: Vec<ExpectedPreference>,
) -> EvaluationScenario {
    EvaluationScenario::new(ScenarioId::new(id), gates, candidates, preferences)
        .expect("scenario fixture must be valid")
}

fn suite(scenarios: Vec<EvaluationScenario>) -> EvaluationSuite {
    EvaluationSuite::new(scenarios).expect("suite fixture must be valid")
}

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1e-12;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}
