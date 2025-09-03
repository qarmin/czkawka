pub struct Version {
    pub major: usize,
    pub minor: usize,
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod imp {
    use super::Version;
    use std::process::Command;
    use regex::Regex;

    // glibc version is taken from std/sys/unix/os.rs
    pub fn get_version() -> Result<Version, String> {
        let output = Command::new("ldd")
            .args(&["--version"])
            .output()
            .expect("failed to execute ldd");
        let output_str = std::str::from_utf8(&output.stdout).unwrap();
        let version_str = ldd_output_to_version_str(output_str)?;

        parse_glibc_version(version_str)
            .ok_or_else(|| format!("Invalid version string from ldd output: {}", version_str,))
    }

    fn ldd_output_to_version_str(output_str: &str) -> Result<&str, String> {
        let version_reg = Regex::new(r#"ldd \(.+\) ([0-9]+\.[0-9]+)"#).unwrap();
        if let Some(captures) = version_reg.captures(output_str) {
            Ok(captures.get(1).unwrap().as_str())
        } else {
            Err(format!(
                "ERROR: failed to detect glibc version. ldd output: {}",
                output_str,
            ))
        }
    }

    // Returns Some((major, minor)) if the string is a valid "x.y" version,
    // ignoring any extra dot-separated parts. Otherwise return None.
    fn parse_glibc_version(version: &str) -> Option<Version> {
        let mut parsed_ints = version.split('.').map(str::parse::<usize>).fuse();
        match (parsed_ints.next(), parsed_ints.next()) {
            (Some(Ok(major)), Some(Ok(minor))) => Some(Version { major, minor }),
            _ => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_ldd_output() {
            let ver_str = ldd_output_to_version_str(
                r#"ldd (GNU libc) 2.12
Copyright (C) 2010 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
Written by Roland McGrath and Ulrich Drepper."#,
            )
                .unwrap();
            assert_eq!(ver_str, "2.12");

            let ver_str = ldd_output_to_version_str(
                r#"ldd (Ubuntu GLIBC 2.41-6ubuntu1.1) 2.41
Copyright (C) 2024 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
Written by Roland McGrath and Ulrich Drepper."#,
            )
                .unwrap();
            assert_eq!(ver_str, "2.31");
        }
    }
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
mod imp {
    use super::Version;
    pub fn get_version() -> Result<Version, String> {
        unimplemented!();
    }
}

#[inline]
pub fn get_version() -> Result<Version, String> {
    imp::get_version()
}
