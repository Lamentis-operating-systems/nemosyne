//! Deterministic evaluation of activation parameters.

mod error;
mod evaluate;
mod input;
mod preference_graph;
mod report;

pub use error::EvaluationError;
pub use evaluate::evaluate_parameters;
pub use input::{
    ActivationParameter, ActivationParameters, EvaluationScenario, EvaluationSuite, EvidenceGate,
    EvidenceParameter, ExpectedPreference, InhibitionParameter, ScenarioId,
};
pub use report::{EvaluationReport, PreferenceEvaluation, PreferenceOutcome, ScenarioEvaluation};
