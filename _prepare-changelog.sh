# exit when any command fails
set -e

input=CHANGELOG.md

echo "Preparing changelog ..."

echo "## [Unreleased] - ReleaseDate\n### Added\n\n### Changed\n\n$(cat $input)" > $input

git unstage
git add $input
git ci -m "Prepare changelog for further work."

echo "Preparing changelog [done]"
