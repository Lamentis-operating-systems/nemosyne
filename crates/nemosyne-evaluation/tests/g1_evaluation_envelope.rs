//! Public-contract tests for the pre-outcome G1 evaluation envelope.

use nemosyne_evaluation::evidence::{
    AdmittedG1RunV1, AttemptId, AttemptedArtifactKind, CapabilityIssuanceState, EvidenceIdentity,
    G1ArtifactBindingV1, G1ArtifactKind, G1AttentionMatchingV1, G1Condition, G1ConditionArtifactV1,
    G1CriticalFailureBoundV1, G1CriticalFailureClass, G1DesignV1, G1Domain, G1EnvelopeError,
    G1ExecutionBindingV1, G1ExecutionIdentity, G1ExposureRequirementV1, G1ExposureScope,
    G1PopulationV1, G1RunArtifactBindingV1, G1RunArtifactKind, G1SubgroupV1, G1TaskId, G1TaskV1,
    G1ThresholdKey, G1ThresholdV1, GuardImplementationId, GuardSubjectV1, GuardWitnessClaimsV1,
    GuardWitnessEvidence, GuardWitnessV1, LedgerBoundary, LedgerCommitment, PrincipalId,
    RunManifestClaimsV1, SealedSourceId, SignedG1EvaluationEnvelopeV1, TrustedTimestamp,
    ValidForOutcomeAccess, ValidatedRunGuardSubjectV1, ValidationWindow, finalize_g1_run_manifest,
};

const ENVELOPE_KEY: [u8; 32] = [201; 32];
const RUN_KEY: [u8; 32] = [202; 32];
const CUSTODIAN_KEY: [u8; 32] = [203; 32];

fn bytes(value: u8) -> [u8; 32] {
    [value; 32]
}

fn identity(value: u8) -> EvidenceIdentity {
    EvidenceIdentity::from_bytes(bytes(value))
}

fn tasks() -> Vec<G1TaskV1> {
    vec![
        G1TaskV1::new(
            G1TaskId::new(10),
            100,
            G1Domain::ContextDependent,
            true,
            G1SubgroupV1::new(1, 10, 100),
            1,
        )
        .expect("fixture task must be valid"),
        G1TaskV1::new(
            G1TaskId::new(20),
            200,
            G1Domain::ContextDependent,
            false,
            G1SubgroupV1::new(1, 10, 100),
            1,
        )
        .expect("fixture task must be valid"),
        G1TaskV1::new(
            G1TaskId::new(30),
            300,
            G1Domain::ContextIndependent,
            false,
            G1SubgroupV1::new(2, 20, 200),
            1,
        )
        .expect("fixture task must be valid"),
    ]
}

fn exposures() -> Vec<G1ExposureRequirementV1> {
    let dependent = G1SubgroupV1::new(1, 10, 100);
    let independent = G1SubgroupV1::new(2, 20, 200);
    vec![
        G1ExposureRequirementV1::new(G1ExposureScope::ContextDependent, None, 2, 2)
            .expect("fixture exposure must be valid"),
        G1ExposureRequirementV1::new(G1ExposureScope::ContextDependent, Some(dependent), 2, 2)
            .expect("fixture exposure must be valid"),
        G1ExposureRequirementV1::new(G1ExposureScope::ContextIndependent, None, 1, 1)
            .expect("fixture exposure must be valid"),
        G1ExposureRequirementV1::new(G1ExposureScope::ContextIndependent, Some(independent), 1, 1)
            .expect("fixture exposure must be valid"),
        G1ExposureRequirementV1::new(G1ExposureScope::ExpectationEligible, None, 1, 1)
            .expect("fixture exposure must be valid"),
        G1ExposureRequirementV1::new(G1ExposureScope::ExpectationEligible, Some(dependent), 1, 1)
            .expect("fixture exposure must be valid"),
    ]
}

fn conditions() -> Vec<G1ConditionArtifactV1> {
    G1Condition::all()
        .iter()
        .copied()
        .enumerate()
        .map(|(index, condition)| {
            G1ConditionArtifactV1::new(condition, identity(index as u8 + 1))
                .expect("fixture condition must be valid")
        })
        .collect()
}

fn artifacts() -> Vec<G1ArtifactBindingV1> {
    G1ArtifactKind::required()
        .iter()
        .copied()
        .enumerate()
        .map(|(index, kind)| {
            G1ArtifactBindingV1::new(kind, identity(index as u8 + 20))
                .expect("fixture artifact must be valid")
        })
        .collect()
}

fn thresholds() -> Vec<G1ThresholdV1> {
    G1ThresholdKey::required_keys()
        .into_iter()
        .map(|key| G1ThresholdV1::new(key, 0.125).expect("fixture threshold must be valid"))
        .collect()
}

