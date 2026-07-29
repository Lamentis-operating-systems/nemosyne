use std::collections::BTreeSet;
use std::fmt;

const MAX_IDENTIFIER_BYTES: usize = 64;
const MAX_CANDIDATES: usize = 32;
const MAX_FOCUS_ITEMS: usize = 16;
const MAX_QUALIFIER_BYTES: usize = 128;

/// A bounded, non-empty identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Identifier(String);

impl Identifier {
    /// Constructs an identifier after checking its finite representation.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(Error::EmptyIdentifier);
        }
        if value.len() > MAX_IDENTIFIER_BYTES {
            return Err(Error::IdentifierTooLong);
        }
        Ok(Self(value))
    }

    /// Returns the identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An optional bounded project cycle.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CycleId(Identifier);

impl CycleId {
    /// Constructs a cycle identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        Ok(Self(Identifier::new(value)?))
    }
}

/// Closed record lifecycle state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RecordState {
    /// The record may be considered.
    Active,
    /// The record is outside the active set.
    Inactive,
}

/// Closed validity state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Validity {
    /// The record is valid for the query.
    Valid,
    /// The record is known to be invalid.
    Invalid,
}

/// Closed replacement state.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Replacement {
    /// The record has not been replaced.
    Current,
    /// The record was replaced by the identified record.
    ReplacedBy(Identifier),
}

/// Bounded authority level. Larger values have greater authority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Authority(u8);

impl Authority {
    /// Constructs an authority level in `0..=15`.
    pub fn new(value: u8) -> Result<Self, Error> {
        if value > 15 {
            return Err(Error::AuthorityOutOfRange);
        }
        Ok(Self(value))
    }
}

/// Bounded priority. Larger values have greater priority within one authority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Priority(u8);

impl Priority {
    /// Constructs a priority in `0..=15`.
    pub fn new(value: u8) -> Result<Self, Error> {
        if value > 15 {
            return Err(Error::PriorityOutOfRange);
        }
        Ok(Self(value))
    }
}

/// Exact subject and project scope requested by a query.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QueryScope {
    /// Exact subject identifier.
    pub subject: Identifier,
    /// Exact project identifier.
    pub project: Identifier,
    /// Optional exact cycle.
    pub cycle: Option<CycleId>,
}

/// Structured controls evaluated before focus readiness.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApplicabilityControls {
    /// Exact subject identifier.
    pub subject: Identifier,
    /// Exact project identifier.
    pub project: Identifier,
    /// Record lifecycle state.
    pub state: RecordState,
    /// Positive revision number.
    pub revision: u32,
    /// Optional exact cycle.
    pub cycle: Option<CycleId>,
    /// Record validity.
    pub validity: Validity,
    /// Replacement state.
    pub replacement: Replacement,
    /// Authority level.
    pub authority: Authority,
    /// Priority within the authority level.
    pub priority: Priority,
}

/// Role of a typed focus item.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FocusRole {
    /// A relevant fact-like constraint.
    Constraint,
    /// A relevant current goal.
    Goal,
    /// A relevant unresolved question.
    OpenQuestion,
}

/// A bounded support handle without source text or provenance.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SupportHandle(Identifier);

impl SupportHandle {
    /// Constructs a support handle.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        Ok(Self(Identifier::new(value)?))
    }
}

/// One prose-free typed focus item.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FocusItem {
    /// Semantic role.
    pub role: FocusRole,
    /// Opaque support handle.
    pub support: SupportHandle,
    /// Optional bounded qualifier.
    pub qualifier: Option<String>,
}

/// A bounded candidate presented to the applicability boundary.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Candidate {
    /// Stable synthetic candidate identifier.
    pub id: Identifier,
    /// Structured applicability controls.
    pub controls: ApplicabilityControls,
    /// Proposed typed focus items.
    pub items: Vec<FocusItem>,
}

/// A valid, canonically ordered, prose-free focus structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusStructure {
    items: Vec<FocusItem>,
    authority: Authority,
}

impl FocusStructure {
    /// Constructs and validates a focus structure.
    pub fn new(mut items: Vec<FocusItem>, authority: Authority) -> Result<Self, Error> {
        validate_items(&items)?;
        items.sort();
        if items.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(Error::DuplicateFocusItem);
        }
        Ok(Self { items, authority })
    }

    /// Returns canonically ordered items.
    pub fn items(&self) -> &[FocusItem] {
        &self.items
    }

    /// Returns the authority ceiling inherited from applicability.
    pub fn authority(&self) -> Authority {
        self.authority
    }
}

/// Closed applicability outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicabilityOutcome {
    /// Exactly one maximal semantic candidate is applicable.
    Applicable(FocusStructure),
    /// No candidate survives the structured boundary.
    Abstain,
    /// Different maximal candidates remain unresolved.
    Conflict,
}

/// Closed readiness outcome preserving terminal applicability outcomes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadinessOutcome {
    /// A valid typed structure is ready for an optional later renderer.
    Ready(FocusStructure),
    /// Terminal abstention.
    Abstain,
    /// Terminal unresolved conflict.
    Conflict,
}

impl From<ApplicabilityOutcome> for ReadinessOutcome {
    fn from(value: ApplicabilityOutcome) -> Self {
        match value {
            ApplicabilityOutcome::Applicable(structure) => Self::Ready(structure),
            ApplicabilityOutcome::Abstain => Self::Abstain,
            ApplicabilityOutcome::Conflict => Self::Conflict,
        }
    }
}

