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

//! Raw FFI bindings to Linux-PAM
//!
//! This modules contains the raw and unchangeg FFI bindings to libpam.so.
//! All C-types are mapped to their responding types from `libc` and functions
//! names are exactly as exported by `libpam.so`
//!
//! Note: If possible the wrapped versions of these functions should be preferred,
//! since they offer some additional typesafety through the use of the enums defined
//! in the [`types`](../types/index.html) module.

use libc::{c_char, c_int, c_void};

use types::*;

extern "C" {
    /* ------------------------ pam_appl.h -------------------------- */
    /* -------------- The Linux-PAM Framework layer API ------------- */
    pub fn pam_start(service_name: *const c_char,
                     user: *const c_char,
                     pam_conversation: *const PamConversation,
                     pamh: *mut *const PamHandle)
                     -> c_int;

    pub fn pam_end(pamh: *mut PamHandle, pam_status: c_int) -> c_int;

    /* Authentication APIs */
    pub fn pam_authenticate(pamh: *mut PamHandle, flags: c_int) -> c_int;

    pub fn pam_setcred(pamh: *mut PamHandle, flags: c_int) -> c_int;

    /* Account Management APIs */
    pub fn pam_acct_mgmt(pamh: *mut PamHandle, flags: c_int) -> c_int;

    /* Session Management APIs */
    pub fn pam_open_session(pamh: *mut PamHandle, flags: c_int) -> c_int;

    pub fn pam_close_session(pamh: *mut PamHandle, flags: c_int) -> c_int;

    /* Password Management APIs */
    pub fn pam_chauthtok(pamh: *mut PamHandle, flags: c_int) -> c_int;
    /* ------------------------ pam_appl.h -------------------------- */

    /* ----------------------- _pam_types.h ------------------------- */
    /* ---------- Common Linux-PAM application/module PI ------------ */
    pub fn pam_set_item(pamh: *mut PamHandle, item_type: c_int, item: *const c_void) -> c_int;

    pub fn pam_get_item(pamh: *const PamHandle,
                        item_type: c_int,
                        item: *mut *const c_void)
                        -> c_int;

    pub fn pam_strerror(pamh: *mut PamHandle, errnum: c_int) -> *const c_char;

    pub fn pam_putenv(pamh: *mut PamHandle, name_value: *const c_char) -> c_int;

    pub fn pam_getenv(pamh: *mut PamHandle, name: *const c_char) -> *const c_char;

    pub fn pam_getenvlist(pamh: *mut PamHandle) -> *const *const c_char;
    /* ----------------------- _pam_types.h ------------------------- */

    /* ----------------------- pam_misc.h --------------------------- */
    #[cfg(target_os = "linux")]
    pub fn pam_misc_paste_env(pamh: *mut PamHandle,
                              user_env: *const *const c_char)
                              -> c_int;

    #[cfg(target_os = "linux")]
    pub fn pam_misc_drop_env(env: *mut *mut c_char) -> c_int;

    #[cfg(target_os = "linux")]
    pub fn pam_misc_setenv(pamh: *mut PamHandle,
                           name: *const c_char,
                           value: *const c_char,
                           readonly: c_int)
                           -> c_int;
    /* ----------------------- pam_misc.h --------------------------- */

    /* ----------------------- pam_modules.h ------------------------ */
    /* -------------------- The Linux-PAM Module PI ----------------- */
    pub fn pam_set_data(pamh: *mut PamHandle,
                        module_data_name: *const c_char,
                        data: *mut c_void,
                        cleanup: Option<extern "C" fn(*mut PamHandle, *mut c_void, c_int)>)
                        -> c_int;

    pub fn pam_get_data(pamh: *const PamHandle,
                        module_data_name: *const c_char,
                        data: *mut *const c_void)
                        -> c_int;

    pub fn pam_get_user(pamh: *const PamHandle,
                        user: *mut *const c_char,
                        prompt: *const c_char)
                        -> c_int;
    /* ----------------------- pam_modules.h ------------------------ */
}
