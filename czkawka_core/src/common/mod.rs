pub mod audio_fingerprint;
pub mod basic_gui_cli;
pub mod build_runtime_info;
pub mod cache;
pub mod config_cache_path;
pub mod consts;
pub mod dir_traversal;
pub mod directories;
pub mod extensions;
pub mod ffmpeg_utils;
pub mod image;
pub mod items;
pub mod logger;
pub mod model;
pub mod process_utils;
pub mod progress_data;
pub mod progress_stop_handler;
pub mod tool_data;
pub mod traits;
pub mod video_utils;

pub mod deletion;
pub mod formatting;
pub mod fs_ops;
pub mod path_utils;
pub mod threads;

pub use formatting::*;
pub use fs_ops::*;
pub use path_utils::*;
pub use threads::*;

#[cfg(test)]
mod test {
    use std::fs::{File, Metadata, read_dir};
    use std::io::Write;
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::MetadataExt;
    use std::path::{Path, PathBuf};
    use std::time::Duration;
    use std::{fs, io};

    use tempfile::tempdir;

    use super::*;
    use crate::common::items::new_excluded_item;

    #[cfg(target_family = "unix")]
    fn assert_inode(before: &Metadata, after: &Metadata) {
        assert_eq!(before.ino(), after.ino());
    }

    #[cfg(target_family = "windows")]
    fn assert_inode(_: &Metadata, _: &Metadata) {}

    #[cfg(target_family = "unix")]
    fn assert_different_inode(before: &Metadata, after: &Metadata) {
        assert_ne!(before.ino(), after.ino());
    }

    #[cfg(target_family = "windows")]
    fn assert_different_inode(_before: &Metadata, _after: &Metadata) {}

    #[test]
    fn test_make_hard_link() -> io::Result<()> {
        // Test 1: Basic hardlink creation
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
            File::create(&src)?;
            let metadata = fs::metadata(&src)?;
            File::create(&dst)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&metadata, &dst_metadata_before);

            make_hard_link(&src, &dst)?;

            make_hard_link(&src, &dst)?;

