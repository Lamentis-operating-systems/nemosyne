use nemosyne_evaluation::activation::ActivationParameter;

use super::{
    ActivationEvidenceCorpus, AnchoredValue, CorpusPartition, CorpusSplit, EvidenceLevel,
    JudgmentApplicability, ScenarioEvidence, ScenarioProvenance,
};

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const PROTOCOL: &str = "nemosyne.activation-evidence-corpus.regression-fingerprint.v1";

pub(super) fn regression_fingerprint(corpus: &ActivationEvidenceCorpus) -> u64 {
    let mut fingerprint = Fingerprint::new();
    fingerprint.text(PROTOCOL);
    fingerprint.u64(corpus.revision().get());

    fingerprint.len(corpus.channels().len());
    for channel in corpus.channels() {
        fingerprint.tag(1);
        fingerprint.u64(channel.channel_id().get());
        fingerprint.text(channel.key());
        fingerprint.text(channel.gate_meaning());
        fingerprint.text(channel.signal_meaning());
        for level in levels() {
            fingerprint.text(channel.gate_anchor(level));
        }
        for level in levels() {
            fingerprint.text(channel.signal_anchor(level));
        }
    }

    fingerprint.len(corpus.categories().len());
    for category in corpus.categories() {
        fingerprint.tag(2);
        fingerprint.u64(category.category_id().get());
        fingerprint.text(category.key());
        fingerprint.text(category.description());
    }

    fingerprint.len(corpus.references().len());
    for reference in corpus.references() {
        fingerprint.tag(3);
        fingerprint.u64(reference.reference_id().get());
        fingerprint.text(reference.key());
        fingerprint.text(reference.rationale());
        fingerprint.len(reference.parameters().parameters().len());
        for parameter in reference.parameters().parameters() {
            hash_parameter(&mut fingerprint, parameter);
        }
    }

    hash_partition(&mut fingerprint, corpus.calibration());
    hash_partition(&mut fingerprint, corpus.held_out());
    fingerprint.finish()
}

fn hash_parameter(fingerprint: &mut Fingerprint, parameter: &ActivationParameter) {
    if let Some(evidence) = parameter.evidence() {
        fingerprint.tag(4);
        fingerprint.u64(evidence.channel_id().get());
        fingerprint.u64(evidence.weight().get().to_bits());
    } else if let Some(inhibition) = parameter.inhibition() {
        fingerprint.tag(5);
        fingerprint.u64(inhibition.channel_id().get());
        fingerprint.u64(inhibition.strength().get().to_bits());
    } else {
        panic!("regression fingerprint protocol v1 does not encode this activation parameter kind");
    }
}

fn hash_partition(fingerprint: &mut Fingerprint, partition: &CorpusPartition) {
    fingerprint.tag(6);
    hash_split(fingerprint, partition.split());
    fingerprint.len(partition.evidence().len());
    for scenario in partition.evidence() {
        hash_scenario_evidence(fingerprint, scenario);
    }

    fingerprint.len(partition.suite().scenarios().len());
    for scenario in partition.suite().scenarios() {
        fingerprint.tag(7);
        fingerprint.u64(scenario.scenario_id().get());
        fingerprint.len(scenario.gates().len());
        for gate in scenario.gates() {
            fingerprint.u64(gate.channel_id().get());
            fingerprint.u64(gate.gate().get().to_bits());
        }
        fingerprint.len(scenario.candidates().len());
        for candidate in scenario.candidates() {
            fingerprint.u64(candidate.candidate_id().get());
            fingerprint.len(candidate.signals().len());
            for signal in candidate.signals() {
                fingerprint.u64(signal.channel_id().get());
                fingerprint.u64(signal.value().get().to_bits());
            }
        }
        fingerprint.len(scenario.preferences().len());
        for preference in scenario.preferences() {
            fingerprint.u64(preference.preferred().get());
            fingerprint.u64(preference.other().get());
        }
    }
}

