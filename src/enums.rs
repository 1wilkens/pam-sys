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
