use std::collections::{BTreeMap, BTreeSet};

use super::{SyntheticPilotError, canonical::Encoder};

const GENERATION_MANIFEST_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.generation.v1";
const GENERATION_LOG_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.generation-log.v1";
const CORPUS_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.corpus.v1";
const CONDITION_SET_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.conditions.v1";
const SCORING_MANIFEST_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.scoring.v1";
const RUNNER_MANIFEST_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.runner.v1";
const FROZEN_PILOT_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.frozen.v1";

fn require_text(value: &str, field: &'static str) -> Result<(), SyntheticPilotError> {
    if value.trim().is_empty() {
        Err(SyntheticPilotError::EmptyField(field))
    } else {
        Ok(())
    }
}

fn require_bytes(value: &[u8], field: &'static str) -> Result<(), SyntheticPilotError> {
    if value.is_empty() {
        Err(SyntheticPilotError::EmptyField(field))
    } else {
        Ok(())
    }
}

/// Domain-separated SHA-256 identity of one frozen pilot artifact.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PilotRoot([u8; 32]);

impl PilotRoot {
    pub(super) const fn from_digest(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the exact digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Stable nonzero identifier of one generated pilot task.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PilotTaskId(u64);

impl PilotTaskId {
    /// Constructs a nonzero task identifier.
    pub fn new(value: u64) -> Result<Self, SyntheticPilotError> {
        if value == 0 {
            Err(SyntheticPilotError::ZeroIdentifier("task_id"))
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Stable nonzero identifier of one retained generation attempt.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenerationAttemptId(u64);

impl GenerationAttemptId {
    /// Constructs a nonzero generation-attempt identifier.
    pub fn new(value: u64) -> Result<Self, SyntheticPilotError> {
        if value == 0 {
            Err(SyntheticPilotError::ZeroIdentifier("generation_attempt_id"))
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Mandatory disclosure for a selected generation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCostPrivacyDisclosureV1 {
    provider: Box<str>,
    model: Box<str>,
    immutable_version: Box<str>,
    maximum_cost: Box<str>,
    data_destination: Box<str>,
    retention_policy: Box<str>,
    privacy_implication: Box<str>,
}

impl ModelCostPrivacyDisclosureV1 {
    /// Constructs a complete model, cost, and privacy disclosure.
    pub fn new(
        provider: impl Into<Box<str>>,
        model: impl Into<Box<str>>,
        immutable_version: impl Into<Box<str>>,
        maximum_cost: impl Into<Box<str>>,
        data_destination: impl Into<Box<str>>,
        retention_policy: impl Into<Box<str>>,
        privacy_implication: impl Into<Box<str>>,
    ) -> Result<Self, SyntheticPilotError> {
        let value = Self {
            provider: provider.into(),
            model: model.into(),
            immutable_version: immutable_version.into(),
            maximum_cost: maximum_cost.into(),
            data_destination: data_destination.into(),
            retention_policy: retention_policy.into(),
            privacy_implication: privacy_implication.into(),
        };
        require_text(&value.provider, "model_provider")?;
        require_text(&value.model, "model_name")?;
        require_text(&value.immutable_version, "model_version")?;
        require_text(&value.maximum_cost, "maximum_cost")?;
        require_text(&value.data_destination, "data_destination")?;
        require_text(&value.retention_policy, "retention_policy")?;
        require_text(&value.privacy_implication, "privacy_implication")?;
        Ok(value)
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.text(&self.provider);
        encoder.text(&self.model);
        encoder.text(&self.immutable_version);
        encoder.text(&self.maximum_cost);
        encoder.text(&self.data_destination);
        encoder.text(&self.retention_policy);
        encoder.text(&self.privacy_implication);
    }

    /// Returns the exact model provider.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns the exact model name.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the immutable model version or weight identity.
    #[must_use]
    pub fn immutable_version(&self) -> &str {
        &self.immutable_version
    }

    /// Returns the approved maximum cost statement.
    #[must_use]
    pub fn maximum_cost(&self) -> &str {
        &self.maximum_cost
    }

    /// Returns the declared data destination.
    #[must_use]
    pub fn data_destination(&self) -> &str {
        &self.data_destination
    }

    /// Returns the declared provider retention policy.
    #[must_use]
    pub fn retention_policy(&self) -> &str {
        &self.retention_policy
    }

    /// Returns the approved privacy-implication statement.
    #[must_use]
    pub fn privacy_implication(&self) -> &str {
        &self.privacy_implication
    }
}

/// Frozen generator configuration required before any AI material is produced.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerationManifestV1 {
    disclosure: ModelCostPrivacyDisclosureV1,
    exact_prompt: Box<[u8]>,
    tokenizer_identity: Box<str>,
    decoding_configuration: Box<str>,
    seed_schedule: Box<str>,
    tool_and_network_policy: Box<str>,
    runtime_identity: Box<str>,
    root: PilotRoot,
}

impl GenerationManifestV1 {
    /// Constructs and content-identifies a complete generation manifest.
    pub fn new(
        disclosure: ModelCostPrivacyDisclosureV1,
        exact_prompt: impl Into<Box<[u8]>>,
        tokenizer_identity: impl Into<Box<str>>,
        decoding_configuration: impl Into<Box<str>>,
        seed_schedule: impl Into<Box<str>>,
        tool_and_network_policy: impl Into<Box<str>>,
        runtime_identity: impl Into<Box<str>>,
    ) -> Result<Self, SyntheticPilotError> {
        let exact_prompt = exact_prompt.into();
        let tokenizer_identity = tokenizer_identity.into();
        let decoding_configuration = decoding_configuration.into();
        let seed_schedule = seed_schedule.into();
        let tool_and_network_policy = tool_and_network_policy.into();
        let runtime_identity = runtime_identity.into();
        require_bytes(&exact_prompt, "generation_prompt")?;
        require_text(&tokenizer_identity, "tokenizer_identity")?;
        require_text(&decoding_configuration, "decoding_configuration")?;
        require_text(&seed_schedule, "generation_seed_schedule")?;
        require_text(&tool_and_network_policy, "tool_and_network_policy")?;
        require_text(&runtime_identity, "generation_runtime_identity")?;
        let mut encoder = Encoder::new();
        disclosure.encode(&mut encoder);
        encoder.bytes(&exact_prompt);
        encoder.text(&tokenizer_identity);
        encoder.text(&decoding_configuration);
        encoder.text(&seed_schedule);
        encoder.text(&tool_and_network_policy);
        encoder.text(&runtime_identity);
        let root = encoder.root(GENERATION_MANIFEST_DOMAIN);
        Ok(Self {
            disclosure,
            exact_prompt,
            tokenizer_identity,
            decoding_configuration,
            seed_schedule,
            tool_and_network_policy,
            runtime_identity,
            root,
        })
    }

    /// Returns the mandatory model, cost, and privacy disclosure.
    #[must_use]
    pub const fn disclosure(&self) -> &ModelCostPrivacyDisclosureV1 {
        &self.disclosure
    }

    /// Returns the exact frozen generation prompt bytes.
    #[must_use]
    pub fn exact_prompt(&self) -> &[u8] {
        &self.exact_prompt
    }

    /// Returns the generation-manifest content root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }

    fn encode_root(&self, encoder: &mut Encoder) {
        encoder.bytes(self.root.as_bytes());
    }
}

/// Terminal classification of one retained generation attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerationAttemptDispositionV1 {
    /// The output became the sole source attempt for one task.
    Accepted(PilotTaskId),
    /// The output was retained but excluded under the frozen selection rule.
    Rejected {
        /// Frozen reason for exclusion.
        reason: Box<str>,
    },
    /// Generation did not produce a usable output.
    GenerationError {
        /// Exact captured error classification.
        reason: Box<str>,
    },
}

impl GenerationAttemptDispositionV1 {
    fn validate(&self) -> Result<(), SyntheticPilotError> {
        match self {
            Self::Accepted(_) => Ok(()),
            Self::Rejected { reason } => require_text(reason, "generation_rejection_reason"),
            Self::GenerationError { reason } => require_text(reason, "generation_error_reason"),
        }
    }

    fn encode(&self, encoder: &mut Encoder) {
        match self {
            Self::Accepted(task_id) => {
                encoder.byte(1);
                encoder.u64(task_id.get());
            }
            Self::Rejected { reason } => {
                encoder.byte(2);
                encoder.text(reason);
            }
            Self::GenerationError { reason } => {
                encoder.byte(3);
                encoder.text(reason);
            }
        }
    }
}

/// Complete retained input, output, and disposition of one generator call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerationAttemptV1 {
    id: GenerationAttemptId,
    exact_input: Box<[u8]>,
    exact_output: Box<[u8]>,
    disposition: GenerationAttemptDispositionV1,
}

impl GenerationAttemptV1 {
    /// Constructs a retained generation attempt.
    pub fn new(
        id: GenerationAttemptId,
        exact_input: impl Into<Box<[u8]>>,
        exact_output: impl Into<Box<[u8]>>,
        disposition: GenerationAttemptDispositionV1,
    ) -> Result<Self, SyntheticPilotError> {
        let exact_input = exact_input.into();
        require_bytes(&exact_input, "generation_attempt_input")?;
        disposition.validate()?;
        Ok(Self {
            id,
            exact_input,
            exact_output: exact_output.into(),
            disposition,
        })
    }

    /// Returns the attempt identifier.
    #[must_use]
    pub const fn id(&self) -> GenerationAttemptId {
        self.id
    }

    /// Returns the complete retained generator output.
    #[must_use]
    pub fn exact_output(&self) -> &[u8] {
        &self.exact_output
    }

    /// Returns the frozen attempt disposition.
    #[must_use]
    pub const fn disposition(&self) -> &GenerationAttemptDispositionV1 {
        &self.disposition
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.u64(self.id.get());
        encoder.bytes(&self.exact_input);
        encoder.bytes(&self.exact_output);
        self.disposition.encode(encoder);
    }
}

/// Append-only logical log retaining every generation attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerationLogV1 {
    generation_manifest_root: PilotRoot,
    attempts: Box<[GenerationAttemptV1]>,
    root: PilotRoot,
}

impl GenerationLogV1 {
    /// Constructs a canonical nonempty generation log.
    pub fn new(
        mut attempts: Vec<GenerationAttemptV1>,
        generation_manifest: &GenerationManifestV1,
    ) -> Result<Self, SyntheticPilotError> {
        if attempts.is_empty() {
            return Err(SyntheticPilotError::EmptyField("generation_attempts"));
        }
        attempts.sort_by_key(GenerationAttemptV1::id);
        for pair in attempts.windows(2) {
            if pair[0].id() == pair[1].id() {
                return Err(SyntheticPilotError::DuplicateAttempt(pair[0].id()));
            }
        }
        let mut encoder = Encoder::new();
        encoder.bytes(generation_manifest.root().as_bytes());
        encoder.u64(attempts.len() as u64);
        for attempt in &attempts {
            attempt.encode(&mut encoder);
        }
        let root = encoder.root(GENERATION_LOG_DOMAIN);
        Ok(Self {
            generation_manifest_root: generation_manifest.root(),
            attempts: attempts.into_boxed_slice(),
            root,
        })
    }

    /// Returns every attempt in canonical identifier order.
    #[must_use]
    pub fn attempts(&self) -> &[GenerationAttemptV1] {
        &self.attempts
    }

    /// Returns the generation-log content root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }
}

/// One generated task with frozen pilot-only labels and provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPilotTaskV1 {
    id: PilotTaskId,
    source_attempt: GenerationAttemptId,
    prompt: Box<[u8]>,
    situation: Box<[u8]>,
    relevant_prior_constraints: Box<[u8]>,
    scoring_labels: Box<[u8]>,
}

impl GeneratedPilotTaskV1 {
    /// Constructs one generated task and its exact frozen labels.
    pub fn new(
        id: PilotTaskId,
        source_attempt: GenerationAttemptId,
        prompt: impl Into<Box<[u8]>>,
        situation: impl Into<Box<[u8]>>,
        relevant_prior_constraints: impl Into<Box<[u8]>>,
        scoring_labels: impl Into<Box<[u8]>>,
    ) -> Result<Self, SyntheticPilotError> {
        let prompt = prompt.into();
        let situation = situation.into();
        let relevant_prior_constraints = relevant_prior_constraints.into();
        let scoring_labels = scoring_labels.into();
        require_bytes(&prompt, "task_prompt")?;
        require_bytes(&situation, "task_situation")?;
        require_bytes(&relevant_prior_constraints, "relevant_prior_constraints")?;
        require_bytes(&scoring_labels, "task_scoring_labels")?;
        Ok(Self {
            id,
            source_attempt,
            prompt,
            situation,
            relevant_prior_constraints,
            scoring_labels,
        })
    }

    /// Returns the task identifier.
    #[must_use]
    pub const fn id(&self) -> PilotTaskId {
        self.id
    }

    /// Returns the sole accepted generation attempt that produced the task.
    #[must_use]
    pub const fn source_attempt(&self) -> GenerationAttemptId {
        self.source_attempt
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.u64(self.id.get());
        encoder.u64(self.source_attempt.get());
        encoder.bytes(&self.prompt);
        encoder.bytes(&self.situation);
        encoder.bytes(&self.relevant_prior_constraints);
        encoder.bytes(&self.scoring_labels);
    }
}

/// Frozen generated pilot corpus, distinct from any formal evaluation corpus.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotCorpusV1 {
    generation_log_root: PilotRoot,
    tasks: Box<[GeneratedPilotTaskV1]>,
    root: PilotRoot,
}

impl PilotCorpusV1 {
    /// Constructs a corpus whose accepted attempts and tasks match one-to-one.
    pub fn new(
        mut tasks: Vec<GeneratedPilotTaskV1>,
        generation_log: &GenerationLogV1,
    ) -> Result<Self, SyntheticPilotError> {
        if tasks.is_empty() {
            return Err(SyntheticPilotError::EmptyField("pilot_tasks"));
        }
        tasks.sort_by_key(GeneratedPilotTaskV1::id);
        for pair in tasks.windows(2) {
            if pair[0].id() == pair[1].id() {
                return Err(SyntheticPilotError::DuplicateTask(pair[0].id()));
            }
        }
        let by_task: BTreeMap<_, _> = tasks.iter().map(|task| (task.id(), task)).collect();
        let mut accepted_attempts = BTreeSet::new();
        for attempt in generation_log.attempts() {
            if let GenerationAttemptDispositionV1::Accepted(task_id) = attempt.disposition() {
                let Some(task) = by_task.get(task_id) else {
                    return Err(SyntheticPilotError::AcceptedAttemptMismatch(attempt.id()));
                };
                if task.source_attempt() != attempt.id() || !accepted_attempts.insert(attempt.id())
                {
                    return Err(SyntheticPilotError::AcceptedAttemptMismatch(attempt.id()));
                }
            }
        }
        for task in &tasks {
            if !accepted_attempts.contains(&task.source_attempt()) {
                return Err(SyntheticPilotError::TaskProvenanceMismatch(task.id()));
            }
        }
        let mut encoder = Encoder::new();
        encoder.bytes(generation_log.root().as_bytes());
        encoder.u64(tasks.len() as u64);
        for task in &tasks {
            task.encode(&mut encoder);
        }
        let root = encoder.root(CORPUS_DOMAIN);
        Ok(Self {
            generation_log_root: generation_log.root(),
            tasks: tasks.into_boxed_slice(),
            root,
        })
    }

    /// Returns generated tasks in canonical identifier order.
    #[must_use]
    pub fn tasks(&self) -> &[GeneratedPilotTaskV1] {
        &self.tasks
    }

    /// Returns the corpus content root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }

    pub(super) fn contains(&self, task_id: PilotTaskId) -> bool {
        self.tasks
            .binary_search_by_key(&task_id, GeneratedPilotTaskV1::id)
            .is_ok()
    }
}

/// One of the seven pilot-only structural conditions.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PilotCondition {
    /// Original prompt without added context.
    Prompt,
    /// Prompt plus the generated situation but no prior-constraint context.
    Situation,
    /// Situation plus same-size irrelevant synthetic context.
    Placebo,
    /// Generated focus plus a neutral expectation carrier.
    Focus,
    /// The same focus plus a generated correct expectation.
    Correct,
    /// The same focus plus a generated deliberately wrong expectation.
    Wrong,
    /// The same focus plus explicit expectation abstention.
    Abstain,
}

impl PilotCondition {
    pub(super) const ALL: [Self; 7] = [
        Self::Prompt,
        Self::Situation,
        Self::Placebo,
        Self::Focus,
        Self::Correct,
        Self::Wrong,
        Self::Abstain,
    ];

