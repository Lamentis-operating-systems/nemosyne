# 0032: Bind authoritative exact sidecars and two-plane consolidation

Status: Accepted
Date: 2026-07-25

## Context

The memory contracts require exact values to remain authoritative beside
lossy numerical facets. A transition record nevertheless committed only to an
undefined exact-sidecar reference. Nothing required that reference to bind the
sidecar schema and canonical bytes, or that construction, publication, reads,
migration, and collision handling verify the same binding. Exact bytes could
therefore be rebound or corrupted without necessarily changing the transition
record-version identity.

The architecture also described the rebuildable numerical plane as the sole
computational state for consolidation, while the consolidation contract
required exact-value equality, temporal, social, and modal scope,
provenance, disclosure, authority, validity, supersession, and contradiction.
Those properties are authoritative-plane facts. An implementation could not
satisfy both statements without choosing an undocumented source of truth.

## Decision

Every authoritative exact sidecar uses one closed
`ExactSidecarContentEnvelopeV1`. The envelope contains its authenticated
`ExactSidecarSchemaId` and a finite canonical set of schema-owned exact
bindings. Each binding contains one schema-owned locator, exact-value schema
and type, an explicit presence variant, and the complete canonical exact value
when present. Missing, unknown, inapplicable, explicit none, zero, and an empty
present value remain distinct.

`ExactSidecarSchemaId` is itself a domain-separated typed content identity over
the complete canonical `ExactSidecarSchemaDefinitionV1`: finite cardinality
and byte limits, locator grammar and total order, exact-value schemas and
types, presence vocabulary, canonical encoding rules, and value domains.
Authentication recomputes that identity from the retained immutable
definition. Reusing one schema ID for different definition bytes is forbidden
and fails closed; a schema-definition change always creates a new schema ID.

For an authenticated identity regime \(K_X\), the domain-separated content
identity is:

\[
\operatorname{ExactSidecarContentId}_{K_X}
=
\operatorname{Digest}_{K_X}\!\left(
\operatorname{DomainSeparator}_{K_X}
\parallel
\operatorname{CanonicalEncode}_{K_X}
(\operatorname{ExactSidecarContentEnvelopeV1})
\right).
\]

This decision normatively adopts, without shortening or redefining it, the
complete `ExactSidecarIdentityRegimeV1` field set and regime-ID preimage in
`cognitive-memory-activation-and-focus.md`. That closed definition contains
exactly `definition_format_version`, `content_identity_schema_id`,
`schema_definition_identity_policy_id`, `domain_separator_bytes`,
`digest_algorithm_id`, `digest_algorithm_version`, `digest_parameter_set`,
`canonical_encoder_id`, `canonical_encoder_version`,
`canonical_encoder_parameter_set`, and `typed_digest_output_contract`; its
content-derived ID uses the owner's declared domain separator and complete
canonical definition under the authenticated regime-definition identity
policy \(K_R\). The immutable memory revision pins that complete regime; none
of its fields is caller-selectable or inferred from persisted bytes. The
initial registered domain-separator bytes are
`nemosyne.exact-sidecar-content.v1`; any later registered separator belongs to
a different regime. `ExactSidecarIdentityRegimeId` is a domain-separated typed
content identity over the complete canonical regime definition and is
recomputed whenever the regime is authenticated. `ExactSidecarContentId` is
typed under that regime.
The only reference form is
`ExactSidecarRefV1 { regime_id, schema, content_id }`. Its `regime_id` must
equal the authenticated memory revision's recomputed regime ID and its
`schema` must equal the recomputed envelope-schema ID. The reference remains
different across regime rotations even if two digest algorithms happen to
emit equal digest bytes.

Each authoritative record contains
`ExactSidecarCustodyBindingV1 { custody_domain_id, sidecar_ref }`. The
record-version envelope commits to that complete binding, so its own content
identity transitively commits to the custody domain, identity regime,
exact-sidecar schema, and content identity. The sidecar envelope excludes the
owning record-version identity, record-derived binding-instance identities,
rebuildable numerical artifacts and indexes, compiler configuration, runtime
state, and request-local witnesses. A schema-permitted record-version
reference must resolve to an already published and fully verified immutable
version in the authenticated parent memory revision. It cannot target the
containing record or any record introduced by the same publication. Every such
edge therefore points to a strictly prior revision, making the transitive
reference graph well-founded. Other current-revision relations live outside
the sidecar. This one-way, prior-revision rule prevents an identity cycle.

