#!/usr/bin/env python3
"""Validate the canonical V1 delivery-program registries."""

from __future__ import annotations

import ast
import hashlib
import os
import re
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path


DELIVERY_PROGRAM_PATH = Path("docs/specifications/v1-delivery-program.md")
PROOF_PROGRAM_PATH = Path("docs/specifications/v1-proof-program.md")
DECISION_DIRECTORY = Path("docs/decisions")
RECEIPT_DIRECTORY = Path("docs/receipts")
SOURCE_GOVERNANCE_PATHS = (
    Path("scripts/check-v1-delivery-program.py"),
    Path("scripts/test-v1-delivery-program-check.sh"),
)

REFERENCE_PATTERN = re.compile(r"`([A-Z][A-Z0-9]*-[0-9]{2})`")
CONFORMANCE_PATTERN = re.compile(
    r"^#### Manual conformance receipt(?: `DOC-CONF-([0-9]{2})`)?$"
)
FINDING_HISTORY_PATTERN = re.compile(
    r"^\| `FND-([0-9]{3})` / P[0-3] \|.*$",
    re.MULTILINE,
)
CONFORMANCE_HISTORY_PATTERN = re.compile(
    r"^#### Manual conformance receipt(?: `DOC-CONF-([0-9]{2})`)?$",
    re.MULTILINE,
)
REQUIRED_REPOSITORY_CHECKS_HEADING = "\n### Required repository checks"
GRAPH_NODE_PATTERN = re.compile(
    r'\b([A-Z][A-Z0-9]*)\["([A-Z][A-Z0-9]*-[0-9]{2})[^"]*"\]'
)
GRAPH_EDGE_PATTERN = re.compile(
    r'^\s*([A-Z][A-Z0-9]*)(?:\["[^"]+"\])?\s+'
    r'(?:-->|-\.\s+"[^"]*"\s+\.->)\s+'
    r'([A-Z][A-Z0-9]*)(?:\["[^"]+"\])?\s*$'
)
MARKDOWN_LINK_PATTERN = re.compile(
    r"\[([^\[\]\r\n]+)\]\(([-A-Za-z0-9._/]+)\)"
)
DECISION_FILE_PATTERN = re.compile(r"^([0-9]{4})-[a-z0-9][a-z0-9-]*\.md$")
PROOF_ANCHORS = (
    "PROOF-G1-CONDITIONS-001",
    "PROOF-G1-HEADROOM-001",
    "PROOF-EXPECTATION-BRANCH-001",
    "PROOF-G9-PROTOCOL-001",
    "PROOF-EVIDENCE-RECEIPTS-001",
)

ATTESTATION_FIELDS = (
    "Schema",
    "Record ID",
    "Kind",
    "Status",
    "Actor",
    "Declaration",
    "Completed at",
    "Source commit",
    "Source tree",
    "Included paths",
    "Archive algorithm",
    "Archive SHA-256",
    "Method",
    "Findings",
    "Disposition",
    "Residual limits",
    "Evidence references",
    "Replaces",
)
ATTESTATION_SCHEMA = "doc00-attestation-v1"
ATTESTATION_INCLUDED_PATHS = "docs/specifications, docs/decisions"
ATTESTATION_ARCHIVE_ALGORITHM = "git-archive-tar-sha256-v1"
ATTESTATION_ARCHIVE_COMMAND = (
    "GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 "
    "git --no-replace-objects "
    "-c core.attributesFile=<OS null device> -c tar.umask=0002 "
    "archive --format=tar <source-commit> -- "
    "docs/specifications docs/decisions"
)
ATTESTATION_HISTORY_CONTRACT = (
    "The replacement source-freeze commit has at most one parent and preserves "
    "every canonical attestation tree entry from that parent byte-for-byte, "
    "including its file mode. At every earlier history head, the canonical set "
    "is either wholly absent or contains all 22 records. A present set must "
    "resolve to one common last-modified evidence commit whose direct parent is "
    "its attested source commit, whose only changes are those 22 records, and "
    "whose source, archive, schema, and replacement bindings are valid. "
    "Historical G0 check requirements are read as exactly one module-level "
    "literal binding per contract name from that set's own source checker; "
    "every additional AST binding is invalid, current policy is not substituted, "
    "and no historical checker code is executed. The protected digest names "
    "likewise have exactly one module-level lowercase SHA-256 string literal and "
    "no other binding or mutation in every current or historical checker "
    "revision. Attribute or subscript stores of protected names, wildcard "
    "imports, and direct, imported, or transitively aliased dynamic namespace "
    "or code-execution primitives are invalid. Current protected-region and G0 "
    "validation use "
    "parsed literal values rather than mutable module globals. Pull-request "
    "digest comparison uses the trusted comparison-base checker to parse both "
    "revisions, and append-only pull-request comparison uses that same extracted "
    "comparison-base checker; the head checker does not validate either claim "
    "about itself. Extracted comparators run with isolated Python startup so "
    "caller-controlled module search paths, user sites, and `sitecustomize` "
    "cannot alter them. A wholly absent set is a valid first-attestation state "
    "only when no canonical path has earlier reachable history. All checker Git "
    "subprocesses disable replacement objects and strip caller-controlled Git "
    "environment overrides. Strict receipt validation rejects nonempty legacy "
    "grafts and shallow repositories so reachable history cannot be substituted "
    "or truncated. Documentation CI checks out full history before strict "
    "validation. Validation traverses every commit reachable from HEAD and "
    "every merge parent with memoized canonical receipt-tree states; it rejects "
    "partial states, deletion after introduction, and any nonidentical parent "
    "states unless a two-parent preserving merge selects the exact evidence "
    "commit as its second parent and that set replaces the first-parent set. "
    "Validation "
    "follows source/evidence pairs recursively to that genuine first-attestation "
    "state. Each historical source also carries this schema as a non-executable "
    "regular file and both governance programs as executable regular files. "
    "Every successor source preserves each existing finding row and complete "
    "conformance section as an exact ordered prefix of both the pull-request "
    "comparison base and the recursively validated predecessor source; only new "
    "sequential entries may be appended. `Replaces` is derived from the validated "
    "predecessor set, never from mutable content introduced by the new source "
    "freeze or an earlier preparatory commit."
)
ATTESTATION_KINDS = {
    "completion": "MergeAuthorization",
    "consolidation": "Consolidation",
    "review": "Review",
}
G0_REQUIRED_CHECK_REFERENCES = (
    "./scripts/test-documentation-change-policy.sh",
    "./scripts/test-documentation-check.sh",
    "./scripts/check-documentation.sh",
    "./scripts/test-v1-delivery-program-check.sh",
    "./scripts/check-v1-delivery-program.py --require-receipts",
    "cargo fmt --all --check",
    (
        "cargo clippy --workspace --all-targets --all-features --locked -- "
        "-D warnings -F missing-docs -F unsafe-code"
    ),
    (
        'RUSTDOCFLAGS="-D warnings -F missing-docs -F unsafe-code" '
        "cargo doc --workspace --all-features --no-deps --locked"
    ),
    "cargo test --workspace --all-features --locked",
    "git diff --check",
)
G0_CHANGE_AWARE_CHECK_PREFIX = (
    "DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh"
)

EXPECTED_PACKAGE_COUNT = 54
EXPECTED_V1_PACKAGE_COUNT = 48
EXPECTED_POST_V1_PACKAGE_COUNT = 6
EXPECTED_DEPENDENCY_COUNT = 127
EXPECTED_V1_DEPENDENCY_COUNT = 123
EXPECTED_GRAPH_DEPENDENCY_COUNT = 132
EXPECTED_V1_GRAPH_DEPENDENCY_COUNT = 123
EXPECTED_INTERFACE_COUNT = 49
EXPECTED_REVIEW_COUNT = 18
EXPECTED_WAVE_COUNT = 34
EXPECTED_FINDING_COUNT = 365
EXPECTED_CONFORMANCE_COUNT = 24
EXPECTED_SPECIFICATION_COUNT = 12
EXPECTED_DECISION_COUNT = 34
EXPECTED_ACCEPTED_DECISION_COUNT = 26
EXPECTED_SUPERSEDED_DECISION_COUNT = 8
CANONICAL_G0_RECORD_ID = "DOC-CONF-24"
EXPECTED_CURRENT_FINDING_RANGE = f"FND-152..{EXPECTED_FINDING_COUNT:03d}"
EXPECTED_PROTECTED_CONFORMANCE_SHA256 = (
    "69630fce62dfbbc5b76a88b44b7cfee078f023685e0e99e640756d50cadac060"
)
EXPECTED_PROTECTED_FINDING_SHA256 = (
    "4b1d0cc32baa2d11656a7c69c8c9a687f6fd67bfa4f2f0652adead37e471d890"
)
PROTECTED_CONFORMANCE_START = b"#### Manual conformance receipt\n"
PROTECTED_CONFORMANCE_END = (
    b"#### Manual conformance receipt `DOC-CONF-22`"
)
PROTECTED_FINDING_START = b"| `FND-001` / P1 |"
PROTECTED_FINDING_END = b"| `FND-152` / P1 |"
EXPECTED_MILESTONES = tuple(f"M{index}" for index in range(8))
EXPECTED_V1_MILESTONES = tuple(f"M{index}" for index in range(7))
EXPECTED_WORK_BREAKDOWN_TABLE_COUNT = 7
EXPECTED_WORK_BREAKDOWN_PACKAGE_GROUPS = (
    ("DOC-00", "EVD-01", "TGT-00", "EVD-02", "BND-01", "TGT-01", "SEC-00"),
    ("CORE-01", "CORE-02", "EVAL-01", "EVAL-02"),
    ("EXP-01", "EXP-02", "EXP-03", "PLAN-01", "PLAN-02"),
    ("REN-01", "REN-02", "REN-03", "VAL-01", "REN-04", "REN-05", "REN-06"),
    (
        "MEM-01",
        "MEM-02",
        "MEM-03",
        "MEM-04",
        "MEM-05",
        "ENC-01",
        "SIT-01",
        "RET-01",
        "SIG-01",
        "ACT-00",
        "ACT-01",
        "COMP-01",
    ),
    (
        "E2E-00",
        "API-01",
        "CLI-01",
        "OBS-01",
        "SEC-01",
        "PERF-01",
        "SYS-01",
        "REL-01",
        "RCV-01",
        "E2E-02",
        "E2E-01",
        "REL-02",
        "REL-03",
    ),
    ("DATA-01", "MEM-06", "ML-01", "ML-02", "ML-03", "P3-01"),
)
EXPECTED_PACKAGE_ORDER = tuple(
    package
    for group in EXPECTED_WORK_BREAKDOWN_PACKAGE_GROUPS
    for package in group
)
EXPECTED_INTERFACE_IDS = (
    "IF-EVIDENCE-ENVELOPE",
    "IF-G1-ENVELOPE",
    "IF-G1-RECEIPT",
    "IF-BOUNDARY-FIXTURES",
    "IF-TARGET-ENVELOPE",
    "IF-SECURITY-ARCH",
    "IF-CORE-PRIMITIVES",
    "IF-DOMAIN-REQUEST",
    "IF-EVALUATION-PAYLOADS",
    "IF-CORPUS-REGISTRY",
    "IF-TRANSITION-DOMAIN",
    "IF-EXPECTATION-SETS",
    "IF-EXPECTATION-REPORT",
    "IF-FOCUS-CANDIDATES",
    "IF-PLAN",
    "IF-RENDER-TEXT",
    "IF-RENDER-DATA",
    "IF-RENDER-CANDIDATE",
    "IF-VALIDATOR",
    "IF-RENDER-QUALIFICATION",
    "IF-RENDER-MODEL-QUALIFICATION",
    "IF-RENDER-DEPLOYMENT",
    "IF-MEMORY-DECISION",
    "IF-MEMORY-REVISION",
    "IF-COMPILE-ADMISSION",
    "IF-MEMORY-MANAGEMENT",
    "IF-RECOVERY",
    "IF-PRIVACY-LIFECYCLE",
    "IF-ENCODED-FACETS",
    "IF-SITUATION",
    "IF-SIGNAL-CONTEXT",
    "IF-RETRIEVAL",
    "IF-SIGNALS",
    "IF-ACTIVATION-PARAMETERS",
    "IF-ACTIVATION",
    "IF-SHARED-SET",
    "IF-G9-PROTOCOL",
    "IF-G9-RUN-MANIFEST",
    "IF-COMPILE-API",
    "IF-CLI",
    "IF-DIAGNOSTICS",
    "IF-SECURITY-RECEIPT",
    "IF-PERFORMANCE-RECEIPT",
    "IF-SYSTEM-RECEIPT",
    "IF-RELEASE-CANDIDATE",
    "IF-RCV-RECEIPT",
    "IF-G9-RECEIPT",
    "IF-SHIP-AUTHORIZATION",
    "IF-SHIPMENT",
)
EXPECTED_FINDING_PRIORITY_DIGITS = (
    "111111111121122111111111211111111111122121121122222111221121111112"
    "111111111111121111111111111221111111111111211121111111111111111111"
    "1212111211221121211111111111212211121111121112121122121121211221121111111121111111122"
    "11112211111211211121111"
    "112231111221122121222121222"
    "1111211211211111121111111122121111121111221"
    "11111221121211122222112111121111111"
    "112111121"
    "11111121"
    "112"
)

WORK_BREAKDOWN_HEADER = (
    "ID",
    "Concise title, objective, scope, and non-scope",
    "Depends on",
    "Acceptance and exit evidence",
    "Contributes to",
)
EXECUTION_METADATA_HEADER = (
    "ID",
    "Consumes → produces; explicit non-scope",
    "Likely surfaces",
    "Type / P / complexity-confidence / lane / PR / milestone",
    "Security, privacy, migration, failure, and rollback review",
)
RESPONSIBILITY_HEADER = (
    "ID",
    "Owner role / value",
    "Requirements and canonical specifications",
    "Decision requirement",
    "Documentation, mathematics, API, and migration impact",
    "Verification and merge evidence",
)
MILESTONE_HEADER = (
    "Milestone",
    "Purpose and entrance",
    "Mandatory packages",
    "Exit evidence and next claim",
    "Stop rule",
)
WAVE_HEADER = (
    "Wave",
    "Packages eligible to proceed",
    "Layer completion evidence",
)
CONSOLIDATION_HEADER = (
    "Receipt / status",
    "Pass and reviewed scope",
    "Current evidence",
    "Reviewer independence and disposition",
)
INTERFACE_HEADER = (
    "Interface ID",
    "Producer",
    "Authorized consumers",
    "Contract and authority",
    "Acceptance, compatibility, and rollback",
)
REVIEW_HEADER = (
    "Receipt",
    "Perspective / independence basis",
    "Status and findings",
    "Disposition, owner, prerequisite, and evidence",
)
FINDING_HEADER = (
    "ID / severity",
    "Finding / perspective",
    "Disposition, resolution, and owner",
    "Residual status and evidence",
)


