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

    // Prepare bindgen builder
    let mut builder = bindgen::Builder::default()
        // Our header
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
        .blacklist_type("__va_list")
        .blacklist_type("__builtin_va_list")
        .blacklist_type("__va_list_tag")
        .blacklist_function("pam_v.*")
        .blacklist_function("pam_syslog")
        .blacklist_function("pam_prompt")
        // Whitelist all PAM constants
        .whitelist_var("PAM_.*")
        // Whitelist all PAM functions..
        .whitelist_function("pam_.*")
        // ..except module related functions (pam_sm_*)
        .blacklist_function("pam_sm_.*");

    // Platform-specific adaptions
    if cfg!(target_os = "linux") {
        // Import libc so our signatures are slightly nicer
        builder = builder
            .raw_line("use libc::{uid_t, gid_t, group, passwd, spwd};")
            // Blacklist types we use from libc
            .blacklist_type(".*gid_t")
            .blacklist_type(".*uid_t")
            .blacklist_type("group")
            .blacklist_type("passwd")
            .blacklist_type("spwd");
    } else if cfg!(target_os = "freebsd") {
        // XXX: this should include all OS that use openPAM
        // Fix PAM_SILENT
        builder = builder
            .raw_line("pub const PAM_SILENT: libc::c_uint = 0x8000_0000;")
            .blacklist_item("PAM_SILENT");
    }

    let bindings = builder
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
