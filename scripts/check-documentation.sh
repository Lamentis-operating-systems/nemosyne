#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "$repository_root"

documentation_root="${DOCUMENTATION_ROOT:-docs}"
failures=0

fail() {
  printf 'documentation error: %s\n' "$1" >&2
  failures=$((failures + 1))
}

require_file() {
  local file="$1"

  if [[ ! -f "$file" ]]; then
    fail "required file is missing: $file"
  fi
}

require_heading() {
  local file="$1"
  local heading="$2"
  local count

  count="$(grep -Fxc "$heading" "$file" || true)"
  if [[ "$count" -ne 1 ]]; then
    fail "$file must contain '$heading' exactly once"
  fi
}

require_heading_order() {
  local file="$1"
  shift

  local heading
  local line
  local previous_line=0

  for heading in "$@"; do
    line="$(grep -Fnx "$heading" "$file" | head -n 1 | cut -d: -f1 || true)"
    if [[ -n "$line" && "$line" -le "$previous_line" ]]; then
      fail "$file has '$heading' out of order"
    fi
    if [[ -n "$line" ]]; then
      previous_line="$line"
    fi
  done
}

require_section_content() {
  local file="$1"
  local heading="$2"

  if ! awk -v heading="$heading" '
    $0 == heading {
      in_section = 1
      next
    }
    in_section && /^## / {
      exit
    }
    in_section && $0 !~ /^[[:space:]]*$/ && $0 !~ /^<!--/ {
      content = 1
    }
    END {
      exit content ? 0 : 1
    }
  ' "$file"; then
    fail "$file has no content under '$heading'"
  fi
}

validate_specification() {
  local file="$1"
  local filename="${file##*/}"

  if [[ ! "$filename" =~ ^[a-z0-9]+(-[a-z0-9]+)*\.md$ ]]; then
    fail "$file must use lowercase kebab-case"
  fi

  if [[ "$(grep -Ec '^Status: (Proposed|Experimental|Validated|Superseded)$' "$file" || true)" -ne 1 ]]; then
    fail "$file has an invalid or missing status"
  fi

  if [[ "$(head -n 1 "$file")" == '# Title' ]] || ! head -n 1 "$file" | grep -Eq '^# .+'; then
    fail "$file must have a specific title"
  fi

  if grep -Fq '<!-- Replace this comment' "$file"; then
    fail "$file contains unresolved template placeholders"
  fi

  require_heading "$file" '## Purpose'
  require_heading "$file" '## Definitions'
  require_heading "$file" '## Preconditions'
  require_heading "$file" '## Invariants'
  require_heading "$file" '## Edge cases'
  require_heading "$file" '## Verification'
  require_heading "$file" '## Open questions'
  require_heading "$file" '## References'

  require_heading_order \
    "$file" \
    '## Purpose' \
    '## Definitions' \
    '## Preconditions' \
    '## Invariants' \
    '## Edge cases' \
    '## Verification' \
    '## Open questions' \
    '## References'

  require_section_content "$file" '## Purpose'
  require_section_content "$file" '## Definitions'
  require_section_content "$file" '## Preconditions'
  require_section_content "$file" '## Invariants'
  require_section_content "$file" '## Edge cases'
  require_section_content "$file" '## Verification'
  require_section_content "$file" '## Open questions'
  require_section_content "$file" '## References'
}

