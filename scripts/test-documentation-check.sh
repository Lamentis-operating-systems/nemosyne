#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "$repository_root"

temporary_directory="$(mktemp -d)"
trap 'rm -r -- "$temporary_directory"' EXIT

valid_body="$temporary_directory/valid.md"
missing_reason_body="$temporary_directory/missing-reason.md"
mismatched_body="$temporary_directory/mismatched.md"
invalid_documentation="$temporary_directory/docs"
missing_documentation="$temporary_directory/missing-docs"
missing_agent_fixture="$temporary_directory/missing-agent-fixture"
invalid_agent_fixture="$temporary_directory/invalid-agent-fixture"
nested_agent_fixture="$temporary_directory/nested-agent-fixture"
override_agent_fixture="$temporary_directory/override-agent-fixture"
governance_fixture="$temporary_directory/governance-fixture"
approved_governance_fixture="$temporary_directory/approved-governance-fixture"

mkdir -p "$invalid_documentation/specifications" "$invalid_documentation/decisions"
mkdir -p "$missing_documentation/specifications" "$missing_documentation/decisions"

printf '# Documentation\n' > "$invalid_documentation/README.md"
printf '# Specifications\n' > "$invalid_documentation/specifications/README.md"
printf '# Template\n' > "$invalid_documentation/specifications/TEMPLATE.md"
printf '# Decisions\n' > "$invalid_documentation/decisions/README.md"
printf '# Template\n' > "$invalid_documentation/decisions/TEMPLATE.md"

create_fixture() {
  local destination="$1"

  mkdir -p "$destination/.github"
  cp -R "$repository_root/docs" "$destination/docs"
  cp -R "$repository_root/scripts" "$destination/scripts"
  cp "$repository_root/AGENTS.md" "$destination/AGENTS.md"
  cp "$repository_root/.github/PULL_REQUEST_TEMPLATE.md" "$destination/.github/PULL_REQUEST_TEMPLATE.md"

  find "$destination/docs/decisions" \
    -type f \
    -name '[0-9][0-9][0-9][0-9]-*.md' \
    -delete

  git -C "$destination" init -q
  git -C "$destination" config user.name 'Documentation Check'
  git -C "$destination" config user.email 'documentation-check@example.invalid'
}

write_decision() {
  local repository="$1"
  local identifier="$2"
  local slug="$3"
  local title="$4"
  local status="$5"
  local rationale="$6"
  local superseded_by="${7:-}"
  local file="$repository/docs/decisions/$identifier-$slug.md"

  {
    printf '# %s: %s\n\n' "$identifier" "$title"
    printf 'Status: %s\n' "$status"
    printf 'Date: 2026-07-21\n'
    if [[ -n "$superseded_by" ]]; then
      printf 'Superseded by: %s\n' "$superseded_by"
    fi
    printf '\n## Context\n\nOriginal context.\n'
    printf '\n## Decision\n\nOriginal decision.\n'
    printf '\n## Rationale\n\n%s\n' "$rationale"
    printf '\n## Alternatives\n\nOriginal alternative.\n'
    printf '\n## Consequences\n\nOriginal consequence.\n'
  } > "$file"
}

write_specification() {
  local repository="$1"
  local filename="$2"
  local title="$3"
  local status="$4"
  local superseded_by="${5:-}"
  local file="$repository/docs/specifications/$filename"

  {
    printf '# %s\n\n' "$title"
    printf 'Status: %s\n' "$status"
    if [[ -n "$superseded_by" ]]; then
      printf 'Superseded by: %s\n' "$superseded_by"
    fi
    printf '\n## Purpose\n\nPurpose.\n'
    printf '\n## Definitions\n\nDefinitions.\n'
    printf '\n## Preconditions\n\nPreconditions.\n'
    printf '\n## Invariants\n\nInvariants.\n'
    printf '\n## Edge cases\n\nEdge cases.\n'
    printf '\n## Verification\n\nVerification.\n'
    printf '\n## Open questions\n\nNone.\n'
    printf '\n## References\n\nReferences.\n'
  } > "$file"
}

write_body() {
  local destination="$1"
  local impact="$2"
  local reason="$3"

  printf '%s\n' \
    "Documentation impact: $impact" \
    "Documentation reason: $reason" \
    > "$destination"
}

expect_repository_check_failure() {
  local repository="$1"
  local message="$2"
  local expected_error="$3"
  local output
  shift 3

  if output="$(
    cd "$repository"
    ./scripts/check-documentation.sh "$@" 2>&1
  )"; then
    printf '%s\n' "$message" >&2
    exit 1
  elif [[ "$output" != *"$expected_error"* ]]; then
    printf '%s\n' "$message" >&2
    printf 'Expected error containing: %s\n' "$expected_error" >&2
    printf 'Actual output:\n%s\n' "$output" >&2
    exit 1
  fi
}

expect_local_check_failure() {
  local repository="$1"
  local base_ref="$2"
  local body="$3"
  local message="$4"
  local expected_error="$5"
  local output

  if output="$(
    cd "$repository"
    DOCUMENTATION_BASE_REF="$base_ref" ./scripts/check-documentation.sh "$body" 2>&1
  )"; then
    printf '%s\n' "$message" >&2
    exit 1
  elif [[ "$output" != *"$expected_error"* ]]; then
    printf '%s\n' "$message" >&2
    printf 'Expected error containing: %s\n' "$expected_error" >&2
    printf 'Actual output:\n%s\n' "$output" >&2
    exit 1
  fi
}

printf '%s\n' \
  'Documentation impact: none' \
  'Documentation reason: This test changes no specification or decision record.' \
  > "$valid_body"

printf '%s\n' \
  'Documentation impact: none' \
  'Documentation reason:' \
  > "$missing_reason_body"

printf '%s\n' \
  'Documentation impact: specification' \
  'Documentation reason: This declaration requires a changed specification file.' \
  > "$mismatched_body"

./scripts/check-documentation.sh HEAD HEAD "$valid_body" >/dev/null

if ./scripts/check-documentation.sh HEAD HEAD "$missing_reason_body" >/dev/null 2>&1; then
  printf '%s\n' 'Expected an empty documentation reason to fail.' >&2
  exit 1
fi

if ./scripts/check-documentation.sh HEAD HEAD "$mismatched_body" >/dev/null 2>&1; then
  printf '%s\n' 'Expected a mismatched documentation impact to fail.' >&2
  exit 1
fi

printf '%s\n' \
  '# Empty specification' \
  '' \
  'Status: Proposed' \
  '' \
  '## Purpose' \
  '## Definitions' \
  '## Preconditions' \
  '## Invariants' \
  '## Edge cases' \
  '## Verification' \
  '## Open questions' \
  '## References' \
  > "$invalid_documentation/specifications/empty-specification.md"

if DOCUMENTATION_ROOT="$invalid_documentation" ./scripts/check-documentation.sh >/dev/null 2>&1; then
  printf '%s\n' 'Expected empty specification sections to fail.' >&2
  exit 1
fi

if DOCUMENTATION_ROOT="$missing_documentation" ./scripts/check-documentation.sh >/dev/null 2>&1; then
  printf '%s\n' 'Expected a missing documentation structure to fail.' >&2
  exit 1
fi

