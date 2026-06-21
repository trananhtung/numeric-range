# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-21

### Added

- Initial release.
- `parse` — expand a compact range string (`"1,3-5,7"`) into `Vec<u64>`.
- `format` — collapse integers into a compact range string (sorted, de-duplicated).
- `ParseError` for empty parts, invalid numbers, malformed ranges, and descending ranges.
- `#![no_std]` support (requires `alloc`); zero dependencies.

[0.1.0]: https://github.com/trananhtung/numeric-range/releases/tag/v0.1.0
