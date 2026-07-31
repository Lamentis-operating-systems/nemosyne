//! Disposable, non-promotional applicability and typed-readiness experiment.
//!
//! This crate cannot produce formal evidence or satisfy a V1 delivery gate.

mod model;

pub use model::{
    ApplicabilityControls, ApplicabilityOutcome, Authority, Candidate, CycleId, Error, FocusItem,
    FocusRole, FocusStructure, Identifier, Priority, QueryScope, ReadinessOutcome, RecordState,
    Replacement, SupportHandle, Validity, evaluate, realize,
};