class ContractError(Exception):
    """A deterministic delivery-program invariant failed."""


@dataclass(frozen=True)
class AttestationHistoryState:
    """One validated canonical attestation state in reachable Git history."""

    records: dict[str, dict[str, str]]
    source_commit: str
    evidence_commit: str
    tree_entries: tuple[tuple[str, str], ...]


def section(text: str, start: str, end: str) -> str:
    """Return one heading-delimited section or fail clearly."""

    try:
        start_index = text.index(start)
        end_index = text.index(end, start_index + len(start))
    except ValueError as error:
        raise ContractError(
            f"missing or misordered section boundary: {start!r} -> {end!r}"
        ) from error
    return text[start_index:end_index]


def validate_protected_byte_region(
    source_bytes: bytes,
    start: bytes,
    end: bytes,
    expected_digest: str,
    label: str,
) -> None:
    """Require one bounded byte region to match its canonical SHA-256."""

    try:
        start_index = source_bytes.index(start)
        end_index = source_bytes.index(end, start_index + len(start))
    except ValueError as error:
        raise ContractError(f"missing or misordered protected {label}") from error
    actual_digest = hashlib.sha256(source_bytes[start_index:end_index]).hexdigest()
    if actual_digest != expected_digest:
        raise ContractError(
            f"protected {label} differs from its canonical byte digest"
        )


def canonical_git_text_bytes(source_bytes: bytes) -> bytes:
    """Normalize CRLF checkout translation to canonical Git LF bytes."""

    return source_bytes.replace(b"\r\n", b"\n")


def table_cells(line: str) -> list[str]:
    """Split a Markdown row without treating code or inline-math pipes as cells."""

    stripped = line.strip()
    if stripped.startswith("|"):
        stripped = stripped[1:]
    if stripped.endswith("|"):
        stripped = stripped[:-1]

    cells: list[str] = []
    current: list[str] = []
    in_code = False
    in_inline_math = False
    index = 0
    while index < len(stripped):
        if stripped.startswith(r"\(", index) and not in_code:
            in_inline_math = True
            current.append(r"\(")
            index += 2
            continue
        if stripped.startswith(r"\)", index) and in_inline_math and not in_code:
            in_inline_math = False
            current.append(r"\)")
            index += 2
            continue
        character = stripped[index]
        if character == "`" and (index == 0 or stripped[index - 1] != "\\"):
            in_code = not in_code
        if (
            character == "|"
            and not in_code
            and not in_inline_math
            and (index == 0 or stripped[index - 1] != "\\")
        ):
            cells.append("".join(current).strip())
            current = []
        else:
            current.append(character)
        index += 1
    if in_code or in_inline_math:
        raise ContractError(f"malformed Markdown table row: {line.strip()}")
    cells.append("".join(current).strip())
    return cells


def bounded_tables_rows(
    body: str,
    header: tuple[str, ...],
    expected_table_count: int,
    label: str,
) -> list[list[list[str]]]:
    """Return every row from exact tables in one bounded section."""

    lines = body.splitlines()
    expected_columns = len(header)
    header_indexes = [
        index
        for index, line in enumerate(lines)
        if line.strip().startswith("|")
        and line.strip().endswith("|")
        and tuple(table_cells(line)) == header
    ]
    if len(header_indexes) != expected_table_count:
        raise ContractError(
            f"{label} must contain exactly {expected_table_count} canonical "
            f"table{'s' if expected_table_count != 1 else ''}"
        )

    consumed_indexes: set[int] = set()
    tables: list[list[list[str]]] = []
    for header_index in header_indexes:
        if header_index in consumed_indexes or header_index + 2 >= len(lines):
            raise ContractError(f"{label} table is incomplete or overlaps")
        separator_index = header_index + 1
        separator_line = lines[separator_index].strip()
        separator = table_cells(separator_line)
        if (
            not separator_line.startswith("|")
            or not separator_line.endswith("|")
            or len(separator) != expected_columns
            or any(
                re.fullmatch(r":?-{3,}:?", cell) is None for cell in separator
            )
        ):
            raise ContractError(f"{label} table has a malformed separator")

        table_indexes = {header_index, separator_index}
        rows: list[list[str]] = []
        index = header_index + 2
        while index < len(lines) and lines[index].strip().startswith("|"):
            stripped = lines[index].strip()
            cells = table_cells(stripped)
            if (
                not stripped.endswith("|")
                or len(cells) != expected_columns
                or index in header_indexes
            ):
                raise ContractError(f"malformed {label} data row: {stripped}")
            rows.append(cells)
            table_indexes.add(index)
            index += 1
        if not rows:
            raise ContractError(f"{label} table has no data rows")
        if consumed_indexes.intersection(table_indexes):
            raise ContractError(f"{label} tables overlap")
        consumed_indexes.update(table_indexes)
        tables.append(rows)

    for index, line in enumerate(lines):
        if index in consumed_indexes or not line.strip() or "|" not in line:
            continue
        cells = table_cells(line)
        if (
            line.strip().startswith("|")
            or line.strip().endswith("|")
            or len(cells) == expected_columns
        ):
            raise ContractError(f"malformed {label} table row: {line.strip()}")
    return tables


def bounded_table_rows(
    body: str,
    header: tuple[str, ...],
    label: str,
) -> list[list[str]]:
    """Return every row from one exact bounded Markdown table."""

    return bounded_tables_rows(body, header, 1, label)[0]


def markdown_table_cells(line: str) -> list[str]:
    """Split a two-column Markdown row while preserving escaped pipes."""

    stripped = line.strip()
    if not stripped.startswith("|") or not stripped.endswith("|"):
        return []
    cells = re.split(r"(?<!\\)\|", stripped[1:-1])
    return [cell.replace(r"\|", "|").strip() for cell in cells]


def markdown_links(text: str) -> list[tuple[str, str]]:
    """Return only syntactically complete inline Markdown links."""

    return MARKDOWN_LINK_PATTERN.findall(text)


def inline_value(value: str) -> str:
    """Normalize one optional inline-code value from an attestation table."""

    normalized = value.strip()
    if len(normalized) >= 2 and normalized.startswith("`") and normalized.endswith("`"):
        normalized = normalized[1:-1]
    return normalized.strip()


def require_unique(values: list[str], label: str) -> None:
    """Reject duplicate identifiers."""

    duplicates = sorted(
        value for value in set(values) if values.count(value) > 1
    )
    if duplicates:
        raise ContractError(f"duplicate {label}: {', '.join(duplicates)}")


def require_contiguous(values: list[int], expected_count: int, label: str) -> None:
    """Reject missing, duplicate, or out-of-order numeric identifiers."""

    expected = list(range(1, expected_count + 1))
    if values != expected:
        raise ContractError(
            f"{label} must be exactly 1..{expected_count} in source order"
        )


def repository_root_for(path: Path) -> Path:
    """Resolve the repository root for a canonical delivery-program path."""

    resolved = path.resolve()
    if (
        resolved.name != DELIVERY_PROGRAM_PATH.name
        or resolved.parent.name != "specifications"
        or resolved.parent.parent.name != "docs"
    ):
        raise ContractError(
            "delivery program must be located at "
            "docs/specifications/v1-delivery-program.md"
        )
    return resolved.parents[2]


def require_regular_file(path: Path, label: str) -> None:
    """Require a non-symlink regular file."""

    if path.is_symlink() or not path.is_file():
        raise ContractError(f"missing or non-regular {label}: {path}")


def git_command(repository_root: Path, *arguments: str) -> list[str]:
    """Build one repository-scoped Git command that ignores replacement objects."""

    return [
        "git",
        "--no-replace-objects",
        "-C",
        str(repository_root),
        "-c",
        f"core.worktree={repository_root}",
        *arguments,
    ]


def git_environment(**overrides: str) -> dict[str, str]:
    """Build a Git environment without caller-controlled repository overrides."""

    environment = {
        name: value
        for name, value in os.environ.items()
        if not name.startswith("GIT_")
    }
    environment.update(
        {
            "GIT_CONFIG_GLOBAL": os.devnull,
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_NO_REPLACE_OBJECTS": "1",
            "GIT_OPTIONAL_LOCKS": "0",
            "GIT_TERMINAL_PROMPT": "0",
            **overrides,
        }
    )
    return environment


def git_bytes(repository_root: Path, *arguments: str) -> bytes:
    """Run one read-only Git query and return its exact stdout bytes."""

    try:
        result = subprocess.run(
            git_command(repository_root, *arguments),
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=git_environment(),
        )
    except (OSError, subprocess.CalledProcessError) as error:
        detail = ""
        if isinstance(error, subprocess.CalledProcessError):
            detail = error.stderr.decode("utf-8", errors="replace").strip()
        suffix = f": {detail}" if detail else ""
        raise ContractError(f"Git query failed{suffix}") from error
    return result.stdout


def git_output(repository_root: Path, *arguments: str) -> str:
    """Run one read-only Git query and return trimmed UTF-8 metadata."""

    try:
        return git_bytes(repository_root, *arguments).decode("utf-8").strip()
    except UnicodeError as error:
        raise ContractError("Git query returned non-UTF-8 text") from error


def git_blob_bytes(repository_root: Path, commit: str, path: str) -> bytes:
    """Read one Git blob without normalizing any content bytes."""

    return git_bytes(repository_root, "show", f"{commit}:{path}")


def git_blob_text(repository_root: Path, commit: str, path: str) -> str:
    """Read one exact UTF-8 Git blob without trimming its content."""

    try:
        return git_blob_bytes(repository_root, commit, path).decode("utf-8")
    except UnicodeError as error:
        raise ContractError(f"Git blob is not UTF-8 text: {path}") from error


def git_optional_tree_entry(repository_root: Path, commit: str, path: str) -> str:
    """Return one complete Git tree entry, or an empty string when absent."""

    entry = git_output(repository_root, "ls-tree", commit, "--", path)
    if entry and ("\n" in entry or not entry.endswith(f"\t{path}")):
        raise ContractError(f"cannot resolve unique Git tree entry: {path}")
    return entry


def git_tree_entry(repository_root: Path, commit: str, path: str) -> str:
    """Return one complete Git tree entry, including mode, type, and object ID."""

    entry = git_optional_tree_entry(repository_root, commit, path)
    if not entry:
        raise ContractError(f"cannot resolve canonical Git tree entry: {path}")
    return entry


def resolved_git_path(repository_root: Path, relative_path: str) -> Path:
    """Resolve one repository-internal Git path without trusting caller overrides."""

    location = git_output(
        repository_root,
        "rev-parse",
        "--git-path",
        relative_path,
    )
    path = Path(location)
    return path if path.is_absolute() else repository_root / path


def validate_git_history_view(repository_root: Path) -> None:
    """Require complete history without legacy graft substitution."""

    grafts_path = resolved_git_path(repository_root, "info/grafts")
    if grafts_path.is_symlink():
        raise ContractError("Git info/grafts must not be a symbolic link")
    if grafts_path.exists() and (
        not grafts_path.is_file() or grafts_path.stat().st_size > 0
    ):
        raise ContractError("Git info/grafts must be absent or empty")
    if (
        git_output(
            repository_root,
            "rev-parse",
            "--is-shallow-repository",
        )
        != "false"
    ):
        raise ContractError(
            "strict receipt validation requires complete non-shallow Git history"
        )


def validate_attested_archive_attributes(
    repository_root: Path,
    commit: str,
) -> None:
    """Reject tracked attributes that can transform the reviewed Git archive."""

    output = git_bytes(
        repository_root,
        "ls-tree",
        "-r",
        "-z",
        "--name-only",
        commit,
        "--",
        ".gitattributes",
        "docs",
    )
    if output and not output.endswith(b"\0"):
        raise ContractError(
            "Git tree query returned malformed archive-attribute paths"
        )
    for raw_path in output.split(b"\0"):
        if not raw_path:
            continue
        if raw_path == b".gitattributes" or (
            raw_path.startswith(b"docs/")
            and raw_path.endswith(b"/.gitattributes")
        ):
            display_path = raw_path.decode("utf-8", errors="backslashreplace")
            raise ContractError(
                "attested Source commit contains forbidden tracked "
                f"archive-attribute file: {display_path}"
            )


def git_archive_digest(repository_root: Path, commit: str) -> str:
    """Reconstruct the canonical reviewed archive and return its SHA-256."""

    validate_attested_archive_attributes(repository_root, commit)

    attributes_path = resolved_git_path(repository_root, "info/attributes")
    if attributes_path.is_symlink():
        raise ContractError("Git info/attributes must not be a symbolic link")
    if attributes_path.exists() and attributes_path.stat().st_size > 0:
        raise ContractError("Git info/attributes must be absent or empty")

    try:
        result = subprocess.run(
            git_command(
                repository_root,
                "-c",
                f"core.attributesFile={os.devnull}",
                "-c",
                "tar.umask=0002",
                "archive",
                "--format=tar",
                commit,
                "--",
                "docs/specifications",
                "docs/decisions",
            ),
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=git_environment(GIT_ATTR_NOSYSTEM="1"),
        )
    except (OSError, subprocess.CalledProcessError) as error:
        detail = ""
        if isinstance(error, subprocess.CalledProcessError):
            detail = error.stderr.decode("utf-8", errors="replace").strip()
        suffix = f": {detail}" if detail else ""
        raise ContractError(f"cannot reconstruct reviewed Git archive{suffix}") from error
    return hashlib.sha256(result.stdout).hexdigest()


def parse_packages(text: str) -> tuple[dict[str, str], set[tuple[str, str]]]:
    """Parse package dependency rows."""

    body = section(
        text,
        "### Full work breakdown",
        "### Work-package interface and execution metadata",
    )
    tables = bounded_tables_rows(
        body,
        WORK_BREAKDOWN_HEADER,
        EXPECTED_WORK_BREAKDOWN_TABLE_COUNT,
        "work breakdown",
    )
    packages: dict[str, str] = {}
    for table_index, rows in enumerate(tables):
        table_packages: list[str] = []
        for cells in rows:
            match = re.fullmatch(r"`([A-Z][A-Z0-9]*-[0-9]{2})`", cells[0])
            if match is None:
                raise ContractError(
                    f"malformed work breakdown package ID: {cells[0]}"
                )
            package_id = match.group(1)
            if package_id in packages:
                raise ContractError(f"duplicate package: {package_id}")
            packages[package_id] = cells[2]
            table_packages.append(package_id)
        if tuple(table_packages) != EXPECTED_WORK_BREAKDOWN_PACKAGE_GROUPS[table_index]:
            raise ContractError(
                "work breakdown package order or table membership differs "
                f"at canonical table {table_index + 1}"
            )

    if len(packages) != EXPECTED_PACKAGE_COUNT:
        raise ContractError(
            f"expected {EXPECTED_PACKAGE_COUNT} packages, found {len(packages)}"
        )

    dependencies: set[tuple[str, str]] = set()
    for target, dependency_cell in packages.items():
        sources = REFERENCE_PATTERN.findall(dependency_cell)
        require_unique(sources, f"dependency for {target}")
        unknown = sorted(set(sources).difference(packages))
        if unknown:
            raise ContractError(
                f"package {target} references unknown dependencies: "
                f"{', '.join(unknown)}"
            )
        dependencies.update((source, target) for source in sources)

    if len(dependencies) != EXPECTED_DEPENDENCY_COUNT:
        raise ContractError(
            "expected "
            f"{EXPECTED_DEPENDENCY_COUNT} declared dependency edges, "
            f"found {len(dependencies)}"
        )
    return packages, dependencies


