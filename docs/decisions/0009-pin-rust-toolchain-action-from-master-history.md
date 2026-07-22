# 0009: Pin the Rust toolchain action from master history

Status: Accepted
Date: 2026-07-23

## Context

The continuous-integration workflow pinned `dtolnay/rust-toolchain` to a generated Rust-version revision while also supplying a `toolchain` input. That revision embedded Rust 1.95.0 directly and did not declare the input, so GitHub ignored the redundant field and emitted a warning in the Quality job and every operating-system test job.

The generated revision had also diverged from the action's `master` history. Upstream warns that a full commit identifier outside that history can eventually be garbage-collected, which would make an otherwise unchanged workflow fail to resolve its action.

## Decision

Use the generic `dtolnay/rust-toolchain` implementation from its upstream `master` history and pin it by one full 40-character commit identifier at every workflow call site. Pass Rust 1.95.0 explicitly through the action's declared `toolchain` input. Install `clippy` and `rustfmt` in the Quality job and use the same action revision without additional components in the operating-system test matrix.

Before adopting or updating the pin, verify from the upstream repository that the selected commit is reachable from `master` and that its `action.yml` declares every supplied input. Keep the workflow's Rust version aligned with `rust-toolchain.toml` and the workspace package contract. Continue using the existing monthly GitHub Actions Dependabot configuration for update proposals; each resulting workflow must still pass the repository's required checks.

## Rationale

A full commit identifier makes the executed third-party action content immutable. Selecting that commit from maintained upstream history avoids relying on generated revisions that upstream does not promise to retain. The generic action accepts an explicit toolchain, so the workflow states the compiler contract directly instead of relying on behavior hidden inside the action revision.

Using one action revision in Quality and Test prevents platform jobs from silently executing different installer code. The `master` comment describes the action lineage, while the separate `toolchain` field describes the Rust compiler version.

## Alternatives

- **Remove the unsupported `toolchain` fields but keep the generated revision.** Rejected because this would silence the warnings while retaining the revision-retention risk and an implicit compiler selection.
- **Use mutable `@master`.** Rejected because upstream could change the executed code without a repository commit or review.
- **Use a version branch such as `@1.95.0`.** Rejected because the branch is mutable and the generated action variant does not accept the explicit toolchain contract.
- **Pin a generated version revision by full commit identifier.** Rejected because upstream does not guarantee retention for commits outside `master` history.
- **Run `rustup` commands directly.** Rejected because it would duplicate tested cross-platform installation behavior without removing the need to maintain an explicit toolchain version.
- **Rely on runner defaults or only on `rust-toolchain.toml`.** Rejected because hosted-runner defaults change over time and the selected generic action requires an explicit `toolchain` input.

## Consequences

All four Rust installation executions use the same immutable generic action code and accept their explicit inputs without GitHub runner warnings. Rust remains pinned to 1.95.0 independently of future generic-action updates.

The toolchain version is intentionally repeated in the workflow and `rust-toolchain.toml`, so version upgrades must keep those declarations aligned. Dependabot may propose a newer generic action commit separately from Rust compiler upgrades. Such updates remain reviewable full-SHA changes and must retain valid action inputs and green Documentation, Quality, Linux, macOS, and Windows checks.
