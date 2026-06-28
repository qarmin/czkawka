fn main() {
    let commit = git_output(["rev-parse", "--short", "HEAD"]);
    let date = git_output(["log", "-1", "--format=%cd", "--date=format:%d-%m-%Y"]);
    println!("cargo:rustc-env=CZKAWKA_CLI_BUILD_COMMIT={commit}");
    println!("cargo:rustc-env=CZKAWKA_CLI_BUILD_DATE={date}");
    println!("cargo:rerun-if-changed=.git/HEAD");
}

fn git_output<const N: usize>(args: [&str; N]) -> String {
    std::process::Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map_or_else(|| "unknown".to_string(), |s| s.trim().to_string())
}