    /// Returns all pilot conditions in canonical order.
    #[must_use]
    pub const fn all() -> &'static [Self; 7] {
        &Self::ALL
    }

    /// Returns the explicitly pilot-scoped stable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Prompt => "pilot_prompt",
            Self::Situation => "pilot_situation",
            Self::Placebo => "pilot_placebo",
            Self::Focus => "pilot_focus",
            Self::Correct => "pilot_correct",
            Self::Wrong => "pilot_wrong",
            Self::Abstain => "pilot_abstain",
        }
    }

    pub(super) const fn tag(self) -> u8 {
        self as u8 + 1
    }

    const fn has_attention(self) -> bool {
        !matches!(self, Self::Prompt | Self::Situation)
    }
}

/// Exact bytes and token count for one task-condition input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotConditionArtifactV1 {
    task_id: PilotTaskId,
    condition: PilotCondition,
    exact_input: Box<[u8]>,
    attention_token_count: Option<u32>,
}

impl PilotConditionArtifactV1 {
    /// Constructs one exact pilot-only condition artifact.
    pub fn new(
        task_id: PilotTaskId,
        condition: PilotCondition,
        exact_input: impl Into<Box<[u8]>>,
        attention_token_count: Option<u32>,
    ) -> Result<Self, SyntheticPilotError> {
        let exact_input = exact_input.into();
        require_bytes(&exact_input, "condition_input")?;
        let valid_count = match (condition.has_attention(), attention_token_count) {
            (false, None) => true,
            (true, Some(count)) => count > 0,
            _ => false,
        };
        if !valid_count {
            return Err(SyntheticPilotError::AttentionTokenMismatch(task_id));
        }
        Ok(Self {
            task_id,
            condition,
            exact_input,
            attention_token_count,
        })
    }

