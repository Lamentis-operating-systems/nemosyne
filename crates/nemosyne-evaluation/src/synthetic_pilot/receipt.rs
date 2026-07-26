use std::collections::BTreeSet;

use super::{
    FrozenSyntheticPilotV1, PilotCondition, PilotRoot, PilotTaskId, SyntheticPilotError,
    canonical::Encoder,
};

const OBSERVATION_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.observations.v1";
const RECEIPT_DOMAIN: &[u8] = b"nemosyne.internal-synthetic-pilot.receipt.v1";

/// Result retained for one task-condition-seed cell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PilotCellResultV1 {
    /// The frozen scorer produced both descriptive binary observations.
    Scored {
        /// Whether the agent followed the relevant prior constraint.
        constraint_followed: bool,
        /// Whether the agent completed the generated task.
        task_completed: bool,
        /// Exact retained scorer and runner observation bytes.
        exact_observation: Box<[u8]>,
    },
    /// The cell could not be scored under the frozen procedure.
    Unavailable {
        /// Exact retained unavailability reason.
        reason: Box<str>,
        /// Exact retained runner output, which may be empty.
        exact_output: Box<[u8]>,
    },
}

impl PilotCellResultV1 {
    fn validate(&self) -> Result<(), SyntheticPilotError> {
        match self {
            Self::Scored {
                exact_observation, ..
            } if exact_observation.is_empty() => {
                Err(SyntheticPilotError::EmptyField("cell_observation"))
            }
            Self::Unavailable { reason, .. } if reason.trim().is_empty() => {
                Err(SyntheticPilotError::EmptyField("cell_unavailable_reason"))
            }
            _ => Ok(()),
        }
    }

    fn encode(&self, encoder: &mut Encoder) {
        match self {
            Self::Scored {
                constraint_followed,
                task_completed,
                exact_observation,
            } => {
                encoder.byte(1);
                encoder.byte(u8::from(*constraint_followed));
                encoder.byte(u8::from(*task_completed));
                encoder.bytes(exact_observation);
            }
            Self::Unavailable {
                reason,
                exact_output,
            } => {
                encoder.byte(2);
                encoder.text(reason);
                encoder.bytes(exact_output);
            }
        }
    }
}

/// One exact task-condition-seed observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotObservationV1 {
    task_id: PilotTaskId,
    condition: PilotCondition,
    seed: u64,
    result: PilotCellResultV1,
}

impl PilotObservationV1 {
    /// Constructs one retained pilot observation.
    pub fn new(
        task_id: PilotTaskId,
        condition: PilotCondition,
        seed: u64,
        result: PilotCellResultV1,
    ) -> Result<Self, SyntheticPilotError> {
        result.validate()?;
        Ok(Self {
            task_id,
            condition,
            seed,
            result,
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

    /// Returns the frozen execution seed.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the retained cell result.
    #[must_use]
    pub const fn result(&self) -> &PilotCellResultV1 {
        &self.result
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.u64(self.task_id.get());
        encoder.byte(self.condition.tag());
        encoder.u64(self.seed);
        self.result.encode(encoder);
    }
}

/// Terminal non-promotional receipt disposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyntheticPilotDisposition {
    /// Every required cell was scored under the frozen procedure.
    Completed,
    /// Execution or scoring violated the frozen pilot procedure.
    Invalid,
    /// The pilot stopped before descriptive completion.
    Aborted,
}

impl SyntheticPilotDisposition {
    const fn tag(self) -> u8 {
        match self {
            Self::Completed => 1,
            Self::Invalid => 2,
            Self::Aborted => 3,
        }
    }
}

/// Descriptive counts for one pilot-only condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PilotConditionSummaryV1 {
    condition: PilotCondition,
    scored_cells: usize,
    constraint_followed_cells: usize,
    task_completed_cells: usize,
    unavailable_cells: usize,
}

impl PilotConditionSummaryV1 {
    /// Returns the summarized pilot condition.
    #[must_use]
    pub const fn condition(self) -> PilotCondition {
        self.condition
    }

    /// Returns the number of scored cells.
    #[must_use]
    pub const fn scored_cells(self) -> usize {
        self.scored_cells
    }

    /// Returns the number of cells in which the prior constraint was followed.
    #[must_use]
    pub const fn constraint_followed_cells(self) -> usize {
        self.constraint_followed_cells
    }

    /// Returns the descriptive constraint-following rate, when defined.
    #[must_use]
    pub fn constraint_following_rate(self) -> Option<f64> {
        (self.scored_cells > 0)
            .then(|| self.constraint_followed_cells as f64 / self.scored_cells as f64)
    }

    /// Returns the number of completed task cells.
    #[must_use]
    pub const fn task_completed_cells(self) -> usize {
        self.task_completed_cells
    }

    /// Returns the number of unavailable cells.
    #[must_use]
    pub const fn unavailable_cells(self) -> usize {
        self.unavailable_cells
    }
}

/// Immutable descriptive receipt for an internal synthetic pilot.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntheticPilotReceiptV1 {
    pilot_root: PilotRoot,
    observation_root: PilotRoot,
    receipt_root: PilotRoot,
    disposition: SyntheticPilotDisposition,
    reason: Option<Box<str>>,
    summaries: Box<[PilotConditionSummaryV1]>,
}