create_fixture "$missing_agent_fixture"
rm "$missing_agent_fixture/AGENTS.md"
expect_repository_check_failure \
  "$missing_agent_fixture" \
  'Expected missing agent instructions to fail.' \
  'required file is missing: AGENTS.md'

create_fixture "$invalid_agent_fixture"
printf '# AGENTS.md\n' > "$invalid_agent_fixture/AGENTS.md"
expect_repository_check_failure \
  "$invalid_agent_fixture" \
  'Expected incomplete agent instructions to fail.' \
  "AGENTS.md must contain '## Sources of truth' exactly once"

create_fixture "$nested_agent_fixture"
mkdir -p "$nested_agent_fixture/crates/example"
printf '# Conflicting instructions\n' > "$nested_agent_fixture/crates/example/AGENTS.md"
git -C "$nested_agent_fixture" add .
expect_repository_check_failure \
  "$nested_agent_fixture" \
  'Expected nested agent instructions to fail.' \
  'additional agent instruction files are not allowed'

create_fixture "$override_agent_fixture"
printf '# Override instructions\n' > "$override_agent_fixture/AGENTS.override.md"
git -C "$override_agent_fixture" add .
expect_repository_check_failure \
  "$override_agent_fixture" \
  'Expected overriding agent instructions to fail.' \
  'additional agent instruction files are not allowed'

create_fixture "$governance_fixture"
git -C "$governance_fixture" add .
git -C "$governance_fixture" commit -qm baseline
governance_base="$(git -C "$governance_fixture" rev-parse HEAD)"

printf '\nAdditional unchecked rule.\n' >> "$governance_fixture/AGENTS.md"
git -C "$governance_fixture" add .
git -C "$governance_fixture" commit -qm governance-change
governance_head="$(git -C "$governance_fixture" rev-parse HEAD)"

printf '%s\n' \
  'Documentation impact: none' \
  'Documentation reason: This fixture changes governance without a decision record.' \
  > "$temporary_directory/governance-body.md"
expect_repository_check_failure \
  "$governance_fixture" \
  'Expected an undecided governance change to fail.' \
  'documentation governance changed without a new accepted decision record' \
  "$governance_base" \
  "$governance_head" \
  "$temporary_directory/governance-body.md"

create_fixture "$approved_governance_fixture"
git -C "$approved_governance_fixture" add .
git -C "$approved_governance_fixture" commit -qm baseline
approved_governance_base="$(git -C "$approved_governance_fixture" rev-parse HEAD)"

printf '\nReviewed governance rule.\n' >> "$approved_governance_fixture/AGENTS.md"
write_decision \
  "$approved_governance_fixture" \
  '0001' \
  'reviewed-governance-change' \
  'Review a governance change' \
  'Accepted' \
  'The governance change requires an explicit historical record.'
git -C "$approved_governance_fixture" add .
git -C "$approved_governance_fixture" commit -qm governance-change
approved_governance_head="$(git -C "$approved_governance_fixture" rev-parse HEAD)"

printf '%s\n' \
  'Documentation impact: decision' \
  'Documentation reason: This fixture records the governance change in a new decision.' \
  > "$temporary_directory/approved-governance-body.md"

(
  cd "$approved_governance_fixture"
  ./scripts/check-documentation.sh \
    "$approved_governance_base" \
    "$approved_governance_head" \
    "$temporary_directory/approved-governance-body.md"
) >/dev/null

source_fixture="$temporary_directory/source-fixture"
create_fixture "$source_fixture"
git -C "$source_fixture" add .
git -C "$source_fixture" commit -qm baseline
source_base="$(git -C "$source_fixture" rev-parse HEAD)"

mkdir -p "$source_fixture/crates/example/src"
printf '%s\n' 'pub fn changed_behavior() {}' > "$source_fixture/crates/example/src/lib.rs"
git -C "$source_fixture" add .
git -C "$source_fixture" commit -qm source-change
source_head="$(git -C "$source_fixture" rev-parse HEAD)"

printf '%s\n' \
  'Documentation impact: none' \
  'Documentation reason: This fixture claims that production behavior needs no specification.' \
  > "$temporary_directory/source-body.md"

if (
  cd "$source_fixture"
  ./scripts/check-documentation.sh \
    "$source_base" \
    "$source_head" \
    "$temporary_directory/source-body.md"
) >/dev/null 2>&1; then
  printf '%s\n' 'Expected an undocumented production source change to fail.' >&2
  exit 1
fi

rewrite_fixture="$temporary_directory/rewrite-fixture"
create_fixture "$rewrite_fixture"
write_decision "$rewrite_fixture" '0001' 'immutable-decision' 'Immutable decision' 'Accepted' 'Original rationale.'
git -C "$rewrite_fixture" add .
git -C "$rewrite_fixture" commit -qm baseline
rewrite_base="$(git -C "$rewrite_fixture" rev-parse HEAD)"

write_decision "$rewrite_fixture" '0001' 'immutable-decision' 'Immutable decision' 'Accepted' 'Rewritten rationale.'
git -C "$rewrite_fixture" add .
git -C "$rewrite_fixture" commit -qm rewrite
rewrite_head="$(git -C "$rewrite_fixture" rev-parse HEAD)"

printf '%s\n' \
  'Documentation impact: decision' \
  'Documentation reason: This fixture attempts to rewrite an accepted historical decision.' \
  > "$temporary_directory/rewrite-body.md"

if (
  cd "$rewrite_fixture"
  ./scripts/check-documentation.sh \
    "$rewrite_base" \
    "$rewrite_head" \
    "$temporary_directory/rewrite-body.md"
) >/dev/null 2>&1; then
  printf '%s\n' 'Expected an accepted decision rewrite to fail.' >&2
  exit 1
fi

supersede_fixture="$temporary_directory/supersede-fixture"
create_fixture "$supersede_fixture"
write_decision "$supersede_fixture" '0001' 'original-decision' 'Original decision' 'Accepted' 'Original rationale.'
git -C "$supersede_fixture" add .
git -C "$supersede_fixture" commit -qm baseline
supersede_base="$(git -C "$supersede_fixture" rev-parse HEAD)"

write_decision \
  "$supersede_fixture" \
  '0001' \
  'original-decision' \
  'Original decision' \
  'Superseded' \
  'Original rationale.' \
  '0002-replacement-decision.md'
write_decision "$supersede_fixture" '0002' 'replacement-decision' 'Replacement decision' 'Accepted' 'Replacement rationale.'
git -C "$supersede_fixture" add .
git -C "$supersede_fixture" commit -qm supersede
supersede_head="$(git -C "$supersede_fixture" rev-parse HEAD)"

printf '%s\n' \
  'Documentation impact: decision' \
  'Documentation reason: This fixture supersedes one accepted decision with another record.' \
  > "$temporary_directory/supersede-body.md"

(
  cd "$supersede_fixture"
  ./scripts/check-documentation.sh \
    "$supersede_base" \
    "$supersede_head" \
    "$temporary_directory/supersede-body.md"
) >/dev/null

metadata_title_decision_fixture="$temporary_directory/metadata-title-decision-fixture"
create_fixture "$metadata_title_decision_fixture"
write_decision \
  "$metadata_title_decision_fixture" \
  '0001' \
  'metadata-title' \
  'Preserve Status: Accepted and Date: 2026-07-21 text' \
  'Accepted' \
  'Original rationale.'
