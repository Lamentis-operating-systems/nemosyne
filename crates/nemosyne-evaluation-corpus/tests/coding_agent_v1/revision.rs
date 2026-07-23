use super::corpus;

#[test]
fn revision_one_full_artifact_is_frozen() {
    assert_eq!(corpus().regression_fingerprint(), 0x8f1d_f04f_fb61_59d5);
}

#[test]
fn revision_fingerprint_is_deterministic() {
    assert_eq!(
        corpus().regression_fingerprint(),
        corpus().regression_fingerprint()
    );
}
