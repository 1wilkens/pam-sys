//! FFI wrappers for the Linux Pluggable Authentication Modules (PAM)
//!
//! This crate provides easy access to the Linux-PAM. Both raw and wrapped versions
//! of the common functions from `libpam.so` are provided as well as types used in
//! these functions.
//!
//! Note: Not all functions are fully supported yet. For now mainly the ones required
//! for authentication with PAM rather than writing a custom PAM module are provided.
extern crate libc;

pub use crate::types::*;
pub use crate::wrapped::*;

pub mod raw;
#[allow(non_camel_case_types, clippy::match_same_arms)]
pub mod types;
pub mod wrapped;
