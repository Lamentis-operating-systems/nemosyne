use crate::{
    AlternativeShareLabel, ArtifactCheck, BoundaryFixtureCatalogError, BoundaryFixtureCatalogV1,
    BoundaryFixtureKind, BoundaryFixtureV1, BoundaryObservationV1, BoundaryViolation,
    ExactSlotFixtureV1, FixtureCapability, FixtureObligation, FixturePlanRole, FixtureTruth,
    FixtureTruthOwner, RenderedClaimKind, SourceFixtureV1,
};

/// Builds the complete checked BND-01 fixture catalog.
///
/// The catalog contains exactly one positive fixture and one targeted
/// counterexample for every F1 through F17 obligation. It does not construct an
/// EVD-02 result, a TGT-00 envelope, or a product runtime artifact.
pub fn bnd_01_fixture_catalog_v1() -> Result<BoundaryFixtureCatalogV1, BoundaryFixtureCatalogError>
{
    BoundaryFixtureCatalogV1::new(vec![
        fixture(
            "f1-prompt-preserved",
            FixtureObligation::F1,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Prompt {
                original: b"fix the parser".to_vec(),
                compiled: b"<attention>inspect framing</attention>\nfix the parser".to_vec(),
            },
            None,
        )?,
        fixture(
            "f1-prompt-normalized",
            FixtureObligation::F1,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Prompt {
                original: b"fix  the parser\n".to_vec(),
                compiled: b"<attention>inspect framing</attention>\nfix the parser\n".to_vec(),
            },
            Some(BoundaryViolation::PromptNotPreserved),
        )?,
        fixture(
            "f2-authorized-before-relevance",
            FixtureObligation::F2,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Authorization {
                authorization_before_relevance: true,
                selected_sources: vec![
                    SourceFixtureV1::new(1, true),
                    SourceFixtureV1::new(2, true),
                ],
            },
            None,
        )?,
        fixture(
            "f2-unauthorized-source-selected",
            FixtureObligation::F2,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Authorization {
                authorization_before_relevance: false,
                selected_sources: vec![
                    SourceFixtureV1::new(1, true),
                    SourceFixtureV1::new(99, false),
                ],
            },
            Some(BoundaryViolation::AuthorizationOrderOrMembership),
        )?,
        fixture(
            "f3-single-pinned-snapshot",
            FixtureObligation::F3,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Snapshot {
                pinned_revision: 7,
                observed_revisions: vec![7, 7, 7],
                exact_slots: vec![exact_slot(1, 1, 41)],
            },
            None,
        )?,
        fixture(
            "f3-mixed-snapshot-revisions",
            FixtureObligation::F3,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Snapshot {
                pinned_revision: 7,
                observed_revisions: vec![7, 8],
                exact_slots: vec![exact_slot(1, 1, 41)],
            },
            Some(BoundaryViolation::SnapshotOrExactSlotMismatch),
        )?,
        fixture(
            "f4-read-only-compile-capabilities",
            FixtureObligation::F4,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::CompileCapabilities {
                capabilities: vec![
                    FixtureCapability::ReadAuthorizedMemory,
                    FixtureCapability::ContentFreeCoordinationWrite,
                ],
            },
            None,
        )?,
        fixture(
            "f4-semantic-write-capability",
            FixtureObligation::F4,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::CompileCapabilities {
                capabilities: vec![
                    FixtureCapability::ReadAuthorizedMemory,
                    FixtureCapability::SemanticStateWrite,
                ],
            },
            Some(BoundaryViolation::CompileCapabilityEscalation),
        )?,
        fixture(
            "f5-focus-authority-lowered",
            FixtureObligation::F5,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Authority {
                role: FixturePlanRole::Focus,
                emitted_ceiling: 2,
                support_ceilings: vec![2, 3],
            },
            None,
        )?,
        fixture(
            "f5-plan-authority-amplified",
            FixtureObligation::F5,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Authority {
                role: FixturePlanRole::Expectation,
                emitted_ceiling: 3,
                support_ceilings: vec![2, 3],
            },
            Some(BoundaryViolation::AuthorityAmplification),
        )?,
        fixture(
            "f6-assertions-have-plan-support",
            FixtureObligation::F6,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Provenance {
                planned_items: vec![10, 20],
                assertion_bindings: vec![10, 20],
            },
            None,
        )?,
        fixture(
            "f6-unknown-assertion-support",
            FixtureObligation::F6,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Provenance {
                planned_items: vec![10],
                assertion_bindings: vec![10, 99],
            },
            Some(BoundaryViolation::ProvenanceIncomplete),
        )?,
        fixture(
            "f7-complete-output-in-budget",
            FixtureObligation::F7,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Budget {
                budget: 100,
                complete_cost: 100,
                mandatory_content: true,
                returned_empty: false,
                truncated: false,
            },
            None,
        )?,
        fixture(
            "f7-truncated-over-budget-output",
            FixtureObligation::F7,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Budget {
                budget: 100,
                complete_cost: 101,
                mandatory_content: true,
                returned_empty: false,
                truncated: true,
            },
            Some(BoundaryViolation::BudgetUnsafe),
        )?,
        fixture(
            "f8-bounded-monotonic-activation",
            FixtureObligation::F8,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Activation {
                evidence_before: 300,
                evidence_after: 500,
                inhibition_before: 200,
                inhibition_after: 100,
                activation_before: 400,
                activation_after: 600,
            },
            None,
        )?,
        fixture(
            "f8-positive-evidence-lowers-activation",
            FixtureObligation::F8,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Activation {
                evidence_before: 300,
                evidence_after: 500,
                inhibition_before: 200,
                inhibition_after: 200,
                activation_before: 600,
                activation_after: 500,
            },
            Some(BoundaryViolation::ActivationInvariant),
        )?,
        fixture(
            "f9-buffered-validated-result",
            FixtureObligation::F9,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::AtomicResult {
                succeeded: true,
                validated: true,
                complete_bytes: 128,
                delivered_bytes: 128,
            },
            None,
        )?,
        fixture(
            "f9-error-exposes-prefix",
            FixtureObligation::F9,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::AtomicResult {
                succeeded: false,
                validated: false,
                complete_bytes: 128,
                delivered_bytes: 16,
            },
            Some(BoundaryViolation::NonAtomicSemanticResult),
        )?,
        fixture(
            "f10-one-owner-per-truth",
            FixtureObligation::F10,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::DerivationOwners {
                owners: vec![
                    FixtureTruthOwner::new(FixtureTruth::PromptBytes, 1),
                    FixtureTruthOwner::new(FixtureTruth::ActivationRanking, 2),
                    FixtureTruthOwner::new(FixtureTruth::PlanMeaning, 3),
                ],
            },
            None,
        )?,
        fixture(
            "f10-plan-meaning-has-two-owners",
            FixtureObligation::F10,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::DerivationOwners {
                owners: vec![
                    FixtureTruthOwner::new(FixtureTruth::PromptBytes, 1),
                    FixtureTruthOwner::new(FixtureTruth::ActivationRanking, 2),
                    FixtureTruthOwner::new(FixtureTruth::PlanMeaning, 3),
                    FixtureTruthOwner::new(FixtureTruth::PlanMeaning, 4),
                ],
            },
            Some(BoundaryViolation::MultipleDerivationPaths),
        )?,
        fixture(
            "f11-authenticate-before-identity",
            FixtureObligation::F11,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::ArtifactChecks {
                checks: vec![
                    ArtifactCheck::AuthenticateManifest,
                    ArtifactCheck::VerifyArtifactIdentity,
                ],
            },
            None,
        )?,
        fixture(
            "f11-identity-before-authenticity",
            FixtureObligation::F11,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::ArtifactChecks {
                checks: vec![
                    ArtifactCheck::VerifyArtifactIdentity,
                    ArtifactCheck::AuthenticateManifest,
                ],
            },
            Some(BoundaryViolation::ArtifactTrustOrder),
        )?,
        fixture(
            "f12-shared-set-and-exact-slot",
            FixtureObligation::F12,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::SharedSet {
                focus_witness: 51,
                expectation_witness: 51,
                planner_witness: 51,
                focus_lineage: 71,
                expectation_lineage: 71,
                plan_lineage: 71,
                exact_slots: vec![exact_slot(7, 3, 103)],
            },
            None,
        )?,
        fixture(
            "f12-rebound-exact-slot",
            FixtureObligation::F12,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::SharedSet {
                focus_witness: 51,
                expectation_witness: 51,
                planner_witness: 51,
                focus_lineage: 71,
                expectation_lineage: 71,
                plan_lineage: 71,
                exact_slots: vec![ExactSlotFixtureV1::new(7, 3, 103, 8, 103)],
            },
            Some(BoundaryViolation::SharedSetOrExactSlotMismatch),
        )?,
        fixture(
            "f13-duplicate-does-not-amplify",
            FixtureObligation::F13,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::DependencyBudget {
                support_before: 700,
                support_after_duplicate: 700,
                same_dependency_group: true,
            },
            None,
        )?,
        fixture(
            "f13-duplicate-amplifies-support",
            FixtureObligation::F13,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::DependencyBudget {
                support_before: 700,
                support_after_duplicate: 900,
                same_dependency_group: true,
            },
            Some(BoundaryViolation::DuplicateAmplification),
        )?,
        fixture(
            "f14-complete-evidence-share-family",
            FixtureObligation::F14,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::AlternativeFamily {
                denominator: 100,
                shares: vec![60, 30, 10],
                label: AlternativeShareLabel::EvidenceShare,
            },
            None,
        )?,
        fixture(
            "f14-evidence-share-promoted-to-probability",
            FixtureObligation::F14,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::AlternativeFamily {
                denominator: 100,
                shares: vec![60, 30, 10],
                label: AlternativeShareLabel::Probability,
            },
            Some(BoundaryViolation::AlternativeNormalizationOrPromotion),
        )?,
        fixture(
            "f15-alternatives-and-abstention-preserved",
            FixtureObligation::F15,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::AlternativesAndAbstention {
                material_alternatives: 2,
                preserved_alternatives: 2,
                maximum_alternatives: 3,
                has_explicit_abstention: true,
                roles: vec![FixturePlanRole::Expectation, FixturePlanRole::Abstention],
            },
            None,
        )?,
        fixture(
            "f15-action-role-crosses-plan-boundary",
            FixtureObligation::F15,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::AlternativesAndAbstention {
                material_alternatives: 2,
                preserved_alternatives: 1,
                maximum_alternatives: 3,
                has_explicit_abstention: false,
                roles: vec![
                    FixturePlanRole::Expectation,
                    FixturePlanRole::ProhibitedActionSelection,
                ],
            },
            Some(BoundaryViolation::AlternativesAbstentionOrActionSelection),
        )?,
        fixture(
            "f16-offline-immutable-assessment",
            FixtureObligation::F16,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::ObservationAssessment {
                offline_only: true,
                prior_unchanged: true,
                memory_unchanged: true,
                capabilities: vec![],
            },
            None,
        )?,
        fixture(
            "f16-runtime-assessment-mutates-memory",
            FixtureObligation::F16,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::ObservationAssessment {
                offline_only: false,
                prior_unchanged: true,
                memory_unchanged: false,
                capabilities: vec![
                    FixtureCapability::RuntimeObservationAssessment,
                    FixtureCapability::SemanticStateWrite,
                ],
            },
            Some(BoundaryViolation::ObservationAssessmentEscalation),
        )?,
        fixture(
            "f17-renderer-only-lexicalizes-plan",
            FixtureObligation::F17,
            BoundaryFixtureKind::Positive,
            BoundaryObservationV1::Renderer {
                claims: vec![
                    RenderedClaimKind::PlannedProposition,
                    RenderedClaimKind::StructuralSurface,
                ],
                all_assertions_bound: true,
                exact_slots: vec![exact_slot(9, 4, 204)],
                capabilities: vec![],
            },
            None,
        )?,
        fixture(
            "f17-renderer-adds-action-and-source-access",
            FixtureObligation::F17,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Renderer {
                claims: vec![
                    RenderedClaimKind::PlannedProposition,
                    RenderedClaimKind::ActionRecommendation,
                ],
                all_assertions_bound: false,
                exact_slots: vec![ExactSlotFixtureV1::new(9, 4, 204, 9, 205)],
                capabilities: vec![
                    FixtureCapability::ActionSelection,
                    FixtureCapability::MemoryTextRead,
                ],
            },
            Some(BoundaryViolation::RendererSemanticAmplification),
        )?,
    ])
}

fn fixture(
    id: &str,
    obligation: FixtureObligation,
    kind: BoundaryFixtureKind,
    observation: BoundaryObservationV1,
    expected_violation: Option<BoundaryViolation>,
) -> Result<BoundaryFixtureV1, BoundaryFixtureCatalogError> {
    BoundaryFixtureV1::new(id, obligation, kind, observation, expected_violation)
}

const fn exact_slot(owner: u32, slot: u32, content: u32) -> ExactSlotFixtureV1 {
    ExactSlotFixtureV1::new(owner, slot, content, owner, content)
}
