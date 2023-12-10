fn main() {
    let rust_version;
    match rustc_version::version_meta() {
        Ok(meta) => {
            let rust_v = meta.semver.to_string();
            let rust_date = meta.commit_date.unwrap_or_default();
            rust_version = format!("{rust_v} ({rust_date})");
        }
        Err(_) => {
            rust_version = "<unknown>".to_string();
        }
    };
    println!("cargo:rustc-env=RUST_VERSION_INTERNAL={rust_version}");
}
