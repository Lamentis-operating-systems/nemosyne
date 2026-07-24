#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repository_root/scripts/check-v1-delivery-program.py"
source_document="$repository_root/docs/specifications/v1-delivery-program.md"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT
null_device="$(
  python3 - <<'PY'
import os

print(os.devnull)
PY
)"

set_source_binding() {
  local fixture_repository="$1"
  local archive_path="$2"

  source_commit="$(git -C "$fixture_repository" rev-parse HEAD)"
  source_tree="$(git -C "$fixture_repository" rev-parse 'HEAD^{tree}')"
  GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
    git --no-replace-objects -C "$fixture_repository" \
      -c "core.attributesFile=$null_device" \
      -c tar.umask=0002 \
      archive --format=tar \
      -o "$archive_path" \
      "$source_commit" \
      -- docs/specifications docs/decisions
  archive_digest="$(
    python3 - "$archive_path" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
  )"
}

python3 "$checker" "$source_document"

expect_failure() {
  local fixture="$1"
  local expected_message="$2"
  local output="$fixture_root/output.txt"

  if python3 "$checker" "$fixture" >"$output" 2>&1; then
    printf 'expected checker failure for %s\n' "$fixture" >&2
    exit 1
  fi

  if ! grep -F "$expected_message" "$output" >/dev/null; then
    printf 'missing expected failure message: %s\n' "$expected_message" >&2
    cat "$output" >&2
    exit 1
  fi
}

expect_checker_failure() {
  local fixture_checker="$1"
  local fixture_document="$2"
  local expected_message="$3"
  local output="$fixture_root/checker-output.txt"

  if python3 "$fixture_checker" "$fixture_document" >"$output" 2>&1; then
    printf 'expected checker failure for %s\n' "$fixture_checker" >&2
    exit 1
  fi

  if ! grep -F "$expected_message" "$output" >/dev/null; then
    printf 'missing expected checker failure message: %s\n' \
      "$expected_message" >&2
    cat "$output" >&2
    exit 1
  fi
}

expect_strict_failure() {
  local fixture_repository="$1"
  local expected_message="$2"
  local output="$fixture_root/strict-output.txt"
  local fixture_document="$fixture_repository/docs/specifications/v1-delivery-program.md"
  local fixture_checker="$fixture_repository/scripts/check-v1-delivery-program.py"

  if [[ ! -f "$fixture_checker" ]]; then
    fixture_checker="$checker"
  fi

  if python3 "$fixture_checker" \
    --require-receipts \
    "$fixture_document" >"$output" 2>&1; then
    printf 'expected strict checker failure for %s\n' "$fixture_repository" >&2
    exit 1
  fi

  if ! grep -F "$expected_message" "$output" >/dev/null; then
    printf 'missing expected strict failure message: %s\n' "$expected_message" >&2
    cat "$output" >&2
    exit 1
  fi
}

structural_base="$fixture_root/structural-base"
mkdir -p \
  "$structural_base/docs/specifications" \
  "$structural_base/docs/decisions" \
  "$structural_base/scripts"
