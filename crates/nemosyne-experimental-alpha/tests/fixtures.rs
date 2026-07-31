//! Package-local positive, negative, boundary, and counterexample fixtures.

use nemosyne_experimental_alpha::{
    ApplicabilityControls, ApplicabilityOutcome, Authority, Candidate, CycleId, Error, FocusItem,
    FocusRole, Identifier, Priority, QueryScope, ReadinessOutcome, RecordState, Replacement,
    SupportHandle, Validity, evaluate, realize,
};

const POSITIVE_FIXTURE_ID: &str = "alpha-positive-001";
const NEGATIVE_FIXTURE_ID: &str = "alpha-negative-001";
const COUNTEREXAMPLE_FIXTURE_ID: &str = "alpha-counterexample-001";

fn identifier(value: &str) -> Identifier {
    Identifier::new(value).unwrap()
}

fn scope() -> QueryScope {
    QueryScope {
        subject: identifier("subject-a"),
        project: identifier("project-a"),
        cycle: Some(CycleId::new("cycle-7").unwrap()),
    }
}

fn candidate(id: &str, authority: u8, priority: u8, support: &str) -> Candidate {
    Candidate {
        id: identifier(id),
        controls: ApplicabilityControls {
            subject: identifier("subject-a"),
            project: identifier("project-a"),
            state: RecordState::Active,
            revision: 3,
            cycle: Some(CycleId::new("cycle-7").unwrap()),
            validity: Validity::Valid,
            replacement: Replacement::Current,
            authority: Authority::new(authority).unwrap(),
            priority: Priority::new(priority).unwrap(),
        },
        items: vec![FocusItem {
            role: FocusRole::Constraint,
            support: SupportHandle::new(support).unwrap(),
            qualifier: Some("current".into()),
        }],
    }
}

#[test]
fn positive_fixture_is_ready_and_byte_deterministic() {
    assert_eq!(POSITIVE_FIXTURE_ID, "alpha-positive-001");
    let high = candidate("record-high", 8, 2, "support-high");
    let low = candidate("record-low", 7, 15, "support-low");
    let forward =
        ReadinessOutcome::from(evaluate(&scope(), vec![low.clone(), high.clone()]).unwrap());
    let reverse = ReadinessOutcome::from(evaluate(&scope(), vec![high, low]).unwrap());
    assert!(matches!(forward, ReadinessOutcome::Ready(_)));
    assert_eq!(realize(&forward).unwrap(), realize(&reverse).unwrap());
}

#[test]
fn negative_controls_abstain_without_baseline_bytes() {
    assert_eq!(NEGATIVE_FIXTURE_ID, "alpha-negative-001");
    let mut cases = Vec::new();

    let mut mismatch = candidate("mismatch", 1, 1, "mismatch");
    mismatch.controls.project = identifier("another-project");
    cases.push(mismatch);

    let mut inactive = candidate("inactive", 1, 1, "inactive");
    inactive.controls.state = RecordState::Inactive;
    cases.push(inactive);

    let mut invalid = candidate("invalid", 1, 1, "invalid");
    invalid.controls.validity = Validity::Invalid;
    cases.push(invalid);

    let mut replaced = candidate("replaced", 1, 1, "replaced");
    replaced.controls.replacement = Replacement::ReplacedBy(identifier("successor"));
    let mut successor = candidate("successor", 1, 1, "successor");
    successor.controls.project = identifier("another-project");
    cases.push(replaced);

    for case in cases {
        let mut input = vec![case];
        if input[0].id == identifier("replaced") {
            input.push(successor.clone());
        }
        let outcome = evaluate(&scope(), input).unwrap();
        assert_eq!(outcome, ApplicabilityOutcome::Abstain);
        assert_eq!(
            realize(&ReadinessOutcome::from(outcome)),
            Err(Error::TerminalOutcome)
        );
    }
}

#[test]
fn counterexample_conflict_cannot_be_selected_away() {
    assert_eq!(COUNTEREXAMPLE_FIXTURE_ID, "alpha-counterexample-001");
    let left = candidate("left", 8, 2, "support-left");
    let right = candidate("right", 8, 2, "support-right");
    let outcome = evaluate(&scope(), vec![left, right]).unwrap();
    assert_eq!(outcome, ApplicabilityOutcome::Conflict);
    assert_eq!(
        realize(&ReadinessOutcome::from(outcome)),
        Err(Error::TerminalOutcome)
    );
}

