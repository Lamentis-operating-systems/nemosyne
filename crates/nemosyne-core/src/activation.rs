//! Deterministic activation scoring and ranking for normalized numeric signals.

use std::error::Error;
use std::fmt;

/// A numeric identifier for an activation candidate.
///
/// The value is opaque. Zero has no special meaning.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CandidateId(u64);

impl CandidateId {
    /// Creates an identifier from its numeric value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// A numeric identifier for an evidence or inhibition channel.
///
/// Channel identifiers share one namespace across both channel categories. The
/// value is opaque, and zero has no special meaning.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChannelId(u64);

impl ChannelId {
    /// Creates an identifier from its numeric value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// A finite numeric value in the closed interval from zero to one.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UnitInterval(f64);

impl UnitInterval {
    /// Creates a validated unit-interval value.
    ///
    /// Negative zero is accepted and stored as positive zero.
    ///
    /// # Errors
    ///
    /// Returns [`ActivationError::InvalidUnitInterval`] when `value` is not
    /// finite or lies outside the closed interval `[0, 1]`.
    pub fn new(value: f64) -> Result<Self, ActivationError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ActivationError::InvalidUnitInterval { value });
        }

        Ok(Self(canonical_zero(value)))
    }

    /// Returns the validated numeric value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    fn from_computed(value: f64) -> Self {
        debug_assert!(value.is_finite());
        Self(canonical_zero(value.clamp(0.0, 1.0)))
    }
}

/// A normalized signal associated with one channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChannelSignal {
    channel_id: ChannelId,
    value: UnitInterval,
}

impl ChannelSignal {
    /// Creates a signal for `channel_id`.
    #[must_use]
    pub const fn new(channel_id: ChannelId, value: UnitInterval) -> Self {
        Self { channel_id, value }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the normalized signal value.
    #[must_use]
    pub const fn value(&self) -> UnitInterval {
        self.value
    }
}

/// Configuration for one positive-evidence channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvidenceChannel {
    channel_id: ChannelId,
    weight: UnitInterval,
    gate: UnitInterval,
}

impl EvidenceChannel {
    /// Creates an evidence-channel configuration.
    #[must_use]
    pub const fn new(channel_id: ChannelId, weight: UnitInterval, gate: UnitInterval) -> Self {
        Self {
            channel_id,
            weight,
            gate,
        }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the base channel weight.
    #[must_use]
    pub const fn weight(&self) -> UnitInterval {
        self.weight
    }

    /// Returns the situation-dependent gate.
    #[must_use]
    pub const fn gate(&self) -> UnitInterval {
        self.gate
    }
}

/// Configuration for one inhibition channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InhibitionChannel {
    channel_id: ChannelId,
    strength: UnitInterval,
}

impl InhibitionChannel {
    /// Creates an inhibition-channel configuration.
    #[must_use]
    pub const fn new(channel_id: ChannelId, strength: UnitInterval) -> Self {
        Self {
            channel_id,
            strength,
        }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the inhibition strength.
    #[must_use]
    pub const fn strength(&self) -> UnitInterval {
        self.strength
    }
}

/// The channel configuration used for one activation run.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationProfile {
    evidence_channels: Vec<EvidenceChannel>,
    inhibition_channels: Vec<InhibitionChannel>,
    evidence_denominator: f64,
}

