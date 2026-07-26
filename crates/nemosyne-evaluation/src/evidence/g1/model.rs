use std::collections::BTreeSet;

use crate::evidence::{EvidenceIdentity, canonical::Encoder};

use super::G1EnvelopeError;

const MAX_TASKS: usize = 4_096;
const MAX_EXPOSURES: usize = 12_291;
const MAX_ARTIFACTS: usize = G1ArtifactKind::ALL.len();
const MAX_RUN_ARTIFACTS: usize = G1RunArtifactKind::ALL.len();

/// One of the seven closed G1 experimental conditions.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1Condition {
    /// Original user prompt only.
    Prompt,
    /// Prompt with situation and metadata but no persistent memory.
    Situation,
    /// Situation plus matched irrelevant placebo attention.
    Placebo,
    /// Expert focus plus the neutral expectation carrier.
    Focus,
    /// The same expert focus plus a qualified correct expectation.
    Correct,
    /// The same expert focus plus the deliberately wrong intervention.
    Wrong,
    /// The same expert focus plus explicit expectation abstention.
    Abstain,
}

impl G1Condition {
    pub(super) const ALL: [Self; 7] = [
        Self::Prompt,
        Self::Situation,
        Self::Placebo,
        Self::Focus,
        Self::Correct,
        Self::Wrong,
        Self::Abstain,
    ];

    /// Returns the closed condition domain in canonical order.
    #[must_use]
    pub const fn all() -> &'static [Self; 7] {
        &Self::ALL
    }

    /// Returns the proof-owned stable condition label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Prompt => "g1_prompt",
            Self::Situation => "g1_situation",
            Self::Placebo => "g1_placebo",
            Self::Focus => "g1_focus",
            Self::Correct => "g1_correct",
            Self::Wrong => "g1_wrong",
            Self::Abstain => "g1_abstain",
        }
    }

    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::Prompt => 1,
            Self::Situation => 2,
            Self::Placebo => 3,
            Self::Focus => 4,
            Self::Correct => 5,
            Self::Wrong => 6,
            Self::Abstain => 7,
        }
    }
}

/// One exact authored artifact for a closed G1 condition.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1ConditionArtifactV1 {
    condition: G1Condition,
    identity: EvidenceIdentity,
}

impl G1ConditionArtifactV1 {
    /// Constructs a condition-to-artifact binding.
    pub fn new(
        condition: G1Condition,
        identity: EvidenceIdentity,
    ) -> Result<Self, G1EnvelopeError> {
        ensure_identity(identity)?;
        Ok(Self {
            condition,
            identity,
        })
    }

    /// Returns the condition.
    #[must_use]
    pub const fn condition(self) -> G1Condition {
        self.condition
    }

    /// Returns the exact authored-artifact identity.
    #[must_use]
    pub const fn identity(self) -> EvidenceIdentity {
        self.identity
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.condition.tag());
        encoder.fixed(self.identity.as_bytes());
    }
}

/// Shared exact matching contract for placebo and expert attention.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct G1AttentionMatchingV1 {
    effective_budget_tokens: u32,
    target_attention_tokens: u32,
    repetitions_per_task_condition: u32,
}

impl G1AttentionMatchingV1 {
    /// Constructs positive exact token and repetition bounds.
    pub fn new(
        effective_budget_tokens: u32,
        target_attention_tokens: u32,
        repetitions_per_task_condition: u32,
    ) -> Result<Self, G1EnvelopeError> {
        if effective_budget_tokens == 0
            || target_attention_tokens == 0
            || target_attention_tokens > effective_budget_tokens
            || repetitions_per_task_condition == 0
        {
            return Err(G1EnvelopeError::InvalidAttentionBounds);
        }
        Ok(Self {
            effective_budget_tokens,
            target_attention_tokens,
            repetitions_per_task_condition,
        })
    }

    /// Returns the common effective downstream budget.
    #[must_use]
    pub const fn effective_budget_tokens(self) -> u32 {
        self.effective_budget_tokens
    }

    /// Returns the exact common attention-token count.
    #[must_use]
    pub const fn target_attention_tokens(self) -> u32 {
        self.target_attention_tokens
    }

    /// Returns the frozen repetition count per task-condition cell.
    #[must_use]
    pub const fn repetitions_per_task_condition(self) -> u32 {
        self.repetitions_per_task_condition
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.u32(self.effective_budget_tokens);
        encoder.u32(self.target_attention_tokens);
        encoder.u32(self.repetitions_per_task_condition);
    }
}

/// One context-dependence domain in the G1 population partition.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1Domain {
    /// Resolution requires the frozen context premise.
    ContextDependent,
    /// Claim-bearing negative control that does not require the premise.
    ContextIndependent,
}

impl G1Domain {
    pub(super) const ALL: [Self; 2] = [Self::ContextDependent, Self::ContextIndependent];

    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::ContextDependent => 1,
            Self::ContextIndependent => 2,
        }
    }
}

/// Stable numeric identity of one G1 task.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1TaskId(u64);

impl G1TaskId {
    /// Constructs a task identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Claim-bearing language, task-family, and risk intersection.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1SubgroupV1 {
    language_id: u32,
    task_family_id: u32,
    risk_group_id: u32,
}