git -C "$metadata_title_decision_fixture" add .
git -C "$metadata_title_decision_fixture" commit -qm baseline
metadata_title_decision_base="$(git -C "$metadata_title_decision_fixture" rev-parse HEAD)"
write_decision \
  "$metadata_title_decision_fixture" \
  '0001' \
  'metadata-title' \
  'Preserve Status: Accepted and Date: 2026-07-21 text' \
  'Superseded' \
  'Original rationale.' \
  '0002-replacement-decision.md'
write_decision \
  "$metadata_title_decision_fixture" \
  '0002' \
  'replacement-decision' \
  'Replacement decision' \
  'Accepted' \
  'Replacement rationale.'
git -C "$metadata_title_decision_fixture" add .
git -C "$metadata_title_decision_fixture" commit -qm supersede
metadata_title_decision_head="$(git -C "$metadata_title_decision_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/metadata-title-decision-body.md" \
  'decision' \
  'This fixture preserves metadata-like text while superseding a decision.'
(
  cd "$metadata_title_decision_fixture"
  ./scripts/check-documentation.sh \
    "$metadata_title_decision_base" \
    "$metadata_title_decision_head" \
    "$temporary_directory/metadata-title-decision-body.md"
) >/dev/null

nested_specification_fixture="$temporary_directory/nested-specification-fixture"
create_fixture "$nested_specification_fixture"
mkdir -p "$nested_specification_fixture/docs/specifications/nested"
printf '# Nested metadata\n' > "$nested_specification_fixture/docs/specifications/nested/README.md"
expect_repository_check_failure \
  "$nested_specification_fixture" \
  'Expected nested specification documentation to fail.' \
  'must be stored directly in docs/specifications'

nested_decision_layout_fixture="$temporary_directory/nested-decision-layout-fixture"
create_fixture "$nested_decision_layout_fixture"
write_decision "$nested_decision_layout_fixture" '0001' 'nested-decision' 'Nested decision' 'Accepted' 'Nested rationale.'
mkdir -p "$nested_decision_layout_fixture/docs/decisions/nested"
mv \
  "$nested_decision_layout_fixture/docs/decisions/0001-nested-decision.md" \
  "$nested_decision_layout_fixture/docs/decisions/nested/0001-nested-decision.md"
expect_repository_check_failure \
  "$nested_decision_layout_fixture" \
  'Expected a nested decision record to fail.' \
  'must be stored directly in docs/decisions'

symlink_directory_fixture="$temporary_directory/symlink-directory-fixture"
create_fixture "$symlink_directory_fixture"
mv \
  "$symlink_directory_fixture/docs/specifications" \
  "$symlink_directory_fixture/docs/specifications-real"
ln -s specifications-real "$symlink_directory_fixture/docs/specifications"
expect_repository_check_failure \
  "$symlink_directory_fixture" \
  'Expected a symbolic-link documentation directory to fail.' \
  'required directory must be a regular directory, not a symbolic link'

symlink_specification_fixture="$temporary_directory/symlink-specification-fixture"
create_fixture "$symlink_specification_fixture"
git -C "$symlink_specification_fixture" add .
git -C "$symlink_specification_fixture" commit -qm baseline
symlink_specification_base="$(git -C "$symlink_specification_fixture" rev-parse HEAD)"
mkdir -p "$symlink_specification_fixture/crates/example/src"
printf '%s\n' 'pub fn undocumented_behavior() {}' > "$symlink_specification_fixture/crates/example/src/lib.rs"
ln -s ../README.md "$symlink_specification_fixture/docs/specifications/fake-contract.md"
git -C "$symlink_specification_fixture" add .
git -C "$symlink_specification_fixture" commit -qm symlink-specification
symlink_specification_head="$(git -C "$symlink_specification_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/symlink-specification-body.md" \
  'specification' \
  'This fixture attempts to satisfy documentation with a symbolic link.'
expect_repository_check_failure \
  "$symlink_specification_fixture" \
  'Expected a symbolic-link specification to fail.' \
  'must be a regular file, not a symbolic link' \
  "$symlink_specification_base" \
  "$symlink_specification_head" \
  "$temporary_directory/symlink-specification-body.md"

symlink_decision_fixture="$temporary_directory/symlink-decision-fixture"
create_fixture "$symlink_decision_fixture"
git -C "$symlink_decision_fixture" add .
git -C "$symlink_decision_fixture" commit -qm baseline
symlink_decision_base="$(git -C "$symlink_decision_fixture" rev-parse HEAD)"
printf '\nGovernance change hidden behind a symbolic link.\n' >> "$symlink_decision_fixture/AGENTS.md"
ln -s 'Status: Accepted' "$symlink_decision_fixture/docs/decisions/0001-fake-decision.md"
git -C "$symlink_decision_fixture" add .
git -C "$symlink_decision_fixture" commit -qm symlink-decision
symlink_decision_head="$(git -C "$symlink_decision_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/symlink-decision-body.md" \
  'decision' \
  'This fixture attempts to authorize governance with a symbolic link.'
expect_repository_check_failure \
  "$symlink_decision_fixture" \
  'Expected a symbolic-link decision to fail.' \
  'must be a regular file, not a symbolic link' \
  "$symlink_decision_base" \
  "$symlink_decision_head" \
  "$temporary_directory/symlink-decision-body.md"

source_rename_fixture="$temporary_directory/source-rename-fixture"
create_fixture "$source_rename_fixture"
mkdir -p "$source_rename_fixture/crates/example/src"
printf '%s\n' 'pub fn original_behavior() {}' > "$source_rename_fixture/crates/example/src/lib.rs"
git -C "$source_rename_fixture" add .
git -C "$source_rename_fixture" commit -qm baseline
source_rename_base="$(git -C "$source_rename_fixture" rev-parse HEAD)"
git -C "$source_rename_fixture" mv crates/example/src/lib.rs crates/example/src/lib.txt
git -C "$source_rename_fixture" commit -qm rename-source
source_rename_head="$(git -C "$source_rename_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/source-rename-body.md" \
  'none' \
  'This fixture attempts to hide a production source change through a rename.'
expect_repository_check_failure \
  "$source_rename_fixture" \
  'Expected a renamed production source file to require a specification.' \
  'production Rust source changed without an updated specification' \
  "$source_rename_base" \
  "$source_rename_head" \
  "$temporary_directory/source-rename-body.md"

documented_source_rename_fixture="$temporary_directory/documented-source-rename-fixture"
create_fixture "$documented_source_rename_fixture"
mkdir -p "$documented_source_rename_fixture/crates/example/src"
printf '%s\n' 'pub fn original_behavior() {}' > "$documented_source_rename_fixture/crates/example/src/lib.rs"
git -C "$documented_source_rename_fixture" add .
git -C "$documented_source_rename_fixture" commit -qm baseline
documented_source_rename_base="$(git -C "$documented_source_rename_fixture" rev-parse HEAD)"
git -C "$documented_source_rename_fixture" mv crates/example/src/lib.rs crates/example/src/lib.txt
write_specification \
  "$documented_source_rename_fixture" \
  'source-rename.md' \
  'Source rename' \
  'Experimental'