`ExactSidecarCustodyDomainId` is a domain-separated typed content identity over
one immutable canonical custody-domain definition. That definition binds the
retention, access, erasure, backup/export, and reference-ledger schema and
policy identities that govern physical storage. Authentication recomputes the
domain identity from the complete retained definition; rebinding one domain ID
to different policy bytes is forbidden. The complete sidecar reference occurs
only once inside `ExactSidecarCustodyBindingV1`, and the record and custody
ledger commit to that same binding rather than reconstructing it from parallel
fields.

Sidecar content, references, and record versions have no unchecked public
constructors. Checked construction authenticates and recomputes the complete
schema definition and identity regime, validates finite bounds, unique
canonical locators, exact types, presence variants, canonical encoding, and
values, then derives the content identity and complete reference.
Record-version construction accepts only that validated reference plus an
authenticated custody domain, derives the complete
`ExactSidecarCustodyBindingV1`, and derives the record-version identity from
the closed record envelope.

A privileged memory-management operation publishes the canonical sidecar
bytes, referring record version, custody-ledger entry, and any new physical
object in one atomic transaction. Before any external sidecar lookup,
publication and every revision-pinned read execute the owner contract's
two-phase `ExactSidecarIntegrityValidationV1`. Its effect-free prelookup phase
recomputes the record identity, authenticates and compares the complete regime
definition and reference, authenticates and compares the schema definition and
reference, and authenticates the custody-domain definition, record-bound
custody value, logical ledger entry, and authorization state. No physical
object, sidecar, collision witness, cache, backup/export state, or nested
target is touched until all four prechecks pass.

The postlookup phase consumes one immutable revision-, ledger-, and
integrity-fence-bound `ExactSidecarResolvedSnapshotV1`. It validates
authenticated physical custody, presence, complete canonical envelope
decoding and re-encoding, envelope schema, recomputed reference, collision
evidence, and bounded nested targets. It evaluates every predicate whose
dependencies exist and then applies the one closed public memory-integrity
precedence declared below. A canonical postlookup envelope-schema mismatch
therefore remains `ExactSidecarSchemaMismatch`, a physical-custody mismatch
remains `ExactSidecarCustodyMismatch`, absence is `ExactSidecarMissing`, and
undecodable bytes are `ExactSidecarContentMismatch`. Encounter order cannot
select a lower-precedence cause. Quarantine is attempted only after
classification selects `ExactSidecarContentIdentityCollision` and proves all
higher-precedence causes absent.

No newly introduced sidecar or referring record becomes observable without
all checks; a previously verified shared sidecar may already exist. A repeated
content identity with byte-identical canonical content is idempotent.
`ExactSidecarContentIdentityCollision` applies only when two distinct canonical
envelopes each independently validate and recompute to the same typed content
identity under the same identity regime. Bytes that do not recompute to their
claimed reference are instead `ExactSidecarReferenceMismatch` under the
declared precedence. A collision quarantines the complete
`(ExactSidecarIdentityRegimeId, ExactSidecarContentId)` trust domain, including
every reachable record, revision, derived artifact, index, rollback artifact,
backup, export, and cache that relies on that identity. Restore, migration,
sharing, and compile remain blocked. The collided trust-domain identity is
permanently unusable: recovery either republishes every retained meaning under
a new authenticated regime and new referring record identities, or erases
every authorized copy. Deleting or selecting one witness, observing later byte
equality, or changing a registry entry never clears the old quarantine. Both
cases fail closed. Missing, truncated, corrupt, cross-record, cross-revision,
schema-mismatched, rebound, or identity-mismatched content substituted
inconsistently with the referring record is an integrity error, never an
absent value or empty sidecar.

Collision enforcement is a store-owned verified-read operation, not compiler
management authority. The observation origin is the closed
`CollisionObservationOriginV1::{Compile, TerminalProbe, Management}` union.
`Compile` binds its compile admission, product-release guard, and resolved
snapshot; `TerminalProbe` binds its terminal probe, terminal-probe-result
guard, and resolved snapshot; `Management` binds its privileged management
admission, independently authenticated management authorization,
lifecycle-commit guard, and resolved snapshot. Cross-origin field reuse is
invalid.

Classification constructs one authenticated `CollisionQuarantineBasisV1`
binding the store, affected `(regime_id, content_id)` trust key, exact
observation origin, permanent collision-tombstone identity, expected and next
integrity-fence generations, complete collision-witness-set commitment,
complete affected-custody/derived-artifact closure commitment, complete active
admission/snapshot closure commitment, and either a proven
`CompleteReverseIndex` scope or `WholeStoreGeneration` fallback. The first
scope is valid only when every witness, custody object, derived artifact,
active admission, and resolved snapshot reachable from the key is covered by
authenticated complete reverse indexes.

