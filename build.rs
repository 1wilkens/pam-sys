extern crate bindgen;

use std::env;
use std::path::PathBuf;

const PAM_IMPL_ENV_VAR: &str = "USE_PAM_IMPL";

#[derive(Clone, Copy, PartialEq, Eq)]
enum PamImplementation {
    LinuxPam,
    OpenPam,
}

fn default_builder() -> bindgen::Builder {
    bindgen::Builder::default()
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
        .blocklist_type("__va_list_tag")
        .blocklist_function("pam_v.*")
        .blocklist_function("pam_syslog")
        .blocklist_function("pam_prompt")

        // Allow all PAM constants
        .allowlist_var("PAM_.*")
        // Allow all PAM functions..
        .allowlist_function("pam_.*")
}

impl PamImplementation {
    fn resolve() -> Self {
        if let Ok(pam_imp) = env::var(PAM_IMPL_ENV_VAR) {
            let pam_impl = pam_imp.to_lowercase();
            return match &pam_impl[..] {
                "linuxpam" => Self::LinuxPam,
                "openpam" => Self::OpenPam,
                _ => {
                    panic!("Unrecognized '{}' environment variable value '{}'. Assessing from other information.", PAM_IMPL_ENV_VAR, pam_impl);
                }
            };
        }

        // LinuxPAM is used by linux and android
        if cfg!(target_os = "linux") || cfg!(target_os = "android") {
            Self::LinuxPam
        } else if cfg!(target_os = "freebsd")
            || cfg!(target_os = "netbsd")
            || cfg!(target_os = "macos")
            || cfg!(target_os = "ios")
            || cfg!(target_os = "dragonfly")
        {
            Self::OpenPam
        } else {
            panic!("Failed to resolve the PAM implementation. Use an appropriate target platform or set the `{}` environment variable to either `LINUXPAM` or `OPENPAM`.", PAM_IMPL_ENV_VAR);
        }
    }

    fn impl_name(self) -> &'static str {
        match self {
            Self::LinuxPam => "linux-pam",
            Self::OpenPam => "openpam",
        }
    }
}

fn main() {
    // Tell cargo to tell rustc to link the system pam shared library.
    println!("cargo:rustc-link-lib=pam");

    let pam_implementation = PamImplementation::resolve();
    println!(
        "cargo:rustc-cfg=pam_sys_pam_impl=\"{impl_name}\"",
        impl_name = pam_implementation.impl_name()
    );

    use PamImplementation::{LinuxPam, OpenPam};

    // pam_misc is only supported on Linux
    if pam_implementation == LinuxPam {
        println!("cargo:rustc-link-lib=pam_misc");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Prepare bindgen builder

    // Platform-specific adaptions
    match pam_implementation {
        LinuxPam => {
            builder = builder
                // Set macro constants to signed int, as all functions that accept these constants use
                // signed int as the parameter type
                .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
                // Use libc types so our signatures are slightly nicer
                .raw_line("use libc::{uid_t, gid_t, group, passwd, spwd};")
                .blocklist_type(".*gid_t")
                .blocklist_type(".*uid_t")
                .blocklist_type("group")
                .blocklist_type("passwd")
                .blocklist_type("spwd");
        }
        OpenPam => {
            builder = builder
                // Use libc types so our signatures are slightly nicer
                .raw_line("use libc::passwd;")
                .blocklist_type("passwd");
        }
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
