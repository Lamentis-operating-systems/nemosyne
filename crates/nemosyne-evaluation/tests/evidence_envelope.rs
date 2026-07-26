//! Integration tests for the public EVD-01 evidence-envelope contract.

use nemosyne_evaluation::evidence::{
    AdmissionJoinField, ArtifactContentId, AttemptId, AttemptedArtifactKind,
    CapabilityIssuanceState, EstablishedIdentity, EstablishedIdentityKind, EvidenceDigest,
    EvidenceDisposition, EvidenceError, EvidenceIdentity, EvidenceSchemaVersion, EvidenceSignature,
    ExperimentReceiptPayloadV1, GuardAuthorityV1, GuardEvidenceError, GuardImplementationId,
    GuardSubjectV1, GuardWitnessClaimsV1, GuardWitnessEvidence, GuardWitnessV1, InputCommitment,
    LedgerBoundary, LedgerCommitment, PreAccessValidationResult, PrincipalId, RejectedAttemptV1,
    RejectedGuardSubjectV1, RejectionJoinField, RejectionReason, RunManifestClaimsV1, SchemaId,
    SealedSourceId, SealedSourceState, SignedRunManifestV1, TrustedTimestamp,
    ValidExperimentReceiptV1, ValidatedRunGuardSubjectV1, ValidationField, ValidationStage,
    ValidationWindow, ValidatorContext, ValidatorImplementationId, admit_for_outcome_access,
    finalize_rejection,
};

const MANIFEST_KEY: [u8; 32] = [11; 32];
const CUSTODIAN_KEY: [u8; 32] = [22; 32];
const VALIDATOR_KEY: [u8; 32] = [33; 32];
const EVALUATOR_KEY: [u8; 32] = [44; 32];

fn bytes<const N: usize>(value: u8) -> [u8; N] {
    [value; N]
}

fn attempt(value: u8) -> AttemptId {
    AttemptId::from_bytes(bytes(value))
}

fn principal(value: u8) -> PrincipalId {
    PrincipalId::from_bytes(bytes(value))
}

fn sealed_source(value: u8) -> SealedSourceId {
    SealedSourceId::from_bytes(bytes(value))
}

fn ledger(head: u8, tail: u8) -> LedgerBoundary {
    LedgerBoundary::new(
        LedgerCommitment::from_bytes(bytes(head)),
        LedgerCommitment::from_bytes(bytes(tail)),
    )
}

fn window(start: u64, end: u64) -> ValidationWindow {
    ValidationWindow::new(
        TrustedTimestamp::from_unix_seconds(start),
        TrustedTimestamp::from_unix_seconds(end),
    )
    .expect("fixture window must be valid")
}

fn manifest_claims(
    attempt_id: AttemptId,
    source: SealedSourceId,
    validation_principals: Vec<PrincipalId>,
    analysis_principals: Vec<PrincipalId>,
) -> RunManifestClaimsV1 {
    RunManifestClaimsV1::new(
        attempt_id,
        AttemptedArtifactKind::G1RunManifest,
        source,
        window(100, 200),
        validation_principals,
        analysis_principals,
        CapabilityIssuanceState::NotIssued,
        ledger(1, 2),
        ledger(3, 4),
    )
    .expect("fixture manifest claims must be valid")
}

