//! FFI wrappers for the Linux Pluggable Authentication Modules (PAM)
//!
//! This crate provides raw access to the Linux-PAM API.
//! Constants, types and functions are supported and created with `bindgen`.
//!
//! Note: Currently only tested on Linux as I lack access to other OSes at
//! the moment. Both `build.rs` and `wrapper.h` probably need to be customized
//! to exclude missing libraries such as `pam_misc` when they are not present.

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    deref_nullptr
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
