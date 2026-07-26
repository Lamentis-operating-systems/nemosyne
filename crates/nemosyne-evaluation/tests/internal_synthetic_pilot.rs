//! Structural tests for the non-promotional internal synthetic-pilot contract.

use nemosyne_evaluation::synthetic_pilot::{
    FrozenSyntheticPilotV1, GeneratedPilotTaskV1, GenerationAttemptDispositionV1,
    GenerationAttemptId, GenerationAttemptV1, GenerationLogV1, GenerationManifestV1,
    ModelCostPrivacyDisclosureV1, PilotCellResultV1, PilotCondition, PilotConditionArtifactV1,
    PilotConditionSetV1, PilotCorpusV1, PilotObservationV1, PilotRunnerIdentityV1,
    PilotRunnerManifestV1, PilotScoringManifestV1, PilotTaskId, SyntheticPilotDisposition,
    SyntheticPilotError, SyntheticPilotReceiptV1,
};

fn disclosure(version: &str) -> ModelCostPrivacyDisclosureV1 {
    ModelCostPrivacyDisclosureV1::new(
        "fixture-provider",
        "fixture-model",
        version,
        "maximum 0 fixture currency",
        "local fixture process",
        "no retention",
        "structural test bytes only",
    )
    .expect("valid disclosure")
}

fn generation(version: &str) -> GenerationManifestV1 {
    GenerationManifestV1::new(
        disclosure(version),
        b"generate a synthetic constraint-following task".as_slice(),
        "fixture-tokenizer-v1",
        "temperature=0;top_p=1",
        "seed=17",
        "tools=none;network=none",
        "fixture-runtime-v1",
    )
    .expect("valid generation manifest")
}

fn task_and_log(generation: &GenerationManifestV1) -> (GenerationLogV1, Vec<GeneratedPilotTaskV1>) {
    let task_id = PilotTaskId::new(1).expect("nonzero task");
    let accepted_id = GenerationAttemptId::new(1).expect("nonzero attempt");
    let rejected_id = GenerationAttemptId::new(2).expect("nonzero attempt");
    let attempts = vec![
        GenerationAttemptV1::new(
            accepted_id,
            b"attempt-one-input".as_slice(),
            b"attempt-one-complete-output".as_slice(),
            GenerationAttemptDispositionV1::Accepted(task_id),
        )
        .expect("accepted attempt"),
        GenerationAttemptV1::new(
            rejected_id,
            b"attempt-two-input".as_slice(),
            b"attempt-two-complete-output".as_slice(),
            GenerationAttemptDispositionV1::Rejected {
                reason: "duplicate semantic case".into(),
            },
        )
        .expect("rejected attempt"),
    ];
    let log = GenerationLogV1::new(attempts, generation).expect("valid generation log");
    let tasks = vec![
        GeneratedPilotTaskV1::new(
            task_id,
            accepted_id,
            b"change the fixture safely".as_slice(),
            b"repository has an existing constraint".as_slice(),
            b"do not change the public fixture identifier".as_slice(),
            b"constraint_followed iff identifier remains unchanged".as_slice(),
        )
        .expect("valid task"),
    ];
    (log, tasks)
}

fn condition_set(corpus: &PilotCorpusV1) -> PilotConditionSetV1 {
    let task_id = corpus.tasks()[0].id();
    let artifacts = PilotCondition::all()
        .iter()
        .map(|condition| {
            let token_count = match condition {
                PilotCondition::Prompt | PilotCondition::Situation => None,
                _ => Some(16),
            };
            PilotConditionArtifactV1::new(
                task_id,
                *condition,
                format!("exact input for {}", condition.label()).into_bytes(),
                token_count,
            )
            .expect("valid condition artifact")
        })
        .collect();
    PilotConditionSetV1::new(artifacts, corpus).expect("complete condition set")
}

