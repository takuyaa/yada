# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/takuyaa/yada/compare/0.4.0...HEAD
[0.4.0]: https://github.com/takuyaa/yada/compare/0.3.2...0.4.0
[0.3.2]: https://github.com/takuyaa/yada/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/takuyaa/yada/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/takuyaa/yada/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/takuyaa/yada/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/takuyaa/yada/releases/tag/0.1.0
