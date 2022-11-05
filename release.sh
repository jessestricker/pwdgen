#! /usr/bin/env bash

set -e -u -o pipefail

# parse inputs
version="$1"

# require clean working tree
change_count=$(git status --porcelain 2>/dev/null | wc -l)
if [[ ${change_count} -ne 0 ]]; then
  echo "error: the working tree must be clean"
  exit 1
fi

# set version in manifest and lockfile
cargo set-version "${version}"
cargo check --quiet # syncs version in lockfile

# update changelog
tag="v${version}"
git-cliff --tag="${tag}" --output=CHANGELOG.md

# create release commit
git add --all
git commit --message="chore: release ${version}"

# create release tag
release_date=$(git log --max-count=1 --pretty=%aD)
GIT_COMMITTER_DATE="${release_date}" git tag --sign --message="Release ${version}" --cleanup=whitespace "${tag}"

# push release commit and tag
git push origin HEAD "${tag}"