git -C "$documented_source_rename_fixture" add .
git -C "$documented_source_rename_fixture" commit -qm documented-source-rename
documented_source_rename_head="$(git -C "$documented_source_rename_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/documented-source-rename-body.md" \
  'specification' \
  'This fixture documents the production source behavior affected by a rename.'
(
  cd "$documented_source_rename_fixture"
  ./scripts/check-documentation.sh \
    "$documented_source_rename_base" \
    "$documented_source_rename_head" \
    "$temporary_directory/documented-source-rename-body.md"
) >/dev/null

governance_rename_fixture="$temporary_directory/governance-rename-fixture"
create_fixture "$governance_rename_fixture"
mkdir -p "$governance_rename_fixture/.github/workflows"
printf '%s\n' 'name: CI' > "$governance_rename_fixture/.github/workflows/ci.yml"
git -C "$governance_rename_fixture" add .
git -C "$governance_rename_fixture" commit -qm baseline
governance_rename_base="$(git -C "$governance_rename_fixture" rev-parse HEAD)"
git -C "$governance_rename_fixture" mv .github/workflows/ci.yml .github/workflows/build.yml
git -C "$governance_rename_fixture" commit -qm rename-governance
governance_rename_head="$(git -C "$governance_rename_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/governance-rename-body.md" \
  'none' \
  'This fixture attempts to hide a governance change through a workflow rename.'
expect_repository_check_failure \
  "$governance_rename_fixture" \
  'Expected a renamed governance file to require an accepted decision.' \
  'documentation governance changed without a new accepted decision record' \
  "$governance_rename_base" \
  "$governance_rename_head" \
  "$temporary_directory/governance-rename-body.md"

documented_governance_rename_fixture="$temporary_directory/documented-governance-rename-fixture"
create_fixture "$documented_governance_rename_fixture"
mkdir -p "$documented_governance_rename_fixture/.github/workflows"
printf '%s\n' 'name: CI' > "$documented_governance_rename_fixture/.github/workflows/ci.yml"
git -C "$documented_governance_rename_fixture" add .
git -C "$documented_governance_rename_fixture" commit -qm baseline
documented_governance_rename_base="$(git -C "$documented_governance_rename_fixture" rev-parse HEAD)"
git -C "$documented_governance_rename_fixture" mv .github/workflows/ci.yml .github/workflows/build.yml
write_decision \
  "$documented_governance_rename_fixture" \
  '0001' \
  'rename-ci-workflow' \
  'Rename the CI workflow' \
  'Accepted' \
  'The workflow rename is governed by an accepted decision.'
git -C "$documented_governance_rename_fixture" add .
git -C "$documented_governance_rename_fixture" commit -qm documented-governance-rename
documented_governance_rename_head="$(git -C "$documented_governance_rename_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/documented-governance-rename-body.md" \
  'decision' \
  'This fixture records the workflow rename in an accepted decision.'
(
  cd "$documented_governance_rename_fixture"
  ./scripts/check-documentation.sh \
    "$documented_governance_rename_base" \
    "$documented_governance_rename_head" \
    "$temporary_directory/documented-governance-rename-body.md"
) >/dev/null

printf '\non: push\n' >> "$documented_governance_rename_fixture/.github/workflows/build.yml"
git -C "$documented_governance_rename_fixture" add .
git -C "$documented_governance_rename_fixture" commit -qm modify-renamed-governance
modified_renamed_governance_head="$(git -C "$documented_governance_rename_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/modified-renamed-governance-body.md" \
  'none' \
  'This fixture changes the renamed workflow without recording a decision.'
expect_repository_check_failure \
  "$documented_governance_rename_fixture" \
  'Expected the renamed workflow to remain governed.' \
  'documentation governance changed without a new accepted decision record' \
  "$documented_governance_rename_head" \
  "$modified_renamed_governance_head" \
  "$temporary_directory/modified-renamed-governance-body.md"

moved_decision_fixture="$temporary_directory/moved-decision-fixture"
create_fixture "$moved_decision_fixture"
write_decision \
  "$moved_decision_fixture" \
  '0001' \
  'moved-governance-decision' \
  'Move a governance decision' \
  'Accepted' \
  'The moved record authorizes the governance change.'
mv \
  "$moved_decision_fixture/docs/decisions/0001-moved-governance-decision.md" \
  "$moved_decision_fixture/moved-governance-decision.md"
git -C "$moved_decision_fixture" add .
git -C "$moved_decision_fixture" commit -qm baseline
moved_decision_base="$(git -C "$moved_decision_fixture" rev-parse HEAD)"
printf '\nGovernance change backed by a moved decision.\n' >> "$moved_decision_fixture/AGENTS.md"
git -C "$moved_decision_fixture" mv \
  moved-governance-decision.md \
  docs/decisions/0001-moved-governance-decision.md
git -C "$moved_decision_fixture" add .
git -C "$moved_decision_fixture" commit -qm move-governance-decision
moved_decision_head="$(git -C "$moved_decision_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/moved-decision-body.md" \
  'decision' \
  'This fixture records the governance change in a moved accepted decision.'
(
  cd "$moved_decision_fixture"
  ./scripts/check-documentation.sh \
    "$moved_decision_base" \
    "$moved_decision_head" \
    "$temporary_directory/moved-decision-body.md"
) >/dev/null

self_supersede_fixture="$temporary_directory/self-supersede-fixture"
create_fixture "$self_supersede_fixture"
write_decision "$self_supersede_fixture" '0001' 'self-reference' 'Self reference' 'Accepted' 'Original rationale.'
git -C "$self_supersede_fixture" add .
git -C "$self_supersede_fixture" commit -qm baseline
self_supersede_base="$(git -C "$self_supersede_fixture" rev-parse HEAD)"
write_decision \
  "$self_supersede_fixture" \
  '0001' \
  'self-reference' \
  'Self reference' \
  'Superseded' \
  'Original rationale.' \
  '0001-self-reference.md'
git -C "$self_supersede_fixture" add .
git -C "$self_supersede_fixture" commit -qm self-supersede
self_supersede_head="$(git -C "$self_supersede_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/self-supersede-body.md" \
  'decision' \
  'This fixture attempts to supersede a decision with the same record.'
expect_repository_check_failure \
  "$self_supersede_fixture" \
  'Expected decision self-supersession to fail.' \
  'cannot supersede itself' \
  "$self_supersede_base" \
  "$self_supersede_head" \
  "$temporary_directory/self-supersede-body.md"

rejected_replacement_fixture="$temporary_directory/rejected-replacement-fixture"
create_fixture "$rejected_replacement_fixture"
write_decision "$rejected_replacement_fixture" '0001' 'original-decision' 'Original decision' 'Accepted' 'Original rationale.'
git -C "$rejected_replacement_fixture" add .
git -C "$rejected_replacement_fixture" commit -qm baseline
rejected_replacement_base="$(git -C "$rejected_replacement_fixture" rev-parse HEAD)"
write_decision \
  "$rejected_replacement_fixture" \
  '0001' \
  'original-decision' \
  'Original decision' \
  'Superseded' \
  'Original rationale.' \
  '0002-rejected-replacement.md'
write_decision \
  "$rejected_replacement_fixture" \
  '0002' \
  'rejected-replacement' \
  'Rejected replacement' \
  'Rejected' \
  'Rejected rationale.'
