use nemosyne_core::activation::{CandidateId, rank_activations};

use super::{assert_close, candidate, evidence, inhibition, profile};

#[test]
fn numeric_red_light_example_ranks_immediate_evidence_first() {
    // These caller-selected channels and values are an algorithm fixture, not
    // a safety policy or a claim that the kernel can identify safe behavior.
    let profile = profile(vec![
        evidence(1, 0.4, 1.0),
        evidence(2, 0.3, 1.0),
        evidence(3, 0.2, 1.0),
        evidence(4, 0.2, 0.5),
        inhibition(5, 0.5),
    ]);
    let candidates = [
        candidate(2, &[(1, 0.2), (2, 0.1), (3, 0.0), (4, 1.0), (5, 0.4)]),
        candidate(1, &[(1, 1.0), (2, 1.0), (3, 1.0), (4, 0.5), (5, 0.0)]),
    ];
    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    assert_eq!(ranked[0].candidate_id(), CandidateId::new(1));
    assert_close(ranked[0].evidence_score().get(), 0.95);
    assert_close(ranked[0].retention().get(), 1.0);
    assert_close(ranked[0].score().get(), 0.95);
    assert_eq!(ranked[1].candidate_id(), CandidateId::new(2));
    assert_close(ranked[1].evidence_score().get(), 0.21);
    assert_close(ranked[1].retention().get(), 0.8);
    assert_close(ranked[1].score().get(), 0.168);
}
