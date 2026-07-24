# 0021: Adopt a recoverable and verifiable release lifecycle

Status: Accepted
Date: 2026-07-24

## Context

The proposed release state machine assumed publication rollback succeeds,
allowed obsolete candidates to remain implicit rollback targets, did not state
what uninstall does to user memory, and did not require fresh-install
provisioning or a closed vulnerability-admission policy.

## Decision

Publication rollback failure enters a terminal
`PartialPublicationQuarantined` state with an incident receipt, distribution
stop, channel inventory, user notification, and mitigation status. It is never
reported as an ordinary successful rollback.

Release lifecycle records include `Withdrawn` and `EndOfLife`. A rollback
target must currently be supported, previously verified and shipped, compatible
with the memory revision, and neither withdrawn nor end-of-life.

Uninstall removes program artifacts and disposable runtime caches but preserves
the authoritative user database, backups, and keys required to decrypt them by
default. Data purge is a separate explicit authorized management operation with
its own confirmation and receipt. Reinstall recovery and purge are tested
separately.

A release candidate packages the authenticated management/provisioning path.
Verification on a clean supported machine performs install, authenticated
empty-store provisioning, and one offline compile before evaluation or
shipment.

Supply-chain admission freezes advisory-source identities, scan time and
implementation, transitive dependency inventory, severity and exploitability
policy, exception authority and expiry, license policy, SBOM identity, and a
fail-closed scan receipt for the exact candidate. An unresolved prohibited
vulnerability or expired exception blocks verification and shipment.

## Rationale

Rollback is a fallible operation and needs an explicit partial-publication
state. Local memory is user-owned data and must not disappear as an uninstall
side effect. A package that cannot initialize a fresh store is not a usable
release. SBOM existence alone does not establish vulnerability acceptance.

## Alternatives

- Treat rollback failure as `PublicationRolledBack`. This hides partial
  distribution and prevents reliable recovery.
- Delete memory on uninstall. This couples application removal to irreversible
  user-data destruction.
- Permit any previously shipped version as rollback target. Withdrawn or
  unsupported software may be unsafe.
- Require only an SBOM and license list. Known vulnerable transitive
  dependencies could still pass.

## Consequences

Release, recovery, management, security, packaging, and clean-install fixtures
must cover the new states and exact-candidate receipts. Support policy and
vulnerability exceptions remain prospectively selected release inputs.
