#!/usr/bin/env bash

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$repository_root"

while IFS= read -r environment_name; do
  if [[ "$environment_name" == GIT_* ]]; then
    unset "$environment_name"
  fi
done < <(compgen -e)
export GIT_CONFIG_COUNT=1
export GIT_CONFIG_GLOBAL=/dev/null
export GIT_CONFIG_KEY_0=core.worktree
export GIT_CONFIG_NOSYSTEM=1
export GIT_CONFIG_VALUE_0="$repository_root"
export GIT_NO_REPLACE_OBJECTS=1
export GIT_OPTIONAL_LOCKS=0
export GIT_TERMINAL_PROMPT=0

documentation_root="${DOCUMENTATION_ROOT:-docs}"
failures=0

fail() {
  printf 'documentation error: %s\n' "$1" >&2
  failures=$((failures + 1))
}

require_file() {
  local file="$1"

  if [[ -L "$file" ]]; then
    fail "required file must be a regular file, not a symbolic link: $file"
  elif [[ ! -f "$file" ]]; then
    fail "required file is missing: $file"
  elif [[ ! -s "$file" ]]; then
    fail "required file is empty: $file"
  fi
}

require_directory() {
  local directory="$1"

  if [[ -L "$directory" ]]; then
    fail "required directory must be a regular directory, not a symbolic link: $directory"
  elif [[ ! -d "$directory" ]]; then
    fail "required directory is missing: $directory"
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

validate_flat_markdown_directory() {
  local directory="$1"
  local file
  local relative_path

  while IFS= read -r file; do
    relative_path="${file#"$directory"/}"
    if [[ "$relative_path" == */* ]]; then
      fail "$file must be stored directly in $directory"
    fi
    if [[ -L "$file" ]]; then
      fail "$file must be a regular file, not a symbolic link"
    fi
  done < <(find "$directory" \( -type f -o -type l \) -name '*.md' | sort)
}

validate_specification() {
  local file="$1"
  local filename="${file##*/}"
  local replacement
  local replacement_path
  local replacement_status
  local status

  if [[ ! "$filename" =~ ^[a-z0-9]+(-[a-z0-9]+)*\.md$ ]]; then
    fail "$file must use lowercase kebab-case"
  fi

  if [[ "$(grep -Ec '^Status:' "$file" || true)" -ne 1 ]] ||
    ! sed -n '3p' "$file" | grep -Eq '^Status: (Proposed|Experimental|Validated|Superseded)$'; then
    fail "$file has an invalid or missing status"
  fi

  status="$(sed -n '3s/^Status: //p' "$file")"
  if [[ "$status" == 'Superseded' ]]; then
    if [[ "$(grep -Ec '^Superseded by:' "$file" || true)" -ne 1 ]] ||
      ! sed -n '4p' "$file" | grep -Eq '^Superseded by: [a-z0-9]+(-[a-z0-9]+)*\.md$'; then
      fail "$file must identify exactly one replacement when superseded"
    else
      replacement="$(sed -n '4s/^Superseded by: //p' "$file")"
      replacement_path="${file%/*}/$replacement"

      if [[ "$replacement" == "$filename" ]]; then
        fail "$file cannot supersede itself"
      fi
      require_file "$replacement_path"
      if [[ -f "$replacement_path" ]]; then
        replacement_status="$(sed -n '3s/^Status: //p' "$replacement_path")"
        case "$replacement_status" in
          Experimental|Validated|Superseded) ;;
          *) fail "$file replacement must be experimental, validated, or superseded" ;;
        esac
      fi
    fi
  elif grep -Eq '^Superseded by:' "$file"; then
    fail "$file may identify a replacement only when superseded"
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
  local replacement
  local replacement_identifier
  local replacement_path
  local replacement_status
  local status

  if [[ ! "$filename" =~ ^[0-9]{4}-[a-z0-9]+(-[a-z0-9]+)*\.md$ ]]; then
    fail "$file must start with a four-digit identifier and use lowercase kebab-case"
  fi

  if [[ "$(grep -Ec '^Status:' "$file" || true)" -ne 1 ]] ||
    ! sed -n '3p' "$file" | grep -Eq '^Status: (Proposed|Accepted|Rejected|Superseded)$'; then
    fail "$file has an invalid or missing status"
  fi

  if [[ "$(grep -Ec '^Date:' "$file" || true)" -ne 1 ]] ||
    ! sed -n '4p' "$file" | grep -Eq '^Date: [0-9]{4}-[0-9]{2}-[0-9]{2}$'; then
    fail "$file has an invalid or missing date"
  fi

  status="$(sed -n '3s/^Status: //p' "$file")"
  if [[ "$status" == 'Superseded' ]]; then
    if [[ "$(grep -Ec '^Superseded by:' "$file" || true)" -ne 1 ]] ||
      ! sed -n '5p' "$file" | grep -Eq '^Superseded by: [0-9]{4}-[a-z0-9]+(-[a-z0-9]+)*\.md$'; then
      fail "$file must identify exactly one replacement when superseded"
    else
      replacement="$(sed -n '5s/^Superseded by: //p' "$file")"
      replacement_identifier="${replacement%%-*}"
      replacement_path="${file%/*}/$replacement"

      if [[ "$replacement" == "$filename" ]]; then
        fail "$file cannot supersede itself"
      elif [[ "$identifier" =~ ^[0-9]{4}$ && "$replacement_identifier" =~ ^[0-9]{4}$ ]] &&
        ((10#$replacement_identifier <= 10#$identifier)); then
        fail "$file replacement must use a later decision identifier"
      fi

      require_file "$replacement_path"
      if [[ -f "$replacement_path" ]]; then
        replacement_status="$(sed -n '3s/^Status: //p' "$replacement_path")"
        case "$replacement_status" in
          Accepted|Superseded) ;;
          *) fail "$file replacement must be accepted or superseded" ;;
        esac
      fi
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
  local actual_decision
  local expected_decision
  local replacement
  local replacement_path
  local replacement_status

  while IFS=$'\t' read -r change old_path new_path; do
    [[ -z "$change" ]] && continue
    change_type="${change:0:1}"

    case "$old_path" in
      docs/decisions/[0-9][0-9][0-9][0-9]-*.md) ;;
      *) continue ;;
    esac

    case "$change_type" in
      A)
        head_status="$(git show "$head_sha:$old_path" | sed -n '3s/^Status: //p')"
        if [[ "$head_status" == 'Superseded' ]]; then
          fail "$old_path cannot be added in superseded state"
        fi
        ;;
      D)
        base_status="$(git show "$base_sha:$old_path" | sed -n '3s/^Status: //p')"
        if [[ "$base_status" != 'Proposed' ]]; then
          fail "$old_path cannot be deleted after leaving Proposed status"
        fi
        ;;
      M)
        base_status="$(git show "$base_sha:$old_path" | sed -n '3s/^Status: //p')"
        head_status="$(git show "$head_sha:$old_path" | sed -n '3s/^Status: //p')"

        case "$base_status" in
          Proposed) ;;
          Accepted)
            actual_decision="$(git show "$head_sha:$old_path"; printf '\034')"
            replacement="$(printf '%s\n' "$actual_decision" | sed -n '5s/^Superseded by: //p')"
            expected_decision="$({ git show "$base_sha:$old_path"; printf '\034'; } | awk -v replacement="$replacement" '
              $0 == "Status: Accepted" && !status_replaced {
                print "Status: Superseded"
                status_replaced = 1
                next
              }
              /^Date: [0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]$/ && !replacement_added {
                print
                print "Superseded by: " replacement
                replacement_added = 1
                next
              }
              { print }
            ')"

            if [[ "$head_status" != 'Superseded' || -z "$replacement" || "$actual_decision" != "$expected_decision" ]]; then
              fail "$old_path may only transition from Accepted to Superseded with a replacement reference"
            fi

            replacement_path="${old_path%/*}/$replacement"
            if git cat-file -e "$base_sha:$replacement_path" 2>/dev/null; then
              fail "$old_path replacement must be added with the superseding change"
            elif ! git cat-file -e "$head_sha:$replacement_path" 2>/dev/null; then
              fail "$old_path replacement is missing from the superseding change"
            else
              replacement_status="$(git show "$head_sha:$replacement_path" | sed -n '3s/^Status: //p')"
              if [[ "$replacement_status" != 'Accepted' ]]; then
                fail "$old_path replacement must be accepted when added"
              fi
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

