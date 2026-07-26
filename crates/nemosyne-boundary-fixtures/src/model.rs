use std::collections::BTreeSet;
use std::fmt;

use nemosyne_evaluation::evidence::ArtifactContentId;
use sha2::{Digest, Sha256};

const CATALOG_IDENTITY_DOMAIN: &[u8] = b"nemosyne.boundary-fixtures.catalog.v1";

/// One formal obligation from F1 through F17.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FixtureObligation {
    /// F1: prompt preservation.
    F1,
    /// F2: authorization before relevance.
    F2,
    /// F3: snapshot consistency.
    F3,
    /// F4: read-only compilation.
    F4,
    /// F5: authority non-amplification.
    F5,
    /// F6: structural provenance completeness.
    F6,
    /// F7: budget safety.
    F7,
    /// F8: activation properties.
    F8,
    /// F9: atomic semantic result.
    F9,
    /// F10: single derivation path.
    F10,
    /// F11: artifact authenticity before artifact identity.
    F11,
    /// F12: shared-set branch coherence.
    F12,
    /// F13: dependency-budget and duplicate non-amplification.
    F13,
    /// F14: alternative-family normalization without probability promotion.
    F14,
    /// F15: bounded alternatives, abstention, and no action selection.
    F15,
    /// F16: offline observation-assessment evidence.
    F16,
    /// F17: renderer semantic non-amplification.
    F17,
}

impl FixtureObligation {
    /// All obligations in canonical order.
    pub const ALL: [Self; 17] = [
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
        Self::F13,
        Self::F14,
        Self::F15,
        Self::F16,
        Self::F17,
    ];

    /// Returns the stable one-based obligation number.
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::F1 => 1,
            Self::F2 => 2,
            Self::F3 => 3,
            Self::F4 => 4,
            Self::F5 => 5,
            Self::F6 => 6,
            Self::F7 => 7,
            Self::F8 => 8,
            Self::F9 => 9,
            Self::F10 => 10,
            Self::F11 => 11,
            Self::F12 => 12,
            Self::F13 => 13,
            Self::F14 => 14,
            Self::F15 => 15,
            Self::F16 => 16,
            Self::F17 => 17,
        }
    }
}

/// Whether a fixture demonstrates admission or a targeted rejection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoundaryFixtureKind {
    /// A conforming boundary observation.
    Positive,
    /// A deliberately nonconforming boundary observation.
    Counterexample,
}

impl BoundaryFixtureKind {
    const fn tag(self) -> u8 {
        match self {
            Self::Positive => 1,
            Self::Counterexample => 2,
        }
    }
}

/// A synthetic source used by authorization fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceFixtureV1 {
    id: u32,
    authorized: bool,
}

impl SourceFixtureV1 {
    /// Creates one synthetic source.
    #[must_use]
    pub const fn new(id: u32, authorized: bool) -> Self {
        Self { id, authorized }
    }

    /// Returns the synthetic source identity.
    #[must_use]
    pub const fn id(self) -> u32 {
        self.id
    }

    /// Reports whether the source is in the authorized view.
    #[must_use]
    pub const fn is_authorized(self) -> bool {
        self.authorized
    }
}

/// A checked exact-slot binding used by snapshot, plan, and renderer fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactSlotFixtureV1 {
    owner: u32,
    slot: u32,
    content: u32,
    bound_owner: u32,
    bound_content: u32,
}

impl ExactSlotFixtureV1 {
    /// Creates a synthetic exact-slot binding and its independently retained join.
    #[must_use]
    pub const fn new(
        owner: u32,
        slot: u32,
        content: u32,
        bound_owner: u32,
        bound_content: u32,
    ) -> Self {
        Self {
            owner,
            slot,
            content,
            bound_owner,
            bound_content,
        }
    }

    const fn is_exact(self) -> bool {
        self.owner == self.bound_owner && self.content == self.bound_content
    }

    fn encode(self, bytes: &mut Vec<u8>) {
        encode_u32(bytes, self.owner);
        encode_u32(bytes, self.slot);
        encode_u32(bytes, self.content);
        encode_u32(bytes, self.bound_owner);
        encode_u32(bytes, self.bound_content);
    }
}

/// A closed plan role represented by boundary fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePlanRole {
    /// Selected focus evidence.
    Focus,
    /// A bounded expectation hypothesis.
    Expectation,
    /// A validator-only structural control.
    Control,
    /// An explicit expectation abstention.
    Abstention,
    /// A prohibited action-selection attempt.
    ProhibitedActionSelection,
}

impl FixturePlanRole {
    const fn tag(self) -> u8 {
        match self {
            Self::Focus => 1,
            Self::Expectation => 2,
            Self::Control => 3,
            Self::Abstention => 4,
            Self::ProhibitedActionSelection => 5,
        }
    }
}