def parse_package_registry(
    text: str,
    start: str,
    end: str,
    expected_packages: set[str],
    label: str,
) -> None:
    """Require one exact package row in a joined package registry."""

    body = section(text, start, end)
    header = (
        EXECUTION_METADATA_HEADER
        if label == "execution-metadata"
        else RESPONSIBILITY_HEADER
    )
    rows = bounded_table_rows(body, header, label)
    identifiers: list[str] = []
    for cells in rows:
        match = re.fullmatch(r"`([A-Z][A-Z0-9]*-[0-9]{2})`", cells[0])
        if match is None:
            raise ContractError(f"malformed {label} package ID: {cells[0]}")
        identifiers.append(match.group(1))
    require_unique(identifiers, f"{label} package")
    if tuple(identifiers) != EXPECTED_PACKAGE_ORDER:
        raise ContractError(
            f"{label} package order must match the canonical work breakdown"
        )
    actual = set(identifiers)
    if actual != expected_packages or len(identifiers) != len(expected_packages):
        missing = sorted(expected_packages.difference(actual))
        extra = sorted(actual.difference(expected_packages))
        raise ContractError(
            f"{label} package registry differs from the full work breakdown; "
            f"missing: {missing}; extra: {extra}"
        )


def parse_waves(text: str) -> tuple[list[str], dict[str, int]]:
    """Parse canonical wave labels and their V1 package assignments."""

    body = section(
        text,
        "#### Merge-wave registry",
        "#### Work-package selection and merge lifecycle",
    )
    rows = bounded_table_rows(body, WAVE_HEADER, "wave registry")
    waves: list[str] = []
    assignments: list[str] = []
    for cells in rows:
        match = re.fullmatch(r"`(W[0-9]{2})`", cells[0])
        if match is None:
            raise ContractError(f"malformed wave registry ID: {cells[0]}")
        waves.append(match.group(1))
        assignments.extend(REFERENCE_PATTERN.findall(cells[1]))

    expected_waves = [f"W{index:02d}" for index in range(EXPECTED_WAVE_COUNT)]
    if waves != expected_waves:
        raise ContractError("wave labels must be exactly W00..W33 in source order")
    require_unique(assignments, "V1 wave package assignment")
    if len(assignments) != EXPECTED_V1_PACKAGE_COUNT:
        raise ContractError(
            f"expected {EXPECTED_V1_PACKAGE_COUNT} V1 wave assignments, "
            f"found {len(assignments)}"
        )
    wave_by_package: dict[str, int] = {}
    for wave_index, cells in enumerate(rows):
        for package in REFERENCE_PATTERN.findall(cells[1]):
            wave_by_package[package] = wave_index
    return waves, wave_by_package


def parse_milestones(text: str) -> None:
    """Require the complete M0..M6 V1 and M7 post-V1 registry."""

    body = section(text, "#### Milestone registry", "#### Merge-wave registry")
    rows = bounded_table_rows(body, MILESTONE_HEADER, "milestone registry")
    milestones: list[str] = []
    for cells in rows:
        match = re.fullmatch(r"`(M[0-9]+)`", cells[0])
        if match is None:
            raise ContractError(f"malformed milestone registry ID: {cells[0]}")
        milestones.append(match.group(1))
    if tuple(milestones) != EXPECTED_MILESTONES:
        raise ContractError(
            "milestone labels must be exactly M0..M7 in source order, "
            "with V1 milestones M0..M6"
        )
    if tuple(milestones[: len(EXPECTED_V1_MILESTONES)]) != EXPECTED_V1_MILESTONES:
        raise ContractError("V1 milestone labels must be exactly M0..M6")


def format_edges(edges: set[tuple[str, str]]) -> str:
    """Format dependency edges in a stable diagnostic order."""

    return ", ".join(f"{source} -> {target}" for source, target in sorted(edges))


def parse_canonical_graph(
    text: str, packages: dict[str, str]
) -> set[tuple[str, str]]:
    """Parse and validate the canonical Mermaid dependency graph."""

    graph_section = section(
        text,
        "### Dependency graph and critical path",
        "The mandatory risk-first sequence is:",
    )
    graph_body = section(graph_section, "```mermaid", "```")

    aliases: dict[str, str] = {}
    for alias, package_id in GRAPH_NODE_PATTERN.findall(graph_body):
        previous = aliases.get(alias)
        if previous is not None and previous != package_id:
            raise ContractError(
                f"graph alias {alias} maps to both {previous} and {package_id}"
            )
        aliases[alias] = package_id

    aliases_by_package: dict[str, list[str]] = defaultdict(list)
    for alias, package_id in aliases.items():
        aliases_by_package[package_id].append(alias)
    duplicate_package_aliases = {
        package_id: package_aliases
        for package_id, package_aliases in aliases_by_package.items()
        if len(package_aliases) > 1
    }
    if duplicate_package_aliases:
        formatted = ", ".join(
            f"{package_id}: {sorted(package_aliases)}"
            for package_id, package_aliases in sorted(duplicate_package_aliases.items())
        )
        raise ContractError(f"canonical graph has multiple aliases per package: {formatted}")
    graph_packages = set(aliases.values())
    expected_packages = set(packages)
    if graph_packages != expected_packages:
        missing = sorted(expected_packages.difference(graph_packages))
        extra = sorted(graph_packages.difference(expected_packages))
        raise ContractError(
            "canonical graph package set differs from the full work breakdown; "
            f"missing: {missing}; extra: {extra}"
        )

    alias_edges: list[tuple[str, str]] = []
    for line in graph_body.splitlines():
        stripped = line.strip()
        if not stripped or stripped in {"```mermaid", "flowchart LR"}:
            continue
        match = GRAPH_EDGE_PATTERN.match(line)
        if match is not None:
            alias_edges.append((match.group(1), match.group(2)))
        else:
            raise ContractError(f"malformed canonical graph edge: {line.strip()}")

    dependencies: list[tuple[str, str]] = []
    for source_alias, target_alias in alias_edges:
        try:
            source = aliases[source_alias]
            target = aliases[target_alias]
        except KeyError as error:
            raise ContractError(
                f"canonical graph edge references unknown alias: {error.args[0]}"
            ) from error
        if source not in packages or target not in packages:
            unknown = source if source not in packages else target
            raise ContractError(
                f"canonical graph references unknown package: {unknown}"
            )
        dependencies.append((source, target))

    unique_dependencies = set(dependencies)
    if len(unique_dependencies) != len(dependencies):
        duplicate_edges = {
            edge for edge in unique_dependencies if dependencies.count(edge) > 1
        }
        raise ContractError(
            "duplicate canonical graph dependency edge: "
            f"{format_edges(duplicate_edges)}"
        )
    if len(unique_dependencies) != EXPECTED_GRAPH_DEPENDENCY_COUNT:
        raise ContractError(
            "expected "
            f"{EXPECTED_GRAPH_DEPENDENCY_COUNT} canonical graph dependency edges, "
            f"found {len(unique_dependencies)}"
        )
    return unique_dependencies


def require_acyclic(
    packages: set[str], dependencies: set[tuple[str, str]], label: str
) -> None:
    """Require one acyclic dependency graph rooted at DOC-00."""

    incoming: dict[str, int] = {package: 0 for package in packages}
    outgoing: dict[str, list[str]] = defaultdict(list)
    for source, target in dependencies:
        if source not in packages or target not in packages:
            continue
        incoming[target] += 1
        outgoing[source].append(target)

    roots = sorted(package for package, degree in incoming.items() if degree == 0)
    if roots != ["DOC-00"]:
        raise ContractError(
            f"{label} dependency root must be DOC-00, found {roots}"
        )

    ready = ["DOC-00"]
    visited: list[str] = []
    while ready:
        current = min(ready)
        ready.remove(current)
        visited.append(current)
        for target in sorted(outgoing[current]):
            incoming[target] -= 1
            if incoming[target] == 0:
                ready.append(target)

    if len(visited) != len(packages):
        blocked = sorted(packages.difference(visited))
        raise ContractError(f"{label} dependency graph contains a cycle: {blocked}")


def validate_specifications(repository_root: Path) -> None:
    """Require the frozen non-template specification inventory."""

    specification_directory = repository_root / DELIVERY_PROGRAM_PATH.parent
    if specification_directory.is_symlink() or not specification_directory.is_dir():
        raise ContractError(
            f"missing or non-regular specification directory: "
            f"{specification_directory}"
        )
    specification_files = sorted(
        path
        for path in specification_directory.glob("*.md")
        if path.name not in {"README.md", "TEMPLATE.md"}
    )
    for specification_file in specification_files:
        if specification_file.is_symlink() or not specification_file.is_file():
            raise ContractError(
                f"specification must be a regular file: {specification_file}"
            )
    if len(specification_files) != EXPECTED_SPECIFICATION_COUNT:
        raise ContractError(
            f"expected {EXPECTED_SPECIFICATION_COUNT} non-template "
            f"specifications, found {len(specification_files)}"
        )


def validate_decisions(repository_root: Path) -> None:
    """Require the frozen 33-record decision inventory and statuses."""

    decision_directory = repository_root / DECISION_DIRECTORY
    if decision_directory.is_symlink() or not decision_directory.is_dir():
        raise ContractError(f"missing or non-regular decision directory: {decision_directory}")

    decision_files = sorted(decision_directory.glob("[0-9][0-9][0-9][0-9]-*.md"))
    decision_ids: list[int] = []
    statuses: dict[int, str] = {}
    for decision_file in decision_files:
        if decision_file.is_symlink() or not decision_file.is_file():
            raise ContractError(f"decision must be a regular file: {decision_file}")
        match = DECISION_FILE_PATTERN.fullmatch(decision_file.name)
        if match is None:
            raise ContractError(f"malformed decision filename: {decision_file.name}")
        decision_id = int(match.group(1))
        decision_ids.append(decision_id)
        lines = decision_file.read_text(encoding="utf-8").splitlines()
        if len(lines) < 3 or not lines[2].startswith("Status: "):
            raise ContractError(f"decision {decision_file.name} has no canonical status")
        status = lines[2].removeprefix("Status: ")
        if status not in {"Accepted", "Superseded"}:
            raise ContractError(
                f"decision {decision_file.name} has non-frozen status: {status}"
            )
        if sum(line.startswith("Status: ") for line in lines) != 1:
            raise ContractError(f"decision {decision_file.name} has duplicate status")
        statuses[decision_id] = status

    require_contiguous(decision_ids, EXPECTED_DECISION_COUNT, "decision IDs")
    accepted = sum(status == "Accepted" for status in statuses.values())
    superseded = sum(status == "Superseded" for status in statuses.values())
    if accepted != EXPECTED_ACCEPTED_DECISION_COUNT:
        raise ContractError(
            f"expected {EXPECTED_ACCEPTED_DECISION_COUNT} Accepted decisions, "
            f"found {accepted}"
        )
    if superseded != EXPECTED_SUPERSEDED_DECISION_COUNT:
        raise ContractError(
            f"expected {EXPECTED_SUPERSEDED_DECISION_COUNT} Superseded decisions, "
            f"found {superseded}"
        )
    superseded_ids = sorted(
        decision_id
        for decision_id, status in statuses.items()
        if status == "Superseded"
    )
    if superseded_ids != [11, 12, 13, 15, 16, 19, 23, 28]:
        raise ContractError(
            "Superseded decisions must be exactly 0011, 0012, 0013, 0015, "
            "0016, 0019, 0023, and 0028"
        )


def validate_proof_anchors(repository_root: Path, delivery_text: str) -> None:
    """Require one proof definition and one canonical delivery-owner reference."""

    proof_path = repository_root / PROOF_PROGRAM_PATH
    require_regular_file(proof_path, "V1 proof program")
    proof_text = proof_path.read_text(encoding="utf-8")
    ownership = section(
        delivery_text, "### Documentation ownership matrix", "### Consolidation record"
    )
    for anchor in PROOF_ANCHORS:
        definition_pattern = re.compile(
            rf"^`{re.escape(anchor)}` is the sole normative owner\b",
            re.MULTILINE,
        )
        definitions = len(definition_pattern.findall(proof_text))
        if definitions != 1:
            raise ContractError(
                f"proof anchor {anchor} must be defined exactly once, "
                f"found {definitions}"
            )
        references = ownership.count(f"`{anchor}`")
        if references != 1:
            raise ContractError(
                f"delivery ownership matrix must reference {anchor} exactly once, "
                f"found {references}"
            )


def parse_attestation_text(text: str, expected_record_id: str) -> dict[str, str]:
    """Parse one exact Decision-0022 attestation table from text."""

    lines = text.splitlines()
    table_line_indexes = [
        index
        for index, line in enumerate(lines)
        if line.strip().startswith("|") and line.strip().endswith("|")
    ]
    header_indexes = [
        index
        for index in table_line_indexes
        if [inline_value(cell) for cell in markdown_table_cells(lines[index])]
        == ["Field", "Value"]
    ]
    if len(header_indexes) != 1:
        raise ContractError(
            f"attestation {expected_record_id} must contain exactly one Field/Value table"
        )
    header_index = header_indexes[0]
    expected_table_indexes = list(
        range(header_index, header_index + len(ATTESTATION_FIELDS) + 2)
    )
    if table_line_indexes != expected_table_indexes:
        raise ContractError(
            f"attestation {expected_record_id} must contain only one contiguous "
            f"{len(ATTESTATION_FIELDS)}-field table"
        )
    if expected_table_indexes[-1] >= len(lines):
        raise ContractError(f"attestation {expected_record_id} table is incomplete")
    non_table_content = [
        (index, line.strip())
        for index, line in enumerate(lines)
        if index not in expected_table_indexes and line.strip()
    ]
    if non_table_content != [(0, f"# {expected_record_id}")]:
        raise ContractError(
            f"attestation {expected_record_id} must contain only its canonical "
            "heading and field table"
        )

    separator = markdown_table_cells(lines[header_index + 1])
    if len(separator) != 2 or any(
        re.fullmatch(r":?-{3,}:?", cell) is None for cell in separator
    ):
        raise ContractError(
            f"attestation {expected_record_id} has a malformed table separator"
        )

    fields: dict[str, str] = {}
    observed_field_order: list[str] = []
    for offset, expected_field in enumerate(ATTESTATION_FIELDS, start=2):
        cells = markdown_table_cells(lines[header_index + offset])
        if len(cells) != 2:
            raise ContractError(
                f"attestation {expected_record_id} has a malformed field row"
            )
        field = inline_value(cells[0])
        value = inline_value(cells[1])
        observed_field_order.append(field)
        if not value:
            raise ContractError(
                f"attestation {expected_record_id} has an empty {field} field"
            )
        fields[field] = value
    if tuple(observed_field_order) != ATTESTATION_FIELDS:
        raise ContractError(
            f"attestation {expected_record_id} fields are missing, extra, "
            "duplicated, or out of order"
        )
    if fields["Record ID"] != expected_record_id:
        raise ContractError(
            f"attestation path expects Record ID {expected_record_id}, "
            f"found {fields['Record ID']}"
        )
    return fields