    /// Returns the task identifier.
    #[must_use]
    pub const fn task_id(&self) -> PilotTaskId {
        self.task_id
    }

    /// Returns the pilot-only condition.
    #[must_use]
    pub const fn condition(&self) -> PilotCondition {
        self.condition
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.u64(self.task_id.get());
        encoder.byte(self.condition.tag());
        encoder.bytes(&self.exact_input);
        match self.attention_token_count {
            Some(count) => {
                encoder.byte(1);
                encoder.u64(u64::from(count));
            }
            None => encoder.byte(0),
        }
    }
}

/// Complete exact seven-condition artifact set for the generated corpus.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotConditionSetV1 {
    corpus_root: PilotRoot,
    artifacts: Box<[PilotConditionArtifactV1]>,
    root: PilotRoot,
}

impl PilotConditionSetV1 {
    /// Constructs a complete token-matched pilot condition set.
    pub fn new(
        mut artifacts: Vec<PilotConditionArtifactV1>,
        corpus: &PilotCorpusV1,
    ) -> Result<Self, SyntheticPilotError> {
        artifacts.sort_by_key(|artifact| (artifact.task_id(), artifact.condition()));
        for artifact in &artifacts {
            if !corpus.contains(artifact.task_id()) {
                return Err(SyntheticPilotError::UnknownConditionTask(
                    artifact.task_id(),
                ));
            }
        }
        for pair in artifacts.windows(2) {
            if (pair[0].task_id(), pair[0].condition()) == (pair[1].task_id(), pair[1].condition())
            {
                return Err(SyntheticPilotError::DuplicateConditionArtifact(
                    pair[0].task_id(),
                    pair[0].condition(),
                ));
            }
        }
        for task in corpus.tasks() {
            let mut attention_count = None;
            for condition in PilotCondition::ALL {
                let artifact = artifacts
                    .binary_search_by_key(&(task.id(), condition), |candidate| {
                        (candidate.task_id(), candidate.condition())
                    })
                    .ok()
                    .map(|index| &artifacts[index])
                    .ok_or(SyntheticPilotError::MissingConditionArtifact(
                        task.id(),
                        condition,
                    ))?;
                if let Some(count) = artifact.attention_token_count {
                    match attention_count {
                        Some(expected) if expected != count => {
                            return Err(SyntheticPilotError::AttentionTokenMismatch(task.id()));
                        }
                        None => attention_count = Some(count),
                        _ => {}
                    }
                }
            }
        }
        let mut encoder = Encoder::new();
        encoder.bytes(corpus.root().as_bytes());
        encoder.u64(artifacts.len() as u64);
        for artifact in &artifacts {
            artifact.encode(&mut encoder);
        }
        let root = encoder.root(CONDITION_SET_DOMAIN);
        Ok(Self {
            corpus_root: corpus.root(),
            artifacts: artifacts.into_boxed_slice(),
            root,
        })
    }