One crash-atomic containment commit creates or verifies the permanent
`ExactSidecarCollisionTombstoneV1`, advances
`ExactSidecarIntegrityFenceV1`, and records a closed revoke disposition for
the origin plus every intersecting active admission and snapshot. When any
closure proof is incomplete, it instead revokes the whole captured store
generation. The same commit closes semantic-read, product-release,
terminal-probe-pass, lifecycle-mutation-commit, and lifecycle admission in
that scope. It does not block on physical resource destruction.

Physical cleanup is the separate durable
`CollisionTerminalRemovalStateV1`. It binds the exact quarantine basis and
either one immutable canonical revoke set or one fenced store generation with
a canonical generation cursor. It also binds the monotonic next cursor,
strictly positive per-step item, work, and released-byte limits, and exactly
one last-step outcome: `Committed` with exact resource-closure receipts,
`Aborted` with a verified no-effect result, or `ReconciliationRequired` with
the unchanged or last committed cursor and a durable recovery fence. A step
cannot exceed any installed limit, skip or duplicate a cursor position,
rebuild a larger queue, lower the integrity fence, reopen admission, publish a
result, or change recovery eligibility. A revoked record leaves this state
only after exact receipts prove all resources closed or a durable
generation-scoped recovery fence proves them unreachable while cleanup remains
represented by the same cursor. Repetition, interruption, and restart resume
the same bounded idempotent state; completion may compact closed entries but
never removes the quarantine basis, tombstone, fence history, or permanent
non-use rule.

```text
CollisionTerminalRemovalStateV1
├── collision_quarantine_basis_id
├── removal_scope
│   ├── CanonicalRevokeSet
│   │   ├── revoke_set_id
│   │   └── canonical_revoked_resource_ids[]
│   └── FencedGeneration
│       ├── fenced_store_generation
│       └── canonical_generation_cursor
├── next_cursor
├── installed_limits
│   ├── max_items_per_step > 0
│   ├── max_work_units_per_step > 0
│   └── max_bytes_released_per_step > 0
└── last_step_outcome
    ├── Committed
    │   ├── prior_cursor
    │   ├── next_cursor
    │   └── exact_resource_closure_receipts[]
    ├── Aborted
    │   ├── unchanged_cursor
    │   └── verified_no_effect
    └── ReconciliationRequired
        ├── unchanged_or_last_committed_cursor
        └── durable_recovery_fence_id
```

Every resolution checks the root fence. Immediately before externally visible
success, each origin performs one final linearizable comparison of its typed
guard, exact trust-key dependency closure, and captured generation against the
current root fence. `Compile` validates its product-release guard,
`TerminalProbe` validates its result guard before issuing a passing receipt,
and `Management` validates its lifecycle-commit guard before committing the
intended mutation. Those validations and containment have one total order. If
containment linearizes first, no product, passing probe receipt, or lifecycle
mutation is released and the origin remains retained for terminal removal. If
a guarded success linearizes first, it is not a success after containment; any
released derived artifact later found dependent on the collided key enters the
affected closure. A check performed only when a snapshot is created cannot
replace the corresponding final guard.

The compiler receives the collision cause only after containment is durable.
Containment failures preserve one of the owner contract's closed
`ExactSidecarIntegrityCoordinationError` sources, in order:
`IntegrityQuarantineGenerationMismatch`,
`IntegrityQuarantineWitnessMismatch`,
`IntegrityQuarantineCommitUnavailable`,
`IntegrityQuarantineOutcomeUnknown`, or
`IntegrityQuarantineReconciliationRequired`. They return no product and do
not reorder memory-integrity causes. The basis contains no sidecar bytes or
memory meaning and is the narrowly store-owned nonsemantic coordination
effect compatible with read-only compile.

Before reopening any admission after restart, the store replays the atomic log
and reconciles every basis, tombstone, fence, revoke disposition, and origin
that may have reached postlookup resolution. A proven commit reconstructs the
same durable containment, its origin-specific terminal disposition, and
pending cleanup; a proven abort must revalidate the exact pinned read through
both phases; an unknown outcome stays fenced with
`IntegrityQuarantineReconciliationRequired` and has no terminal origin
disposition until commit or abort is proven. For a proven committed
containment, `Compile` records a terminal collision-revoked disposition with no
product and a permanently rejected release guard. `TerminalProbe` records a
terminal collision-observed disposition with no passing receipt and a rejected
result guard. `Management` records a terminal collision-blocked disposition
without its intended lifecycle mutation, keeps its lifecycle-commit guard
rejected, and requires fresh authorization for any later operation. Each
origin record and its live resources remain retained until
`CollisionTerminalRemovalStateV1` proves exact closure or durable recovery
fencing. Restart replays those exact origin-specific outcomes before
idempotent terminal removal and never resumes management from ambient
authority.

