use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Re-run build script if i18n.toml or this build script changes
    println!("cargo:rerun-if-changed=i18n.toml");
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let crate_root = Path::new(&manifest_dir);
    let i18n_dir = crate_root.join("i18n");
    if let Err(e) = fs::create_dir_all(&i18n_dir) {
        panic!("Failed to create i18n directory {:?}: {}", i18n_dir, e);
    }

    let toml_path = crate_root.join("i18n.toml");
    let mut entries: Vec<(String, String)> = Vec::new();

    if toml_path.exists() {
        let toml_str = fs::read_to_string(&toml_path).unwrap_or_else(|e| {
            panic!("Failed to read {:?}: {}", toml_path, e);
        });

        // Try to extract [en] section first. If present, prefer it.
        if let Some(en_section) = extract_section(&toml_str, "en") {
            entries = parse_key_values(&en_section);
        }

        // If no [en] section or empty, parse top-level simple key = "value" pairs
        if entries.is_empty() {
            entries = parse_key_values(&toml_str);
        }
    }

    // Ensure en directory exists and target filename matches what the i18n macros expect
    let en_dir = i18n_dir.join("en");
    if let Err(e) = fs::create_dir_all(&en_dir) {
        panic!("Failed to create en i18n directory {:?}: {}", en_dir, e);
    }

    // If still empty, write minimal fallback translations
    if entries.is_empty() {
        let fallback = r#"core_similarity_very_high = Very high
core_similarity_high = High
core_similarity_medium = Medium
core_similarity_small = Small
core_similarity_very_small = Very small
core_similarity_minimal = Minimal
core_ffmpeg_not_found = FFmpeg not found
"#;
        let target = en_dir.join("czkawka_core.ftl");
        fs::write(&target, fallback).unwrap_or_else(|e| {
            panic!("Failed to write fallback {:?}: {}", target, e);
        });
        return;
    }

    // Build the .ftl contents
    let mut ftl = String::new();
    for (k, v) in entries {
        // For .ftl keep value as-is; keys/values are simple strings
        ftl.push_str(&format!("{} = {}\n", k, v));
    }

    let target = en_dir.join("czkawka_core.ftl");
    fs::write(&target, ftl).unwrap_or_else(|e| {
        panic!("Failed to write {:?}: {}", target, e);
    });
}

fn extract_section(s: &str, section: &str) -> Option<String> {
    let start = format!("[{}]", section);
    let mut in_section = false;
    let mut collected = String::new();
    for line in s.lines() {
        let line_trim = line.trim();
        if line_trim.starts_with('[') && line_trim.ends_with(']') {
            if in_section {
                break; // end of section
            }
            if line_trim == start {
                in_section = true;
                continue;
            }
        }
        if in_section {
            collected.push_str(line);
            collected.push('\n');
        }
    }
    if in_section { Some(collected) } else { None }
}

fn parse_key_values(s: &str) -> Vec<(String, String)> {
    let mut res = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim().trim_matches('"').trim_matches('\'').to_string();
            let mut val = line[idx+1..].trim();
            // strip optional quotes
            if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                val = &val[1..val.len()-1];
            }
            // Unescape simple escaped quotes
            let val = val.replace("\\\"", "\"").replace("\\'", "'");
            res.push((key, val));
        }
    }
    res
}