    /// Returns exact artifacts in canonical task-condition order.
    #[must_use]
    pub fn artifacts(&self) -> &[PilotConditionArtifactV1] {
        &self.artifacts
    }

    /// Returns the complete condition-set content root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }
}

/// Frozen deterministic scoring procedure for the pilot question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotScoringManifestV1 {
    implementation_identity: Box<str>,
    constraint_following_rule: Box<[u8]>,
    task_completion_rule: Box<[u8]>,
    unavailable_cell_rule: Box<[u8]>,
    aggregation_rule: Box<[u8]>,
    root: PilotRoot,
}

impl PilotScoringManifestV1 {
    /// Constructs a fixed deterministic scoring manifest.
    pub fn new(
        implementation_identity: impl Into<Box<str>>,
        constraint_following_rule: impl Into<Box<[u8]>>,
        task_completion_rule: impl Into<Box<[u8]>>,
        unavailable_cell_rule: impl Into<Box<[u8]>>,
        aggregation_rule: impl Into<Box<[u8]>>,
    ) -> Result<Self, SyntheticPilotError> {
        let implementation_identity = implementation_identity.into();
        let constraint_following_rule = constraint_following_rule.into();
        let task_completion_rule = task_completion_rule.into();
        let unavailable_cell_rule = unavailable_cell_rule.into();
        let aggregation_rule = aggregation_rule.into();
        require_text(&implementation_identity, "scoring_implementation_identity")?;
        require_bytes(&constraint_following_rule, "constraint_following_rule")?;
        require_bytes(&task_completion_rule, "task_completion_rule")?;
        require_bytes(&unavailable_cell_rule, "unavailable_cell_rule")?;
        require_bytes(&aggregation_rule, "aggregation_rule")?;
        let mut encoder = Encoder::new();
        encoder.text(&implementation_identity);
        encoder.bytes(&constraint_following_rule);
        encoder.bytes(&task_completion_rule);
        encoder.bytes(&unavailable_cell_rule);
        encoder.bytes(&aggregation_rule);
        let root = encoder.root(SCORING_MANIFEST_DOMAIN);
        Ok(Self {
            implementation_identity,
            constraint_following_rule,
            task_completion_rule,
            unavailable_cell_rule,
            aggregation_rule,
            root,
        })
    }

