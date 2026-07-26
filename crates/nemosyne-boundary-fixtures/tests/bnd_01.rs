//! Integration evidence for the complete BND-01 fixture catalog.

use std::collections::BTreeMap;

use nemosyne_boundary_fixtures::{
    BoundaryFixtureCatalogError, BoundaryFixtureCatalogV1, BoundaryFixtureKind, BoundaryFixtureV1,
    BoundaryObservationV1, BoundaryViolation, FixtureObligation, bnd_01_fixture_catalog_v1,
};

#[test]
fn catalog_has_one_positive_and_counterexample_for_every_obligation() {
    let catalog = bnd_01_fixture_catalog_v1().expect("canonical catalog must be valid");
    assert_eq!(catalog.fixtures().len(), 34);

    let mut coverage = BTreeMap::new();
    for fixture in catalog.fixtures() {
        *coverage
            .entry((fixture.obligation(), fixture.kind()))
            .or_insert(0_usize) += 1;
    }

    for obligation in FixtureObligation::ALL {
        assert_eq!(
            coverage.get(&(obligation, BoundaryFixtureKind::Positive)),
            Some(&1)
        );
        assert_eq!(
            coverage.get(&(obligation, BoundaryFixtureKind::Counterexample)),
            Some(&1)
        );
    }
}

#[test]
fn catalog_contains_the_exact_declared_fixture_inventory() {
    let catalog = bnd_01_fixture_catalog_v1().expect("canonical catalog must be valid");
    let actual: Vec<_> = catalog
        .fixtures()
        .iter()
        .map(|fixture| fixture.id())
        .collect();
    assert_eq!(
        actual,
        vec![
            "f1-prompt-preserved",
            "f1-prompt-normalized",
            "f2-authorized-before-relevance",
            "f2-unauthorized-source-selected",
            "f3-single-pinned-snapshot",
            "f3-mixed-snapshot-revisions",
            "f4-read-only-compile-capabilities",
            "f4-semantic-write-capability",
            "f5-focus-authority-lowered",
            "f5-plan-authority-amplified",
            "f6-assertions-have-plan-support",
            "f6-unknown-assertion-support",
            "f7-complete-output-in-budget",
            "f7-truncated-over-budget-output",
            "f8-bounded-monotonic-activation",
            "f8-positive-evidence-lowers-activation",
            "f9-buffered-validated-result",
            "f9-error-exposes-prefix",
            "f10-one-owner-per-truth",
            "f10-plan-meaning-has-two-owners",
            "f11-authenticate-before-identity",
            "f11-identity-before-authenticity",
            "f12-shared-set-and-exact-slot",
            "f12-rebound-exact-slot",
            "f13-duplicate-does-not-amplify",
            "f13-duplicate-amplifies-support",
            "f14-complete-evidence-share-family",
            "f14-evidence-share-promoted-to-probability",
            "f15-alternatives-and-abstention-preserved",
            "f15-action-role-crosses-plan-boundary",
            "f16-offline-immutable-assessment",
            "f16-runtime-assessment-mutates-memory",
            "f17-renderer-only-lexicalizes-plan",
            "f17-renderer-adds-action-and-source-access",
        ]
    );
}

#[test]
fn every_fixture_executes_to_its_declared_outcome() {
    let catalog = bnd_01_fixture_catalog_v1().expect("canonical catalog must be valid");
    for fixture in catalog.fixtures() {
        match fixture.kind() {
            BoundaryFixtureKind::Positive => assert_eq!(fixture.evaluate(), Ok(())),
            BoundaryFixtureKind::Counterexample => {
                assert_eq!(fixture.evaluate().err(), fixture.expected_violation())
            }
        }
    }
}

