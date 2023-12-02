/* automatically generated by rust-bindgen 0.69.1 */

use libc::passwd;

pub const OPENPAM_VERSION: u32 = 20230627;
pub const OPENPAM_RELEASE: &[u8; 8] = b"Ximenia\0";
pub const PAM_SOEXT: &[u8; 4] = b".so\0";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pam_message {
    pub msg_style: libc::c_int,
    pub msg: *mut libc::c_char,
}
#[test]
fn bindgen_test_layout_pam_message() {
    const UNINIT: ::std::mem::MaybeUninit<pam_message> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pam_message>(),
        16usize,
        concat!("Size of: ", stringify!(pam_message))
    );
    assert_eq!(
        ::std::mem::align_of::<pam_message>(),
        8usize,
        concat!("Alignment of ", stringify!(pam_message))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).msg_style) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pam_message),
            "::",
            stringify!(msg_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).msg) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(pam_message),
            "::",
            stringify!(msg)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pam_response {
    pub resp: *mut libc::c_char,
    pub resp_retcode: libc::c_int,
}
#[test]
fn bindgen_test_layout_pam_response() {
    const UNINIT: ::std::mem::MaybeUninit<pam_response> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pam_response>(),
        16usize,
        concat!("Size of: ", stringify!(pam_response))
    );
    assert_eq!(
        ::std::mem::align_of::<pam_response>(),
        8usize,
        concat!("Alignment of ", stringify!(pam_response))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resp) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pam_response),
            "::",
            stringify!(resp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resp_retcode) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(pam_response),
            "::",
            stringify!(resp_retcode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pam_conv {
    pub conv: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: libc::c_int,
            arg2: *mut *const pam_message,
            arg3: *mut *mut pam_response,
            arg4: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub appdata_ptr: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout_pam_conv() {
    const UNINIT: ::std::mem::MaybeUninit<pam_conv> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pam_conv>(),
        16usize,
        concat!("Size of: ", stringify!(pam_conv))
    );
    assert_eq!(
        ::std::mem::align_of::<pam_conv>(),
        8usize,
        concat!("Alignment of ", stringify!(pam_conv))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).conv) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pam_conv),
            "::",
            stringify!(conv)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).appdata_ptr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(pam_conv),
            "::",
            stringify!(appdata_ptr)
        )
    );
}
pub type pam_handle_t = u8;
pub const PAM_SUCCESS: _bindgen_ty_1 = 0;
pub const PAM_OPEN_ERR: _bindgen_ty_1 = 1;
pub const PAM_SYMBOL_ERR: _bindgen_ty_1 = 2;
pub const PAM_SERVICE_ERR: _bindgen_ty_1 = 3;
pub const PAM_SYSTEM_ERR: _bindgen_ty_1 = 4;
pub const PAM_BUF_ERR: _bindgen_ty_1 = 5;
pub const PAM_CONV_ERR: _bindgen_ty_1 = 6;
pub const PAM_PERM_DENIED: _bindgen_ty_1 = 7;
pub const PAM_MAXTRIES: _bindgen_ty_1 = 8;
pub const PAM_AUTH_ERR: _bindgen_ty_1 = 9;
pub const PAM_NEW_AUTHTOK_REQD: _bindgen_ty_1 = 10;
pub const PAM_CRED_INSUFFICIENT: _bindgen_ty_1 = 11;
pub const PAM_AUTHINFO_UNAVAIL: _bindgen_ty_1 = 12;
pub const PAM_USER_UNKNOWN: _bindgen_ty_1 = 13;
pub const PAM_CRED_UNAVAIL: _bindgen_ty_1 = 14;
pub const PAM_CRED_EXPIRED: _bindgen_ty_1 = 15;
pub const PAM_CRED_ERR: _bindgen_ty_1 = 16;
pub const PAM_ACCT_EXPIRED: _bindgen_ty_1 = 17;
pub const PAM_AUTHTOK_EXPIRED: _bindgen_ty_1 = 18;
pub const PAM_SESSION_ERR: _bindgen_ty_1 = 19;
pub const PAM_AUTHTOK_ERR: _bindgen_ty_1 = 20;
pub const PAM_AUTHTOK_RECOVERY_ERR: _bindgen_ty_1 = 21;
pub const PAM_AUTHTOK_LOCK_BUSY: _bindgen_ty_1 = 22;
pub const PAM_AUTHTOK_DISABLE_AGING: _bindgen_ty_1 = 23;
pub const PAM_NO_MODULE_DATA: _bindgen_ty_1 = 24;
pub const PAM_IGNORE: _bindgen_ty_1 = 25;
pub const PAM_ABORT: _bindgen_ty_1 = 26;
pub const PAM_TRY_AGAIN: _bindgen_ty_1 = 27;
pub const PAM_MODULE_UNKNOWN: _bindgen_ty_1 = 28;
pub const PAM_DOMAIN_UNKNOWN: _bindgen_ty_1 = 29;
pub const PAM_BAD_HANDLE: _bindgen_ty_1 = 30;
pub const PAM_BAD_ITEM: _bindgen_ty_1 = 31;
pub const PAM_BAD_FEATURE: _bindgen_ty_1 = 32;
pub const PAM_BAD_CONSTANT: _bindgen_ty_1 = 33;
pub const PAM_NUM_ERRORS: _bindgen_ty_1 = 34;
pub type _bindgen_ty_1 = libc::c_uint;
pub const PAM_PROMPT_ECHO_OFF: _bindgen_ty_2 = 1;
pub const PAM_PROMPT_ECHO_ON: _bindgen_ty_2 = 2;
pub const PAM_ERROR_MSG: _bindgen_ty_2 = 3;
pub const PAM_TEXT_INFO: _bindgen_ty_2 = 4;
pub const PAM_MAX_NUM_MSG: _bindgen_ty_2 = 32;
pub const PAM_MAX_MSG_SIZE: _bindgen_ty_2 = 512;
pub const PAM_MAX_RESP_SIZE: _bindgen_ty_2 = 512;
pub type _bindgen_ty_2 = libc::c_uint;
pub const PAM_SILENT: _bindgen_ty_3 = -2147483648;
pub const PAM_DISALLOW_NULL_AUTHTOK: _bindgen_ty_3 = 1;
pub const PAM_ESTABLISH_CRED: _bindgen_ty_3 = 1;
pub const PAM_DELETE_CRED: _bindgen_ty_3 = 2;
pub const PAM_REINITIALIZE_CRED: _bindgen_ty_3 = 4;
pub const PAM_REFRESH_CRED: _bindgen_ty_3 = 8;
pub const PAM_PRELIM_CHECK: _bindgen_ty_3 = 1;
pub const PAM_UPDATE_AUTHTOK: _bindgen_ty_3 = 2;
pub const PAM_CHANGE_EXPIRED_AUTHTOK: _bindgen_ty_3 = 4;
pub type _bindgen_ty_3 = libc::c_int;
pub const PAM_SERVICE: _bindgen_ty_4 = 1;
pub const PAM_USER: _bindgen_ty_4 = 2;
pub const PAM_TTY: _bindgen_ty_4 = 3;
pub const PAM_RHOST: _bindgen_ty_4 = 4;
pub const PAM_CONV: _bindgen_ty_4 = 5;
pub const PAM_AUTHTOK: _bindgen_ty_4 = 6;
pub const PAM_OLDAUTHTOK: _bindgen_ty_4 = 7;
pub const PAM_RUSER: _bindgen_ty_4 = 8;
pub const PAM_USER_PROMPT: _bindgen_ty_4 = 9;
pub const PAM_REPOSITORY: _bindgen_ty_4 = 10;
pub const PAM_AUTHTOK_PROMPT: _bindgen_ty_4 = 11;
pub const PAM_OLDAUTHTOK_PROMPT: _bindgen_ty_4 = 12;
pub const PAM_HOST: _bindgen_ty_4 = 13;
pub const PAM_NUM_ITEMS: _bindgen_ty_4 = 14;
pub type _bindgen_ty_4 = libc::c_uint;
extern "C" {
    pub fn pam_acct_mgmt(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_authenticate(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_chauthtok(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_close_session(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_end(_pamh: *mut pam_handle_t, _status: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_get_data(
        _pamh: *const pam_handle_t,
        _module_data_name: *const libc::c_char,
        _data: *mut *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_get_item(
        _pamh: *const pam_handle_t,
        _item_type: libc::c_int,
        _item: *mut *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_get_user(
        _pamh: *mut pam_handle_t,
        _user: *mut *const libc::c_char,
        _prompt: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_getenv(_pamh: *mut pam_handle_t, _name: *const libc::c_char) -> *const libc::c_char;
}
extern "C" {
    pub fn pam_getenvlist(_pamh: *mut pam_handle_t) -> *mut *mut libc::c_char;
}
extern "C" {
    pub fn pam_open_session(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_putenv(_pamh: *mut pam_handle_t, _namevalue: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn pam_set_data(
        _pamh: *mut pam_handle_t,
        _module_data_name: *const libc::c_char,
        _data: *mut libc::c_void,
        _cleanup: ::std::option::Option<
            unsafe extern "C" fn(
                _pamh: *mut pam_handle_t,
                _data: *mut libc::c_void,
                _pam_end_status: libc::c_int,
            ),
        >,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_set_item(
        _pamh: *mut pam_handle_t,
        _item_type: libc::c_int,
        _item: *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_setcred(_pamh: *mut pam_handle_t, _flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pam_start(
        _service: *const libc::c_char,
        _user: *const libc::c_char,
        _pam_conv: *const pam_conv,
        _pamh: *mut *mut pam_handle_t,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_strerror(
        _pamh: *const pam_handle_t,
        _error_number: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn openpam_borrow_cred(_pamh: *mut pam_handle_t, _pwd: *const passwd) -> libc::c_int;
}
extern "C" {
    pub fn openpam_subst(
        _pamh: *const pam_handle_t,
        _buf: *mut libc::c_char,
        _bufsize: *mut usize,
        _template: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn openpam_free_data(
        _pamh: *mut pam_handle_t,
        _data: *mut libc::c_void,
        _status: libc::c_int,
    );
}
extern "C" {
    pub fn openpam_free_envlist(_envlist: *mut *mut libc::c_char);
}
extern "C" {
    pub fn openpam_get_option(
        _pamh: *mut pam_handle_t,
        _option: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn openpam_restore_cred(_pamh: *mut pam_handle_t) -> libc::c_int;
}
extern "C" {
    pub fn openpam_set_option(
        _pamh: *mut pam_handle_t,
        _option: *const libc::c_char,
        _value: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_error(_pamh: *const pam_handle_t, _fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn pam_get_authtok(
        _pamh: *mut pam_handle_t,
        _item: libc::c_int,
        _authtok: *mut *const libc::c_char,
        _prompt: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_info(_pamh: *const pam_handle_t, _fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    pub fn pam_setenv(
        _pamh: *mut pam_handle_t,
        _name: *const libc::c_char,
        _value: *const libc::c_char,
        _overwrite: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn openpam_straddch(
        _str: *mut *mut libc::c_char,
        _sizep: *mut usize,
        _lenp: *mut usize,
        ch: libc::c_int,
    ) -> libc::c_int;
}
pub const OPENPAM_RESTRICT_SERVICE_NAME: _bindgen_ty_5 = 0;
pub const OPENPAM_VERIFY_POLICY_FILE: _bindgen_ty_5 = 1;
pub const OPENPAM_RESTRICT_MODULE_NAME: _bindgen_ty_5 = 2;
pub const OPENPAM_VERIFY_MODULE_FILE: _bindgen_ty_5 = 3;
pub const OPENPAM_FALLBACK_TO_OTHER: _bindgen_ty_5 = 4;
pub const OPENPAM_NUM_FEATURES: _bindgen_ty_5 = 5;
pub type _bindgen_ty_5 = libc::c_uint;
extern "C" {
    pub fn openpam_set_feature(_feature: libc::c_int, _onoff: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn openpam_get_feature(_feature: libc::c_int, _onoff: *mut libc::c_int) -> libc::c_int;
}
pub const PAM_LOG_LIBDEBUG: _bindgen_ty_6 = -1;
pub const PAM_LOG_DEBUG: _bindgen_ty_6 = 0;
pub const PAM_LOG_VERBOSE: _bindgen_ty_6 = 1;
pub const PAM_LOG_NOTICE: _bindgen_ty_6 = 2;
pub const PAM_LOG_ERROR: _bindgen_ty_6 = 3;
pub type _bindgen_ty_6 = libc::c_int;
extern "C" {
    pub fn openpam_ttyconv(
        _n: libc::c_int,
        _msg: *mut *const pam_message,
        _resp: *mut *mut pam_response,
        _data: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn openpam_nullconv(
        _n: libc::c_int,
        _msg: *mut *const pam_message,
        _resp: *mut *mut pam_response,
        _data: *mut libc::c_void,
    ) -> libc::c_int;
}
pub const PAM_SM_AUTHENTICATE: _bindgen_ty_7 = 0;
pub const PAM_SM_SETCRED: _bindgen_ty_7 = 1;
pub const PAM_SM_ACCT_MGMT: _bindgen_ty_7 = 2;
pub const PAM_SM_OPEN_SESSION: _bindgen_ty_7 = 3;
pub const PAM_SM_CLOSE_SESSION: _bindgen_ty_7 = 4;
pub const PAM_SM_CHAUTHTOK: _bindgen_ty_7 = 5;
pub const PAM_NUM_PRIMITIVES: _bindgen_ty_7 = 6;
pub type _bindgen_ty_7 = libc::c_uint;
extern "C" {
    pub fn pam_sm_acct_mgmt(
        _pamh: *mut pam_handle_t,
        _flags: libc::c_int,
        _argc: libc::c_int,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_sm_authenticate(
        _pamh: *mut pam_handle_t,
        _flags: libc::c_int,
        _argc: libc::c_int,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_sm_chauthtok(
        _pamh: *mut pam_handle_t,
        _flags: libc::c_int,
        _argc: libc::c_int,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_sm_close_session(
        _pamh: *mut pam_handle_t,
        _flags: libc::c_int,
        _args: libc::c_int,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_sm_open_session(
        _pamh: *mut pam_handle_t,
        _flags: libc::c_int,
        _argc: libc::c_int,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pam_sm_setcred(
        _pamh: *mut pam_handle_t,
        _flags: libc::c_int,
        _argc: libc::c_int,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int;
}
