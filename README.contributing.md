# Contributing to in_definite

Contributions to in_definite are welcome.

Please try to:

- follow the existing style and setup, unless of course there is good reason to deviate.
- run unit tests (`./test.sh`) and e2e tests (`./test.e2e.local.sh`).
- add a matching entry to the CHANGELOG.md (note: the version and date will be automatically populated on publish).


## Notes on Publishing (for owners only)

1. Add and review entries in the CHANGELOG.md.
  - do not edit `## [Unreleased] - ReleaseDate`, it will be automatically populated on publish.
2. Run the e2e test, that consumes the local package: `./test.e2e.local.sh`.
3. Run `publish-dry-run.sh` (this includes formatting and unit tests).
4. Check the results. If all OK, then run `publish.sh` (this includes formatting and unit tests).
5. Bump the version in the test harness - see [Cargo.toml](./tests_e2e/in_definite_cmd_published/Cargo.toml)
6. Run the e2e test, that consumes the published package: `./test.e2e.published.sh`.
