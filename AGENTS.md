# AGENTS.md

These instructions apply to the entire repository. Do not add `AGENTS.override.md` or nested `AGENTS.md` files unless a later decision record establishes their governance.

## Sources of truth

- Inspect the current checkout and the relevant documentation before editing.
- Treat `docs/specifications` as the current behavioral and mathematical contract.
- Treat `docs/decisions` as the historical record of significant design choices.
- Treat Rustdoc as the public API contract and tests as executable evidence.
- Keep `README.md` concise; detailed behavior and design rationale belong in the documentation system.
- Do not present issues, conversations, hypotheses, or proposed designs as accepted project truth.
- Resolve or report conflicts between code, tests, specifications, and decisions explicitly.

## Required workflow

1. Confirm the task scope and preserve unrelated changes.
2. Read the relevant specification, decision records, code, and tests.
3. Make the smallest coherent change that satisfies the task.
4. Update the required documentation in the same pull request.
5. Add or update tests for changed behavior and edge cases.
6. Run the repository checks before declaring the change complete.

Use focused `task/*` branches and pull requests into `main`. Do not push directly to `main` or bypass required checks.

## Documentation

- Every change to production Rust source under `crates/*/src` requires an updated specification.
- Add a decision record for a significant, long-lived design choice, including public contracts, algorithms, data formats, dependency boundaries, security properties, and compatibility policy.
- Start new records from the templates in `docs/specifications` and `docs/decisions`.
- Use statuses honestly. Do not mark a specification `Validated` without named implementation and evaluation evidence.
- Never rewrite or delete an accepted, rejected, or superseded decision. Supersede it with a new record.
- Keep normative claims precise, testable, and linked to verification where practical.
- Complete the pull request template's documentation impact and reason fields exactly as required.
- Do not weaken the documentation checker, its tests, or CI to make an unrelated change pass.
- Treat the root instructions, contributor guide, pull request template, all GitHub Actions workflows, documentation indexes and templates, and documentation-check scripts as documentation governance.
- Accompany every change to a documentation-governance file with a new accepted decision record.

## Engineering

- Do not treat architecture as settled unless a current specification defines it or an accepted decision selects it.
- Do not add a production dependency or expand repository scope without explicit justification.
- Document every public Rust item and keep unsafe Rust forbidden.
- Prefer deterministic behavior, explicit errors, and tests at observable boundaries.
- Do not commit secrets, generated build output, local editor state, or unrelated formatting changes.

## Verification

Run the structural documentation check and code checks before committing:

```text
./scripts/test-documentation-check.sh
./scripts/check-documentation.sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -F missing-docs -F unsafe-code
RUSTDOCFLAGS="-D warnings -F missing-docs -F unsafe-code" cargo doc --workspace --all-features --no-deps --locked
cargo test --workspace --all-features --locked
```

After committing, run the change-aware documentation check against the pull request body from a clean worktree:

```text
DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /absolute/path/outside/repository/pull-request-body.md
```

CI supplies the base, head, and pull request body directly. Before committing, also run `git diff --check`. Report any check that could not be run and why. Do not claim validation beyond the evidence produced.

## Code Review Rules

- Flag production behavior changes without a corresponding specification update.
- Flag significant design choices without a new decision record.
- Flag changes that rewrite historical decision records.
- Flag unsupported claims, hidden architecture assumptions, and validation claims without evidence.
- Flag changes that weaken CI or documentation governance without a focused rationale and regression tests.