/// Capabilities that a synthetic boundary may attempt to carry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCapability {
    /// Read the already authorized immutable memory view.
    ReadAuthorizedMemory,
    /// Perform content-free compile admission coordination.
    ContentFreeCoordinationWrite,
    /// Mutate semantic product state.
    SemanticStateWrite,
    /// Invoke collision or memory-management recovery.
    ManagementWrite,
    /// Access a network.
    Network,
    /// Expose observation assessment as a runtime endpoint.
    RuntimeObservationAssessment,
    /// Select or recommend an action.
    ActionSelection,
    /// Read memory prose or reconstruct source text inside a renderer.
    MemoryTextRead,
}

impl FixtureCapability {
    const fn tag(self) -> u8 {
        match self {
            Self::ReadAuthorizedMemory => 1,
            Self::ContentFreeCoordinationWrite => 2,
            Self::SemanticStateWrite => 3,
            Self::ManagementWrite => 4,
            Self::Network => 5,
            Self::RuntimeObservationAssessment => 6,
            Self::ActionSelection => 7,
            Self::MemoryTextRead => 8,
        }
    }
}

/// The canonical owner of one independently retained semantic truth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureTruth {
    /// Retained original prompt bytes.
    PromptBytes,
    /// Activation and ranking values.
    ActivationRanking,
    /// Selected structured meaning.
    PlanMeaning,
}

impl FixtureTruth {
    const fn tag(self) -> u8 {
        match self {
            Self::PromptBytes => 1,
            Self::ActivationRanking => 2,
            Self::PlanMeaning => 3,
        }
    }
}

/// One synthetic truth-to-owner assignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureTruthOwner {
    truth: FixtureTruth,
    owner: u32,
}

impl FixtureTruthOwner {
    /// Creates one truth ownership assignment.
    #[must_use]
    pub const fn new(truth: FixtureTruth, owner: u32) -> Self {
        Self { truth, owner }
    }
}

/// The order-sensitive checks permitted for an authenticated artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactCheck {
    /// Authenticate the trusted manifest and its policy scope.
    AuthenticateManifest,
    /// Verify the artifact content against the authenticated manifest.
    VerifyArtifactIdentity,
}

impl ArtifactCheck {
    const fn tag(self) -> u8 {
        match self {
            Self::AuthenticateManifest => 1,
            Self::VerifyArtifactIdentity => 2,
        }
    }
}

/// The semantic label attached to normalized alternative support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlternativeShareLabel {
    /// A within-family evidence share.
    EvidenceShare,
    /// A prohibited probability promotion.
    Probability,
}

impl AlternativeShareLabel {
    const fn tag(self) -> u8 {
        match self {
            Self::EvidenceShare => 1,
            Self::Probability => 2,
        }
    }
}

/// Assertion kinds represented by renderer fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderedClaimKind {
    /// A proposition already selected by the structured plan.
    PlannedProposition,
    /// A structural delimiter carrying no independent meaning.
    StructuralSurface,
    /// A prohibited promoted probability statement.
    Probability,
    /// A prohibited answer to the retained prompt.
    Answer,
    /// A prohibited action recommendation.
    ActionRecommendation,
}

impl RenderedClaimKind {
    const fn tag(self) -> u8 {
        match self {
            Self::PlannedProposition => 1,
            Self::StructuralSurface => 2,
            Self::Probability => 3,
            Self::Answer => 4,
            Self::ActionRecommendation => 5,
        }
    }
}

