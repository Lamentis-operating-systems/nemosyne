//! Versioned evidence for activation-parameter evaluation.

mod coding_agent;
mod error;
mod fingerprint;
mod model;
mod revision_one;

pub use coding_agent::coding_agent_v1;
pub use error::{CorpusError, FactReferenceLocation};
pub use model::{
    ActivationEvidenceCorpus, AnchoredValue, CandidateEvidence, CorpusPartition, CorpusRevision,
    CorpusSplit, EvidenceChannelDefinition, EvidenceLevel, FactId, GateEvidence,
    JudgmentApplicability, PreferenceEvidence, ReferenceId, ReferenceParameterSet,
    ScenarioCategory, ScenarioCategoryId, ScenarioEvidence, ScenarioFact, ScenarioProvenance,
    SemanticCaseId,
};
