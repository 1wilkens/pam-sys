use libc::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy)]
pub struct PamHandle;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct PamMessage {
    pub msg_style:  c_int,
    pub msg:        *const c_char
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct PamResponse {
    pub resp:           *mut c_char,
    pub resp_retcode:   c_int
}

#[repr(C)]
#[derive(Copy)]
pub struct PamConversation {
    /* int (*conv)(int num_msg, const struct pam_message **msg,
		struct pam_response **resp, void *appdata_ptr); */
    pub conv:       Option<extern "C" fn (c_int, *mut *mut PamMessage, *mut *mut PamResponse, *mut c_void) -> c_int>,
    //pub conv:       *const c_void,
    pub data_ptr:   *mut c_void
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct PamXAuthData {
    pub namelen:    c_int,
    pub name:       *mut c_char,
    pub datalen:    c_int,
    pub data:       *mut c_char
}

#[derive(Debug, PartialEq)]
pub enum PamReturnCode {
    SUCCESS                 = 0,    /* Successful function return */
    OPEN_ERR                = 1,    /* dlopen() failure when dynamically
                                    loading a service module */
    SYMBOL_ERR              = 2,    /* Symbol not found */
    SERVICE_ERR             = 3,    /* Error in service module */
    SYSTEM_ERR              = 4,    /* System error */
    BUF_ERR                 = 5,    /* Memory buffer error */
    PERM_DENIED             = 6,    /* Permission denied */
    AUTH_ERR                = 7,    /* Authentication failure */
    CRED_INSUFFICIENT       = 8,    /* Can not access authentication data
                                    due to insufficient credentials */
    AUTHINFO_UNAVAIL        = 9,    /* Underlying authentication service
                                    can not retrieve authentication
                                    information  */
    USER_UNKNOWN            = 10,   /* User not known to the underlying
                                    authenticaiton module */
    MAXTRIES                = 11,   /* An authentication service has
                                    maintained a retry count which has
                                    been reached.  No further retries
                                    should be attempted */
    NEW_AUTHTOK_REQD        = 12,   /* New authentication token required.
                                    This is normally returned if the
                                    machine security policies require
                                    that the password should be changed
                                    beccause the password is NULL or it
                                    has aged */
    ACCT_EXPIRED            = 13,   /* User account has expired */
    SESSION_ERR             = 14,   /* Can not make/remove an entry for
                                    the specified session */
    CRED_UNAVAIL            = 15,   /* Underlying authentication service
                                    can not retrieve user credentials
                                    unavailable */
    CRED_EXPIRED            = 16,   /* User credentials expired */
    CRED_ERR                = 17,   /* Failure setting user credentials */
    NO_MODULE_DATA          = 18,   /* No module specific data is present */
    CONV_ERR                = 19,   /* Conversation error */
    AUTHTOK_ERR             = 20,   /* Authentication token manipulation error */
    AUTHTOK_RECOVERY_ERR    = 21,   /* Authentication information
                                    cannot be recovered */
    AUTHTOK_LOCK_BUSY       = 22,   /* Authentication token lock busy */
    AUTHTOK_DISABLE_AGING   = 23,   /* Authentication token aging disabled */
    TRY_AGAIN               = 24,   /* Preliminary check by password service */
    IGNORE                  = 25,   /* Ignore underlying account module
                                    regardless of whether the control
                                    flag is required, optional, or sufficient */
    ABORT                   = 26,   /* Critical error (?module fail now request) */
    AUTHTOK_EXPIRED         = 27,   /* user's authentication token has expired */
    MODULE_UNKNOWN          = 28,   /* module is not known */
    BAD_ITEM                = 29,   /* Bad item passed to pam_*_item() */
    CONV_AGAIN              = 30,   /* conversation function is event driven
                                    and data is not available yet */
    INCOMPLETE              = 31    /* please call this function again to
                                    complete authentication stack. Before
                                    calling again, verify that conversation
                                    is completed */
}

impl PamReturnCode {
    pub fn from_i32(status: i32) -> PamReturnCode {
        match status {
            0   => PamReturnCode::SUCCESS               ,
            1   => PamReturnCode::OPEN_ERR              ,
            2   => PamReturnCode::SYMBOL_ERR            ,
            3   => PamReturnCode::SERVICE_ERR           ,
            4   => PamReturnCode::SYSTEM_ERR            ,
            5   => PamReturnCode::BUF_ERR               ,
            6   => PamReturnCode::PERM_DENIED           ,
            7   => PamReturnCode::AUTH_ERR              ,
            8   => PamReturnCode::CRED_INSUFFICIENT     ,
            9   => PamReturnCode::AUTHINFO_UNAVAIL      ,
            10  => PamReturnCode::USER_UNKNOWN          ,
            11  => PamReturnCode::MAXTRIES              ,
            12  => PamReturnCode::NEW_AUTHTOK_REQD      ,
            13  => PamReturnCode::ACCT_EXPIRED          ,
            14  => PamReturnCode::SESSION_ERR           ,
            15  => PamReturnCode::CRED_UNAVAIL          ,
            16  => PamReturnCode::CRED_EXPIRED          ,
            17  => PamReturnCode::CRED_ERR              ,
            18  => PamReturnCode::NO_MODULE_DATA        ,
            19  => PamReturnCode::CONV_ERR              ,
            20  => PamReturnCode::AUTHTOK_ERR           ,
            21  => PamReturnCode::AUTHTOK_RECOVERY_ERR  ,
            22  => PamReturnCode::AUTHTOK_LOCK_BUSY     ,
            23  => PamReturnCode::AUTHTOK_DISABLE_AGING ,
            24  => PamReturnCode::TRY_AGAIN             ,
            25  => PamReturnCode::IGNORE                ,
            26  => PamReturnCode::ABORT                 ,
            27  => PamReturnCode::AUTHTOK_EXPIRED       ,
            28  => PamReturnCode::MODULE_UNKNOWN        ,
            29  => PamReturnCode::BAD_ITEM              ,
            30  => PamReturnCode::CONV_AGAIN            ,
            31  => PamReturnCode::INCOMPLETE            ,
            _   => PamReturnCode::SYSTEM_ERR
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PamFlag {
    SILENT                  = 0x8000,

    /* Note: these flags are used by pam_authenticate{,_secondary}() */

    /* The authentication service should return AUTH_ERROR if the
     * user has a null authentication token */
    DISALLOW_NULL_AUTHTOK   = 0x0001,

    /* Note: these flags are used for pam_setcred() */

    /* Set user credentials for an authentication service */
    ESTABLISH_CRED          = 0x0002,

    /* Delete user credentials associated with an authentication service */
    DELETE_CRED             = 0x0004,

    /* Reinitialize user credentials */
    REINITIALIZE_CRED       = 0x0008,

    /* Extend lifetime of user credentials */
    REFRESH_CRED            = 0x0010,

    /* Note: these flags are used by pam_chauthtok */

    /* The password service should only update those passwords that have
     * aged.  If this flag is not passed, the password service should
     * update all passwords. */
    CHANGE_EXPIRED_AUTHTOK  = 0x0020,

    NONE                    = 0
}

#[derive(Debug, PartialEq)]
pub enum PamItemType {
    /* These defines are used by pam_set_item() and pam_get_item().
    Please check the spec which are allowed for use by applications
    and which are only allowed for use by modules. */

    SERVICE         = 1,    /* The service name */
    USER            = 2,    /* The user name */
    TTY             = 3,    /* The tty name */
    RHOST           = 4,    /* The remote host name */
    CONV            = 5,    /* The pam_conv structure */
    AUTHTOK         = 6,    /* The authentication token (password) */
    OLDAUTHTOK      = 7,    /* The old authentication token */
    RUSER           = 8,    /* The remote user name */
    USER_PROMPT     = 9,    /* the prompt for getting a username
                            Linux-PAM extensions */
    FAIL_DELAY      = 10,   /* app supplied function to override failure
                            delays */
    XDISPLAY        = 11,   /* X display name */
    XAUTHDATA       = 12,   /* X server authentication data */
    AUTHTOK_TYPE    = 13    /* The type for pam_get_authtok */
}

#[derive(Debug, PartialEq)]
pub enum PamMessageStyle {
    PROMPT_ECHO_OFF = 1,
    PROMPT_ECHO_ON  = 2,
    ERROR_MSG       = 3,
    TEXT_INFO       = 4
}

impl PamMessageStyle {
    pub fn from_i32(style: i32) -> PamMessageStyle {
        match style {
            1 => PamMessageStyle::PROMPT_ECHO_OFF,
            2 => PamMessageStyle::PROMPT_ECHO_ON,
            3 => PamMessageStyle::ERROR_MSG,
            4 => PamMessageStyle::TEXT_INFO,
            _ => PamMessageStyle::ERROR_MSG
        }
    }
}
