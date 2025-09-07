fn main() {
    let rust_version = match rustc_version::version_meta() {
        Ok(meta) => {
            let rust_v = meta.semver.to_string();
            let rust_date = meta.commit_date.unwrap_or_default();
            format!("{rust_v} ({rust_date})")
        }
        Err(_) => "<unknown>".to_string(),
    };
    println!("cargo:rustc-env=RUST_VERSION_INTERNAL={rust_version}");

    if let Ok(encoded) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        println!("cargo:rustc-env=UUSED_RUSTFLAGS={encoded}");
    }

    let using_cranelift =
        std::env::var("CARGO_PROFILE_RELEASE_CODEGEN_UNITS") == Ok("1".to_string()) || std::env::var("CARGO_PROFILE_DEV_CODEGEN_BACKEND") == Ok("cranelift".to_string());

    if using_cranelift {
        println!("cargo:rustc-env=USING_CRANELIFT=1");
    }

    if cfg!(target_os = "linux") {
        if let Ok(ver) = glibc_musl_version::get_os_libc_versions() {
            println!("cargo:rustc-env=CZKAWKA_LIBC_VERSIONS={ver}");
        }

        if cfg!(target_env = "gnu") {
            println!("cargo:rustc-env=CZKAWKA_LIBC=glibc");
        } else if cfg!(target_env = "musl") {
            println!("cargo:rustc-env=CZKAWKA_LIBC=musl");
        } else {
            println!("cargo:rustc-env=CZKAWKA_LIBC=unknown");
        }
    }
}