fn critical_failures() -> Vec<G1CriticalFailureBoundV1> {
    G1CriticalFailureClass::all()
        .iter()
        .copied()
        .map(|class| {
            G1CriticalFailureBoundV1::new(class, 7, 0.125)
                .expect("fixture critical bound must be valid")
        })
        .collect()
}

fn design() -> G1DesignV1 {
    G1DesignV1::new(
        conditions(),
        G1AttentionMatchingV1::new(512, 64, 2).expect("fixture match must be valid"),
        G1PopulationV1::new(tasks(), 3, exposures()).expect("fixture population must be valid"),
        thresholds(),
        critical_failures(),
        artifacts(),
    )
    .expect("fixture design must be valid")
}

fn run_artifacts() -> Vec<G1RunArtifactBindingV1> {
    G1RunArtifactKind::required()
        .iter()
        .copied()
        .enumerate()
        .map(|(index, kind)| {
            G1RunArtifactBindingV1::new(kind, identity(index as u8 + 100))
                .expect("fixture run artifact must be valid")
        })
        .collect()
}

fn ledger(head: u8, tail: u8) -> LedgerBoundary {
    LedgerBoundary::new(
        LedgerCommitment::from_bytes(bytes(head)),
        LedgerCommitment::from_bytes(bytes(tail)),
    )
}

fn window() -> ValidationWindow {
    ValidationWindow::new(
        TrustedTimestamp::from_unix_seconds(1_000),
        TrustedTimestamp::from_unix_seconds(2_000),
    )
    .expect("fixture window must be valid")
}

fn claims() -> RunManifestClaimsV1 {
    RunManifestClaimsV1::new(
        AttemptId::from_bytes(bytes(210)),
        AttemptedArtifactKind::G1RunManifest,
        SealedSourceId::from_bytes(bytes(211)),
        window(),
        vec![PrincipalId::from_bytes(bytes(212))],
        vec![PrincipalId::from_bytes(bytes(213))],
        CapabilityIssuanceState::NotIssued,
        ledger(214, 215),
        ledger(216, 217),
    )
    .expect("fixture claims must be valid")
}

fn admitted_run(
    envelope: &SignedG1EvaluationEnvelopeV1,
    execution: &G1ExecutionBindingV1,
) -> ValidForOutcomeAccess {
    let manifest = finalize_g1_run_manifest(envelope, execution, claims(), &RUN_KEY)
        .expect("complete G1 run manifest must be valid");
    let subject = ValidatedRunGuardSubjectV1::from_manifest(&manifest);
    let witness = GuardWitnessV1::sign(
        GuardWitnessClaimsV1::new(
            manifest.claims().attempt_id(),
            GuardSubjectV1::ValidatedRun(subject),
            window(),
            vec![PrincipalId::from_bytes(bytes(212))],
            vec![PrincipalId::from_bytes(bytes(213))],
            CapabilityIssuanceState::NotIssued,
            ledger(214, 215),
            ledger(216, 217),
            GuardImplementationId::from_bytes(bytes(221)),
        )
        .expect("fixture witness claims must be valid"),
        &CUSTODIAN_KEY,
    );
    let authority = nemosyne_evaluation::evidence::GuardAuthorityV1::from_signing_key_bytes(
        &CUSTODIAN_KEY,
        GuardImplementationId::from_bytes(bytes(221)),
    );
    let evidence = GuardWitnessEvidence::authenticate(witness, authority);
    ValidForOutcomeAccess::new(&manifest, &evidence)
        .expect("the exact manifest and matching guard witness must admit access")
}