/// One abstract observation evaluated by the formal-boundary checker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundaryObservationV1 {
    /// F1 prompt and final bytes.
    Prompt {
        /// Original prompt bytes.
        original: Vec<u8>,
        /// Complete compiled bytes.
        compiled: Vec<u8>,
    },
    /// F2 authorization and relevance ordering.
    Authorization {
        /// Whether authorization completed before relevance.
        authorization_before_relevance: bool,
        /// Sources admitted to relevance and selection.
        selected_sources: Vec<SourceFixtureV1>,
    },
    /// F3 pinned revisions and exact-slot joins.
    Snapshot {
        /// The pinned revision.
        pinned_revision: u64,
        /// Revisions observed by all semantic reads.
        observed_revisions: Vec<u64>,
        /// Exact slots resolved inside the pinned snapshot.
        exact_slots: Vec<ExactSlotFixtureV1>,
    },
    /// F4 capabilities reachable from compilation.
    CompileCapabilities {
        /// Reachable capabilities.
        capabilities: Vec<FixtureCapability>,
    },
    /// F5 plan authority and its essential support.
    Authority {
        /// The plan role receiving the derived authority.
        role: FixturePlanRole,
        /// Derived authority ceiling.
        emitted_ceiling: u8,
        /// Every essential support ceiling.
        support_ceilings: Vec<u8>,
    },
    /// F6 structural support and assertion bindings.
    Provenance {
        /// Planned proposition identities.
        planned_items: Vec<u32>,
        /// Assertion-to-plan bindings.
        assertion_bindings: Vec<u32>,
    },
    /// F7 planning and post-substitution budget facts.
    Budget {
        /// Frozen complete-output budget.
        budget: u32,
        /// Complete post-substitution cost.
        complete_cost: u32,
        /// Whether justified mandatory content exists.
        mandatory_content: bool,
        /// Whether a successful result is empty.
        returned_empty: bool,
        /// Whether semantic content was truncated.
        truncated: bool,
    },
    /// F8 bounded monotonic activation observations in basis points.
    Activation {
        /// Positive evidence before the intervention.
        evidence_before: u16,
        /// Positive evidence after the intervention.
        evidence_after: u16,
        /// Positive inhibition before the intervention.
        inhibition_before: u16,
        /// Positive inhibition after the intervention.
        inhibition_after: u16,
        /// Activation before the intervention.
        activation_before: u16,
        /// Activation after the intervention.
        activation_after: u16,
    },
    /// F9 validation and delivery state.
    AtomicResult {
        /// Whether semantic compilation succeeded.
        succeeded: bool,
        /// Whether validation completed before delivery.
        validated: bool,
        /// Complete buffered byte count.
        complete_bytes: u32,
        /// Bytes exposed by the semantic result path.
        delivered_bytes: u32,
    },
    /// F10 assignments of canonical truths to owners.
    DerivationOwners {
        /// Truth ownership assignments.
        owners: Vec<FixtureTruthOwner>,
    },
    /// F11 artifact preflight order.
    ArtifactChecks {
        /// Ordered preflight checks.
        checks: Vec<ArtifactCheck>,
    },
    /// F12 focus, expectation, planner, lineage, and exact-slot joins.
    SharedSet {
        /// Focus branch set-instance witness.
        focus_witness: u64,
        /// Expectation branch set-instance witness.
        expectation_witness: u64,
        /// Independently retained planner witness.
        planner_witness: u64,
        /// Focus lineage identity.
        focus_lineage: u64,
        /// Expectation lineage identity.
        expectation_lineage: u64,
        /// Plan lineage identity.
        plan_lineage: u64,
        /// Exact slots carried by the plan.
        exact_slots: Vec<ExactSlotFixtureV1>,
    },
    /// F13 duplicate and dependency-budget result.
    DependencyBudget {
        /// Support before adding a duplicate.
        support_before: u32,
        /// Support after adding the duplicate.
        support_after_duplicate: u32,
        /// Whether both inputs share one dependency group.
        same_dependency_group: bool,
    },
    /// F14 one complete alternative family.
    AlternativeFamily {
        /// Common exact denominator.
        denominator: u32,
        /// Canonical known and unknown shares.
        shares: Vec<u32>,
        /// Semantic label presented to consumers.
        label: AlternativeShareLabel,
    },
    /// F15 bounded alternatives, explicit abstention, and plan roles.
    AlternativesAndAbstention {
        /// Count of material transition alternatives.
        material_alternatives: u16,
        /// Count preserved in the expectation result.
        preserved_alternatives: u16,
        /// Configured finite alternative limit.
        maximum_alternatives: u16,
        /// Whether a separate unsupported frame explicitly abstained.
        has_explicit_abstention: bool,
        /// Roles emitted across the positive and abstaining frames.
        roles: Vec<FixturePlanRole>,
    },
    /// F16 immutable offline assessment boundary.
    ObservationAssessment {
        /// Whether assessment runs only in the offline harness.
        offline_only: bool,
        /// Whether the prior fixture remains byte-identical.
        prior_unchanged: bool,
        /// Whether persistent memory remains unchanged.
        memory_unchanged: bool,
        /// Capabilities reachable from the assessment.
        capabilities: Vec<FixtureCapability>,
    },
    /// F17 renderer claims, exact slots, and reachable capabilities.
    Renderer {
        /// Rendered claim kinds.
        claims: Vec<RenderedClaimKind>,
        /// Whether every assertion maps to a selected plan proposition.
        all_assertions_bound: bool,
        /// Exact slots used by the renderer.
        exact_slots: Vec<ExactSlotFixtureV1>,
        /// Capabilities reachable by the renderer.
        capabilities: Vec<FixtureCapability>,
    },
}