impl G1SubgroupV1 {
    /// Constructs one exact subgroup intersection.
    #[must_use]
    pub const fn new(language_id: u32, task_family_id: u32, risk_group_id: u32) -> Self {
        Self {
            language_id,
            task_family_id,
            risk_group_id,
        }
    }

    /// Returns the language identity.
    #[must_use]
    pub const fn language_id(self) -> u32 {
        self.language_id
    }

    /// Returns the task-family identity.
    #[must_use]
    pub const fn task_family_id(self) -> u32 {
        self.task_family_id
    }

    /// Returns the risk-group identity.
    #[must_use]
    pub const fn risk_group_id(self) -> u32 {
        self.risk_group_id
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.u32(self.language_id);
        encoder.u32(self.task_family_id);
        encoder.u32(self.risk_group_id);
    }
}

/// One prospectively classified G1 task and exact rational design weight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct G1TaskV1 {
    id: G1TaskId,
    cluster_id: u64,
    domain: G1Domain,
    expectation_eligible: bool,
    subgroup: G1SubgroupV1,
    weight_numerator: u64,
}

impl G1TaskV1 {
    /// Constructs one task using the population's common weight denominator.
    pub fn new(
        id: G1TaskId,
        cluster_id: u64,
        domain: G1Domain,
        expectation_eligible: bool,
        subgroup: G1SubgroupV1,
        weight_numerator: u64,
    ) -> Result<Self, G1EnvelopeError> {
        if weight_numerator == 0 {
            return Err(G1EnvelopeError::ZeroDesignWeight { task_id: id });
        }
        if expectation_eligible && domain != G1Domain::ContextDependent {
            return Err(G1EnvelopeError::ExpectationTaskOutsideDependentDomain { task_id: id });
        }
        Ok(Self {
            id,
            cluster_id,
            domain,
            expectation_eligible,
            subgroup,
            weight_numerator,
        })
    }

    /// Returns the task identity.
    #[must_use]
    pub const fn id(self) -> G1TaskId {
        self.id
    }

    /// Returns the independent-cluster identity.
    #[must_use]
    pub const fn cluster_id(self) -> u64 {
        self.cluster_id
    }

    /// Returns the context-dependence domain.
    #[must_use]
    pub const fn domain(self) -> G1Domain {
        self.domain
    }

    /// Returns whether all four expectation interventions are prospectively valid.
    #[must_use]
    pub const fn expectation_eligible(self) -> bool {
        self.expectation_eligible
    }

    /// Returns the claim-bearing subgroup intersection.
    #[must_use]
    pub const fn subgroup(self) -> G1SubgroupV1 {
        self.subgroup
    }

    /// Returns the positive numerator over the population common denominator.
    #[must_use]
    pub const fn weight_numerator(self) -> u64 {
        self.weight_numerator
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.u64(self.id.get());
        encoder.u64(self.cluster_id);
        encoder.byte(self.domain.tag());
        encoder.byte(u8::from(self.expectation_eligible));
        self.subgroup.encode(encoder);
        encoder.u64(self.weight_numerator);
    }
}

/// Closed population slice that owns an exposure minimum.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1ExposureScope {
    /// Context-dependent population.
    ContextDependent,
    /// Context-independent population.
    ContextIndependent,
    /// Expectation-eligible subset of the dependent population.
    ExpectationEligible,
}

impl G1ExposureScope {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::ContextDependent => 1,
            Self::ContextIndependent => 2,
            Self::ExpectationEligible => 3,
        }
    }

    fn includes(self, task: G1TaskV1) -> bool {
        match self {
            Self::ContextDependent => task.domain == G1Domain::ContextDependent,
            Self::ContextIndependent => task.domain == G1Domain::ContextIndependent,
            Self::ExpectationEligible => task.expectation_eligible,
        }
    }
}

/// Positive designed task and independent-cluster minima for one population slice.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1ExposureRequirementV1 {
    scope: G1ExposureScope,
    subgroup: Option<G1SubgroupV1>,
    minimum_tasks: u32,
    minimum_clusters: u32,
}

impl G1ExposureRequirementV1 {
    /// Constructs a positive exposure requirement.
    pub fn new(
        scope: G1ExposureScope,
        subgroup: Option<G1SubgroupV1>,
        minimum_tasks: u32,
        minimum_clusters: u32,
    ) -> Result<Self, G1EnvelopeError> {
        if minimum_tasks == 0 || minimum_clusters == 0 {
            return Err(G1EnvelopeError::InvalidExposure { scope, subgroup });
        }
        Ok(Self {
            scope,
            subgroup,
            minimum_tasks,
            minimum_clusters,
        })
    }

    /// Returns the population scope.
    #[must_use]
    pub const fn scope(self) -> G1ExposureScope {
        self.scope
    }

    /// Returns the optional subgroup intersection.
    #[must_use]
    pub const fn subgroup(self) -> Option<G1SubgroupV1> {
        self.subgroup
    }

    /// Returns the minimum task count.
    #[must_use]
    pub const fn minimum_tasks(self) -> u32 {
        self.minimum_tasks
    }