extract_trusted_delivery_checker() {
  local base_sha="$1"
  local destination="$2"

  if ! git --no-replace-objects show \
    "$base_sha:scripts/check-v1-delivery-program.py" \
    >"$destination"; then
    fail "cannot read the trusted DOC-00 comparator from $base_sha"
    return 1
  fi
}

run_trusted_delivery_checker() {
  local trusted_checker="$1"
  shift

  NEMOSYNE_REPOSITORY_ROOT="$repository_root" \
    python3 -I -S "$trusted_checker" "$@"
}

protected_delivery_digest_at() {
  local trusted_checker="$1"
  local commit="$2"
  local constant="$3"
  local digest

  if ! digest="$(
    run_trusted_delivery_checker \
      "$trusted_checker" \
      --protected-digest-at \
      "$commit" \
      "$constant" 2>/dev/null
  )"; then
    return 1
  fi

  printf '%s' "$digest"
}

validate_protected_digest_rebaseline() {
  local base_sha="$1"
  local head_sha="$2"
  local governing_decision='docs/decisions/0030-protect-doc-00-history-and-governance.md'
  local constant
  local base_digest
  local head_digest
  local digest_changed=false
  local base_status
  local head_status
  local replacement
  local replacement_path
  local replacement_status
  local trusted_checker

  # Decision 0030 and these digests are introduced together. The stricter
  # replacement rule begins once that accepted baseline exists.
  if ! git cat-file -e "$base_sha:$governing_decision" 2>/dev/null; then
    return
  fi

  trusted_checker="$(
    mktemp "${TMPDIR:-/tmp}/nemosyne-doc00-digest-checker.XXXXXX"
  )"
  if ! extract_trusted_delivery_checker "$base_sha" "$trusted_checker"; then
    rm -f "$trusted_checker"
    return
  fi

  for constant in \
    EXPECTED_PROTECTED_CONFORMANCE_SHA256 \
    EXPECTED_PROTECTED_FINDING_SHA256
  do
    if ! base_digest="$(
      protected_delivery_digest_at \
        "$trusted_checker" \
        "$base_sha" \
        "$constant"
    )"; then
      fail "cannot read a unique protected DOC-00 digest binding from $base_sha"
      rm -f "$trusted_checker"
      return
    fi
    if ! head_digest="$(
      protected_delivery_digest_at \
        "$trusted_checker" \
        "$head_sha" \
        "$constant"
    )"; then
      fail "cannot read a unique protected DOC-00 digest binding from $head_sha"
      rm -f "$trusted_checker"
      return
    fi
    if [[ "$base_digest" != "$head_digest" ]]; then
      digest_changed=true
    fi
  done
  rm -f "$trusted_checker"

  if [[ "$digest_changed" != true ]]; then
    return
  fi

  base_status="$(
    git show "$base_sha:$governing_decision" 2>/dev/null |
      sed -n '3s/^Status: //p' || true
  )"
  head_status="$(
    git show "$head_sha:$governing_decision" 2>/dev/null |
      sed -n '3s/^Status: //p' || true
  )"
  replacement="$(
    git show "$head_sha:$governing_decision" 2>/dev/null |
      sed -n '5s/^Superseded by: //p' || true
  )"
  replacement_path="docs/decisions/$replacement"
  replacement_status=''
  if [[ -n "$replacement" ]] &&
    git cat-file -e "$head_sha:$replacement_path" 2>/dev/null; then
    replacement_status="$(
      git show "$head_sha:$replacement_path" |
        sed -n '3s/^Status: //p'
    )"
  fi

  if [[ "$base_status" != 'Accepted' ||
    "$head_status" != 'Superseded' ||
    -z "$replacement" ||
    "$replacement" == "$(basename "$governing_decision")" ||
    "$replacement_status" != 'Accepted' ]] ||
    git cat-file -e "$base_sha:$replacement_path" 2>/dev/null; then
    fail "protected DOC-00 digest rebaseline requires Decision 0030 to be superseded by a newly added Accepted replacement decision"
  fi
}

