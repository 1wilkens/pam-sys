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
    /* ------------------------ pam_appl.h -------------------------- */
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
    /* ------------------------ pam_appl.h -------------------------- */

    /* ----------------------- _pam_types.h ------------------------- */
    /* ---------- Common Linux-PAM application/module PI ------------ */
    pub fn pam_set_item(pamh: *mut PamHandle, item_type: c_int, item: *const c_void) -> c_int;

    pub fn pam_get_item(pamh: *const PamHandle, item_type: c_int, item: *const *const c_void) -> c_int;

    pub fn pam_strerror(pamh: *mut PamHandle, errnum: c_int) -> *const c_char;

    pub fn pam_putenv(pamh: *mut PamHandle, name_value: *const c_char) -> c_int;

    pub fn pam_getenv(pamh: *mut PamHandle, name: *const c_char) -> *const c_char;

    pub fn pam_getenvlist(pamh: *mut PamHandle) -> *const *const c_char;
    /* ----------------------- _pam_types.h ------------------------- */

    /* ----------------------- pam_modules.h ------------------------ */
    /* -------------------- The Linux-PAM Module PI ----------------- */

    /*pub fn pam_set_data(pamh: *mut PamHandle, module_data_name: *const c_char,
        data: *mut c_void, cleanup: extern fn()  void (*cleanup)(pamh: *mut PamHandle, void *data,
    			     int error_status));*/

    pub fn pam_get_data(pamh: *const PamHandle, module_data_name: *const c_char, data: *const *const c_void);

    pub fn pam_get_user(pamh: *mut PamHandle, user: *const *const c_char, prompt: *const c_char);
    /* ----------------------- pam_modules.h ------------------------ */
}