impl SyntheticPilotReceiptV1 {
    /// Finalizes a descriptive receipt while consuming the frozen pilot.
    pub fn finalize(
        pilot: FrozenSyntheticPilotV1,
        disposition: SyntheticPilotDisposition,
        mut observations: Vec<PilotObservationV1>,
        reason: Option<impl Into<Box<str>>>,
    ) -> Result<Self, SyntheticPilotError> {
        let reason = reason.map(Into::into);
        match disposition {
            SyntheticPilotDisposition::Completed if reason.is_some() => {
                return Err(SyntheticPilotError::EmptyField(
                    "completed_receipt_must_not_have_reason",
                ));
            }
            SyntheticPilotDisposition::Invalid | SyntheticPilotDisposition::Aborted => {
                if reason
                    .as_deref()
                    .is_none_or(|value| value.trim().is_empty())
                {
                    return Err(SyntheticPilotError::EmptyField("terminal_reason"));
                }
            }
            SyntheticPilotDisposition::Completed => {}
        }
        observations.sort_by_key(|observation| {
            (
                observation.task_id(),
                observation.condition(),
                observation.seed(),
            )
        });
        for observation in &observations {
            if !pilot.corpus().contains(observation.task_id()) {
                return Err(SyntheticPilotError::UnknownObservationTask(
                    observation.task_id(),
                ));
            }
            if pilot
                .runner()
                .seeds()
                .binary_search(&observation.seed())
                .is_err()
            {
                return Err(SyntheticPilotError::UnknownObservationSeed(
                    observation.seed(),
                ));
            }
        }
        for pair in observations.windows(2) {
            let left = (pair[0].task_id(), pair[0].condition(), pair[0].seed());
            let right = (pair[1].task_id(), pair[1].condition(), pair[1].seed());
            if left == right {
                return Err(SyntheticPilotError::DuplicateObservation(
                    left.0, left.1, left.2,
                ));
            }
        }
        if disposition == SyntheticPilotDisposition::Completed {
            let observed: BTreeSet<_> = observations
                .iter()
                .map(|item| (item.task_id(), item.condition(), item.seed()))
                .collect();
            for task in pilot.corpus().tasks() {
                for condition in PilotCondition::all() {
                    for seed in pilot.runner().seeds() {
                        if !observed.contains(&(task.id(), *condition, *seed)) {
                            return Err(SyntheticPilotError::MissingObservation(
                                task.id(),
                                *condition,
                                *seed,
                            ));
                        }
                    }
                }
            }
            for observation in &observations {
                if matches!(observation.result(), PilotCellResultV1::Unavailable { .. }) {
                    return Err(SyntheticPilotError::UnavailableCompletedCell(
                        observation.task_id(),
                        observation.condition(),
                        observation.seed(),
                    ));
                }
            }
        }
        let mut observation_encoder = Encoder::new();
        observation_encoder.bytes(pilot.root().as_bytes());
        observation_encoder.u64(observations.len() as u64);
        for observation in &observations {
            observation.encode(&mut observation_encoder);
        }
        let observation_root = observation_encoder.root(OBSERVATION_DOMAIN);
        let mut summaries = Vec::with_capacity(PilotCondition::all().len());
        for condition in PilotCondition::all() {
            let mut summary = PilotConditionSummaryV1 {
                condition: *condition,
                scored_cells: 0,
                constraint_followed_cells: 0,
                task_completed_cells: 0,
                unavailable_cells: 0,
            };
            for observation in observations
                .iter()
                .filter(|item| item.condition() == *condition)
            {
                match observation.result() {
                    PilotCellResultV1::Scored {
                        constraint_followed,
                        task_completed,
                        ..
                    } => {
                        summary.scored_cells += 1;
                        summary.constraint_followed_cells += usize::from(*constraint_followed);
                        summary.task_completed_cells += usize::from(*task_completed);
                    }
                    PilotCellResultV1::Unavailable { .. } => summary.unavailable_cells += 1,
                }
            }
            summaries.push(summary);
        }
        let mut receipt_encoder = Encoder::new();
        receipt_encoder.text("InternalSyntheticPilot");
        receipt_encoder.text("NonPromotional");
        receipt_encoder.bytes(pilot.root().as_bytes());
        receipt_encoder.bytes(observation_root.as_bytes());
        receipt_encoder.byte(disposition.tag());
        match &reason {
            Some(value) => {
                receipt_encoder.byte(1);
                receipt_encoder.text(value);
            }
            None => receipt_encoder.byte(0),
        }
        for summary in &summaries {
            receipt_encoder.byte(summary.condition.tag());
            receipt_encoder.u64(summary.scored_cells as u64);
            receipt_encoder.u64(summary.constraint_followed_cells as u64);
            receipt_encoder.u64(summary.task_completed_cells as u64);
            receipt_encoder.u64(summary.unavailable_cells as u64);
        }
        let receipt_root = receipt_encoder.root(RECEIPT_DOMAIN);
        Ok(Self {
            pilot_root: pilot.root(),
            observation_root,
            receipt_root,
            disposition,
            reason,
            summaries: summaries.into_boxed_slice(),
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

    /// Returns the terminal descriptive disposition.
    #[must_use]
    pub const fn disposition(&self) -> SyntheticPilotDisposition {
        self.disposition
    }

    /// Returns the exact terminal reason for invalid or aborted receipts.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Returns the frozen pre-outcome pilot root.
    #[must_use]
    pub const fn pilot_root(&self) -> PilotRoot {
        self.pilot_root
    }

    /// Returns the retained observation root.
    #[must_use]
    pub const fn observation_root(&self) -> PilotRoot {
        self.observation_root
    }

    /// Returns the complete receipt root.
    #[must_use]
    pub const fn receipt_root(&self) -> PilotRoot {
        self.receipt_root
    }

    /// Returns canonical descriptive summaries for all seven conditions.
    #[must_use]
    pub fn summaries(&self) -> &[PilotConditionSummaryV1] {
        &self.summaries
    }
}
