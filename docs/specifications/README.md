# Specifications

Specifications are the current source of truth for observable behavior, mathematical definitions, preconditions, invariants, edge cases, and verification.

Store specifications as regular Markdown files directly in this directory. Use lowercase kebab-case filenames. Start from `TEMPLATE.md` and remove all placeholder text.

Keep `Status` on line 3. A superseded specification places `Superseded by` on line 4.

Allowed statuses:

- `Proposed`: under discussion and not implemented.
- `Experimental`: implemented but not validated as a stable contract.
- `Validated`: supported by sufficient implementation and evaluation evidence.
- `Superseded`: replaced by a newer specification.

A behavior change must update its specification in the same pull request.

To replace a specification, add a new `Experimental` or `Validated` specification in the same change. Set the previous specification to `Superseded` and add `Superseded by: <filename>` below its status. Superseded specifications are immutable.