git -C "$rejected_replacement_fixture" add .
git -C "$rejected_replacement_fixture" commit -qm rejected-replacement
rejected_replacement_head="$(git -C "$rejected_replacement_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/rejected-replacement-body.md" \
  'decision' \
  'This fixture attempts to replace an accepted decision with a rejected record.'
expect_repository_check_failure \
  "$rejected_replacement_fixture" \
  'Expected a rejected decision replacement to fail.' \
  'replacement must be accepted or superseded' \
  "$rejected_replacement_base" \
  "$rejected_replacement_head" \
  "$temporary_directory/rejected-replacement-body.md"

existing_replacement_fixture="$temporary_directory/existing-replacement-fixture"
create_fixture "$existing_replacement_fixture"
write_decision "$existing_replacement_fixture" '0001' 'original-decision' 'Original decision' 'Accepted' 'Original rationale.'
write_decision "$existing_replacement_fixture" '0002' 'existing-decision' 'Existing decision' 'Accepted' 'Existing rationale.'
git -C "$existing_replacement_fixture" add .
git -C "$existing_replacement_fixture" commit -qm baseline
existing_replacement_base="$(git -C "$existing_replacement_fixture" rev-parse HEAD)"
write_decision \
  "$existing_replacement_fixture" \
  '0001' \
  'original-decision' \
  'Original decision' \
  'Superseded' \
  'Original rationale.' \
  '0002-existing-decision.md'
git -C "$existing_replacement_fixture" add .
git -C "$existing_replacement_fixture" commit -qm existing-replacement
existing_replacement_head="$(git -C "$existing_replacement_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/existing-replacement-body.md" \
  'decision' \
  'This fixture attempts to reuse an existing decision as a new replacement.'
expect_repository_check_failure \
  "$existing_replacement_fixture" \
  'Expected a pre-existing decision replacement to fail.' \
  'replacement must be added with the superseding change' \
  "$existing_replacement_base" \
  "$existing_replacement_head" \
  "$temporary_directory/existing-replacement-body.md"

lower_identifier_replacement_fixture="$temporary_directory/lower-identifier-replacement-fixture"
create_fixture "$lower_identifier_replacement_fixture"
write_decision "$lower_identifier_replacement_fixture" '0002' 'original-decision' 'Original decision' 'Accepted' 'Original rationale.'
git -C "$lower_identifier_replacement_fixture" add .
git -C "$lower_identifier_replacement_fixture" commit -qm baseline
lower_identifier_replacement_base="$(git -C "$lower_identifier_replacement_fixture" rev-parse HEAD)"
write_decision \
  "$lower_identifier_replacement_fixture" \
  '0002' \
  'original-decision' \
  'Original decision' \
  'Superseded' \
  'Original rationale.' \
  '0001-lower-replacement.md'
write_decision \
  "$lower_identifier_replacement_fixture" \
  '0001' \
  'lower-replacement' \
  'Lower replacement' \
  'Accepted' \
  'Replacement rationale.'
git -C "$lower_identifier_replacement_fixture" add .
git -C "$lower_identifier_replacement_fixture" commit -qm lower-identifier-replacement
lower_identifier_replacement_head="$(git -C "$lower_identifier_replacement_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/lower-identifier-replacement-body.md" \
  'decision' \
  'This fixture attempts to replace a decision with a lower identifier.'
expect_repository_check_failure \
  "$lower_identifier_replacement_fixture" \
  'Expected a lower decision replacement identifier to fail.' \
  'replacement must use a later decision identifier' \
  "$lower_identifier_replacement_base" \
  "$lower_identifier_replacement_head" \
  "$temporary_directory/lower-identifier-replacement-body.md"

initially_superseded_decision_fixture="$temporary_directory/initially-superseded-decision-fixture"
create_fixture "$initially_superseded_decision_fixture"
git -C "$initially_superseded_decision_fixture" add .
git -C "$initially_superseded_decision_fixture" commit -qm baseline
initially_superseded_decision_base="$(git -C "$initially_superseded_decision_fixture" rev-parse HEAD)"
write_decision \
  "$initially_superseded_decision_fixture" \
  '0001' \
  'fabricated-history' \
  'Fabricated history' \
  'Superseded' \
  'Fabricated rationale.' \
  '0002-current-decision.md'
write_decision \
  "$initially_superseded_decision_fixture" \
  '0002' \
  'current-decision' \
  'Current decision' \
  'Accepted' \
  'Current rationale.'
git -C "$initially_superseded_decision_fixture" add .
git -C "$initially_superseded_decision_fixture" commit -qm fabricated-history
initially_superseded_decision_head="$(git -C "$initially_superseded_decision_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/initially-superseded-decision-body.md" \
  'decision' \
  'This fixture attempts to add a decision in an already superseded state.'
expect_repository_check_failure \
  "$initially_superseded_decision_fixture" \
  'Expected a newly added superseded decision to fail.' \
  'cannot be added in superseded state' \
  "$initially_superseded_decision_base" \
  "$initially_superseded_decision_head" \
  "$temporary_directory/initially-superseded-decision-body.md"

metadata_rewrite_fixture="$temporary_directory/metadata-rewrite-fixture"
create_fixture "$metadata_rewrite_fixture"
write_decision \
  "$metadata_rewrite_fixture" \
  '0001' \
  'metadata-rewrite' \
  'Metadata rewrite' \
  'Accepted' \
  'Status: Original historical text.'
git -C "$metadata_rewrite_fixture" add .
git -C "$metadata_rewrite_fixture" commit -qm baseline
metadata_rewrite_base="$(git -C "$metadata_rewrite_fixture" rev-parse HEAD)"
write_decision \
  "$metadata_rewrite_fixture" \
  '0001' \
  'metadata-rewrite' \
  'Metadata rewrite' \
  'Superseded' \
  'Status: Rewritten historical text.' \
  '0002-replacement-decision.md'
write_decision "$metadata_rewrite_fixture" '0002' 'replacement-decision' 'Replacement decision' 'Accepted' 'Replacement rationale.'
git -C "$metadata_rewrite_fixture" add .
git -C "$metadata_rewrite_fixture" commit -qm metadata-rewrite
metadata_rewrite_head="$(git -C "$metadata_rewrite_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/metadata-rewrite-body.md" \
  'decision' \
  'This fixture hides a historical rewrite behind a metadata-looking prefix.'
expect_repository_check_failure \
  "$metadata_rewrite_fixture" \
  'Expected metadata-looking historical content changes to fail.' \
  'may only transition from Accepted to Superseded with a replacement reference' \
  "$metadata_rewrite_base" \
  "$metadata_rewrite_head" \
  "$temporary_directory/metadata-rewrite-body.md"

decision_trailing_lines_fixture="$temporary_directory/decision-trailing-lines-fixture"
create_fixture "$decision_trailing_lines_fixture"
write_decision "$decision_trailing_lines_fixture" '0001' 'original-decision' 'Original decision' 'Accepted' 'Original rationale.'
git -C "$decision_trailing_lines_fixture" add .
git -C "$decision_trailing_lines_fixture" commit -qm baseline
decision_trailing_lines_base="$(git -C "$decision_trailing_lines_fixture" rev-parse HEAD)"
write_decision \
  "$decision_trailing_lines_fixture" \
  '0001' \
  'original-decision' \
  'Original decision' \
  'Superseded' \
  'Original rationale.' \
  '0002-replacement-decision.md'
