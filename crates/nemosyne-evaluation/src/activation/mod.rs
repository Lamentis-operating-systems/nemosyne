//! Deterministic evaluation of activation parameters.

mod error;
mod evaluate;
mod model;

pub use error::EvaluationError;
pub use evaluate::evaluate_parameters;
pub use model::{
    ActivationParameter, ActivationParameters, EvaluationReport, EvaluationScenario,
    EvaluationSuite, EvidenceGate, EvidenceParameter, ExpectedPreference, InhibitionParameter,
    PreferenceEvaluation, PreferenceOutcome, ScenarioEvaluation, ScenarioId,
};
