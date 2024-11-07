extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Prepare bindgen builder
    let mut builder = bindgen::Builder::default();

    if cfg!(feature = "dylib") {
        builder = builder.dynamic_library_name("pam");
    } else {
        // Tell cargo to tell rustc to link the system pam
        // shared library.
        println!("cargo:rustc-link-lib=pam");

        // pam_misc is only supported on Linux afaik
        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=pam_misc");
        }
    }

    builder = builder
        // Our header
        .header("wrapper.h")
        // Use libc for c-types
        .ctypes_prefix("libc")
        // pam_handle_t is opaque
        .opaque_type("pam_handle_t")
        // Block varargs functions and related types for now
        // TODO: find a nice solution for this
        .blocklist_type("va_list")
        .blocklist_type("__va_list")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__gnuc_va_list")
        .blocklist_type("__va_list_tag")
        .blocklist_function("pam_v.*")
        .blocklist_function("pam_syslog")
        .blocklist_function("pam_prompt")
        // Allow all PAM constants
        .allowlist_var("PAM_.*")
        // Allow all PAM functions..
        .allowlist_function("pam_.*")
        // ..except module related functions (pam_sm_*)
        .blocklist_function("pam_sm_.*");

    // Platform-specific adaptions
    if cfg!(target_os = "linux") {
        builder = builder
            // Set macro constants to signed int, as all functions that accept
            // these constants use signed int as the parameter type
            .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
            //
            // Use libc types so our signatures are slightly nicer
            .raw_line("use libc::{uid_t, gid_t, group, passwd, spwd};")
            .blocklist_type(".*gid_t")
            .blocklist_type(".*uid_t")
            .blocklist_type("group")
            .blocklist_type("passwd")
            .blocklist_type("spwd");
    } else if cfg!(target_os = "freebsd") || cfg!(target_os = "netbsd") {
        // XXX: this should include all OS that use openPAM
        builder = builder
            // Use libc types so our signatures are slightly nicer
            .raw_line("use libc::passwd;")
            .blocklist_type("passwd");
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
