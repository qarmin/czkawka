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

    // Find if app is build with cranelift
    if let Ok(codegen) = std::env::var("CARGO_PROFILE_RELEASE_CODEGEN_UNITS") {
        if codegen == "1" {
            println!("cargo:rustc-env=USING_CRANELIFT=1");
        }
    }
}