validate_delivery_append_only_history() {
  local base_sha="$1"
  local head_sha="$2"
  local delivery_program='docs/specifications/v1-delivery-program.md'
  local governing_decision='docs/decisions/0030-protect-doc-00-history-and-governance.md'
  local trusted_checker=''
  local output

  if ! git cat-file -e "$base_sha:$delivery_program" 2>/dev/null ||
    ! git cat-file -e "$head_sha:$delivery_program" 2>/dev/null; then
    return
  fi

  if ! git cat-file -e "$head_sha:$governing_decision" 2>/dev/null; then
    if ! git --no-replace-objects grep -Fq \
      'Decision 0030' \
      "$head_sha" \
      -- \
      "$delivery_program"; then
      return
    fi
  fi

  trusted_checker="$(
    mktemp "${TMPDIR:-/tmp}/nemosyne-doc00-append-checker.XXXXXX"
  )"
  if git cat-file -e "$base_sha:$governing_decision" 2>/dev/null; then
    if ! extract_trusted_delivery_checker "$base_sha" "$trusted_checker"; then
      rm -f "$trusted_checker"
      return
    fi
  elif ! extract_trusted_delivery_checker "$head_sha" "$trusted_checker"; then
    rm -f "$trusted_checker"
    return
  fi

  if ! output="$(
    run_trusted_delivery_checker "$trusted_checker" \
      --check-append-only \
      "$base_sha" \
      "$head_sha" 2>&1
  )"; then
    rm -f "$trusted_checker"
    output="${output#V1 delivery-program check failed: }"
    fail "$output"
    return
  fi
  rm -f "$trusted_checker"
}