validate_decision() {
  local file="$1"
  local filename="${file##*/}"
  local identifier="${filename%%-*}"

  if [[ ! "$filename" =~ ^[0-9]{4}-[a-z0-9]+(-[a-z0-9]+)*\.md$ ]]; then
    fail "$file must start with a four-digit identifier and use lowercase kebab-case"
  fi

  if [[ "$(grep -Ec '^Status: (Proposed|Accepted|Rejected|Superseded)$' "$file" || true)" -ne 1 ]]; then
    fail "$file has an invalid or missing status"
  fi

  if [[ "$(grep -Ec '^Date: [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$file" || true)" -ne 1 ]]; then
    fail "$file has an invalid or missing date"
  fi

  if grep -Fxq 'Status: Superseded' "$file"; then
    if [[ "$(grep -Ec '^Superseded by: [0-9]{4}-[a-z0-9]+(-[a-z0-9]+)*\.md$' "$file" || true)" -ne 1 ]]; then
      fail "$file must identify exactly one replacement when superseded"
    else
      local replacement
      replacement="$(sed -n 's/^Superseded by: //p' "$file")"
      require_file "${file%/*}/$replacement"
    fi
  elif grep -Eq '^Superseded by:' "$file"; then
    fail "$file may identify a replacement only when superseded"
  fi

  if ! head -n 1 "$file" | grep -Eq "^# $identifier: .+" || [[ "$(head -n 1 "$file")" == "# $identifier: Title" ]]; then
    fail "$file title must start with '# $identifier:' and be specific"
  fi

  if grep -Fq '<!-- Replace this comment' "$file"; then
    fail "$file contains unresolved template placeholders"
  fi

  require_heading "$file" '## Context'
  require_heading "$file" '## Decision'
  require_heading "$file" '## Rationale'
  require_heading "$file" '## Alternatives'
  require_heading "$file" '## Consequences'

  require_heading_order \
    "$file" \
    '## Context' \
    '## Decision' \
    '## Rationale' \
    '## Alternatives' \
    '## Consequences'

  require_section_content "$file" '## Context'
  require_section_content "$file" '## Decision'
  require_section_content "$file" '## Rationale'
  require_section_content "$file" '## Alternatives'
  require_section_content "$file" '## Consequences'
}

validate_decision_history() {
  local base_sha="$1"
  local head_sha="$2"
  local change
  local change_type
  local old_path
  local new_path
  local base_status
  local head_status
  local base_without_metadata
  local head_without_metadata

  while IFS=$'\t' read -r change old_path new_path; do
    [[ -z "$change" ]] && continue
    change_type="${change:0:1}"

    case "$old_path" in
      docs/decisions/[0-9][0-9][0-9][0-9]-*.md) ;;
      *) continue ;;
    esac

    case "$change_type" in
      A) ;;
      D)
        base_status="$(git show "$base_sha:$old_path" | sed -n 's/^Status: //p' | head -n 1)"
        if [[ "$base_status" != 'Proposed' ]]; then
          fail "$old_path cannot be deleted after leaving Proposed status"
        fi
        ;;
      M)
        base_status="$(git show "$base_sha:$old_path" | sed -n 's/^Status: //p' | head -n 1)"
        head_status="$(git show "$head_sha:$old_path" | sed -n 's/^Status: //p' | head -n 1)"

        case "$base_status" in
          Proposed) ;;
          Accepted)
            base_without_metadata="$(git show "$base_sha:$old_path" | sed '/^Status: /d;/^Superseded by: /d')"
            head_without_metadata="$(git show "$head_sha:$old_path" | sed '/^Status: /d;/^Superseded by: /d')"

            if [[ "$head_status" != 'Superseded' || "$base_without_metadata" != "$head_without_metadata" ]]; then
              fail "$old_path may only transition from Accepted to Superseded with a replacement reference"
            fi
            ;;
          Rejected|Superseded)
            fail "$old_path is historical and cannot be modified"
            ;;
          *)
            fail "$old_path has an invalid historical status"
            ;;
        esac
        ;;
      R)
        fail "$old_path cannot be renamed to $new_path; decision filenames are permanent"
        ;;
      *)
        fail "unsupported decision history change for $old_path: $change"
        ;;
    esac
  done < <(git diff --name-status "$base_sha" "$head_sha" -- docs/decisions)
}

validate_specification_history() {
  local base_sha="$1"
  local head_sha="$2"
  local change
  local change_type
  local old_path
  local new_path
  local base_status

  while IFS=$'\t' read -r change old_path new_path; do
    [[ -z "$change" ]] && continue
    change_type="${change:0:1}"

    case "$old_path" in
      docs/specifications/README.md|docs/specifications/TEMPLATE.md) continue ;;
      docs/specifications/*.md) ;;
      *) continue ;;
    esac

    case "$change_type" in
      A) ;;
      D)
        fail "$old_path cannot be deleted; supersede it instead"
        ;;
      M)
        base_status="$(git show "$base_sha:$old_path" | sed -n 's/^Status: //p' | head -n 1)"
        if [[ "$base_status" == 'Superseded' ]]; then
          fail "$old_path is historical and cannot be modified"
        fi
        ;;
      R)
        fail "$old_path cannot be renamed to $new_path; specification filenames are permanent"
        ;;
      *)
        fail "unsupported specification history change for $old_path: $change"
        ;;
    esac
  done < <(git diff --name-status "$base_sha" "$head_sha" -- docs/specifications)
}

require_file "$documentation_root/README.md"
require_file "$documentation_root/specifications/README.md"
require_file "$documentation_root/specifications/TEMPLATE.md"
require_file "$documentation_root/decisions/README.md"
require_file "$documentation_root/decisions/TEMPLATE.md"

if [[ "$documentation_root" == 'docs' ]]; then
  require_file '.github/PULL_REQUEST_TEMPLATE.md'
  require_file 'scripts/check-documentation.sh'
  require_file 'scripts/test-documentation-check.sh'
fi

while IFS= read -r file; do
  validate_specification "$file"
done < <(find "$documentation_root/specifications" -type f -name '*.md' ! -name 'README.md' ! -name 'TEMPLATE.md' | sort)

while IFS= read -r file; do
  validate_decision "$file"
done < <(find "$documentation_root/decisions" -type f -name '*.md' ! -name 'README.md' ! -name 'TEMPLATE.md' | sort)

duplicate_decision_identifiers="$({
  while IFS= read -r file; do
    filename="${file##*/}"
    printf '%s\n' "${filename%%-*}"
  done < <(find "$documentation_root/decisions" -type f -name '[0-9][0-9][0-9][0-9]-*.md' | sort)
} | sort | uniq -d)"

