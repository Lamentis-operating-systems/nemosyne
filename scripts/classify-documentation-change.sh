#!/usr/bin/env bash

set -euo pipefail

if [[ "$#" -ne 6 ]]; then
  printf 'usage: %s <event-name> <event-actor> <pr-author> <head-repository> <repository> <head-ref>\n' "$0" >&2
  exit 2
fi

event_name="$1"
event_actor="$2"
pull_request_author="$3"
head_repository="$4"
repository="$5"
head_ref="$6"

if [[ "$event_name" == 'pull_request' &&
  "$event_actor" == 'dependabot[bot]' &&
  "$pull_request_author" == 'dependabot[bot]' &&
  -n "$head_repository" &&
  -n "$repository" &&
  "$head_repository" == "$repository" &&
  "$head_ref" == dependabot/github_actions/?* ]]; then
  printf 'skip\n'
else
  printf 'validate\n'
fi