    /// Returns the scoring-manifest content root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }
}

/// Exact implementation and environment identity of a controlled pilot runner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotRunnerIdentityV1 {
    source_commit: Box<str>,
    implementation_identity: Box<str>,
    runtime_environment_identity: Box<str>,
    outcome_sink_identity: Box<str>,
}

impl PilotRunnerIdentityV1 {
    /// Constructs one complete controlled-runner identity.
    pub fn new(
        source_commit: impl Into<Box<str>>,
        implementation_identity: impl Into<Box<str>>,
        runtime_environment_identity: impl Into<Box<str>>,
        outcome_sink_identity: impl Into<Box<str>>,
    ) -> Result<Self, SyntheticPilotError> {
        let value = Self {
            source_commit: source_commit.into(),
            implementation_identity: implementation_identity.into(),
            runtime_environment_identity: runtime_environment_identity.into(),
            outcome_sink_identity: outcome_sink_identity.into(),
        };
        require_text(&value.source_commit, "runner_source_commit")?;
        require_text(
            &value.implementation_identity,
            "runner_implementation_identity",
        )?;
        require_text(
            &value.runtime_environment_identity,
            "runner_environment_identity",
        )?;
        require_text(&value.outcome_sink_identity, "outcome_sink_identity")?;
        Ok(value)
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.text(&self.source_commit);
        encoder.text(&self.implementation_identity);
        encoder.text(&self.runtime_environment_identity);
        encoder.text(&self.outcome_sink_identity);
    }
}