if [[ -n "$duplicate_decision_identifiers" ]]; then
  fail "decision identifiers must be unique: $duplicate_decision_identifiers"
fi

if [[ "$#" -ne 0 && "$#" -ne 3 ]]; then
  printf 'usage: %s [<base-sha> <head-sha> <pull-request-body>]\n' "$0" >&2
  exit 2
fi

if [[ "$#" -eq 3 ]]; then
  base_sha="$1"
  head_sha="$2"
  pull_request_body="$3"
  comparison_base=""

  if ! comparison_base="$(git merge-base "$base_sha" "$head_sha")"; then
    fail "cannot determine the pull request merge base"
    comparison_base="$head_sha"
  fi

  if [[ ! -f "$pull_request_body" ]]; then
    fail "pull request body file does not exist: $pull_request_body"
  else
    impact_count="$(grep -Ec '^Documentation impact:' "$pull_request_body" || true)"
    reason_count="$(grep -Ec '^Documentation reason:' "$pull_request_body" || true)"

    if [[ "$impact_count" -ne 1 ]]; then
      fail "pull request body must contain exactly one 'Documentation impact:' declaration"
    fi

    if [[ "$reason_count" -ne 1 ]]; then
      fail "pull request body must contain exactly one 'Documentation reason:' declaration"
    fi

    impact="$(sed -n 's/^Documentation impact:[[:space:]]*//p' "$pull_request_body" | head -n 1)"
    reason="$(sed -n 's/^Documentation reason:[[:space:]]*//p' "$pull_request_body" | head -n 1)"
    reason="$(printf '%s' "$reason" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"

    case "$impact" in
      none|specification|decision|specification-and-decision) ;;
      *) fail "documentation impact must be none, specification, decision, or specification-and-decision" ;;
    esac

    if [[ "${#reason}" -lt 20 ]]; then
      fail "documentation reason must contain at least 20 characters"
    fi

    if [[ "$reason" == *'<!--'* ]]; then
      fail "documentation reason must not contain an HTML comment"
    fi

    specification_changed=false
    decision_changed=false
    production_source_changed=false

    while IFS= read -r file; do
      case "$file" in
        crates/*/src/*.rs) production_source_changed=true ;;
        docs/specifications/*.md)
          if [[ "$file" != 'docs/specifications/README.md' && "$file" != 'docs/specifications/TEMPLATE.md' ]]; then
            specification_changed=true
          fi
          ;;
        docs/decisions/[0-9][0-9][0-9][0-9]-*.md) decision_changed=true ;;
      esac
    done < <(git diff --name-only "$comparison_base" "$head_sha")

    if [[ "$production_source_changed" == true && "$specification_changed" != true ]]; then
      fail "production Rust source changed without an updated specification"
    fi

    if [[ -n "$comparison_base" ]]; then
      validate_decision_history "$comparison_base" "$head_sha"
      validate_specification_history "$comparison_base" "$head_sha"
    fi

    case "$impact" in
      none)
        if [[ "$specification_changed" == true || "$decision_changed" == true ]]; then
          fail "documentation impact is none, but a specification or decision changed"
        fi
        ;;
      specification)
        if [[ "$specification_changed" != true || "$decision_changed" == true ]]; then
          fail "documentation impact does not match the changed specification and decision files"
        fi
        ;;
      decision)
        if [[ "$specification_changed" == true || "$decision_changed" != true ]]; then
          fail "documentation impact does not match the changed specification and decision files"
        fi
        ;;
      specification-and-decision)
        if [[ "$specification_changed" != true || "$decision_changed" != true ]]; then
          fail "documentation impact requires both a specification and a decision change"
        fi
        ;;
    esac
  fi
fi

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi

printf 'Documentation checks passed.\n'