def parse_attestation(path: Path, expected_record_id: str) -> dict[str, str]:
    """Parse one exact Decision-0022 attestation file."""

    require_regular_file(path, f"attestation {expected_record_id}")
    return parse_attestation_text(
        path.read_text(encoding="utf-8"), expected_record_id
    )


def validate_completion_time(record_id: str, value: str) -> None:
    """Require an RFC3339-compatible UTC completion timestamp."""

    candidate = f"{value[:-1]}+00:00" if value.endswith("Z") else value
    try:
        parsed = datetime.fromisoformat(candidate)
    except ValueError as error:
        raise ContractError(
            f"attestation {record_id} has invalid Completed at timestamp"
        ) from error
    if parsed.tzinfo is None or parsed.utcoffset() != timezone.utc.utcoffset(parsed):
        raise ContractError(
            f"attestation {record_id} Completed at must be an explicit UTC timestamp"
        )


def validate_attestation_record_set(
    records: list[tuple[str, str, dict[str, str]]],
    expected_consolidation_range: str | None,
    required_check_references: tuple[str, ...],
    change_aware_check_prefix: str,
) -> str:
    """Validate the complete stable semantics of one canonical record set."""

    completion_record_id = records[0][0]
    review_actors: list[str] = []
    observed_consolidation_ranges: list[str] = []
    for record_id, record_kind, fields in records:
        if fields["Schema"] != ATTESTATION_SCHEMA:
            raise ContractError(f"attestation {record_id} has an unsupported Schema")
        if fields["Kind"] != ATTESTATION_KINDS[record_kind]:
            raise ContractError(
                f"attestation {record_id} Kind must be "
                f"{ATTESTATION_KINDS[record_kind]}"
            )
        if fields["Included paths"] != ATTESTATION_INCLUDED_PATHS:
            raise ContractError(
                f"attestation {record_id} has noncanonical Included paths"
            )
        if fields["Archive algorithm"] != ATTESTATION_ARCHIVE_ALGORITHM:
            raise ContractError(
                f"attestation {record_id} has noncanonical Archive algorithm"
            )
        validate_completion_time(record_id, fields["Completed at"])
        if (
            re.fullmatch(
                r"(?:[0-9a-f]{40}|[0-9a-f]{64})",
                fields["Source commit"],
            )
            is None
        ):
            raise ContractError(
                f"attestation {record_id} has invalid Source commit"
            )
        if (
            re.fullmatch(
                r"(?:[0-9a-f]{40}|[0-9a-f]{64})",
                fields["Source tree"],
            )
            is None
        ):
            raise ContractError(f"attestation {record_id} has invalid Source tree")
        if re.fullmatch(r"[0-9a-f]{64}", fields["Archive SHA-256"]) is None:
            raise ContractError(
                f"attestation {record_id} has invalid Archive SHA-256"
            )
        if fields["Disposition"] != "Pass":
            raise ContractError(f"attestation {record_id} Disposition must be Pass")
        if fields["Findings"] != "None":
            raise ContractError(f"attestation {record_id} Findings must be None")
        expected_status = (
            "MergeAuthorized" if record_kind == "completion" else "Pass"
        )
        if fields["Status"] != expected_status:
            raise ContractError(
                f"attestation {record_id} Status must be {expected_status}"
            )
        if record_kind == "completion":
            if fields["Actor"] != "Codex /root":
                raise ContractError(
                    f"attestation {completion_record_id} Actor must be Codex /root"
                )
            declaration = " ".join(fields["Declaration"].lower().split())
            if declaration != (
                "principal integrator for doc-00 merge authorization; "
                "not the accountable human or an independent reviewer."
            ):
                raise ContractError(
                    f"attestation {completion_record_id} lacks the canonical "
                    "merge-authorization declaration"
                )
        elif record_kind == "review":
            declaration = " ".join(fields["Declaration"].lower().split())
            if declaration != (
                "independent reviewer; did not author or remediate "
                "the reviewed source."
            ):
                raise ContractError(
                    f"attestation {record_id} lacks an independence declaration"
                )
            review_actors.append(fields["Actor"])
        elif record_kind == "consolidation":
            declaration = " ".join(fields["Declaration"].lower().split())
            if declaration != "integration owner for the named consolidation pass.":
                raise ContractError(
                    f"attestation {record_id} lacks an ownership declaration"
                )
            evidence_match = re.fullmatch(
                r"(FND-152\.\.[0-9]{3}) reconciliation\.",
                fields["Evidence references"],
            )
            if evidence_match is None:
                raise ContractError(
                    f"attestation {record_id} Evidence references must name "
                    "one canonical FND-152..NNN reconciliation range"
                )
            observed_range = evidence_match.group(1)
            observed_consolidation_ranges.append(observed_range)
            if (
                expected_consolidation_range is not None
                and observed_range != expected_consolidation_range
            ):
                raise ContractError(
                    f"attestation {record_id} Evidence references must be "
                    f"{expected_consolidation_range} reconciliation."
                )

    require_unique(review_actors, "review Actor")
    if len(set(observed_consolidation_ranges)) != 1:
        raise ContractError(
            "canonical consolidation attestations disagree on their "
            "finding reconciliation range"
        )
    consolidation_range = observed_consolidation_ranges[0]

    g0_evidence = records[0][2]["Evidence references"]
    for index in range(1, 4):
        record_id = f"CONSOL-{index:02d}"
        target = f"consolidations/consol-{index:02d}.md"
        link = f"[{record_id}]({target})"
        if g0_evidence.count(link) != 1:
            raise ContractError(
                f"attestation {completion_record_id} must reference "
                f"{link} exactly once"
            )
    for index in range(1, EXPECTED_REVIEW_COUNT + 1):
        record_id = f"REV-{index:02d}"
        target = f"reviews/rev-{index:02d}.md"
        link = f"[{record_id}]({target})"
        if g0_evidence.count(link) != 1:
            raise ContractError(
                f"attestation {completion_record_id} must reference "
                f"{link} exactly once"
            )
    expected_g0_links = [
        (f"CONSOL-{index:02d}", f"consolidations/consol-{index:02d}.md")
        for index in range(1, 4)
    ] + [
        (f"REV-{index:02d}", f"reviews/rev-{index:02d}.md")
        for index in range(1, EXPECTED_REVIEW_COUNT + 1)
    ]
    if markdown_links(g0_evidence) != expected_g0_links:
        raise ContractError(
            f"attestation {completion_record_id} must contain only the "
            "21 canonical "
            "sub-attestation links in registry order"
        )
    g0_code_references = re.findall(r"`([^`]+)`", g0_evidence)
    for check_reference in required_check_references:
        if g0_code_references.count(check_reference) != 1:
            raise ContractError(
                f"attestation {completion_record_id} must reference "
                "repository check "
                f"{check_reference} exactly once"
            )
    change_aware_references = [
        reference
        for reference in g0_code_references
        if reference.startswith(change_aware_check_prefix)
    ]
    change_aware_pattern = re.compile(
        rf"^{re.escape(change_aware_check_prefix)} "
        r"/[-A-Za-z0-9._/]+$"
    )
    if (
        len(change_aware_references) != 1
        or change_aware_pattern.fullmatch(change_aware_references[0]) is None
    ):
        raise ContractError(
            f"attestation {completion_record_id} must reference exactly one "
            f"change-aware repository check `{change_aware_check_prefix} "
            "/absolute/pr-body-path` without shell operators"
        )

    binding_fields = (
        "Source commit",
        "Source tree",
        "Included paths",
        "Archive algorithm",
        "Archive SHA-256",
    )
    reference = records[0][2]
    for record_id, _, fields in records[1:]:
        for field in binding_fields:
            if fields[field] != reference[field]:
                raise ContractError(
                    f"attestation {record_id} disagrees on common {field}"
                )
    return consolidation_range


def checker_literal_bindings(
    checker_text: str,
    names: set[str],
    label: str,
) -> dict[str, ast.expr]:
    """Read unique module literals while rejecting every competing binding."""

    if len(checker_text.encode("utf-8")) > 1_000_000:
        raise ContractError(f"{label} exceeds the AST inspection limit")
    try:
        module = ast.parse(checker_text)
    except (SyntaxError, ValueError) as error:
        raise ContractError(f"{label} is not valid Python") from error

    assignments: dict[str, list[tuple[ast.Name, ast.expr]]] = {
        name: [] for name in names
    }
    for statement in module.body:
        target: ast.Name | None = None
        value: ast.expr | None = None
        if (
            isinstance(statement, ast.Assign)
            and len(statement.targets) == 1
            and isinstance(statement.targets[0], ast.Name)
        ):
            target = statement.targets[0]
            value = statement.value
        elif (
            isinstance(statement, ast.AnnAssign)
            and isinstance(statement.target, ast.Name)
        ):
            target = statement.target
            value = statement.value
        if target is None or target.id not in names or value is None:
            continue
        assignments[target.id].append((target, value))

    for name, bindings in assignments.items():
        if len(bindings) != 1:
            raise ContractError(
                f"{label} must define "
                f"{name} exactly once as a module-level literal assignment"
            )

    allowed_stores = {
        id(bindings[0][0]) for bindings in assignments.values()
    }

    def reject_binding(name: str) -> None:
        raise ContractError(
            f"{label} must not rebind or mutate "
            f"{name} outside its single module-level literal assignment"
        )

    match_name_types = tuple(
        node_type
        for node_type in (
            getattr(ast, "MatchAs", None),
            getattr(ast, "MatchStar", None),
        )
        if node_type is not None
    )
    match_mapping_type = getattr(ast, "MatchMapping", None)
    dynamic_primitives = {
        "__import__",
        "compile",
        "delattr",
        "eval",
        "exec",
        "globals",
        "locals",
        "setattr",
        "vars",
    }
    builtins_aliases = {"builtins", "__builtins__"}
    sys_aliases = {"sys"}
    module_namespace_aliases: set[str] = set()
    primitive_aliases = {
        primitive: primitive for primitive in dynamic_primitives
    }

    for node in ast.walk(module):
        if isinstance(node, ast.Import):
            for alias in node.names:
                if alias.name == "builtins":
                    builtins_aliases.add(alias.asname or alias.name)
                elif alias.name == "sys":
                    sys_aliases.add(alias.asname or alias.name)
        elif isinstance(node, ast.ImportFrom) and node.module == "builtins":
            for alias in node.names:
                if alias.name in dynamic_primitives | {"getattr"}:
                    primitive_aliases[alias.asname or alias.name] = alias.name

    def static_string(node: ast.expr) -> str | None:
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            return node.value if len(node.value) <= 1_024 else None
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Add):
            left = static_string(node.left)
            right = static_string(node.right)
            if left is not None and right is not None and len(left) + len(right) <= 1_024:
                return left + right
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Mult):
            if (
                (value := static_string(node.left)) is not None
                and isinstance(node.right, ast.Constant)
                and isinstance(node.right.value, int)
                and 0 <= node.right.value <= 1_024
                and len(value) * node.right.value <= 1_024
            ):
                return value * node.right.value
            if (
                (value := static_string(node.right)) is not None
                and isinstance(node.left, ast.Constant)
                and isinstance(node.left.value, int)
                and 0 <= node.left.value <= 1_024
                and len(value) * node.left.value <= 1_024
            ):
                return value * node.left.value
        return None

    def builtins_namespace(node: ast.expr) -> bool:
        if isinstance(node, ast.Name):
            return node.id in builtins_aliases
        return (
            isinstance(node, ast.Attribute)
            and node.attr == "__dict__"
            and isinstance(node.value, ast.Name)
            and node.value.id in builtins_aliases
        )

    def current_module_object(node: ast.expr) -> bool:
        return (
            isinstance(node, ast.Subscript)
            and isinstance(node.value, ast.Attribute)
            and node.value.attr == "modules"
            and isinstance(node.value.value, ast.Name)
            and node.value.value.id in sys_aliases
            and isinstance(node.slice, ast.Name)
            and node.slice.id == "__name__"
        )

    def module_namespace(node: ast.expr) -> bool:
        if isinstance(node, ast.Name):
            return node.id in module_namespace_aliases
        return (
            isinstance(node, ast.Attribute)
            and node.attr == "__dict__"
            and current_module_object(node.value)
        )

    def referenced_primitive(node: ast.expr) -> str | None:
        if isinstance(node, ast.Name):
            return primitive_aliases.get(node.id)
        if (
            isinstance(node, ast.Attribute)
            and isinstance(node.value, ast.Name)
            and node.value.id in builtins_aliases
            and node.attr in dynamic_primitives
        ):
            return node.attr
        if (
            isinstance(node, ast.Subscript)
            and builtins_namespace(node.value)
            and (primitive := static_string(node.slice)) in dynamic_primitives
        ):
            return primitive
        if (
            isinstance(node, ast.Call)
            and referenced_primitive(node.func) == "getattr"
            and len(node.args) >= 2
            and builtins_namespace(node.args[0])
            and (primitive := static_string(node.args[1])) in dynamic_primitives
        ):
            return primitive
        return None

    # Resolve direct and transitive aliases before inspecting calls. This is
    # intentionally conservative: the checker contract is data, so it has no
    # legitimate need to alias namespace or code-execution primitives.
    primitive_aliases["getattr"] = "getattr"
    changed = True
    while changed:
        changed = False
        for node in ast.walk(module):
            target: ast.Name | None = None
            value: ast.expr | None = None
            if (
                isinstance(node, ast.Assign)
                and len(node.targets) == 1
                and isinstance(node.targets[0], ast.Name)
            ):
                target = node.targets[0]
                value = node.value
            elif isinstance(node, ast.AnnAssign) and isinstance(
                node.target, ast.Name
            ):
                target = node.target
                value = node.value
            if target is None or value is None:
                continue
            if (
                builtins_namespace(value)
                and target.id not in builtins_aliases
            ):
                builtins_aliases.add(target.id)
                changed = True
            if (
                module_namespace(value)
                and target.id not in module_namespace_aliases
            ):
                module_namespace_aliases.add(target.id)
                changed = True
            primitive = referenced_primitive(value)
            if primitive is not None and primitive_aliases.get(target.id) != primitive:
                primitive_aliases[target.id] = primitive
                changed = True

    for node in ast.walk(module):
        if isinstance(node, ast.ImportFrom) and any(
            alias.name == "*" for alias in node.names
        ):
            raise ContractError(
                f"{label} must not use wildcard imports while declaring "
                "checker contract literals"
            )
        if (
            isinstance(node, ast.Subscript)
            and builtins_namespace(node.value)
            and static_string(node.slice) is None
        ):
            raise ContractError(
                f"{label} must not access a dynamic builtins namespace value "
                "while declaring checker contract literals"
            )
        if (
            isinstance(node, ast.Subscript)
            and isinstance(node.ctx, (ast.Store, ast.Del))
            and module_namespace(node.value)
        ):
            raise ContractError(
                f"{label} must not mutate the current module namespace while "
                "declaring checker contract literals"
            )
        if (
            isinstance(node, ast.Call)
            and referenced_primitive(node.func) == "getattr"
            and len(node.args) >= 2
            and builtins_namespace(node.args[0])
            and static_string(node.args[1]) is None
        ):
            raise ContractError(
                f"{label} must not access a computed builtins attribute while "
                "declaring checker contract literals"
            )
        if isinstance(node, ast.Call) and (
            (primitive := referenced_primitive(node.func)) is not None
            and (
                primitive != "getattr"
                or not (
                    isinstance(node.func, ast.Name)
                    and node.func.id == "getattr"
                )
                or (
                    len(node.args) >= 2
                    and (
                        static_string(node.args[1]) in names
                        or static_string(node.args[1]) in dynamic_primitives
                    )
                )
            )
        ):
            raise ContractError(
                f"{label} must not use dynamic namespace primitive "
                f"{primitive} while declaring checker contract literals"
            )
        if (
            isinstance(node, ast.Name)
            and node.id in names
            and isinstance(node.ctx, (ast.Store, ast.Del))
            and id(node) not in allowed_stores
        ):
            reject_binding(node.id)
        if (
            isinstance(node, ast.Attribute)
            and node.attr in names
            and isinstance(node.ctx, (ast.Store, ast.Del))
        ):
            reject_binding(node.attr)
        if (
            isinstance(node, ast.Subscript)
            and isinstance(node.ctx, (ast.Store, ast.Del))
            and static_string(node.slice) in names
        ):
            reject_binding(static_string(node.slice) or "")
        if isinstance(node, (ast.Global, ast.Nonlocal)):
            for name in node.names:
                if name in names:
                    reject_binding(name)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            if node.name in names:
                reject_binding(node.name)
        if isinstance(node, ast.arg) and node.arg in names:
            reject_binding(node.arg)
        if isinstance(node, ast.alias):
            bound_name = node.asname or node.name.split(".", maxsplit=1)[0]
            if bound_name in names:
                reject_binding(bound_name)
        if isinstance(node, ast.ExceptHandler):
            if node.name in names:
                reject_binding(node.name)
        if match_name_types and isinstance(node, match_name_types):
            if node.name in names:
                reject_binding(node.name)
        if match_mapping_type is not None and isinstance(node, match_mapping_type):
            if node.rest in names:
                reject_binding(node.rest)

    return {
        name: assignments[name][0][1]
        for name in names
    }


