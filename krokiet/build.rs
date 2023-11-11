use std::env;

fn main() {
    if env::var("SLINT_STYLE").is_err() || env::var("SLINT_STYLE") == Ok(String::new()) {
        slint_build::compile_with_config("ui/main_window.slint", slint_build::CompilerConfiguration::new().with_style("fluent-dark".into())).unwrap();
    } else {
        slint_build::compile("ui/main_window.slint").unwrap();
    }
}