impl ActivationProfile {
    /// Creates and canonicalizes an activation profile.
    ///
    /// Channels are stored in ascending identifier order. Channel identifiers
    /// must be unique across evidence and inhibition channels, and at least one
    /// evidence channel must have a positive effective weight.
    ///
    /// # Errors
    ///
    /// Returns an error for a duplicate channel identifier or when every
    /// evidence channel has an effective weight of zero.
    pub fn new(
        mut evidence_channels: Vec<EvidenceChannel>,
        mut inhibition_channels: Vec<InhibitionChannel>,
    ) -> Result<Self, ActivationError> {
        evidence_channels.sort_unstable_by_key(EvidenceChannel::channel_id);
        inhibition_channels.sort_unstable_by_key(InhibitionChannel::channel_id);

        if let Some(channel_id) = first_duplicate_channel(
            evidence_channels
                .iter()
                .map(EvidenceChannel::channel_id)
                .chain(
                    inhibition_channels
                        .iter()
                        .map(InhibitionChannel::channel_id),
                ),
        ) {
            return Err(ActivationError::DuplicateProfileChannel { channel_id });
        }

        let evidence_denominator = evidence_channels.iter().fold(0.0, |sum, channel| {
            sum + channel.weight.get() * channel.gate.get()
        });

        if evidence_denominator == 0.0 {
            return Err(ActivationError::NoEffectiveEvidence);
        }

        Ok(Self {
            evidence_channels,
            inhibition_channels,
            evidence_denominator,
        })
    }

    /// Returns evidence channels in ascending identifier order.
    #[must_use]
    pub fn evidence_channels(&self) -> &[EvidenceChannel] {
        &self.evidence_channels
    }

    /// Returns inhibition channels in ascending identifier order.
    #[must_use]
    pub fn inhibition_channels(&self) -> &[InhibitionChannel] {
        &self.inhibition_channels
    }
}

/// One candidate and its complete normalized signal set.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationCandidate {
    candidate_id: CandidateId,
    evidence_signals: Vec<ChannelSignal>,
    inhibition_signals: Vec<ChannelSignal>,
}

impl ActivationCandidate {
    /// Creates and canonicalizes a candidate.
    ///
    /// Signals are stored in ascending channel-identifier order. Channel
    /// identifiers must be unique across evidence and inhibition signals.
    /// Exact correspondence with a profile is checked by
    /// [`rank_activations`].
    ///
    /// # Errors
    ///
    /// Returns an error when a channel identifier occurs more than once.
    pub fn new(
        candidate_id: CandidateId,
        mut evidence_signals: Vec<ChannelSignal>,
        mut inhibition_signals: Vec<ChannelSignal>,
    ) -> Result<Self, ActivationError> {
        evidence_signals.sort_unstable_by_key(ChannelSignal::channel_id);
        inhibition_signals.sort_unstable_by_key(ChannelSignal::channel_id);

        if let Some(channel_id) = first_duplicate_channel(
            evidence_signals
                .iter()
                .map(ChannelSignal::channel_id)
                .chain(inhibition_signals.iter().map(ChannelSignal::channel_id)),
        ) {
            return Err(ActivationError::DuplicateCandidateChannel {
                candidate_id,
                channel_id,
            });
        }

        Ok(Self {
            candidate_id,
            evidence_signals,
            inhibition_signals,
        })
    }

    /// Returns the candidate identifier.
    #[must_use]
    pub const fn candidate_id(&self) -> CandidateId {
        self.candidate_id
    }

    /// Returns evidence signals in ascending channel-identifier order.
    #[must_use]
    pub fn evidence_signals(&self) -> &[ChannelSignal] {
        &self.evidence_signals
    }

    /// Returns inhibition signals in ascending channel-identifier order.
    #[must_use]
    pub fn inhibition_signals(&self) -> &[ChannelSignal] {
        &self.inhibition_signals
    }
}

/// The contribution of one evidence channel to a candidate's evidence score.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvidenceContribution {
    channel_id: ChannelId,
    weight: UnitInterval,
    gate: UnitInterval,
    signal: UnitInterval,
    effective_weight: UnitInterval,
    contribution: UnitInterval,
}

impl EvidenceContribution {
    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the base channel weight.
    #[must_use]
    pub const fn weight(&self) -> UnitInterval {
        self.weight
    }

    /// Returns the situation-dependent gate.
    #[must_use]
    pub const fn gate(&self) -> UnitInterval {
        self.gate
    }

    /// Returns the candidate's channel signal.
    #[must_use]
    pub const fn signal(&self) -> UnitInterval {
        self.signal
    }

    /// Returns the product of weight and gate.
    #[must_use]
    pub const fn effective_weight(&self) -> UnitInterval {
        self.effective_weight
    }