def checker_g0_contract(checker_text: str, label: str) -> tuple[tuple[str, ...], str]:
    """Parse the bounded G0 contract from one checker revision."""

    assignments = checker_literal_bindings(
        checker_text,
        {
            "G0_REQUIRED_CHECK_REFERENCES",
            "G0_CHANGE_AWARE_CHECK_PREFIX",
        },
        label,
    )
    required_node = assignments["G0_REQUIRED_CHECK_REFERENCES"]
    if not isinstance(required_node, ast.Tuple):
        raise ContractError(
            f"{label} G0_REQUIRED_CHECK_REFERENCES must be "
            "a literal tuple"
        )
    required_references: list[str] = []
    for element in required_node.elts:
        if not isinstance(element, ast.Constant) or not isinstance(
            element.value, str
        ):
            raise ContractError(
                f"{label} G0_REQUIRED_CHECK_REFERENCES must contain "
                "only string literals"
            )
        if not element.value:
            raise ContractError(
                f"{label} G0_REQUIRED_CHECK_REFERENCES contains "
                "an empty value"
            )
        required_references.append(element.value)
    if not required_references:
        raise ContractError(
            f"{label} G0_REQUIRED_CHECK_REFERENCES must not be empty"
        )
    require_unique(required_references, "prior G0 required check reference")

    prefix_node = assignments["G0_CHANGE_AWARE_CHECK_PREFIX"]
    if (
        not isinstance(prefix_node, ast.Constant)
        or not isinstance(prefix_node.value, str)
        or not prefix_node.value
    ):
        raise ContractError(
            f"{label} G0_CHANGE_AWARE_CHECK_PREFIX must be "
            "one nonempty string literal"
        )
    return tuple(required_references), prefix_node.value


def checker_protected_digests(checker_text: str, label: str) -> dict[str, str]:
    """Parse the protected-history digests from one checker revision."""

    names = {
        "EXPECTED_PROTECTED_CONFORMANCE_SHA256",
        "EXPECTED_PROTECTED_FINDING_SHA256",
    }
    assignments = checker_literal_bindings(checker_text, names, label)
    digests: dict[str, str] = {}
    for name in sorted(names):
        node = assignments[name]
        if (
            not isinstance(node, ast.Constant)
            or not isinstance(node.value, str)
            or re.fullmatch(r"[0-9a-f]{64}", node.value) is None
        ):
            raise ContractError(
                f"{label} {name} must be one 64-character lowercase "
                "SHA-256 string literal"
            )
        digests[name] = node.value
    return digests


def source_checker_text(repository_root: Path, source_commit: str) -> str:
    """Read one checker revision from Git without executing it."""

    return git_blob_text(
        repository_root,
        source_commit,
        SOURCE_GOVERNANCE_PATHS[0].as_posix(),
    )


def source_checker_g0_contract(
    repository_root: Path,
    source_commit: str,
) -> tuple[tuple[str, ...], str]:
    """Read historical G0 literals from the bound checker without executing it."""

    return checker_g0_contract(
        source_checker_text(repository_root, source_commit),
        "prior source checker",
    )


def source_checker_protected_digests(
    repository_root: Path,
    source_commit: str,
) -> dict[str, str]:
    """Read historical protected digests without executing checker code."""

    return checker_protected_digests(
        source_checker_text(repository_root, source_commit),
        "source checker",
    )


def source_finding_range(repository_root: Path, source_commit: str) -> str:
    """Derive the append-only finding range from one attested source."""

    source_text = git_blob_text(
        repository_root,
        source_commit,
        DELIVERY_PROGRAM_PATH.as_posix(),
    )
    finding_ids = [
        int(match.group(1))
        for match in re.finditer(
            r"^\| `FND-([0-9]{3})` / P[0-3] \|",
            source_text,
            re.MULTILINE,
        )
    ]
    if not finding_ids or finding_ids != list(range(1, len(finding_ids) + 1)):
        raise ContractError(
            "prior attested source has a noncanonical finding sequence"
        )
    if finding_ids[-1] < 152:
        raise ContractError(
            "prior attested source ends before the current finding range"
        )
    return f"FND-152..{finding_ids[-1]:03d}"


def delivery_history_entries(
    text: str,
) -> tuple[list[tuple[int, str]], list[tuple[int, str]]]:
    """Extract canonical finding rows and conformance sections from one source."""

    finding_entries = [
        (int(match.group(1)), match.group(0))
        for match in FINDING_HISTORY_PATTERN.finditer(text)
    ]
    if not finding_entries:
        raise ContractError("delivery-program finding history is empty")
    require_contiguous(
        [identifier for identifier, _ in finding_entries],
        len(finding_entries),
        "delivery-program finding-history IDs",
    )

    conformance_matches = list(CONFORMANCE_HISTORY_PATTERN.finditer(text))
    if not conformance_matches:
        raise ContractError("delivery-program conformance history is empty")
    conformance_ids = [
        int(match.group(1) or "01") for match in conformance_matches
    ]
    require_contiguous(
        conformance_ids,
        len(conformance_ids),
        "delivery-program conformance-history IDs",
    )
    final_boundary = text.find(
        REQUIRED_REPOSITORY_CHECKS_HEADING,
        conformance_matches[-1].end(),
    )
    if final_boundary < 0:
        raise ContractError(
            "delivery-program conformance history lacks its final boundary"
        )
    conformance_entries: list[tuple[int, str]] = []
    for index, (identifier, match) in enumerate(
        zip(conformance_ids, conformance_matches)
    ):
        end = (
            conformance_matches[index + 1].start()
            if index + 1 < len(conformance_matches)
            else final_boundary
        )
        conformance_entries.append(
            (identifier, text[match.start():end].rstrip("\r\n"))
        )
    return finding_entries, conformance_entries


def validate_append_only_entries(
    predecessor: list[tuple[int, str]],
    successor: list[tuple[int, str]],
    label: str,
    identifier_prefix: str,
    identifier_width: int,
) -> None:
    """Require an existing ordered history to remain an exact prefix."""

    for index, (identifier, predecessor_text) in enumerate(predecessor):
        formatted_identifier = (
            f"{identifier_prefix}-{identifier:0{identifier_width}d}"
        )
        if index >= len(successor) or successor[index][0] != identifier:
            raise ContractError(
                f"previously attested {label} {formatted_identifier} "
                "is missing or reordered"
            )
        if successor[index][1] != predecessor_text:
            raise ContractError(
                f"previously attested {label} {formatted_identifier} was rewritten"
            )


def validate_delivery_history_append_only(
    repository_root: Path,
    predecessor_source: str,
    successor_source: str,
) -> None:
    """Require one source revision to append, never rewrite, ledger history."""

    predecessor_text = git_blob_text(
        repository_root,
        predecessor_source,
        DELIVERY_PROGRAM_PATH.as_posix(),
    )
    successor_text = git_blob_text(
        repository_root,
        successor_source,
        DELIVERY_PROGRAM_PATH.as_posix(),
    )
    predecessor_findings, predecessor_conformances = delivery_history_entries(
        predecessor_text
    )
    successor_findings, successor_conformances = delivery_history_entries(
        successor_text
    )
    validate_append_only_entries(
        predecessor_findings,
        successor_findings,
        "finding",
        "FND",
        3,
    )
    validate_append_only_entries(
        predecessor_conformances,
        successor_conformances,
        "conformance receipt",
        "DOC-CONF",
        2,
    )


def reviewed_archive_changed(
    repository_root: Path,
    predecessor_source: str,
    successor_source: str,
) -> bool:
    """Return whether one source revision changes the reviewed archive paths."""

    return any(
        git_optional_tree_entry(repository_root, predecessor_source, relative_path)
        != git_optional_tree_entry(repository_root, successor_source, relative_path)
        for relative_path in ("docs/specifications", "docs/decisions")
    )


def validate_conformance_successor_for_changed_archive(
    repository_root: Path,
    predecessor_source: str,
    successor_source: str,
) -> None:
    """Bind source-conformance growth exactly to reviewed-archive change."""

    predecessor_text = git_blob_text(
        repository_root,
        predecessor_source,
        DELIVERY_PROGRAM_PATH.as_posix(),
    )
    successor_text = git_blob_text(
        repository_root,
        successor_source,
        DELIVERY_PROGRAM_PATH.as_posix(),
    )
    _, predecessor_conformances = delivery_history_entries(predecessor_text)
    _, successor_conformances = delivery_history_entries(successor_text)
    expected_identifier = len(predecessor_conformances) + 1
    changed = reviewed_archive_changed(
        repository_root,
        predecessor_source,
        successor_source,
    )
    if not changed:
        if len(successor_conformances) != len(predecessor_conformances):
            raise ContractError(
                "unchanged reviewed archive must not append a conformance receipt"
            )
        return
    if (
        len(successor_conformances) != expected_identifier
        or successor_conformances[-1][0] != expected_identifier
    ):
        raise ContractError(
            "changed reviewed archive must append exactly one next conformance "
            f"receipt DOC-CONF-{expected_identifier:02d}"
        )


def commit_parents(repository_root: Path, commit: str) -> tuple[str, ...]:
    """Return the direct parents of one full commit identity."""

    commit_line = git_output(
        repository_root,
        "rev-list",
        "--parents",
        "-n",
        "1",
        commit,
    ).split()
    if not commit_line or commit_line[0] != commit:
        raise ContractError(f"cannot resolve commit parents for {commit}")
    return tuple(commit_line[1:])


def canonical_attestation_entries(
    repository_root: Path,
    commit: str,
    expected_records: list[tuple[str, str, str]],
) -> dict[str, str]:
    """Read all canonical receipt tree entries with one bounded Git query."""

    expected_paths = tuple(
        relative_path for relative_path, _, _ in expected_records
    )
    entries = {relative_path: "" for relative_path in expected_paths}
    output = git_output(
        repository_root,
        "ls-tree",
        commit,
        "--",
        *expected_paths,
    )
    for line in output.splitlines():
        metadata, separator, relative_path = line.partition("\t")
        if (
            not separator
            or relative_path not in entries
            or entries[relative_path]
            or len(metadata.split()) != 3
        ):
            raise ContractError(
                "cannot resolve unique canonical attestation tree entries"
            )
        entries[relative_path] = line
    return entries


def attestation_entry_key(
    entries: dict[str, str],
    expected_records: list[tuple[str, str, str]],
) -> tuple[tuple[str, str], ...]:
    """Return canonical receipt entries in path-contract order."""

    return tuple(
        (relative_path, entries[relative_path])
        for relative_path, _, _ in expected_records
    )


def validate_replacement_bindings(
    records: dict[str, dict[str, str]],
    predecessor: AttestationHistoryState | None,
    label: str,
) -> None:
    """Require one record set to replace exactly its validated predecessor."""

    for record_id, fields in records.items():
        replaces = fields["Replaces"]
        if predecessor is None:
            if replaces != "None":
                raise ContractError(
                    f"first {label} {record_id} must use Replaces: None"
                )
            continue
        predecessor_digest = predecessor.records[record_id]["Archive SHA-256"]
        expected_replaces = (
            f"{record_id} at archive digest {predecessor_digest}"
        )
        if replaces != expected_replaces:
            raise ContractError(
                f"{label} {record_id} Replaces must name prior Record ID "
                f"{record_id} and archive digest {predecessor_digest}"
            )
        if fields["Archive SHA-256"] == predecessor_digest:
            raise ContractError(
                f"replacement {label} {record_id} must bind a new archive digest"
            )


