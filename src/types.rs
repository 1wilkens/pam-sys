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

//! Types defined by Linux-PAM
//!
//! This modules contains struct and enum definitions used by `pam-sys`.

use libc::{c_char, c_int, c_void};

use std::fmt::{Display, Error, Formatter};

/// Type alias for the PAM "conversation function" used as part of the `PamConversation` struct
pub type ConvClosure = (extern "C" fn(c_int,
                                  *mut *mut PamMessage,
                                  *mut *mut PamResponse,
                                  *mut c_void)
                                  -> c_int);

/// Opaque struct internal to Linux-PAM
///
/// From `_pam_types.h`:
///
/// "This is a blind structure. Users aren't allowed to see
/// inside a `pam_handle_t`, so we don't define struct `pam_handle` here.
/// This is defined in a file private to the PAM library.
/// (i.e., it's private to PAM service modules, too!)"
pub enum PamHandle {}

/// Message struct to transfer authentication data to the user
///
/// From `_pam_types.h`:
///
/// "Used to pass prompting text, error messages, or other informatory text to the user.
/// This structure is allocated and freed by the PAM library (or loaded module)."
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PamMessage {
    pub msg_style: c_int,
    pub msg: *const c_char,
}


/// Response struct to transfer the user's response back to Linux-PAM
///
/// From `_pam_types.h`:
///
/// "Used to return the user's response to the PAM library.
/// This structure is allocated by the application program,
/// and free()'d by the Linux-PAM library (or calling module)."
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PamResponse {
    pub resp: *mut c_char,
    /// currently un-used, zero expected
    pub resp_retcode: c_int,
}

/// Conversation structure containing the `converse` function and authentication data
///
/// From `_pam_types.h`:
///
/// "The actual conversation structure itself"
#[repr(C)]
pub struct PamConversation {
    /* int (*conv)(int num_msg, const struct pam_message **msg,
        struct pam_response **resp, void *appdata_ptr); */
    pub conv: Option<ConvClosure>,
    pub data_ptr: *mut c_void,
}

/// Special struct for the `PAM_XAUTHDATA` pam item
///
/// From `_pam_types.h`:
///
/// "Used by the `PAM_XAUTHDATA` pam item. Contains X authentication
/// data used by modules to connect to the user's X display.
/// Note: this structure is intentionally compatible with `xcb_auth_info_t`."
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PamXAuthData {
    pub namelen: c_int,
    pub name: *mut c_char,
    pub datalen: c_int,
    pub data: *mut c_char,
}

/// The Linux-PAM return values
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PamReturnCode {
    /// Successful function return
    SUCCESS = 0,

    /// dlopen() failure when dynamically loading a service module
    OPEN_ERR = 1,

    /// Symbol not found
    SYMBOL_ERR = 2,

    /// Error in service module
    SERVICE_ERR = 3,

    /// System error
    SYSTEM_ERR = 4,

    /// Memory buffer error
    BUF_ERR = 5,

    /// Permission denied
    PERM_DENIED = 6,

    /// Authentication failure
    AUTH_ERR = 7,

    /// Can not access authentication data due to insufficient credentials
    CRED_INSUFFICIENT = 8,

    /// Underlying authentication service can not retrieve authentication information
    AUTHINFO_UNAVAIL = 9,

    /// User not known to the underlying authentication module
    USER_UNKNOWN = 10,

    /// An authentication service has maintained a retry count which has been reached.
    /// No further retries should be attempted
    MAXTRIES = 11,

    /// New authentication token required.
    /// This is normally returned if the machine security policies require
    /// that the password should be changed beccause the password is NULL or it has aged
    NEW_AUTHTOK_REQD = 12,

    /// User account has expired
    ACCT_EXPIRED = 13,

    /// Can not make/remove an entry for the specified session
    SESSION_ERR = 14,

    /// Underlying authentication service can not retrieve user credentials unavailable
    CRED_UNAVAIL = 15,

    /// User credentials expired
    CRED_EXPIRED = 16,

    /// Failure setting user credentials
    CRED_ERR = 17,

    /// No module specific data is present
    NO_MODULE_DATA = 18,

    /// Conversation error
    CONV_ERR = 19,

    /// Authentication token manipulation error
    AUTHTOK_ERR = 20,

    /// Authentication information cannot be recovered
    AUTHTOK_RECOVERY_ERR = 21,

    /// Authentication token lock busy
    AUTHTOK_LOCK_BUSY = 22,

    /// Authentication token aging disabled
    AUTHTOK_DISABLE_AGING = 23,

    /// Preliminary check by password service
    TRY_AGAIN = 24,

    /// Ignore underlying account module regardless of whether
    /// the control flag is required, optional, or sufficient
    IGNORE = 25,

    /// Critical error (?module fail now request)
    AUTHTOK_EXPIRED = 27,

    /// user's authentication token has expired
    ABORT = 26,

    /// module is not known
    MODULE_UNKNOWN = 28,

    /// Bad item passed to pam_*_item()
    BAD_ITEM = 29,

    /// conversation function is event driven and data is not available yet
    CONV_AGAIN = 30,

    /// please call this function again to complete authentication stack.
    /// Before calling again, verify that conversation is completed
    INCOMPLETE = 31,
}