            assert_inode(&metadata, &fs::metadata(&dst)?);
            assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
            assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);
            assert_inode(&metadata, &fs::metadata(&src)?);
            assert_eq!(metadata.permissions(), fs::metadata(&src)?.permissions());
            assert_eq!(metadata.modified()?, fs::metadata(&src)?.modified()?);

            let mut actual = read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>();
            actual.sort_unstable();
            assert_eq!(vec![src, dst], actual);
        }

        // Test 2: Hardlink creation fails when source doesn't exist
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
            File::create(&dst)?;
            let metadata = fs::metadata(&dst)?;

            assert!(make_hard_link(&src, &dst).is_err());

            assert_inode(&metadata, &fs::metadata(&dst)?);
            assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
            assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);

            assert_eq!(vec![dst], read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>());
        }

        // Test 3: Hardlink with content preservation
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("src_file"), dir.path().join("dst_file"));
            let content = "test content for hardlink";
            {
                let mut f = File::create(&src)?;
                writeln!(f, "{content}")?;
            }
            {
                let mut f = File::create(&dst)?;
                writeln!(f, "old content")?;
            }

            let src_metadata = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata, &dst_metadata_before);

            make_hard_link(&src, &dst)?;

            let src_content = fs::read_to_string(&src)?;
            let dst_content = fs::read_to_string(&dst)?;
            assert_eq!(src_content, dst_content);
            assert_eq!(src_content, format!("{content}\n"));
            assert_inode(&src_metadata, &fs::metadata(&dst)?);
        }

        // Test 4: Hardlink on readonly file
        #[cfg(target_family = "unix")]
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("readonly_src"), dir.path().join("readonly_dst"));

            {
                let mut f = File::create(&src)?;
                writeln!(f, "readonly content")?;
            }

            let mut perms = fs::metadata(&src)?.permissions();
            perms.set_readonly(true);
            fs::set_permissions(&src, perms)?;

            assert!(fs::metadata(&src)?.permissions().readonly());

            {
                let mut f = File::create(&dst)?;
                writeln!(f, "dst content")?;
            }

            let src_metadata_before = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata_before, &dst_metadata_before);

            make_hard_link(&src, &dst).unwrap();

            assert_inode(&src_metadata_before, &fs::metadata(&dst)?);
            assert_eq!(fs::read_to_string(&src)?, fs::read_to_string(&dst)?);

            assert!(fs::metadata(&src)?.permissions().readonly());
            assert!(fs::metadata(&dst)?.permissions().readonly());
        }

        // Test 5: Hardlink on readonly destination file
        #[cfg(target_family = "unix")]
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("src_normal"), dir.path().join("dst_readonly"));

            {
                let mut f = File::create(&src)?;
                writeln!(f, "source content")?;
            }

            {
                let mut f = File::create(&dst)?;
                writeln!(f, "destination content")?;
            }
            let mut perms = fs::metadata(&dst)?.permissions();
            perms.set_readonly(true);
            fs::set_permissions(&dst, perms)?;

            assert!(fs::metadata(&dst)?.permissions().readonly());

            let src_metadata = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata, &dst_metadata_before);

            make_hard_link(&src, &dst).unwrap();

            assert_inode(&src_metadata, &fs::metadata(&dst)?);
            assert_eq!(fs::read_to_string(&src)?, fs::read_to_string(&dst)?);
        }

        // Test 6: Hardlink when destination doesn't exist - should fail
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("src"), dir.path().join("nonexistent"));
            File::create(&src)?;

            let result = make_hard_link(&src, &dst);
            assert!(result.is_err(), "Should fail when destination doesn't exist");
        }

        // Test 7: Hardlink preserves file size
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("large_src"), dir.path().join("large_dst"));

            let large_content = "x".repeat(10000);
            {
                let mut f = File::create(&src)?;
                write!(f, "{large_content}")?;
            }
            File::create(&dst)?;

            let src_size = fs::metadata(&src)?.len();
            let src_metadata = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata, &dst_metadata_before);

            make_hard_link(&src, &dst)?;

            assert_eq!(src_size, fs::metadata(&dst)?.len());
            assert_eq!(large_content, fs::read_to_string(&dst)?);
        }

        // Test 8: Multiple hardlinks to same file
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let src = dir.path().join("original");
            let dst1 = dir.path().join("link1");
            let dst2 = dir.path().join("link2");

            {
                let mut f = File::create(&src)?;
                writeln!(f, "original")?;
            }
            File::create(&dst1)?;
            File::create(&dst2)?;

            let src_metadata = fs::metadata(&src)?;
            let dst1_metadata_before = fs::metadata(&dst1)?;
            let dst2_metadata_before = fs::metadata(&dst2)?;

            // Before hardlinks - all files should have different inodes
            assert_different_inode(&src_metadata, &dst1_metadata_before);
            assert_different_inode(&src_metadata, &dst2_metadata_before);
            assert_different_inode(&dst1_metadata_before, &dst2_metadata_before);

            make_hard_link(&src, &dst1)?;
            make_hard_link(&src, &dst2)?;

            assert_inode(&src_metadata, &fs::metadata(&dst1)?);
            assert_inode(&src_metadata, &fs::metadata(&dst2)?);
        }

        Ok(())
    }

    // Windows needs super user permissions
    #[cfg(target_family = "unix")]
    #[test]
    fn test_make_file_symlink() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        let content = "hello softlink";
        {
            let mut f = File::create(&src)?;
            writeln!(f, "{content}")?;
        }
        File::create(&dst)?;

        make_file_symlink(&src, &dst)?;

        let symlink_meta = fs::symlink_metadata(&dst)?;
        assert!(symlink_meta.file_type().is_symlink());

        let src_content = fs::read_to_string(&src)?;
        let dst_content = fs::read_to_string(&dst)?;
        assert_eq!(src_content, dst_content);

        let mut actual = read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>();
        actual.sort_unstable();
        assert_eq!(vec![src, dst], actual);
        Ok(())
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_make_file_symlink_fails() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        {
            let mut f = File::create(&dst)?;
            writeln!(f, "original")?;
        }
        let metadata = fs::metadata(&dst)?;

        match make_file_symlink(&src, &dst) {
            Err(_) => {
                assert_eq!(fs::read_to_string(&dst)?, "original\n");
                assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
            }
            Ok(()) => {
                let symlink_meta = fs::symlink_metadata(&dst)?;
                assert!(symlink_meta.file_type().is_symlink());
                fs::read_to_string(&dst).unwrap_err();
            }
        }
        Ok(())
    }

    #[test]
    fn test_remove_folder_if_contains_only_empty_folders() {
        let dir = tempdir().expect("Cannot create temporary directory");
        let sub_dir = dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).expect("Cannot create directory");

        // Test with empty directory
        remove_folder_if_contains_only_empty_folders(&sub_dir, false).unwrap();
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing an empty directory
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        fs::create_dir(sub_dir.join("empty_sub_dir")).expect("Cannot create directory");
        remove_folder_if_contains_only_empty_folders(&sub_dir, false).unwrap();
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing a file
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        let mut file = File::create(sub_dir.join("file.txt")).expect("Cannot create file");
        writeln!(file, "Hello, world!").expect("Cannot write to file");
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_err());
        assert!(Path::new(&sub_dir).exists());
    }

    #[test]
    fn test_regex() {
        assert!(regex_check(&new_excluded_item("*"), "/home/rafal"));
        assert!(regex_check(&new_excluded_item("*home*"), "/home/rafal"));
        assert!(regex_check(&new_excluded_item("*home"), "/home"));
        assert!(regex_check(&new_excluded_item("*home/"), "/home/"));
        assert!(regex_check(&new_excluded_item("*home/*"), "/home/"));
        assert!(regex_check(&new_excluded_item("*.git*"), "/home/.git"));
        assert!(regex_check(&new_excluded_item("/home/*/.*"), "/home/user/.random"));
        assert!(regex_check(&new_excluded_item("*/home/rafal*rafal*rafal*rafal*"), "/home/rafal/rafalrafalrafal"));
        assert!(regex_check(&new_excluded_item("AAA"), "AAA"));
        assert!(regex_check(&new_excluded_item("AAA*"), "AAABDGG/QQPW*"));
        assert!(!regex_check(&new_excluded_item("*home"), "/home/"));
        assert!(!regex_check(&new_excluded_item("*home"), "/homefasfasfasfasf/"));
        assert!(!regex_check(&new_excluded_item("*home"), "/homefasfasfasfasf"));
        assert!(!regex_check(&new_excluded_item("rafal*afal*fal"), "rafal"));
        assert!(!regex_check(&new_excluded_item("rafal*a"), "rafal"));
        assert!(!regex_check(&new_excluded_item("AAAAAAAA****"), "/AAAAAAAAAAAAAAAAA"));
        assert!(!regex_check(&new_excluded_item("*.git/*"), "/home/.git"));
        assert!(!regex_check(&new_excluded_item("*home/*koc"), "/koc/home/"));
        assert!(!regex_check(&new_excluded_item("*home/"), "/home"));
        assert!(!regex_check(&new_excluded_item("*TTT"), "/GGG"));
        assert!(regex_check(
            &new_excluded_item("*/home/*/.local/share/containers"),
            "/var/home/roman/.local/share/containers"
        ));

        if cfg!(target_family = "windows") {
            assert!(regex_check(&new_excluded_item("*\\home"), "C:\\home"));
        }
    }

    #[test]
    fn test_windows_path() {
        assert_eq!(PathBuf::from("C:\\path.txt"), normalize_windows_path("c:/PATH.tXt"));
        assert_eq!(PathBuf::from("H:\\reka\\weza\\roman.txt"), normalize_windows_path("h:/RekA/Weza\\roMan.Txt"));
        assert_eq!(PathBuf::from("T:\\a"), normalize_windows_path("T:\\A"));
        assert_eq!(PathBuf::from("\\\\aBBa"), normalize_windows_path("\\\\aBBa"));
        assert_eq!(PathBuf::from("a"), normalize_windows_path("a"));
        assert_eq!(PathBuf::from(""), normalize_windows_path(""));
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(Duration::from_millis(0)), "0ms");
        assert_eq!(format_time(Duration::from_millis(1)), "1ms");
        assert_eq!(format_time(Duration::from_millis(999)), "999ms");

        assert_eq!(format_time(Duration::from_millis(1000)), "1s");
        assert_eq!(format_time(Duration::from_millis(1234)), "1.23s");
        assert_eq!(format_time(Duration::from_millis(5678)), "5.67s");
        assert_eq!(format_time(Duration::from_secs(59)), "59s");

        assert_eq!(format_time(Duration::from_secs(60)), "1m");
        assert_eq!(format_time(Duration::from_secs(61)), "1m 1s");
        assert_eq!(format_time(Duration::from_millis(61234)), "1m 1s");
        assert_eq!(format_time(Duration::from_secs(125)), "2m 5s");
        assert_eq!(format_time(Duration::from_secs(3599)), "59m 59s");

        assert_eq!(format_time(Duration::from_secs(3600)), "1h");
        assert_eq!(format_time(Duration::from_secs(3661)), "1h 1m 1s");
        assert_eq!(format_time(Duration::from_secs(7384)), "2h 3m 4s");
        assert_eq!(format_time(Duration::from_secs(86400)), "24h");

        assert_eq!(format_time(Duration::from_millis(999)), "999ms");
        assert_eq!(format_time(Duration::from_millis(1001)), "1s");
        assert_eq!(format_time(Duration::from_millis(59999)), "59.99s");
        assert_eq!(format_time(Duration::from_millis(60000)), "1m");
        assert_eq!(format_time(Duration::from_millis(60100)), "1m");
        assert_eq!(format_time(Duration::from_millis(120000)), "2m");
    }
}