    /// Returns the minimum independent-cluster count.
    #[must_use]
    pub const fn minimum_clusters(self) -> u32 {
        self.minimum_clusters
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.scope.tag());
        match self.subgroup {
            Some(subgroup) => {
                encoder.byte(1);
                subgroup.encode(encoder);
            }
            None => encoder.byte(0),
        }
        encoder.u32(self.minimum_tasks);
        encoder.u32(self.minimum_clusters);
    }
}

/// Canonical finite G1 population, partition, weights, and exposure contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G1PopulationV1 {
    tasks: Box<[G1TaskV1]>,
    weight_denominator: u64,
    exposures: Box<[G1ExposureRequirementV1]>,
}

impl G1PopulationV1 {
    /// Constructs and validates the complete prospective population design.
    pub fn new(
        mut tasks: Vec<G1TaskV1>,
        weight_denominator: u64,
        mut exposures: Vec<G1ExposureRequirementV1>,
    ) -> Result<Self, G1EnvelopeError> {
        if tasks.is_empty() {
            return Err(G1EnvelopeError::EmptyCollection);
        }
        if tasks.len() > MAX_TASKS {
            return Err(G1EnvelopeError::TooManyItems {
                actual: tasks.len(),
                maximum: MAX_TASKS,
            });
        }
        tasks.sort_unstable_by_key(|task| task.id);
        if let Some(pair) = tasks.windows(2).find(|pair| pair[0].id == pair[1].id) {
            return Err(G1EnvelopeError::DuplicateTask {
                task_id: pair[0].id,
            });
        }
        for domain in G1Domain::ALL {
            if !tasks.iter().any(|task| task.domain == domain) {
                return Err(G1EnvelopeError::EmptyDomain { domain });
            }
        }
        if !tasks.iter().any(|task| task.expectation_eligible) {
            return Err(G1EnvelopeError::EmptyExpectationSubset);
        }
        let numerator_sum = tasks.iter().try_fold(0_u128, |sum, task| {
            sum.checked_add(u128::from(task.weight_numerator))
        });
        if weight_denominator == 0 || numerator_sum != Some(u128::from(weight_denominator)) {
            return Err(G1EnvelopeError::InvalidDesignWeightMass);
        }

        if exposures.len() > MAX_EXPOSURES {
            return Err(G1EnvelopeError::TooManyItems {
                actual: exposures.len(),
                maximum: MAX_EXPOSURES,
            });
        }
        exposures.sort_unstable();
        if let Some(pair) = exposures
            .windows(2)
            .find(|pair| pair[0].scope == pair[1].scope && pair[0].subgroup == pair[1].subgroup)
        {
            return Err(G1EnvelopeError::DuplicateExposure {
                scope: pair[0].scope,
                subgroup: pair[0].subgroup,
            });
        }
        validate_exposures(&tasks, &exposures)?;

        Ok(Self {
            tasks: tasks.into_boxed_slice(),
            weight_denominator,
            exposures: exposures.into_boxed_slice(),
        })
    }

    /// Returns tasks in ascending task-ID order.
    #[must_use]
    pub fn tasks(&self) -> &[G1TaskV1] {
        &self.tasks
    }

    /// Returns the exact common rational-weight denominator.
    #[must_use]
    pub const fn weight_denominator(&self) -> u64 {
        self.weight_denominator
    }

    /// Returns exposure requirements in canonical scope/subgroup order.
    #[must_use]
    pub fn exposures(&self) -> &[G1ExposureRequirementV1] {
        &self.exposures
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.u32(u32::try_from(self.tasks.len()).expect("validated task count fits in u32"));
        encoder.u64(self.weight_denominator);
        for task in &self.tasks {
            task.encode(encoder);
        }
        encoder.u32(
            u32::try_from(self.exposures.len()).expect("validated exposure count fits in u32"),
        );
        for exposure in &self.exposures {
            exposure.encode(encoder);
        }
    }
}

fn validate_exposures(
    tasks: &[G1TaskV1],
    exposures: &[G1ExposureRequirementV1],
) -> Result<(), G1EnvelopeError> {
    let mut required = BTreeSet::new();
    for scope in [
        G1ExposureScope::ContextDependent,
        G1ExposureScope::ContextIndependent,
        G1ExposureScope::ExpectationEligible,
    ] {
        required.insert((scope, None));
        for subgroup in tasks
            .iter()
            .copied()
            .filter(|task| scope.includes(*task))
            .map(G1TaskV1::subgroup)
        {
            required.insert((scope, Some(subgroup)));
        }
    }
    let supplied: BTreeSet<_> = exposures
        .iter()
        .map(|exposure| (exposure.scope, exposure.subgroup))
        .collect();
    if let Some((scope, subgroup)) = required.difference(&supplied).next() {
        return Err(G1EnvelopeError::MissingExposure {
            scope: *scope,
            subgroup: *subgroup,
        });
    }
    for exposure in exposures {
        if !required.contains(&(exposure.scope, exposure.subgroup)) {
            return Err(G1EnvelopeError::InvalidExposure {
                scope: exposure.scope,
                subgroup: exposure.subgroup,
            });
        }
        let members: Vec<_> = tasks
            .iter()
            .copied()
            .filter(|task| {
                exposure.scope.includes(*task)
                    && exposure
                        .subgroup
                        .is_none_or(|subgroup| task.subgroup == subgroup)
            })
            .collect();
        let clusters: BTreeSet<_> = members.iter().map(|task| task.cluster_id).collect();
        if usize::try_from(exposure.minimum_tasks)
            .ok()
            .is_none_or(|minimum| minimum > members.len())
            || usize::try_from(exposure.minimum_clusters)
                .ok()
                .is_none_or(|minimum| minimum > clusters.len())
        {
            return Err(G1EnvelopeError::InvalidExposure {
                scope: exposure.scope,
                subgroup: exposure.subgroup,
            });
        }
    }
    Ok(())
}

