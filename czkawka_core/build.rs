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

    // Get Git commit hash
    let git_commit = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| if output.status.success() { String::from_utf8(output.stdout).ok() } else { None })
        .map_or_else(|| "<unknown>".to_string(), |s| s.trim().to_string());
    println!("cargo:rustc-env=CZKAWKA_GIT_COMMIT={git_commit}");

    // Get short Git commit hash
    let git_commit_short = if git_commit != "<unknown>" && git_commit.len() >= 10 {
        git_commit.chars().take(10).collect::<String>()
    } else {
        git_commit
    };
    println!("cargo:rustc-env=CZKAWKA_GIT_COMMIT_SHORT={git_commit_short}");

    // Commit date
    let git_commit_date = std::process::Command::new("git")
        .args(["log", "-1", "--format=%cd", "--date=format:%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|output| if output.status.success() { String::from_utf8(output.stdout).ok() } else { None })
        .map_or_else(|| "<unknown>".to_string(), |s| s.trim().to_string());
    println!("cargo:rustc-env=CZKAWKA_GIT_COMMIT_DATE={git_commit_date}");

    // Official build flag
    if std::env::var("CZKAWKA_OFFICIAL_BUILD") == Ok("1".to_string()) {
        println!("cargo:rustc-env=CZKAWKA_OFFICIAL_BUILD=1");
    } else {
        println!("cargo:rustc-env=CZKAWKA_OFFICIAL_BUILD=0");
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