printf '\n\n' >> "$decision_trailing_lines_fixture/docs/decisions/0001-original-decision.md"
write_decision \
  "$decision_trailing_lines_fixture" \
  '0002' \
  'replacement-decision' \
  'Replacement decision' \
  'Accepted' \
  'Replacement rationale.'
git -C "$decision_trailing_lines_fixture" add .
git -C "$decision_trailing_lines_fixture" commit -qm add-trailing-lines
decision_trailing_lines_head="$(git -C "$decision_trailing_lines_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/decision-trailing-lines-body.md" \
  'decision' \
  'This fixture appends blank lines while superseding a decision.'
expect_repository_check_failure \
  "$decision_trailing_lines_fixture" \
  'Expected trailing decision lines during supersession to fail.' \
  'may only transition from Accepted to Superseded with a replacement reference' \
  "$decision_trailing_lines_base" \
  "$decision_trailing_lines_head" \
  "$temporary_directory/decision-trailing-lines-body.md"

orphaned_specification_fixture="$temporary_directory/orphaned-specification-fixture"
create_fixture "$orphaned_specification_fixture"
write_specification "$orphaned_specification_fixture" 'original-contract.md' 'Original contract' 'Validated'
git -C "$orphaned_specification_fixture" add .
git -C "$orphaned_specification_fixture" commit -qm baseline
orphaned_specification_base="$(git -C "$orphaned_specification_fixture" rev-parse HEAD)"
write_specification "$orphaned_specification_fixture" 'original-contract.md' 'Original contract' 'Superseded'
git -C "$orphaned_specification_fixture" add .
git -C "$orphaned_specification_fixture" commit -qm orphan-specification
orphaned_specification_head="$(git -C "$orphaned_specification_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/orphaned-specification-body.md" \
  'specification' \
  'This fixture supersedes a specification without naming its replacement.'
expect_repository_check_failure \
  "$orphaned_specification_fixture" \
  'Expected an orphaned superseded specification to fail.' \
  'must identify exactly one replacement when superseded' \
  "$orphaned_specification_base" \
  "$orphaned_specification_head" \
  "$temporary_directory/orphaned-specification-body.md"

hidden_superseded_specification_fixture="$temporary_directory/hidden-superseded-specification-fixture"
create_fixture "$hidden_superseded_specification_fixture"
git -C "$hidden_superseded_specification_fixture" add .
git -C "$hidden_superseded_specification_fixture" commit -qm baseline
hidden_superseded_specification_base="$(git -C "$hidden_superseded_specification_fixture" rev-parse HEAD)"
write_specification \
  "$hidden_superseded_specification_fixture" \
  'hidden-superseded-contract.md' \
  'Hidden superseded contract' \
  'Superseded'
hidden_superseded_path="$hidden_superseded_specification_fixture/docs/specifications/hidden-superseded-contract.md"
awk 'NR == 1 { print; print ""; print "Status: Bogus"; next } { print }' \
  "$hidden_superseded_path" > "$hidden_superseded_path.tmp"
mv "$hidden_superseded_path.tmp" "$hidden_superseded_path"
git -C "$hidden_superseded_specification_fixture" add .
git -C "$hidden_superseded_specification_fixture" commit -qm hidden-superseded-status
hidden_superseded_specification_head="$(git -C "$hidden_superseded_specification_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/hidden-superseded-specification-body.md" \
  'specification' \
  'This fixture attempts to hide a superseded status before validation.'
expect_repository_check_failure \
  "$hidden_superseded_specification_fixture" \
  'Expected hidden superseded specification metadata to fail.' \
  'has an invalid or missing status' \
  "$hidden_superseded_specification_base" \
  "$hidden_superseded_specification_head" \
  "$temporary_directory/hidden-superseded-specification-body.md"

misplaced_specification_fixture="$temporary_directory/misplaced-specification-fixture"
create_fixture "$misplaced_specification_fixture"
git -C "$misplaced_specification_fixture" add .
git -C "$misplaced_specification_fixture" commit -qm baseline
misplaced_specification_base="$(git -C "$misplaced_specification_fixture" rev-parse HEAD)"
misplaced_specification="$misplaced_specification_fixture/docs/specifications/misplaced-contract.md"
{
  printf '# Misplaced contract\n'
  printf '\n## Purpose\n\nStatus: Validated\nPurpose.\n'
  printf '\n## Definitions\n\nDefinitions.\n'
  printf '\n## Preconditions\n\nPreconditions.\n'
  printf '\n## Invariants\n\nInvariants.\n'
  printf '\n## Edge cases\n\nEdge cases.\n'
  printf '\n## Verification\n\nVerification.\n'
  printf '\n## Open questions\n\nNone.\n'
  printf '\n## References\n\nReferences.\n'
} > "$misplaced_specification"
git -C "$misplaced_specification_fixture" add .
git -C "$misplaced_specification_fixture" commit -qm misplaced-specification
misplaced_specification_head="$(git -C "$misplaced_specification_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/misplaced-specification-body.md" \
  'specification' \
  'This fixture attempts to add a specification with misplaced metadata.'
expect_repository_check_failure \
  "$misplaced_specification_fixture" \
  'Expected misplaced specification metadata to fail.' \
  'has an invalid or missing status' \
  "$misplaced_specification_base" \
  "$misplaced_specification_head" \
  "$temporary_directory/misplaced-specification-body.md"

existing_specification_replacement_fixture="$temporary_directory/existing-specification-replacement-fixture"
create_fixture "$existing_specification_replacement_fixture"
write_specification "$existing_specification_replacement_fixture" 'original-contract.md' 'Original contract' 'Validated'
write_specification "$existing_specification_replacement_fixture" 'existing-contract.md' 'Existing contract' 'Experimental'
git -C "$existing_specification_replacement_fixture" add .
git -C "$existing_specification_replacement_fixture" commit -qm baseline
existing_specification_replacement_base="$(git -C "$existing_specification_replacement_fixture" rev-parse HEAD)"
write_specification \
  "$existing_specification_replacement_fixture" \
  'original-contract.md' \
  'Original contract' \
  'Superseded' \
  'existing-contract.md'
git -C "$existing_specification_replacement_fixture" add .
git -C "$existing_specification_replacement_fixture" commit -qm existing-specification-replacement
existing_specification_replacement_head="$(git -C "$existing_specification_replacement_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/existing-specification-replacement-body.md" \
  'specification' \
  'This fixture attempts to reuse an existing specification as a new replacement.'
expect_repository_check_failure \
  "$existing_specification_replacement_fixture" \
  'Expected a pre-existing specification replacement to fail.' \
  'replacement must be added with the superseding change' \
  "$existing_specification_replacement_base" \
  "$existing_specification_replacement_head" \
  "$temporary_directory/existing-specification-replacement-body.md"

rewritten_specification_fixture="$temporary_directory/rewritten-specification-fixture"
create_fixture "$rewritten_specification_fixture"
write_specification "$rewritten_specification_fixture" 'original-contract.md' 'Original contract' 'Validated'
git -C "$rewritten_specification_fixture" add .
git -C "$rewritten_specification_fixture" commit -qm baseline
rewritten_specification_base="$(git -C "$rewritten_specification_fixture" rev-parse HEAD)"
write_specification \
  "$rewritten_specification_fixture" \
  'original-contract.md' \
  'Original contract' \
  'Superseded' \
  'replacement-contract.md'
