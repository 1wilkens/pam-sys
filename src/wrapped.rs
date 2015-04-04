//! Wrapped FFI bindings to Linux-PAM
//!
//! This module provides wrapped versions of some of the functions from
//! the [`raw`](../raw/index.html) module which use the appropriate enums
//! instead of `c_int`. These wrappers should always be preferred as one
//! can safely match on their return types and prevent illegal arguments
//! from beeing passed to the native library.
//!
//! Note: These wrappers get added as I need them. Feel free to open an issue
//! or PR for the ones that you require which haven't been added yet.

use libc::{c_char, c_int, c_void};

use types::*;
use raw::*;

#[inline]
pub unsafe fn start(service: *const c_char, user: *const c_char,
    conversation: *const PamConversation, handle: *mut *mut PamHandle) -> PamReturnCode {
    PamReturnCode::from_i32(pam_start(service, user, conversation, handle as *mut *const PamHandle))
}

#[inline]
pub unsafe fn end(handle: *mut PamHandle, status: c_int) -> PamReturnCode {
    PamReturnCode::from_i32(pam_end(handle, status))
}

#[inline]
pub unsafe fn authenticate(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_authenticate(handle, flags as i32))
}

#[inline]
pub unsafe fn acct_mgmt(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_acct_mgmt(handle, flags as i32))
}

#[inline]
pub unsafe fn setcred(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_setcred(handle, flags as i32))
}

#[inline]
pub unsafe fn open_session(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_open_session(handle, flags as i32))
}

#[inline]
pub unsafe fn close_session(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_close_session(handle, flags as i32))
}

#[inline]
pub unsafe fn set_item(handle: *mut PamHandle, item_type: PamItemType, item: *const c_void) -> PamReturnCode {
    PamReturnCode::from_i32(pam_set_item(handle, item_type as i32, item))
}

#[inline]
pub unsafe fn get_item(handle: *const PamHandle, item_type: PamItemType, item: *const *const c_void) -> PamReturnCode {
    PamReturnCode::from_i32(pam_get_item(handle, item_type as i32, item))
}