    /// Returns the channel's normalized contribution to the evidence score.
    #[must_use]
    pub const fn contribution(&self) -> UnitInterval {
        self.contribution
    }
}

/// The retention effect of one inhibition channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InhibitionContribution {
    channel_id: ChannelId,
    strength: UnitInterval,
    signal: UnitInterval,
    retention_factor: UnitInterval,
}

impl InhibitionContribution {
    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the configured inhibition strength.
    #[must_use]
    pub const fn strength(&self) -> UnitInterval {
        self.strength
    }

    /// Returns the candidate's channel signal.
    #[must_use]
    pub const fn signal(&self) -> UnitInterval {
        self.signal
    }

    /// Returns the channel's multiplicative retention factor.
    #[must_use]
    pub const fn retention_factor(&self) -> UnitInterval {
        self.retention_factor
    }
}

/// A complete explanation of one candidate's activation score.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationBreakdown {
    evidence_score: UnitInterval,
    retention: UnitInterval,
    score: UnitInterval,
    evidence_contributions: Vec<EvidenceContribution>,
    inhibition_contributions: Vec<InhibitionContribution>,
}

impl ActivationBreakdown {
    /// Returns the weighted positive-evidence score.
    #[must_use]
    pub const fn evidence_score(&self) -> UnitInterval {
        self.evidence_score
    }

    /// Returns the combined multiplicative retention.
    #[must_use]
    pub const fn retention(&self) -> UnitInterval {
        self.retention
    }

    /// Returns the final activation score.
    #[must_use]
    pub const fn score(&self) -> UnitInterval {
        self.score
    }

    /// Returns evidence contributions in ascending channel-identifier order.
    #[must_use]
    pub fn evidence_contributions(&self) -> &[EvidenceContribution] {
        &self.evidence_contributions
    }

    /// Returns inhibition effects in ascending channel-identifier order.
    #[must_use]
    pub fn inhibition_contributions(&self) -> &[InhibitionContribution] {
        &self.inhibition_contributions
    }
}

/// One ranked candidate and its activation breakdown.
#[derive(Clone, Debug, PartialEq)]
pub struct RankedActivation {
    candidate_id: CandidateId,
    breakdown: ActivationBreakdown,
}

impl RankedActivation {
    /// Returns the candidate identifier.
    #[must_use]
    pub const fn candidate_id(&self) -> CandidateId {
        self.candidate_id
    }

    /// Returns the candidate's complete score breakdown.
    #[must_use]
    pub const fn breakdown(&self) -> &ActivationBreakdown {
        &self.breakdown
    }
}

/// An invalid activation input or incompatible candidate/profile structure.
#[derive(Clone, Debug, PartialEq)]
pub enum ActivationError {
    /// A numeric input is non-finite or outside `[0, 1]`.
    InvalidUnitInterval {
        /// The rejected numeric value.
        value: f64,
    },
    /// A profile defines a channel identifier more than once.
    DuplicateProfileChannel {
        /// The duplicated channel identifier.
        channel_id: ChannelId,
    },
    /// No evidence channel has a positive effective weight.
    NoEffectiveEvidence,
    /// A candidate supplies a channel identifier more than once.
    DuplicateCandidateChannel {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The duplicated channel identifier.
        channel_id: ChannelId,
    },
    /// A candidate identifier occurs more than once in one ranking request.
    DuplicateCandidate {
        /// The duplicated candidate identifier.
        candidate_id: CandidateId,
    },
    /// A candidate omits an evidence signal required by the profile.
    MissingEvidenceSignal {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The missing channel identifier.
        channel_id: ChannelId,
    },
    /// A candidate supplies an evidence signal not defined by the profile.
    UnexpectedEvidenceSignal {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The unexpected channel identifier.
        channel_id: ChannelId,
    },
    /// A candidate omits an inhibition signal required by the profile.
    MissingInhibitionSignal {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The missing channel identifier.
        channel_id: ChannelId,
    },
    /// A candidate supplies an inhibition signal not defined by the profile.
    UnexpectedInhibitionSignal {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The unexpected channel identifier.
        channel_id: ChannelId,
    },
}