printf '\nRewritten historical content.\n' >> "$rewritten_specification_fixture/docs/specifications/original-contract.md"
write_specification "$rewritten_specification_fixture" 'replacement-contract.md' 'Replacement contract' 'Experimental'
git -C "$rewritten_specification_fixture" add .
git -C "$rewritten_specification_fixture" commit -qm rewrite-and-supersede
rewritten_specification_head="$(git -C "$rewritten_specification_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/rewritten-specification-body.md" \
  'specification' \
  'This fixture attempts to rewrite a specification while superseding it.'
expect_repository_check_failure \
  "$rewritten_specification_fixture" \
  'Expected a specification rewrite during supersession to fail.' \
  'may only transition to Superseded with a replacement reference' \
  "$rewritten_specification_base" \
  "$rewritten_specification_head" \
  "$temporary_directory/rewritten-specification-body.md"

specification_trailing_lines_fixture="$temporary_directory/specification-trailing-lines-fixture"
create_fixture "$specification_trailing_lines_fixture"
write_specification "$specification_trailing_lines_fixture" 'original-contract.md' 'Original contract' 'Validated'
git -C "$specification_trailing_lines_fixture" add .
git -C "$specification_trailing_lines_fixture" commit -qm baseline
specification_trailing_lines_base="$(git -C "$specification_trailing_lines_fixture" rev-parse HEAD)"
write_specification \
  "$specification_trailing_lines_fixture" \
  'original-contract.md' \
  'Original contract' \
  'Superseded' \
  'replacement-contract.md'
printf '\n\n' >> "$specification_trailing_lines_fixture/docs/specifications/original-contract.md"
write_specification \
  "$specification_trailing_lines_fixture" \
  'replacement-contract.md' \
  'Replacement contract' \
  'Experimental'
git -C "$specification_trailing_lines_fixture" add .
git -C "$specification_trailing_lines_fixture" commit -qm add-trailing-lines
specification_trailing_lines_head="$(git -C "$specification_trailing_lines_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/specification-trailing-lines-body.md" \
  'specification' \
  'This fixture appends blank lines while superseding a specification.'
expect_repository_check_failure \
  "$specification_trailing_lines_fixture" \
  'Expected trailing specification lines during supersession to fail.' \
  'may only transition to Superseded with a replacement reference' \
  "$specification_trailing_lines_base" \
  "$specification_trailing_lines_head" \
  "$temporary_directory/specification-trailing-lines-body.md"

downgraded_specification_replacement_fixture="$temporary_directory/downgraded-specification-replacement-fixture"
create_fixture "$downgraded_specification_replacement_fixture"
write_specification \
  "$downgraded_specification_replacement_fixture" \
  'original-contract.md' \
  'Original contract' \
  'Superseded' \
  'replacement-contract.md'
write_specification \
  "$downgraded_specification_replacement_fixture" \
  'replacement-contract.md' \
  'Replacement contract' \
  'Experimental'
git -C "$downgraded_specification_replacement_fixture" add .
git -C "$downgraded_specification_replacement_fixture" commit -qm baseline
downgraded_specification_replacement_base="$(git -C "$downgraded_specification_replacement_fixture" rev-parse HEAD)"
write_specification \
  "$downgraded_specification_replacement_fixture" \
  'replacement-contract.md' \
  'Replacement contract' \
  'Proposed'
git -C "$downgraded_specification_replacement_fixture" add .
git -C "$downgraded_specification_replacement_fixture" commit -qm downgrade-replacement
downgraded_specification_replacement_head="$(git -C "$downgraded_specification_replacement_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/downgraded-specification-replacement-body.md" \
  'specification' \
  'This fixture downgrades the active target of a superseded specification.'
expect_repository_check_failure \
  "$downgraded_specification_replacement_fixture" \
  'Expected a downgraded specification replacement to fail.' \
  'replacement must be experimental, validated, or superseded' \
  "$downgraded_specification_replacement_base" \
  "$downgraded_specification_replacement_head" \
  "$temporary_directory/downgraded-specification-replacement-body.md"

valid_specification_replacement_fixture="$temporary_directory/valid-specification-replacement-fixture"
create_fixture "$valid_specification_replacement_fixture"
write_specification "$valid_specification_replacement_fixture" 'original-contract.md' 'Original contract' 'Validated'
git -C "$valid_specification_replacement_fixture" add .
git -C "$valid_specification_replacement_fixture" commit -qm baseline
valid_specification_replacement_base="$(git -C "$valid_specification_replacement_fixture" rev-parse HEAD)"
write_specification \
  "$valid_specification_replacement_fixture" \
  'original-contract.md' \
  'Original contract' \
  'Superseded' \
  'replacement-contract.md'
write_specification "$valid_specification_replacement_fixture" 'replacement-contract.md' 'Replacement contract' 'Experimental'
git -C "$valid_specification_replacement_fixture" add .
git -C "$valid_specification_replacement_fixture" commit -qm valid-specification-replacement
valid_specification_replacement_head="$(git -C "$valid_specification_replacement_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/valid-specification-replacement-body.md" \
  'specification' \
  'This fixture replaces a specification with a new experimental contract.'
(
  cd "$valid_specification_replacement_fixture"
  ./scripts/check-documentation.sh \
    "$valid_specification_replacement_base" \
    "$valid_specification_replacement_head" \
    "$temporary_directory/valid-specification-replacement-body.md"
) >/dev/null

metadata_title_specification_fixture="$temporary_directory/metadata-title-specification-fixture"
create_fixture "$metadata_title_specification_fixture"
write_specification \
  "$metadata_title_specification_fixture" \
  'metadata-title-contract.md' \
  'Preserve Status: Validated text' \
  'Validated'
git -C "$metadata_title_specification_fixture" add .
git -C "$metadata_title_specification_fixture" commit -qm baseline
metadata_title_specification_base="$(git -C "$metadata_title_specification_fixture" rev-parse HEAD)"
write_specification \
  "$metadata_title_specification_fixture" \
  'metadata-title-contract.md' \
  'Preserve Status: Validated text' \
  'Superseded' \
  'replacement-contract.md'
write_specification \
  "$metadata_title_specification_fixture" \
  'replacement-contract.md' \
  'Replacement contract' \
  'Validated'
git -C "$metadata_title_specification_fixture" add .
git -C "$metadata_title_specification_fixture" commit -qm supersede
metadata_title_specification_head="$(git -C "$metadata_title_specification_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/metadata-title-specification-body.md" \
  'specification' \
  'This fixture preserves metadata-like text while superseding a specification.'
(
  cd "$metadata_title_specification_fixture"
  ./scripts/check-documentation.sh \
    "$metadata_title_specification_base" \
    "$metadata_title_specification_head" \
    "$temporary_directory/metadata-title-specification-body.md"
) >/dev/null

