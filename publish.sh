#!/bin/bash
set -e  # exit when any command fails

./pre-publish.sh

echo "Publishing..."

pushd in_definite

cargo release --no-confirm --execute  # Will stop if git has uncommitted changes.

popd

./_prepare-changelog.sh
git push

echo "Publishing [done]"