#[test]
fn construction_is_canonical_under_input_permutation() {
    let canonical = bnd_01_fixture_catalog_v1().expect("canonical catalog must be valid");
    let mut reversed = canonical.fixtures().to_vec();
    reversed.reverse();
    let reconstructed =
        BoundaryFixtureCatalogV1::new(reversed).expect("permuted catalog must reconstruct");

    assert_eq!(canonical.content_id(), reconstructed.content_id());
    assert_eq!(canonical.fixtures(), reconstructed.fixtures());
}

#[test]
fn correcting_a_fixture_changes_the_catalog_identity() {
    let canonical = bnd_01_fixture_catalog_v1().expect("canonical catalog must be valid");
    let mut corrected = canonical.fixtures().to_vec();
    let fixture = corrected
        .iter_mut()
        .find(|fixture| fixture.id() == "f1-prompt-preserved")
        .expect("fixture exists");
    *fixture = BoundaryFixtureV1::new(
        "f1-prompt-preserved",
        FixtureObligation::F1,
        BoundaryFixtureKind::Positive,
        BoundaryObservationV1::Prompt {
            original: b"fix the parser".to_vec(),
            compiled: b"<attention>check exact framing</attention>\nfix the parser".to_vec(),
        },
        None,
    )
    .expect("replacement fixture is valid");
    let corrected =
        BoundaryFixtureCatalogV1::new(corrected).expect("corrected catalog remains complete");

    assert_ne!(canonical.content_id(), corrected.content_id());
}

#[test]
fn missing_and_duplicate_coverage_are_rejected() {
    let canonical = bnd_01_fixture_catalog_v1().expect("canonical catalog must be valid");
    let mut missing = canonical.fixtures().to_vec();
    missing.retain(|fixture| {
        !(fixture.obligation() == FixtureObligation::F17
            && fixture.kind() == BoundaryFixtureKind::Counterexample)
    });
    assert_eq!(
        BoundaryFixtureCatalogV1::new(missing),
        Err(BoundaryFixtureCatalogError::MissingCoverage {
            obligation: FixtureObligation::F17,
            kind: BoundaryFixtureKind::Counterexample,
        })
    );

    let mut duplicate = canonical.fixtures().to_vec();
    duplicate.push(
        BoundaryFixtureV1::new(
            "f17-second-counterexample",
            FixtureObligation::F17,
            BoundaryFixtureKind::Counterexample,
            BoundaryObservationV1::Renderer {
                claims: vec![],
                all_assertions_bound: false,
                exact_slots: vec![],
                capabilities: vec![],
            },
            Some(BoundaryViolation::RendererSemanticAmplification),
        )
        .expect("synthetic duplicate is structurally valid"),
    );
    assert_eq!(
        BoundaryFixtureCatalogV1::new(duplicate),
        Err(BoundaryFixtureCatalogError::DuplicateCoverage {
            obligation: FixtureObligation::F17,
            kind: BoundaryFixtureKind::Counterexample,
        })
    );
}

#[test]
fn wrong_observation_shape_is_rejected() {
    let fixture = BoundaryFixtureV1::new(
        "f2-wrong-observation-shape",
        FixtureObligation::F2,
        BoundaryFixtureKind::Counterexample,
        BoundaryObservationV1::Prompt {
            original: b"a".to_vec(),
            compiled: b"a".to_vec(),
        },
        Some(BoundaryViolation::ObservationKindMismatch),
    )
    .expect("fixture declaration is valid");

    assert_eq!(
        fixture.evaluate(),
        Err(BoundaryViolation::ObservationKindMismatch)
    );
}

#[test]
fn invalid_fixture_labels_are_rejected() {
    for id in ["", "-leading", "trailing-", "double--dash", "Uppercase"] {
        assert_eq!(
            BoundaryFixtureV1::new(
                id,
                FixtureObligation::F1,
                BoundaryFixtureKind::Positive,
                BoundaryObservationV1::Prompt {
                    original: b"a".to_vec(),
                    compiled: b"a".to_vec(),
                },
                None,
            ),
            Err(BoundaryFixtureCatalogError::InvalidFixtureId)
        );
    }
}
