use std::env;

fn main() {
    if env::var("CARGO_FEATURE_ADD_WINDOWS_ICON").is_ok() {
        if let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS") {
            if target_os == "windows" {
                let mut res = winresource::WindowsResource::new();
                res.set_icon("icons/krokiet_logo_flag.ico");
                res.compile().expect("Unable to compile icon");
            }
        }
    }
}
