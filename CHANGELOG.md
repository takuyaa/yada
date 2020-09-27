# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/takuyaa/yada/compare/0.3.0...HEAD
[0.3.0]: https://github.com/takuyaa/yada/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/takuyaa/yada/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/takuyaa/yada/releases/tag/0.1.0