/// One of the four separately gated leakage classes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1LeakageClass {
    /// Answer leakage.
    Answer,
    /// Action or tool-selection leakage.
    Action,
    /// Unsupported fact promotion.
    Fact,
    /// Unsupported probability or confidence promotion.
    Probability,
}

impl G1LeakageClass {
    pub(super) const ALL: [Self; 4] = [Self::Answer, Self::Action, Self::Fact, Self::Probability];

    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::Answer => 1,
            Self::Action => 2,
            Self::Fact => 3,
            Self::Probability => 4,
        }
    }
}

/// A focus-headroom baseline.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1Baseline {
    /// Original prompt only.
    Prompt,
    /// Situation and metadata without persistent memory.
    Situation,
    /// Matched irrelevant placebo attention.
    Placebo,
}

impl G1Baseline {
    const ALL: [Self; 3] = [Self::Prompt, Self::Situation, Self::Placebo];

    const fn tag(self) -> u8 {
        match self {
            Self::Prompt => 1,
            Self::Situation => 2,
            Self::Placebo => 3,
        }
    }
}

/// An expectation-branch role used by threshold keys.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1ExpectationRole {
    /// Neutral-carrier focus comparator.
    Focus,
    /// Correct qualified expectation.
    Correct,
    /// Deliberately wrong expectation.
    Wrong,
    /// Explicit expectation abstention.
    Abstain,
}

impl G1ExpectationRole {
    const ALL: [Self; 4] = [Self::Focus, Self::Correct, Self::Wrong, Self::Abstain];

    const fn tag(self) -> u8 {
        match self {
            Self::Focus => 1,
            Self::Correct => 2,
            Self::Wrong => 3,
            Self::Abstain => 4,
        }
    }

    /// Returns the fixed G1 condition selected by this expectation role.
    #[must_use]
    pub const fn condition(self) -> G1Condition {
        match self {
            Self::Focus => G1Condition::Focus,
            Self::Correct => G1Condition::Correct,
            Self::Wrong => G1Condition::Wrong,
            Self::Abstain => G1Condition::Abstain,
        }
    }
}

/// One supported non-wrong control for wrong-condition harm.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1WrongControl {
    /// Correct qualified expectation.
    Correct,
    /// Explicit expectation abstention.
    Abstain,
}

impl G1WrongControl {
    const fn tag(self) -> u8 {
        match self {
            Self::Correct => 1,
            Self::Abstain => 2,
        }
    }
}

/// One proof-owned G1 threshold coordinate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1ThresholdKey {
    /// Context-dependent focus superiority against a baseline.
    FocusSuperiority(G1Baseline),
    /// Context-independent focus non-inferiority against a baseline.
    FocusNonInferiority(G1Baseline),
    /// Maximum focus population harm for a baseline and domain.
    FocusPopulationHarm(G1Baseline, G1Domain),
    /// Maximum focus conditional reversal for a baseline and domain.
    FocusConditionalReversal(G1Baseline, G1Domain),
    /// Minimum correct-expectation contribution.
    CorrectContribution,
    /// Maximum correct-expectation population harm.
    CorrectPopulationHarm,
    /// Maximum correct-expectation conditional reversal.
    CorrectConditionalReversal,
    /// Maximum absolute correct-expectation anchoring.
    CorrectAnchoring,
    /// Maximum correct-minus-focus anchoring difference.
    CorrectAnchoringDifference,
    /// Maximum absolute leakage for one role and class.
    AbsoluteLeakage(G1ExpectationRole, G1LeakageClass),
    /// Maximum correct-minus-focus leakage difference for one class.
    CorrectLeakageDifference(G1LeakageClass),
    /// Minimum blinded correct-versus-wrong differentiation.
    CorrectWrongDifferentiation,
    /// Minimum blinded abstain-versus-wrong differentiation.
    AbstainWrongDifferentiation,
    /// Maximum wrong-condition harm against one non-wrong control.
    WrongPopulationHarm(G1WrongControl),
    /// Maximum absolute wrong-condition anchoring.
    WrongAnchoring,
    /// Maximum wrong-condition leakage for one class.
    WrongLeakage(G1LeakageClass),
}

