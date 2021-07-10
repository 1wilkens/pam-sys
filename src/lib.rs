//! FFI wrappers for the Linux Pluggable Authentication Modules (PAM)
//!
//! This crate provides raw access to the Linux-PAM API.
//! Constants, types and functions are supported and created with `bindgen`.
//!
//! Note: Currently only tested on linux as I lack access to other OSes at
//! the moment. Both `build.rs` and `wrapper.h` probably need to be customized
//! to exclude missing libraries such as `pam_misc` when they are not present.

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_os = "freebsd")]
mod enums {
    // PAM flags
    pub const PAM_SILENT: libc::c_uint = 0x8000_0000;
    pub const PAM_DISALLOW_NULL_AUTHTOK: libc::c_uint = 1;
    pub const PAM_ESTABLISH_CRED: libc::c_uint = 1;
    pub const PAM_DELETE_CRED: libc::c_uint = 2;
    pub const PAM_REINITIALIZE_CRED: libc::c_uint = 4;
    pub const PAM_REFRESH_CRED: libc::c_uint = 8;
    pub const PAM_PRELIM_CHECK: libc::c_uint = 1;
    pub const PAM_UPDATE_AUTHTOK: libc::c_uint = 2;
    pub const PAM_CHANGE_EXPIRED_AUTHTOK: libc::c_uint = 4;
    // flags not defined in FreeBSD
    pub const PAM_CONV_AGAIN: libc::c_uint = 30;
    pub const PAM_INCOMPLETE: libc::c_uint = 310;
    pub const PAM_FAIL_DELAY: libc::c_uint = 10;
    pub const PAM_XDISPLAY: libc::c_uint = 11;
    pub const PAM_XAUTHDATA: libc::c_uint = 12;
    pub const PAM_AUTHTOK_TYPE: libc::c_uint = 13;
}

// Non-compat Linux definitions
// FIXME: use compat versions here?
#[cfg(not(target_os = "freebsd"))]
mod enums {
    pub const PAM_SILENT: libc::c_uint = 0x8000;
    pub const PAM_DISALLOW_NULL_AUTHTOK: libc::c_uint = 0x1;
    pub const PAM_ESTABLISH_CRED: libc::c_uint = 0x2;
    pub const PAM_DELETE_CRED: libc::c_uint = 0x4;
    pub const PAM_REINITIALIZE_CRED: libc::c_uint = 0x8;
    pub const PAM_REFRESH_CRED: libc::c_uint = 0x10;
    pub const PAM_PRELIM_CHECK: libc::c_uint = 0x4000;
    pub const PAM_UPDATE_AUTHTOK: libc::c_uint = 0x2000;
    pub const PAM_CHANGE_EXPIRED_AUTHTOK: libc::c_uint = 0x20;
}

pub use enums::*;
