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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PamImplementation {
    LinuxPAM,
    OpenPAM,
}

pub const PAM_IMPLEMENTATION: PamImplementation = {
    #[cfg(PAM_SYS_IMPL = "linux-pam")]
    {
        PamImplementation::LinuxPAM
    }

    #[cfg(PAM_SYS_IMPL = "openpam")]
    {
        PamImplementation::OpenPAM
    }

    #[cfg(not(any(
        PAM_SYS_IMPL = "linux-pam",
        PAM_SYS_IMPL = "openpam",
    )))]
    compile_error!("No valid PAM implementation selected")
};

#[cfg(any(doc, PAM_SYS_IMPL = "linux-pam"))]
pub mod linux_pam;
#[cfg(any(doc, PAM_SYS_IMPL = "openpam"))]
pub mod openpam;

#[cfg(PAM_SYS_IMPL = "linux-pam")]
pub use linux_pam::*;

#[cfg(PAM_SYS_IMPL = "openpam")]
pub use openpam::*;