Collision resolution is the separate `CollisionRecoveryTransactionV1`, not
the ordinary exact-old-pair `QuarantineRecoveryTransactionV1`. It binds the
exact `CollisionQuarantineBasisV1`, trust key, permanent tombstone, current
fence generation, complete witness-set commitment, and complete
affected-custody closure commitment. Its closed disposition is either
`NewRegimeRepublication`, with a new authenticated regime, complete retained-
meaning correspondence, and new referring-record-version set, or
`CompleteAuthorizedErasure`, with exact erasure authorization and a complete
controlled-custody erasure-receipt commitment. It cannot choose one witness,
restore the old key, use exact-old-pair rollback, or treat later equality as
repair. Completion preserves the old tombstone and non-use rule across
republication, erasure, purge, restore, reprovisioning, reinstallation, and
store replacement; reopening requires the authenticated tombstone registry or
an equivalent append-only trust-root commitment to survive.

Physical deduplication is permitted only inside one authenticated
`ExactSidecarCustodyDomainId`. A custody ledger records every live
record-version reference and every retention-bearing rollback, backup, import,
export, or other controlled physical copy. Each
`SealedCustodyHoldingReferenceV1` has one
`SealedCustodyHoldingId` derived from the canonical immutable tuple of its
authenticated `RetentionObligationOwnerId`, retention-policy revision, trusted
authorization and retention interval, `RetentionObligationSourceV1`, complete
sidecar custody binding, and purpose. The source binds a closed purpose-kind
tag, source authority, never-reused monotonic source-event sequence,
content-derived source identity, and source revision. The source identity uses
the exact preimage selected by the owner contract. Caller-controlled owner,
policy, clock, source, sequence, nonce, or holding identity is forbidden. The
identity excludes current physical location and authorization state, so an
obligation remains stable across an atomic move or revocation.

Same-ID/same-obligation creation is idempotent and
same-ID/different-obligation fails closed. Independently authenticated source
events remain distinct holding entries even when every other obligation field
and the retained bytes are equal; deduplication cannot merge, shorten, satisfy,
or remove either obligation. A nonsemantic sealed-holding entry may retain
bytes only under its authenticated policy; it grants no compile, read, export,
or rollback authority. No untracked controlled copy may retain authoritative
bytes, and erasure claims are scoped explicitly to custody domains the system
controls.

Publication, migration, rollback creation, erasure, and garbage collection
atomically update the record state, logical and sealed-holding ledger entries,
and affected physical-object state. Erasing one record removes its logical
sidecar reference, derived artifacts, indexes, and access path; it never
deletes bytes required by another authorized entry and never permits the
erased record to recover them through deduplication. A record targeted by a
live nested reference cannot be erased independently: the same authorized
transaction must first publish verified replacement records without the edge
or remove every incoming logical reference and derived artifact. Otherwise
erasure fails without state change. Physical blob deletion is claimed only
after the final authorized logical or sealed-holding reference is removed,
inbound nested references are absent, collision quarantine does not apply, and
garbage collection is verified. Cross-custody-domain physical deduplication is
forbidden in V1. A referenced sidecar cannot be deleted independently of that
ledger.

Migration never rewrites a sidecar or record in place. Byte-identical canonical
sidecar content preserves its reference only when source and target use the
same complete `ExactSidecarIdentityRegimeV1`. A regime change, or a change to
schema, locator, type, presence, or value, derives a new typed sidecar identity,
reference, and record-version identity even when the decoded values or
canonical bytes happen to be equal. Migration validates the complete source
and target schema, regime, custody, and content bindings and publishes the
target pair plus every ledger and physical-object transition atomically. It
retains a rollback artifact only when the authenticated retention and erasure
policy permits that exact content for the required interval. If policy forbids
or later requires erasure, the migration cannot claim rollback eligibility to
that source; erasure removes that source's logical rollback reference and
derived artifacts through the privileged management path. Any bytes still
retained for another live record remain inaccessible to the erased source. An
eligible rollback restores the exact retained earlier immutable
record-sidecar-custody binding through the same atomic ledger transition.

