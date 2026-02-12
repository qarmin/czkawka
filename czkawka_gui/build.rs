use std::env;

fn main() {
    if env::var("CARGO_FEATURE_ADD_WINDOWS_ICON").is_ok()
        && let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS")
        && target_os == "windows"
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.compile().expect("Unable to compile icon");
    }
}