fn manifest_for(attempt_id: AttemptId, source: SealedSourceId) -> SignedRunManifestV1 {
    SignedRunManifestV1::sign(
        manifest_claims(
            attempt_id,
            source,
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        b"frozen-g1-run-manifest",
        &MANIFEST_KEY,
    )
    .expect("fixture manifest must be valid")
}

fn witness_claims(
    attempt_id: AttemptId,
    subject: GuardSubjectV1,
    validation_principals: Vec<PrincipalId>,
    analysis_principals: Vec<PrincipalId>,
) -> GuardWitnessClaimsV1 {
    GuardWitnessClaimsV1::new(
        attempt_id,
        subject,
        window(100, 200),
        validation_principals,
        analysis_principals,
        CapabilityIssuanceState::NotIssued,
        ledger(1, 2),
        ledger(3, 4),
        GuardImplementationId::from_bytes(bytes(9)),
    )
    .expect("fixture witness claims must be valid")
}

fn admission_witness(manifest: &SignedRunManifestV1) -> GuardWitnessV1 {
    custom_admission_witness(
        manifest.claims().attempt_id(),
        ValidatedRunGuardSubjectV1::from_manifest(manifest),
        window(100, 200),
        vec![principal(1), principal(2)],
        vec![principal(3), principal(4)],
        ledger(1, 2),
        ledger(3, 4),
    )
}

#[allow(clippy::too_many_arguments)]
fn custom_admission_witness(
    attempt_id: AttemptId,
    subject: ValidatedRunGuardSubjectV1,
    validation_window: ValidationWindow,
    validation_principals: Vec<PrincipalId>,
    analysis_principals: Vec<PrincipalId>,
    outcome_access_ledger: LedgerBoundary,
    analysis_job_ledger: LedgerBoundary,
) -> GuardWitnessV1 {
    GuardWitnessV1::sign(
        GuardWitnessClaimsV1::new(
            attempt_id,
            GuardSubjectV1::ValidatedRun(subject),
            validation_window,
            validation_principals,
            analysis_principals,
            CapabilityIssuanceState::NotIssued,
            outcome_access_ledger,
            analysis_job_ledger,
            GuardImplementationId::from_bytes(bytes(9)),
        )
        .expect("custom witness claims must be valid"),
        &CUSTODIAN_KEY,
    )
}

fn validator() -> ValidatorContext<'static> {
    ValidatorContext::new(
        TrustedTimestamp::from_unix_seconds(201),
        ValidatorImplementationId::from_bytes(bytes(8)),
        &VALIDATOR_KEY,
    )
}

fn guard_authority() -> GuardAuthorityV1 {
    GuardAuthorityV1::from_signing_key_bytes(
        &CUSTODIAN_KEY,
        GuardImplementationId::from_bytes(bytes(9)),
    )
}

fn trusted_evidence(witness: GuardWitnessV1) -> GuardWitnessEvidence {
    GuardWitnessEvidence::authenticate(witness, guard_authority())
}

fn established_source(source: SealedSourceId) -> EstablishedIdentity {
    EstablishedIdentity::new(
        EstablishedIdentityKind::SealedSource,
        EvidenceIdentity::from_bytes(*source.as_bytes()),
    )
}

fn admit(
    manifest: &SignedRunManifestV1,
) -> PreAccessValidationResult<nemosyne_evaluation::evidence::ValidForOutcomeAccess> {
    let evidence = trusted_evidence(admission_witness(manifest));
    admit_for_outcome_access(
        manifest,
        &evidence,
        vec![established_source(manifest.claims().sealed_source_id())],
        &validator(),
    )
    .expect("fixture admission validation must be constructible")
}

#[test]
fn valid_manifest_witness_admission_and_receipt_round_trip() {
    let manifest = manifest_for(attempt(1), sealed_source(7));
    let PreAccessValidationResult::Valid(admission) = admit(&manifest) else {
        panic!("matching evidence must produce admission");
    };

    let payload = ExperimentReceiptPayloadV1::new(
        SchemaId::from_bytes(bytes(71)),
        EvidenceDisposition::Pass,
        b"reconstructible-evaluation-payload",
    )
    .expect("payload must be valid");
    let receipt = ValidExperimentReceiptV1::sign(admission, payload, &EVALUATOR_KEY)
        .expect("admission must permit a receipt");

    assert_eq!(receipt.manifest_content_id(), manifest.content_id());
    assert_eq!(receipt.payload().disposition(), EvidenceDisposition::Pass);
    assert_eq!(
        receipt.payload().bytes(),
        b"reconstructible-evaluation-payload"
    );
    receipt
        .verify(guard_authority())
        .expect("receipt signature and admission must verify");
    assert_eq!(receipt.manifest(), &manifest);
    assert_eq!(receipt.witness().claims().attempt_id(), attempt(1));
    let wrong_authority = GuardAuthorityV1::from_signing_key_bytes(
        &bytes(99),
        GuardImplementationId::from_bytes(bytes(9)),
    );
    assert_eq!(
        receipt.verify(wrong_authority),
        Err(EvidenceError::UntrustedGuardAuthority)
    );
}