fn hash_scenario_evidence(fingerprint: &mut Fingerprint, scenario: &ScenarioEvidence) {
    fingerprint.tag(8);
    fingerprint.u64(scenario.scenario_id().get());
    fingerprint.u64(scenario.semantic_case_id().get());
    fingerprint.u64(scenario.category_id().get());
    fingerprint.text(scenario.title());
    fingerprint.text(scenario.situation());
    hash_provenance(fingerprint, scenario.provenance());

    fingerprint.len(scenario.facts().len());
    for fact in scenario.facts() {
        fingerprint.u64(fact.fact_id().get());
        fingerprint.text(fact.statement());
    }

    fingerprint.len(scenario.gates().values().len());
    for gate in scenario.gates().values() {
        hash_anchored_value(fingerprint, gate);
    }

    fingerprint.len(scenario.candidates().len());
    for candidate in scenario.candidates() {
        fingerprint.tag(9);
        fingerprint.u64(candidate.candidate_id().get());
        fingerprint.text(candidate.label());
        fingerprint.len(candidate.signals().len());
        for signal in candidate.signals() {
            hash_anchored_value(fingerprint, signal);
        }
    }

    fingerprint.len(scenario.preferences().len());
    for preference in scenario.preferences() {
        fingerprint.tag(10);
        fingerprint.u64(preference.expectation().preferred().get());
        fingerprint.u64(preference.expectation().other().get());
        fingerprint.len(preference.fact_ids().len());
        for fact_id in preference.fact_ids() {
            fingerprint.u64(fact_id.get());
        }
        fingerprint.text(preference.rationale());
    }
}

fn hash_anchored_value(fingerprint: &mut Fingerprint, value: &AnchoredValue) {
    fingerprint.tag(11);
    fingerprint.u64(value.channel_id().get());
    fingerprint.tag(level_tag(value.level()));
    fingerprint.tag(match value.applicability() {
        JudgmentApplicability::Applicable => 1,
        JudgmentApplicability::Inactive => 2,
    });
    fingerprint.len(value.fact_ids().len());
    for fact_id in value.fact_ids() {
        fingerprint.u64(fact_id.get());
    }
    fingerprint.text(value.rationale());
}

fn hash_split(fingerprint: &mut Fingerprint, split: CorpusSplit) {
    fingerprint.tag(match split {
        CorpusSplit::Calibration => 1,
        CorpusSplit::HeldOut => 2,
    });
}

fn hash_provenance(fingerprint: &mut Fingerprint, provenance: ScenarioProvenance) {
    let ScenarioProvenance::Constructed = provenance;
    fingerprint.tag(1);
}

const fn level_tag(level: EvidenceLevel) -> u8 {
    match level {
        EvidenceLevel::Absent => 0,
        EvidenceLevel::Low => 1,
        EvidenceLevel::Medium => 2,
        EvidenceLevel::High => 3,
        EvidenceLevel::Maximal => 4,
    }
}

const fn levels() -> [EvidenceLevel; 5] {
    [
        EvidenceLevel::Absent,
        EvidenceLevel::Low,
        EvidenceLevel::Medium,
        EvidenceLevel::High,
        EvidenceLevel::Maximal,
    ]
}

struct Fingerprint(u64);

impl Fingerprint {
    const fn new() -> Self {
        Self(FNV_OFFSET_BASIS)
    }

    fn tag(&mut self, value: u8) {
        self.bytes(&[value]);
    }

    fn len(&mut self, value: usize) {
        self.u64(value as u64);
    }

    fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }

    fn text(&mut self, value: &str) {
        self.len(value.len());
        self.bytes(value.as_bytes());
    }

    fn bytes(&mut self, values: &[u8]) {
        for value in values {
            self.0 ^= u64::from(*value);
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }

    const fn finish(self) -> u64 {
        self.0
    }
}