/// A closed reason why a fixture violates its named formal boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryViolation {
    /// The observation kind does not match the named obligation.
    ObservationKindMismatch,
    /// F1 prompt bytes are not the exact final suffix.
    PromptNotPreserved,
    /// F2 relevance saw a source before successful authorization.
    AuthorizationOrderOrMembership,
    /// F3 reads or exact slots do not belong to one pinned snapshot.
    SnapshotOrExactSlotMismatch,
    /// F4 exposes a prohibited compile capability.
    CompileCapabilityEscalation,
    /// F5 emitted authority exceeds its support or uses a prohibited role.
    AuthorityAmplification,
    /// F6 a planned or rendered assertion lacks structural support.
    ProvenanceIncomplete,
    /// F7 output is over budget, truncated, or falsely empty.
    BudgetUnsafe,
    /// F8 activation is out of bounds or violates the tested monotonic relation.
    ActivationInvariant,
    /// F9 a semantic error exposes bytes or success bypasses complete validation.
    NonAtomicSemanticResult,
    /// F10 one canonical truth has multiple owners or is unowned.
    MultipleDerivationPaths,
    /// F11 artifact identity is checked before manifest authenticity.
    ArtifactTrustOrder,
    /// F12 branch witnesses, lineage, or exact slots disagree.
    SharedSetOrExactSlotMismatch,
    /// F13 a duplicate amplifies one dependency budget.
    DuplicateAmplification,
    /// F14 shares are incomplete or promoted to probability.
    AlternativeNormalizationOrPromotion,
    /// F15 alternatives are dropped, bounds exceeded, abstention omitted, or action selected.
    AlternativesAbstentionOrActionSelection,
    /// F16 assessment mutates evidence or enters the runtime capability graph.
    ObservationAssessmentEscalation,
    /// F17 the renderer adds semantics or reaches prohibited capabilities.
    RendererSemanticAmplification,
}

/// One named positive or counterexample boundary fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryFixtureV1 {
    id: String,
    obligation: FixtureObligation,
    kind: BoundaryFixtureKind,
    observation: BoundaryObservationV1,
    expected_violation: Option<BoundaryViolation>,
}

impl BoundaryFixtureV1 {
    /// Creates one fixture definition.
    pub fn new(
        id: impl Into<String>,
        obligation: FixtureObligation,
        kind: BoundaryFixtureKind,
        observation: BoundaryObservationV1,
        expected_violation: Option<BoundaryViolation>,
    ) -> Result<Self, BoundaryFixtureCatalogError> {
        let id = id.into();
        let label_bytes = id.as_bytes();
        if label_bytes.is_empty()
            || !label_bytes
                .iter()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
            || !label_bytes
                .first()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            || !label_bytes
                .last()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            || label_bytes.windows(2).any(|pair| pair == b"--")
        {
            return Err(BoundaryFixtureCatalogError::InvalidFixtureId);
        }
        match (kind, expected_violation) {
            (BoundaryFixtureKind::Positive, None)
            | (BoundaryFixtureKind::Counterexample, Some(_)) => {}
            _ => return Err(BoundaryFixtureCatalogError::InvalidExpectedOutcome),
        }
        Ok(Self {
            id,
            obligation,
            kind,
            observation,
            expected_violation,
        })
    }

    /// Returns the stable fixture label.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the covered formal obligation.
    #[must_use]
    pub const fn obligation(&self) -> FixtureObligation {
        self.obligation
    }

    /// Returns whether this is a positive fixture or counterexample.
    #[must_use]
    pub const fn kind(&self) -> BoundaryFixtureKind {
        self.kind
    }

    /// Returns the synthetic observation.
    #[must_use]
    pub const fn observation(&self) -> &BoundaryObservationV1 {
        &self.observation
    }

    /// Returns the counterexample's expected rejection reason.
    #[must_use]
    pub const fn expected_violation(&self) -> Option<BoundaryViolation> {
        self.expected_violation
    }

    /// Executes the named formal-boundary check.
    pub fn evaluate(&self) -> Result<(), BoundaryViolation> {
        validate_observation(self.obligation, &self.observation)
    }

    fn encode(&self, bytes: &mut Vec<u8>) {
        encode_bytes(bytes, self.id.as_bytes());
        bytes.push(self.obligation.number());
        bytes.push(self.kind.tag());
        bytes.push(self.expected_violation.map_or(0, violation_tag));
        self.observation.encode(bytes);
    }
}

/// A complete, checked, content-identified F1 through F17 fixture catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryFixtureCatalogV1 {
    fixtures: Box<[BoundaryFixtureV1]>,
    content_id: ArtifactContentId,
}