Request-local proposition consolidation is explicitly a two-plane operation.
Typed numerical facets may order the complete admitted source and pair work,
but they never propose or define equivalence buckets and never omit a pair
from the exhaustive bounded reference set. They never establish claim
equivalence, merge authority, or resolve conflict. For every pair, the
boundary independently validates one
`AuthoritativePropositionProjectionV1` for each member. Its closed source union
is either `Request`, carrying a tagged request source identity, validated exact
request projection, source receipt, and attribution without persistent record
fields, or `Memory`, carrying the tagged memory source identity, immutable
record version, derived artifact, memory revision, complete exact-sidecar
custody binding, provenance root, and dependency group. Common projection
fields then decide:

- canonical proposition/schema identity;
- exact-value and presence equality;
- temporal, social, and modal scope compatibility;
- provenance and dependency identity;
- disclosure, authority, allowed-use, and surface-authority ceilings;
- current validity and supersession; and
- contradiction and conflict status.

No request variant can carry memory-only identities, and no memory variant can
omit them. Candidate nomination must be conservative for the registered
identity-equivalence contract. That predicate is registered and proven
reflexive, symmetric, and transitive over its exact identity fields. Request
variants bind their numerical source to the exact same tagged request
identity, exact request projection, source receipt, and attribution; memory
variants bind theirs to the exact same record version, derived artifact,
memory revision, and complete custody binding. Every singleton and both
members of every exhaustive pair are validated independently before
comparison.

Pair-work accounting uses proposition-plane sources, never activated-record
count. For each admitted memory record \(i\), the compiler validates a finite
canonical \(\mathcal S_i^M\) of paired numerical and authoritative
focus-visible memory sources, then forms
\[
\mathcal S_M=
\operatorname{CanonicalSort}
\left(\mathop{\biguplus}_i\mathcal S_i^M\right),
\qquad
n_{\mathrm{src}}^M=|\mathcal S_M|=\sum_i|\mathcal S_i^M|.
\]
A record may contribute zero, one, or several sources. With the disjoint
request set \(\mathcal R_Q\), the complete source set is
\(\mathcal S=\mathcal R_Q\uplus\mathcal S_M\) and
\(n=n_Q+n_{\mathrm{src}}^M\). Per-record, flattened-source, and pair-work
ceilings are authenticated and checked before materialization. The reference
pair count, source-set identity, exhaustive witness, workspace bound, and
quadratic comparison term all use this \(n\); \(n_a\) is not a substitute.

Scope overlap, authority compatibility, disclosure, validity, supersession,
and conflict are cluster-wide constraints rather than assumed equivalence
relations. The V1 reference algorithm validates the finite source set, checks
the configured pair-work ceiling before comparison, and enumerates every
unordered source pair in lexicographic canonical-source order. Numerical
facets may order that complete pair set but cannot omit a pair. Exceeding the
ceiling returns `Consolidation(PairWorkCapacityExceeded)` before a partial
partition exists. Any optimized implementation must provide a machine-checked
`ExhaustiveConsolidationPairSetWitnessV1` against that exact source-set
identity, exhaustive pair set, identity-equivalence contract, and checked
pair-work ceiling; otherwise it is invalid.

After pair validation, the registered identity-equivalence relation over
`EquivalenceIdentityProjectionV1` first partitions sources into buckets by
canonical equivalence key. Buckets and their members are ordered by canonical
source key. Inside each bucket, the reference algorithm visits sources in that
order. Clusters are ordered by their smallest member key. A source joins the
first cluster for which it is compatible with every existing member under
every cluster-wide constraint; if none qualifies, it creates one singleton
cluster at the canonical end. Cluster members remain canonically ordered. This
rule is the sole bounded complete-link reference partition. A nontransitive
compatibility triad can therefore never merge incompatible endpoints or
produce an implementation-order-dependent result. A numerical match cannot
override an authoritative mismatch, and authoritative fields are never
reconstructed from vectors. Request sources use the same rule with their
validated exact request projection and tagged attribution. The output
preserves the complete support, exact bindings, qualifications, and authority
meet. Consolidation remains request-local and read-only.

The closed construction errors are `UnknownIdentityRegime`,
`IdentityRegimeMismatch`, `UnknownSchema`, `SchemaIdentityMismatch`,
`UnknownCustodyDomain`, `CustodyDomainIdentityMismatch`, `InvalidBinding`,
`DuplicateLocator`, `InvalidPresence`, `LimitExceeded`, and
`NonCanonicalEncoding`. The closed memory-integrity causes are
`RecordVersionIdentityMismatch`, `ExactSidecarIdentityRegimeMismatch`,
`ExactSidecarSchemaMismatch`, `ExactSidecarCustodyMismatch`,
`ExactSidecarMissing`,
`ExactSidecarContentMismatch`, `ExactSidecarReferenceMismatch`,
`ExactSidecarContentIdentityCollision`,
and `ExactSidecarNestedReferenceInvalid`. Store import, publication, read,
migration, retrieval, consolidation, and planning adapters preserve the typed
cause and fail before returning partial semantic or product state. Public
compile and CLI mapping remains owned by the reference architecture.

