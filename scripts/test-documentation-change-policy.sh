#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "$repository_root"

classify() {
  ./scripts/classify-documentation-change.sh "$@"
}

expect_classification() {
  local expected="$1"
  shift

  local actual
  actual="$(classify "$@")"
  if [[ "$actual" != "$expected" ]]; then
    printf 'Expected documentation change policy %s, received %s.\n' "$expected" "$actual" >&2
    exit 1
  fi
}

repository='Lamentis-operating-systems/nemosyne'

expect_classification \
  'skip' \
  'pull_request' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  "$repository" \
  "$repository" \
  'dependabot/github_actions/actions/checkout-7.0.1'

expect_classification \
  'validate' \
  'push' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  "$repository" \
  "$repository" \
  'dependabot/github_actions/actions/checkout-7.0.1'

expect_classification \
  'validate' \
  'pull_request' \
  'maintainer' \
  'dependabot[bot]' \
  "$repository" \
  "$repository" \
  'dependabot/github_actions/actions/checkout-7.0.1'

expect_classification \
  'validate' \
  'pull_request' \
  'dependabot[bot]' \
  'maintainer' \
  "$repository" \
  "$repository" \
  'dependabot/github_actions/actions/checkout-7.0.1'

expect_classification \
  'validate' \
  'pull_request' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  'external/nemosyne' \
  "$repository" \
  'dependabot/github_actions/actions/checkout-7.0.1'

expect_classification \
  'validate' \
  'pull_request' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  "$repository" \
  "$repository" \
  'dependabot/cargo/serde-2.0.0'

expect_classification \
  'validate' \
  'pull_request' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  "$repository" \
  "$repository" \
  'dependabot/github_actions/'

expect_classification \
  'validate' \
  'pull_request' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  '' \
  '' \
  'dependabot/github_actions/actions/checkout-7.0.1'

if classify \
  'pull_request' \
  'dependabot[bot]' \
  'dependabot[bot]' \
  "$repository" \
  "$repository" >/dev/null 2>&1; then
  printf 'Expected incomplete policy input to fail.\n' >&2
  exit 1
fi

printf 'Documentation change policy tests passed.\n'