impl BoundaryFixtureCatalogV1 {
    /// Validates complete positive/counterexample coverage and derives its identity.
    pub fn new(mut fixtures: Vec<BoundaryFixtureV1>) -> Result<Self, BoundaryFixtureCatalogError> {
        fixtures.sort_by(|left, right| {
            (left.obligation, left.kind, left.id.as_str()).cmp(&(
                right.obligation,
                right.kind,
                right.id.as_str(),
            ))
        });

        let mut ids = BTreeSet::new();
        for fixture in &fixtures {
            if !ids.insert(fixture.id.clone()) {
                return Err(BoundaryFixtureCatalogError::DuplicateFixtureId);
            }
            match (fixture.kind, fixture.evaluate(), fixture.expected_violation) {
                (BoundaryFixtureKind::Positive, Ok(()), None) => {}
                (BoundaryFixtureKind::Counterexample, Err(actual), Some(expected))
                    if actual == expected => {}
                _ => {
                    return Err(BoundaryFixtureCatalogError::UnexpectedFixtureOutcome {
                        obligation: fixture.obligation,
                        kind: fixture.kind,
                    });
                }
            }
        }

        for obligation in FixtureObligation::ALL {
            for kind in [
                BoundaryFixtureKind::Positive,
                BoundaryFixtureKind::Counterexample,
            ] {
                let count = fixtures
                    .iter()
                    .filter(|fixture| fixture.obligation == obligation && fixture.kind == kind)
                    .count();
                match count {
                    1 => {}
                    0 => {
                        return Err(BoundaryFixtureCatalogError::MissingCoverage {
                            obligation,
                            kind,
                        });
                    }
                    _ => {
                        return Err(BoundaryFixtureCatalogError::DuplicateCoverage {
                            obligation,
                            kind,
                        });
                    }
                }
            }
        }

        let mut canonical = Vec::new();
        canonical.extend_from_slice(CATALOG_IDENTITY_DOMAIN);
        encode_len(&mut canonical, fixtures.len());
        for fixture in &fixtures {
            fixture.encode(&mut canonical);
        }
        let digest = Sha256::digest(&canonical);
        let content_id = ArtifactContentId::from_bytes(digest.into());
        Ok(Self {
            fixtures: fixtures.into_boxed_slice(),
            content_id,
        })
    }

    /// Returns all 34 fixtures in canonical obligation/kind/label order.
    #[must_use]
    pub fn fixtures(&self) -> &[BoundaryFixtureV1] {
        &self.fixtures
    }

    /// Returns the EVD-01-compatible content identity of the complete catalog.
    #[must_use]
    pub const fn content_id(&self) -> ArtifactContentId {
        self.content_id
    }
}

/// A catalog construction or completeness failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryFixtureCatalogError {
    /// A fixture label is empty or not lowercase kebab case.
    InvalidFixtureId,
    /// Positive and counterexample expected outcomes are inconsistent.
    InvalidExpectedOutcome,
    /// Two fixtures use the same label.
    DuplicateFixtureId,
    /// One required positive or counterexample case is absent.
    MissingCoverage {
        /// Missing obligation.
        obligation: FixtureObligation,
        /// Missing fixture kind.
        kind: BoundaryFixtureKind,
    },
    /// More than one case claims the same obligation and kind.
    DuplicateCoverage {
        /// Duplicated obligation.
        obligation: FixtureObligation,
        /// Duplicated fixture kind.
        kind: BoundaryFixtureKind,
    },
    /// A fixture did not produce its declared outcome.
    UnexpectedFixtureOutcome {
        /// Fixture obligation.
        obligation: FixtureObligation,
        /// Fixture kind.
        kind: BoundaryFixtureKind,
    },
}

impl fmt::Display for BoundaryFixtureCatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for BoundaryFixtureCatalogError {}