Validation uses that declaration order. It performs no external sidecar
lookup until the record, regime, schema-reference, and custody-ledger
prechecks have passed, and no quarantine effect until postlookup
classification has selected collision after excluding every higher cause.
Postlookup envelope-schema and physical-custody faults still occupy their
declared public positions. `RecordVersionIdentityMismatch` means the containing
record-version
envelope, including the committed custody binding, does not recompute to its
stored record-version identity. `ExactSidecarIdentityRegimeMismatch` means the
revision, recomputed regime definition, reference, or typed content ID does
not name one identical regime. `ExactSidecarSchemaMismatch` means the
registered, reference, and envelope schemas disagree.
`ExactSidecarCustodyMismatch` means the record, custody ledger, authenticated
custody domain, and physical content binding disagree. `ExactSidecarMissing`
means no retained content exists for the authenticated reference.
`ExactSidecarContentMismatch` means retained bytes fail canonical decoding or
byte-identical re-encoding. `ExactSidecarReferenceMismatch` means the digest
recomputed from valid canonical content differs from the reference's
`content_id`.
`ExactSidecarContentIdentityCollision` means two distinct retained canonical
envelopes each independently validate and recompute to one typed content
identity under the same identity regime.
`ExactSidecarNestedReferenceInvalid` means canonical content and its reference
are valid but a nested record target is absent, erased, unresolved, unverified,
current-revision, forward, self, or same-publication.
The first applicable cause wins, independent of lookup or collection order.

`FocusCandidateError` is the closed outer tagged sum, in stage precedence
order: `EligibleActivatedSet(source)`, `RequestProposition(source)`,
`AuthoritativeProjection(source)`, `Consolidation(source)`,
`Capacity(source)`, and `CandidateInvariant(source)`. Each variant preserves
one cause from the following complete ordered inner families:

```text
FocusAggregateValidationError =
  UnknownEligibleSetSchema
| EligibleSetSchemaMismatch
| InvocationWitnessUnavailable
| EligibleSetWitnessUnavailable
| MissingSourceReceiptField
| DuplicateSourceReceiptField
| SourceReceiptIdentityMismatch
| SourceReceiptConfigurationMismatch
| ExactSidecarIntegrity(ExactSidecarIntegrityErrorV1)
| ActivatedRecordBindingMismatch
| InvalidActivationValue
| ActivationExplanationReferenceMismatch
| ProvenanceBindingMismatch
| AuthorityCeilingMismatch
| AllowedUseCeilingMismatch
| DuplicateActivatedRecord
| NonCanonicalActivatedRecordOrder
| ActivatedRecordLimitExceeded
| RetrievalCandidateLimitMismatch
| RetrievalCompletenessClassMismatch
| RetrievalIndexIdentityMismatch
| RetrievalRepresentationIdentityMismatch

RequestPropositionError =
  InvalidQueryBinding
| LineageMismatch
| InvalidSourceLocator
| UnknownSourceKind
| UnknownDerivation
| InvalidPropositionMeaning
| AuthorityMappingUnavailable
| InvalidExactBinding
| InvalidSupportScore
| DuplicateSourceIdentity
| DuplicateSourceOrderKey
| RequestPropositionLimitExceeded

AuthoritativePropositionProjectionError =
  ProjectionArtifactUnavailable
| ProjectionArtifactIdentityMismatch
| MissingProjection
| DuplicateProjection
| UnknownProjectionSchema
| ProjectionSchemaMismatch
| ForbiddenSourceVariant
| MissingSourceBindingField
| UnexpectedSourceBindingField
| SourceBindingMismatch
| ExactProjectionMismatch
| ExactSidecarIntegrity(ExactSidecarIntegrityErrorV1)
| CustodyBindingMismatch
| ProjectionLimitExceeded

PropositionConsolidationError =
  EquivalenceContractUnavailable
| InvalidEquivalenceContract
| CompleteLinkContractUnavailable
| InvalidExhaustivePairSetWitness
| ExhaustiveSourceSetMismatch
| ExhaustivePairSetMismatch
| ExhaustiveEquivalenceContractMismatch
| ExhaustivePairWorkCeilingMismatch
| PairWorkCapacityExceeded
| ComparisonWorkCapacityExceeded
| ConsolidationWorkspaceCapacityExceeded
| ExactSlotValueConflict
| ClusterCompatibilityViolation
| NonCanonicalPartition
| OptimizedPartitionMismatch

FocusCandidateCapacityError =
  CandidateLimitExceeded
| SupportBindingLimitExceeded
| ControlExclusionLimitExceeded

FocusCandidateConstructionError =
  UnknownFocusRole
| DuplicateFocusRole
| NonTotalFocusRoleOrder
| FocusCandidateOrderKeyMismatch
| DuplicateFocusCandidateOrderKey
| MissingRequiredQualification
| ExactBindingMismatch
| UnresolvedContradiction
| DuplicatePropositionSemanticKey
| EmptyCandidateSupport
| CandidateSupportPartitionMismatch
| CandidatePropositionIdentityMismatch
| InvalidCandidateActivation
| CandidateSourceReceiptMismatch
| CandidateInvocationWitnessMismatch
| CandidateEligibleSetWitnessMismatch
| CandidateAuthorityCeilingExceeded
| CandidateAllowedUseCeilingExceeded
| CandidateSurfaceAuthorityCeilingExceeded
| InvalidMandatoryClassification
| NonCanonicalCandidateSetOrder
```