def validate_historical_evidence_pair(
    repository_root: Path,
    evidence_commit: str,
    source_commit: str,
    source_state: AttestationHistoryState | None,
    tree_entries: dict[str, str],
    expected_records: list[tuple[str, str, str]],
) -> AttestationHistoryState:
    """Validate one historical source/evidence pair without executing its code."""

    records: dict[str, dict[str, str]] = {}
    record_list: list[tuple[str, str, dict[str, str]]] = []
    for relative_path, record_id, record_kind in expected_records:
        prior_text = git_blob_text(
            repository_root,
            evidence_commit,
            relative_path,
        )
        fields = parse_attestation_text(prior_text, record_id)
        records[record_id] = fields
        record_list.append((record_id, record_kind, fields))
    reference = record_list[0][2]

    if reference["Source commit"] != source_commit:
        raise ContractError(
            "prior canonical attestation evidence commit must directly follow "
            "its recorded Source commit"
        )
    if (
        re.fullmatch(r"(?:[0-9a-f]{40}|[0-9a-f]{64})", source_commit)
        is None
    ):
        raise ContractError(
            f"prior attestation {reference['Record ID']} has invalid Source commit"
        )
    object_type = git_output(repository_root, "cat-file", "-t", source_commit)
    if object_type != "commit":
        raise ContractError(
            "prior attested Source commit is not a Git commit object"
        )
    resolved_commit = git_output(
        repository_root,
        "rev-parse",
        "--verify",
        f"{source_commit}^{{commit}}",
    )
    if resolved_commit != source_commit:
        raise ContractError(
            "prior attested Source commit must be the full Git object ID"
        )

    for governance_path in SOURCE_GOVERNANCE_PATHS:
        relative_path = governance_path.as_posix()
        try:
            governance_entry = git_tree_entry(
                repository_root,
                source_commit,
                relative_path,
            )
        except ContractError as error:
            raise ContractError(
                "prior attested Source commit is missing governance program "
                f"{relative_path}"
            ) from error
        if not governance_entry.startswith("100755 blob "):
            raise ContractError(
                "prior attested governance program must be an executable "
                f"regular file: {relative_path}"
            )
    readme_relative_path = (RECEIPT_DIRECTORY / "README.md").as_posix()
    try:
        readme_entry = git_tree_entry(
            repository_root,
            source_commit,
            readme_relative_path,
        )
    except ContractError as error:
        raise ContractError(
            "prior attested Source commit is missing DOC-00 receipt README"
        ) from error
    if not readme_entry.startswith("100644 blob "):
        raise ContractError(
            "prior DOC-00 receipt README must be a non-executable regular file"
        )
    prior_readme_text = git_blob_text(
        repository_root,
        source_commit,
        readme_relative_path,
    )
    validate_receipt_readme_text(prior_readme_text)

    source_checker_digests = source_checker_protected_digests(
        repository_root,
        source_commit,
    )
    source_delivery_bytes = git_blob_bytes(
        repository_root,
        source_commit,
        DELIVERY_PROGRAM_PATH.as_posix(),
    )
    validate_protected_byte_region(
        source_delivery_bytes,
        PROTECTED_CONFORMANCE_START,
        PROTECTED_CONFORMANCE_END,
        source_checker_digests["EXPECTED_PROTECTED_CONFORMANCE_SHA256"],
        "prior DOC-CONF-01..21 history",
    )
    validate_protected_byte_region(
        source_delivery_bytes,
        PROTECTED_FINDING_START,
        PROTECTED_FINDING_END,
        source_checker_digests["EXPECTED_PROTECTED_FINDING_SHA256"],
        "prior FND-001..151 ledger",
    )

    required_checks, change_aware_prefix = source_checker_g0_contract(
        repository_root,
        source_commit,
    )
    consolidation_range = validate_attestation_record_set(
        record_list,
        None,
        required_checks,
        change_aware_prefix,
    )
    actual_tree = git_output(
        repository_root,
        "rev-parse",
        f"{source_commit}^{{tree}}",
    )
    if actual_tree != reference["Source tree"]:
        raise ContractError(
            "prior attested Source tree does not match Source commit"
        )
    expected_finding_range = source_finding_range(repository_root, source_commit)
    if consolidation_range != expected_finding_range:
        raise ContractError(
            "prior consolidation attestations do not match their attested "
            f"source finding range {expected_finding_range}"
        )

    evidence_parents = commit_parents(repository_root, evidence_commit)
    if evidence_parents != (source_commit,):
        raise ContractError(
            "prior canonical attestation evidence commit must have exactly "
            "its attested Source commit as parent"
        )
    expected_changed_paths = {
        relative_path for relative_path, _, _ in expected_records
    }
    changed_paths_output = git_output(
        repository_root,
        "diff",
        "--no-renames",
        "--name-only",
        f"{source_commit}..{evidence_commit}",
    )
    changed_paths = (
        set(changed_paths_output.splitlines()) if changed_paths_output else set()
    )
    if changed_paths != expected_changed_paths:
        missing = sorted(expected_changed_paths.difference(changed_paths))
        extra = sorted(changed_paths.difference(expected_changed_paths))
        raise ContractError(
            "prior changes after the attested Source commit must be exactly "
            "the 22 canonical attestation files; "
            f"missing: {missing}; extra: {extra}"
        )
    for relative_path, record_id, _ in expected_records:
        evidence_entry = tree_entries[relative_path]
        if not evidence_entry.startswith("100644 blob "):
            raise ContractError(
                f"prior attestation {record_id} must be a non-executable regular file"
            )

    source_parents = commit_parents(repository_root, source_commit)
    if len(source_parents) > 1:
        raise ContractError(
            "prior attested Source commit must not be a merge commit"
        )
    source_parent = source_parents[0] if source_parents else None
    for relative_path, _, _ in expected_records:
        source_entry = git_optional_tree_entry(
            repository_root,
            source_commit,
            relative_path,
        )
        parent_entry = (
            ""
            if source_parent is None
            else git_optional_tree_entry(
                repository_root,
                source_parent,
                relative_path,
            )
        )
        if source_entry != parent_entry:
            raise ContractError(
                "prior attested Source commit must preserve canonical "
                f"attestation byte-for-byte: {relative_path}"
            )

    if source_state is not None:
        validate_delivery_history_append_only(
            repository_root,
            source_state.source_commit,
            source_commit,
        )
    validate_replacement_bindings(records, source_state, "prior attestation")

    actual_digest = git_archive_digest(repository_root, source_commit)
    if actual_digest != reference["Archive SHA-256"]:
        raise ContractError(
            "prior attested Archive SHA-256 does not match the reconstructed "
            "Git archive"
        )
    if source_state is not None:
        validate_conformance_successor_for_changed_archive(
            repository_root,
            source_state.source_commit,
            source_commit,
        )
    return AttestationHistoryState(
        records=records,
        source_commit=source_commit,
        evidence_commit=evidence_commit,
        tree_entries=attestation_entry_key(tree_entries, expected_records),
    )


def validate_attestation_history(
    repository_root: Path,
    history_head: str | None,
    expected_records: list[tuple[str, str, str]],
) -> AttestationHistoryState | None:
    """Validate every reachable canonical attestation state in the Git DAG."""

    if history_head is None:
        return None

    history_lines = git_output(
        repository_root,
        "rev-list",
        "--reverse",
        "--topo-order",
        "--parents",
        history_head,
    ).splitlines()
    states: dict[str, AttestationHistoryState | None] = {}
    entries_by_receipt_tree: dict[str, dict[str, str]] = {}

    for history_line in history_lines:
        commit, *parents = history_line.split()
        missing_parents = [parent for parent in parents if parent not in states]
        if missing_parents:
            raise ContractError(
                "attestation history traversal encountered unresolved parents: "
                + ", ".join(missing_parents)
            )
        parent_states = [states[parent] for parent in parents]

        receipt_tree = git_optional_tree_entry(
            repository_root,
            commit,
            RECEIPT_DIRECTORY.as_posix(),
        )
        if receipt_tree not in entries_by_receipt_tree:
            entries_by_receipt_tree[receipt_tree] = canonical_attestation_entries(
                repository_root,
                commit,
                expected_records,
            )
        tree_entries = entries_by_receipt_tree[receipt_tree]
        present_paths = {
            relative_path
            for relative_path, entry in tree_entries.items()
            if entry
        }

        if not present_paths:
            if any(state is not None for state in parent_states):
                raise ContractError(
                    "prior canonical attestation set is absent after earlier "
                    f"canonical history at commit {commit}"
                )
            states[commit] = None
            continue
        if len(present_paths) != len(expected_records):
            raise ContractError(
                "prior canonical attestation set must be wholly absent or contain "
                f"all 22 records at commit {commit}"
            )
        for relative_path, entry in tree_entries.items():
            if not entry.startswith("100644 blob "):
                raise ContractError(
                    "prior canonical attestation must be a non-executable regular "
                    f"file: {relative_path}"
                )

        current_entry_key = attestation_entry_key(tree_entries, expected_records)
        matching_parent_states = [
            state
            for state in parent_states
            if state is not None and state.tree_entries == current_entry_key
        ]
        nonmatching_parent_states = [
            state
            for state in parent_states
            if state is None or state.tree_entries != current_entry_key
        ]

        if matching_parent_states:
            selected_state = matching_parent_states[0]
            if len(parents) > 1 and nonmatching_parent_states:
                second_parent_state = (
                    parent_states[1] if len(parent_states) == 2 else None
                )
                if (
                    len(parents) != 2
                    or second_parent_state is None
                    or second_parent_state.tree_entries != current_entry_key
                    or parents[1] != second_parent_state.evidence_commit
                ):
                    raise ContractError(
                        "nonidentical attestation histories require a two-parent "
                        "preserving merge whose second parent is the selected "
                        f"canonical evidence commit: {commit}"
                    )
                validate_replacement_bindings(
                    second_parent_state.records,
                    parent_states[0],
                    "preserving merge attestation",
                )
                selected_state = second_parent_state
            states[commit] = selected_state
            continue

        if len(parents) != 1:
            raise ContractError(
                "a complete canonical attestation state may change only in a "
                f"one-parent evidence commit: {commit}"
            )
        states[commit] = validate_historical_evidence_pair(
            repository_root,
            commit,
            parents[0],
            parent_states[0],
            tree_entries,
            expected_records,
        )

    state = states.get(history_head)
    if state is None:
        return None
    return state


def validate_receipt_readme_text(text: str) -> None:
    """Require README text to declare the exact strict attestation schema."""

    declared_fields = re.findall(r"^[0-9]+\. `([^`]+)`$", text, re.MULTILINE)
    if tuple(declared_fields) != ATTESTATION_FIELDS:
        raise ContractError(
            f"docs/receipts/README.md must declare the exact "
            f"{len(ATTESTATION_FIELDS)}-field "
            "doc00-attestation-v1 schema"
        )
    required_fragments = (
        "`doc-00-g0.md`",
        "`consolidations/consol-01.md`",
        "`consol-03.md`",
        "`reviews/rev-01.md`",
        "`rev-18.md`",
        f"`{ATTESTATION_SCHEMA}`",
        f"`{ATTESTATION_INCLUDED_PATHS}`",
        f"`{ATTESTATION_ARCHIVE_ALGORITHM}`",
    )
    for fragment in required_fragments:
        if fragment not in text:
            raise ContractError(
                f"DOC-00 receipt README is missing canonical contract: {fragment}"
            )
    normalized_text = " ".join(text.split())
    required_normalized_fragments = (
        "`Status: MergeAuthorized`",
        "`MergeAuthorization`",
        ATTESTATION_ARCHIVE_COMMAND,
        (
            "`Independent reviewer; did not author or remediate "
            "the reviewed source.`"
        ),
        "`Integration owner for the named consolidation pass.`",
        (
            "`Principal integrator for DOC-00 merge authorization; "
            "not the accountable human or an independent reviewer.`"
        ),
        ATTESTATION_HISTORY_CONTRACT,
    )
    for fragment in required_normalized_fragments:
        if fragment not in normalized_text:
            raise ContractError(
                "DOC-00 receipt README is missing canonical strict value: "
                f"{fragment}"
            )


def validate_receipt_readme(repository_root: Path) -> None:
    """Validate the local canonical receipt README file."""

    readme_path = repository_root / RECEIPT_DIRECTORY / "README.md"
    require_regular_file(readme_path, "DOC-00 receipt README")
    validate_receipt_readme_text(readme_path.read_text(encoding="utf-8"))