/// Typed construction or evaluation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// An identifier is empty or whitespace-only.
    EmptyIdentifier,
    /// An identifier exceeds 64 UTF-8 bytes.
    IdentifierTooLong,
    /// Authority is outside `0..=15`.
    AuthorityOutOfRange,
    /// Priority is outside `0..=15`.
    PriorityOutOfRange,
    /// Revision zero is invalid.
    RevisionZero,
    /// No candidate was supplied.
    EmptyCandidates,
    /// More than 32 candidates were supplied.
    TooManyCandidates,
    /// A focus structure has no items.
    EmptyFocus,
    /// A focus structure contains more than 16 items.
    TooManyFocusItems,
    /// A qualifier exceeds 128 UTF-8 bytes.
    QualifierTooLong,
    /// A support handle occurs more than once within a candidate.
    DuplicateSupport,
    /// A canonical focus item occurs more than once.
    DuplicateFocusItem,
    /// A replacement points to the same record.
    SelfReplacement,
    /// A candidate identifier occurs more than once.
    DuplicateCandidateId,
    /// A replacement target is absent from the bounded input.
    BrokenReplacement,
    /// A baseline was requested for a terminal outcome.
    TerminalOutcome,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// Evaluates structured controls and returns a closed applicability outcome.
pub fn evaluate(
    scope: &QueryScope,
    mut candidates: Vec<Candidate>,
) -> Result<ApplicabilityOutcome, Error> {
    if candidates.is_empty() {
        return Err(Error::EmptyCandidates);
    }
    if candidates.len() > MAX_CANDIDATES {
        return Err(Error::TooManyCandidates);
    }
    candidates.sort();
    if candidates.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return Err(Error::DuplicateCandidateId);
    }
    let candidate_ids: BTreeSet<&Identifier> =
        candidates.iter().map(|candidate| &candidate.id).collect();
    for candidate in &candidates {
        validate_candidate(candidate)?;
        if matches!(
            &candidate.controls.replacement,
            Replacement::ReplacedBy(replacement) if !candidate_ids.contains(replacement)
        ) {
            return Err(Error::BrokenReplacement);
        }
    }

    let mut eligible: Vec<&Candidate> = candidates
        .iter()
        .filter(|candidate| {
            candidate.controls.subject == scope.subject
                && candidate.controls.project == scope.project
                && candidate.controls.cycle == scope.cycle
                && candidate.controls.state == RecordState::Active
                && candidate.controls.validity == Validity::Valid
                && candidate.controls.replacement == Replacement::Current
        })
        .collect();
    if eligible.is_empty() {
        return Ok(ApplicabilityOutcome::Abstain);
    }

    eligible.sort_by_key(|candidate| {
        (
            candidate.controls.authority,
            candidate.controls.priority,
            candidate.controls.revision,
        )
    });
    let maximal = eligible.last().expect("eligible was checked as non-empty");
    let rank = (
        maximal.controls.authority,
        maximal.controls.priority,
        maximal.controls.revision,
    );
    let maximal: Vec<&Candidate> = eligible
        .into_iter()
        .filter(|candidate| {
            (
                candidate.controls.authority,
                candidate.controls.priority,
                candidate.controls.revision,
            ) == rank
        })
        .collect();

    let first = FocusStructure::new(maximal[0].items.clone(), maximal[0].controls.authority)?;
    if maximal.iter().skip(1).any(|candidate| {
        FocusStructure::new(candidate.items.clone(), candidate.controls.authority)
            .is_ok_and(|structure| structure != first)
    }) {
        return Ok(ApplicabilityOutcome::Conflict);
    }
    Ok(ApplicabilityOutcome::Applicable(first))
}

/// Produces deterministic structural bytes for a ready outcome.
pub fn realize(outcome: &ReadinessOutcome) -> Result<Vec<u8>, Error> {
    let ReadinessOutcome::Ready(structure) = outcome else {
        return Err(Error::TerminalOutcome);
    };
    let mut output = format!("authority={}\n", structure.authority.0);
    for item in &structure.items {
        let role = match item.role {
            FocusRole::Constraint => "constraint",
            FocusRole::Goal => "goal",
            FocusRole::OpenQuestion => "open-question",
        };
        output.push_str(role);
        output.push('|');
        output.push_str(item.support.0.as_str());
        output.push('|');
        if let Some(qualifier) = &item.qualifier {
            output.push_str(&escape(qualifier));
        }
        output.push('\n');
    }
    Ok(output.into_bytes())
}

fn validate_candidate(candidate: &Candidate) -> Result<(), Error> {
    if candidate.controls.revision == 0 {
        return Err(Error::RevisionZero);
    }
    if matches!(
        &candidate.controls.replacement,
        Replacement::ReplacedBy(replacement) if replacement == &candidate.id
    ) {
        return Err(Error::SelfReplacement);
    }
    validate_items(&candidate.items)
}

fn validate_items(items: &[FocusItem]) -> Result<(), Error> {
    if items.is_empty() {
        return Err(Error::EmptyFocus);
    }
    if items.len() > MAX_FOCUS_ITEMS {
        return Err(Error::TooManyFocusItems);
    }
    let mut support = BTreeSet::new();
    for item in items {
        if item
            .qualifier
            .as_ref()
            .is_some_and(|qualifier| qualifier.len() > MAX_QUALIFIER_BYTES)
        {
            return Err(Error::QualifierTooLong);
        }
        if !support.insert(&item.support) {
            return Err(Error::DuplicateSupport);
        }
    }
    Ok(())
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\n', "\\n")
}