impl fmt::Display for ActivationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUnitInterval { value } => {
                write!(formatter, "value {value:?} is not finite or outside [0, 1]")
            }
            Self::DuplicateProfileChannel { channel_id } => write!(
                formatter,
                "profile channel {} is defined more than once",
                channel_id.get()
            ),
            Self::NoEffectiveEvidence => {
                formatter.write_str("profile has no positive effective evidence weight")
            }
            Self::DuplicateCandidateChannel {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} supplies channel {} more than once",
                candidate_id.get(),
                channel_id.get()
            ),
            Self::DuplicateCandidate { candidate_id } => write!(
                formatter,
                "candidate {} occurs more than once",
                candidate_id.get()
            ),
            Self::MissingEvidenceSignal {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} is missing evidence channel {}",
                candidate_id.get(),
                channel_id.get()
            ),
            Self::UnexpectedEvidenceSignal {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} supplies unexpected evidence channel {}",
                candidate_id.get(),
                channel_id.get()
            ),
            Self::MissingInhibitionSignal {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} is missing inhibition channel {}",
                candidate_id.get(),
                channel_id.get()
            ),
            Self::UnexpectedInhibitionSignal {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} supplies unexpected inhibition channel {}",
                candidate_id.get(),
                channel_id.get()
            ),
        }
    }
}

impl Error for ActivationError {}

/// Scores and ranks all candidates against one activation profile.
///
/// Every candidate must provide exactly one signal for every profile channel
/// and no other signal. Results are ordered by descending activation score and
/// then by ascending candidate identifier for exact score ties. No candidate is
/// filtered from the result.
///
/// # Errors
///
/// Returns an error for duplicate candidate identifiers or when a candidate's
/// signal channels do not exactly match the profile.
pub fn rank_activations(
    profile: &ActivationProfile,
    candidates: &[ActivationCandidate],
) -> Result<Vec<RankedActivation>, ActivationError> {
    let mut canonical_candidates: Vec<&ActivationCandidate> = candidates.iter().collect();
    canonical_candidates.sort_unstable_by_key(|candidate| candidate.candidate_id);

    if let Some(pair) = canonical_candidates
        .windows(2)
        .find(|pair| pair[0].candidate_id == pair[1].candidate_id)
    {
        return Err(ActivationError::DuplicateCandidate {
            candidate_id: pair[0].candidate_id,
        });
    }

    for candidate in &canonical_candidates {
        validate_signal_set(
            candidate.candidate_id,
            &profile.evidence_channels,
            &candidate.evidence_signals,
            SignalCategory::Evidence,
        )?;
        validate_signal_set(
            candidate.candidate_id,
            &profile.inhibition_channels,
            &candidate.inhibition_signals,
            SignalCategory::Inhibition,
        )?;
    }

    let mut ranked = Vec::with_capacity(canonical_candidates.len());
    for candidate in canonical_candidates {
        ranked.push(score_candidate(profile, candidate));
    }

    ranked.sort_unstable_by(|left, right| {
        right
            .breakdown
            .score
            .get()
            .total_cmp(&left.breakdown.score.get())
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
    });

    Ok(ranked)
}

