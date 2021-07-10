extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system pam
    // shared library.
    println!("cargo:rustc-link-lib=pam");
    // pam_misc is only supported on Linux afaik
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=pam_misc");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .ctypes_prefix("libc")
        // Set macro constants to signed int, as all functions that accept
        // these constants use signed int as the parameter type
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // pam_handle is opaque, don't implement Copy
        .no_copy("pam_handle")
        // Blacklist varargs functions and related types for now
        // TODO: find a nice solution for this
        .blacklist_type("va_list")
        .blacklist_type("__builtin_va_list")
        .blacklist_type("__va_list_tag")
        .blacklist_type("__va_list")
        .blacklist_function("pam_v.*")
        // Blacklist types we use from libc
        .blacklist_type(".*gid_t")
        .blacklist_type(".*uid_t")
        .blacklist_type("group")
        .blacklist_type("passwd")
        .blacklist_type("spwd")
        .blacklist_item("PAM_SILENT")
        .blacklist_item("PAM_DISALLOW_NULL_AUTHTOK")
        .blacklist_item("PAM_ESTABLISH_CRED")
        .blacklist_item("PAM_DELETE_CRED")
        .blacklist_item("PAM_REINITIALIZE_CRED")
        .blacklist_item("PAM_REFRESH_CRED")
        .blacklist_item("PAM_PRELIM_CHECK")
        .blacklist_item("PAM_UPDATE_AUTHTOK")
        .blacklist_item("PAM_CHANGE_EXPIRED_AUTHTOK")
        // Whitelist all PAM constants
        .whitelist_var("PAM_SUCCESS")
        .whitelist_var("PAM_OPEN_ERR")
        .whitelist_var("PAM_SYMBOL_ERR")
        .whitelist_var("PAM_SERVICE_ERR")
        .whitelist_var("PAM_SYSTEM_ERR")
        .whitelist_var("PAM_BUF_ERR")
        .whitelist_var("PAM_CONV_ERR")
        .whitelist_var("PAM_PERM_DENIED")
        .whitelist_var("PAM_MAXTRIES")
        .whitelist_var("PAM_AUTH_ERR")
        .whitelist_var("PAM_NEW_AUTHTOK_REQD")
        .whitelist_var("PAM_CRED_INSUFFICIENT")
        .whitelist_var("PAM_AUTHINFO_UNAVAIL")
        .whitelist_var("PAM_USER_UNKNOWN")
        .whitelist_var("PAM_CRED_UNAVAIL")
        .whitelist_var("PAM_CRED_EXPIRED")
        .whitelist_var("PAM_CRED_ERR")
        .whitelist_var("PAM_ACCT_EXPIRED")
        .whitelist_var("PAM_AUTHTOK_EXPIRED")
        .whitelist_var("PAM_SESSION_ERR")
        .whitelist_var("PAM_AUTHTOK_ERR")
        .whitelist_var("PAM_AUTHTOK_RECOVERY_ERR")
        .whitelist_var("PAM_AUTHTOK_LOCK_BUSY")
        .whitelist_var("PAM_AUTHTOK_DISABLE_AGING")
        .whitelist_var("PAM_NO_MODULE_DATA")
        .whitelist_var("PAM_IGNORE")
        .whitelist_var("PAM_ABORT")
        .whitelist_var("PAM_TRY_AGAIN")
        .whitelist_var("PAM_MODULE_UNKNOWN")
        .whitelist_var("PAM_DOMAIN_UNKNOWN")
        .whitelist_var("PAM_BAD_HANDLE")
        .whitelist_var("PAM_BAD_ITEM")
        .whitelist_var("PAM_BAD_FEATURE")
        .whitelist_var("PAM_BAD_CONSTANT")
        .whitelist_var("PAM_NUM_ERRORS")
        .whitelist_var("PAM_PROMPT_ECHO_OFF")
        .whitelist_var("PAM_PROMPT_ECHO_ON")
        .whitelist_var("PAM_ERROR_MSG")
        .whitelist_var("PAM_TEXT_INFO")
        .whitelist_var("PAM_MAX_NUM_MSG")
        .whitelist_var("PAM_MAX_MSG_SIZE")
        .whitelist_var("PAM_MAX_RESP_SIZE")
        .whitelist_var("PAM_SERVICE")
        .whitelist_var("PAM_USER")
        .whitelist_var("PAM_TTY")
        .whitelist_var("PAM_RHOST")
        .whitelist_var("PAM_CONV")
        .whitelist_var("PAM_AUTHTOK")
        .whitelist_var("PAM_OLDAUTHTOK")
        .whitelist_var("PAM_RUSER")
        .whitelist_var("PAM_USER_PROMPT")
        .whitelist_var("PAM_REPOSITORY")
        .whitelist_var("PAM_AUTHTOK_PROMPT")
        .whitelist_var("PAM_OLDAUTHTOK_PROMPT")
        .whitelist_var("PAM_HOST")
        .whitelist_var("PAM_NUM_ITEMS")
        // Whitelist all PAM functions..
        .whitelist_function("pam_.*")
        // ..except module related functions (pam_sm_*)
        .blacklist_function("pam_sm_.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