impl G1ThresholdKey {
    pub(super) fn required() -> Vec<Self> {
        let mut keys = Vec::with_capacity(52);
        for baseline in G1Baseline::ALL {
            keys.push(Self::FocusSuperiority(baseline));
            keys.push(Self::FocusNonInferiority(baseline));
            for domain in G1Domain::ALL {
                keys.push(Self::FocusPopulationHarm(baseline, domain));
                keys.push(Self::FocusConditionalReversal(baseline, domain));
            }
        }
        keys.extend([
            Self::CorrectContribution,
            Self::CorrectPopulationHarm,
            Self::CorrectConditionalReversal,
            Self::CorrectAnchoring,
            Self::CorrectAnchoringDifference,
            Self::CorrectWrongDifferentiation,
            Self::AbstainWrongDifferentiation,
            Self::WrongPopulationHarm(G1WrongControl::Correct),
            Self::WrongPopulationHarm(G1WrongControl::Abstain),
            Self::WrongAnchoring,
        ]);
        for role in G1ExpectationRole::ALL {
            for class in G1LeakageClass::ALL {
                keys.push(Self::AbsoluteLeakage(role, class));
            }
        }
        for class in G1LeakageClass::ALL {
            keys.push(Self::CorrectLeakageDifference(class));
            keys.push(Self::WrongLeakage(class));
        }
        keys.sort_unstable();
        keys
    }

    /// Returns every required threshold coordinate in canonical order.
    #[must_use]
    pub fn required_keys() -> Vec<Self> {
        Self::required()
    }

    fn permits_zero(self) -> bool {
        matches!(
            self,
            Self::CorrectAnchoringDifference | Self::CorrectLeakageDifference(_)
        )
    }

    pub(super) fn encode(self, encoder: &mut Encoder) {
        match self {
            Self::FocusSuperiority(baseline) => {
                encoder.byte(1);
                encoder.byte(baseline.tag());
            }
            Self::FocusNonInferiority(baseline) => {
                encoder.byte(2);
                encoder.byte(baseline.tag());
            }
            Self::FocusPopulationHarm(baseline, domain) => {
                encoder.byte(3);
                encoder.byte(baseline.tag());
                encoder.byte(domain.tag());
            }
            Self::FocusConditionalReversal(baseline, domain) => {
                encoder.byte(4);
                encoder.byte(baseline.tag());
                encoder.byte(domain.tag());
            }
            Self::CorrectContribution => encoder.byte(5),
            Self::CorrectPopulationHarm => encoder.byte(6),
            Self::CorrectConditionalReversal => encoder.byte(7),
            Self::CorrectAnchoring => encoder.byte(8),
            Self::CorrectAnchoringDifference => encoder.byte(9),
            Self::AbsoluteLeakage(role, class) => {
                encoder.byte(10);
                encoder.byte(role.tag());
                encoder.byte(class.tag());
            }
            Self::CorrectLeakageDifference(class) => {
                encoder.byte(11);
                encoder.byte(class.tag());
            }
            Self::CorrectWrongDifferentiation => encoder.byte(12),
            Self::AbstainWrongDifferentiation => encoder.byte(13),
            Self::WrongPopulationHarm(role) => {
                encoder.byte(14);
                encoder.byte(role.tag());
            }
            Self::WrongAnchoring => encoder.byte(15),
            Self::WrongLeakage(class) => {
                encoder.byte(16);
                encoder.byte(class.tag());
            }
        }
    }
}

/// One finite G1 threshold validated against its proof-owned domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G1ThresholdV1 {
    key: G1ThresholdKey,
    value: f64,
}

impl Eq for G1ThresholdV1 {}

impl G1ThresholdV1 {
    /// Constructs a threshold in `(0, 1)`, or `[0, 1)` for paired maxima.
    pub fn new(key: G1ThresholdKey, value: f64) -> Result<Self, G1EnvelopeError> {
        let valid = value.is_finite()
            && value < 1.0
            && if key.permits_zero() {
                value >= 0.0
            } else {
                value > 0.0
            };
        if !valid {
            return Err(G1EnvelopeError::InvalidThreshold { key });
        }
        Ok(Self {
            key,
            value: if value == 0.0 { 0.0 } else { value },
        })
    }

    /// Returns the threshold coordinate.
    #[must_use]
    pub const fn key(self) -> G1ThresholdKey {
        self.key
    }

    /// Returns the finite threshold value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        self.key.encode(encoder);
        encoder.u64(self.value.to_bits());
    }
}

/// Closed critical failure classes requiring independent rate bounds.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1CriticalFailureClass {
    /// Answer leakage.
    AnswerLeakage,
    /// Action leakage.
    ActionLeakage,
    /// Fact promotion.
    FactPromotion,
    /// Probability promotion.
    ProbabilityPromotion,
    /// Expectation anchoring.
    Anchoring,
}

impl G1CriticalFailureClass {
    pub(super) const ALL: [Self; 5] = [
        Self::AnswerLeakage,
        Self::ActionLeakage,
        Self::FactPromotion,
        Self::ProbabilityPromotion,
        Self::Anchoring,
    ];

    /// Returns the closed critical-failure domain in canonical order.
    #[must_use]
    pub const fn all() -> &'static [Self; 5] {
        &Self::ALL
    }

    const fn tag(self) -> u8 {
        match self {
            Self::AnswerLeakage => 1,
            Self::ActionLeakage => 2,
            Self::FactPromotion => 3,
            Self::ProbabilityPromotion => 4,
            Self::Anchoring => 5,
        }
    }
}

