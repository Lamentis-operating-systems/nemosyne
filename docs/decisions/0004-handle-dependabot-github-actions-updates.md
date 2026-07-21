# 0004: Handle Dependabot GitHub Actions updates

Status: Accepted
Date: 2026-07-21

## Context

Dependabot does not use the repository pull request template. Its GitHub Actions dependency updates therefore fail change-aware documentation validation even though the generated branch is limited to dependency maintenance.

## Decision

Keep the Documentation job and its structural checks active for every pull request. Use a deterministic classifier to skip only its change-aware pull request validation when all of these GitHub-controlled values match: the event is a pull request, the event actor is `dependabot[bot]`, the pull request author is `dependabot[bot]`, the head repository is this repository, and the head branch starts with `dependabot/github_actions/` followed by a non-empty dependency path.

Any human-triggered pull request event, external repository, different author, or different Dependabot ecosystem continues through normal change-aware validation. Treat the matching GitHub Actions dependency update as automated governance maintenance that does not require a separate decision record in the generated pull request. Classifier errors fail the Documentation job.

## Rationale

The Documentation check remains visible and required while generated GitHub Actions dependency updates no longer depend on human-authored metadata. Combining event, actor, author, repository, and ecosystem branch scope is narrower than trusting the branch name or bot identity alone. A tested classifier keeps the policy independent of workflow expression syntax.

## Alternatives

Skipping the entire workflow would remove structural evidence. Editing generated pull requests would defeat unattended maintenance. Parsing every GitHub Actions dependency diff would add a second workflow-language validator for a case already constrained by GitHub-controlled Dependabot metadata.

## Consequences

GitHub Actions Dependabot updates can merge after the remaining required checks pass. Cargo updates are not covered. A maintainer event on the Dependabot branch restores normal change-aware validation. Regression tests cover the accepted trust combination and fail-closed alternatives.
