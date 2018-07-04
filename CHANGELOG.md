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

## [0.5.6] - 2018-07-04
### Fixed
- Fixed NULL pointer usage in wrapped::get_env function when pam_getenv fails

## [0.5.5] - 2018-03-21
### Added
- Implement `pam_get_user`

### Changed
- Addded `cache: cargo` directive to speedup CI
- Updated `libc` dependency (0.2.33 -> 0.2.39)

## [0.5.4] - 2017-11-30
### Changed
- Only provide official support for Rust stable, beta and nightly (mainly through travis)
- Updated `libc` dependency (0.2.20 -> 0.2.33)

### Fixed
- Only link `pam_misc` on Linux

## [0.5.3] - 2017-02-18
### Fixed
- Fixed `readonly` flag in `pam_misc_setenv`
- Added some `null` checks for some `wrapped` functions

## [0.5.2] - 2017-02-18
### Fixed
- Fixed missing link argument to `pam_misc`

## [0.5.1] - 2017-02-17
### Fixed
- Fixed compilation failure on Rust 1.4.0
- Fixed wrong category

## [0.5.0] - 2017-02-17
### Added
- Added travis-ci badge to `Cargo.toml`
- Added categories to `Cargo.toml`

### Changed
- Changed `raw::pam_getenvlist`'s return type back to `*const *const c_char`
- Made `wrapped` functions safe and changed arguments to use better types

### Fixed
- Fixed `raw::pam_misc_dropenv` and `wrapped::misc_dropenv`'s return type

## [0.4.3] - 2017-01-20
### Added
- Added license badge to `README.md`

### Changed
- Updated `libc` dependency (0.2.9 -> 0.2.20)
- Moved documentation to [docs.rs](https://docs.rs/pam-sys/)

### Removed
- Removed obsolete dependencies (`gcc` & `pkg-config`)
- Removed `.travis-update-gh-pages.sh` and obsolete rust versions from `.travis.yml`

## [0.4.1] - 2016-04-11
### Changed
- Relicense under MIT/Apache-2.0

## [0.4.0] - 2016-04-11
### Added
- Better travis-ci integration
    - Test on 1.4.0-1.6.0, stable, beta and nightly
    - Use containers for faster builds

### Changed
- Updated `libc` dependency (0.2.2 -> 0.2.9)
- Changed wrapped/{strerror, getenv} to return `Option<&'static str>` instead of `*const c_char`

## [0.3.0] - 2015-12-08
### Added
- CHANGELOG.md

### Changed
- Updated `libc` dependency (0.1.8 -> 0.2.2)
- PamHandle from empty struct to zero variant enum (as recommended in [the Rust Book](https://doc.rust-lang.org/nightly/book/ffi.html#representing-opaque-structs))


[Unreleased]: https://github.com/1wilkens/pam-sys/compare/v0.5.6...HEAD
[0.5.6]: https://github.com/1wilkens/pam-sys/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/1wilkens/pam-sys/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/1wilkens/pam-sys/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/1wilkens/pam-sys/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/1wilkens/pam-sys/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/1wilkens/pam-sys/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/1wilkens/pam-sys/compare/v0.4.3...v0.5.0
[0.4.3]: https://github.com/1wilkens/pam-sys/compare/v0.4.1...v0.4.3
[0.4.1]: https://github.com/1wilkens/pam-sys/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/1wilkens/pam-sys/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/1wilkens/pam-sys/compare/f051f14b76ad1e06be1832604e0ca570743460ac...v0.3.0
