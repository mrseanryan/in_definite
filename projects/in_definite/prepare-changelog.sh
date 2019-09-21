input=../../CHANGELOG.md

echo -e "## [Unreleased] - ReleaseDate\n### Added\n\n### Changed\n\n$(cat $input)" > $input
