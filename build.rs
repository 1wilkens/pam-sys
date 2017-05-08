fn main() {
    //TODO: expand this
    println!("cargo:rustc-link-lib=pam");
    if cfg!(not(target_os = "macos")) {
        println!("cargo:rustc-link-lib=pam_misc");
    }
}
