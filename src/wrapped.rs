// Copyright (C) 2015-2017 Florian Wilkens
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
use std::ffi::{CStr, CString};
use std::ptr::null;

use types::*;
use raw::*;

/* ------------------------ pam_appl.h -------------------------- */
#[inline]
pub fn start(service: &str,
             user: Option<&str>,
             conversation: &PamConversation,
             handle: *mut *mut PamHandle)
             -> PamReturnCode {
    if let Ok(service) = CString::new(service) {
        if let Some(usr) = user {
            if let Ok(user) = CString::new(usr) {
                unsafe {
                    From::from(pam_start(service.as_ptr(),
                                         user.as_ptr(),
                                         conversation,
                                         handle as *mut *const PamHandle))
                }
            } else {
                PamReturnCode::BUF_ERR
            }
        } else {
            unsafe {
                From::from(pam_start(service.as_ptr(),
                                     null(),
                                     conversation,
                                     handle as *mut *const PamHandle))
            }
        }
    } else {
        PamReturnCode::SERVICE_ERR
    }
}

#[inline]
pub fn end(handle: &mut PamHandle, status: PamReturnCode) -> PamReturnCode {
    From::from(unsafe { pam_end(handle, status as c_int) })
}

#[inline]
pub fn authenticate(handle: &mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(unsafe { pam_authenticate(handle, flags as c_int) })
}

#[inline]
pub fn setcred(handle: &mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(unsafe { pam_setcred(handle, flags as c_int) })
}

#[inline]
pub fn acct_mgmt(handle: &mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(unsafe { pam_acct_mgmt(handle, flags as c_int) })
}

#[inline]
pub fn open_session(handle: &mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(unsafe { pam_open_session(handle, flags as c_int) })
}

#[inline]
pub fn close_session(handle: &mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(unsafe { pam_close_session(handle, flags as c_int) })
}

#[inline]
pub fn chauthtok(handle: &mut PamHandle, flags: PamFlag) -> PamReturnCode {
    From::from(unsafe { pam_chauthtok(handle, flags as c_int) })
}
/* ------------------------ pam_appl.h -------------------------- */

/* ----------------------- _pam_types.h ------------------------- */
#[inline]
pub fn set_item(handle: &mut PamHandle, item_type: PamItemType, item: &c_void) -> PamReturnCode {
    From::from(unsafe { pam_set_item(handle, item_type as c_int, item) })
}

#[inline]
pub fn get_item(handle: &PamHandle,
                item_type: PamItemType,
                item: &mut *const c_void)
                -> PamReturnCode {
    From::from(unsafe { pam_get_item(handle, item_type as c_int, item) })
}

#[inline]
pub fn strerror(handle: &mut PamHandle, errnum: PamReturnCode) -> Option<&str> {
    unsafe { CStr::from_ptr(pam_strerror(handle, errnum as c_int)) }.to_str().ok()
}

#[inline]
pub fn putenv(handle: &mut PamHandle, name_value: &str) -> PamReturnCode {
    if let Ok(name_value) = CString::new(name_value) {
        From::from(unsafe { pam_putenv(handle, name_value.as_ptr()) })
    } else {
        // Not sure whether this is the correct return value
        PamReturnCode::BUF_ERR
    }
}

#[inline]
pub fn getenv<'a>(handle: &'a mut PamHandle, name: &str) -> Option<&'a str> {
    use std::ptr;
    if let Ok(name) = CString::new(name) {
        let env = unsafe{pam_getenv(handle, name.as_ptr())};
        if env != ptr::null(){
            unsafe { CStr::from_ptr(env) }.to_str().ok()
        }
        else{
            None
        }
    } else {
        None
    }
}

#[inline]
pub fn getenvlist(handle: &mut PamHandle) -> *const *const c_char {
    //TODO: find a convenient way to handle this with Rust types
    unsafe { pam_getenvlist(handle) }
}
/* ----------------------- _pam_types.h ------------------------- */

/* ----------------------- pam_misc.h --------------------------- */
#[inline]
#[cfg(target_os = "linux")]
pub fn misc_paste_env(handle: &mut PamHandle, user_env: &[&str]) -> PamReturnCode {
    // Taken from: https://github.com/rust-lang/rust/issues/9564#issuecomment-95354558
    let user_env: Vec<_> = user_env.iter()
        .map(|&env| CString::new(env).unwrap())
        .collect();
    let env_ptrs: Vec<_> = user_env.iter()
        .map(|env| env.as_ptr())
        .chain(Some(null()))
        .collect();
    From::from(unsafe { pam_misc_paste_env(handle, env_ptrs.as_ptr()) })
}

#[inline]
#[cfg(target_os = "linux")]
pub fn misc_drop_env(env: &mut *mut c_char) -> PamReturnCode {
    From::from(unsafe { pam_misc_drop_env(env) })
}

#[inline]
#[cfg(target_os = "linux")]
pub fn misc_setenv(handle: &mut PamHandle,
                   name: &str,
                   value: &str,
                   readonly: bool)
                   -> PamReturnCode {
    if let (Ok(name), Ok(value)) = (CString::new(name), CString::new(value)) {
        From::from(unsafe {
            pam_misc_setenv(handle,
                            name.as_ptr(),
                            value.as_ptr(),
                            if readonly { 0 } else { 1 })
        })
    } else {
        PamReturnCode::BUF_ERR
    }
}
/* ----------------------- pam_misc.h --------------------------- */

/* ----------------------- pam_modules.h ------------------------ */
#[inline]
pub fn set_data(handle: &mut PamHandle,
                module_data_name: &str,
                data: &mut c_void,
                cleanup: Option<extern "C" fn(*mut PamHandle, *mut c_void, c_int)>)
                -> PamReturnCode {
    if let Ok(module_data_name) = CString::new(module_data_name) {
        From::from(unsafe { pam_set_data(handle, module_data_name.as_ptr(), data, cleanup) })

    } else {
        PamReturnCode::BUF_ERR
    }
}

//pub fn get_data(handle: *const PamHandle, module_data_name: *const c_char, data: *const *const c_void);

pub fn get_user(handle: &PamHandle,
                user: &mut *const c_char,
                prompt: *const c_char)
                -> PamReturnCode {
                
    From::from(
        unsafe {
            pam_get_user(
                handle,
                user,
                prompt
            )
        }
    )
}

/* ----------------------- pam_modules.h ------------------------ */
