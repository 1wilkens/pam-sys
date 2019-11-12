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
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // TODO: find out what is broken here
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //
        // Import std::os::raw so our signatures a slightly nicer
        .raw_line("use libc::{uid_t, gid_t, group, passwd, spwd};")
        // And use just `raw` as type prefix
        .ctypes_prefix("libc")
        // Blacklist varargs functions and related types for now
        // TODO: find a nice solution for this
        .blacklist_type("va_list")
        .blacklist_type("__builtin_va_list")
        .blacklist_type("__va_list_tag")
        .blacklist_function("pam_v.*")
        // Blacklist types we use from libc
        .blacklist_type(".*gid_t")
        .blacklist_type(".*uid_t")
        .blacklist_type("group")
        .blacklist_type("passwd")
        .blacklist_type("spwd")
        // Whitelist all PAM constants
        .whitelist_var("PAM_.*")
        // Whitelist all PAM functions
        .whitelist_function("pam_.*")
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