fn scoring() -> PilotScoringManifestV1 {
    PilotScoringManifestV1::new(
        "fixture-scorer-v1",
        b"one iff the frozen prior constraint remains satisfied".as_slice(),
        b"one iff the task-specific deterministic command succeeds".as_slice(),
        b"unavailable on timeout, crash, or missing scorer observation".as_slice(),
        b"report per-condition counts and rates without a threshold".as_slice(),
    )
    .expect("valid scoring manifest")
}

fn runner() -> PilotRunnerManifestV1 {
    let identity = PilotRunnerIdentityV1::new(
        "0123456789abcdef0123456789abcdef01234567",
        "fixture-runner-v1",
        "fixture-environment-root-v1",
        "fixture-outcome-sink-v1",
    )
    .expect("valid runner identity");
    PilotRunnerManifestV1::new(
        identity,
        PilotCondition::all().to_vec(),
        vec![101],
        b"fresh isolated task-condition process".as_slice(),
        b"retain timeout, crash, output, and scorer bytes".as_slice(),
    )
    .expect("valid runner")
}

fn frozen(version: &str) -> FrozenSyntheticPilotV1 {
    let generation = generation(version);
    let (generation_log, tasks) = task_and_log(&generation);
    let corpus = PilotCorpusV1::new(tasks, &generation_log).expect("valid corpus");
    let conditions = condition_set(&corpus);
    FrozenSyntheticPilotV1::freeze(
        generation,
        generation_log,
        corpus,
        conditions,
        scoring(),
        runner(),
    )
    .expect("exactly joined frozen pilot")
}

fn completed_observations(pilot: &FrozenSyntheticPilotV1) -> Vec<PilotObservationV1> {
    let task_id = pilot.corpus().tasks()[0].id();
    PilotCondition::all()
        .iter()
        .map(|condition| {
            let followed = matches!(
                condition,
                PilotCondition::Focus
                    | PilotCondition::Correct
                    | PilotCondition::Wrong
                    | PilotCondition::Abstain
            );
            PilotObservationV1::new(
                task_id,
                *condition,
                101,
                PilotCellResultV1::Scored {
                    constraint_followed: followed,
                    task_completed: true,
                    exact_observation: format!("scored {}", condition.label())
                        .into_bytes()
                        .into_boxed_slice(),
                },
            )
            .expect("valid observation")
        })
        .collect()
}

#[test]
fn completed_receipt_is_descriptive_and_non_promotional() {
    let pilot = frozen("fixture-version-1");
    assert_eq!(
        pilot.generation().disclosure().immutable_version(),
        "fixture-version-1"
    );
    assert_eq!(
        pilot.generation().disclosure().maximum_cost(),
        "maximum 0 fixture currency"
    );
    assert_eq!(
        pilot.generation().disclosure().data_destination(),
        "local fixture process"
    );
    assert_eq!(
        pilot.generation().disclosure().retention_policy(),
        "no retention"
    );
    assert_eq!(
        pilot.generation().disclosure().privacy_implication(),
        "structural test bytes only"
    );
    assert_eq!(pilot.generation_log().attempts().len(), 2);
    assert_eq!(pilot.conditions().artifacts().len(), 7);
    let observations = completed_observations(&pilot);

    let receipt = SyntheticPilotReceiptV1::finalize(
        pilot,
        SyntheticPilotDisposition::Completed,
        observations,
        None::<Box<str>>,
    )
    .expect("completed receipt");

    assert_eq!(receipt.evidence_class(), "InternalSyntheticPilot");
    assert_eq!(receipt.promotion_status(), "NonPromotional");
    assert_eq!(receipt.disposition(), SyntheticPilotDisposition::Completed);
    assert_eq!(receipt.summaries().len(), 7);
    let focus = receipt
        .summaries()
        .iter()
        .find(|summary| summary.condition() == PilotCondition::Focus)
        .expect("focus summary");
    let prompt = receipt
        .summaries()
        .iter()
        .find(|summary| summary.condition() == PilotCondition::Prompt)
        .expect("prompt summary");
    assert_eq!(focus.constraint_following_rate(), Some(1.0));
    assert_eq!(prompt.constraint_following_rate(), Some(0.0));
}