#[test]
fn complete_design_is_canonical_signed_and_content_identified() {
    let signed = SignedG1EvaluationEnvelopeV1::sign(design(), &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    signed.verify().expect("valid envelope must verify");
    assert_eq!(signed.design().conditions().len(), 7);
    assert_eq!(signed.design().thresholds().len(), 52);
    assert_eq!(signed.design().artifacts().len(), 49);

    let mut permuted_conditions = conditions();
    permuted_conditions.reverse();
    let mut permuted_thresholds = thresholds();
    permuted_thresholds.reverse();
    let mut permuted_artifacts = artifacts();
    permuted_artifacts.reverse();
    let mut permuted_exposures = exposures();
    permuted_exposures.reverse();
    let mut permuted_tasks = tasks();
    permuted_tasks.reverse();
    let permuted = G1DesignV1::new(
        permuted_conditions,
        G1AttentionMatchingV1::new(512, 64, 2).expect("fixture match must be valid"),
        G1PopulationV1::new(permuted_tasks, 3, permuted_exposures)
            .expect("fixture population must be valid"),
        permuted_thresholds,
        critical_failures().into_iter().rev().collect(),
        permuted_artifacts,
    )
    .expect("permuted design must be valid");
    let signed_permuted = SignedG1EvaluationEnvelopeV1::sign(permuted, &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");

    assert_eq!(signed.content_id(), signed_permuted.content_id());
    assert_eq!(signed.digest(), signed_permuted.digest());
    assert_eq!(signed.signature(), signed_permuted.signature());
}

#[test]
fn design_mutation_changes_the_content_identity() {
    let first = SignedG1EvaluationEnvelopeV1::sign(design(), &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    let changed = G1DesignV1::new(
        conditions(),
        G1AttentionMatchingV1::new(512, 65, 2).expect("fixture match must be valid"),
        G1PopulationV1::new(tasks(), 3, exposures()).expect("fixture population must be valid"),
        thresholds(),
        critical_failures(),
        artifacts(),
    )
    .expect("changed design must be valid");
    let second = SignedG1EvaluationEnvelopeV1::sign(changed, &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    assert_ne!(first.content_id(), second.content_id());
}

#[test]
fn envelope_rejects_missing_closed_fields() {
    let mut missing_condition = conditions();
    missing_condition.pop();
    assert_eq!(
        G1DesignV1::new(
            missing_condition,
            G1AttentionMatchingV1::new(512, 64, 2).expect("fixture match must be valid"),
            G1PopulationV1::new(tasks(), 3, exposures()).expect("fixture population must be valid"),
            thresholds(),
            critical_failures(),
            artifacts(),
        ),
        Err(G1EnvelopeError::MissingCondition {
            condition: G1Condition::Abstain,
        })
    );

    let mut missing_threshold = thresholds();
    let removed = missing_threshold
        .pop()
        .expect("fixture has thresholds")
        .key();
    assert!(matches!(
        G1DesignV1::new(
            conditions(),
            G1AttentionMatchingV1::new(512, 64, 2).expect("fixture match must be valid"),
            G1PopulationV1::new(tasks(), 3, exposures())
                .expect("fixture population must be valid"),
            missing_threshold,
            critical_failures(),
            artifacts(),
        ),
        Err(G1EnvelopeError::MissingThreshold { key }) if key == removed
    ));

    let mut missing_artifact = artifacts();
    missing_artifact.pop();
    assert!(matches!(
        G1DesignV1::new(
            conditions(),
            G1AttentionMatchingV1::new(512, 64, 2).expect("fixture match must be valid"),
            G1PopulationV1::new(tasks(), 3, exposures()).expect("fixture population must be valid"),
            thresholds(),
            critical_failures(),
            missing_artifact,
        ),
        Err(G1EnvelopeError::MissingArtifact {
            kind: G1ArtifactKind::CustodyPolicy,
        })
    ));

    let mut missing_critical = critical_failures();
    missing_critical.pop();
    assert!(matches!(
        G1DesignV1::new(
            conditions(),
            G1AttentionMatchingV1::new(512, 64, 2).expect("fixture match must be valid"),
            G1PopulationV1::new(tasks(), 3, exposures()).expect("fixture population must be valid"),
            thresholds(),
            missing_critical,
            artifacts(),
        ),
        Err(G1EnvelopeError::MissingCriticalFailure {
            class: G1CriticalFailureClass::Anchoring,
        })
    ));
}

#[test]
fn population_rejects_invalid_partition_mass_and_exposure() {
    let invalid_task = G1TaskV1::new(
        G1TaskId::new(99),
        1,
        G1Domain::ContextIndependent,
        true,
        G1SubgroupV1::new(1, 1, 1),
        1,
    );
    assert_eq!(
        invalid_task,
        Err(G1EnvelopeError::ExpectationTaskOutsideDependentDomain {
            task_id: G1TaskId::new(99),
        })
    );
    assert_eq!(
        G1PopulationV1::new(tasks(), 4, exposures()),
        Err(G1EnvelopeError::InvalidDesignWeightMass)
    );

    let mut incomplete = exposures();
    incomplete.retain(|exposure| {
        !(exposure.scope() == G1ExposureScope::ExpectationEligible && exposure.subgroup().is_none())
    });
    assert_eq!(
        G1PopulationV1::new(tasks(), 3, incomplete),
        Err(G1EnvelopeError::MissingExposure {
            scope: G1ExposureScope::ExpectationEligible,
            subgroup: None,
        })
    );
}

#[test]
fn threshold_domains_are_strict_and_key_specific() {
    for value in [f64::NAN, f64::INFINITY, -0.1, 1.0] {
        assert!(matches!(
            G1ThresholdV1::new(G1ThresholdKey::CorrectContribution, value),
            Err(G1EnvelopeError::InvalidThreshold { .. })
        ));
    }
    assert!(matches!(
        G1ThresholdV1::new(G1ThresholdKey::CorrectContribution, 0.0),
        Err(G1EnvelopeError::InvalidThreshold { .. })
    ));
    let negative_zero = G1ThresholdV1::new(G1ThresholdKey::CorrectAnchoringDifference, -0.0)
        .expect("paired maximum may be zero");
    assert_eq!(negative_zero.value().to_bits(), 0.0_f64.to_bits());
    assert!(matches!(
        G1CriticalFailureBoundV1::new(G1CriticalFailureClass::Anchoring, 1, 1.0),
        Err(G1EnvelopeError::InvalidCriticalFailureRate {
            class: G1CriticalFailureClass::Anchoring,
        })
    ));
}

#[test]
fn finalized_run_requires_and_binds_the_complete_envelope_and_guard_witness() {
    let envelope = SignedG1EvaluationEnvelopeV1::sign(design(), &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    let execution = G1ExecutionBindingV1::new(
        G1ExecutionIdentity::from_bytes(bytes(220))
            .expect("fixture execution identity must be valid"),
        run_artifacts(),
    )
    .expect("fixture execution binding must be valid");
    admitted_run(&envelope, &execution);
}

#[test]
fn run_binding_requires_every_exact_execution_artifact() {
    let mut incomplete = run_artifacts();
    incomplete.pop();
    assert_eq!(
        G1ExecutionBindingV1::new(
            G1ExecutionIdentity::from_bytes(bytes(220))
                .expect("fixture execution identity must be valid"),
            incomplete,
        ),
        Err(G1EnvelopeError::MissingRunArtifact {
            kind: G1RunArtifactKind::TokenMatchingAudit,
        })
    );
}

#[test]
fn admitted_g1_run_binds_the_exact_envelope_and_execution() {
    let envelope = SignedG1EvaluationEnvelopeV1::sign(design(), &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    let execution = G1ExecutionBindingV1::new(
        G1ExecutionIdentity::from_bytes(bytes(220))
            .expect("fixture execution identity must be valid"),
        run_artifacts(),
    )
    .expect("fixture execution binding must be valid");
    let admission = admitted_run(&envelope, &execution);

    let bound = AdmittedG1RunV1::bind(admission, &envelope, &execution)
        .expect("the exact signed design and execution must bind");

    assert_eq!(bound.envelope(), &envelope);
    assert_eq!(bound.execution(), &execution);
    assert_eq!(bound.envelope_content_id(), envelope.content_id());
    assert_eq!(bound.execution_identity(), execution.execution_identity());
    assert_eq!(
        bound.admission().manifest_content_id(),
        bound.admission().manifest().content_id()
    );
}

#[test]
fn admitted_g1_run_rejects_a_different_execution_binding() {
    let envelope = SignedG1EvaluationEnvelopeV1::sign(design(), &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    let execution = G1ExecutionBindingV1::new(
        G1ExecutionIdentity::from_bytes(bytes(220))
            .expect("fixture execution identity must be valid"),
        run_artifacts(),
    )
    .expect("fixture execution binding must be valid");
    let different_execution = G1ExecutionBindingV1::new(
        G1ExecutionIdentity::from_bytes(bytes(222))
            .expect("fixture execution identity must be valid"),
        run_artifacts(),
    )
    .expect("fixture execution binding must be valid");
    let admission = admitted_run(&envelope, &execution);

    assert_eq!(
        AdmittedG1RunV1::bind(admission, &envelope, &different_execution),
        Err(G1EnvelopeError::RunBindingMismatch)
    );
}

#[test]
fn admitted_g1_run_rejects_a_different_signed_envelope() {
    let envelope = SignedG1EvaluationEnvelopeV1::sign(design(), &ENVELOPE_KEY)
        .expect("fixture envelope must be bounded");
    let differently_signed_envelope = SignedG1EvaluationEnvelopeV1::sign(design(), &[204; 32])
        .expect("fixture envelope must be bounded");
    let execution = G1ExecutionBindingV1::new(
        G1ExecutionIdentity::from_bytes(bytes(220))
            .expect("fixture execution identity must be valid"),
        run_artifacts(),
    )
    .expect("fixture execution binding must be valid");
    let admission = admitted_run(&envelope, &execution);

    assert_eq!(
        AdmittedG1RunV1::bind(admission, &differently_signed_envelope, &execution),
        Err(G1EnvelopeError::RunBindingMismatch)
    );
}