validate_specification_history() {
  local base_sha="$1"
  local head_sha="$2"
  local change
  local change_type
  local old_path
  local new_path
  local base_status
  local head_status
  local actual_specification
  local expected_specification
  local replacement
  local replacement_path
  local replacement_status

  while IFS=$'\t' read -r change old_path new_path; do
    [[ -z "$change" ]] && continue
    change_type="${change:0:1}"

    case "$old_path" in
      docs/specifications/README.md|docs/specifications/TEMPLATE.md) continue ;;
      docs/specifications/*.md) ;;
      *) continue ;;
    esac

    case "$change_type" in
      A)
        head_status="$(git show "$head_sha:$old_path" | sed -n '3s/^Status: //p')"
        if [[ "$head_status" == 'Superseded' ]]; then
          fail "$old_path cannot be added in superseded state"
        fi
        ;;
      D)
        fail "$old_path cannot be deleted; supersede it instead"
        ;;
      M)
        base_status="$(git show "$base_sha:$old_path" | sed -n '3s/^Status: //p')"
        head_status="$(git show "$head_sha:$old_path" | sed -n '3s/^Status: //p')"
        if [[ "$base_status" == 'Superseded' ]]; then
          fail "$old_path is historical and cannot be modified"
        elif [[ "$head_status" == 'Superseded' ]]; then
          actual_specification="$(git show "$head_sha:$old_path"; printf '\034')"
          replacement="$(printf '%s\n' "$actual_specification" | sed -n '4s/^Superseded by: //p')"
          expected_specification="$({ git show "$base_sha:$old_path"; printf '\034'; } | awk -v base_status="$base_status" -v replacement="$replacement" '
            $0 == "Status: " base_status && !status_replaced {
              print "Status: Superseded"
              print "Superseded by: " replacement
              status_replaced = 1
              next
            }
            { print }
          ')"
          replacement_path="${old_path%/*}/$replacement"

          if [[ -z "$replacement" || "$actual_specification" != "$expected_specification" ]]; then
            fail "$old_path may only transition to Superseded with a replacement reference"
          elif git cat-file -e "$base_sha:$replacement_path" 2>/dev/null; then
            fail "$old_path replacement must be added with the superseding change"
          elif ! git cat-file -e "$head_sha:$replacement_path" 2>/dev/null; then
            fail "$old_path replacement is missing from the superseding change"
          else
            replacement_status="$(git show "$head_sha:$replacement_path" | sed -n '3s/^Status: //p')"
            case "$replacement_status" in
              Experimental|Validated) ;;
              *) fail "$old_path replacement must be experimental or validated when added" ;;
            esac
          fi
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

validate_documentation_history() {
  local base_sha
  local head_sha
  base_sha="$(git rev-parse "$1")"
  head_sha="$(git rev-parse "$2")"
  local previous_sha="$base_sha"
  local commit_sha

  while IFS= read -r commit_sha; do
    [[ -z "$commit_sha" ]] && continue
    validate_decision_history "$previous_sha" "$commit_sha"
    validate_specification_history "$previous_sha" "$commit_sha"
    validate_delivery_append_only_history "$previous_sha" "$commit_sha"
    previous_sha="$commit_sha"
  done < <(git rev-list --reverse --first-parent "$base_sha..$head_sha")

  if [[ "$previous_sha" != "$head_sha" ]]; then
    fail "pull request history is not connected to its merge base through first parents"
  fi
}

validate_agent_instruction_files() {
  local file
  local unexpected_files=""

  while IFS= read -r file; do
    case "$file" in
      AGENTS.md) ;;
      AGENTS.override.md|*/AGENTS.md|*/AGENTS.override.md)
        unexpected_files+="${unexpected_files:+, }$file"
        ;;
    esac
  done < <(git ls-files)

  if [[ -n "$unexpected_files" ]]; then
    fail "additional agent instruction files are not allowed: $unexpected_files"
  fi
}

require_directory "$documentation_root"
require_directory "$documentation_root/specifications"
require_directory "$documentation_root/decisions"
require_file "$documentation_root/README.md"
require_file "$documentation_root/specifications/README.md"
require_file "$documentation_root/specifications/TEMPLATE.md"
require_file "$documentation_root/decisions/README.md"
require_file "$documentation_root/decisions/TEMPLATE.md"

