#! /usr/bin/env bash

set -e -u -o pipefail

# parse inputs
version="$1"

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