/// Frozen controlled-runner configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotRunnerManifestV1 {
    identity: PilotRunnerIdentityV1,
    condition_order: Box<[PilotCondition]>,
    seeds: Box<[u64]>,
    isolation_policy: Box<[u8]>,
    failure_capture_policy: Box<[u8]>,
    root: PilotRoot,
}

impl PilotRunnerManifestV1 {
    /// Constructs a runner manifest with a complete order and fixed seed schedule.
    pub fn new(
        identity: PilotRunnerIdentityV1,
        condition_order: Vec<PilotCondition>,
        mut seeds: Vec<u64>,
        isolation_policy: impl Into<Box<[u8]>>,
        failure_capture_policy: impl Into<Box<[u8]>>,
    ) -> Result<Self, SyntheticPilotError> {
        let isolation_policy = isolation_policy.into();
        let failure_capture_policy = failure_capture_policy.into();
        require_bytes(&isolation_policy, "runner_isolation_policy")?;
        require_bytes(&failure_capture_policy, "runner_failure_capture_policy")?;
        let condition_set: BTreeSet<_> = condition_order.iter().copied().collect();
        if condition_order.len() != PilotCondition::ALL.len()
            || condition_set.len() != PilotCondition::ALL.len()
            || !PilotCondition::ALL
                .iter()
                .all(|condition| condition_set.contains(condition))
        {
            return Err(SyntheticPilotError::InvalidConditionOrder);
        }
        if seeds.is_empty() {
            return Err(SyntheticPilotError::InvalidSeedSchedule);
        }
        seeds.sort_unstable();
        if seeds.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(SyntheticPilotError::InvalidSeedSchedule);
        }
        let mut encoder = Encoder::new();
        identity.encode(&mut encoder);
        encoder.u64(condition_order.len() as u64);
        for condition in &condition_order {
            encoder.byte(condition.tag());
        }
        encoder.u64(seeds.len() as u64);
        for seed in &seeds {
            encoder.u64(*seed);
        }
        encoder.bytes(&isolation_policy);
        encoder.bytes(&failure_capture_policy);
        encoder.text("outcome_driven_regeneration=forbidden");
        let root = encoder.root(RUNNER_MANIFEST_DOMAIN);
        Ok(Self {
            identity,
            condition_order: condition_order.into_boxed_slice(),
            seeds: seeds.into_boxed_slice(),
            isolation_policy,
            failure_capture_policy,
            root,
        })
    }

    /// Returns the frozen execution seeds in canonical numeric order.
    #[must_use]
    pub fn seeds(&self) -> &[u64] {
        &self.seeds
    }

    /// Returns the runner-manifest content root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }
}