/// Minimum exposure and maximum one-sided rate bound for a critical class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct G1CriticalFailureBoundV1 {
    class: G1CriticalFailureClass,
    minimum_exposure: u32,
    maximum_rate_bits: u64,
}

impl G1CriticalFailureBoundV1 {
    /// Constructs a positive exposure and a maximum rate in `(0, 1)`.
    pub fn new(
        class: G1CriticalFailureClass,
        minimum_exposure: u32,
        maximum_rate: f64,
    ) -> Result<Self, G1EnvelopeError> {
        if minimum_exposure == 0 {
            return Err(G1EnvelopeError::InvalidCriticalFailureExposure { class });
        }
        if !maximum_rate.is_finite() || maximum_rate <= 0.0 || maximum_rate >= 1.0 {
            return Err(G1EnvelopeError::InvalidCriticalFailureRate { class });
        }
        Ok(Self {
            class,
            minimum_exposure,
            maximum_rate_bits: maximum_rate.to_bits(),
        })
    }

    /// Returns the critical failure class.
    #[must_use]
    pub const fn class(self) -> G1CriticalFailureClass {
        self.class
    }

    /// Returns the minimum exposure.
    #[must_use]
    pub const fn minimum_exposure(self) -> u32 {
        self.minimum_exposure
    }

    /// Returns the maximum one-sided rate bound.
    #[must_use]
    pub fn maximum_rate(self) -> f64 {
        f64::from_bits(self.maximum_rate_bits)
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.class.tag());
        encoder.u32(self.minimum_exposure);
        encoder.u64(self.maximum_rate_bits);
    }
}

/// Closed identities required to make the G1 design reconstructible.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1ArtifactKind {
    /// G1 semantic root.
    G1SemanticRoot,
    /// Case-semantics authorship protocol.
    CaseSemantics,
    /// Population membership authorship protocol.
    MembershipAuthorship,
    /// Sealed population root.
    PopulationRoot,
    /// Expert condition-authoring protocol.
    ConditionAuthoring,
    /// Neutral carrier grammar.
    NeutralCarrierGrammar,
    /// Exact neutral carrier bytes.
    NeutralCarrierBytes,
    /// Neutral carrier semantic adjudication.
    NeutralCarrierAdjudication,
    /// Placebo source pool.
    PlaceboSourcePool,
    /// Placebo generation procedure.
    PlaceboGeneration,
    /// Placebo irrelevance rubric.
    PlaceboIrrelevanceRubric,
    /// Blinded placebo irrelevance adjudication.
    PlaceboAdjudication,
    /// Tokenizer implementation.
    Tokenizer,
    /// Exact tokenizer configuration.
    TokenizerConfiguration,
    /// Token-matching procedure.
    TokenMatching,
    /// Tokenizer and count audit.
    TokenizerAudit,
    /// Prompt template.
    PromptTemplate,
    /// Output language rule and selected language.
    Language,
    /// Message role.
    MessageRole,
    /// Message placement.
    Placement,
    /// Downstream model.
    DownstreamModel,
    /// Runtime environment.
    Runtime,
    /// Decoding configuration.
    Decoding,
    /// Sampling design and inclusion probabilities.
    SamplingDesign,
    /// Independent-cluster hierarchy.
    ClusterHierarchy,
    /// Primary endpoint.
    PrimaryEndpoint,
    /// Confidence procedure.
    ConfidenceProcedure,
    /// Hypothesis-test implementation.
    HypothesisTest,
    /// G1-local multiplicity procedure.
    MultiplicityProcedure,
    /// Subgroup gate procedure.
    SubgroupGate,
    /// Sample-size and power calculation.
    SampleSizePower,
    /// Seed schedule and aggregation.
    SeedSchedule,
    /// Timeout, crash, and missing-data policy.
    MissingFailurePolicy,
    /// Exclusion and corruption policy.
    ExclusionCorruptionPolicy,
    /// Expectation-eligibility rule.
    ExpectationEligibility,
    /// Correct-expectation semantic correctness gate.
    CorrectnessGate,
    /// Expectation qualification gate.
    QualificationGate,
    /// Material-alternative gate.
    AlternativeGate,
    /// Expectation faithfulness gate.
    FaithfulnessGate,
    /// Exact-value gate.
    ExactValueGate,
    /// Correct-versus-wrong blinded discriminator.
    CorrectWrongDiscriminator,
    /// Abstain-versus-wrong blinded discriminator.
    AbstainWrongDiscriminator,
    /// Anchoring rubric.
    AnchoringRubric,
    /// Answer-leakage rubric.
    AnswerLeakageRubric,
    /// Action-leakage rubric.
    ActionLeakageRubric,
    /// Fact-leakage rubric.
    FactLeakageRubric,
    /// Probability-leakage rubric.
    ProbabilityLeakageRubric,
    /// Analysis implementation.
    AnalysisImplementation,
    /// Custody policy.
    CustodyPolicy,
}

