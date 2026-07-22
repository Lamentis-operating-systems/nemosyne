use super::ActivationError;

/// A numeric identifier for an activation candidate.
///
/// The value is opaque. Zero has no special meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

/// A numeric identifier for an activation channel.
///
/// Evidence and inhibition channels share one namespace. The value is opaque,
/// and zero has no special meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub(super) fn from_computed(value: f64) -> Self {
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

/// One evidence or inhibition channel in an activation profile.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum ActivationChannel {
    /// A channel that contributes positive evidence.
    Evidence(EvidenceChannel),
    /// A channel that multiplicatively retains evidence.
    Inhibition(InhibitionChannel),
}

impl ActivationChannel {
    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        match self {
            Self::Evidence(channel) => channel.channel_id,
            Self::Inhibition(channel) => channel.channel_id,
        }
    }

    /// Returns the evidence configuration when this is an evidence channel.
    #[must_use]
    pub const fn evidence(&self) -> Option<&EvidenceChannel> {
        match self {
            Self::Evidence(channel) => Some(channel),
            Self::Inhibition(_) => None,
        }
    }

    /// Returns the inhibition configuration when this is an inhibition channel.
    #[must_use]
    pub const fn inhibition(&self) -> Option<&InhibitionChannel> {
        match self {
            Self::Evidence(_) => None,
            Self::Inhibition(channel) => Some(channel),
        }
    }
}

impl From<EvidenceChannel> for ActivationChannel {
    fn from(channel: EvidenceChannel) -> Self {
        Self::Evidence(channel)
    }
}

impl From<InhibitionChannel> for ActivationChannel {
    fn from(channel: InhibitionChannel) -> Self {
        Self::Inhibition(channel)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum PreparedChannel {
    Evidence {
        channel: EvidenceChannel,
        effective_weight: UnitInterval,
        normalized_weight: UnitInterval,
    },
    Inhibition(InhibitionChannel),
}

impl PreparedChannel {
    pub(super) const fn channel_id(self) -> ChannelId {
        match self {
            Self::Evidence { channel, .. } => channel.channel_id,
            Self::Inhibition(channel) => channel.channel_id,
        }
    }
}

/// The canonical channel configuration used for one activation run.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationProfile {
    channels: Box<[ActivationChannel]>,
    prepared_channels: Box<[PreparedChannel]>,
}

impl ActivationProfile {
    /// Creates and prepares an activation profile.
    ///
    /// Channels are stored in ascending identifier order. Identifiers must be
    /// unique, and at least one evidence channel must have a positive effective
    /// weight.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate channel identifiers, an effective-weight
    /// underflow, or the absence of effective evidence.
    pub fn new(mut channels: Vec<ActivationChannel>) -> Result<Self, ActivationError> {
        channels.sort_unstable_by_key(ActivationChannel::channel_id);

        if let Some(pair) = channels
            .windows(2)
            .find(|pair| pair[0].channel_id() == pair[1].channel_id())
        {
            return Err(ActivationError::DuplicateProfileChannel {
                channel_id: pair[0].channel_id(),
            });
        }

        let mut effective_weights = Vec::with_capacity(channels.len());
        let mut denominator = 0.0;
        for channel in &channels {
            if let ActivationChannel::Evidence(channel) = channel {
                let weight = channel.weight.get();
                let gate = channel.gate.get();
                let effective_weight = weight * gate;
                if weight > 0.0 && gate > 0.0 && effective_weight == 0.0 {
                    return Err(ActivationError::EffectiveWeightUnderflow {
                        channel_id: channel.channel_id,
                    });
                }
                denominator += effective_weight;
                effective_weights.push((channel.channel_id, effective_weight));
            }
        }

        if denominator == 0.0 {
            return Err(ActivationError::NoEffectiveEvidence);
        }

        let mut prepared = Vec::with_capacity(channels.len());
        let mut evidence_index = 0;
        for channel in &channels {
            match channel {
                ActivationChannel::Evidence(channel) => {
                    let (channel_id, effective_weight) = effective_weights[evidence_index];
                    debug_assert_eq!(channel_id, channel.channel_id);
                    evidence_index += 1;
                    let normalized_weight = effective_weight / denominator;
                    if effective_weight > 0.0 && normalized_weight == 0.0 {
                        return Err(ActivationError::NormalizedWeightUnderflow {
                            channel_id: channel.channel_id,
                        });
                    }
                    prepared.push(PreparedChannel::Evidence {
                        channel: *channel,
                        effective_weight: UnitInterval::from_computed(effective_weight),
                        normalized_weight: UnitInterval::from_computed(normalized_weight),
                    });
                }
                ActivationChannel::Inhibition(channel) => {
                    prepared.push(PreparedChannel::Inhibition(*channel));
                }
            }
        }

        Ok(Self {
            channels: channels.into_boxed_slice(),
            prepared_channels: prepared.into_boxed_slice(),
        })
    }

