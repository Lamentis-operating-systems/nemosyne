use super::model::PreparedChannel;
use super::{
    ActivationCandidate, ActivationError, ActivationExplanation, ActivationProfile, CandidateId,
    ChannelId, EvidenceContribution, InhibitionContribution, RankedActivation, UnitInterval,
};

/// Scores and ranks all candidates against one activation profile.
///
/// Every candidate must provide exactly one signal for every profile channel
/// and no other signal. Results are ordered by descending activation score and
/// then by ascending candidate identifier for exact score ties. No candidate is
/// filtered from the result. Per-channel details are available separately from
/// [`explain_activation`].
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
    canonical_candidates.sort_unstable_by_key(|candidate| candidate.candidate_id());

    if let Some(pair) = canonical_candidates
        .windows(2)
        .find(|pair| pair[0].candidate_id() == pair[1].candidate_id())
    {
        return Err(ActivationError::DuplicateCandidate {
            candidate_id: pair[0].candidate_id(),
        });
    }

    let mut ranked = Vec::with_capacity(canonical_candidates.len());
    for candidate in canonical_candidates {
        validate_signals(profile, candidate)?;
        ranked.push(evaluate_candidate(profile, candidate, &mut NoTrace));
    }

    ranked.sort_unstable_by(|left, right| {
        right
            .score()
            .get()
            .total_cmp(&left.score().get())
            .then_with(|| left.candidate_id().cmp(&right.candidate_id()))
    });
    Ok(ranked)
}

/// Explains one candidate's activation against a profile.
///
/// The aggregate values are calculated by the same evaluator used by
/// [`rank_activations`]. Unlike ranking, this operation records every
/// per-channel contribution.
///
/// # Errors
///
/// Returns an error when the candidate's signal channels do not exactly match
/// the profile.
pub fn explain_activation(
    profile: &ActivationProfile,
    candidate: &ActivationCandidate,
) -> Result<ActivationExplanation, ActivationError> {
    validate_signals(profile, candidate)?;
    let mut trace = ExplanationTrace::new(profile);
    let activation = evaluate_candidate(profile, candidate, &mut trace);
    Ok(trace.finish(activation))
}

fn validate_signals(
    profile: &ActivationProfile,
    candidate: &ActivationCandidate,
) -> Result<(), ActivationError> {
    let expected = profile.prepared_channels();
    let actual = candidate.signals();
    let mut expected_index = 0;
    let mut actual_index = 0;

    while expected_index < expected.len() && actual_index < actual.len() {
        let expected_id = expected[expected_index].channel_id();
        let actual_id = actual[actual_index].channel_id();
        match expected_id.cmp(&actual_id) {
            std::cmp::Ordering::Less => {
                return Err(missing(candidate.candidate_id(), expected_id));
            }
            std::cmp::Ordering::Greater => {
                return Err(unexpected(candidate.candidate_id(), actual_id));
            }
            std::cmp::Ordering::Equal => {
                expected_index += 1;
                actual_index += 1;
            }
        }
    }

    if let Some(channel) = expected.get(expected_index) {
        return Err(missing(candidate.candidate_id(), channel.channel_id()));
    }
    if let Some(signal) = actual.get(actual_index) {
        return Err(unexpected(candidate.candidate_id(), signal.channel_id()));
    }
    Ok(())
}

fn evaluate_candidate<T: Trace>(
    profile: &ActivationProfile,
    candidate: &ActivationCandidate,
    trace: &mut T,
) -> RankedActivation {
    let mut evidence_score = 0.0;
    let mut retention = 1.0;

    for (channel, signal) in profile
        .prepared_channels()
        .iter()
        .copied()
        .zip(candidate.signals().iter().copied())
    {
        match channel {
            PreparedChannel::Evidence {
                channel,
                effective_weight,
                normalized_weight,
            } => {
                let contribution = normalized_weight.get() * signal.value().get();
                evidence_score += contribution;
                trace.evidence(EvidenceContribution::new(
                    channel.channel_id(),
                    channel.weight(),
                    channel.gate(),
                    signal.value(),
                    effective_weight,
                    normalized_weight,
                    UnitInterval::from_computed(contribution),
                ));
            }
            PreparedChannel::Inhibition(channel) => {
                let factor = 1.0 - channel.strength().get() * signal.value().get();
                retention *= factor;
                trace.inhibition(InhibitionContribution::new(
                    channel.channel_id(),
                    channel.strength(),
                    signal.value(),
                    UnitInterval::from_computed(factor),
                ));
            }
        }
    }

    let evidence_score = UnitInterval::from_computed(evidence_score);
    let retention = UnitInterval::from_computed(retention);
    RankedActivation::new(
        candidate.candidate_id(),
        evidence_score,
        retention,
        UnitInterval::from_computed(evidence_score.get() * retention.get()),
    )
}

trait Trace {
    fn evidence(&mut self, contribution: EvidenceContribution);
    fn inhibition(&mut self, contribution: InhibitionContribution);
}

struct NoTrace;

impl Trace for NoTrace {
    fn evidence(&mut self, _contribution: EvidenceContribution) {}
    fn inhibition(&mut self, _contribution: InhibitionContribution) {}
}

struct ExplanationTrace {
    evidence: Vec<EvidenceContribution>,
    inhibition: Vec<InhibitionContribution>,
}

impl ExplanationTrace {
    fn new(profile: &ActivationProfile) -> Self {
        let evidence_count = profile
            .prepared_channels()
            .iter()
            .filter(|channel| matches!(channel, PreparedChannel::Evidence { .. }))
            .count();
        Self {
            evidence: Vec::with_capacity(evidence_count),
            inhibition: Vec::with_capacity(profile.prepared_channels().len() - evidence_count),
        }
    }

    fn finish(self, activation: RankedActivation) -> ActivationExplanation {
        ActivationExplanation::new(
            activation,
            self.evidence.into_boxed_slice(),
            self.inhibition.into_boxed_slice(),
        )
    }
}

impl Trace for ExplanationTrace {
    fn evidence(&mut self, contribution: EvidenceContribution) {
        self.evidence.push(contribution);
    }

    fn inhibition(&mut self, contribution: InhibitionContribution) {
        self.inhibition.push(contribution);
    }
}

const fn missing(candidate_id: CandidateId, channel_id: ChannelId) -> ActivationError {
    ActivationError::MissingSignal {
        candidate_id,
        channel_id,
    }
}

const fn unexpected(candidate_id: CandidateId, channel_id: ChannelId) -> ActivationError {
    ActivationError::UnexpectedSignal {
        candidate_id,
        channel_id,
    }
}