The families contain \(22,12,14,15,3,21\) discriminants respectively.
`InvalidExactBinding` retains either
`InvalidExactSlotSemanticDescriptor` or `ExactSlotValueConflict` as its typed
source. Unknown serialized discriminants fail authenticated decoding; no
catch-all or message-only alternative exists.

Aggregate validation precedes request projection; all source projections
validate before exhaustive pair construction; consolidation completes before
capacity-controlled candidate construction. The outer `Capacity` variant is
reserved for bounded focus-candidate construction after consolidation;
pair-work exhaustion is the closed
`Consolidation(PairWorkCapacityExceeded)` cause. Within one stage, the inner
declaration order and then the smallest canonical affected key decide. Public
mappings are total and owned by the reference architecture; no adapter may
replace a typed source with message text or a different outer tag.

## Rationale

Content-addressing makes the exact bytes that can affect a claim part of the
record's immutable identity without putting rebuildable facets in the
authoritative record or creating a record-sidecar cycle. Reverification at
publication and read prevents a valid-looking reference from becoming an
unchecked trust boundary. Atomic visibility and immutable migration make
crash recovery and rollback reconstructible.

Two-plane consolidation preserves the computational value of numerical
representations while keeping claim identity, scope, provenance, disclosure,
authority, validity, and conflict under their authoritative source. A
complete-work contract plus an exhaustive-pair witness permits indexes,
canonical numerical work ordering, and other bounded acceleration without
allowing an optimization to omit a comparison or change the semantic
partition.

## Alternatives

- Store only an opaque sidecar locator. This permits rebinding and does not
  commit the record identity to exact content.
- Keep the regime outside `ExactSidecarRefV1`. Equal digest bytes across a
  regime rotation could then leave the serialized reference and record
  identity unchanged.
- Treat `ExactSidecarSchemaId` as an opaque registry label. Rebinding that label
  could reinterpret old canonical bytes without changing any content identity.
- Put the owning record-version identity in the sidecar identity. This creates
  a cyclic record-sidecar identity dependency.
- Include numerical artifacts in authoritative record identity. This makes
  encoder or index rebuilds rewrite source truth and conflicts with
  Decision 0024.
- Trust content identity without retaining and comparing canonical bytes.
  This cannot detect an observed same-identity/different-content collision.
- Deduplicate exact bytes across unrelated custody domains. One record's
  retention policy could then pin or delete another record's authoritative
  source.
- Bind custody to a mutable policy label or update its ledger separately from
  record publication. Policy rebinding or a crash could then expose a record
  without a valid retention owner.
- Leave backups, exports, imports, or other controlled retained copies outside
  the custody ledger. The last authorized reference and any erasure claim
  would then be unknowable.
- Quarantine only the first observed record after an identity collision. Every
  other reference to the ambiguous typed identity would remain untrustworthy.
- Return a collision to the compiler before a durable store-owned
  tombstone/fence handoff. A crash would leave an acknowledged collision
  usable after restart.
- Resolve sidecar content before authenticating the containing record and its
  custody binding. Corrupt record bytes could probe unrelated content or
  trigger quarantine effects.
- Let prelookup and postlookup checks return their first encountered error.
  Physical schema/custody checks and storage iteration order could then change
  the public cause.
