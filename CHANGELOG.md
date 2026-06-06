# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2026-06-06
### Added
- Validation in `DoubleArray::new` to prevent undefined behavior when constructing from malformed byte slices ([#24](https://github.com/takuyaa/yada/issues/24)), by [@vabr-g](https://github.com/vabr-g) and [@kampersanda](https://github.com/kampersanda).
- `DoubleArray::new_unchecked` (unsafe) for callers who can guarantee validity and want to skip the validation cost, by [@vabr-g](https://github.com/vabr-g) and [@kampersanda](https://github.com/kampersanda).
- `YadaError` variants `EmptyDoubleArray`, `UnalignedDoubleArray`, `UnalignedDoubleArrayBlocks`, and `InvalidDoubleArrayUnit` for byte slice validation failures, by [@vabr-g](https://github.com/vabr-g) and [@kampersanda](https://github.com/kampersanda).
- `Debug` impl for `DoubleArray`.

### Changed
- **BREAKING:** `DoubleArray::new` now returns `Result<Self, YadaError>` instead of `Self`, by [@vabr-g](https://github.com/vabr-g) and [@kampersanda](https://github.com/kampersanda).

## [0.6.0] - 2026-06-06
### Added
- `YadaError` enum and `errors` module with explicit error variants for build failures, by [@kampersanda](https://github.com/kampersanda).
- Keyset validation in `DoubleArrayBuilder::build`: detects empty keysets, empty keys, keys containing NUL bytes, duplicate keys, unsorted keysets, values greater than `2^31 - 1`, and tries exceeding `2^29` units, by [@kampersanda](https://github.com/kampersanda).
- `Default` impl for `DoubleArrayBuilder` and `Unit`.
- `Display` impl for `Unit`.
- GitHub Actions CI: `test` (Linux x86/ARM, macOS, Windows), `fmt`, `clippy`, `doc`, and `msrv` jobs.
- GitHub Actions release workflow: SemVer tag pushes verify the version, run tests, publish to crates.io via Trusted Publishing (OIDC), and create a GitHub Release from the matching `CHANGELOG.md` section.
- `RELEASE.md` documenting the maintainer release procedure.
- `rust-version = "1.58.0"` in `Cargo.toml`.

### Changed
- **BREAKING:** `DoubleArrayBuilder::build` now returns `Result<Vec<u8>, YadaError>` instead of `Option<Vec<u8>>`, by [@kampersanda](https://github.com/kampersanda).
- Skip unnecessary recursion into leaf nodes during build, by [@kampersanda](https://github.com/kampersanda).
- Update README MSRV from `1.46.0` to `1.58.0`.

### Removed
- **BREAKING:** Removed the stateful `build_from_keyset` API. Use `DoubleArrayBuilder::build` instead, by [@kampersanda](https://github.com/kampersanda).

## [0.5.1] - 2024-02-25
### Changed
- Fix a corner case of `exact_match_search` by [@BlueGreenMagick](https://github.com/BlueGreenMagick).
- Add a test for the corner case.

## [0.5.0] - 2021-11-01
### Changed
- Improve search performance using `get_unchecked`.

## [0.4.1] - 2021-11-01
### Changed
- Improve search performance thnaks to inlining by [@kampersanda](https://github.com/kampersanda).

## [0.4.0] - 2020-10-11
### Added
- Add benchmarks.
- Add file converter script for benchmarks.

### Changed
- Relax the maximum size of a double array.

## [0.3.2] - 2020-10-01
### Changed
- Make the `DoubleArray` cloneable by [@johtani](https://github.com/johtani).

## [0.3.1] - 2020-09-30
### Changed
- Fix duplicated offset issue.

## [0.3.0] - 2020-09-28
### Added
- Add an example `load_from_file`.

### Changed
- `DoubleArrayBuilder` does not require a trailing null character to build double arrays.
- `exact_match_search` does not require a trailing null character in the keys.

## [0.2.0] - 2020-09-26
### Added
- Specify Rust version by [@johtani](https://github.com/johtani).
- Add CHANGELOG.md.

### Changed
- `common_prefix_search` returns key length.

## [0.1.0] - 2020-09-20
### Added
- Initial release.

[Unreleased]: https://github.com/takuyaa/yada/compare/0.7.0...HEAD
[0.7.0]: https://github.com/takuyaa/yada/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/takuyaa/yada/compare/0.5.1...0.6.0
[0.5.1]: https://github.com/takuyaa/yada/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/takuyaa/yada/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/takuyaa/yada/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/takuyaa/yada/compare/0.3.2...0.4.0
[0.3.2]: https://github.com/takuyaa/yada/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/takuyaa/yada/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/takuyaa/yada/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/takuyaa/yada/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/takuyaa/yada/releases/tag/0.1.0