#[test]
fn completed_receipt_requires_every_scored_cell() {
    let pilot = frozen("fixture-version-1");
    let task_id = pilot.corpus().tasks()[0].id();
    let observations = vec![
        PilotObservationV1::new(
            task_id,
            PilotCondition::Prompt,
            101,
            PilotCellResultV1::Scored {
                constraint_followed: false,
                task_completed: true,
                exact_observation: b"one cell".to_vec().into_boxed_slice(),
            },
        )
        .expect("valid observation"),
    ];

    let error = SyntheticPilotReceiptV1::finalize(
        pilot,
        SyntheticPilotDisposition::Completed,
        observations,
        None::<Box<str>>,
    )
    .expect_err("incomplete completion must reject");

    assert!(matches!(
        error,
        SyntheticPilotError::MissingObservation(_, _, _)
    ));
}

#[test]
fn invalid_receipt_retains_partial_observations_without_promotion() {
    let pilot = frozen("fixture-version-1");
    let task_id = pilot.corpus().tasks()[0].id();
    let observations = vec![
        PilotObservationV1::new(
            task_id,
            PilotCondition::Prompt,
            101,
            PilotCellResultV1::Unavailable {
                reason: "runner environment changed".into(),
                exact_output: b"captured runner bytes".to_vec().into_boxed_slice(),
            },
        )
        .expect("valid unavailable observation"),
    ];

    let receipt = SyntheticPilotReceiptV1::finalize(
        pilot,
        SyntheticPilotDisposition::Invalid,
        observations,
        Some("frozen runner identity mismatch"),
    )
    .expect("invalid receipt");

    assert_eq!(receipt.disposition(), SyntheticPilotDisposition::Invalid);
    assert_eq!(receipt.reason(), Some("frozen runner identity mismatch"));
    assert_eq!(receipt.promotion_status(), "NonPromotional");
}

#[test]
fn frozen_identity_changes_with_model_version() {
    assert_ne!(
        frozen("fixture-version-1").root(),
        frozen("fixture-version-2").root()
    );
}

#[test]
fn condition_set_rejects_a_missing_structural_variant() {
    let generation = generation("fixture-version-1");
    let (generation_log, tasks) = task_and_log(&generation);
    let corpus = PilotCorpusV1::new(tasks, &generation_log).expect("valid corpus");
    let task_id = corpus.tasks()[0].id();
    let artifacts = PilotCondition::all()
        .iter()
        .filter(|condition| **condition != PilotCondition::Placebo)
        .map(|condition| {
            let token_count = match condition {
                PilotCondition::Prompt | PilotCondition::Situation => None,
                _ => Some(16),
            };
            PilotConditionArtifactV1::new(
                task_id,
                *condition,
                condition.label().as_bytes(),
                token_count,
            )
            .expect("valid artifact")
        })
        .collect();

    assert_eq!(
        PilotConditionSetV1::new(artifacts, &corpus),
        Err(SyntheticPilotError::MissingConditionArtifact(
            task_id,
            PilotCondition::Placebo
        ))
    );
}

#[test]
fn freeze_rejects_cross_bound_corpus_and_conditions() {
    let generation_one = generation("fixture-version-1");
    let (log_one, tasks_one) = task_and_log(&generation_one);
    let corpus_one = PilotCorpusV1::new(tasks_one, &log_one).expect("first corpus");

    let generation_two = generation("fixture-version-2");
    let (log_two, tasks_two) = task_and_log(&generation_two);
    let corpus_two = PilotCorpusV1::new(tasks_two, &log_two).expect("second corpus");
    let conditions_two = condition_set(&corpus_two);

    assert_eq!(
        FrozenSyntheticPilotV1::freeze(
            generation_one,
            log_one,
            corpus_one,
            conditions_two,
            scoring(),
            runner(),
        ),
        Err(SyntheticPilotError::ConditionCorpusMismatch)
    );
}