fn validate_observation(
    obligation: FixtureObligation,
    observation: &BoundaryObservationV1,
) -> Result<(), BoundaryViolation> {
    match (obligation, observation) {
        (FixtureObligation::F1, BoundaryObservationV1::Prompt { original, compiled }) => {
            if compiled.ends_with(original) {
                Ok(())
            } else {
                Err(BoundaryViolation::PromptNotPreserved)
            }
        }
        (
            FixtureObligation::F2,
            BoundaryObservationV1::Authorization {
                authorization_before_relevance,
                selected_sources,
            },
        ) => {
            if *authorization_before_relevance
                && selected_sources.iter().all(|source| source.authorized)
            {
                Ok(())
            } else {
                Err(BoundaryViolation::AuthorizationOrderOrMembership)
            }
        }
        (
            FixtureObligation::F3,
            BoundaryObservationV1::Snapshot {
                pinned_revision,
                observed_revisions,
                exact_slots,
            },
        ) => {
            if !observed_revisions.is_empty()
                && observed_revisions
                    .iter()
                    .all(|revision| revision == pinned_revision)
                && exact_slots.iter().all(|slot| slot.is_exact())
            {
                Ok(())
            } else {
                Err(BoundaryViolation::SnapshotOrExactSlotMismatch)
            }
        }
        (FixtureObligation::F4, BoundaryObservationV1::CompileCapabilities { capabilities }) => {
            if capabilities.iter().all(|capability| {
                matches!(
                    capability,
                    FixtureCapability::ReadAuthorizedMemory
                        | FixtureCapability::ContentFreeCoordinationWrite
                )
            }) {
                Ok(())
            } else {
                Err(BoundaryViolation::CompileCapabilityEscalation)
            }
        }
        (
            FixtureObligation::F5,
            BoundaryObservationV1::Authority {
                role,
                emitted_ceiling,
                support_ceilings,
            },
        ) => {
            if !matches!(role, FixturePlanRole::ProhibitedActionSelection)
                && !support_ceilings.is_empty()
                && support_ceilings
                    .iter()
                    .all(|support| emitted_ceiling <= support)
            {
                Ok(())
            } else {
                Err(BoundaryViolation::AuthorityAmplification)
            }
        }
        (
            FixtureObligation::F6,
            BoundaryObservationV1::Provenance {
                planned_items,
                assertion_bindings,
            },
        ) => {
            if !planned_items.is_empty()
                && !assertion_bindings.is_empty()
                && assertion_bindings
                    .iter()
                    .all(|binding| planned_items.contains(binding))
            {
                Ok(())
            } else {
                Err(BoundaryViolation::ProvenanceIncomplete)
            }
        }
        (
            FixtureObligation::F7,
            BoundaryObservationV1::Budget {
                budget,
                complete_cost,
                mandatory_content,
                returned_empty,
                truncated,
            },
        ) => {
            if complete_cost <= budget && !truncated && !(*mandatory_content && *returned_empty) {
                Ok(())
            } else {
                Err(BoundaryViolation::BudgetUnsafe)
            }
        }
        (
            FixtureObligation::F8,
            BoundaryObservationV1::Activation {
                evidence_before,
                evidence_after,
                inhibition_before,
                inhibition_after,
                activation_before,
                activation_after,
            },
        ) => {
            let bounded = [
                *evidence_before,
                *evidence_after,
                *inhibition_before,
                *inhibition_after,
                *activation_before,
                *activation_after,
            ]
            .into_iter()
            .all(|value| value <= 1_000);
            let evidence_monotone =
                evidence_after < evidence_before || activation_after >= activation_before;
            let inhibition_monotone =
                inhibition_after > inhibition_before || activation_after >= activation_before;
            if bounded && evidence_monotone && inhibition_monotone {
                Ok(())
            } else {
                Err(BoundaryViolation::ActivationInvariant)
            }
        }
        (
            FixtureObligation::F9,
            BoundaryObservationV1::AtomicResult {
                succeeded,
                validated,
                complete_bytes,
                delivered_bytes,
            },
        ) => {
            if (*succeeded && *validated && complete_bytes == delivered_bytes)
                || (!*succeeded && *delivered_bytes == 0)
            {
                Ok(())
            } else {
                Err(BoundaryViolation::NonAtomicSemanticResult)
            }
        }
        (FixtureObligation::F10, BoundaryObservationV1::DerivationOwners { owners }) => {
            let complete = FixtureTruth::all().iter().all(|truth| {
                let matching: BTreeSet<_> = owners
                    .iter()
                    .filter(|owner| owner.truth == *truth)
                    .map(|owner| owner.owner)
                    .collect();
                matching.len() == 1
            });
            if complete {
                Ok(())
            } else {
                Err(BoundaryViolation::MultipleDerivationPaths)
            }
        }
        (FixtureObligation::F11, BoundaryObservationV1::ArtifactChecks { checks }) => {
            if checks
                == &[
                    ArtifactCheck::AuthenticateManifest,
                    ArtifactCheck::VerifyArtifactIdentity,
                ]
            {
                Ok(())
            } else {
                Err(BoundaryViolation::ArtifactTrustOrder)
            }
        }
        (
            FixtureObligation::F12,
            BoundaryObservationV1::SharedSet {
                focus_witness,
                expectation_witness,
                planner_witness,
                focus_lineage,
                expectation_lineage,
                plan_lineage,
                exact_slots,
            },
        ) => {
            if focus_witness == expectation_witness
                && focus_witness == planner_witness
                && focus_lineage == expectation_lineage
                && focus_lineage == plan_lineage
                && exact_slots.iter().all(|slot| slot.is_exact())
            {
                Ok(())
            } else {
                Err(BoundaryViolation::SharedSetOrExactSlotMismatch)
            }
        }
        (
            FixtureObligation::F13,
            BoundaryObservationV1::DependencyBudget {
                support_before,
                support_after_duplicate,
                same_dependency_group,
            },
        ) => {
            if !same_dependency_group || support_before == support_after_duplicate {
                Ok(())
            } else {
                Err(BoundaryViolation::DuplicateAmplification)
            }
        }
        (
            FixtureObligation::F14,
            BoundaryObservationV1::AlternativeFamily {
                denominator,
                shares,
                label,
            },
        ) => {
            let sum = shares
                .iter()
                .try_fold(0_u32, |total, share| total.checked_add(*share));
            if *denominator > 0
                && sum == Some(*denominator)
                && *label == AlternativeShareLabel::EvidenceShare
            {
                Ok(())
            } else {
                Err(BoundaryViolation::AlternativeNormalizationOrPromotion)
            }
        }
        (
            FixtureObligation::F15,
            BoundaryObservationV1::AlternativesAndAbstention {
                material_alternatives,
                preserved_alternatives,
                maximum_alternatives,
                has_explicit_abstention,
                roles,
            },
        ) => {
            if material_alternatives == preserved_alternatives
                && preserved_alternatives <= maximum_alternatives
                && *has_explicit_abstention
                && roles.contains(&FixturePlanRole::Expectation)
                && roles.contains(&FixturePlanRole::Abstention)
                && !roles.contains(&FixturePlanRole::ProhibitedActionSelection)
            {
                Ok(())
            } else {
                Err(BoundaryViolation::AlternativesAbstentionOrActionSelection)
            }
        }
        (
            FixtureObligation::F16,
            BoundaryObservationV1::ObservationAssessment {
                offline_only,
                prior_unchanged,
                memory_unchanged,
                capabilities,
            },
        ) => {
            if *offline_only && *prior_unchanged && *memory_unchanged && capabilities.is_empty() {
                Ok(())
            } else {
                Err(BoundaryViolation::ObservationAssessmentEscalation)
            }
        }
        (
            FixtureObligation::F17,
            BoundaryObservationV1::Renderer {
                claims,
                all_assertions_bound,
                exact_slots,
                capabilities,
            },
        ) => {
            let claims_safe = claims.iter().all(|claim| {
                matches!(
                    claim,
                    RenderedClaimKind::PlannedProposition | RenderedClaimKind::StructuralSurface
                )
            });
            if *all_assertions_bound
                && exact_slots.iter().all(|slot| slot.is_exact())
                && claims_safe
                && capabilities.is_empty()
            {
                Ok(())
            } else {
                Err(BoundaryViolation::RendererSemanticAmplification)
            }
        }
        _ => Err(BoundaryViolation::ObservationKindMismatch),
    }
}

