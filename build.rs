use std::env;

const PAM_IMPL_ENV_VAR: &str = "PAM_SYS_IMPL";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PamImplementation {
    LinuxPam,
    OpenPam,
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

    fn get_additional_libs(self) -> &'static [&'static str] {
        match self {
            Self::LinuxPam => &["pam_misc"],
            Self::OpenPam => &[],
        }
    }
}

fn main() {
    let pam_implementation = PamImplementation::resolve();
    println!(
        "cargo:rustc-cfg=PAM_SYS_IMPL=\"{impl_name}\"",
        impl_name = pam_implementation.impl_name()
    );

    // Tell cargo to tell rustc to link the system pam shared library.
    println!("cargo:rustc-link-lib=pam");
    for additional_lib in pam_implementation.get_additional_libs() {
        println!("cargo:rustc-link-lib={additional_lib}",);
    }
}