    /// Returns all channels in ascending identifier order.
    #[must_use]
    pub fn channels(&self) -> &[ActivationChannel] {
        &self.channels
    }

    pub(super) fn prepared_channels(&self) -> &[PreparedChannel] {
        &self.prepared_channels
    }
}

/// One candidate and its complete normalized signal set.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationCandidate {
    candidate_id: CandidateId,
    signals: Box<[ChannelSignal]>,
}

impl ActivationCandidate {
    /// Creates and canonicalizes a candidate.
    ///
    /// Signals are stored in ascending channel-identifier order and must be
    /// unique. Exact correspondence with a profile is checked during ranking
    /// or explanation.
    ///
    /// # Errors
    ///
    /// Returns an error when a channel identifier occurs more than once.
    pub fn new(
        candidate_id: CandidateId,
        mut signals: Vec<ChannelSignal>,
    ) -> Result<Self, ActivationError> {
        signals.sort_unstable_by_key(ChannelSignal::channel_id);
        if let Some(pair) = signals
            .windows(2)
            .find(|pair| pair[0].channel_id == pair[1].channel_id)
        {
            return Err(ActivationError::DuplicateCandidateChannel {
                candidate_id,
                channel_id: pair[0].channel_id,
            });
        }

        Ok(Self {
            candidate_id,
            signals: signals.into_boxed_slice(),
        })
    }

    /// Returns the candidate identifier.
    #[must_use]
    pub const fn candidate_id(&self) -> CandidateId {
        self.candidate_id
    }

    /// Returns signals in ascending channel-identifier order.
    #[must_use]
    pub fn signals(&self) -> &[ChannelSignal] {
        &self.signals
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
    normalized_weight: UnitInterval,
    contribution: UnitInterval,
}

impl EvidenceContribution {
    pub(super) const fn new(
        channel_id: ChannelId,
        weight: UnitInterval,
        gate: UnitInterval,
        signal: UnitInterval,
        effective_weight: UnitInterval,
        normalized_weight: UnitInterval,
        contribution: UnitInterval,
    ) -> Self {
        Self {
            channel_id,
            weight,
            gate,
            signal,
            effective_weight,
            normalized_weight,
            contribution,
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
    /// Returns the effective weight divided by the evidence denominator.
    #[must_use]
    pub const fn normalized_weight(&self) -> UnitInterval {
        self.normalized_weight
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
    pub(super) const fn new(
        channel_id: ChannelId,
        strength: UnitInterval,
        signal: UnitInterval,
        retention_factor: UnitInterval,
    ) -> Self {
        Self {
            channel_id,
            strength,
            signal,
            retention_factor,
        }
    }

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

/// One compact ranked activation result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RankedActivation {
    candidate_id: CandidateId,
    evidence_score: UnitInterval,
    retention: UnitInterval,
    score: UnitInterval,
}

impl RankedActivation {
    pub(super) const fn new(
        candidate_id: CandidateId,
        evidence_score: UnitInterval,
        retention: UnitInterval,
        score: UnitInterval,
    ) -> Self {
        Self {
            candidate_id,
            evidence_score,
            retention,
            score,
        }
    }

    /// Returns the candidate identifier.
    #[must_use]
    pub const fn candidate_id(&self) -> CandidateId {
        self.candidate_id
    }
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
}

/// A complete explanation of one candidate's activation score.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationExplanation {
    activation: RankedActivation,
    evidence_contributions: Box<[EvidenceContribution]>,
    inhibition_contributions: Box<[InhibitionContribution]>,
}

impl ActivationExplanation {
    pub(super) fn new(
        activation: RankedActivation,
        evidence_contributions: Box<[EvidenceContribution]>,
        inhibition_contributions: Box<[InhibitionContribution]>,
    ) -> Self {
        Self {
            activation,
            evidence_contributions,
            inhibition_contributions,
        }
    }

    /// Returns the compact activation values reproduced by this explanation.
    #[must_use]
    pub const fn activation(&self) -> &RankedActivation {
        &self.activation
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

fn canonical_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}
