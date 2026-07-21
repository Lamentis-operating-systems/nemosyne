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

expect_repository_check_failure() {
  local repository="$1"
  local message="$2"
  shift 2

  if (
    cd "$repository"
    ./scripts/check-documentation.sh "$@"
  ) >/dev/null 2>&1; then
    printf '%s\n' "$message" >&2
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
  'Expected missing agent instructions to fail.'

create_fixture "$invalid_agent_fixture"
printf '# AGENTS.md\n' > "$invalid_agent_fixture/AGENTS.md"
expect_repository_check_failure \
  "$invalid_agent_fixture" \
  'Expected incomplete agent instructions to fail.'

create_fixture "$nested_agent_fixture"
mkdir -p "$nested_agent_fixture/crates/example"
printf '# Conflicting instructions\n' > "$nested_agent_fixture/crates/example/AGENTS.md"
git -C "$nested_agent_fixture" add .
expect_repository_check_failure \
  "$nested_agent_fixture" \
  'Expected nested agent instructions to fail.'

create_fixture "$override_agent_fixture"
printf '# Override instructions\n' > "$override_agent_fixture/AGENTS.override.md"
git -C "$override_agent_fixture" add .
expect_repository_check_failure \
  "$override_agent_fixture" \
  'Expected overriding agent instructions to fail.'

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

printf 'Documentation check tests passed.\n'
