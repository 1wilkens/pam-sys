# pam-sys - Rust FFI bindings to the Linux Pluggable Authentication Modules (PAM)

[![Crates.io](https://img.shields.io/crates/v/pam-sys.svg)](https://crates.io/crates/pam-sys)
[![Documentation](https://docs.rs/pam-sys/badge.svg)](https://docs.rs/pam-sys/)
[![Build Status Azure](https://dev.azure.com/1wilkens/ci/_apis/build/status/pam-sys?branchName=master)](https://dev.azure.com/1wilkens/ci/_build/latest?definitionId=1&branchName=master)
[![Build Status Cirrus](https://img.shields.io/cirrus/github/1wilkens/pam-sys/master?style=flat-square)](https://cirrus-ci.com/github/1wilkens/pam-sys)
[![License](https://img.shields.io/crates/l/pam-sys.svg?branch=master)](https://travis-ci.org/1wilkens/pam-sys)

This crate uses [`bindgen`](https://github.com/rust-lang/rust-bindgen) to generate the raw FFI definitions for PAM. For a rustified API consider using [`pam`](https://github.com/1wilkens/pam).

## Supported Rust versions
The library is only continuously built against Rust stable, beta and nightly but as it does not use a lot of new language features it should probably compile on older versions as well.
If you encounter problems building on older versions and a small fix can be applied to make the build succeed, consider opening a pull request.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