impl G1ArtifactKind {
    pub(super) const ALL: [Self; 49] = [
        Self::G1SemanticRoot,
        Self::CaseSemantics,
        Self::MembershipAuthorship,
        Self::PopulationRoot,
        Self::ConditionAuthoring,
        Self::NeutralCarrierGrammar,
        Self::NeutralCarrierBytes,
        Self::NeutralCarrierAdjudication,
        Self::PlaceboSourcePool,
        Self::PlaceboGeneration,
        Self::PlaceboIrrelevanceRubric,
        Self::PlaceboAdjudication,
        Self::Tokenizer,
        Self::TokenizerConfiguration,
        Self::TokenMatching,
        Self::TokenizerAudit,
        Self::PromptTemplate,
        Self::Language,
        Self::MessageRole,
        Self::Placement,
        Self::DownstreamModel,
        Self::Runtime,
        Self::Decoding,
        Self::SamplingDesign,
        Self::ClusterHierarchy,
        Self::PrimaryEndpoint,
        Self::ConfidenceProcedure,
        Self::HypothesisTest,
        Self::MultiplicityProcedure,
        Self::SubgroupGate,
        Self::SampleSizePower,
        Self::SeedSchedule,
        Self::MissingFailurePolicy,
        Self::ExclusionCorruptionPolicy,
        Self::ExpectationEligibility,
        Self::CorrectnessGate,
        Self::QualificationGate,
        Self::AlternativeGate,
        Self::FaithfulnessGate,
        Self::ExactValueGate,
        Self::CorrectWrongDiscriminator,
        Self::AbstainWrongDiscriminator,
        Self::AnchoringRubric,
        Self::AnswerLeakageRubric,
        Self::ActionLeakageRubric,
        Self::FactLeakageRubric,
        Self::ProbabilityLeakageRubric,
        Self::AnalysisImplementation,
        Self::CustodyPolicy,
    ];

    /// Returns every required design artifact kind in canonical order.
    #[must_use]
    pub const fn required() -> &'static [Self; 49] {
        &Self::ALL
    }

    const fn tag(self) -> u8 {
        self as u8 + 1
    }
}

/// One required G1 design-artifact identity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1ArtifactBindingV1 {
    kind: G1ArtifactKind,
    identity: EvidenceIdentity,
}

impl G1ArtifactBindingV1 {
    /// Constructs a nonempty design-artifact binding.
    pub fn new(kind: G1ArtifactKind, identity: EvidenceIdentity) -> Result<Self, G1EnvelopeError> {
        ensure_identity(identity)?;
        Ok(Self { kind, identity })
    }

    /// Returns the artifact kind.
    #[must_use]
    pub const fn kind(self) -> G1ArtifactKind {
        self.kind
    }

    /// Returns the artifact identity.
    #[must_use]
    pub const fn identity(self) -> EvidenceIdentity {
        self.identity
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.kind.tag());
        encoder.fixed(self.identity.as_bytes());
    }
}

/// Opaque identity of one fresh G1 execution instance.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1ExecutionIdentity([u8; 32]);

impl G1ExecutionIdentity {
    /// Constructs a nonempty execution identity.
    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self, G1EnvelopeError> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(G1EnvelopeError::EmptyIdentity);
        }
        Ok(Self(bytes))
    }

    /// Returns the exact bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Closed exact artifacts resolved during G1 run finalization.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum G1RunArtifactKind {
    /// Exact condition execution order.
    ConditionOrder,
    /// Audit over the exact seven condition artifacts used.
    ConditionArtifactAudit,
    /// Exact hardware identity.
    Hardware,
    /// Exact operating-system identity.
    OperatingSystem,
    /// Exact execution environment snapshot.
    Environment,
    /// Exact token-matching audit artifact.
    TokenMatchingAudit,
}

impl G1RunArtifactKind {
    pub(super) const ALL: [Self; 6] = [
        Self::ConditionOrder,
        Self::ConditionArtifactAudit,
        Self::Hardware,
        Self::OperatingSystem,
        Self::Environment,
        Self::TokenMatchingAudit,
    ];

    /// Returns every required run artifact kind in canonical order.
    #[must_use]
    pub const fn required() -> &'static [Self; 6] {
        &Self::ALL
    }

    const fn tag(self) -> u8 {
        self as u8 + 1
    }
}

/// One exact run-finalization artifact.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct G1RunArtifactBindingV1 {
    kind: G1RunArtifactKind,
    identity: EvidenceIdentity,
}

impl G1RunArtifactBindingV1 {
    /// Constructs a nonempty run-artifact binding.
    pub fn new(
        kind: G1RunArtifactKind,
        identity: EvidenceIdentity,
    ) -> Result<Self, G1EnvelopeError> {
        ensure_identity(identity)?;
        Ok(Self { kind, identity })
    }

    /// Returns the run artifact kind.
    #[must_use]
    pub const fn kind(self) -> G1RunArtifactKind {
        self.kind
    }

    /// Returns the run artifact identity.
    #[must_use]
    pub const fn identity(self) -> EvidenceIdentity {
        self.identity
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.kind.tag());
        encoder.fixed(self.identity.as_bytes());
    }
}

/// Canonical exact execution-instance and run-artifact binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G1ExecutionBindingV1 {
    execution_identity: G1ExecutionIdentity,
    artifacts: Box<[G1RunArtifactBindingV1]>,
}