impl FixtureTruth {
    const fn all() -> [Self; 3] {
        [
            Self::PromptBytes,
            Self::ActivationRanking,
            Self::PlanMeaning,
        ]
    }
}

impl BoundaryObservationV1 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            Self::Prompt { original, compiled } => {
                bytes.push(1);
                encode_bytes(bytes, original);
                encode_bytes(bytes, compiled);
            }
            Self::Authorization {
                authorization_before_relevance,
                selected_sources,
            } => {
                bytes.push(2);
                encode_bool(bytes, *authorization_before_relevance);
                encode_len(bytes, selected_sources.len());
                for source in selected_sources {
                    encode_u32(bytes, source.id);
                    encode_bool(bytes, source.authorized);
                }
            }
            Self::Snapshot {
                pinned_revision,
                observed_revisions,
                exact_slots,
            } => {
                bytes.push(3);
                encode_u64(bytes, *pinned_revision);
                encode_len(bytes, observed_revisions.len());
                for revision in observed_revisions {
                    encode_u64(bytes, *revision);
                }
                encode_slots(bytes, exact_slots);
            }
            Self::CompileCapabilities { capabilities } => {
                bytes.push(4);
                encode_capabilities(bytes, capabilities);
            }
            Self::Authority {
                role,
                emitted_ceiling,
                support_ceilings,
            } => {
                bytes.push(5);
                bytes.push(role.tag());
                bytes.push(*emitted_ceiling);
                encode_bytes(bytes, support_ceilings);
            }
            Self::Provenance {
                planned_items,
                assertion_bindings,
            } => {
                bytes.push(6);
                encode_u32_values(bytes, planned_items);
                encode_u32_values(bytes, assertion_bindings);
            }
            Self::Budget {
                budget,
                complete_cost,
                mandatory_content,
                returned_empty,
                truncated,
            } => {
                bytes.push(7);
                encode_u32(bytes, *budget);
                encode_u32(bytes, *complete_cost);
                encode_bool(bytes, *mandatory_content);
                encode_bool(bytes, *returned_empty);
                encode_bool(bytes, *truncated);
            }
            Self::Activation {
                evidence_before,
                evidence_after,
                inhibition_before,
                inhibition_after,
                activation_before,
                activation_after,
            } => {
                bytes.push(8);
                for value in [
                    evidence_before,
                    evidence_after,
                    inhibition_before,
                    inhibition_after,
                    activation_before,
                    activation_after,
                ] {
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
            }
            Self::AtomicResult {
                succeeded,
                validated,
                complete_bytes,
                delivered_bytes,
            } => {
                bytes.push(9);
                encode_bool(bytes, *succeeded);
                encode_bool(bytes, *validated);
                encode_u32(bytes, *complete_bytes);
                encode_u32(bytes, *delivered_bytes);
            }
            Self::DerivationOwners { owners } => {
                bytes.push(10);
                encode_len(bytes, owners.len());
                for owner in owners {
                    bytes.push(owner.truth.tag());
                    encode_u32(bytes, owner.owner);
                }
            }
            Self::ArtifactChecks { checks } => {
                bytes.push(11);
                encode_len(bytes, checks.len());
                bytes.extend(checks.iter().map(|check| check.tag()));
            }
            Self::SharedSet {
                focus_witness,
                expectation_witness,
                planner_witness,
                focus_lineage,
                expectation_lineage,
                plan_lineage,
                exact_slots,
            } => {
                bytes.push(12);
                for value in [
                    focus_witness,
                    expectation_witness,
                    planner_witness,
                    focus_lineage,
                    expectation_lineage,
                    plan_lineage,
                ] {
                    encode_u64(bytes, *value);
                }
                encode_slots(bytes, exact_slots);
            }
            Self::DependencyBudget {
                support_before,
                support_after_duplicate,
                same_dependency_group,
            } => {
                bytes.push(13);
                encode_u32(bytes, *support_before);
                encode_u32(bytes, *support_after_duplicate);
                encode_bool(bytes, *same_dependency_group);
            }
            Self::AlternativeFamily {
                denominator,
                shares,
                label,
            } => {
                bytes.push(14);
                encode_u32(bytes, *denominator);
                encode_u32_values(bytes, shares);
                bytes.push(label.tag());
            }
            Self::AlternativesAndAbstention {
                material_alternatives,
                preserved_alternatives,
                maximum_alternatives,
                has_explicit_abstention,
                roles,
            } => {
                bytes.push(15);
                for value in [
                    material_alternatives,
                    preserved_alternatives,
                    maximum_alternatives,
                ] {
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                encode_bool(bytes, *has_explicit_abstention);
                encode_len(bytes, roles.len());
                bytes.extend(roles.iter().map(|role| role.tag()));
            }
            Self::ObservationAssessment {
                offline_only,
                prior_unchanged,
                memory_unchanged,
                capabilities,
            } => {
                bytes.push(16);
                encode_bool(bytes, *offline_only);
                encode_bool(bytes, *prior_unchanged);
                encode_bool(bytes, *memory_unchanged);
                encode_capabilities(bytes, capabilities);
            }
            Self::Renderer {
                claims,
                all_assertions_bound,
                exact_slots,
                capabilities,
            } => {
                bytes.push(17);
                encode_len(bytes, claims.len());
                bytes.extend(claims.iter().map(|claim| claim.tag()));
                encode_bool(bytes, *all_assertions_bound);
                encode_slots(bytes, exact_slots);
                encode_capabilities(bytes, capabilities);
            }
        }
    }
}