rejected_governance_fixture="$temporary_directory/rejected-governance-fixture"
create_fixture "$rejected_governance_fixture"
git -C "$rejected_governance_fixture" add .
git -C "$rejected_governance_fixture" commit -qm baseline
rejected_governance_base="$(git -C "$rejected_governance_fixture" rev-parse HEAD)"
printf '\nRejected governance rule.\n' >> "$rejected_governance_fixture/AGENTS.md"
write_decision \
  "$rejected_governance_fixture" \
  '0001' \
  'rejected-governance-change' \
  'Reject a governance change' \
  'Rejected' \
  'The governance change was not accepted.'
git -C "$rejected_governance_fixture" add .
git -C "$rejected_governance_fixture" commit -qm rejected-governance
rejected_governance_head="$(git -C "$rejected_governance_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/rejected-governance-body.md" \
  'decision' \
  'This fixture changes governance but records only a rejected decision.'
expect_repository_check_failure \
  "$rejected_governance_fixture" \
  'Expected a rejected decision not to authorize a governance change.' \
  'documentation governance changed without a new accepted decision record' \
  "$rejected_governance_base" \
  "$rejected_governance_head" \
  "$temporary_directory/rejected-governance-body.md"

ambiguous_status_governance_fixture="$temporary_directory/ambiguous-status-governance-fixture"
create_fixture "$ambiguous_status_governance_fixture"
git -C "$ambiguous_status_governance_fixture" add .
git -C "$ambiguous_status_governance_fixture" commit -qm baseline
ambiguous_status_governance_base="$(git -C "$ambiguous_status_governance_fixture" rev-parse HEAD)"
printf '\nGovernance change with ambiguous decision metadata.\n' >> "$ambiguous_status_governance_fixture/AGENTS.md"
write_decision \
  "$ambiguous_status_governance_fixture" \
  '0001' \
  'ambiguous-status' \
  'Ambiguous status' \
  'Accepted' \
  'Ambiguous rationale.'
printf '\nStatus: Bogus\n' >> "$ambiguous_status_governance_fixture/docs/decisions/0001-ambiguous-status.md"
git -C "$ambiguous_status_governance_fixture" add .
git -C "$ambiguous_status_governance_fixture" commit -qm ambiguous-status
ambiguous_status_governance_head="$(git -C "$ambiguous_status_governance_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/ambiguous-status-governance-body.md" \
  'decision' \
  'This fixture attempts to authorize governance with ambiguous status metadata.'
expect_repository_check_failure \
  "$ambiguous_status_governance_fixture" \
  'Expected ambiguous decision status metadata to fail.' \
  'has an invalid or missing status' \
  "$ambiguous_status_governance_base" \
  "$ambiguous_status_governance_head" \
  "$temporary_directory/ambiguous-status-governance-body.md"

misplaced_metadata_governance_fixture="$temporary_directory/misplaced-metadata-governance-fixture"
create_fixture "$misplaced_metadata_governance_fixture"
git -C "$misplaced_metadata_governance_fixture" add .
git -C "$misplaced_metadata_governance_fixture" commit -qm baseline
misplaced_metadata_governance_base="$(git -C "$misplaced_metadata_governance_fixture" rev-parse HEAD)"
printf '\nGovernance change with misplaced decision metadata.\n' >> "$misplaced_metadata_governance_fixture/AGENTS.md"
misplaced_decision="$misplaced_metadata_governance_fixture/docs/decisions/0001-misplaced-metadata.md"
{
  printf '# 0001: Misplaced metadata\n'
  printf '\n## Context\n\nStatus: Accepted\nContext.\n'
  printf '\n## Decision\n\nDate: 2026-07-21\nDecision.\n'
  printf '\n## Rationale\n\nRationale.\n'
  printf '\n## Alternatives\n\nAlternative.\n'
  printf '\n## Consequences\n\nConsequence.\n'
} > "$misplaced_decision"
git -C "$misplaced_metadata_governance_fixture" add .
git -C "$misplaced_metadata_governance_fixture" commit -qm misplaced-metadata
misplaced_metadata_governance_head="$(git -C "$misplaced_metadata_governance_fixture" rev-parse HEAD)"
write_body \
  "$temporary_directory/misplaced-metadata-governance-body.md" \
  'decision' \
  'This fixture attempts to authorize governance with misplaced metadata.'
expect_repository_check_failure \
  "$misplaced_metadata_governance_fixture" \
  'Expected misplaced decision metadata to fail.' \
  'has an invalid or missing status' \
  "$misplaced_metadata_governance_base" \
  "$misplaced_metadata_governance_head" \
  "$temporary_directory/misplaced-metadata-governance-body.md"

local_source_fixture="$temporary_directory/local-source-fixture"
create_fixture "$local_source_fixture"
git -C "$local_source_fixture" add .
git -C "$local_source_fixture" commit -qm baseline
local_source_base="$(git -C "$local_source_fixture" rev-parse HEAD)"
mkdir -p "$local_source_fixture/crates/example/src"
printf '%s\n' 'pub fn changed_behavior() {}' > "$local_source_fixture/crates/example/src/lib.rs"
git -C "$local_source_fixture" add .
git -C "$local_source_fixture" commit -qm undocumented-source
write_body \
  "$temporary_directory/local-source-body.md" \
  'none' \
  'This fixture claims a committed production change has no documentation impact.'
expect_local_check_failure \
  "$local_source_fixture" \
  "$local_source_base" \
  "$temporary_directory/local-source-body.md" \
  'Expected local change-aware validation to reject undocumented source.' \
  'production Rust source changed without an updated specification'

valid_local_fixture="$temporary_directory/valid-local-fixture"
create_fixture "$valid_local_fixture"
git -C "$valid_local_fixture" add .
git -C "$valid_local_fixture" commit -qm baseline
valid_local_base="$(git -C "$valid_local_fixture" rev-parse HEAD)"
mkdir -p "$valid_local_fixture/crates/example/src"
printf '%s\n' 'pub fn documented_behavior() {}' > "$valid_local_fixture/crates/example/src/lib.rs"
write_specification "$valid_local_fixture" 'documented-behavior.md' 'Documented behavior' 'Experimental'
git -C "$valid_local_fixture" add .
git -C "$valid_local_fixture" commit -qm documented-source
write_body \
  "$temporary_directory/valid-local-body.md" \
  'specification' \
  'This fixture documents the committed production source behavior.'
(
  cd "$valid_local_fixture"
  DOCUMENTATION_BASE_REF="$valid_local_base" \
    ./scripts/check-documentation.sh "$temporary_directory/valid-local-body.md"
) >/dev/null

dirty_local_fixture="$temporary_directory/dirty-local-fixture"
create_fixture "$dirty_local_fixture"
git -C "$dirty_local_fixture" add .
git -C "$dirty_local_fixture" commit -qm baseline
dirty_local_base="$(git -C "$dirty_local_fixture" rev-parse HEAD)"
printf '\nUncommitted governance change.\n' >> "$dirty_local_fixture/AGENTS.md"
write_body \
  "$temporary_directory/dirty-local-body.md" \
  'none' \
  'This fixture attempts change-aware validation with a dirty worktree.'
expect_local_check_failure \
  "$dirty_local_fixture" \
  "$dirty_local_base" \
  "$temporary_directory/dirty-local-body.md" \
  'Expected local change-aware validation to reject a dirty worktree.' \
  'local change-aware validation requires a clean worktree'

printf 'Documentation check tests passed.\n'
