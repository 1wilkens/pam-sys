# Changelog
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

<!--
### Added - for new features.
### Changed - for changes in existing functionality.
### Deprecated - for once-stable features removed in upcoming releases.
### Removed - for deprecated features removed in this release.
### Fixed - for any bug fixes.
### Security - to invite users to upgrade in case of vulnerabilities.
-->

## [Unreleased]
- Better travis-ci integration (test on stable, beta and nightly)

## [0.3.0] - 2015-12-08
### Added
- CHANGELOG.md

### Changed
- Updated `libc` dependency (0.1.8 -> 0.2.2)
- PamHandle from empty struct to zero variant enum (as recommended in [the Rust Book](https://doc.rust-lang.org/nightly/book/ffi.html#representing-opaque-structs))


[Unreleased]: https://github.com/mrfloya/pam-sys/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/mrfloya/pam-sys/compare/f051f14b76ad1e06be1832604e0ca570743460ac...v0.3.0