def validate_receipts(
    repository_root: Path,
    required_check_references: tuple[str, ...],
    change_aware_check_prefix: str,
) -> None:
    """Validate complete content-bound DOC-00 receipt evidence."""

    validate_git_history_view(repository_root)
    validate_receipt_readme(repository_root)
    receipt_root = repository_root / RECEIPT_DIRECTORY
    expected_records: list[tuple[Path, str, str]] = [
        (receipt_root / "doc-00-g0.md", CANONICAL_G0_RECORD_ID, "completion")
    ]
    expected_records.extend(
        (
            receipt_root / "consolidations" / f"consol-{index:02d}.md",
            f"CONSOL-{index:02d}",
            "consolidation",
        )
        for index in range(1, 4)
    )
    expected_records.extend(
        (
            receipt_root / "reviews" / f"rev-{index:02d}.md",
            f"REV-{index:02d}",
            "review",
        )
        for index in range(1, EXPECTED_REVIEW_COUNT + 1)
    )
    history_records = [
        (path.relative_to(repository_root).as_posix(), record_id, record_kind)
        for path, record_id, record_kind in expected_records
    ]

    for directory in (
        receipt_root,
        receipt_root / "consolidations",
        receipt_root / "reviews",
    ):
        if directory.is_symlink() or not directory.is_dir():
            raise ContractError(
                f"missing or non-regular attestation directory: {directory}"
            )
    expected_markdown_paths = {
        (receipt_root / "README.md").relative_to(repository_root).as_posix(),
        *(
            path.relative_to(repository_root).as_posix()
            for path, _, _ in expected_records
        ),
    }
    actual_markdown_paths = {
        path.relative_to(repository_root).as_posix()
        for path in receipt_root.rglob("*.md")
    }
    if actual_markdown_paths != expected_markdown_paths:
        missing = sorted(expected_markdown_paths.difference(actual_markdown_paths))
        extra = sorted(actual_markdown_paths.difference(expected_markdown_paths))
        raise ContractError(
            "canonical attestation file set differs from the README contract; "
            f"missing: {missing}; extra: {extra}"
        )

    readme_relative_path = (receipt_root / "README.md").relative_to(
        repository_root
    ).as_posix()
    tracked_readme = git_output(
        repository_root, "ls-files", "--full-name", "--", readme_relative_path
    )
    if tracked_readme != readme_relative_path:
        raise ContractError(
            "DOC-00 receipt README is not tracked at its canonical path"
        )

    records: list[tuple[str, str, dict[str, str]]] = []
    for path, record_id, record_kind in expected_records:
        relative_path = path.relative_to(repository_root).as_posix()
        tracked = git_output(
            repository_root, "ls-files", "--full-name", "--", relative_path
        )
        if tracked != relative_path:
            raise ContractError(f"attestation is not tracked at its canonical path: {path}")
        fields = parse_attestation(path, record_id)
        records.append((record_id, record_kind, fields))

    validate_attestation_record_set(
        records,
        EXPECTED_CURRENT_FINDING_RANGE,
        required_check_references,
        change_aware_check_prefix,
    )
    reference = records[0][2]

    source_commit = reference["Source commit"]
    source_tree = reference["Source tree"]
    object_type = git_output(repository_root, "cat-file", "-t", source_commit)
    if object_type != "commit":
        raise ContractError("attested Source commit is not a Git commit object")
    resolved_commit = git_output(
        repository_root, "rev-parse", "--verify", f"{source_commit}^{{commit}}"
    )
    if resolved_commit != source_commit:
        raise ContractError("attested Source commit must be the full Git object ID")
    source_commit_line = git_output(
        repository_root, "rev-list", "--parents", "-n", "1", source_commit
    ).split()
    if len(source_commit_line) == 1:
        source_parent: str | None = None
    elif len(source_commit_line) == 2:
        source_parent = source_commit_line[1]
    else:
        raise ContractError(
            "attested Source commit must not be a merge commit"
        )
    prior_history = validate_attestation_history(
        repository_root,
        source_parent,
        history_records,
    )
    if prior_history is None:
        prior_records = None
    else:
        prior_records = prior_history.records
        validate_delivery_history_append_only(
            repository_root,
            prior_history.source_commit,
            source_commit,
        )
    actual_tree = git_output(repository_root, "rev-parse", f"{source_commit}^{{tree}}")
    if actual_tree != source_tree:
        raise ContractError("attested Source tree does not match Source commit")
    for governance_path in SOURCE_GOVERNANCE_PATHS:
        relative_path = governance_path.as_posix()
        try:
            source_entry = git_tree_entry(
                repository_root, source_commit, relative_path
            )
            head_entry = git_tree_entry(repository_root, "HEAD", relative_path)
        except ContractError as error:
            raise ContractError(
                "attested Source commit is missing governance program "
                f"{relative_path}"
            ) from error
        if not source_entry.startswith("100755 blob "):
            raise ContractError(
                "attested governance program must be an executable regular "
                f"file: {relative_path}"
            )
        if source_entry != head_entry:
            raise ContractError(
                f"governance program differs from Source commit: {relative_path}"
            )
    governance_status = git_output(
        repository_root,
        "status",
        "--porcelain",
        "--untracked-files=all",
        "--",
        *(path.as_posix() for path in SOURCE_GOVERNANCE_PATHS),
    )
    if governance_status:
        raise ContractError("attested governance programs have uncommitted changes")
    canonical_checker = (
        repository_root / SOURCE_GOVERNANCE_PATHS[0]
    ).resolve()
    if Path(__file__).resolve() != canonical_checker:
        raise ContractError(
            "strict receipt validation must execute the repository's canonical "
            "scripts/check-v1-delivery-program.py"
        )
    try:
        subprocess.run(
            git_command(
                repository_root,
                "merge-base",
                "--is-ancestor",
                source_commit,
                "HEAD",
            ),
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            env=git_environment(),
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise ContractError("attested Source commit is not an ancestor of HEAD") from error

    evidence_commits = {
        git_output(
            repository_root,
            "log",
            "-1",
            "--format=%H",
            "HEAD",
            "--",
            path.relative_to(repository_root).as_posix(),
        )
        for path, _, _ in expected_records
    }
    if "" in evidence_commits or len(evidence_commits) != 1:
        raise ContractError(
            "all 22 canonical attestations must share one last-modified "
            "evidence commit"
        )
    evidence_commit = evidence_commits.pop()
    evidence_commit_line = git_output(
        repository_root, "rev-list", "--parents", "-n", "1", evidence_commit
    ).split()
    if len(evidence_commit_line) != 2 or evidence_commit_line[1] != source_commit:
        raise ContractError(
            "the canonical attestation evidence commit must have exactly "
            "the attested Source commit as its parent"
        )
    try:
        subprocess.run(
            git_command(
                repository_root,
                "merge-base",
                "--is-ancestor",
                evidence_commit,
                "HEAD",
            ),
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            env=git_environment(),
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise ContractError(
            "canonical attestation evidence commit is not an ancestor of HEAD"
        ) from error
    changed_paths_output = git_output(
        repository_root,
        "diff",
        "--name-only",
        f"{source_commit}..{evidence_commit}",
    )
    changed_paths = set(changed_paths_output.splitlines()) if changed_paths_output else set()
    expected_changed_paths = {
        path.relative_to(repository_root).as_posix()
        for path, _, _ in expected_records
    }
    if changed_paths != expected_changed_paths:
        missing = sorted(expected_changed_paths.difference(changed_paths))
        extra = sorted(changed_paths.difference(expected_changed_paths))
        raise ContractError(
            "changes after the attested Source commit must be exactly the "
            "22 canonical attestation files; "
            f"missing: {missing}; extra: {extra}"
        )
    readme_path = (receipt_root / "README.md").relative_to(
        repository_root
    ).as_posix()
    source_readme = git_tree_entry(repository_root, source_commit, readme_path)
    head_readme = git_tree_entry(repository_root, "HEAD", readme_path)
    if not source_readme.startswith("100644 blob "):
        raise ContractError(
            "DOC-00 receipt README must be a non-executable regular file"
        )
    if source_readme != head_readme:
        raise ContractError(
            "DOC-00 receipt README differs from the attested Source commit"
        )
    for path, record_id, _ in expected_records:
        relative_path = path.relative_to(repository_root).as_posix()
        evidence_entry = git_tree_entry(
            repository_root, evidence_commit, relative_path
        )
        head_entry = git_tree_entry(repository_root, "HEAD", relative_path)
        if not evidence_entry.startswith("100644 blob "):
            raise ContractError(
                f"attestation {record_id} must be a non-executable regular file"
            )
        if evidence_entry != head_entry:
            raise ContractError(
                f"attestation {record_id} differs from the evidence commit"
            )

    for (path, record_id, _), (_, _, fields) in zip(expected_records, records):
        relative_path = path.relative_to(repository_root).as_posix()
        source_entry = git_optional_tree_entry(
            repository_root, source_commit, relative_path
        )
        parent_entry = (
            ""
            if source_parent is None
            else git_optional_tree_entry(
                repository_root, source_parent, relative_path
            )
        )
        if source_entry != parent_entry:
            raise ContractError(
                "attested Source commit must preserve prior canonical "
                f"attestation byte-for-byte: {relative_path}"
            )
        replaces = fields["Replaces"]
        if prior_records is not None:
            prior_digest = prior_records[record_id]["Archive SHA-256"]
            expected_replaces = f"{record_id} at archive digest {prior_digest}"
            if replaces != expected_replaces:
                raise ContractError(
                    f"attestation {record_id} Replaces must name prior Record ID "
                    f"{record_id} and archive digest {prior_digest}"
                )
            if fields["Archive SHA-256"] == prior_digest:
                raise ContractError(
                    f"replacement attestation {record_id} must bind a new "
                    "archive digest"
                )
        elif replaces != "None":
            raise ContractError(
                f"first attestation {record_id} must use Replaces: None"
            )

    included_status = git_output(
        repository_root,
        "status",
        "--porcelain",
        "--untracked-files=all",
        "--",
        "docs/specifications",
        "docs/decisions",
    )
    if included_status:
        raise ContractError("attested included paths have uncommitted changes")
    for included_path in ("docs/specifications", "docs/decisions"):
        source_object = git_output(
            repository_root, "rev-parse", f"{source_commit}:{included_path}"
        )
        head_object = git_output(repository_root, "rev-parse", f"HEAD:{included_path}")
        if source_object != head_object:
            raise ContractError(
                f"attested source differs from HEAD at {included_path}"
            )

    actual_digest = git_archive_digest(repository_root, source_commit)
    if actual_digest != reference["Archive SHA-256"]:
        raise ContractError(
            "attested Archive SHA-256 does not match the reconstructed Git archive"
        )
    if prior_history is not None:
        validate_conformance_successor_for_changed_archive(
            repository_root,
            prior_history.source_commit,
            source_commit,
        )
    receipt_status = git_output(
        repository_root,
        "status",
        "--porcelain",
        "--untracked-files=all",
        "--",
        "docs/receipts",
    )
    if receipt_status:
        raise ContractError("canonical DOC-00 attestations have uncommitted changes")
    head_commit = git_output(repository_root, "rev-parse", "--verify", "HEAD^{commit}")
    if head_commit != evidence_commit:
        merge_candidates = git_output(
            repository_root,
            "rev-list",
            "--merges",
            "--ancestry-path",
            f"{evidence_commit}..{head_commit}",
        ).splitlines()
        preserving_merges: list[str] = []
        for merge_commit in merge_candidates:
            merge_line = git_output(
                repository_root,
                "rev-list",
                "--parents",
                "-n",
                "1",
                merge_commit,
            ).split()
            if len(merge_line) == 3 and merge_line[2] == evidence_commit:
                preserving_merges.append(merge_commit)
        if len(preserving_merges) != 1:
            raise ContractError(
                "HEAD after the evidence commit must descend from a "
                "two-parent merge commit whose second parent is the exact "
                "evidence commit"
            )
        later_commits = git_output(
            repository_root,
            "rev-list",
            "--ancestry-path",
            f"{preserving_merges[0]}..{head_commit}",
        ).splitlines()
        for later_commit in later_commits:
            commit_line = git_output(
                repository_root,
                "rev-list",
                "--parents",
                "-n",
                "1",
                later_commit,
            ).split()
            if len(commit_line) < 2:
                raise ContractError(
                    "history after the preserving merge contains an "
                    "unresolvable commit parent"
                )
            bound_changes = git_output(
                repository_root,
                "diff",
                "--name-only",
                commit_line[1],
                later_commit,
                "--",
                "docs/specifications",
                "docs/decisions",
                "docs/receipts",
                *(path.as_posix() for path in SOURCE_GOVERNANCE_PATHS),
            )
            if bound_changes:
                raise ContractError(
                    "history after the preserving merge must not modify a "
                    "DOC-00-bound path"
                )

    head_history = validate_attestation_history(
        repository_root,
        head_commit,
        history_records,
    )
    if (
        head_history is None
        or head_history.source_commit != source_commit
        or head_history.evidence_commit != evidence_commit
    ):
        raise ContractError(
            "HEAD history does not resolve to the current canonical "
            "source/evidence pair"
        )


def validate(path: Path, require_receipts: bool = False) -> None:
    """Validate all mechanically checkable DOC-00 registry invariants."""

    repository_root = repository_root_for(path)
    checker_path = repository_root / SOURCE_GOVERNANCE_PATHS[0]
    require_regular_file(checker_path, "V1 delivery-program checker")
    checker_text = checker_path.read_text(encoding="utf-8")
    parsed_protected_digests = checker_protected_digests(
        checker_text,
        "current source checker",
    )
    runtime_protected_digests = {
        "EXPECTED_PROTECTED_CONFORMANCE_SHA256": (
            EXPECTED_PROTECTED_CONFORMANCE_SHA256
        ),
        "EXPECTED_PROTECTED_FINDING_SHA256": EXPECTED_PROTECTED_FINDING_SHA256,
    }
    if parsed_protected_digests != runtime_protected_digests:
        raise ContractError(
            "current protected-history digest literals differ from their "
            "runtime bindings"
        )
    parsed_required_checks, parsed_change_aware_prefix = checker_g0_contract(
        checker_text,
        "current source checker",
    )
    if (
        parsed_required_checks != G0_REQUIRED_CHECK_REFERENCES
        or parsed_change_aware_prefix != G0_CHANGE_AWARE_CHECK_PREFIX
    ):
        raise ContractError(
            "current G0 contract literals differ from their runtime bindings"
        )
    protected_conformance_digest = parsed_protected_digests[
        "EXPECTED_PROTECTED_CONFORMANCE_SHA256"
    ]
    protected_finding_digest = parsed_protected_digests[
        "EXPECTED_PROTECTED_FINDING_SHA256"
    ]
    source_bytes = canonical_git_text_bytes(path.read_bytes())
    validate_protected_byte_region(
        source_bytes,
        PROTECTED_CONFORMANCE_START,
        PROTECTED_CONFORMANCE_END,
        protected_conformance_digest,
        "DOC-CONF-01..21 history",
    )
    validate_protected_byte_region(
        source_bytes,
        PROTECTED_FINDING_START,
        PROTECTED_FINDING_END,
        protected_finding_digest,
        "FND-001..151 ledger",
    )
    text = path.read_text(encoding="utf-8")
    packages, dependencies = parse_packages(text)
    expected_packages = set(packages)
    parse_package_registry(
        text,
        "### Work-package interface and execution metadata",
        "### Work-package responsibility registry",
        expected_packages,
        "execution-metadata",
    )
    parse_package_registry(
        text,
        "### Work-package responsibility registry",
        "### Interface and artifact ownership registry",
        expected_packages,
        "responsibility",
    )
    parse_milestones(text)
    _, wave_by_package = parse_waves(text)
    v1_packages = set(wave_by_package)

    if not v1_packages.issubset(packages):
        unknown = sorted(v1_packages.difference(packages))
        raise ContractError(f"wave registry references unknown packages: {unknown}")
    if len(packages) - len(v1_packages) != EXPECTED_POST_V1_PACKAGE_COUNT:
        raise ContractError(
            f"expected {EXPECTED_POST_V1_PACKAGE_COUNT} post-V1 packages"
        )

    v1_dependencies = {
        edge
        for edge in dependencies
        if edge[0] in v1_packages and edge[1] in v1_packages
    }
    if len(v1_dependencies) != EXPECTED_V1_DEPENDENCY_COUNT:
        raise ContractError(
            f"expected {EXPECTED_V1_DEPENDENCY_COUNT} V1 dependency edges, "
            f"found {len(v1_dependencies)}"
        )
    invalid_wave_edges = {
        edge
        for edge in v1_dependencies
        if wave_by_package[edge[0]] >= wave_by_package[edge[1]]
    }
    if invalid_wave_edges:
        raise ContractError(
            "V1 dependencies must point from an earlier to a later wave: "
            f"{format_edges(invalid_wave_edges)}"
        )

    graph_dependencies = parse_canonical_graph(text, packages)
    graph_v1_dependencies = {
        edge
        for edge in graph_dependencies
        if edge[0] in v1_packages and edge[1] in v1_packages
    }
    if len(graph_v1_dependencies) != EXPECTED_V1_GRAPH_DEPENDENCY_COUNT:
        raise ContractError(
            "expected "
            f"{EXPECTED_V1_GRAPH_DEPENDENCY_COUNT} canonical V1 graph dependency "
            f"edges, found {len(graph_v1_dependencies)}"
        )
    if graph_v1_dependencies != v1_dependencies:
        missing = v1_dependencies.difference(graph_v1_dependencies)
        extra = graph_v1_dependencies.difference(v1_dependencies)
        raise ContractError(
            "canonical V1 graph edges differ from dependency-table V1 edges; "
            f"missing from graph: [{format_edges(missing)}]; "
            f"extra in graph: [{format_edges(extra)}]"
        )
    require_acyclic(expected_packages, graph_dependencies, "canonical")
    require_acyclic(v1_packages, v1_dependencies, "V1")

    validate_specifications(repository_root)
    validate_decisions(repository_root)
    validate_proof_anchors(repository_root, text)

    consolidation_body = section(
        text, "### Consolidation record", "### Research evidence ledger"
    )
    consolidation_registry_body = section(
        consolidation_body,
        "The consolidation claims above are backed",
        "These attestations are repository review evidence",
    )
    consolidation_table = bounded_table_rows(
        consolidation_registry_body,
        CONSOLIDATION_HEADER,
        "consolidation registry",
    )
    consolidation_rows: dict[str, list[str]] = {}
    consolidation_ids: list[str] = []
    for cells in consolidation_table:
        match = re.fullmatch(
            r"`(CONSOL-[0-9]{2})` / Content-bound attestation",
            cells[0],
        )
        if match is None:
            raise ContractError(
                f"malformed consolidation registry ID: {cells[0]}"
            )
        consolidation_id = match.group(1)
        consolidation_ids.append(consolidation_id)
        consolidation_rows[consolidation_id] = cells
    expected_consolidations = [f"CONSOL-{index:02d}" for index in range(1, 4)]
    if consolidation_ids != expected_consolidations:
        raise ContractError(
            "consolidation registry must be exactly CONSOL-01..CONSOL-03"
        )
    for index in range(1, 4):
        consolidation_id = f"CONSOL-{index:02d}"
        receipt_target = f"../receipts/consolidations/consol-{index:02d}.md"
        cells = consolidation_rows[consolidation_id]
        links_by_cell = [markdown_links(cell) for cell in cells]
        expected_link = (
            f"`docs/receipts/consolidations/consol-{index:02d}.md`",
            receipt_target,
        )
        if (
            links_by_cell[2] != [expected_link]
            or any(links_by_cell[cell] for cell in (0, 1, 3))
        ):
            raise ContractError(
                "consolidation registry must contain exactly the canonical "
                f"consolidation receipt target {receipt_target}"
            )
        if cells[2].count(f"`{EXPECTED_CURRENT_FINDING_RANGE}`") != 1:
            raise ContractError(
                "consolidation registry must bind the canonical current finding "
                f"range {EXPECTED_CURRENT_FINDING_RANGE} exactly once"
            )
    g0_target = "../receipts/doc-00-g0.md"
    g0_lines = [
        line for line in text.splitlines() if "doc-00-g0.md" in line
    ]
    if (
        len(g0_lines) != 1
        or markdown_links(g0_lines[0])
        != [("`docs/receipts/doc-00-g0.md`", g0_target)]
    ):
        raise ContractError(
            "delivery program must contain exactly the canonical "
            f"DOC-00 completion target {g0_target}"
        )

    interface_body = section(
        text,
        "### Interface and artifact ownership registry",
        "#### Closed planning-error adaptation",
    )
    interface_rows = bounded_table_rows(
        interface_body,
        INTERFACE_HEADER,
        "interface registry",
    )
    interfaces: list[str] = []
    for cells in interface_rows:
        match = re.fullmatch(r"`(IF-[A-Z0-9]+(?:-[A-Z0-9]+)*)`", cells[0])
        if match is None:
            raise ContractError(f"malformed interface registry ID: {cells[0]}")
        interfaces.append(match.group(1))
    require_unique(interfaces, "interface")
    if tuple(interfaces) != EXPECTED_INTERFACE_IDS:
        raise ContractError(
            "interface registry IDs and source order must match the canonical "
            f"{EXPECTED_INTERFACE_COUNT}-interface inventory"
        )

    review_body = section(
        text,
        "#### Independent documentation-review receipt plan",
        "### Review finding and resolution ledger",
    )
    review_table = bounded_table_rows(
        review_body,
        REVIEW_HEADER,
        "review registry",
    )
    review_rows: dict[str, list[str]] = {}
    reviews: list[str] = []
    for cells in review_table:
        match = re.fullmatch(r"`(REV-[0-9]{2})`", cells[0])
        if match is None:
            raise ContractError(f"malformed review registry ID: {cells[0]}")
        review_id = match.group(1)
        reviews.append(review_id)
        review_rows[review_id] = cells
    expected_reviews = [
        f"REV-{index:02d}" for index in range(1, EXPECTED_REVIEW_COUNT + 1)
    ]
    if reviews != expected_reviews:
        raise ContractError("review registry must be exactly REV-01..REV-18")
    for index in range(1, EXPECTED_REVIEW_COUNT + 1):
        review_id = f"REV-{index:02d}"
        receipt_target = f"../receipts/reviews/rev-{index:02d}.md"
        cells = review_rows[review_id]
        links_by_cell = [markdown_links(cell) for cell in cells]
        if (
            links_by_cell[2] != [("external content-bound record", receipt_target)]
            or any(links_by_cell[cell] for cell in (0, 1, 3))
        ):
            raise ContractError(
                "review registry must contain exactly the canonical receipt target "
                f"{receipt_target}"
            )

    finding_body = section(
        text,
        "### Review finding and resolution ledger",
        "### CI and release flow",
    )
    finding_rows = bounded_table_rows(
        finding_body,
        FINDING_HEADER,
        "finding ledger",
    )
    findings: list[int] = []
    finding_priority_digits: list[str] = []
    for cells in finding_rows:
        match = re.fullmatch(r"`FND-([0-9]{3})` / P([0-3])", cells[0])
        if match is None:
            raise ContractError(f"malformed finding ledger ID: {cells[0]}")
        findings.append(int(match.group(1)))
        finding_priority_digits.append(match.group(2))
    require_contiguous(findings, EXPECTED_FINDING_COUNT, "finding IDs")
    if "".join(finding_priority_digits) != EXPECTED_FINDING_PRIORITY_DIGITS:
        raise ContractError(
            "finding severities must match the canonical "
            f"FND-001..FND-{EXPECTED_FINDING_COUNT:03d} ledger"
        )

    conformances = [
        int(match.group(1) or "01")
        for line in text.splitlines()
        if (match := CONFORMANCE_PATTERN.match(line)) is not None
    ]
    require_contiguous(
        conformances, EXPECTED_CONFORMANCE_COUNT, "conformance receipt IDs"
    )
    current_conformance = section(
        text,
        (
            "#### Manual conformance receipt "
            f"`DOC-CONF-{EXPECTED_CONFORMANCE_COUNT:02d}`"
        ),
        "Any later source, count, interface, ownership, finding, review disposition,",
    )
    current_inventory = (
        f"The program remains {EXPECTED_PACKAGE_COUNT} unique packages, "
        f"{EXPECTED_V1_PACKAGE_COUNT} V1 and "
        f"{EXPECTED_POST_V1_PACKAGE_COUNT} post-V1. The canonical dependency "
        f"table has {EXPECTED_DEPENDENCY_COUNT} total and "
        f"{EXPECTED_V1_DEPENDENCY_COUNT} V1-to-V1 relations; the Mermaid graph "
        f"has {EXPECTED_GRAPH_DEPENDENCY_COUNT} total and "
        f"{EXPECTED_V1_GRAPH_DEPENDENCY_COUNT} V1-to-V1 edges after removing "
        "the stale `EVD-01 → OBS-01` edge and adding the "
        "management-publication predecessor for `RET-01`. The active registry "
        f"retains {EXPECTED_INTERFACE_COUNT} unique interfaces and "
        f"{EXPECTED_WAVE_COUNT} stable wave labels. There are "
        f"{EXPECTED_FINDING_COUNT} unique sequential findings, "
        f"{EXPECTED_CONFORMANCE_COUNT} append-only conformance receipts, "
        f"{EXPECTED_REVIEW_COUNT} external review paths, "
        f"{EXPECTED_SPECIFICATION_COUNT} non-template specifications, and "
        f"{EXPECTED_DECISION_COUNT} numbered decisions: "
        f"{EXPECTED_ACCEPTED_DECISION_COUNT} `Accepted` and "
        f"{EXPECTED_SUPERSEDED_DECISION_COUNT} `Superseded`"
    )
    current_finding_range = f"`{EXPECTED_CURRENT_FINDING_RANGE}`"
    if current_conformance.count(current_finding_range) != 1:
        raise ContractError(
            f"active DOC-CONF-{EXPECTED_CONFORMANCE_COUNT:02d} must contain "
            "canonical current-state fragment "
            f"exactly once: {current_finding_range}"
        )
    if current_conformance.count(current_inventory) != 1:
        raise ContractError(
            f"active DOC-CONF-{EXPECTED_CONFORMANCE_COUNT:02d} complete "
            "structural inventory differs from the canonical inventory"
        )

    required_fragments = [
        "scripts/check-v1-delivery-program.py",
        "scripts/test-v1-delivery-program-check.sh",
        "0017-control-evaluation-interventions-and-pre-access-evidence.md",
        "0018-canonicalize-predictive-and-planning-identities.md",
        "0019-establish-render-domain-and-bounded-validation.md",
        "0020-freeze-deterministic-public-call-semantics.md",
        "0021-adopt-a-recoverable-verifiable-release-lifecycle.md",
        "0022-establish-content-bound-doc-00-attestations.md",
        "0023-bind-complete-renderer-training-state.md",
        "0024-separate-transition-records-from-derived-artifacts.md",
        "0025-complete-pre-access-and-statistical-guards.md",
        "0026-distinguish-initial-release-from-predecessor-rollback.md",
        "0027-complete-release-support-and-update-recovery.md",
        "0028-bind-update-writer-exclusion-and-ship-authorization.md",
        "0029-require-positive-update-success-per-supported-tuple.md",
        "0030-protect-doc-00-history-and-governance.md",
        "0031-complete-compile-and-update-admission-handoffs.md",
        "0032-bind-authoritative-exact-sidecars-and-two-plane-consolidation.md",
        "0033-separate-source-conformance-from-stable-g0-identity.md",
        "0034-adopt-vector-conditioned-focus-adapter-boundary.md",
        f"DOC-CONF-{EXPECTED_CONFORMANCE_COUNT - 1:02d}",
        f"DOC-CONF-{EXPECTED_CONFORMANCE_COUNT:02d}",
    ]
    for fragment in required_fragments:
        if fragment not in text:
            raise ContractError(f"required DOC-00 reference is missing: {fragment}")

    if require_receipts:
        validate_receipts(
            repository_root,
            parsed_required_checks,
            parsed_change_aware_prefix,
        )


def main() -> int:
    """Run the checker."""

    arguments = sys.argv[1:]
    if arguments[:1] == ["--protected-digest-at"]:
        if len(arguments) != 3:
            print(
                f"usage: {Path(sys.argv[0]).name} "
                "--protected-digest-at <commit> <constant>",
                file=sys.stderr,
            )
            return 2
        configured_root = os.environ.get("NEMOSYNE_REPOSITORY_ROOT")
        repository_root = (
            Path(configured_root).resolve()
            if configured_root is not None
            else Path(__file__).resolve().parent.parent
        )
        constant = arguments[2]
        if constant not in {
            "EXPECTED_PROTECTED_CONFORMANCE_SHA256",
            "EXPECTED_PROTECTED_FINDING_SHA256",
        }:
            print("unsupported protected digest constant", file=sys.stderr)
            return 2
        try:
            digests = source_checker_protected_digests(
                repository_root,
                arguments[1],
            )
        except (ContractError, OSError, UnicodeError) as error:
            print(f"V1 delivery-program check failed: {error}", file=sys.stderr)
            return 1
        print(digests[constant])
        return 0

    if arguments[:1] == ["--check-append-only"]:
        if len(arguments) != 3:
            print(
                f"usage: {Path(sys.argv[0]).name} "
                "--check-append-only <base-commit> <head-commit>",
                file=sys.stderr,
            )
            return 2
        configured_root = os.environ.get("NEMOSYNE_REPOSITORY_ROOT")
        repository_root = (
            Path(configured_root).resolve()
            if configured_root is not None
            else Path(__file__).resolve().parent.parent
        )
        try:
            validate_delivery_history_append_only(
                repository_root,
                arguments[1],
                arguments[2],
            )
            validate_conformance_successor_for_changed_archive(
                repository_root,
                arguments[1],
                arguments[2],
            )
        except (ContractError, OSError, UnicodeError) as error:
            print(f"V1 delivery-program check failed: {error}", file=sys.stderr)
            return 1
        print("V1 delivery-program append-only history is consistent.")
        return 0

    require_receipts = False
    if "--require-receipts" in arguments:
        if arguments.count("--require-receipts") != 1:
            print(
                f"usage: {Path(sys.argv[0]).name} "
                "[--require-receipts] [delivery-program.md]",
                file=sys.stderr,
            )
            return 2
        require_receipts = True
        arguments.remove("--require-receipts")
    if len(arguments) > 1 or any(argument.startswith("-") for argument in arguments):
        print(
            f"usage: {Path(sys.argv[0]).name} "
            "[--require-receipts] [delivery-program.md]",
            file=sys.stderr,
        )
        return 2
    path = (
        Path(arguments[0])
        if arguments
        else Path(__file__).resolve().parent.parent / DELIVERY_PROGRAM_PATH
    )
    try:
        validate(path, require_receipts=require_receipts)
    except (ContractError, OSError, UnicodeError) as error:
        print(f"V1 delivery-program check failed: {error}", file=sys.stderr)
        return 1
    scope = " and DOC-00 attestations" if require_receipts else ""
    print(f"V1 delivery-program registries{scope} are consistent.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
