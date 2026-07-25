# DOC-CONF-24

| Field | Value |
| --- | --- |
| Schema | `doc00-attestation-v1` |
| Record ID | `DOC-CONF-24` |
| Kind | MergeAuthorization |
| Status | `MergeAuthorized` |
| Actor | `Codex /root` |
| Declaration | Principal integrator for DOC-00 merge authorization; not the accountable human or an independent reviewer. |
| Completed at | `2026-07-25T23:41:27Z` |
| Source commit | `c0adf49dfd41a69298bc514aaa064561866ea966` |
| Source tree | `aa86dae511a1b91c2fcea55d19bdcc89c50aa4fb` |
| Included paths | `docs/specifications, docs/decisions` |
| Archive algorithm | `git-archive-tar-sha256-v1` |
| Archive SHA-256 | `2ddbd2db49b1accf0c72a5fda44bf0d43a737d79041c1ca7f4692447f5b6016c` |
| Method | Conformance and repository-check reconciliation. |
| Findings | `None` |
| Disposition | `Pass` |
| Residual limits | Documentation evidence only; merge authorization remains conditional on strict receipt, change-aware, repository, PR CI, rebase-integration, and main-push CI checks. |
| Evidence references | [CONSOL-01](consolidations/consol-01.md); [CONSOL-02](consolidations/consol-02.md); [CONSOL-03](consolidations/consol-03.md); [REV-01](reviews/rev-01.md); [REV-02](reviews/rev-02.md); [REV-03](reviews/rev-03.md); [REV-04](reviews/rev-04.md); [REV-05](reviews/rev-05.md); [REV-06](reviews/rev-06.md); [REV-07](reviews/rev-07.md); [REV-08](reviews/rev-08.md); [REV-09](reviews/rev-09.md); [REV-10](reviews/rev-10.md); [REV-11](reviews/rev-11.md); [REV-12](reviews/rev-12.md); [REV-13](reviews/rev-13.md); [REV-14](reviews/rev-14.md); [REV-15](reviews/rev-15.md); [REV-16](reviews/rev-16.md); [REV-17](reviews/rev-17.md); [REV-18](reviews/rev-18.md); `./scripts/test-documentation-change-policy.sh`; `./scripts/test-documentation-check.sh`; `./scripts/check-documentation.sh`; `./scripts/test-v1-delivery-program-check.sh`; `./scripts/check-v1-delivery-program.py --require-receipts`; `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -F missing-docs -F unsafe-code`; `RUSTDOCFLAGS="-D warnings -F missing-docs -F unsafe-code" cargo doc --workspace --all-features --no-deps --locked`; `cargo test --workspace --all-features --locked`; `git diff --check`; `DOCUMENTATION_BASE_REF=origin/main ./scripts/check-documentation.sh /private/tmp/nemosyne-doc00-pr-body.md`. |
| Replaces | `DOC-CONF-24 at archive digest a764b339b88726a1353aef29b9c4f217c40fd3186d02af2fb6a066b5c1e5ea20` |