fn score_candidate(
    profile: &ActivationProfile,
    candidate: &ActivationCandidate,
) -> RankedActivation {
    let mut evidence_contributions = Vec::with_capacity(profile.evidence_channels.len());
    let mut evidence_score = 0.0;

    for (channel, signal) in profile
        .evidence_channels
        .iter()
        .zip(&candidate.evidence_signals)
    {
        let effective_weight = channel.weight.get() * channel.gate.get();
        let normalized_weight = effective_weight / profile.evidence_denominator;
        let contribution = normalized_weight * signal.value.get();
        evidence_score += contribution;

        evidence_contributions.push(EvidenceContribution {
            channel_id: channel.channel_id,
            weight: channel.weight,
            gate: channel.gate,
            signal: signal.value,
            effective_weight: UnitInterval::from_computed(effective_weight),
            contribution: UnitInterval::from_computed(contribution),
        });
    }

    let mut inhibition_contributions = Vec::with_capacity(profile.inhibition_channels.len());
    let mut retention = 1.0;

    for (channel, signal) in profile
        .inhibition_channels
        .iter()
        .zip(&candidate.inhibition_signals)
    {
        let retention_factor = 1.0 - channel.strength.get() * signal.value.get();
        retention *= retention_factor;

        inhibition_contributions.push(InhibitionContribution {
            channel_id: channel.channel_id,
            strength: channel.strength,
            signal: signal.value,
            retention_factor: UnitInterval::from_computed(retention_factor),
        });
    }

    let evidence_score = UnitInterval::from_computed(evidence_score);
    let retention = UnitInterval::from_computed(retention);
    let score = UnitInterval::from_computed(evidence_score.get() * retention.get());

    RankedActivation {
        candidate_id: candidate.candidate_id,
        breakdown: ActivationBreakdown {
            evidence_score,
            retention,
            score,
            evidence_contributions,
            inhibition_contributions,
        },
    }
}

trait ProfileChannel {
    fn channel_id(&self) -> ChannelId;
}

impl ProfileChannel for EvidenceChannel {
    fn channel_id(&self) -> ChannelId {
        self.channel_id
    }
}

impl ProfileChannel for InhibitionChannel {
    fn channel_id(&self) -> ChannelId {
        self.channel_id
    }
}

#[derive(Clone, Copy)]
enum SignalCategory {
    Evidence,
    Inhibition,
}

fn validate_signal_set<C: ProfileChannel>(
    candidate_id: CandidateId,
    channels: &[C],
    signals: &[ChannelSignal],
    category: SignalCategory,
) -> Result<(), ActivationError> {
    let mut channel_index = 0;
    let mut signal_index = 0;

    while channel_index < channels.len() && signal_index < signals.len() {
        let channel_id = channels[channel_index].channel_id();
        let signal_id = signals[signal_index].channel_id;

        match signal_id.cmp(&channel_id) {
            std::cmp::Ordering::Less => {
                return Err(unexpected_signal_error(candidate_id, signal_id, category));
            }
            std::cmp::Ordering::Greater => {
                return Err(missing_signal_error(candidate_id, channel_id, category));
            }
            std::cmp::Ordering::Equal => {
                channel_index += 1;
                signal_index += 1;
            }
        }
    }

    if channel_index < channels.len() {
        return Err(missing_signal_error(
            candidate_id,
            channels[channel_index].channel_id(),
            category,
        ));
    }

    if signal_index < signals.len() {
        return Err(unexpected_signal_error(
            candidate_id,
            signals[signal_index].channel_id,
            category,
        ));
    }

    Ok(())
}

fn missing_signal_error(
    candidate_id: CandidateId,
    channel_id: ChannelId,
    category: SignalCategory,
) -> ActivationError {
    match category {
        SignalCategory::Evidence => ActivationError::MissingEvidenceSignal {
            candidate_id,
            channel_id,
        },
        SignalCategory::Inhibition => ActivationError::MissingInhibitionSignal {
            candidate_id,
            channel_id,
        },
    }
}

fn unexpected_signal_error(
    candidate_id: CandidateId,
    channel_id: ChannelId,
    category: SignalCategory,
) -> ActivationError {
    match category {
        SignalCategory::Evidence => ActivationError::UnexpectedEvidenceSignal {
            candidate_id,
            channel_id,
        },
        SignalCategory::Inhibition => ActivationError::UnexpectedInhibitionSignal {
            candidate_id,
            channel_id,
        },
    }
}

fn first_duplicate_channel(ids: impl Iterator<Item = ChannelId>) -> Option<ChannelId> {
    let mut ids: Vec<ChannelId> = ids.collect();
    ids.sort_unstable();
    ids.windows(2)
        .find(|pair| pair[0] == pair[1])
        .map(|pair| pair[0])
}

const fn canonical_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}
