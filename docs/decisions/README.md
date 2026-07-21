# Decisions

Decision records preserve the context and consequences of significant technical choices.

Store records as regular Markdown files directly in this directory. Use sequential four-digit identifiers and lowercase kebab-case filenames, for example `0001-example-decision.md`. Start from `TEMPLATE.md` and remove all placeholder text.

Keep `Status` on line 3 and `Date` on line 4. A superseded record places `Superseded by` on line 5.

Allowed statuses:

- `Proposed`: under discussion.
- `Accepted`: selected for implementation.
- `Rejected`: considered and not selected.
- `Superseded`: replaced by a later decision record.

Do not modify or delete an accepted, rejected, or superseded record. To replace an accepted decision, add a later-numbered accepted record in the same change. Change only the previous record's status to `Superseded` and add `Superseded by: <filename>` below its date.