impl Display for PamReturnCode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{:?} ({})", self, *self as i32))
    }
}

impl From<i32> for PamReturnCode {
    fn from(status: i32) -> PamReturnCode {
        match status {
            0 => PamReturnCode::SUCCESS,
            1 => PamReturnCode::OPEN_ERR,
            2 => PamReturnCode::SYMBOL_ERR,
            3 => PamReturnCode::SERVICE_ERR,
            4 => PamReturnCode::SYSTEM_ERR,
            5 => PamReturnCode::BUF_ERR,
            6 => PamReturnCode::PERM_DENIED,
            7 => PamReturnCode::AUTH_ERR,
            8 => PamReturnCode::CRED_INSUFFICIENT,
            9 => PamReturnCode::AUTHINFO_UNAVAIL,
            10 => PamReturnCode::USER_UNKNOWN,
            11 => PamReturnCode::MAXTRIES,
            12 => PamReturnCode::NEW_AUTHTOK_REQD,
            13 => PamReturnCode::ACCT_EXPIRED,
            14 => PamReturnCode::SESSION_ERR,
            15 => PamReturnCode::CRED_UNAVAIL,
            16 => PamReturnCode::CRED_EXPIRED,
            17 => PamReturnCode::CRED_ERR,
            18 => PamReturnCode::NO_MODULE_DATA,
            19 => PamReturnCode::CONV_ERR,
            20 => PamReturnCode::AUTHTOK_ERR,
            21 => PamReturnCode::AUTHTOK_RECOVERY_ERR,
            22 => PamReturnCode::AUTHTOK_LOCK_BUSY,
            23 => PamReturnCode::AUTHTOK_DISABLE_AGING,
            24 => PamReturnCode::TRY_AGAIN,
            25 => PamReturnCode::IGNORE,
            26 => PamReturnCode::ABORT,
            27 => PamReturnCode::AUTHTOK_EXPIRED,
            28 => PamReturnCode::MODULE_UNKNOWN,
            29 => PamReturnCode::BAD_ITEM,
            30 => PamReturnCode::CONV_AGAIN,
            31 => PamReturnCode::INCOMPLETE,
            _ => PamReturnCode::SYSTEM_ERR,
        }
    }
}

/// The Linux-PAM flags
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PamFlag {
    /// Authentication service should not generate any messages
    SILENT = 0x8000,

    /// The authentication service should return AUTH_ERROR
    /// if the user has a null authentication token
    /// (used by pam_authenticate{,_secondary}())
    DISALLOW_NULL_AUTHTOK = 0x0001,

    /// Set user credentials for an authentication service
    /// (used for pam_setcred())
    ESTABLISH_CRED = 0x0002,

    /// Delete user credentials associated with an authentication service
    /// (used for pam_setcred())
    DELETE_CRED = 0x0004,

    /// Reinitialize user credentials
    /// (used for pam_setcred())
    REINITIALIZE_CRED = 0x0008,

    /// Extend lifetime of user credentials
    /// (used for pam_setcred())
    REFRESH_CRED = 0x0010,

    /// The password service should only update those passwords that have aged.
    /// If this flag is not passed, the password service should update all passwords.
    /// (used by pam_chauthtok)
    CHANGE_EXPIRED_AUTHTOK = 0x0020,

    NONE = 0x0000,
}

impl Display for PamFlag {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{:?} ({})", self, *self as i32))
    }
}

/// The Linux-PAM item types
///
/// These defines are used by `pam_set_item()` `and pam_get_item()`.
/// Please check the spec which are allowed for use by applications
/// and which are only allowed for use by modules.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PamItemType {
    /// The service name
    SERVICE = 1,

    /// The user name
    USER = 2,

    /// The tty name
    TTY = 3,

    /// The remote host name
    RHOST = 4,

    /// The pam_conv structure
    CONV = 5,

    /// The authentication token (password)
    AUTHTOK = 6,

    /// The old authentication token
    OLDAUTHTOK = 7,

    /// The remote user name
    RUSER = 8,

    /// the prompt for getting a username Linux-PAM extensions
    USER_PROMPT = 9,

    /// app supplied function to override failure delays
    FAIL_DELAY = 10,

    /// X display name
    XDISPLAY = 11,

    /// X server authentication data
    XAUTHDATA = 12,

    /// The type for pam_get_authtok
    AUTHTOK_TYPE = 13,
}

impl Display for PamItemType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{:?} ({})", self, *self as i32))
    }
}

/// The Linux-PAM message styles
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PamMessageStyle {
    PROMPT_ECHO_OFF = 1,
    PROMPT_ECHO_ON = 2,
    ERROR_MSG = 3,
    TEXT_INFO = 4,
}

impl Display for PamMessageStyle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{:?} ({})", self, *self as i32))
    }
}

impl From<i32> for PamMessageStyle {
    fn from(style: i32) -> PamMessageStyle {
        match style {
            1 => PamMessageStyle::PROMPT_ECHO_OFF,
            2 => PamMessageStyle::PROMPT_ECHO_ON,
            3 => PamMessageStyle::ERROR_MSG,
            4 => PamMessageStyle::TEXT_INFO,
            _ => PamMessageStyle::ERROR_MSG,
        }
    }
}
