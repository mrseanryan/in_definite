## [Unreleased] - ReleaseDate\n### Added\n\n### Changed\n\n## [Unreleased] - ReleaseDate
### Added

### Changed

## [0.2.5] - 2022-03-12
### Changed
- Improved the badges in the README

## [0.2.4] - 2020-11-14
### Added
- Irregular single letters: f, h, l, m, n, r, s, u, x

### Changed

## [0.2.3] - 2020-03-18
### Added
- Irregular words: Utah, utahn

## [0.2.2] - 2020-03-15
### Changed
- Minor internal cleaning up
- Removed dependency on the `regex` crate

## [0.2.1] - 2019-09-24
### Changed
- Internal refactoring and cleaning up

## [0.2.0] - 2019-09-24
### Changed
- *Breaking change* Improved API that correctly handles empty or all-whitespace input.
- `is_an(word: &str) -> bool` changed to `is_an(word: &str) -> Is` where Is is an enum: A, An, None.

## [0.1.10] - 2019-09-21
### Changed
- Add 'english' keyword to crate.

## [0.1.9] - 2019-09-21
### Changed
- Add version badges.
- Run tests before publish.
- Exit publish script immediately on error.

## [0.1.8] - 2019-09-21
### Changed
Improve publish scripts.

## [0.1.7] - 2019-09-21
### Changed
Adjust publish script, to prepare changelog for continuing dev.
## [0.1.6] - 2019-09-21
### Changed
Adjust cargo-release settings.

## [0.1.5] - 2019-09-21
### Changed
- Added irregular words euro, urologist.
- Added more tests, especially around eu- words.

## [0.1.4] - 2019-09-19
### Changed
Fix readme about case handling.

## [0.1.3] - 2019-09-19
### Added
Handle title and mixed cases. 

### Changed
Add more tests.

## [0.1.2] - 2019-09-18
### Changed
Improved description of crate.

## [0.1.1] - 2019-09-18
### Added
Initial release, porting indefinite from npm.
