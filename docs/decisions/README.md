# Decisions

Decision records preserve the context and consequences of significant technical choices.

Use sequential four-digit identifiers and lowercase kebab-case filenames, for example `0001-example-decision.md`. Start from `TEMPLATE.md` and remove all placeholder text.

Allowed statuses:

- `Proposed`: under discussion.
- `Accepted`: selected for implementation.
- `Rejected`: considered and not selected.
- `Superseded`: replaced by a later decision record.

Do not modify or delete an accepted, rejected, or superseded record. To replace an accepted decision, add a new record and change only the previous record's status to `Superseded` while adding `Superseded by: <filename>` below its date.
