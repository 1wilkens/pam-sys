extern crate libc;

use libc::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy)]
pub struct PamHandle;

#[repr(C)]
#[derive(Copy)]
pub struct PamConv {
    pub conv: *const c_void,
    pub data_ptr: *mut c_void
}

extern "C" {
    /* --------------             pam_appl.h            ------------- */
    /* -------------- The Linux-PAM Framework layer API ------------- */

    pub fn pam_start(service_name: *const c_char, user: *const c_char,
        pam_conversation: *const PamConv, pamh: *mut *const PamHandle) -> c_int;

    pub fn pam_end(pamh: *mut PamHandle, pam_status: c_int) -> c_int;

    /* Authentication API's */
    pub fn pam_authenticate(pamh: *mut PamHandle, flags: c_int) -> c_int;

    pub fn pam_setcred(pamh: *mut PamHandle, flags: c_int) -> c_int;

    /* Account Management API's */
    pub fn pam_acct_mgmt(pamh: *mut PamHandle, flags: c_int) -> c_int;

    /* Session Management API's */
    pub fn pam_open_session(pamh: *mut PamHandle, flags: c_int) -> c_int;

    pub fn pam_close_session(pamh: *mut PamHandle, flags: c_int) -> c_int;

    /* Password Management API's */
    pub fn pam_chauthtok(pamh: *mut PamHandle, flags: c_int) -> c_int;
}
