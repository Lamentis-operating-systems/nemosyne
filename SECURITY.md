# Security Policy

NSIT has no released or supported implementation yet. The `main` branch is public
development history and receives best-effort review; it is not a security-supported
distribution.

## Report privately

Do not disclose an unresolved vulnerability in an issue, pull request, discussion, or
commit. Use GitHub's
[private vulnerability reporting](https://github.com/Lamentis-operating-systems/nsit/security/advisories/new)
and include:

- the affected revision and component;
- prerequisites and a minimal reproduction;
- the security boundary and impact;
- sanitized evidence; and
- a suggested mitigation, if known.

Never include active credentials, personal data, or unrelated confidential material.
If a credential was exposed, revoke or rotate it immediately; deleting a commit is not
sufficient.

Maintainers assess reports on a best-effort basis. There is currently no response-time
guarantee or bug-bounty program. Please keep a report private until disclosure is
coordinated.

## Current security boundary

No NSIT encoder, decoder, validator, sandbox, or supported format exists yet. Future
automation must treat source code, NSIT input, model output, dependencies, and generated
files as untrusted until validated. A successful parse, round trip, build, test suite,
or code scan is evidence for a named property, not a general security guarantee.

Good-faith research must stay within systems and data you own or are explicitly
authorized to test. This policy grants no authority to test third-party systems.
