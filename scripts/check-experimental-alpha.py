#!/usr/bin/env python3
"""Check the non-promotional Experimental Alpha dependency boundary."""

import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
ALPHA = "nemosyne-experimental-alpha"
FORBIDDEN = re.compile(
    r"(G1|receipt|disposition|promotion|release|persistence|database|telemetry|"
    r"network|llm|model[_ -]?selection)",
    re.IGNORECASE,
)


def fail(message: str) -> None:
    print(f"experimental-alpha check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


metadata = json.loads(
    subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=ROOT,
        text=True,
    )
)
packages = {package["name"]: package for package in metadata["packages"]}
if list(name for name in packages if name == ALPHA) != [ALPHA]:
    fail("exactly one Alpha package is required")
alpha = packages[ALPHA]
if alpha.get("publish") not in (None, []):
    fail("Alpha must not be publishable")

workspace_names = set(packages)
for package in packages.values():
    workspace_dependencies = {
        dependency["name"]
        for dependency in package["dependencies"]
        if dependency["name"] in workspace_names
    }
    if package["name"] == ALPHA:
        if workspace_dependencies - {"nemosyne-core"}:
            fail("Alpha has a forbidden workspace dependency")
    elif ALPHA in workspace_dependencies:
        fail(f"{package['name']} depends on Alpha")

source = "\n".join(
    path.read_text()
    for path in sorted((ROOT / "crates" / ALPHA / "src").glob("**/*.rs"))
)
for match in re.finditer(r"\bpub\s+(?:struct|enum|trait|fn|mod|const|use)\s+([A-Za-z0-9_]+)", source):
    if match.group(1).endswith("V1"):
        fail(f"public V1 identifier: {match.group(1)}")
for line in source.splitlines():
    if not line.lstrip().startswith("//") and FORBIDDEN.search(line):
        fail(f"forbidden capability term in production source: {line.strip()}")

print("experimental-alpha boundary: ok")
