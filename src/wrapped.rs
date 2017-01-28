// Copyright (C) 2015 Florian Wilkens
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
// OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
use std::ffi::CStr;

use types::*;
use raw::*;

/* ------------------------ pam_appl.h -------------------------- */
#[inline]
pub unsafe fn start(service: *const c_char,
                    user: *const c_char,
                    conversation: *const PamConversation,
                    handle: *mut *mut PamHandle)
                    -> PamReturnCode {
    From::from(pam_start(service, user, conversation, handle as *mut *const PamHandle))
}

#[inline]
pub unsafe fn end(handle: *mut PamHandle, status: PamReturnCode) -> PamReturnCode {
    From::from(pam_end(handle, status as c_int))
}

#[inline]
pub unsafe fn authenticate(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(pam_authenticate(handle, flags as c_int))
}

#[inline]
pub unsafe fn setcred(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(pam_setcred(handle, flags as c_int))
}

#[inline]
pub unsafe fn acct_mgmt(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(pam_acct_mgmt(handle, flags as c_int))
}

#[inline]
pub unsafe fn open_session(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(pam_open_session(handle, flags as c_int))
}

#[inline]
pub unsafe fn close_session(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(pam_close_session(handle, flags as c_int))
}

#[inline]
pub unsafe fn chauthtok(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(pam_chauthtok(handle, flags as c_int))
}
/* ------------------------ pam_appl.h -------------------------- */

/* ----------------------- _pam_types.h ------------------------- */
#[inline]
pub unsafe fn set_item(handle: *mut PamHandle,
                       item_type: PamItemType,
                       item: *const c_void)
                       -> PamReturnCode {
    From::from(pam_set_item(handle, item_type as c_int, item))
}

#[inline]
pub unsafe fn get_item(handle: *const PamHandle,
                       item_type: PamItemType,
                       item: *mut *const c_void)
                       -> PamReturnCode {
    From::from(pam_get_item(handle, item_type as c_int, item))
}

#[inline]
pub unsafe fn strerror<'a>(handle: &'a mut PamHandle, errnum: PamReturnCode) -> Option<&'a str> {
    CStr::from_ptr(pam_strerror(handle, errnum as c_int)).to_str().ok()
}

#[inline]
pub unsafe fn putenv(handle: *mut PamHandle, name_value: *const c_char) -> PamReturnCode {
    From::from(pam_putenv(handle, name_value))
}

#[inline]
pub unsafe fn getenv<'a>(handle: &'a mut PamHandle, name: *const c_char) -> Option<&'a str> {
    CStr::from_ptr(pam_getenv(handle, name)).to_str().ok()
}

#[inline]
pub unsafe fn getenvlist(handle: *mut PamHandle) -> *mut *mut c_char {
    //TODO: find a convenient way to handle this with Rust types
    pam_getenvlist(handle)
}
/* ----------------------- _pam_types.h ------------------------- */

/* ----------------------- pam_misc.h --------------------------- */
pub unsafe fn misc_paste_env(handle: *mut PamHandle,
                             user_env: *const *const c_char)
                             -> PamReturnCode {
    From::from(pam_misc_paste_env(handle, user_env))
}

pub unsafe fn misc_drop_env(env: *mut *mut c_char) -> *mut *mut c_char {
    pam_misc_drop_env(env)
}

pub unsafe fn misc_setenv(handle: *mut PamHandle,
                          name: *const c_char,
                          value: *const c_char,
                          readonly: c_int)
                          -> PamReturnCode {
    From::from(pam_misc_setenv(handle, name, value, readonly))
}
/* ----------------------- pam_misc.h --------------------------- */

/* ----------------------- pam_modules.h ------------------------ */
#[inline]
pub unsafe fn set_data(handle: *mut PamHandle,
                       module_data_name: *const c_char,
                       data: *mut c_void,
                       cleanup: Option<extern "C" fn(*mut PamHandle, *mut c_void, c_int)>)
                       -> PamReturnCode {
    From::from(pam_set_data(handle, module_data_name, data, cleanup))
}

//pub fn get_data(handle: *const PamHandle, module_data_name: *const c_char, data: *const *const c_void);
//
//pub fn get_user(pamh: *mut PamHandle, user: *const *const c_char, prompt: *const c_char);
/* ----------------------- pam_modules.h ------------------------ */