/// Immutable pre-outcome package for one internal synthetic pilot.
#[derive(Debug, Eq, PartialEq)]
pub struct FrozenSyntheticPilotV1 {
    generation: GenerationManifestV1,
    generation_log: GenerationLogV1,
    corpus: PilotCorpusV1,
    conditions: PilotConditionSetV1,
    scoring: PilotScoringManifestV1,
    runner: PilotRunnerManifestV1,
    root: PilotRoot,
}

impl FrozenSyntheticPilotV1 {
    /// Freezes every exactly joined generation, corpus, condition, scoring, and runner root.
    pub fn freeze(
        generation: GenerationManifestV1,
        generation_log: GenerationLogV1,
        corpus: PilotCorpusV1,
        conditions: PilotConditionSetV1,
        scoring: PilotScoringManifestV1,
        runner: PilotRunnerManifestV1,
    ) -> Result<Self, SyntheticPilotError> {
        if generation_log.generation_manifest_root != generation.root()
            || corpus.generation_log_root != generation_log.root()
        {
            return Err(SyntheticPilotError::CorpusGenerationLogMismatch);
        }
        if conditions.corpus_root != corpus.root() {
            return Err(SyntheticPilotError::ConditionCorpusMismatch);
        }
        let mut encoder = Encoder::new();
        generation.encode_root(&mut encoder);
        encoder.bytes(generation_log.root().as_bytes());
        encoder.bytes(corpus.root().as_bytes());
        encoder.bytes(conditions.root().as_bytes());
        encoder.bytes(scoring.root().as_bytes());
        encoder.bytes(runner.root().as_bytes());
        encoder.text("InternalSyntheticPilot");
        encoder.text("NonPromotional");
        encoder.text("outcome_driven_regeneration=forbidden");
        let root = encoder.root(FROZEN_PILOT_DOMAIN);
        Ok(Self {
            generation,
            generation_log,
            corpus,
            conditions,
            scoring,
            runner,
            root,
        })
    }