#[test]
fn failed_and_inconclusive_receipts_remain_distinct_valid_post_admission_evidence() {
    for disposition in [EvidenceDisposition::Fail, EvidenceDisposition::Inconclusive] {
        let manifest = manifest_for(attempt(disposition as u8 + 10), sealed_source(7));
        let PreAccessValidationResult::Valid(admission) = admit(&manifest) else {
            panic!("matching evidence must produce admission");
        };
        let payload = ExperimentReceiptPayloadV1::new(
            SchemaId::from_bytes(bytes(72)),
            disposition,
            b"result",
        )
        .expect("payload must be valid");
        let receipt = ValidExperimentReceiptV1::sign(admission, payload, &EVALUATOR_KEY)
            .expect("admission must permit a receipt");
        assert_eq!(receipt.payload().disposition(), disposition);
        receipt
            .verify(guard_authority())
            .expect("receipt signature and admission must verify");
    }
}

#[test]
fn guarded_rejection_retains_commitment_not_raw_attempted_content() {
    let secret = b"api_key=never-retain-this-secret";
    let commitment = InputCommitment::complete(secret).expect("commitment must be valid");
    let rejected = RejectedAttemptV1::new(
        attempt(2),
        AttemptedArtifactKind::G1Envelope,
        commitment,
        vec![],
        SealedSourceState::Absent,
        ValidationStage::Structure,
        ValidationField::Manifest,
        RejectionReason::MalformedStructure,
    )
    .expect("rejection must be valid");
    let witness = GuardWitnessV1::sign(
        witness_claims(
            attempt(2),
            GuardSubjectV1::RejectedAttempt(RejectedGuardSubjectV1::new(
                AttemptedArtifactKind::G1Envelope,
                commitment,
                SealedSourceState::Absent,
            )),
            vec![principal(1)],
            vec![principal(2)],
        ),
        &CUSTODIAN_KEY,
    );

    let result: PreAccessValidationResult<()> =
        finalize_rejection(rejected, &trusted_evidence(witness), &validator());
    let PreAccessValidationResult::Rejected(receipt) = result else {
        panic!("matching rejection evidence must produce a rejection receipt");
    };
    receipt.verify().expect("rejection signature must verify");
    assert_eq!(receipt.attempt().input_commitment(), commitment);
    assert!(!format!("{receipt:?}").contains("never-retain-this-secret"));
}