cp "$repository_root"/docs/specifications/*.md \
  "$structural_base/docs/specifications/"
cp "$source_document" \
  "$structural_base/docs/specifications/v1-delivery-program.md"
cp "$repository_root"/docs/decisions/[0-9][0-9][0-9][0-9]-*.md \
  "$structural_base/docs/decisions/"
cp "$checker" "$structural_base/scripts/check-v1-delivery-program.py"

expect_invalid_current_checker_binding() {
  local case_name="$1"
  local mutation="$2"
  local expected_message="$3"
  local case_repository="$fixture_root/current-invalid-digest-$case_name"
  local case_checker
  local case_document

  cp -R "$structural_base" "$case_repository"
  mkdir -p "$case_repository/scripts"
  case_checker="$case_repository/scripts/check-v1-delivery-program.py"
  case_document="$case_repository/docs/specifications/v1-delivery-program.md"
  cp "$checker" "$case_checker"
  case "$mutation" in
    augassign)
      printf '%s\n' \
        '' \
        'EXPECTED_PROTECTED_FINDING_SHA256 += ""' \
        >>"$case_checker"
      ;;
    nested)
      printf '%s\n' \
        '' \
        'def mutate_protected_digest():' \
        '    EXPECTED_PROTECTED_CONFORMANCE_SHA256 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' \
        >>"$case_checker"
      ;;
    dynamic)
      python3 - "$case_checker" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
marker = '\nif __name__ == "__main__":\n'
mutation = (
    '\ndef mutate_protected_digest_dynamically():\n'
    '    globals()["UNRELATED_CHECKER_STATE"] = '
    '"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"\n'
)
if text.count(marker) != 1:
    raise SystemExit("expected one checker main guard")
path.write_text(text.replace(marker, mutation + marker, 1))
PY
      ;;
    computed-namespace)
      printf '%s\n' \
        '' \
        'import sys as current_checker_sys' \
        'current_checker_namespace = current_checker_sys.modules[__name__].__dict__' \
        'current_checker_key = "EXPECTED_PROTECTED_" + "FINDING_SHA256"' \
        'current_checker_namespace[current_checker_key] = "f" * 64' \
        >>"$case_checker"
      ;;
    computed-exec)
      printf '%s\n' \
        '' \
        'import builtins as current_checker_builtins' \
        'current_checker_runner = current_checker_builtins.__dict__["ex" + "ec"]' \
        'current_checker_runner("EXPECTED_PROTECTED_FINDING_SHA256 = " + repr("f" * 64))' \
        >>"$case_checker"
      ;;
    wildcard)
      printf '%s\n' \
        '' \
        'from forged_digest_values import *' \
        >>"$case_checker"
      ;;
    g0-dynamic)
      printf '%s\n' \
        '' \
        'def mutate_current_g0_contract_dynamically():' \
        '    globals()["UNRELATED_G0_STATE"] = ()' \
        >>"$case_checker"
      ;;
    *)
      printf 'unknown current checker mutation: %s\n' "$mutation" >&2
      exit 1
      ;;
  esac
  chmod +x "$case_checker"
  expect_checker_failure "$case_checker" "$case_document" "$expected_message"
}

expect_invalid_current_checker_binding \
  augassign \
  augassign \
  'current source checker must not rebind or mutate EXPECTED_PROTECTED_FINDING_SHA256 outside its single module-level literal assignment'
expect_invalid_current_checker_binding \
  nested \
  nested \
  'current source checker must not rebind or mutate EXPECTED_PROTECTED_CONFORMANCE_SHA256 outside its single module-level literal assignment'
expect_invalid_current_checker_binding \
  dynamic \
  dynamic \
  'current source checker must not use dynamic namespace primitive globals while declaring checker contract literals'
expect_invalid_current_checker_binding \
  computed-namespace \
  computed-namespace \
  'current source checker must not mutate the current module namespace while declaring checker contract literals'
expect_invalid_current_checker_binding \
  computed-exec \
  computed-exec \
  'current source checker must not use dynamic namespace primitive exec while declaring checker contract literals'
expect_invalid_current_checker_binding \
  wildcard \
  wildcard \
  'current source checker must not use wildcard imports while declaring checker contract literals'
expect_invalid_current_checker_binding \
  g0-dynamic \
  g0-dynamic \
  'current source checker must not use dynamic namespace primitive globals while declaring checker contract literals'

append_only_repository="$fixture_root/append-only-history"
cp -R "$structural_base" "$append_only_repository"
mkdir -p "$append_only_repository/scripts"
cp "$checker" "$append_only_repository/scripts/check-v1-delivery-program.py"
git -C "$append_only_repository" init -q
git -C "$append_only_repository" config user.name "Append-only Fixture"
git -C "$append_only_repository" config \
  user.email \
  "append-only-fixture@example.invalid"
git -C "$append_only_repository" config commit.gpgsign false
git -C "$append_only_repository" add .
git -C "$append_only_repository" commit -qm 'Record append-only baseline'
append_only_base="$(git -C "$append_only_repository" rev-parse HEAD)"
python3 - \
  "$append_only_repository/docs/specifications/v1-delivery-program.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
finding_marker = "\n### CI and release flow"
finding_row = (
    "| `FND-321` / P2 | Appended finding fixture / governance | "
    "Accepted; append only | Fixture evidence |"
)
if text.count(finding_marker) != 1:
    raise SystemExit("expected one finding append boundary")
text = text.replace(finding_marker, f"\n{finding_row}\n{finding_marker}", 1)

conformance_marker = "\n### Required repository checks"
conformance_section = (
    "\n#### Manual conformance receipt `DOC-CONF-23`\n\n"
    "Appended conformance fixture.\n"
)
if text.count(conformance_marker) != 1:
    raise SystemExit("expected one conformance append boundary")
text = text.replace(
    conformance_marker,
    f"{conformance_section}{conformance_marker}",
    1,
)
path.write_text(text)
PY
git -C "$append_only_repository" add \
  docs/specifications/v1-delivery-program.md
git -C "$append_only_repository" commit -qm 'Append delivery history'
append_only_head="$(git -C "$append_only_repository" rev-parse HEAD)"
python3 \
  "$append_only_repository/scripts/check-v1-delivery-program.py" \
  --check-append-only \
  "$append_only_base" \
  "$append_only_head"

new_structural_case() {
  local name="$1"
  current_case="$fixture_root/$name"
  cp -R "$structural_base" "$current_case"
  current_document="$current_case/docs/specifications/v1-delivery-program.md"
}

mutate_current_conformance() {
  local old="$1"
  local new="$2"

  python3 - "$current_document" "$old" "$new" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
old = sys.argv[2]
new = sys.argv[3]
text = path.read_text()
start = text.index("#### Manual conformance receipt `DOC-CONF-22`")
end = text.index(
    "Any later source, count, interface, ownership, finding, review disposition,",
    start,
)
section = text[start:end]
if section.count(old) != 1:
    raise SystemExit(f"expected one current-conformance mutation target: {old}")
path.write_text(text[:start] + section.replace(old, new, 1) + text[end:])
PY
}

new_structural_case finding
sed 's#`FND-320` / P2#`FND-999` / P2#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'finding IDs must be exactly 1..320 in source order'

new_structural_case wave-label
sed 's#^| `W33` |#| `W32` |#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'wave labels must be exactly W00..W33 in source order'

new_structural_case graph-count
grep -vF \
  '    T0 --> G1' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'expected 132 canonical graph dependency edges, found 131'

new_structural_case graph-duplicate
sed 's#    T0 --> G1#    E1 --> G1#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'duplicate canonical graph dependency edge: EVD-01 -> EVD-02'

new_structural_case graph-parity
sed 's#    T0 --> G1#    T0 --> T#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'canonical V1 graph edges differ from dependency-table V1 edges'

new_structural_case graph-chain
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace(
    '    D["DOC-00"] --> E1["EVD-01"]',
    '    D["DOC-00"] --> E1["EVD-01"] --> D',
    1,
)
path.write_text(text)
PY
expect_failure "$current_document" \
  'malformed canonical graph edge: D["DOC-00"] --> E1["EVD-01"] --> D'

new_structural_case graph-trailing-token
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace("    T0 --> G1", "    T0 --> G1 trailing-token", 1)
path.write_text(text)
PY
expect_failure "$current_document" \
  'malformed canonical graph edge: T0 --> G1 trailing-token'

new_structural_case graph-post-v1-cycle
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace("    X3 --> ML1", "    ML3 --> ML1", 1)
path.write_text(text)
PY
expect_failure "$current_document" \
  'canonical dependency graph contains a cycle'

new_structural_case metadata-registry
sed \
  's#| `DOC-00` | Current docs and research#| `EVD-01` | Current docs and research#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'duplicate execution-metadata package: EVD-01'

new_structural_case metadata-order
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
lines = path.read_text().splitlines()
heading = lines.index("### Work-package interface and execution metadata")
first = next(
    index
    for index in range(heading, len(lines))
    if lines[index].startswith("| `DOC-00` |")
)
second = next(
    index
    for index in range(heading, len(lines))
    if lines[index].startswith("| `EVD-01` |")
)
lines[first], lines[second] = lines[second], lines[first]
path.write_text("\n".join(lines) + "\n")
PY
expect_failure "$current_document" \
  'execution-metadata package order must match the canonical work breakdown'

new_structural_case malformed-metadata-registry
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `DOC-00` | Current docs and research"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
line = text[line_start:line_end]
malformed = "| DOC-00 | malformed duplicate | surface | metadata | review |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed execution-metadata package ID: DOC-00'

new_structural_case responsibility-registry
sed \
  's#| `DOC-00` | Principal architect#| `EVD-01` | Principal architect#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'duplicate responsibility package: EVD-01'

new_structural_case malformed-work-breakdown
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `DOC-00` | **Re-found the V1 contracts."
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| DOC-00 | malformed duplicate | None | evidence | G0 |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed work breakdown package ID: DOC-00'

new_structural_case no-leading-pipe-work-breakdown
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `DOC-00` | **Re-found the V1 contracts."
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "DOC-00 | malformed duplicate | None | evidence | G0 |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed work breakdown table row: DOC-00 | malformed duplicate | None | evidence | G0 |'

new_structural_case work-breakdown-order
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
lines = path.read_text().splitlines()
first = next(index for index, line in enumerate(lines) if line.startswith("| `DOC-00` |"))
second = next(index for index, line in enumerate(lines) if line.startswith("| `EVD-01` |"))
lines[first], lines[second] = lines[second], lines[first]
path.write_text("\n".join(lines) + "\n")
PY
expect_failure "$current_document" \
  'work breakdown package order or table membership differs at canonical table 1'

new_structural_case unknown-dependency
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
line_start = text.index("| `EVD-01` | **Establish the evidence envelope.")
line_end = text.index("\n", line_start)
line = text[line_start:line_end]
line = line.replace("| `DOC-00` |", "| `DOC-00`, `FAKE-99` |", 1)
path.write_text(text[:line_start] + line + text[line_end:])
PY
expect_failure "$current_document" \
  'package EVD-01 references unknown dependencies: FAKE-99'

new_structural_case wave-order
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace("| `W00` | `DOC-00` |", "| `W00` | `EVD-01` |", 1)
text = text.replace("| `W01` | `EVD-01` |", "| `W01` | `DOC-00` |", 1)
path.write_text(text)
PY
expect_failure "$current_document" \
  'V1 dependencies must point from an earlier to a later wave: DOC-00 -> EVD-01'

new_structural_case malformed-wave
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `W00` | `DOC-00` |"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| W00 | `DOC-00` | malformed duplicate |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed wave registry ID: W00'

new_structural_case milestone
sed 's#^| `M6` |#| `M5` |#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'milestone labels must be exactly M0..M7 in source order'

new_structural_case malformed-milestone
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `M0` |"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| M0 | purpose | packages | evidence | stop |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed milestone registry ID: M0'

new_structural_case decision-count
rm "$current_case/docs/decisions/0030-protect-doc-00-history-and-governance.md"
expect_failure "$current_document" \
  'decision IDs must be exactly 1..31 in source order'

new_structural_case specification-count
rm "$current_case/docs/specifications/curated-activation-evidence.md"
expect_failure "$current_document" \
  'expected 12 non-template specifications, found 11'

new_structural_case decision-status
decision_file="$current_case/docs/decisions/0014-adopt-memory-grounded-predictive-attention.md"
sed 's#^Status: Accepted$#Status: Superseded#' \
  "$decision_file" >"$decision_file.tmp"
mv "$decision_file.tmp" "$decision_file"
expect_failure "$current_document" \
  'expected 27 Accepted decisions, found 26'

new_structural_case decision-supersession
old_decision="$current_case/docs/decisions/0013-adopt-a-vector-prefix-local-renderer-qualification-path.md"
new_decision="$current_case/docs/decisions/0014-adopt-memory-grounded-predictive-attention.md"
sed 's#^Status: Superseded$#Status: Accepted#' \
  "$old_decision" >"$old_decision.tmp"
mv "$old_decision.tmp" "$old_decision"
sed 's#^Status: Accepted$#Status: Superseded#' \
  "$new_decision" >"$new_decision.tmp"
mv "$new_decision.tmp" "$new_decision"
expect_failure "$current_document" \
  'Superseded decisions must be exactly 0011, 0012, 0013, and 0028'

new_structural_case proof-definition
proof_file="$current_case/docs/specifications/v1-proof-program.md"
sed \
  's#`PROOF-G1-CONDITIONS-001` is the sole normative owner#`PROOF-G1-CONDITIONS-001` is a normative owner#' \
  "$proof_file" >"$proof_file.tmp"
mv "$proof_file.tmp" "$proof_file"
expect_failure "$current_document" \
  'proof anchor PROOF-G1-CONDITIONS-001 must be defined exactly once, found 0'

new_structural_case proof-reference
sed \
  's#`PROOF-G1-HEADROOM-001`#`PROOF-G1-HEADROOM-999`#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'delivery ownership matrix must reference PROOF-G1-HEADROOM-001 exactly once, found 0'

new_structural_case malformed-interface-registry
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `IF-EVIDENCE-ENVELOPE` |"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| IF-EVIDENCE-ENVELOPE | producer | consumers | contract | evidence |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed interface registry ID: IF-EVIDENCE-ENVELOPE'

new_structural_case interface-order
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
lines = path.read_text().splitlines()
first = next(
    index for index, line in enumerate(lines)
    if line.startswith("| `IF-EVIDENCE-ENVELOPE` |")
)
second = next(
    index for index, line in enumerate(lines)
    if line.startswith("| `IF-G1-ENVELOPE` |")
)
lines[first], lines[second] = lines[second], lines[first]
path.write_text("\n".join(lines) + "\n")
PY
expect_failure "$current_document" \
  'interface registry IDs and source order must match the canonical 49-interface inventory'

new_structural_case review-target
sed \
  's#../receipts/reviews/rev-18.md#https://invalid.example/receipts/reviews/rev-18.md#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'review registry must contain exactly the canonical receipt target ../receipts/reviews/rev-18.md'

new_structural_case review-extra-target
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "../receipts/reviews/rev-18.md)"
text = text.replace(needle, needle + " [extra](../receipts/reviews/rev-17.md)", 1)
path.write_text(text)
PY
expect_failure "$current_document" \
  'review registry must contain exactly the canonical receipt target ../receipts/reviews/rev-18.md'

new_structural_case malformed-review-link
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace(
    "[external content-bound record](../receipts/reviews/rev-18.md)",
    "external content-bound record](../receipts/reviews/rev-18.md)",
    1,
)
path.write_text(text)
PY
expect_failure "$current_document" \
  'review registry must contain exactly the canonical receipt target ../receipts/reviews/rev-18.md'

new_structural_case wrong-column-review-link
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
target = "../receipts/reviews/rev-18.md"
link = f"[external content-bound record]({target})"
line_start = text.index("| `REV-18` |")
line_end = text.index("\n", line_start)
line = text[line_start:line_end]
line = line.replace(link, "external content-bound record", 1)
line = line[:-1] + f" {link} |"
path.write_text(text[:line_start] + line + text[line_end:])
PY
expect_failure "$current_document" \
  'review registry must contain exactly the canonical receipt target ../receipts/reviews/rev-18.md'

new_structural_case malformed-review-registry
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `REV-01` |"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| REV-01 | perspective | status | evidence |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed review registry ID: REV-01'

new_structural_case consolidation-target
sed \
  's#](../receipts/consolidations/consol-03.md)#](https://invalid.example/receipts/consolidations/consol-03.md)#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'consolidation registry must contain exactly the canonical consolidation receipt target ../receipts/consolidations/consol-03.md'

new_structural_case malformed-consolidation-link
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace(
    "[`docs/receipts/consolidations/consol-03.md`](../receipts/consolidations/consol-03.md)",
    "`docs/receipts/consolidations/consol-03.md`](../receipts/consolidations/consol-03.md)",
    1,
)
path.write_text(text)
PY
expect_failure "$current_document" \
  'consolidation registry must contain exactly the canonical consolidation receipt target ../receipts/consolidations/consol-03.md'

new_structural_case malformed-consolidation-registry
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `CONSOL-01` / Content-bound attestation |"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| CONSOL-01 / Content-bound attestation | scope | evidence | disposition |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed consolidation registry ID: CONSOL-01 / Content-bound attestation'

new_structural_case stale-consolidation-range
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
line_start = text.index("| `CONSOL-03` / Content-bound attestation |")
line_end = text.index("\n", line_start)
line = text[line_start:line_end].replace("`FND-152..320`", "`FND-152..319`", 1)
path.write_text(text[:line_start] + line + text[line_end:])
PY
expect_failure "$current_document" \
  'consolidation registry must bind the canonical current finding range FND-152..320 exactly once'

new_structural_case stale-current-conformance-range
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text().replace(
    "`FND-152..320`. `FND-001..151`",
    "`FND-152..319`. `FND-001..151`",
    1,
)
path.write_text(text)
PY
expect_failure "$current_document" \
  'active DOC-CONF-22 must contain canonical current-state fragment exactly once: `FND-152..320`'

new_structural_case protected-history-crlf-checkout
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
source = path.read_bytes()
if b"\r" in source:
    raise SystemExit("expected canonical LF source fixture")
path.write_bytes(source.replace(b"\n", b"\r\n"))
PY
python3 "$checker" "$current_document"

new_structural_case protected-conformance-history
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
start = text.index("#### Manual conformance receipt `DOC-CONF-10`")
end = text.index("#### Manual conformance receipt `DOC-CONF-11`", start)
section = text[start:end]
if section.count("G9 protocol and gates") != 1:
    raise SystemExit("expected one protected DOC-CONF-10 mutation target")
path.write_text(
    text[:start]
    + section.replace("G9 protocol and gates", "G9 protocol and gate", 1)
    + text[end:]
)
PY
expect_failure "$current_document" \
  'protected DOC-CONF-01..21 history differs from its canonical byte digest'

new_structural_case protected-finding-history
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
start = text.index("| `FND-151` / P1 |")
end = text.index("\n", start)
row = text[start:end]
if row.count("local notation") != 1:
    raise SystemExit("expected one protected FND-151 mutation target")
path.write_text(
    text[:start]
    + row.replace("local notation", "local notations", 1)
    + text[end:]
)
PY
expect_failure "$current_document" \
  'protected FND-001..151 ledger differs from its canonical byte digest'

new_structural_case stale-current-interface-count
sed \
  's#The active registry retains 49 unique interfaces#The active registry retains 48 unique interfaces#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-finding-count
sed \
  's#There are 320 unique sequential findings#There are 319 unique sequential findings#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-package-counts
mutate_current_conformance \
  'The program remains 54 unique packages, 48 V1 and 6 post-V1.' \
  'The program remains 53 unique packages, 47 V1 and seven post-V1.'
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-dependency-counts
mutate_current_conformance \
  'The canonical dependency table has 127 total and 123 V1-to-V1 relations' \
  'The canonical dependency table has 126 total and 122 V1-to-V1 relations'
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-graph-counts
mutate_current_conformance \
  'the Mermaid graph has 132 total and 123 V1-to-V1 edges' \
  'the Mermaid graph has 131 total and 122 V1-to-V1 edges'
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-wave-count
mutate_current_conformance \
  '49 unique interfaces and 34 stable wave labels' \
  '49 unique interfaces and 33 stable wave labels'
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-evidence-counts
mutate_current_conformance \
  '22 append-only conformance receipts, 18 external review paths, 12 non-template specifications' \
  '21 append-only conformance receipts, 17 external review paths, 11 non-template specifications'
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case stale-current-decision-counts
mutate_current_conformance \
  '31 numbered decisions: 27 `Accepted` and 4 `Superseded`' \
  '30 numbered decisions: 26 `Accepted` and 5 `Superseded`'
expect_failure "$current_document" \
  'active DOC-CONF-22 complete structural inventory differs from the canonical inventory'

new_structural_case malformed-finding-ledger
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = "| `FND-153` / P1 |"
line_start = text.index(needle)
line_end = text.index("\n", line_start)
malformed = "| FND-153 / P1 | finding | disposition | evidence |"
path.write_text(text[:line_end] + "\n" + malformed + text[line_end:])
PY
expect_failure "$current_document" \
  'malformed finding ledger ID: FND-153 / P1'

new_structural_case finding-severity
sed 's#`FND-153` / P1#`FND-153` / P2#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'finding severities must match the canonical FND-001..FND-320 ledger'

new_structural_case completion-target
sed \
  's#](../receipts/doc-00-g0.md)#](https://invalid.example/receipts/doc-00-g0.md)#' \
  "$current_document" >"$current_document.tmp"
mv "$current_document.tmp" "$current_document"
expect_failure "$current_document" \
  'delivery program must contain exactly the canonical DOC-00 completion target ../receipts/doc-00-g0.md'

new_structural_case malformed-completion-link
python3 - "$current_document" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace(
    "[`docs/receipts/doc-00-g0.md`](../receipts/doc-00-g0.md)",
    "`docs/receipts/doc-00-g0.md`](../receipts/doc-00-g0.md)",
    1,
)
path.write_text(text)
PY
expect_failure "$current_document" \
  'delivery program must contain exactly the canonical DOC-00 completion target ../receipts/doc-00-g0.md'

strict_repository="$fixture_root/strict-repository"
cp -R "$structural_base" "$strict_repository"
mkdir -p \
  "$strict_repository/docs/receipts/consolidations" \
  "$strict_repository/docs/receipts/reviews" \
  "$strict_repository/scripts"
cp "$repository_root/docs/receipts/README.md" \
  "$strict_repository/docs/receipts/README.md"
cp "$checker" "$strict_repository/scripts/check-v1-delivery-program.py"
cp \
  "$repository_root/scripts/test-v1-delivery-program-check.sh" \
  "$strict_repository/scripts/test-v1-delivery-program-check.sh"

git -C "$strict_repository" init -q
git -C "$strict_repository" config user.name "DOC-00 Fixture"
git -C "$strict_repository" config user.email "doc00-fixture@example.invalid"
git -C "$strict_repository" config commit.gpgsign false
git -C "$strict_repository" add \
  docs/specifications \
  docs/decisions \
  docs/receipts/README.md \
  scripts
GIT_AUTHOR_DATE='2026-07-24T10:00:00Z' \
GIT_COMMITTER_DATE='2026-07-24T10:00:00Z' \
  git -C "$strict_repository" commit -qm 'Freeze reviewed source'

source_commit="$(git -C "$strict_repository" rev-parse HEAD)"
source_tree="$(git -C "$strict_repository" rev-parse 'HEAD^{tree}')"
archive_path="$fixture_root/strict-source.tar"
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
  git --no-replace-objects -C "$strict_repository" \
    -c "core.attributesFile=$null_device" \
    -c tar.umask=0002 \
    archive --format=tar \
    -o "$archive_path" \
    "$source_commit" \
    -- docs/specifications docs/decisions
archive_digest="$(
  python3 - "$archive_path" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
strict_source_commit="$source_commit"
strict_source_tree="$source_tree"
strict_archive_digest="$archive_digest"

write_receipt() {
  local path="$1"
  local record_id="$2"
  local kind="$3"
  local status="$4"
  local actor="$5"
  local declaration="$6"
  local method="$7"
  local evidence="$8"
  local replaces="${9:-None}"

  {
    printf '# %s\n\n' "$record_id"
    printf '| Field | Value |\n'
    printf '| --- | --- |\n'
    printf '| Schema | `%s` |\n' 'doc00-attestation-v1'
    printf '| Record ID | `%s` |\n' "$record_id"
    printf '| Kind | %s |\n' "$kind"
    printf '| Status | `%s` |\n' "$status"
    printf '| Actor | `%s` |\n' "$actor"
    printf '| Declaration | %s |\n' "$declaration"
    printf '| Completed at | `%s` |\n' '2026-07-24T12:00:00Z'
    printf '| Source commit | `%s` |\n' "$source_commit"
    printf '| Source tree | `%s` |\n' "$source_tree"
    printf '| Included paths | `%s` |\n' 'docs/specifications, docs/decisions'
    printf '| Archive algorithm | `%s` |\n' 'git-archive-tar-sha256-v1'
    printf '| Archive SHA-256 | `%s` |\n' "$archive_digest"
    printf '| Method | %s |\n' "$method"
    printf '| Findings | `None` |\n'
    printf '| Disposition | `Pass` |\n'
    printf '| Residual limits | Documentation evidence only. |\n'
    printf '| Evidence references | %s |\n' "$evidence"
    printf '| Replaces | `%s` |\n' "$replaces"
  } >"$path"
}

g0_evidence=''
for index in 1 2 3; do
  formatted_index="$(printf '%02d' "$index")"
  g0_evidence="${g0_evidence}[CONSOL-${formatted_index}](consolidations/consol-${formatted_index}.md); "
done
for index in $(seq 1 18); do
  formatted_index="$(printf '%02d' "$index")"
  g0_evidence="${g0_evidence}[REV-${formatted_index}](reviews/rev-${formatted_index}.md); "
done
append_g0_check() {
  g0_evidence="${g0_evidence}\`$1\`; "
}
append_g0_check './scripts/test-documentation-change-policy.sh'
append_g0_check './scripts/test-documentation-check.sh'
append_g0_check './scripts/check-documentation.sh'
append_g0_check './scripts/test-v1-delivery-program-check.sh'
append_g0_check './scripts/check-v1-delivery-program.py --require-receipts'
append_g0_check 'cargo fmt --all --check'
append_g0_check \
  'cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -F missing-docs -F unsafe-code'
append_g0_check \
  'RUSTDOCFLAGS="-D warnings -F missing-docs -F unsafe-code" cargo doc --workspace --all-features --no-deps --locked'
append_g0_check 'cargo test --workspace --all-features --locked'
append_g0_check 'git diff --check'
append_g0_check \
  'DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /tmp/doc-00-pull-request-body.md'
g0_evidence="${g0_evidence%??}."

write_receipt_set() {
  local fixture_repository="$1"
  local prior_digest="${2:-}"
  local replaces='None'

  mkdir -p \
    "$fixture_repository/docs/receipts/consolidations" \
    "$fixture_repository/docs/receipts/reviews"

  if [[ -n "$prior_digest" ]]; then
    replaces="DOC-CONF-22 at archive digest $prior_digest"
  fi
  write_receipt \
    "$fixture_repository/docs/receipts/doc-00-g0.md" \
    'DOC-CONF-22' \
    'MergeAuthorization' \
    'MergeAuthorized' \
    'Codex /root' \
    'Principal integrator for DOC-00 merge authorization; not the accountable human or an independent reviewer.' \
    'Conformance and repository-check reconciliation.' \
    "$g0_evidence" \
    "$replaces"

  for index in 1 2 3; do
    replaces='None'
    if [[ -n "$prior_digest" ]]; then
      replaces="CONSOL-0${index} at archive digest $prior_digest"
    fi
    write_receipt \
      "$fixture_repository/docs/receipts/consolidations/consol-0${index}.md" \
      "CONSOL-0${index}" \
      'Consolidation' \
      'Pass' \
      "consolidator-0${index}" \
      'Integration owner for the named consolidation pass.' \
      'Independent consolidation pass.' \
      'FND-152..320 reconciliation.' \
      "$replaces"
  done

  for index in $(seq 1 18); do
    formatted_index="$(printf '%02d' "$index")"
    replaces='None'
    if [[ -n "$prior_digest" ]]; then
      replaces="REV-${formatted_index} at archive digest $prior_digest"
    fi
    write_receipt \
      "$fixture_repository/docs/receipts/reviews/rev-${formatted_index}.md" \
      "REV-${formatted_index}" \
      'Review' \
      'Pass' \
      "reviewer-${formatted_index}" \
      'Independent reviewer; did not author or remediate the reviewed source.' \
      'Independent registered-perspective review.' \
      "REV-${formatted_index} review evidence." \
      "$replaces"
  done
}

append_replacement_pair() {
  local fixture_repository="$1"
  local prior_digest="$2"
  local fixture_name="$3"

  printf '\nFixture source freeze: %s.\n' "$fixture_name" \
    >>"$fixture_repository/docs/specifications/v1-proof-program.md"
  git -C "$fixture_repository" add \
    docs/specifications/v1-proof-program.md
  GIT_AUTHOR_DATE='2026-07-24T12:10:00Z' \
  GIT_COMMITTER_DATE='2026-07-24T12:10:00Z' \
    git -C "$fixture_repository" commit -qm 'Freeze replacement fixture source'
  set_source_binding \
    "$fixture_repository" \
    "$fixture_root/$fixture_name-source.tar"
  write_receipt_set "$fixture_repository" "$prior_digest"
  git -C "$fixture_repository" add docs/receipts
  GIT_AUTHOR_DATE='2026-07-24T12:20:00Z' \
  GIT_COMMITTER_DATE='2026-07-24T12:20:00Z' \
    git -C "$fixture_repository" commit -qm 'Record replacement fixture evidence'
}

write_receipt_set "$strict_repository"

git -C "$strict_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T12:30:00Z' \
GIT_COMMITTER_DATE='2026-07-24T12:30:00Z' \
  git -C "$strict_repository" commit -qm 'Record DOC-00 attestations'
git -C "$strict_repository" config tar.umask 0077

python3 "$strict_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$strict_repository/docs/specifications/v1-delivery-program.md"

GIT_CONFIG_COUNT=1 \
GIT_CONFIG_KEY_0=core.worktree \
GIT_CONFIG_VALUE_0="$fixture_root" \
GIT_DIR="$fixture_root/nonexistent-git-dir" \
GIT_OBJECT_DIRECTORY="$fixture_root/nonexistent-object-directory" \
GIT_SHALLOW_FILE="$fixture_root/nonexistent-shallow-file" \
GIT_WORK_TREE="$fixture_root" \
  python3 "$strict_repository/scripts/check-v1-delivery-program.py" \
    --require-receipts \
    "$strict_repository/docs/specifications/v1-delivery-program.md"

grafted_repository="$fixture_root/grafted-repository"
cp -R "$strict_repository" "$grafted_repository"
git -C "$grafted_repository" rev-parse HEAD \
  >"$grafted_repository/.git/info/grafts"
expect_strict_failure "$grafted_repository" \
  'Git info/grafts must be absent or empty'

shallow_repository="$fixture_root/shallow-repository"
git clone -q \
  --depth=1 \
  "file://$strict_repository" \
  "$shallow_repository"
expect_strict_failure "$shallow_repository" \
  'strict receipt validation requires complete non-shallow Git history'

nonexec_source_repository="$fixture_root/nonexec-source"
cp -R "$structural_base" "$nonexec_source_repository"
mkdir -p \
  "$nonexec_source_repository/docs/receipts" \
  "$nonexec_source_repository/scripts"
cp "$repository_root/docs/receipts/README.md" \
  "$nonexec_source_repository/docs/receipts/README.md"
cp "$checker" "$nonexec_source_repository/scripts/check-v1-delivery-program.py"
cp "$repository_root/scripts/test-v1-delivery-program-check.sh" \
  "$nonexec_source_repository/scripts/test-v1-delivery-program-check.sh"
git -C "$nonexec_source_repository" init -q
git -C "$nonexec_source_repository" config user.name "DOC-00 Mode Fixture"
git -C "$nonexec_source_repository" config \
  user.email \
  "doc00-mode-fixture@example.invalid"
git -C "$nonexec_source_repository" config commit.gpgsign false
git -C "$nonexec_source_repository" add \
  docs/specifications \
  docs/decisions \
  docs/receipts/README.md \
  scripts
git -C "$nonexec_source_repository" update-index \
  --chmod=-x \
  scripts/check-v1-delivery-program.py
GIT_AUTHOR_DATE='2026-07-24T12:35:00Z' \
GIT_COMMITTER_DATE='2026-07-24T12:35:00Z' \
  git -C "$nonexec_source_repository" commit -qm 'Freeze non-executable checker'
source_commit="$(git -C "$nonexec_source_repository" rev-parse HEAD)"
source_tree="$(git -C "$nonexec_source_repository" rev-parse 'HEAD^{tree}')"
nonexec_archive_path="$fixture_root/nonexec-source.tar"
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
  git --no-replace-objects -C "$nonexec_source_repository" \
    -c "core.attributesFile=$null_device" \
    -c tar.umask=0002 \
    archive --format=tar \
    -o "$nonexec_archive_path" \
    "$source_commit" \
    -- docs/specifications docs/decisions
archive_digest="$(
  python3 - "$nonexec_archive_path" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
write_receipt_set "$nonexec_source_repository"
git -C "$nonexec_source_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T12:40:00Z' \
GIT_COMMITTER_DATE='2026-07-24T12:40:00Z' \
  git -C "$nonexec_source_repository" commit -qm 'Bind non-executable source'
expect_strict_failure "$nonexec_source_repository" \
  'attested governance program must be an executable regular file: scripts/check-v1-delivery-program.py'
source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"

new_strict_case() {
  local name="$1"
  strict_case="$fixture_root/strict-$name"
  cp -R "$strict_repository" "$strict_case"
}

new_strict_case missing-receipt
rm "$strict_case/docs/receipts/reviews/rev-18.md"
expect_strict_failure "$strict_case" \
  "canonical attestation file set differs from the README contract; missing: ['docs/receipts/reviews/rev-18.md']"

new_strict_case symlink-receipt
rm "$strict_case/docs/receipts/reviews/rev-18.md"
ln -s rev-17.md "$strict_case/docs/receipts/reviews/rev-18.md"
expect_strict_failure "$strict_case" \
  'missing or non-regular attestation REV-18'

new_strict_case extra-receipt
cp \
  "$strict_case/docs/receipts/reviews/rev-18.md" \
  "$strict_case/docs/receipts/reviews/rev-19.md"
expect_strict_failure "$strict_case" \
  "canonical attestation file set differs from the README contract; missing: []; extra: ['docs/receipts/reviews/rev-19.md']"

new_strict_case readme-strict-value
receipt_readme="$strict_case/docs/receipts/README.md"
sed 's#`MergeAuthorization`#`Completion`#' \
  "$receipt_readme" >"$receipt_readme.tmp"
mv "$receipt_readme.tmp" "$receipt_readme"
expect_strict_failure "$strict_case" \
  'DOC-00 receipt README is missing canonical strict value: `MergeAuthorization`'

new_strict_case readme-history-contract
receipt_readme="$strict_case/docs/receipts/README.md"
sed \
  's#every additional AST binding is#an additional AST binding may be#' \
  "$receipt_readme" >"$receipt_readme.tmp"
mv "$receipt_readme.tmp" "$receipt_readme"
expect_strict_failure "$strict_case" \
  'DOC-00 receipt README is missing canonical strict value: The replacement source-freeze commit'

new_strict_case readme-archive-command
receipt_readme="$strict_case/docs/receipts/README.md"
sed 's#git --no-replace-objects#git#' \
  "$receipt_readme" >"$receipt_readme.tmp"
mv "$receipt_readme.tmp" "$receipt_readme"
expect_strict_failure "$strict_case" \
  'DOC-00 receipt README is missing canonical strict value: GIT_NO_REPLACE_OBJECTS=1'

new_strict_case untracked-receipt
git -C "$strict_case" rm --cached -q docs/receipts/reviews/rev-18.md
expect_strict_failure "$strict_case" \
  'attestation is not tracked at its canonical path'

new_strict_case field-order
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed 's#| Method |#| Actor |#' "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation REV-18 fields are missing, extra, duplicated, or out of order'

new_strict_case trailing-malformed-attestation-row
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
printf '%s\n' '| Extra | value' >>"$receipt"
expect_strict_failure "$strict_case" \
  'attestation REV-18 must contain only its canonical heading and field table'

new_strict_case record-id
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed 's#| Record ID | `REV-18` |#| Record ID | `REV-17` |#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation path expects Record ID REV-18, found REV-17'

new_strict_case wrong-kind
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed 's#| Kind | Review |#| Kind | Consolidation |#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation REV-18 Kind must be Review'

new_strict_case incomplete-g0
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed 's#| Status | `MergeAuthorized` |#| Status | `In progress` |#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 Status must be MergeAuthorized'

new_strict_case completion-declaration
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#Principal integrator for DOC-00 merge authorization; not the accountable human or an independent reviewer.#Observed DOC-00 merge authorization.#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 lacks the canonical merge-authorization declaration'

new_strict_case completion-actor
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed 's#| Actor | `Codex /root` |#| Actor | `principal-architect` |#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 Actor must be Codex /root'

new_strict_case positive-review-declaration
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed \
  's#Independent reviewer; did not author or remediate the reviewed source.#I authored and remediated the reviewed source.#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation REV-18 lacks an independence declaration'

new_strict_case vague-consolidation-declaration
receipt="$strict_case/docs/receipts/consolidations/consol-03.md"
sed \
  's#Integration owner for the named consolidation pass.#Participated in consolidation.#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation CONSOL-03 lacks an ownership declaration'

new_strict_case stale-consolidation-evidence-range
receipt="$strict_case/docs/receipts/consolidations/consol-03.md"
sed \
  's#FND-152..320 reconciliation.#FND-152..319 reconciliation.#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation CONSOL-03 Evidence references must be FND-152..320 reconciliation.'

new_strict_case duplicate-review-actor
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed 's#| Actor | `reviewer-18` |#| Actor | `reviewer-17` |#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'duplicate review Actor: reviewer-17'

new_strict_case missing-g0-record-reference
receipt="$strict_case/docs/receipts/doc-00-g0.md"
python3 - "$receipt" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace("[REV-18](reviews/rev-18.md)", "REV-18", 1)
path.write_text(text)
PY
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference [REV-18](reviews/rev-18.md) exactly once'

new_strict_case extra-g0-record-reference
receipt="$strict_case/docs/receipts/doc-00-g0.md"
python3 - "$receipt" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
text = text.replace(
    "; `./scripts/test-documentation-change-policy.sh`",
    "; [REV-19](reviews/rev-19.md); `./scripts/test-documentation-change-policy.sh`",
    1,
)
path.write_text(text)
PY
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must contain only the 21 canonical sub-attestation links in registry order'

new_strict_case missing-g0-check-reference
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#git diff --check#git diff check omitted#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference repository check git diff --check exactly once'

new_strict_case missing-change-aware-g0-reference
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /tmp/doc-00-pull-request-body.md#./scripts/check-documentation.sh /tmp/doc-00-pull-request-body.md#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference exactly one change-aware repository check `DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /absolute/pr-body-path` without shell operators'

new_strict_case missing-change-aware-body-path
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /tmp/doc-00-pull-request-body.md#DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference exactly one change-aware repository check `DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /absolute/pr-body-path` without shell operators'

new_strict_case change-aware-shell-operator
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#/tmp/doc-00-pull-request-body.md#/tmp/doc-00-pull-request-body.md;touch#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference exactly one change-aware repository check `DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /absolute/pr-body-path` without shell operators'

new_strict_case change-aware-redirection
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#/tmp/doc-00-pull-request-body.md#/tmp/doc-00-pull-request-body.md>output#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference exactly one change-aware repository check `DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /absolute/pr-body-path` without shell operators'

new_strict_case change-aware-command-substitution
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#/tmp/doc-00-pull-request-body.md#/tmp/$(touch)-pull-request-body.md#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference exactly one change-aware repository check `DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /absolute/pr-body-path` without shell operators'

new_strict_case truncated-g0-clippy-reference
receipt="$strict_case/docs/receipts/doc-00-g0.md"
sed \
  's#cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -F missing-docs -F unsafe-code#cargo clippy --workspace --all-targets --all-features --locked#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation DOC-CONF-22 must reference repository check cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -F missing-docs -F unsafe-code exactly once'

new_strict_case binding-drift
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed "s#| Archive SHA-256 | \`$archive_digest\` |#| Archive SHA-256 | \`$(printf '0%.0s' $(seq 1 64))\` |#" \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'attestation REV-18 disagrees on common Archive SHA-256'

new_strict_case tree-mismatch
python3 - "$strict_case/docs/receipts" <<'PY'
from pathlib import Path
import sys

for path in Path(sys.argv[1]).rglob("*.md"):
    if path.name == "README.md":
        continue
    text = path.read_text()
    text = text.replace(
        next(line for line in text.splitlines() if line.startswith("| Source tree |")),
        "| Source tree | `" + ("0" * 40) + "` |",
    )
    path.write_text(text)
PY
expect_strict_failure "$strict_case" \
  'attested Source tree does not match Source commit'

new_strict_case digest-mismatch
python3 - "$strict_case/docs/receipts" <<'PY'
from pathlib import Path
import sys

for path in Path(sys.argv[1]).rglob("*.md"):
    if path.name == "README.md":
        continue
    text = path.read_text()
    text = text.replace(
        next(
            line
            for line in text.splitlines()
            if line.startswith("| Archive SHA-256 |")
        ),
        "| Archive SHA-256 | `" + ("0" * 64) + "` |",
    )
    path.write_text(text)
PY
expect_strict_failure "$strict_case" \
  'attested Archive SHA-256 does not match the reconstructed Git archive'

new_strict_case stale-source
printf '\n' >>"$strict_case/docs/specifications/v1-delivery-program.md"
git -C "$strict_case" add docs/specifications/v1-delivery-program.md
GIT_AUTHOR_DATE='2026-07-24T13:00:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:00:00Z' \
  git -C "$strict_case" commit -qm 'Change reviewed source'
expect_strict_failure "$strict_case" \
  'attested source differs from HEAD at docs/specifications'

new_strict_case dirty-source
printf '\n' >>"$strict_case/docs/specifications/v1-delivery-program.md"
expect_strict_failure "$strict_case" \
  'attested included paths have uncommitted changes'

new_strict_case dirty-governance-program
printf '\n' >>"$strict_case/scripts/check-v1-delivery-program.py"
expect_strict_failure "$strict_case" \
  'attested governance programs have uncommitted changes'

new_strict_case governance-mode-drift
git -C "$strict_case" update-index \
  --chmod=-x \
  scripts/check-v1-delivery-program.py
GIT_AUTHOR_DATE='2026-07-24T13:10:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:10:00Z' \
  git -C "$strict_case" commit -qm 'Change governance checker mode'
expect_strict_failure "$strict_case" \
  'governance program differs from Source commit: scripts/check-v1-delivery-program.py'

new_strict_case external-strict-checker
output="$fixture_root/external-checker-output.txt"
if python3 "$checker" \
  --require-receipts \
  "$strict_case/docs/specifications/v1-delivery-program.md" \
  >"$output" 2>&1; then
  printf 'expected external strict checker rejection\n' >&2
  exit 1
fi
if ! grep -F \
  "strict receipt validation must execute the repository's canonical scripts/check-v1-delivery-program.py" \
  "$output" >/dev/null; then
  cat "$output" >&2
  exit 1
fi

new_strict_case info-attributes
printf '%s\n' 'docs/** export-ignore' >"$strict_case/.git/info/attributes"
expect_strict_failure "$strict_case" \
  'Git info/attributes must be absent or empty'

new_strict_case dirty-receipt
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed \
  's#Documentation evidence only.#Documentation evidence only; local note.#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'canonical DOC-00 attestations have uncommitted changes'

new_strict_case first-record-replaces
receipt="$strict_case/docs/receipts/reviews/rev-18.md"
sed \
  "s#| Replaces | \`None\` |#| Replaces | \`REV-18 at archive digest $archive_digest\` |#" \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$strict_case" \
  'first attestation REV-18 must use Replaces: None'

new_strict_case extra-evidence-path
git -C "$strict_case" reset --soft "$source_commit"
printf '%s\n' 'not canonical evidence' >"$strict_case/extra-evidence.txt"
git -C "$strict_case" add docs/receipts extra-evidence.txt
GIT_AUTHOR_DATE='2026-07-24T13:15:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:15:00Z' \
  git -C "$strict_case" commit -qm 'Add receipts and unrelated evidence'
expect_strict_failure "$strict_case" \
  "changes after the attested Source commit must be exactly the 22 canonical attestation files; missing: []; extra: ['extra-evidence.txt']"

new_strict_case reverted-intermediate-commit
printf '%s\n' 'temporary' >"$strict_case/temporary-evidence.txt"
git -C "$strict_case" add temporary-evidence.txt
GIT_AUTHOR_DATE='2026-07-24T13:20:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:20:00Z' \
  git -C "$strict_case" commit -qm 'Add temporary intermediate evidence'
git -C "$strict_case" rm -q temporary-evidence.txt
GIT_AUTHOR_DATE='2026-07-24T13:25:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:25:00Z' \
  git -C "$strict_case" commit -qm 'Remove temporary intermediate evidence'
expect_strict_failure "$strict_case" \
  'HEAD after the evidence commit must descend from a two-parent merge commit whose second parent is the exact evidence commit'

new_strict_case intermediate-before-evidence
original_evidence_commit="$(git -C "$strict_case" rev-parse HEAD)"
git -C "$strict_case" reset --hard -q "$source_commit"
printf '%s\n' 'temporary' >"$strict_case/temporary-evidence.txt"
git -C "$strict_case" add temporary-evidence.txt
GIT_AUTHOR_DATE='2026-07-24T13:30:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:30:00Z' \
  git -C "$strict_case" commit -qm 'Insert intermediate commit'
git -C "$strict_case" rm -q temporary-evidence.txt
git -C "$strict_case" checkout "$original_evidence_commit" -- \
  docs/receipts/doc-00-g0.md \
  docs/receipts/consolidations \
  docs/receipts/reviews
git -C "$strict_case" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T13:35:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:35:00Z' \
  git -C "$strict_case" commit -qm 'Record evidence after an intermediate commit'
expect_strict_failure "$strict_case" \
  'the canonical attestation evidence commit must have exactly the attested Source commit as its parent'

merge_repository="$fixture_root/merge-repository"
cp -R "$strict_repository" "$merge_repository"
evidence_commit="$(git -C "$merge_repository" rev-parse HEAD)"
git -C "$merge_repository" checkout -q -b merge-main "$source_commit"
printf '%s\n' 'unrelated main history' >"$merge_repository/main-history.txt"
git -C "$merge_repository" add main-history.txt
GIT_AUTHOR_DATE='2026-07-24T13:40:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:40:00Z' \
  git -C "$merge_repository" commit -qm 'Advance main independently'
GIT_AUTHOR_DATE='2026-07-24T13:45:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:45:00Z' \
  git -C "$merge_repository" merge \
    --no-ff \
    -q \
    -m 'Merge DOC-00 evidence' \
    "$evidence_commit"
python3 "$merge_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$merge_repository/docs/specifications/v1-delivery-program.md"

printf '%s\n' 'unrelated post-merge history' \
  >"$merge_repository/post-merge-history.txt"
git -C "$merge_repository" add post-merge-history.txt
GIT_AUTHOR_DATE='2026-07-24T13:50:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:50:00Z' \
  git -C "$merge_repository" commit -qm 'Advance after preserving merge'
python3 "$merge_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$merge_repository/docs/specifications/v1-delivery-program.md"

post_merge_revert_repository="$fixture_root/post-merge-reverted-bound-path"
cp -R "$merge_repository" "$post_merge_revert_repository"
printf '\nTemporary post-merge change.\n' \
  >>"$post_merge_revert_repository/docs/specifications/v1-proof-program.md"
git -C "$post_merge_revert_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T13:55:00Z' \
GIT_COMMITTER_DATE='2026-07-24T13:55:00Z' \
  git -C "$post_merge_revert_repository" commit -qm 'Temporarily change bound source'
git -C "$post_merge_revert_repository" checkout HEAD~1 -- \
  docs/specifications/v1-proof-program.md
git -C "$post_merge_revert_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:00:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:00:00Z' \
  git -C "$post_merge_revert_repository" commit -qm 'Revert bound source change'
expect_strict_failure "$post_merge_revert_repository" \
  'history after the preserving merge must not modify a DOC-00-bound path'

post_merge_side_repository="$fixture_root/post-merge-side-branch-change"
cp -R "$merge_repository" "$post_merge_side_repository"
git -C "$post_merge_side_repository" checkout -q -b bound-side-change
printf '\nSide-branch-only bound change.\n' \
  >>"$post_merge_side_repository/docs/specifications/v1-proof-program.md"
git -C "$post_merge_side_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:02:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:02:00Z' \
  git -C "$post_merge_side_repository" commit -qm 'Change bound source on side branch'
git -C "$post_merge_side_repository" checkout -q merge-main
GIT_AUTHOR_DATE='2026-07-24T14:04:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:04:00Z' \
  git -C "$post_merge_side_repository" merge \
    --no-ff \
    --no-commit \
    bound-side-change
git -C "$post_merge_side_repository" checkout HEAD -- \
  docs/specifications/v1-proof-program.md
git -C "$post_merge_side_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:05:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:05:00Z' \
  git -C "$post_merge_side_repository" commit -qm 'Resolve bound source to original'
expect_strict_failure "$post_merge_side_repository" \
  'history after the preserving merge must not modify a DOC-00-bound path'

pre_merge_side_repository="$fixture_root/pre-merge-side-branch-change"
cp -R "$merge_repository" "$pre_merge_side_repository"
git -C "$pre_merge_side_repository" checkout -q -b pre-doc-bound-change \
  "$source_commit"
printf '\nHistorical side-branch change.\n' \
  >>"$pre_merge_side_repository/docs/specifications/v1-proof-program.md"
git -C "$pre_merge_side_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T09:00:00Z' \
GIT_COMMITTER_DATE='2026-07-24T09:00:00Z' \
  git -C "$pre_merge_side_repository" commit -qm 'Historical bound side change'
git -C "$pre_merge_side_repository" checkout -q merge-main
GIT_AUTHOR_DATE='2026-07-24T14:07:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:07:00Z' \
  git -C "$pre_merge_side_repository" merge \
    --no-ff \
    --no-commit \
    pre-doc-bound-change
git -C "$pre_merge_side_repository" checkout HEAD -- \
  docs/specifications/v1-proof-program.md
git -C "$pre_merge_side_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:08:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:08:00Z' \
  git -C "$pre_merge_side_repository" commit -qm 'Merge historical branch without bound drift'
expect_strict_failure "$pre_merge_side_repository" \
  'nonidentical attestation histories require a two-parent preserving merge whose second parent is the selected canonical evidence commit'

stale_partial_repository="$fixture_root/stale-pre-merge-partial-history"
cp -R "$merge_repository" "$stale_partial_repository"
git -C "$stale_partial_repository" checkout -q -b stale-partial-history \
  "$strict_source_commit"
git -C "$stale_partial_repository" checkout "$evidence_commit" -- \
  docs/receipts/reviews/rev-18.md
git -C "$stale_partial_repository" add docs/receipts/reviews/rev-18.md
GIT_AUTHOR_DATE='2026-07-24T14:08:10Z' \
GIT_COMMITTER_DATE='2026-07-24T14:08:10Z' \
  git -C "$stale_partial_repository" commit -qm 'Create stale partial receipt state'
git -C "$stale_partial_repository" checkout -q merge-main
GIT_AUTHOR_DATE='2026-07-24T14:08:20Z' \
GIT_COMMITTER_DATE='2026-07-24T14:08:20Z' \
  git -C "$stale_partial_repository" merge \
    --no-ff \
    --no-commit \
    stale-partial-history
GIT_AUTHOR_DATE='2026-07-24T14:08:30Z' \
GIT_COMMITTER_DATE='2026-07-24T14:08:30Z' \
  git -C "$stale_partial_repository" commit -qm 'Hide stale partial receipt state'
expect_strict_failure "$stale_partial_repository" \
  'prior canonical attestation set must be wholly absent or contain all 22 records'

stale_deletion_repository="$fixture_root/stale-pre-merge-deleted-history"
cp -R "$merge_repository" "$stale_deletion_repository"
git -C "$stale_deletion_repository" checkout -q -b stale-deleted-history \
  "$evidence_commit"
find "$stale_deletion_repository/docs/receipts" \
  -type f \
  -name '*.md' \
  ! -name 'README.md' \
  -delete
git -C "$stale_deletion_repository" add -u docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:08:40Z' \
GIT_COMMITTER_DATE='2026-07-24T14:08:40Z' \
  git -C "$stale_deletion_repository" commit -qm 'Delete stale receipt history'
git -C "$stale_deletion_repository" checkout -q merge-main
GIT_AUTHOR_DATE='2026-07-24T14:08:50Z' \
GIT_COMMITTER_DATE='2026-07-24T14:08:50Z' \
  git -C "$stale_deletion_repository" merge \
    --no-ff \
    --no-commit \
    stale-deleted-history
git -C "$stale_deletion_repository" checkout HEAD -- docs/receipts
git -C "$stale_deletion_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:09:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:00Z' \
  git -C "$stale_deletion_repository" commit -qm 'Hide stale receipt deletion'
expect_strict_failure "$stale_deletion_repository" \
  'prior canonical attestation set is absent after earlier canonical history'

stale_divergent_repository="$fixture_root/stale-pre-merge-divergent-history"
cp -R "$merge_repository" "$stale_divergent_repository"
git -C "$stale_divergent_repository" checkout -q -b stale-divergent-history \
  "$evidence_commit"
append_replacement_pair \
  "$stale_divergent_repository" \
  "$strict_archive_digest" \
  'stale-pre-merge-divergent-side'
git -C "$stale_divergent_repository" checkout -q merge-main
GIT_AUTHOR_DATE='2026-07-24T14:09:10Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:10Z' \
  git -C "$stale_divergent_repository" merge \
    --no-ff \
    --no-commit \
    stale-divergent-history
git -C "$stale_divergent_repository" checkout HEAD -- \
  docs/receipts \
  docs/specifications/v1-proof-program.md
git -C "$stale_divergent_repository" add \
  docs/receipts \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:09:20Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:20Z' \
  git -C "$stale_divergent_repository" commit -qm 'Hide stale divergent receipts'
expect_strict_failure "$stale_divergent_repository" \
  'nonidentical attestation histories require a two-parent preserving merge whose second parent is the selected canonical evidence commit'

source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"

historical_merge_repository="$fixture_root/historical-preserving-merge"
cp -R "$merge_repository" "$historical_merge_repository"
append_replacement_pair \
  "$historical_merge_repository" \
  "$strict_archive_digest" \
  'historical-preserving-merge'
python3 "$historical_merge_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$historical_merge_repository/docs/specifications/v1-delivery-program.md"

absent_merge_repository="$fixture_root/history-present-absent-merge"
cp -R "$strict_repository" "$absent_merge_repository"
absent_merge_valid_branch="$(
  git -C "$absent_merge_repository" branch --show-current
)"
git -C "$absent_merge_repository" checkout -q -b absent-history \
  "$strict_source_commit"
printf '%s\n' 'history without canonical receipts' \
  >"$absent_merge_repository/absent-history.txt"
git -C "$absent_merge_repository" add absent-history.txt
GIT_AUTHOR_DATE='2026-07-24T14:09:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:00Z' \
  git -C "$absent_merge_repository" commit -qm 'Advance absent receipt history'
git -C "$absent_merge_repository" checkout -q "$absent_merge_valid_branch"
GIT_AUTHOR_DATE='2026-07-24T14:09:10Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:10Z' \
  git -C "$absent_merge_repository" merge \
    --no-ff \
    --no-commit \
    absent-history
find "$absent_merge_repository/docs/receipts" \
  -type f \
  -name '*.md' \
  ! -name 'README.md' \
  -delete
git -C "$absent_merge_repository" add -u docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:09:20Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:20Z' \
  git -C "$absent_merge_repository" commit -qm 'Merge by hiding present receipts'
append_replacement_pair \
  "$absent_merge_repository" \
  '' \
  'history-present-absent-merge'
expect_strict_failure "$absent_merge_repository" \
  'prior canonical attestation set is absent after earlier canonical history'

partial_merge_repository="$fixture_root/history-complete-partial-merge"
cp -R "$strict_repository" "$partial_merge_repository"
partial_merge_valid_branch="$(
  git -C "$partial_merge_repository" branch --show-current
)"
git -C "$partial_merge_repository" checkout -q -b partial-history
rm "$partial_merge_repository/docs/receipts/reviews/rev-18.md"
git -C "$partial_merge_repository" add -u docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:09:30Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:30Z' \
  git -C "$partial_merge_repository" commit -qm 'Create partial receipt history'
git -C "$partial_merge_repository" checkout -q "$partial_merge_valid_branch"
GIT_AUTHOR_DATE='2026-07-24T14:09:40Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:40Z' \
  git -C "$partial_merge_repository" merge \
    --no-ff \
    --no-commit \
    partial-history
git -C "$partial_merge_repository" checkout HEAD -- \
  docs/receipts/reviews/rev-18.md
git -C "$partial_merge_repository" add docs/receipts/reviews/rev-18.md
GIT_AUTHOR_DATE='2026-07-24T14:09:50Z' \
GIT_COMMITTER_DATE='2026-07-24T14:09:50Z' \
  git -C "$partial_merge_repository" commit -qm 'Hide partial receipt parent'
append_replacement_pair \
  "$partial_merge_repository" \
  "$strict_archive_digest" \
  'history-complete-partial-merge'
expect_strict_failure "$partial_merge_repository" \
  'prior canonical attestation set must be wholly absent or contain all 22 records'

divergent_merge_repository="$fixture_root/history-divergent-complete-merge"
cp -R "$strict_repository" "$divergent_merge_repository"
divergent_merge_valid_branch="$(
  git -C "$divergent_merge_repository" branch --show-current
)"
git -C "$divergent_merge_repository" checkout -q -b replacement-history
append_replacement_pair \
  "$divergent_merge_repository" \
  "$strict_archive_digest" \
  'history-divergent-side'
git -C "$divergent_merge_repository" checkout -q "$divergent_merge_valid_branch"
GIT_AUTHOR_DATE='2026-07-24T14:10:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:10:00Z' \
  git -C "$divergent_merge_repository" merge \
    --no-ff \
    --no-commit \
    replacement-history
git -C "$divergent_merge_repository" checkout HEAD -- \
  docs/receipts \
  docs/specifications/v1-proof-program.md
git -C "$divergent_merge_repository" add \
  docs/receipts \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:10:10Z' \
GIT_COMMITTER_DATE='2026-07-24T14:10:10Z' \
  git -C "$divergent_merge_repository" commit -qm 'Hide divergent receipt parent'
append_replacement_pair \
  "$divergent_merge_repository" \
  "$strict_archive_digest" \
  'history-divergent-complete-merge'
expect_strict_failure "$divergent_merge_repository" \
  'nonidentical attestation histories require a two-parent preserving merge whose second parent is the selected canonical evidence commit'

replacement_merge_repository="$fixture_root/history-valid-replacement-merge"
cp -R "$strict_repository" "$replacement_merge_repository"
replacement_merge_valid_branch="$(
  git -C "$replacement_merge_repository" branch --show-current
)"
git -C "$replacement_merge_repository" checkout -q -b replacement-evidence
append_replacement_pair \
  "$replacement_merge_repository" \
  "$strict_archive_digest" \
  'history-valid-replacement-side'
replacement_merge_digest="$archive_digest"
git -C "$replacement_merge_repository" checkout -q \
  "$replacement_merge_valid_branch"
GIT_AUTHOR_DATE='2026-07-24T14:10:20Z' \
GIT_COMMITTER_DATE='2026-07-24T14:10:20Z' \
  git -C "$replacement_merge_repository" merge \
    --no-ff \
    -q \
    -m 'Merge replacement evidence' \
    replacement-evidence
append_replacement_pair \
  "$replacement_merge_repository" \
  "$replacement_merge_digest" \
  'history-valid-replacement-descendant'
python3 "$replacement_merge_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$replacement_merge_repository/docs/specifications/v1-delivery-program.md"

source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"

replacement_repository="$fixture_root/replacement-repository"
cp -R "$strict_repository" "$replacement_repository"
prior_archive_digest="$archive_digest"
printf '\n' >>"$replacement_repository/docs/specifications/v1-proof-program.md"
git -C "$replacement_repository" add docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:00:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:00:00Z' \
  git -C "$replacement_repository" commit -qm 'Freeze replacement source'

source_commit="$(git -C "$replacement_repository" rev-parse HEAD)"
source_tree="$(git -C "$replacement_repository" rev-parse 'HEAD^{tree}')"
replacement_archive_path="$fixture_root/replacement-source.tar"
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
  git --no-replace-objects -C "$replacement_repository" \
    -c "core.attributesFile=$null_device" \
    -c tar.umask=0002 \
    archive --format=tar \
    -o "$replacement_archive_path" \
    "$source_commit" \
    -- docs/specifications docs/decisions
archive_digest="$(
  python3 - "$replacement_archive_path" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
write_receipt_set "$replacement_repository" "$prior_archive_digest"
git -C "$replacement_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:30:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:30:00Z' \
  git -C "$replacement_repository" commit -qm 'Replace DOC-00 attestations'

python3 "$replacement_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$replacement_repository/docs/specifications/v1-delivery-program.md"

replacement_source_commit="$source_commit"
replacement_source_tree="$source_tree"
replacement_archive_digest="$archive_digest"

rewritten_finding_repository="$fixture_root/replacement-rewritten-finding"
cp -R "$strict_repository" "$rewritten_finding_repository"
python3 - \
  "$rewritten_finding_repository/docs/specifications/v1-delivery-program.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
old = "Recursive predecessor validation accepted a historical evidence pair"
new = "Altered predecessor validation accepted a historical evidence pair"
if text.count(old) != 1:
    raise SystemExit("expected one FND-263 mutation target")
path.write_text(text.replace(old, new, 1))
PY
git -C "$rewritten_finding_repository" add \
  docs/specifications/v1-delivery-program.md
GIT_AUTHOR_DATE='2026-07-24T14:15:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:15:00Z' \
  git -C "$rewritten_finding_repository" commit -qm 'Rewrite attested finding'
set_source_binding \
  "$rewritten_finding_repository" \
  "$fixture_root/replacement-rewritten-finding.tar"
write_receipt_set "$rewritten_finding_repository" "$strict_archive_digest"
git -C "$rewritten_finding_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:16:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:16:00Z' \
  git -C "$rewritten_finding_repository" commit -qm 'Bind rewritten finding'
expect_strict_failure "$rewritten_finding_repository" \
  'previously attested finding FND-263 was rewritten'

rewritten_conformance_repository="$fixture_root/replacement-rewritten-conformance"
cp -R "$strict_repository" "$rewritten_conformance_repository"
python3 - \
  "$rewritten_conformance_repository/docs/specifications/v1-delivery-program.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
old = "This source receipt\ncontains no self-referential final digest"
new = "This source receipt\ncontains no circular final digest"
if text.count(old) != 1:
    raise SystemExit("expected one DOC-CONF-22 mutation target")
path.write_text(text.replace(old, new, 1))
PY
git -C "$rewritten_conformance_repository" add \
  docs/specifications/v1-delivery-program.md
GIT_AUTHOR_DATE='2026-07-24T14:17:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:17:00Z' \
  git -C "$rewritten_conformance_repository" commit -qm 'Rewrite attested conformance'
set_source_binding \
  "$rewritten_conformance_repository" \
  "$fixture_root/replacement-rewritten-conformance.tar"
write_receipt_set "$rewritten_conformance_repository" "$strict_archive_digest"
git -C "$rewritten_conformance_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:18:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:18:00Z' \
  git -C "$rewritten_conformance_repository" commit -qm 'Bind rewritten conformance'
expect_strict_failure "$rewritten_conformance_repository" \
  'previously attested conformance receipt DOC-CONF-22 was rewritten'

source_commit="$replacement_source_commit"
source_tree="$replacement_source_tree"
archive_digest="$replacement_archive_digest"

replacement_failure="$fixture_root/replacement-missing-history"
cp -R "$replacement_repository" "$replacement_failure"
receipt="$replacement_failure/docs/receipts/reviews/rev-18.md"
sed \
  "s#| Replaces | \`REV-18 at archive digest $prior_archive_digest\` |#| Replaces | \`None\` |#" \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$replacement_failure" \
  "attestation REV-18 Replaces must name prior Record ID REV-18 and archive digest $prior_archive_digest"

deceptive_replacement="$fixture_root/replacement-deceptive-history"
cp -R "$replacement_repository" "$deceptive_replacement"
receipt="$deceptive_replacement/docs/receipts/reviews/rev-18.md"
sed \
  "s#| Replaces | \`REV-18 at archive digest $prior_archive_digest\` |#| Replaces | \`does not replace REV-18 at archive digest $prior_archive_digest\` |#" \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
expect_strict_failure "$deceptive_replacement" \
  "attestation REV-18 Replaces must name prior Record ID REV-18 and archive digest $prior_archive_digest"

same_digest_replacement="$fixture_root/replacement-same-digest"
cp -R "$replacement_repository" "$same_digest_replacement"
python3 - \
  "$same_digest_replacement/docs/receipts" \
  "$archive_digest" \
  "$prior_archive_digest" <<'PY'
from pathlib import Path
import sys

receipt_root = Path(sys.argv[1])
current_digest = sys.argv[2]
prior_digest = sys.argv[3]
for path in receipt_root.rglob("*.md"):
    if path.name == "README.md":
        continue
    text = path.read_text()
    text = text.replace(
        f"| Archive SHA-256 | `{current_digest}` |",
        f"| Archive SHA-256 | `{prior_digest}` |",
    )
    path.write_text(text)
PY
expect_strict_failure "$same_digest_replacement" \
  'replacement attestation DOC-CONF-22 must bind a new archive digest'

deleted_history_repository="$fixture_root/replacement-deleted-source-history"
cp -R "$strict_repository" "$deleted_history_repository"
find "$deleted_history_repository/docs/receipts" \
  -type f \
  -name '*.md' \
  ! -name 'README.md' \
  -delete
git -C "$deleted_history_repository" add -u docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:32:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:32:00Z' \
  git -C "$deleted_history_repository" commit -qm 'Delete prior receipt history'
source_commit="$(git -C "$deleted_history_repository" rev-parse HEAD)"
source_tree="$(git -C "$deleted_history_repository" rev-parse 'HEAD^{tree}')"
deleted_history_archive="$fixture_root/replacement-deleted-source-history.tar"
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
  git --no-replace-objects -C "$deleted_history_repository" \
    -c "core.attributesFile=$null_device" \
    -c tar.umask=0002 \
    archive --format=tar \
    -o "$deleted_history_archive" \
    "$source_commit" \
    -- docs/specifications docs/decisions
archive_digest="$(
  python3 - "$deleted_history_archive" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
write_receipt_set "$deleted_history_repository"
git -C "$deleted_history_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:33:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:33:00Z' \
  git -C "$deleted_history_repository" commit -qm 'Reset deleted receipt history'
expect_strict_failure "$deleted_history_repository" \
  'attested Source commit must preserve prior canonical attestation byte-for-byte: docs/receipts/doc-00-g0.md'

rewritten_history_repository="$fixture_root/replacement-rewritten-source-history"
cp -R "$strict_repository" "$rewritten_history_repository"
forged_prior_digest='aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
python3 - \
  "$rewritten_history_repository/docs/receipts" \
  "$prior_archive_digest" \
  "$forged_prior_digest" <<'PY'
from pathlib import Path
import sys

receipt_root = Path(sys.argv[1])
prior_digest = sys.argv[2]
forged_digest = sys.argv[3]
for path in receipt_root.rglob("*.md"):
    if path.name == "README.md":
        continue
    text = path.read_text()
    text = text.replace(
        f"| Archive SHA-256 | `{prior_digest}` |",
        f"| Archive SHA-256 | `{forged_digest}` |",
    )
    path.write_text(text)
PY
git -C "$rewritten_history_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:34:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:34:00Z' \
  git -C "$rewritten_history_repository" commit -qm 'Rewrite prior receipt history'
source_commit="$(git -C "$rewritten_history_repository" rev-parse HEAD)"
source_tree="$(git -C "$rewritten_history_repository" rev-parse 'HEAD^{tree}')"
rewritten_history_archive="$fixture_root/replacement-rewritten-source-history.tar"
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
  git --no-replace-objects -C "$rewritten_history_repository" \
    -c "core.attributesFile=$null_device" \
    -c tar.umask=0002 \
    archive --format=tar \
    -o "$rewritten_history_archive" \
    "$source_commit" \
    -- docs/specifications docs/decisions
archive_digest="$(
  python3 - "$rewritten_history_archive" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
write_receipt_set "$rewritten_history_repository" "$forged_prior_digest"
git -C "$rewritten_history_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:00Z' \
  git -C "$rewritten_history_repository" commit -qm 'Replace forged receipt history'
expect_strict_failure "$rewritten_history_repository" \
  'attested Source commit must preserve prior canonical attestation byte-for-byte: docs/receipts/doc-00-g0.md'

leading_whitespace_repository="$fixture_root/replacement-leading-whitespace-prior-receipt"
cp -R "$strict_repository" "$leading_whitespace_repository"
git -C "$leading_whitespace_repository" checkout -q --detach \
  "$strict_source_commit"
source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"
write_receipt_set "$leading_whitespace_repository"
python3 - \
  "$leading_whitespace_repository/docs/receipts/reviews/rev-18.md" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
path.write_text("\n" + path.read_text())
PY
git -C "$leading_whitespace_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:10Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:10Z' \
  git -C "$leading_whitespace_repository" commit -qm 'Record noncanonical prior receipt'
printf '\n' \
  >>"$leading_whitespace_repository/docs/specifications/v1-proof-program.md"
git -C "$leading_whitespace_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:35:20Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:20Z' \
  git -C "$leading_whitespace_repository" commit -qm 'Freeze source over noncanonical prior receipt'
set_source_binding \
  "$leading_whitespace_repository" \
  "$fixture_root/replacement-leading-whitespace-prior-receipt.tar"
write_receipt_set \
  "$leading_whitespace_repository" \
  "$strict_archive_digest"
git -C "$leading_whitespace_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:30Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:30Z' \
  git -C "$leading_whitespace_repository" commit -qm 'Replace noncanonical prior receipt'
expect_strict_failure "$leading_whitespace_repository" \
  'attestation REV-18 must contain only its canonical heading and field table'

invalid_prior_semantics_repository="$fixture_root/replacement-invalid-prior-semantics"
cp -R "$strict_repository" "$invalid_prior_semantics_repository"
git -C "$invalid_prior_semantics_repository" checkout -q --detach \
  "$strict_source_commit"
source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"
write_receipt_set "$invalid_prior_semantics_repository"
receipt="$invalid_prior_semantics_repository/docs/receipts/reviews/rev-18.md"
sed \
  's#| Actor | `reviewer-18` |#| Actor | `reviewer-17` |#' \
  "$receipt" >"$receipt.tmp"
mv "$receipt.tmp" "$receipt"
git -C "$invalid_prior_semantics_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:30Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:30Z' \
  git -C "$invalid_prior_semantics_repository" commit -qm 'Record invalid prior semantics'
printf '\n' \
  >>"$invalid_prior_semantics_repository/docs/specifications/v1-proof-program.md"
git -C "$invalid_prior_semantics_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:35:40Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:40Z' \
  git -C "$invalid_prior_semantics_repository" commit -qm 'Freeze source over invalid prior semantics'
set_source_binding \
  "$invalid_prior_semantics_repository" \
  "$fixture_root/replacement-invalid-prior-semantics.tar"
write_receipt_set "$invalid_prior_semantics_repository" "$strict_archive_digest"
git -C "$invalid_prior_semantics_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:50Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:50Z' \
  git -C "$invalid_prior_semantics_repository" commit -qm 'Replace invalid prior semantics'
expect_strict_failure "$invalid_prior_semantics_repository" \
  'duplicate review Actor: reviewer-17'

invalid_prior_governance_repository="$fixture_root/replacement-invalid-prior-governance"
cp -R "$strict_repository" "$invalid_prior_governance_repository"
git -C "$invalid_prior_governance_repository" checkout -q --detach \
  "$strict_source_commit"
git -C "$invalid_prior_governance_repository" update-index \
  --chmod=-x \
  scripts/test-v1-delivery-program-check.sh
GIT_AUTHOR_DATE='2026-07-24T14:35:51Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:51Z' \
  git -C "$invalid_prior_governance_repository" commit -qm 'Freeze invalid prior governance mode'
set_source_binding \
  "$invalid_prior_governance_repository" \
  "$fixture_root/replacement-invalid-prior-governance-source.tar"
prior_governance_archive_digest="$archive_digest"
write_receipt_set "$invalid_prior_governance_repository"
git -C "$invalid_prior_governance_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:52Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:52Z' \
  git -C "$invalid_prior_governance_repository" commit -qm 'Record invalid prior governance pair'
git -C "$invalid_prior_governance_repository" update-index \
  --chmod=+x \
  scripts/test-v1-delivery-program-check.sh
printf '\n' \
  >>"$invalid_prior_governance_repository/docs/specifications/v1-proof-program.md"
git -C "$invalid_prior_governance_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:35:53Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:53Z' \
  git -C "$invalid_prior_governance_repository" commit -qm 'Freeze valid current governance mode'
set_source_binding \
  "$invalid_prior_governance_repository" \
  "$fixture_root/replacement-invalid-prior-governance-current.tar"
write_receipt_set \
  "$invalid_prior_governance_repository" \
  "$prior_governance_archive_digest"
git -C "$invalid_prior_governance_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:54Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:54Z' \
  git -C "$invalid_prior_governance_repository" commit -qm 'Replace invalid prior governance pair'
expect_strict_failure "$invalid_prior_governance_repository" \
  'prior attested governance program must be an executable regular file: scripts/test-v1-delivery-program-check.sh'

invalid_prior_schema_repository="$fixture_root/replacement-invalid-prior-schema"
cp -R "$strict_repository" "$invalid_prior_schema_repository"
git -C "$invalid_prior_schema_repository" checkout -q --detach \
  "$strict_source_commit"
sed \
  's#`MergeAuthorization`#`InvalidMergeAuthorization`#' \
  "$invalid_prior_schema_repository/docs/receipts/README.md" \
  >"$invalid_prior_schema_repository/docs/receipts/README.md.tmp"
mv \
  "$invalid_prior_schema_repository/docs/receipts/README.md.tmp" \
  "$invalid_prior_schema_repository/docs/receipts/README.md"
git -C "$invalid_prior_schema_repository" add docs/receipts/README.md
GIT_AUTHOR_DATE='2026-07-24T14:35:55Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:55Z' \
  git -C "$invalid_prior_schema_repository" commit -qm 'Freeze invalid prior receipt schema'
set_source_binding \
  "$invalid_prior_schema_repository" \
  "$fixture_root/replacement-invalid-prior-schema-source.tar"
prior_schema_archive_digest="$archive_digest"
write_receipt_set "$invalid_prior_schema_repository"
git -C "$invalid_prior_schema_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:56Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:56Z' \
  git -C "$invalid_prior_schema_repository" commit -qm 'Record invalid prior schema pair'
git -C "$invalid_prior_schema_repository" checkout \
  "$strict_source_commit" \
  -- docs/receipts/README.md
printf '\n' \
  >>"$invalid_prior_schema_repository/docs/specifications/v1-proof-program.md"
git -C "$invalid_prior_schema_repository" add \
  docs/receipts/README.md \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:35:57Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:57Z' \
  git -C "$invalid_prior_schema_repository" commit -qm 'Freeze valid current receipt schema'
set_source_binding \
  "$invalid_prior_schema_repository" \
  "$fixture_root/replacement-invalid-prior-schema-current.tar"
write_receipt_set \
  "$invalid_prior_schema_repository" \
  "$prior_schema_archive_digest"
git -C "$invalid_prior_schema_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:58Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:58Z' \
  git -C "$invalid_prior_schema_repository" commit -qm 'Replace invalid prior schema pair'
expect_strict_failure "$invalid_prior_schema_repository" \
  'DOC-00 receipt README is missing canonical strict value: `MergeAuthorization`'

invalid_prior_readme_mode_repository="$fixture_root/replacement-invalid-prior-readme-mode"
cp -R "$strict_repository" "$invalid_prior_readme_mode_repository"
git -C "$invalid_prior_readme_mode_repository" checkout -q --detach \
  "$strict_source_commit"
git -C "$invalid_prior_readme_mode_repository" update-index \
  --chmod=+x \
  docs/receipts/README.md
GIT_AUTHOR_DATE='2026-07-24T14:35:59Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:59Z' \
  git -C "$invalid_prior_readme_mode_repository" commit -qm 'Freeze invalid prior receipt schema mode'
set_source_binding \
  "$invalid_prior_readme_mode_repository" \
  "$fixture_root/replacement-invalid-prior-readme-mode-source.tar"
prior_readme_mode_archive_digest="$archive_digest"
write_receipt_set "$invalid_prior_readme_mode_repository"
git -C "$invalid_prior_readme_mode_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:35:59Z' \
GIT_COMMITTER_DATE='2026-07-24T14:35:59Z' \
  git -C "$invalid_prior_readme_mode_repository" commit -qm 'Record invalid prior schema mode pair'
git -C "$invalid_prior_readme_mode_repository" update-index \
  --chmod=-x \
  docs/receipts/README.md
printf '\n' \
  >>"$invalid_prior_readme_mode_repository/docs/specifications/v1-proof-program.md"
git -C "$invalid_prior_readme_mode_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:36:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:36:00Z' \
  git -C "$invalid_prior_readme_mode_repository" commit -qm 'Freeze valid current receipt schema mode'
set_source_binding \
  "$invalid_prior_readme_mode_repository" \
  "$fixture_root/replacement-invalid-prior-readme-mode-current.tar"
write_receipt_set \
  "$invalid_prior_readme_mode_repository" \
  "$prior_readme_mode_archive_digest"
git -C "$invalid_prior_readme_mode_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:36:01Z' \
GIT_COMMITTER_DATE='2026-07-24T14:36:01Z' \
  git -C "$invalid_prior_readme_mode_repository" commit -qm 'Replace invalid prior schema mode pair'
expect_strict_failure "$invalid_prior_readme_mode_repository" \
  'prior DOC-00 receipt README must be a non-executable regular file'

expect_invalid_prior_checker_binding() {
  local case_name="$1"
  local mutation="$2"
  local expected_message="$3"
  local case_repository="$fixture_root/replacement-invalid-prior-checker-$case_name"
  local prior_checker_archive_digest

  cp -R "$strict_repository" "$case_repository"
  git -C "$case_repository" checkout -q --detach "$strict_source_commit"
  case "$mutation" in
    augassign)
      printf '%s\n' \
        '' \
        'G0_REQUIRED_CHECK_REFERENCES += ("./scripts/forged-check.sh",)' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    nested)
      printf '%s\n' \
        '' \
        'def mutate_historical_g0_contract():' \
        '    G0_CHANGE_AWARE_CHECK_PREFIX = "./scripts/forged-check.sh"' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-augassign)
      printf '%s\n' \
        '' \
        'EXPECTED_PROTECTED_FINDING_SHA256 += ""' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-nested)
      printf '%s\n' \
        '' \
        'def mutate_historical_protected_digest():' \
        '    EXPECTED_PROTECTED_CONFORMANCE_SHA256 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-dynamic)
      printf '%s\n' \
        '' \
        'def mutate_historical_digest_dynamically():' \
        '    globals()["UNRELATED_CHECKER_STATE"] = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    g0-dynamic)
      printf '%s\n' \
        '' \
        'def mutate_historical_g0_contract_dynamically():' \
        '    globals()["UNRELATED_G0_STATE"] = ()' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-attribute)
      printf '%s\n' \
        '' \
        'class HistoricalDigestNamespace:' \
        '    pass' \
        'historical_digest_namespace = HistoricalDigestNamespace()' \
        'historical_digest_namespace.EXPECTED_PROTECTED_FINDING_SHA256 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    g0-subscript)
      printf '%s\n' \
        '' \
        'historical_g0_namespace = {}' \
        'historical_g0_namespace["G0_REQUIRED_CHECK_REFERENCES"] = ()' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-alias-globals)
      printf '%s\n' \
        '' \
        'historical_namespace_accessor = globals' \
        'historical_namespace_accessor()["UNRELATED_CHECKER_STATE"] = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    g0-alias-setattr)
      printf '%s\n' \
        '' \
        'from builtins import setattr as historical_attribute_writer' \
        'historical_attribute_writer(object(), "G0_CHANGE_AWARE_CHECK_PREFIX", "./scripts/forged-check.sh")' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-alias-exec)
      printf '%s\n' \
        '' \
        'historical_code_runner = exec' \
        'historical_code_runner("EXPECTED_PROTECTED_FINDING_SHA256 = " + repr("f" * 64))' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-computed-namespace)
      printf '%s\n' \
        '' \
        'import sys as historical_checker_sys' \
        'historical_checker_namespace = historical_checker_sys.modules[__name__].__dict__' \
        'historical_checker_key = "EXPECTED_PROTECTED_" + "FINDING_SHA256"' \
        'historical_checker_namespace[historical_checker_key] = "f" * 64' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    protected-computed-exec)
      printf '%s\n' \
        '' \
        'import builtins as historical_checker_builtins' \
        'historical_checker_runner = historical_checker_builtins.__dict__["ex" + "ec"]' \
        'historical_checker_runner("EXPECTED_PROTECTED_FINDING_SHA256 = " + repr("f" * 64))' \
        >>"$case_repository/scripts/check-v1-delivery-program.py"
      ;;
    *)
      printf 'unknown historical checker mutation: %s\n' "$mutation" >&2
      exit 1
      ;;
  esac
  git -C "$case_repository" add scripts/check-v1-delivery-program.py
  GIT_AUTHOR_DATE='2026-07-24T14:36:02Z' \
  GIT_COMMITTER_DATE='2026-07-24T14:36:02Z' \
    git -C "$case_repository" commit -qm 'Freeze invalid prior checker binding'
  set_source_binding \
    "$case_repository" \
    "$fixture_root/replacement-invalid-prior-checker-$case_name-source.tar"
  prior_checker_archive_digest="$archive_digest"
  write_receipt_set "$case_repository"
  git -C "$case_repository" add docs/receipts
  GIT_AUTHOR_DATE='2026-07-24T14:36:03Z' \
  GIT_COMMITTER_DATE='2026-07-24T14:36:03Z' \
    git -C "$case_repository" commit -qm 'Record invalid prior checker pair'

  git -C "$case_repository" checkout "$strict_source_commit" -- \
    scripts/check-v1-delivery-program.py
  printf '\n' >>"$case_repository/docs/specifications/v1-proof-program.md"
  git -C "$case_repository" add \
    docs/specifications/v1-proof-program.md \
    scripts/check-v1-delivery-program.py
  GIT_AUTHOR_DATE='2026-07-24T14:36:04Z' \
  GIT_COMMITTER_DATE='2026-07-24T14:36:04Z' \
    git -C "$case_repository" commit -qm 'Freeze valid current checker binding'
  set_source_binding \
    "$case_repository" \
    "$fixture_root/replacement-invalid-prior-checker-$case_name-current.tar"
  write_receipt_set "$case_repository" "$prior_checker_archive_digest"
  git -C "$case_repository" add docs/receipts
  GIT_AUTHOR_DATE='2026-07-24T14:36:05Z' \
  GIT_COMMITTER_DATE='2026-07-24T14:36:05Z' \
    git -C "$case_repository" commit -qm 'Replace invalid prior checker pair'
  expect_strict_failure "$case_repository" "$expected_message"
}

expect_invalid_prior_checker_binding \
  augassign \
  augassign \
  'prior source checker must not rebind or mutate G0_REQUIRED_CHECK_REFERENCES outside its single module-level literal assignment'
expect_invalid_prior_checker_binding \
  nested \
  nested \
  'prior source checker must not rebind or mutate G0_CHANGE_AWARE_CHECK_PREFIX outside its single module-level literal assignment'
expect_invalid_prior_checker_binding \
  protected-augassign \
  protected-augassign \
  'source checker must not rebind or mutate EXPECTED_PROTECTED_FINDING_SHA256 outside its single module-level literal assignment'
expect_invalid_prior_checker_binding \
  protected-nested \
  protected-nested \
  'source checker must not rebind or mutate EXPECTED_PROTECTED_CONFORMANCE_SHA256 outside its single module-level literal assignment'
expect_invalid_prior_checker_binding \
  protected-dynamic \
  protected-dynamic \
  'source checker must not use dynamic namespace primitive globals while declaring checker contract literals'
expect_invalid_prior_checker_binding \
  g0-dynamic \
  g0-dynamic \
  'source checker must not use dynamic namespace primitive globals while declaring checker contract literals'
expect_invalid_prior_checker_binding \
  protected-attribute \
  protected-attribute \
  'source checker must not rebind or mutate EXPECTED_PROTECTED_FINDING_SHA256 outside its single module-level literal assignment'
expect_invalid_prior_checker_binding \
  g0-subscript \
  g0-subscript \
  'prior source checker must not rebind or mutate G0_REQUIRED_CHECK_REFERENCES outside its single module-level literal assignment'
expect_invalid_prior_checker_binding \
  protected-alias-globals \
  protected-alias-globals \
  'source checker must not use dynamic namespace primitive globals while declaring checker contract literals'
expect_invalid_prior_checker_binding \
  g0-alias-setattr \
  g0-alias-setattr \
  'source checker must not use dynamic namespace primitive setattr while declaring checker contract literals'
expect_invalid_prior_checker_binding \
  protected-alias-exec \
  protected-alias-exec \
  'source checker must not use dynamic namespace primitive exec while declaring checker contract literals'
expect_invalid_prior_checker_binding \
  protected-computed-namespace \
  protected-computed-namespace \
  'source checker must not mutate the current module namespace while declaring checker contract literals'
expect_invalid_prior_checker_binding \
  protected-computed-exec \
  protected-computed-exec \
  'source checker must not use dynamic namespace primitive exec while declaring checker contract literals'
source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"

partial_prior_repository="$fixture_root/replacement-partial-prior-set"
cp -R "$strict_repository" "$partial_prior_repository"
rm "$partial_prior_repository/docs/receipts/reviews/rev-18.md"
git -C "$partial_prior_repository" add -u docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:36:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:36:00Z' \
  git -C "$partial_prior_repository" commit -qm 'Prepare partial receipt history'
printf '\n' >>"$partial_prior_repository/docs/specifications/v1-proof-program.md"
git -C "$partial_prior_repository" add docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:37:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:37:00Z' \
  git -C "$partial_prior_repository" commit -qm 'Freeze clean source over partial history'
set_source_binding \
  "$partial_prior_repository" \
  "$fixture_root/replacement-partial-prior-set.tar"
write_receipt_set "$partial_prior_repository" "$strict_archive_digest"
git -C "$partial_prior_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:38:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:38:00Z' \
  git -C "$partial_prior_repository" commit -qm 'Replace partial receipt history'
expect_strict_failure "$partial_prior_repository" \
  'prior canonical attestation set must be wholly absent or contain all 22 records'

preparatory_forgery_repository="$fixture_root/replacement-preparatory-forgery"
cp -R "$strict_repository" "$preparatory_forgery_repository"
forged_prior_digest='aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
python3 - \
  "$preparatory_forgery_repository/docs/receipts" \
  "$strict_archive_digest" \
  "$forged_prior_digest" <<'PY'
from pathlib import Path
import sys

receipt_root = Path(sys.argv[1])
prior_digest = sys.argv[2]
forged_digest = sys.argv[3]
for path in receipt_root.rglob("*.md"):
    if path.name == "README.md":
        continue
    text = path.read_text()
    old = f"| Archive SHA-256 | `{prior_digest}` |"
    if text.count(old) != 1:
        raise SystemExit(f"expected one prior digest in {path}")
    path.write_text(text.replace(old, f"| Archive SHA-256 | `{forged_digest}` |", 1))
PY
git -C "$preparatory_forgery_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:39:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:39:00Z' \
  git -C "$preparatory_forgery_repository" commit -qm 'Prepare forged receipt history'
printf '\n' \
  >>"$preparatory_forgery_repository/docs/specifications/v1-proof-program.md"
git -C "$preparatory_forgery_repository" add \
  docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:40:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:40:00Z' \
  git -C "$preparatory_forgery_repository" commit -qm 'Freeze clean source over forged history'
set_source_binding \
  "$preparatory_forgery_repository" \
  "$fixture_root/replacement-preparatory-forgery.tar"
write_receipt_set "$preparatory_forgery_repository" "$forged_prior_digest"
git -C "$preparatory_forgery_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:41:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:41:00Z' \
  git -C "$preparatory_forgery_repository" commit -qm 'Replace forged prior set'
expect_strict_failure "$preparatory_forgery_repository" \
  'prior canonical attestation evidence commit must directly follow its recorded Source commit'

history_reset_repository="$fixture_root/replacement-preparatory-history-reset"
cp -R "$strict_repository" "$history_reset_repository"
find "$history_reset_repository/docs/receipts" \
  -type f \
  -name '*.md' \
  ! -name 'README.md' \
  -delete
git -C "$history_reset_repository" add -u docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:42:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:42:00Z' \
  git -C "$history_reset_repository" commit -qm 'Prepare absent receipt history'
printf '\n' >>"$history_reset_repository/docs/specifications/v1-proof-program.md"
git -C "$history_reset_repository" add docs/specifications/v1-proof-program.md
GIT_AUTHOR_DATE='2026-07-24T14:43:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:43:00Z' \
  git -C "$history_reset_repository" commit -qm 'Freeze clean source over absent history'
set_source_binding \
  "$history_reset_repository" \
  "$fixture_root/replacement-preparatory-history-reset.tar"
write_receipt_set "$history_reset_repository"
git -C "$history_reset_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:44:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:44:00Z' \
  git -C "$history_reset_repository" commit -qm 'Restart erased receipt history'
expect_strict_failure "$history_reset_repository" \
  'prior canonical attestation set is absent after earlier canonical history'

replace_object_repository="$fixture_root/replacement-object-history-reset"
cp -R "$history_reset_repository" "$replace_object_repository"
hidden_history_commit="$(
  git --no-replace-objects -C "$replace_object_repository" rev-parse HEAD^^
)"
hidden_history_tree="$(
  git --no-replace-objects -C "$replace_object_repository" \
    rev-parse "$hidden_history_commit^{tree}"
)"
forged_root_commit="$(
  GIT_AUTHOR_DATE='2026-07-24T14:44:10Z' \
  GIT_COMMITTER_DATE='2026-07-24T14:44:10Z' \
    git -C "$replace_object_repository" commit-tree \
      "$hidden_history_tree" \
      -m 'Forge a replacement root'
)"
git -C "$replace_object_repository" replace \
  "$hidden_history_commit" \
  "$forged_root_commit"
expect_strict_failure "$replace_object_repository" \
  'prior canonical attestation set is absent after earlier canonical history'

historical_checker_repository="$fixture_root/replacement-historical-checker-contract"
cp -R "$strict_repository" "$historical_checker_repository"
python3 - \
  "$historical_checker_repository/scripts/check-v1-delivery-program.py" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
old = '    "git diff --check",\n)\nG0_CHANGE_AWARE_CHECK_PREFIX'
new = (
    '    "git diff --check",\n'
    '    "./scripts/future-governance-check.sh",\n'
    ')\nG0_CHANGE_AWARE_CHECK_PREFIX'
)
if text.count(old) != 1:
    raise SystemExit("expected one G0 check tuple mutation target")
path.write_text(text.replace(old, new, 1))
PY
printf '\n' \
  >>"$historical_checker_repository/docs/specifications/v1-proof-program.md"
git -C "$historical_checker_repository" add \
  docs/specifications/v1-proof-program.md \
  scripts/check-v1-delivery-program.py
GIT_AUTHOR_DATE='2026-07-24T14:45:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:45:00Z' \
  git -C "$historical_checker_repository" commit -qm 'Extend current G0 check contract'
set_source_binding \
  "$historical_checker_repository" \
  "$fixture_root/replacement-historical-checker-contract.tar"
saved_g0_evidence="$g0_evidence"
g0_evidence="${g0_evidence%.}; \`./scripts/future-governance-check.sh\`."
write_receipt_set "$historical_checker_repository" "$strict_archive_digest"
g0_evidence="$saved_g0_evidence"
git -C "$historical_checker_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T14:46:00Z' \
GIT_COMMITTER_DATE='2026-07-24T14:46:00Z' \
  git -C "$historical_checker_repository" commit -qm 'Replace under extended G0 contract'
python3 "$historical_checker_repository/scripts/check-v1-delivery-program.py" \
  --require-receipts \
  "$historical_checker_repository/docs/specifications/v1-delivery-program.md"

source_commit="$strict_source_commit"
source_tree="$strict_source_tree"
archive_digest="$strict_archive_digest"

sha256_repository="$fixture_root/sha256-repository"
if git init -q --object-format=sha256 "$sha256_repository" 2>/dev/null; then
  mkdir -p \
    "$sha256_repository/docs/specifications" \
    "$sha256_repository/docs/decisions" \
    "$sha256_repository/docs/receipts" \
    "$sha256_repository/scripts"
  cp "$structural_base/docs/specifications/"*.md \
    "$sha256_repository/docs/specifications/"
  cp "$structural_base/docs/decisions/"*.md \
    "$sha256_repository/docs/decisions/"
  cp "$repository_root/docs/receipts/README.md" \
    "$sha256_repository/docs/receipts/README.md"
  cp "$checker" "$sha256_repository/scripts/check-v1-delivery-program.py"
  cp \
    "$repository_root/scripts/test-v1-delivery-program-check.sh" \
    "$sha256_repository/scripts/test-v1-delivery-program-check.sh"

  git -C "$sha256_repository" config user.name "DOC-00 SHA-256 Fixture"
  git -C "$sha256_repository" config \
    user.email \
    "doc00-sha256-fixture@example.invalid"
  git -C "$sha256_repository" config commit.gpgsign false
  git -C "$sha256_repository" add \
    docs/specifications \
    docs/decisions \
    docs/receipts/README.md \
    scripts
  GIT_AUTHOR_DATE='2026-07-24T15:00:00Z' \
  GIT_COMMITTER_DATE='2026-07-24T15:00:00Z' \
    git -C "$sha256_repository" commit -qm 'Freeze SHA-256 source'

  source_commit="$(git -C "$sha256_repository" rev-parse HEAD)"
  source_tree="$(git -C "$sha256_repository" rev-parse 'HEAD^{tree}')"
  sha256_archive_path="$fixture_root/sha256-source.tar"
  GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
    git --no-replace-objects -C "$sha256_repository" \
      -c "core.attributesFile=$null_device" \
      -c tar.umask=0002 \
      archive --format=tar \
      -o "$sha256_archive_path" \
      "$source_commit" \
      -- docs/specifications docs/decisions
  archive_digest="$(
    python3 - "$sha256_archive_path" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
  )"
  write_receipt_set "$sha256_repository"
  git -C "$sha256_repository" add docs/receipts
  GIT_AUTHOR_DATE='2026-07-24T15:30:00Z' \
  GIT_COMMITTER_DATE='2026-07-24T15:30:00Z' \
    git -C "$sha256_repository" commit -qm 'Record SHA-256 attestations'

  python3 "$sha256_repository/scripts/check-v1-delivery-program.py" \
    --require-receipts \
    "$sha256_repository/docs/specifications/v1-delivery-program.md"

  sha256_prefix_repository="$fixture_root/sha256-prefix-repository"
  cp -R "$sha256_repository" "$sha256_prefix_repository"
  abbreviated_commit="${source_commit:0:40}"
  python3 - \
    "$sha256_prefix_repository/docs/receipts" \
    "$source_commit" \
    "$abbreviated_commit" <<'PY'
from pathlib import Path
import sys

receipt_root = Path(sys.argv[1])
full_commit = sys.argv[2]
abbreviated_commit = sys.argv[3]
for path in receipt_root.rglob("*.md"):
    if path.name == "README.md":
        continue
    text = path.read_text()
    text = text.replace(
        f"| Source commit | `{full_commit}` |",
        f"| Source commit | `{abbreviated_commit}` |",
    )
    path.write_text(text)
PY
  expect_strict_failure "$sha256_prefix_repository" \
    'attested Source commit must be the full Git object ID'
fi

missing_checker_repository="$fixture_root/missing-source-checker"
cp -R "$structural_base" "$missing_checker_repository"
mkdir -p "$missing_checker_repository/docs/receipts"
cp "$repository_root/docs/receipts/README.md" \
  "$missing_checker_repository/docs/receipts/README.md"
git -C "$missing_checker_repository" init -q
git -C "$missing_checker_repository" config \
  user.name \
  "DOC-00 Missing Checker Fixture"
git -C "$missing_checker_repository" config \
  user.email \
  "doc00-missing-checker@example.invalid"
git -C "$missing_checker_repository" config commit.gpgsign false
git -C "$missing_checker_repository" add \
  docs/specifications \
  docs/decisions \
  docs/receipts/README.md
GIT_AUTHOR_DATE='2026-07-24T16:00:00Z' \
GIT_COMMITTER_DATE='2026-07-24T16:00:00Z' \
  git -C "$missing_checker_repository" commit -qm 'Freeze source without checker'

source_commit="$(git -C "$missing_checker_repository" rev-parse HEAD)"
source_tree="$(git -C "$missing_checker_repository" rev-parse 'HEAD^{tree}')"
missing_checker_archive_path="$fixture_root/missing-checker-source.tar"
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 \
  git --no-replace-objects -C "$missing_checker_repository" \
    -c "core.attributesFile=$null_device" \
    -c tar.umask=0002 \
    archive --format=tar \
    -o "$missing_checker_archive_path" \
    "$source_commit" \
    -- docs/specifications docs/decisions
archive_digest="$(
  python3 - "$missing_checker_archive_path" <<'PY'
from hashlib import sha256
from pathlib import Path
import sys

print(sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
)"
write_receipt_set "$missing_checker_repository"
git -C "$missing_checker_repository" add docs/receipts
GIT_AUTHOR_DATE='2026-07-24T16:30:00Z' \
GIT_COMMITTER_DATE='2026-07-24T16:30:00Z' \
  git -C "$missing_checker_repository" commit -qm 'Record unattached evidence'
expect_strict_failure "$missing_checker_repository" \
  'attested Source commit is missing governance program scripts/check-v1-delivery-program.py'
