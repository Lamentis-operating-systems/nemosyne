//! Reproducible internal synthetic-pilot custody and descriptive reporting.
//!
//! This module is deliberately separate from the formal evidence package. It
//! cannot construct a formal evaluation envelope, outcome admission, product
//! disposition, or release authorization.

mod canonical;
mod error;
mod model;
mod receipt;

pub use error::SyntheticPilotError;
pub use model::{
    FrozenSyntheticPilotV1, GeneratedPilotTaskV1, GenerationAttemptDispositionV1,
    GenerationAttemptId, GenerationAttemptV1, GenerationLogV1, GenerationManifestV1,
    ModelCostPrivacyDisclosureV1, PilotCondition, PilotConditionArtifactV1, PilotConditionSetV1,
    PilotCorpusV1, PilotRoot, PilotRunnerIdentityV1, PilotRunnerManifestV1, PilotScoringManifestV1,
    PilotTaskId,
};
pub use receipt::{
    PilotCellResultV1, PilotConditionSummaryV1, PilotObservationV1, SyntheticPilotDisposition,
    SyntheticPilotReceiptV1,
};
