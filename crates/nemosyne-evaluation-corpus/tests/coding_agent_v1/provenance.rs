use std::collections::BTreeSet;

use nemosyne_evaluation_corpus::activation::{
    EvidenceLevel, JudgmentApplicability, ScenarioProvenance,
};

use super::corpus;

#[test]
fn every_numeric_judgment_and_preference_resolves_to_constructed_facts() {
    let corpus = corpus();

    for partition in [corpus.calibration(), corpus.held_out()] {
        for scenario in partition.evidence() {
            assert_eq!(scenario.provenance(), ScenarioProvenance::Constructed);
            assert_clean_text(scenario.title());
            assert_clean_text(scenario.situation());

            let fact_ids: BTreeSet<_> =
                scenario.facts().iter().map(|fact| fact.fact_id()).collect();
            assert!(!fact_ids.is_empty());
            for fact in scenario.facts() {
                assert_clean_text(fact.statement());
                assert_authoring_fact(fact.statement());
            }

            for gate in scenario.gates().values() {
                assert_eq!(
                    gate.applicability(),
                    JudgmentApplicability::Applicable,
                    "gate judgments are always applicable"
                );
                assert_clean_text(gate.rationale());
                assert_resolved(gate.fact_ids(), &fact_ids);
            }
            for candidate in scenario.candidates() {
                assert_clean_text(candidate.label());
                for signal in candidate.signals() {
                    assert_clean_text(signal.rationale());
                    assert_resolved(signal.fact_ids(), &fact_ids);
                    let gate = scenario
                        .gates()
                        .values()
                        .iter()
                        .find(|gate| gate.channel_id() == signal.channel_id())
                        .expect("every signal has a corresponding gate");
                    if gate.level() == EvidenceLevel::Absent {
                        assert_eq!(signal.level(), EvidenceLevel::Absent);
                        assert_eq!(signal.applicability(), JudgmentApplicability::Inactive);
                    } else {
                        assert_eq!(signal.applicability(), JudgmentApplicability::Applicable);
                    }
                }
            }
            for preference in scenario.preferences() {
                assert_clean_text(preference.rationale());
                assert_resolved(preference.fact_ids(), &fact_ids);
            }
        }
    }
}

#[test]
fn every_channel_judgment_retains_an_independently_authored_rationale() {
    let corpus = corpus();

    for partition in [corpus.calibration(), corpus.held_out()] {
        for scenario in partition.evidence() {
            assert_unique_rationales(
                scenario
                    .gates()
                    .values()
                    .iter()
                    .map(|value| value.rationale()),
            );
            for candidate in scenario.candidates() {
                assert_unique_rationales(candidate.signals().iter().map(|value| value.rationale()));
            }
        }
    }
}

#[test]
fn metadata_exactly_covers_the_derived_evaluator_input() {
    let corpus = corpus();

    for partition in [corpus.calibration(), corpus.held_out()] {
        for evidence in partition.evidence() {
            let scenario = partition
                .suite()
                .scenarios()
                .iter()
                .find(|scenario| scenario.scenario_id() == evidence.scenario_id())
                .expect("every evidence item must have one derived numeric scenario");

            assert_eq!(scenario.gates().len(), evidence.gates().values().len());
            for (gate, annotated) in scenario.gates().iter().zip(evidence.gates().values()) {
                assert_eq!(gate.channel_id(), annotated.channel_id());
                assert_eq!(gate.gate().get(), annotated.level().as_f64());
            }
            assert_eq!(scenario.candidates().len(), evidence.candidates().len());
            for (candidate, annotated) in scenario.candidates().iter().zip(evidence.candidates()) {
                assert_eq!(candidate.candidate_id(), annotated.candidate_id());
                for (signal, annotation) in candidate.signals().iter().zip(annotated.signals()) {
                    assert_eq!(signal.channel_id(), annotation.channel_id());
                    assert_eq!(signal.value().get(), annotation.level().as_f64());
                }
            }
            assert_eq!(scenario.preferences().len(), evidence.preferences().len());
            for (preference, annotated) in scenario.preferences().iter().zip(evidence.preferences())
            {
                assert_eq!(*preference, annotated.expectation());
            }
        }
    }
}

#[test]
fn distractors_remain_ranked_without_becoming_expected_preferences() {
    let corpus = corpus();

    for scenario_id in [2201, 2202] {
        let evidence = corpus
            .held_out()
            .find(nemosyne_evaluation::activation::ScenarioId::new(
                scenario_id,
            ))
            .expect("distractor scenario must exist");
        assert_eq!(evidence.candidates().len(), 3);
        assert_eq!(evidence.preferences().len(), 1);
        let preference = evidence.preferences()[0].expectation();
        assert_ne!(preference.preferred().get(), 3);
        assert_ne!(preference.other().get(), 3);
    }
}

fn assert_resolved(
    references: &[nemosyne_evaluation_corpus::activation::FactId],
    known: &BTreeSet<nemosyne_evaluation_corpus::activation::FactId>,
) {
    assert!(!references.is_empty());
    assert!(references.iter().all(|fact_id| known.contains(fact_id)));
}

fn assert_clean_text(value: &str) {
    assert!(!value.is_empty());
    assert_eq!(value, value.trim());
}

fn assert_unique_rationales<'a>(rationales: impl Iterator<Item = &'a str>) {
    let rationales: Vec<_> = rationales.collect();
    let unique: BTreeSet<_> = rationales.iter().copied().collect();
    assert_eq!(
        unique.len(),
        rationales.len(),
        "channel rationales must be authored independently"
    );
}

fn assert_authoring_fact(statement: &str) {
    let statement = statement.to_ascii_lowercase();
    for forbidden in ["evaluator", "parameter", "ranking", "score", "weight"] {
        assert!(
            !statement.contains(forbidden),
            "fact contains forbidden result language: {statement}"
        );
    }
}
