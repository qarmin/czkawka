pub struct TestCase {
    pub name: &'static str,
    /// Args passed to czkawka. Use the literal string `"TestFiles"` as the
    /// directory placeholder – it will be replaced with the actual per-test
    /// unique directory at runtime.
    pub args: &'static [&'static str],
    pub expected_files: &'static [&'static str],
    pub expected_folders: &'static [&'static str],
    pub expected_symlinks: &'static [&'static str],
}

pub fn all_test_cases() -> Vec<TestCase> {
    vec![
        //  Misc 
        TestCase {
            name: "empty_files",
            args: &["empty-files", "-d", "TestFiles", "-D", "-W"],
            expected_files: &["EmptyFile"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "big_files",
            args: &["big", "-d", "TestFiles", "-n", "2", "-D", "-W"],
            expected_files: &["Music/M4.mp3", "Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "smallest_files",
            args: &["big", "-d", "TestFiles", "-J", "-n", "5", "-D", "-W"],
            expected_files: &[
                "Broken/Br.jpg",
                "Broken/Br.mp3",
                "Broken/Br.pdf",
                "Broken/Br.zip",
                "EmptyFolders/ThreeButNot/KEKEKE",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "biggest_files",
            args: &["big", "-d", "TestFiles", "-n", "6", "-D", "-W"],
            expected_files: &[
                "Music/M3.flac",
                "Music/M4.mp3",
                "Videos/V1.mp4",
                "Videos/V2.mp4",
                "Videos/V3.webm",
                "Videos/V5.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "empty_folders",
            args: &["empty-folders", "-d", "TestFiles", "-D", "-W"],
            expected_files: &[],
            expected_folders: &["EmptyFolders/One", "EmptyFolders/Two", "EmptyFolders/Two/TwoInside"],
            expected_symlinks: &[],
        },
        TestCase {
            name: "temporary_files",
            args: &["temp", "-d", "TestFiles", "-D", "-W"],
            expected_files: &["Temporary/Boczze.cache"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "symlinks_files",
            args: &["symlinks", "-d", "TestFiles", "-D", "-W"],
            expected_files: &[],
            expected_folders: &[],
            expected_symlinks: &["Symlinks/EmptyFiles"],
        },
        //  Duplicates 
        TestCase {
            name: "dup_one_oldest",
            args: &["dup", "-d", "TestFiles", "-D", "OO", "-W"],
            expected_files: &["Images/A2.jpg", "Music/M5.mp3", "Videos/V2.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_one_newest",
            args: &["dup", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Images/A1.jpg", "Music/M2.mp3", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_oldest",
            args: &["dup", "-d", "TestFiles", "-D", "AEO", "-W"],
            expected_files: &[
                "Images/A1.jpg",
                "Images/A5.jpg",
                "Music/M1.mp3",
                "Music/M2.mp3",
                "Videos/V1.mp4",
                "Videos/V5.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_newest",
            args: &["dup", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &[
                "Images/A2.jpg",
                "Images/A5.jpg",
                "Music/M1.mp3",
                "Music/M5.mp3",
                "Videos/V1.mp4",
                "Videos/V2.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_one_smallest",
            args: &["dup", "-d", "TestFiles", "-D", "OS", "-W"],
            expected_files: &["Images/A1.jpg", "Music/M1.mp3", "Videos/V1.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_one_biggest",
            args: &["dup", "-d", "TestFiles", "-D", "OB", "-W"],
            expected_files: &["Images/A5.jpg", "Music/M5.mp3", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_smallest",
            args: &["dup", "-d", "TestFiles", "-D", "AES", "-W"],
            expected_files: &[
                "Images/A2.jpg",
                "Images/A5.jpg",
                "Music/M2.mp3",
                "Music/M5.mp3",
                "Videos/V2.mp4",
                "Videos/V5.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_biggest",
            args: &["dup", "-d", "TestFiles", "-D", "AEB", "-W"],
            expected_files: &[
                "Images/A1.jpg",
                "Images/A2.jpg",
                "Music/M1.mp3",
                "Music/M2.mp3",
                "Videos/V1.mp4",
                "Videos/V2.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  Music by tags 
        TestCase {
            name: "music_tags_one_oldest",
            args: &["music", "-d", "TestFiles", "-D", "OO", "-W"],
            expected_files: &["Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_one_newest",
            args: &["music", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Music/M2.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_oldest",
            args: &["music", "-d", "TestFiles", "-D", "AEO", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_newest",
            args: &["music", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_one_smallest",
            args: &["music", "-d", "TestFiles", "-D", "OS", "-W"],
            expected_files: &["Music/M1.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_one_biggest",
            args: &["music", "-d", "TestFiles", "-D", "OB", "-W"],
            expected_files: &["Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_smallest",
            args: &["music", "-d", "TestFiles", "-D", "AES", "-W"],
            expected_files: &["Music/M2.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_biggest",
            args: &["music", "-d", "TestFiles", "-D", "AEB", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  Music by content 
        TestCase {
            name: "music_content_one_oldest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OO", "-W"],
            expected_files: &["Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_one_newest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "ON", "-W"],
            expected_files: &["Music/M2.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_oldest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEO", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_newest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEN", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_one_smallest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OS", "-W"],
            expected_files: &["Music/M2.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_one_biggest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OB", "-W"],
            expected_files: &["Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_smallest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AES", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_biggest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEB", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  Video 
        TestCase {
            name: "video_one_oldest",
            args: &["video", "-d", "TestFiles", "-D", "OO", "-W"],
            expected_files: &["Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_one_newest",
            args: &["video", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_oldest",
            args: &["video", "-d", "TestFiles", "-D", "AEO", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_newest",
            args: &["video", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_one_smallest",
            args: &["video", "-d", "TestFiles", "-D", "OS", "-W"],
            expected_files: &["Videos/V2.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_one_biggest",
            args: &["video", "-d", "TestFiles", "-D", "OB", "-W"],
            expected_files: &["Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_smallest",
            args: &["video", "-d", "TestFiles", "-D", "AES", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V3.webm", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_biggest",
            args: &["video", "-d", "TestFiles", "-D", "AEB", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  Similar images 
        // The perceptual-hash group contains A1.jpg, A2.jpg, A3.png and A5.jpg
        // (all visually identical; A4.jpg is a different image and is excluded).
        // -m 1 lowers the default 16 KiB size threshold so the 8 772 B JPEGs
        // are not skipped.
        //
        // mtime ordering (oldest → newest):
        //   A3.png  2023-10-11 19:47  ← overall oldest, also biggest (20 920 B)
        //   A2.jpg  2023-10-12 15:21  ← tied with A5
        //   A5.jpg  2023-10-12 15:21  ← tied with A2
        //   A1.jpg  2023-10-12 15:36  ← newest
        //
        // size ordering: A1 = A2 = A5 = 8 772 B (smallest, tied)  <  A3 = 20 920 B (biggest)
        // tiebreaker among equal sizes/mtimes: alphabetical (A1 < A2 < A5).
        TestCase {
            name: "image_one_oldest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "OO", "-W"],
            // OO deletes ONE oldest: A3.png (mtime 2023-10-11, oldest in the group)
            expected_files: &["Images/A3.png"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_one_newest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "ON", "-W"],
            // ON deletes ONE newest: A1.jpg (mtime 2023-10-12 15:36, newest)
            expected_files: &["Images/A1.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_all_except_oldest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "AEO", "-W"],
            // AEO keeps oldest (A3.png); deletes A1, A2, A5
            expected_files: &["Images/A1.jpg", "Images/A2.jpg", "Images/A5.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_all_except_newest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "AEN", "-W"],
            // AEN keeps newest (A1.jpg); deletes A2, A3, A5
            expected_files: &["Images/A2.jpg", "Images/A3.png", "Images/A5.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_one_smallest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "OS", "-W"],
            // OS deletes ONE smallest: A1.jpg (alphabetically first among the three 8 772 B files)
            expected_files: &["Images/A1.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_one_biggest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "OB", "-W"],
            // OB deletes ONE biggest: A3.png (20 920 B, only one at that size)
            expected_files: &["Images/A3.png"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_all_except_smallest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "AES", "-W"],
            // AES keeps smallest (A1.jpg, alphabetically first among tied); deletes A2, A3, A5
            expected_files: &["Images/A2.jpg", "Images/A3.png", "Images/A5.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "image_all_except_biggest",
            args: &["image", "-d", "TestFiles", "-m", "1", "-D", "AEB", "-W"],
            // AEB keeps biggest (A3.png, 20 920 B); deletes A1, A2, A5
            expected_files: &["Images/A1.jpg", "Images/A2.jpg", "Images/A5.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  Bad extensions 
        // BadExtensions/BE.jpg is actually an MP4 (ftypisom magic bytes).
        // Running `ext -F` renames it to BE.mp4, so BE.jpg disappears.
        TestCase {
            name: "ext_fix",
            args: &["ext", "-d", "TestFiles", "-F", "-W"],
            expected_files: &["BadExtensions/BE.jpg"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  Bad names 
        // The synthetic entry BadNames/photo.JPG has an all-uppercase extension.
        // Running `bad-names -u -F` renames it to photo.jpg, so it disappears.
        TestCase {
            name: "bad_names_uppercase_ext",
            args: &["bad-names", "-d", "TestFiles", "-u", "-F", "-W"],
            expected_files: &["BadNames/photo.JPG"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        //  EXIF remover 
        // Scan only (no -F flag): no files are modified, so nothing disappears.
        // This verifies the tool runs cleanly against the real test data.
        TestCase {
            name: "exif_remover_scan",
            args: &["exif-remover", "-d", "TestFiles", "-W"],
            expected_files: &[],
            expected_folders: &[],
            expected_symlinks: &[],
        },
    ]
}
