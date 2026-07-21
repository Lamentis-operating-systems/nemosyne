# Contributing

Nemosyne is at an early stage. Please open an issue before proposing a substantial change.

Keep pull requests focused, explain the reason for the change, and include relevant tests. By contributing, you agree that your contribution is licensed under the Apache License 2.0.

## Documentation

Behavioral and mathematical contracts belong in `docs/specifications`. Significant design choices belong in `docs/decisions`. Public Rust APIs must be documented next to the code.

Every pull request must declare its documentation impact and explain that declaration. Every change to production Rust source requires an updated specification. A significant design choice requires a decision record. Tests must verify normative claims where practical.

The documentation check rejects missing declarations, malformed documents, and declarations that do not match the changed files.