    /// Returns the fixed evidence-class label.
    #[must_use]
    pub const fn evidence_class(&self) -> &'static str {
        "InternalSyntheticPilot"
    }

    /// Returns the fixed non-promotional label.
    #[must_use]
    pub const fn promotion_status(&self) -> &'static str {
        "NonPromotional"
    }

    /// Returns the complete pre-outcome pilot root.
    #[must_use]
    pub const fn root(&self) -> PilotRoot {
        self.root
    }

    /// Returns the frozen generation manifest.
    #[must_use]
    pub const fn generation(&self) -> &GenerationManifestV1 {
        &self.generation
    }

    /// Returns the retained generation log.
    #[must_use]
    pub const fn generation_log(&self) -> &GenerationLogV1 {
        &self.generation_log
    }

    /// Returns the frozen generated corpus.
    #[must_use]
    pub const fn corpus(&self) -> &PilotCorpusV1 {
        &self.corpus
    }

    /// Returns the exact seven-condition artifacts.
    #[must_use]
    pub const fn conditions(&self) -> &PilotConditionSetV1 {
        &self.conditions
    }

    /// Returns the fixed scoring manifest.
    #[must_use]
    pub const fn scoring(&self) -> &PilotScoringManifestV1 {
        &self.scoring
    }

    /// Returns the controlled-runner manifest.
    #[must_use]
    pub const fn runner(&self) -> &PilotRunnerManifestV1 {
        &self.runner
    }
}