- Key sealed retention only by content, purpose, or policy. Independent
  obligations could collapse, expire, or be erased together.
- Let numerical similarity decide consolidation. Similar meanings may differ
  in exact value, scope, authority, validity, or contradiction state.
- Permit numerical nomination to omit candidate pairs. This makes semantic
  results depend silently on an incomplete optimization.
- Leave complete-link cluster selection implicit. A nontransitive
  compatibility graph could then produce different partitions under different
  iteration orders.
- Perform persistent consolidation during compile. This violates the
  semantically read-only V1 compile boundary.

## Consequences

Memory-domain implementations require content-derived immutable schema and
identity-regime definitions, checked constructors, complete serialized
references and custody bindings, domain-separated identities, atomic
record-sidecar-ledger publication, prelookup record authentication,
revision-pinned verified reads, permanent trust-domain-wide collision
quarantine, a complete logical/sealed-holding custody ledger, and immutable
migration fixtures. Tests must cover permutation invariance; every regime,
schema definition, custody-domain definition, locator, type, presence, and
value mutation; equal digest bytes under different regimes; attempted schema,
regime, and custody-domain ID rebinding; stable explicit empty content;
malformed and duplicate bindings; missing, truncated, corrupt, rebound,
cross-revision, and cross-record binding substitutions; rejection before
external lookup when any record/regime/schema-reference/custody-ledger
precheck fails; the same public precedence for postlookup envelope-schema and
physical-custody faults; same-identity/same-bytes
idempotence and valid same-reference reuse within one compatible custody
domain; prohibited cross-domain physical deduplication; one of several
references erased without dangling or resurrection; stable sealed-holding
identity across physical moves, distinct authenticated source obligations
with otherwise equal fields, one obligation removed without affecting
another, and rejection of caller-controlled obligation identity fields;
retained-copy and inbound
nested-reference handling; verified last-reference physical garbage
collection; same-identity/different-bytes cases in which neither, one, or both
contents independently recompute to the claimed identity; complete collision
fan-out quarantine and new-regime recovery across records, artifacts, backups,
exports, caches, migrations, and restores; collision-handoff fault injection
before, during, and after its linearization point, including committed,
aborted, and unknown restart outcomes plus conservative whole-store fencing
when reachability is unproved; all three origin constructors and cross-origin
substitution rejection; atomic logical revocation without synchronous resource
destruction; positive cleanup limits, monotonic cursors, bounded repeated
terminal-removal steps, and committed, aborted, and reconciliation-required
cleanup outcomes; final linearizable compile-product, terminal-probe-pass, and
management-lifecycle guards under every containment race; atomicity across
every publication, migration, rollback, erasure, and garbage-collection fault
point; same-regime and rotated-regime migration; policy-permitted rollback,
policy-forbidden retention, later erasure; rejection of independent deletion;
and prior-revision target existence plus transitive reference well-foundedness
with its exact typed failure.

Consolidation implementations require the finite exhaustive unordered-pair
reference algorithm or a machine-checked identical pair-set witness, explicit
pre-partition capacity failure, independently validated closed-union
authoritative projections for both pair members, a registered equivalence
relation for identity fields, the canonical first-compatible complete-link
partition, and cross-plane substitution tests. The outer
`FocusCandidateError` retains the exact six variants and precedence declared
above, including every typed inner source. Tests must prove that numerical
permutation or optimization cannot change the exhaustive pair set or
authoritative partition, a numerical match cannot merge an exact or scoped
conflict, every ordering of a nontransitive compatibility triad produces the
same canonical safe partition, authoritative equality cannot be fabricated
from a vector, request variants cannot acquire persistent memory identity or
be swapped across otherwise equal tags, memory variants cannot omit their
record/artifact/revision/custody bindings, every singleton and pair member is
validated, records contributing zero/one/many proposition-plane sources use
the flattened \(n_{\mathrm{src}}^M\) for the source-set identity, pair count,
work ceiling, and complexity bound, and every emitted proposition reconstructs
its complete source and authority bindings. Every discriminant in all six
closed inner families must be constructible in tests and map exactly once with
its unchanged typed source, canonical cross-stage and within-stage precedence,
fixed retry and disposition behavior, and no partial candidate set. Unknown
authenticated discriminants must fail decoding before focus construction.

This decision refines Decisions 0014, 0018, and 0024. It does not supersede
their predictive architecture, identity, or record/artifact separation, and
it does not select a physical database, digest implementation, encoder,
vector space, or persistent consolidation policy. Digest uniqueness remains
conditional on canonical-encoding injectivity and the named
collision-resistance assumption; an observed collision always fails closed.