#[test]
fn missing_invalid_and_wrong_subject_witnesses_are_custody_failures() {
    let source = sealed_source(7);
    let manifest = manifest_for(attempt(3), source);
    let identities = vec![established_source(source)];

    let missing = admit_for_outcome_access(
        &manifest,
        &GuardWitnessEvidence::missing(),
        identities.clone(),
        &validator(),
    )
    .expect("missing witness must still produce a terminal result");
    assert_custody_error(missing, GuardEvidenceError::Missing);

    let valid_witness = admission_witness(&manifest);
    let invalid = GuardWitnessEvidence::from_signed_parts(
        valid_witness.claims().clone(),
        valid_witness.content_id(),
        valid_witness.digest(),
        valid_witness.custodian_id(),
        valid_witness.custodian_key(),
        EvidenceSignature::from_bytes(bytes(99)),
        guard_authority(),
    );
    let invalid_result =
        admit_for_outcome_access(&manifest, &invalid, identities.clone(), &validator())
            .expect("invalid witness must still produce a terminal result");
    assert_custody_error(invalid_result, GuardEvidenceError::Invalid);

    let rejection_subject = GuardSubjectV1::RejectedAttempt(RejectedGuardSubjectV1::new(
        AttemptedArtifactKind::G1RunManifest,
        InputCommitment::complete(b"other").expect("commitment must be valid"),
        SealedSourceState::Established(source),
    ));
    let wrong_subject = trusted_evidence(GuardWitnessV1::sign(
        witness_claims(
            attempt(3),
            rejection_subject,
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        &CUSTODIAN_KEY,
    ));
    let wrong_result =
        admit_for_outcome_access(&manifest, &wrong_subject, identities, &validator())
            .expect("wrong subject must still produce a terminal result");
    assert_custody_error(wrong_result, GuardEvidenceError::WrongSubject);
}

#[test]
fn a_self_consistent_witness_from_an_untrusted_custodian_is_not_authenticated() {
    let source = sealed_source(7);
    let manifest = manifest_for(attempt(31), source);
    let witness = GuardWitnessV1::sign(
        witness_claims(
            attempt(31),
            GuardSubjectV1::ValidatedRun(ValidatedRunGuardSubjectV1::from_manifest(&manifest)),
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        &[55; 32],
    );
    let result = admit_for_outcome_access(
        &manifest,
        &trusted_evidence(witness),
        vec![established_source(source)],
        &validator(),
    )
    .expect("untrusted authority must produce a terminal custody result");
    assert_custody_error(result, GuardEvidenceError::Invalid);
}

#[test]
fn admission_and_rejection_mismatches_use_fixed_first_field_precedence() {
    let source = sealed_source(7);
    let manifest = manifest_for(attempt(4), source);
    let other_manifest = manifest_for(attempt(99), sealed_source(88));
    let multi_mismatch_witness = trusted_evidence(GuardWitnessV1::sign(
        witness_claims(
            attempt(99),
            GuardSubjectV1::ValidatedRun(ValidatedRunGuardSubjectV1::from_manifest(
                &other_manifest,
            )),
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        &CUSTODIAN_KEY,
    ));
    let result = admit_for_outcome_access(
        &manifest,
        &multi_mismatch_witness,
        vec![established_source(source)],
        &validator(),
    )
    .expect("mismatch must produce custody failure");
    assert_custody_error(
        result,
        GuardEvidenceError::AdmissionMismatch {
            field: AdmissionJoinField::AttemptId,
        },
    );

    let commitment = InputCommitment::complete(b"attempt").expect("commitment must be valid");
    let rejected = RejectedAttemptV1::new(
        attempt(5),
        AttemptedArtifactKind::G1Envelope,
        commitment,
        vec![],
        SealedSourceState::Absent,
        ValidationStage::Structure,
        ValidationField::Manifest,
        RejectionReason::InvalidField,
    )
    .expect("rejection must be valid");
    let replay = trusted_evidence(GuardWitnessV1::sign(
        witness_claims(
            attempt(6),
            GuardSubjectV1::RejectedAttempt(RejectedGuardSubjectV1::new(
                AttemptedArtifactKind::G9Protocol,
                InputCommitment::complete(b"different").expect("commitment must be valid"),
                SealedSourceState::Absent,
            )),
            vec![principal(1)],
            vec![principal(2)],
        ),
        &CUSTODIAN_KEY,
    ));
    let rejection_result: PreAccessValidationResult<()> =
        finalize_rejection(rejected, &replay, &validator());
    let PreAccessValidationResult::CustodyEvidenceUnavailable(record) = rejection_result else {
        panic!("cross-attempt replay must not produce a rejection receipt");
    };
    assert_eq!(
        record.error(),
        GuardEvidenceError::RejectionMismatch {
            field: RejectionJoinField::AttemptId
        }
    );
    record
        .verify()
        .expect("custody record signature must verify");
}

#[test]
fn every_rejection_join_field_is_reported_without_fallback() {
    let base_commitment = InputCommitment::complete(b"attempt").expect("commitment must be valid");
    let cases = [
        (
            attempt(41),
            AttemptedArtifactKind::G1Envelope,
            base_commitment,
            SealedSourceState::Absent,
            RejectionJoinField::AttemptId,
        ),
        (
            attempt(40),
            AttemptedArtifactKind::G9Protocol,
            base_commitment,
            SealedSourceState::Absent,
            RejectionJoinField::AttemptedKind,
        ),
        (
            attempt(40),
            AttemptedArtifactKind::G1Envelope,
            InputCommitment::complete(b"other").expect("commitment must be valid"),
            SealedSourceState::Absent,
            RejectionJoinField::InputCommitment,
        ),
        (
            attempt(40),
            AttemptedArtifactKind::G1Envelope,
            base_commitment,
            SealedSourceState::Established(sealed_source(9)),
            RejectionJoinField::EstablishedSealedSource,
        ),
    ];

    for (witness_attempt, kind, commitment, source_state, expected_field) in cases {
        let rejected = RejectedAttemptV1::new(
            attempt(40),
            AttemptedArtifactKind::G1Envelope,
            base_commitment,
            vec![],
            SealedSourceState::Absent,
            ValidationStage::Structure,
            ValidationField::Manifest,
            RejectionReason::InvalidField,
        )
        .expect("rejection must be valid");
        let witness = GuardWitnessV1::sign(
            witness_claims(
                witness_attempt,
                GuardSubjectV1::RejectedAttempt(RejectedGuardSubjectV1::new(
                    kind,
                    commitment,
                    source_state,
                )),
                vec![principal(1)],
                vec![principal(2)],
            ),
            &CUSTODIAN_KEY,
        );
        let result: PreAccessValidationResult<()> =
            finalize_rejection(rejected, &trusted_evidence(witness), &validator());
        let PreAccessValidationResult::CustodyEvidenceUnavailable(record) = result else {
            panic!("mismatch must produce custody failure");
        };
        assert_eq!(
            record.error(),
            GuardEvidenceError::RejectionMismatch {
                field: expected_field
            }
        );
    }
}

#[test]
fn every_reachable_admission_join_field_is_reported_without_fallback() {
    let source = sealed_source(7);
    let manifest = manifest_for(attempt(50), source);
    let base = ValidatedRunGuardSubjectV1::from_manifest(&manifest);
    let cases = vec![
        (
            AdmissionJoinField::AttemptId,
            custom_admission_witness(
                attempt(51),
                base,
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::RunManifestContentId,
            custom_admission_witness(
                attempt(50),
                ValidatedRunGuardSubjectV1::new(
                    ArtifactContentId::from_bytes(bytes(99)),
                    manifest.digest(),
                    manifest.signature(),
                    source,
                ),
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::RunManifestDigest,
            custom_admission_witness(
                attempt(50),
                ValidatedRunGuardSubjectV1::new(
                    manifest.content_id(),
                    EvidenceDigest::from_bytes(bytes(99)),
                    manifest.signature(),
                    source,
                ),
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::RunManifestSignature,
            custom_admission_witness(
                attempt(50),
                ValidatedRunGuardSubjectV1::new(
                    manifest.content_id(),
                    manifest.digest(),
                    EvidenceSignature::from_bytes(bytes(99)),
                    source,
                ),
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::SealedSourceId,
            custom_admission_witness(
                attempt(50),
                ValidatedRunGuardSubjectV1::new(
                    manifest.content_id(),
                    manifest.digest(),
                    manifest.signature(),
                    sealed_source(99),
                ),
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::ValidationWindowStart,
            custom_admission_witness(
                attempt(50),
                base,
                window(99, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::ValidationWindowEnd,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 201),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::ValidationPrincipalSet,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 200),
                vec![principal(1), principal(9)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::AnalysisPrincipalSet,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(9)],
                ledger(1, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::OutcomeAccessLedgerHead,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(9, 2),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::OutcomeAccessLedgerTail,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 9),
                ledger(3, 4),
            ),
        ),
        (
            AdmissionJoinField::AnalysisJobLedgerHead,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(9, 4),
            ),
        ),
        (
            AdmissionJoinField::AnalysisJobLedgerTail,
            custom_admission_witness(
                attempt(50),
                base,
                window(100, 200),
                vec![principal(1), principal(2)],
                vec![principal(3), principal(4)],
                ledger(1, 2),
                ledger(3, 9),
            ),
        ),
    ];

    for (expected_field, witness) in cases {
        let result = admit_for_outcome_access(
            &manifest,
            &trusted_evidence(witness),
            vec![established_source(source)],
            &validator(),
        )
        .expect("mismatch must produce a custody result");
        assert_custody_error(
            result,
            GuardEvidenceError::AdmissionMismatch {
                field: expected_field,
            },
        );
    }
}

#[test]
fn canonical_order_makes_manifest_witness_and_rejection_permutation_invariant() {
    let source = sealed_source(7);
    let first_manifest = SignedRunManifestV1::sign(
        manifest_claims(
            attempt(7),
            source,
            vec![principal(2), principal(1)],
            vec![principal(4), principal(3)],
        ),
        b"same",
        &MANIFEST_KEY,
    )
    .expect("manifest must be valid");
    let second_manifest = SignedRunManifestV1::sign(
        manifest_claims(
            attempt(7),
            source,
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        b"same",
        &MANIFEST_KEY,
    )
    .expect("manifest must be valid");
    assert_eq!(first_manifest, second_manifest);

    let first_witness = GuardWitnessV1::sign(
        witness_claims(
            attempt(7),
            GuardSubjectV1::ValidatedRun(ValidatedRunGuardSubjectV1::from_manifest(
                &first_manifest,
            )),
            vec![principal(2), principal(1)],
            vec![principal(4), principal(3)],
        ),
        &CUSTODIAN_KEY,
    );
    let second_witness = GuardWitnessV1::sign(
        witness_claims(
            attempt(7),
            GuardSubjectV1::ValidatedRun(ValidatedRunGuardSubjectV1::from_manifest(
                &second_manifest,
            )),
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        &CUSTODIAN_KEY,
    );
    assert_eq!(first_witness, second_witness);

    let config = EstablishedIdentity::new(
        EstablishedIdentityKind::Configuration,
        EvidenceIdentity::from_bytes(bytes(1)),
    );
    let source_identity = established_source(source);
    let commitment = InputCommitment::complete(b"same").expect("commitment must be valid");
    let first_rejection = RejectedAttemptV1::new(
        attempt(7),
        AttemptedArtifactKind::G1Envelope,
        commitment,
        vec![source_identity, config],
        SealedSourceState::Established(source),
        ValidationStage::Structure,
        ValidationField::Manifest,
        RejectionReason::InvalidField,
    )
    .expect("rejection must be valid");
    let second_rejection = RejectedAttemptV1::new(
        attempt(7),
        AttemptedArtifactKind::G1Envelope,
        commitment,
        vec![config, source_identity],
        SealedSourceState::Established(source),
        ValidationStage::Structure,
        ValidationField::Manifest,
        RejectionReason::InvalidField,
    )
    .expect("rejection must be valid");
    assert_eq!(first_rejection, second_rejection);
}

#[test]
fn constructors_reject_unknown_versions_invalid_bounds_and_inconsistent_state() {
    assert_eq!(
        EvidenceSchemaVersion::try_from(1).expect("version 1 must be supported"),
        EvidenceSchemaVersion::V1
    );
    assert!(matches!(
        EvidenceSchemaVersion::try_from(2),
        Err(EvidenceError::UnknownSchemaVersion { version: 2 })
    ));
    assert!(matches!(
        ValidationWindow::new(
            TrustedTimestamp::from_unix_seconds(2),
            TrustedTimestamp::from_unix_seconds(1),
        ),
        Err(EvidenceError::InvalidValidationWindow)
    ));
    assert!(matches!(
        RunManifestClaimsV1::new(
            attempt(1),
            AttemptedArtifactKind::G1Envelope,
            sealed_source(1),
            window(1, 2),
            vec![principal(1)],
            vec![principal(2)],
            CapabilityIssuanceState::NotIssued,
            ledger(1, 2),
            ledger(3, 4),
        ),
        Err(EvidenceError::ExpectedRunManifest)
    ));
    assert!(matches!(
        RunManifestClaimsV1::new(
            attempt(1),
            AttemptedArtifactKind::G1RunManifest,
            sealed_source(1),
            window(1, 2),
            vec![principal(1), principal(1)],
            vec![principal(2)],
            CapabilityIssuanceState::NotIssued,
            ledger(1, 2),
            ledger(3, 4),
        ),
        Err(EvidenceError::DuplicatePrincipal)
    ));

    let commitment = InputCommitment::incomplete(
        b"prefix",
        ValidationStage::Envelope,
        ValidationField::Schema,
    )
    .expect("commitment must be valid");
    assert!(matches!(
        RejectedAttemptV1::new(
            attempt(1),
            AttemptedArtifactKind::G1Envelope,
            commitment,
            vec![],
            SealedSourceState::Absent,
            ValidationStage::Structure,
            ValidationField::Manifest,
            RejectionReason::MalformedStructure,
        ),
        Err(EvidenceError::InconsistentCommitmentLocation)
    ));
    assert!(matches!(
        RejectedAttemptV1::new(
            attempt(1),
            AttemptedArtifactKind::G1Envelope,
            InputCommitment::complete(b"x").expect("commitment must be valid"),
            vec![],
            SealedSourceState::Established(sealed_source(1)),
            ValidationStage::Structure,
            ValidationField::Manifest,
            RejectionReason::MalformedStructure,
        ),
        Err(EvidenceError::InvalidEstablishedSealedSource)
    ));

    let too_many_principals = (0..257)
        .map(|value| {
            let mut id = [0; 32];
            id[30..].copy_from_slice(&(value as u16).to_be_bytes());
            PrincipalId::from_bytes(id)
        })
        .collect();
    assert!(matches!(
        RunManifestClaimsV1::new(
            attempt(1),
            AttemptedArtifactKind::G1RunManifest,
            sealed_source(1),
            window(1, 2),
            too_many_principals,
            vec![principal(2)],
            CapabilityIssuanceState::NotIssued,
            ledger(1, 2),
            ledger(3, 4),
        ),
        Err(EvidenceError::TooManyPrincipals {
            actual: 257,
            maximum: 256
        })
    ));

    assert!(matches!(
        ExperimentReceiptPayloadV1::new(
            SchemaId::from_bytes(bytes(1)),
            EvidenceDisposition::Pass,
            b"",
        ),
        Err(EvidenceError::EmptyPayload)
    ));
}

#[test]
fn content_and_signature_domains_distinguish_payloads_and_artifact_roles() {
    let first = manifest_for(attempt(8), sealed_source(7));
    let second = SignedRunManifestV1::sign(
        manifest_claims(
            attempt(8),
            sealed_source(7),
            vec![principal(1), principal(2)],
            vec![principal(3), principal(4)],
        ),
        b"changed",
        &MANIFEST_KEY,
    )
    .expect("manifest must be valid");
    assert_ne!(first.content_id(), second.content_id());
    assert_ne!(first.digest(), second.digest());
    assert_ne!(
        first.content_id().as_bytes(),
        InputCommitment::complete(first.payload())
            .expect("commitment must be valid")
            .digest()
            .as_bytes()
    );
}

fn assert_custody_error<T>(result: PreAccessValidationResult<T>, expected: GuardEvidenceError) {
    let PreAccessValidationResult::CustodyEvidenceUnavailable(record) = result else {
        panic!("expected custody failure");
    };
    assert_eq!(record.error(), expected);
    record
        .verify()
        .expect("custody record signature must verify");
}

#[test]
fn invalid_untrusted_witness_parts_are_discarded_without_raw_retention() {
    let manifest = manifest_for(attempt(9), sealed_source(7));
    let witness = admission_witness(&manifest);
    let invalid = GuardWitnessEvidence::from_signed_parts(
        witness.claims().clone(),
        ArtifactContentId::from_bytes(bytes(0)),
        EvidenceDigest::from_bytes(bytes(0)),
        witness.custodian_id(),
        witness.custodian_key(),
        EvidenceSignature::from_bytes(bytes(0)),
        guard_authority(),
    );
    let rendered = format!("{invalid:?}");
    assert_eq!(rendered, "GuardWitnessEvidence(Invalid)");
}