if [[ "$documentation_root" == 'docs' ]]; then
  require_file 'AGENTS.md'
  require_file '.github/PULL_REQUEST_TEMPLATE.md'
  require_file 'scripts/classify-documentation-change.sh'
  require_file 'scripts/check-documentation.sh'
  require_file 'scripts/test-documentation-change-policy.sh'
  require_file 'scripts/test-documentation-check.sh'

  if [[ -s 'AGENTS.md' ]]; then
    require_heading 'AGENTS.md' '## Sources of truth'
    require_heading 'AGENTS.md' '## Required workflow'
    require_heading 'AGENTS.md' '## Documentation'
    require_heading 'AGENTS.md' '## Engineering'
    require_heading 'AGENTS.md' '## Verification'
    require_heading 'AGENTS.md' '## Code Review Rules'

    require_heading_order \
      'AGENTS.md' \
      '## Sources of truth' \
      '## Required workflow' \
      '## Documentation' \
      '## Engineering' \
      '## Verification' \
      '## Code Review Rules'

    require_section_content 'AGENTS.md' '## Sources of truth'
    require_section_content 'AGENTS.md' '## Required workflow'
    require_section_content 'AGENTS.md' '## Documentation'
    require_section_content 'AGENTS.md' '## Engineering'
    require_section_content 'AGENTS.md' '## Verification'
    require_section_content 'AGENTS.md' '## Code Review Rules'
  fi

  validate_agent_instruction_files
fi

validate_flat_markdown_directory "$documentation_root/specifications"
validate_flat_markdown_directory "$documentation_root/decisions"

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

if [[ "$#" -ne 0 && "$#" -ne 1 && "$#" -ne 3 ]]; then
  printf 'usage: %s [<pull-request-body> | <base-sha> <head-sha> <pull-request-body>]\n' "$0" >&2
  exit 2
fi

if [[ "$#" -ne 0 ]]; then
  if [[ "$#" -eq 1 ]]; then
    base_sha="${DOCUMENTATION_BASE_REF:-origin/main}"
    head_sha='HEAD'
    pull_request_body="$1"

    if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
      fail "local change-aware validation requires a clean worktree"
    fi
  else
    base_sha="$1"
    head_sha="$2"
    pull_request_body="$3"
  fi

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
    new_accepted_decision_added=false
    governance_changed=false
    production_source_changed=false

    while IFS= read -r -d '' file; do
      case "$file" in
        crates/*/src/*.rs) production_source_changed=true ;;
        docs/specifications/README.md|docs/specifications/TEMPLATE.md) governance_changed=true ;;
        docs/specifications/*.md) specification_changed=true ;;
        docs/decisions/README.md|docs/decisions/TEMPLATE.md) governance_changed=true ;;
        docs/decisions/[0-9][0-9][0-9][0-9]-*.md) decision_changed=true ;;
        AGENTS.md|\
          CONTRIBUTING.md|\
          .github/PULL_REQUEST_TEMPLATE.md|\
          docs/README.md|\
          scripts/classify-documentation-change.sh|\
          scripts/check-documentation.sh|\
          scripts/test-documentation-change-policy.sh|\
          scripts/test-documentation-check.sh|\
          scripts/check-v1-delivery-program.py|\
          scripts/test-v1-delivery-program-check.sh)
          governance_changed=true
          ;;
        .github/workflows/*.yml|.github/workflows/*.yaml) governance_changed=true ;;
      esac
    done < <(git diff --no-renames --name-only -z "$comparison_base" "$head_sha" --)

    while IFS= read -r file; do
      case "$file" in
        docs/decisions/[0-9][0-9][0-9][0-9]-*.md)
          if [[ "$(git show "$head_sha:$file" | sed -n '3s/^Status: //p')" == 'Accepted' ]]; then
            new_accepted_decision_added=true
          fi
          ;;
      esac
    done < <(git diff --no-renames --diff-filter=A --name-only "$comparison_base" "$head_sha" --)

    if [[ "$production_source_changed" == true && "$specification_changed" != true ]]; then
      fail "production Rust source changed without an updated specification"
    fi

    if [[ "$governance_changed" == true && "$new_accepted_decision_added" != true ]]; then
      fail "documentation governance changed without a new accepted decision record"
    fi

    if [[ -n "$comparison_base" ]]; then
      validate_documentation_history "$comparison_base" "$head_sha"
      validate_protected_digest_rebaseline "$comparison_base" "$head_sha"
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
