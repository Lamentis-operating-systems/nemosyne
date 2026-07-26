//! Executable synthetic fixtures for the Nemosyne F1 through F17 boundaries.
//!
//! This crate is offline verification infrastructure. It contains no product
//! runtime, storage, model, network, management, or outcome-access capability.

mod catalog;
mod model;

pub use catalog::bnd_01_fixture_catalog_v1;
pub use model::{
    AlternativeShareLabel, ArtifactCheck, BoundaryFixtureCatalogError, BoundaryFixtureCatalogV1,
    BoundaryFixtureKind, BoundaryFixtureV1, BoundaryObservationV1, BoundaryViolation,
    ExactSlotFixtureV1, FixtureCapability, FixtureObligation, FixturePlanRole, FixtureTruth,
    FixtureTruthOwner, RenderedClaimKind, SourceFixtureV1,
};