#[test]
fn malformed_and_boundary_inputs_are_typed() {
    assert_eq!(Identifier::new(" \t"), Err(Error::EmptyIdentifier));
    assert_eq!(
        Identifier::new("x".repeat(65)),
        Err(Error::IdentifierTooLong)
    );
    assert_eq!(Authority::new(16), Err(Error::AuthorityOutOfRange));
    assert_eq!(Priority::new(16), Err(Error::PriorityOutOfRange));

    let mut zero_revision = candidate("zero", 1, 1, "support");
    zero_revision.controls.revision = 0;
    assert_eq!(
        evaluate(&scope(), vec![zero_revision]),
        Err(Error::RevisionZero)
    );

    let broken = {
        let mut value = candidate("broken", 1, 1, "support");
        value.controls.replacement = Replacement::ReplacedBy(identifier("absent"));
        value
    };
    assert_eq!(
        evaluate(&scope(), vec![broken]),
        Err(Error::BrokenReplacement)
    );
}

#[test]
fn lower_authority_and_replaced_records_cannot_override() {
    let high = candidate("high", 12, 0, "high");
    let mut replaced = candidate("replaced", 15, 15, "replaced");
    replaced.controls.replacement = Replacement::ReplacedBy(identifier("high"));
    let low = candidate("low", 11, 15, "low");
    let outcome = ReadinessOutcome::from(evaluate(&scope(), vec![low, replaced, high]).unwrap());
    let bytes = String::from_utf8(realize(&outcome).unwrap()).unwrap();
    assert!(bytes.contains("high"));
    assert!(!bytes.contains("low"));
    assert!(!bytes.contains("replaced"));
}

#[test]
fn priority_revision_cycle_and_item_order_are_deterministic() {
    let mut earlier = candidate("earlier", 8, 2, "earlier");
    earlier.controls.revision = 2;
    let mut lower_priority = candidate("lower-priority", 8, 1, "lower-priority");
    lower_priority.controls.revision = 99;
    let current = candidate("current", 8, 2, "current");
    let outcome =
        ReadinessOutcome::from(evaluate(&scope(), vec![earlier, lower_priority, current]).unwrap());
    let bytes = String::from_utf8(realize(&outcome).unwrap()).unwrap();
    assert!(bytes.contains("current"));

    let mut wrong_cycle = candidate("wrong-cycle", 15, 15, "wrong-cycle");
    wrong_cycle.controls.cycle = Some(CycleId::new("cycle-8").unwrap());
    let outcome = ReadinessOutcome::from(
        evaluate(&scope(), vec![wrong_cycle, candidate("ok", 1, 1, "ok")]).unwrap(),
    );
    assert!(
        String::from_utf8(realize(&outcome).unwrap())
            .unwrap()
            .contains("ok")
    );

    let mut left = candidate("left-order", 4, 4, "b");
    left.items.push(FocusItem {
        role: FocusRole::Goal,
        support: SupportHandle::new("a").unwrap(),
        qualifier: None,
    });
    let mut right = left.clone();
    right.id = identifier("right-order");
    right.items.reverse();
    let outcome = evaluate(&scope(), vec![left, right]).unwrap();
    assert!(matches!(outcome, ApplicabilityOutcome::Applicable(_)));
}

#[test]
fn duplicate_and_capacity_counterexamples_reject() {
    let duplicate = candidate("duplicate-id", 1, 1, "one");
    let mut other = candidate("duplicate-id", 1, 1, "two");
    other.controls.priority = Priority::new(2).unwrap();
    assert_eq!(
        evaluate(&scope(), vec![duplicate, other]),
        Err(Error::DuplicateCandidateId)
    );

    let mut duplicate_support = candidate("duplicate-support", 1, 1, "same");
    duplicate_support.items.push(FocusItem {
        role: FocusRole::Goal,
        support: SupportHandle::new("same").unwrap(),
        qualifier: None,
    });
    assert_eq!(
        evaluate(&scope(), vec![duplicate_support]),
        Err(Error::DuplicateSupport)
    );

    let mut empty = candidate("empty", 1, 1, "unused");
    empty.items.clear();
    assert_eq!(evaluate(&scope(), vec![empty]), Err(Error::EmptyFocus));

    let mut long_qualifier = candidate("long-qualifier", 1, 1, "support");
    long_qualifier.items[0].qualifier = Some("x".repeat(129));
    assert_eq!(
        evaluate(&scope(), vec![long_qualifier]),
        Err(Error::QualifierTooLong)
    );
}