fn violation_tag(violation: BoundaryViolation) -> u8 {
    match violation {
        BoundaryViolation::ObservationKindMismatch => 1,
        BoundaryViolation::PromptNotPreserved => 2,
        BoundaryViolation::AuthorizationOrderOrMembership => 3,
        BoundaryViolation::SnapshotOrExactSlotMismatch => 4,
        BoundaryViolation::CompileCapabilityEscalation => 5,
        BoundaryViolation::AuthorityAmplification => 6,
        BoundaryViolation::ProvenanceIncomplete => 7,
        BoundaryViolation::BudgetUnsafe => 8,
        BoundaryViolation::ActivationInvariant => 9,
        BoundaryViolation::NonAtomicSemanticResult => 10,
        BoundaryViolation::MultipleDerivationPaths => 11,
        BoundaryViolation::ArtifactTrustOrder => 12,
        BoundaryViolation::SharedSetOrExactSlotMismatch => 13,
        BoundaryViolation::DuplicateAmplification => 14,
        BoundaryViolation::AlternativeNormalizationOrPromotion => 15,
        BoundaryViolation::AlternativesAbstentionOrActionSelection => 16,
        BoundaryViolation::ObservationAssessmentEscalation => 17,
        BoundaryViolation::RendererSemanticAmplification => 18,
    }
}

fn encode_slots(bytes: &mut Vec<u8>, slots: &[ExactSlotFixtureV1]) {
    encode_len(bytes, slots.len());
    for slot in slots {
        slot.encode(bytes);
    }
}

fn encode_capabilities(bytes: &mut Vec<u8>, capabilities: &[FixtureCapability]) {
    encode_len(bytes, capabilities.len());
    bytes.extend(capabilities.iter().map(|capability| capability.tag()));
}

fn encode_u32_values(bytes: &mut Vec<u8>, values: &[u32]) {
    encode_len(bytes, values.len());
    for value in values {
        encode_u32(bytes, *value);
    }
}

fn encode_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    encode_len(bytes, value.len());
    bytes.extend_from_slice(value);
}

fn encode_len(bytes: &mut Vec<u8>, length: usize) {
    bytes.extend_from_slice(&(length as u64).to_be_bytes());
}

fn encode_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn encode_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn encode_bool(bytes: &mut Vec<u8>, value: bool) {
    bytes.push(u8::from(value));
}
