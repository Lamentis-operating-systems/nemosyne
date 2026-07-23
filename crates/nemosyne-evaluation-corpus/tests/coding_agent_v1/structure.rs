use std::collections::{BTreeMap, BTreeSet};

use nemosyne_core::activation::ChannelId;
use nemosyne_evaluation::activation::ActivationParameter;
use nemosyne_evaluation_corpus::activation::{CorpusRevision, CorpusSplit, EvidenceLevel};

use super::{assert_strictly_sorted, corpus};

#[test]
fn exposes_the_complete_canonical_revision() {
    let corpus = corpus();

    assert_eq!(corpus.revision(), CorpusRevision::new(1));
    assert_eq!(corpus.channels().len(), 5);
    assert_eq!(corpus.categories().len(), 4);
    assert_eq!(corpus.references().len(), 2);
    assert_eq!(corpus.calibration().split(), CorpusSplit::Calibration);
    assert_eq!(corpus.held_out().split(), CorpusSplit::HeldOut);
    assert_eq!(corpus.calibration().evidence().len(), 8);
    assert_eq!(corpus.held_out().evidence().len(), 8);

    assert_eq!(
        corpus
            .channels()
            .iter()
            .map(|channel| (channel.channel_id().get(), channel.key()))
            .collect::<Vec<_>>(),
        vec![
            (10, "trigger_alignment"),
            (20, "observed_state_alignment"),
            (30, "active_outcome_alignment"),
            (40, "capability_fit"),
            (50, "constraint_alignment"),
        ]
    );
    assert_eq!(
        corpus
            .references()
            .iter()
            .map(|reference| (reference.reference_id().get(), reference.key()))
            .collect::<Vec<_>>(),
        vec![(10, "trigger_only"), (20, "uniform_evidence")]
    );

    assert_strictly_sorted(corpus.channels().iter().map(|channel| channel.channel_id()));
    assert_strictly_sorted(
        corpus
            .categories()
            .iter()
            .map(|category| category.category_id()),
    );
    assert_strictly_sorted(
        corpus
            .references()
            .iter()
            .map(|reference| reference.reference_id()),
    );
    for partition in [corpus.calibration(), corpus.held_out()] {
        assert_strictly_sorted(
            partition
                .evidence()
                .iter()
                .map(|scenario| scenario.scenario_id()),
        );
        for scenario in partition.evidence() {
            assert_strictly_sorted(scenario.facts().iter().map(|fact| fact.fact_id()));
            assert_strictly_sorted(
                scenario
                    .gates()
                    .values()
                    .iter()
                    .map(|value| value.channel_id()),
            );
            assert_strictly_sorted(
                scenario
                    .candidates()
                    .iter()
                    .map(|candidate| candidate.candidate_id()),
            );
            for candidate in scenario.candidates() {
                assert_strictly_sorted(
                    candidate.signals().iter().map(|signal| signal.channel_id()),
                );
            }
        }
    }
}

#[test]
fn evidence_grid_is_exact_and_channel_specific() {
    let corpus = corpus();
    let levels = [
        (EvidenceLevel::Absent, 0.0),
        (EvidenceLevel::Low, 0.25),
        (EvidenceLevel::Medium, 0.5),
        (EvidenceLevel::High, 0.75),
        (EvidenceLevel::Maximal, 1.0),
    ];

    for (level, expected) in levels {
        assert_eq!(level.as_f64(), expected);
        for channel in corpus.channels() {
            assert!(!channel.gate_anchor(level).is_empty());
            assert!(!channel.signal_anchor(level).is_empty());
        }
    }
}

#[test]
fn partitions_are_disjoint_by_scenario_and_semantic_case() {
    let corpus = corpus();
    let calibration_scenarios: BTreeSet<_> = corpus
        .calibration()
        .evidence()
        .iter()
        .map(|scenario| scenario.scenario_id())
        .collect();
    let held_out_scenarios: BTreeSet<_> = corpus
        .held_out()
        .evidence()
        .iter()
        .map(|scenario| scenario.scenario_id())
        .collect();
    let calibration_cases: BTreeSet<_> = corpus
        .calibration()
        .evidence()
        .iter()
        .map(|scenario| scenario.semantic_case_id())
        .collect();
    let held_out_cases: BTreeSet<_> = corpus
        .held_out()
        .evidence()
        .iter()
        .map(|scenario| scenario.semantic_case_id())
        .collect();

    assert!(calibration_scenarios.is_disjoint(&held_out_scenarios));
    assert!(calibration_cases.is_disjoint(&held_out_cases));
}

#[test]
fn every_category_has_both_splits_and_every_case_is_a_reversed_pair() {
    let corpus = corpus();
    let mut category_splits = BTreeSet::new();
    let mut cases = BTreeMap::new();
    for partition in [corpus.calibration(), corpus.held_out()] {
        for scenario in partition.evidence() {
            category_splits.insert((scenario.category_id(), partition.split()));
            cases
                .entry(scenario.semantic_case_id())
                .or_insert_with(Vec::new)
                .push(scenario);
        }
    }

    for category in corpus.categories() {
        assert!(category_splits.contains(&(category.category_id(), CorpusSplit::Calibration)));
        assert!(category_splits.contains(&(category.category_id(), CorpusSplit::HeldOut)));
    }

    assert_eq!(cases.len(), 8);
    for pair in cases.values() {
        assert_eq!(pair.len(), 2);
        let first_candidates: Vec<_> = pair[0]
            .candidates()
            .iter()
            .map(|candidate| (candidate.candidate_id(), candidate.label()))
            .collect();
        let second_candidates: Vec<_> = pair[1]
            .candidates()
            .iter()
            .map(|candidate| (candidate.candidate_id(), candidate.label()))
            .collect();
        assert_eq!(first_candidates, second_candidates);
        assert_eq!(pair[0].preferences().len(), 1);
        assert_eq!(pair[1].preferences().len(), 1);
        let first = pair[0].preferences()[0].expectation();
        let second = pair[1].preferences()[0].expectation();
        assert_eq!(first.preferred(), second.other());
        assert_eq!(first.other(), second.preferred());
    }
}

#[test]
fn references_use_only_the_exact_corpus_evidence_schema() {
    let corpus = corpus();
    let expected_channels = [10, 20, 30, 40, 50];

    for reference in corpus.references() {
        assert_eq!(reference.parameters().parameters().len(), 5);
        for (parameter, expected_channel) in reference
            .parameters()
            .parameters()
            .iter()
            .zip(expected_channels)
        {
            let ActivationParameter::Evidence(evidence) = parameter else {
                panic!("corpus references must not contain inhibition");
            };
            assert_eq!(evidence.channel_id(), ChannelId::new(expected_channel));
        }
    }
}

#[test]
fn repeated_construction_is_equal() {
    assert_eq!(corpus(), corpus());
}