impl G1ExecutionBindingV1 {
    /// Constructs a complete canonical run binding.
    pub fn new(
        execution_identity: G1ExecutionIdentity,
        artifacts: Vec<G1RunArtifactBindingV1>,
    ) -> Result<Self, G1EnvelopeError> {
        let artifacts = canonical_run_artifacts(artifacts)?;
        Ok(Self {
            execution_identity,
            artifacts,
        })
    }

    /// Returns the fresh execution-instance identity.
    #[must_use]
    pub const fn execution_identity(&self) -> G1ExecutionIdentity {
        self.execution_identity
    }

    /// Returns run artifacts in canonical kind order.
    #[must_use]
    pub fn artifacts(&self) -> &[G1RunArtifactBindingV1] {
        &self.artifacts
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.fixed(self.execution_identity.as_bytes());
        encoder.u32(
            u32::try_from(self.artifacts.len()).expect("fixed run artifact count fits in u32"),
        );
        for artifact in &self.artifacts {
            artifact.encode(encoder);
        }
    }
}

pub(super) fn canonical_conditions(
    mut conditions: Vec<G1ConditionArtifactV1>,
) -> Result<Box<[G1ConditionArtifactV1]>, G1EnvelopeError> {
    conditions.sort_unstable();
    if let Some(pair) = conditions
        .windows(2)
        .find(|pair| pair[0].condition == pair[1].condition)
    {
        return Err(G1EnvelopeError::DuplicateCondition {
            condition: pair[0].condition,
        });
    }
    for condition in G1Condition::ALL {
        if !conditions.iter().any(|item| item.condition == condition) {
            return Err(G1EnvelopeError::MissingCondition { condition });
        }
    }
    Ok(conditions.into_boxed_slice())
}

pub(super) fn canonical_artifacts(
    mut artifacts: Vec<G1ArtifactBindingV1>,
) -> Result<Box<[G1ArtifactBindingV1]>, G1EnvelopeError> {
    if artifacts.len() > MAX_ARTIFACTS {
        return Err(G1EnvelopeError::TooManyItems {
            actual: artifacts.len(),
            maximum: MAX_ARTIFACTS,
        });
    }
    artifacts.sort_unstable();
    if let Some(pair) = artifacts
        .windows(2)
        .find(|pair| pair[0].kind == pair[1].kind)
    {
        return Err(G1EnvelopeError::DuplicateArtifact { kind: pair[0].kind });
    }
    for kind in G1ArtifactKind::ALL {
        if !artifacts.iter().any(|binding| binding.kind == kind) {
            return Err(G1EnvelopeError::MissingArtifact { kind });
        }
    }
    Ok(artifacts.into_boxed_slice())
}

pub(super) fn canonical_thresholds(
    mut thresholds: Vec<G1ThresholdV1>,
) -> Result<Box<[G1ThresholdV1]>, G1EnvelopeError> {
    thresholds.sort_unstable_by_key(|threshold| threshold.key);
    if let Some(pair) = thresholds
        .windows(2)
        .find(|pair| pair[0].key == pair[1].key)
    {
        return Err(G1EnvelopeError::DuplicateThreshold { key: pair[0].key });
    }
    for key in G1ThresholdKey::required() {
        if !thresholds.iter().any(|threshold| threshold.key == key) {
            return Err(G1EnvelopeError::MissingThreshold { key });
        }
    }
    Ok(thresholds.into_boxed_slice())
}

pub(super) fn canonical_critical_failures(
    mut bounds: Vec<G1CriticalFailureBoundV1>,
) -> Result<Box<[G1CriticalFailureBoundV1]>, G1EnvelopeError> {
    bounds.sort_unstable_by_key(|bound| bound.class);
    if let Some(pair) = bounds
        .windows(2)
        .find(|pair| pair[0].class == pair[1].class)
    {
        return Err(G1EnvelopeError::DuplicateCriticalFailure {
            class: pair[0].class,
        });
    }
    for class in G1CriticalFailureClass::ALL {
        if !bounds.iter().any(|bound| bound.class == class) {
            return Err(G1EnvelopeError::MissingCriticalFailure { class });
        }
    }
    Ok(bounds.into_boxed_slice())
}

fn canonical_run_artifacts(
    mut artifacts: Vec<G1RunArtifactBindingV1>,
) -> Result<Box<[G1RunArtifactBindingV1]>, G1EnvelopeError> {
    if artifacts.len() > MAX_RUN_ARTIFACTS {
        return Err(G1EnvelopeError::TooManyItems {
            actual: artifacts.len(),
            maximum: MAX_RUN_ARTIFACTS,
        });
    }
    artifacts.sort_unstable();
    if let Some(pair) = artifacts
        .windows(2)
        .find(|pair| pair[0].kind == pair[1].kind)
    {
        return Err(G1EnvelopeError::DuplicateRunArtifact { kind: pair[0].kind });
    }
    for kind in G1RunArtifactKind::ALL {
        if !artifacts.iter().any(|binding| binding.kind == kind) {
            return Err(G1EnvelopeError::MissingRunArtifact { kind });
        }
    }
    Ok(artifacts.into_boxed_slice())
}

fn ensure_identity(identity: EvidenceIdentity) -> Result<(), G1EnvelopeError> {
    if identity.as_bytes().iter().all(|byte| *byte == 0) {
        return Err(G1EnvelopeError::EmptyIdentity);
    }
    Ok(())
}
