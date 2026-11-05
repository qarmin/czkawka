//! Remembering encoder configurations between runs.
//!
//! A file that lost its `moov` cannot be rebuilt without the decoder parameters that went with
//! it, and those come from a sibling recording off the same device (see [`super::mp4_profile`]).
//! Learning them is a pass over the whole batch, and until now it happened once per invocation
//! and was thrown away afterwards: repairing ten files out of a collection of ten thousand meant
//! either reading the collection again or naming a reference by hand.
//!
//! What is worth keeping is tiny and long-lived. A configuration is about 750 bytes, identical
//! ones collapse, and it describes a *camera* rather than a recording - so a few dozen entries
//! cover a user's entire archive and stay correct for years. That is a cache, so it lives where
//! caches live, and losing it costs a re-scan and nothing else.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::mp4_profile::{self, Profile};

/// Entries kept at most, so an archive full of one-off devices cannot grow the cache without
/// bound. Measured on a 62 GB carved collection: 31 distinct configurations.
const MAX_PROFILES: usize = 512;

/// Environment variable naming the store, which is how a test points it somewhere disposable.
const STORE_VARIABLE: &str = "FILE_FIXER_PROFILE_STORE";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    pub profiles: Vec<Profile>,
    /// MPEG-4 Part 2 sequence headers, kept beside the H.264 and HEVC parameter sets for the
    /// same reason: an old phone recording carries its picture dimensions nowhere else.
    #[serde(default)]
    pub mpeg4_headers: Vec<Vec<u8>>,
}

impl Store {
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty() && self.mpeg4_headers.is_empty()
    }
}

/// Where the store lives, unless the environment names somewhere else.
pub fn default_path() -> Option<PathBuf> {
    resolve(std::env::var_os(STORE_VARIABLE), std::env::var_os("XDG_CACHE_HOME"), std::env::var_os("HOME"))
}

/// The path rule on its own, so it can be checked without touching the process environment.
fn resolve(named: Option<OsString>, cache_home: Option<OsString>, home: Option<OsString>) -> Option<PathBuf> {
    if let Some(named) = named {
        return Some(PathBuf::from(named));
    }
    let cache = cache_home.map(PathBuf::from).or_else(|| home.map(|home| PathBuf::from(home).join(".cache")))?;
    Some(cache.join("file_fixer").join("profiles.json"))
}

/// Reads the store, treating anything unreadable as an empty one.
///
/// A cache that cannot be parsed is a cache that needs rebuilding, not an error worth stopping
/// a repair over - the format may simply be one version older than this binary.
pub fn read(path: &Path) -> Store {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_else(|error| {
            log::debug!("the profile store at {} is not readable: {error}", path.display());
            Store::default()
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Store::default(),
        Err(error) => {
            log::debug!("cannot read the profile store at {}: {error}", path.display());
            Store::default()
        }
    }
}

pub fn write(path: &Path, store: &Store) -> std::io::Result<()> {
    if let Some(directory) = path.parent() {
        std::fs::create_dir_all(directory)?;
    }
    let json = serde_json::to_vec_pretty(store).map_err(std::io::Error::other)?;
    // Persisted through a temp file in the same directory so an interrupted write cannot leave a half-written store.
    let directory = path.parent().unwrap_or_else(|| Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(directory)?;
    std::io::Write::write_all(&mut temp, &json)?;
    temp.persist(path).map_err(|error| error.error)?;
    Ok(())
}

/// Loads the store into the in-memory library the repair strategies consult.
///
/// Returns how many configurations came back, which is worth telling the user: a rebuild that
/// succeeds because of a camera scanned last month is otherwise indistinguishable from magic.
pub fn load(path: &Path) -> usize {
    let store = read(path);
    let count = store.profiles.len();
    mp4_profile::learn(store.profiles);
    mp4_profile::learn_mpeg4(store.mpeg4_headers);
    count
}

/// Writes everything learned so far, merged with whatever the store already held.
///
/// Merging rather than replacing matters because runs are not nested: repairing one directory
/// today must not forget the camera that was scanned yesterday.
pub fn save(path: &Path) -> std::io::Result<usize> {
    let mut store = read(path);
    let before = (store.profiles.len(), store.mpeg4_headers.len());

    mp4_profile::merge(&mut store.profiles, mp4_profile::learned());
    for header in mp4_profile::learned_mpeg4() {
        if !store.mpeg4_headers.contains(&header) {
            store.mpeg4_headers.push(header);
        }
    }
    store.profiles.truncate(MAX_PROFILES);

    let count = store.profiles.len();
    // Every repair run ends here, and most of them learn nothing the store did not already
    // hold; rewriting the file anyway would touch the user's cache for no reason.
    if (count, store.mpeg4_headers.len()) == before {
        return Ok(count);
    }
    write(path, &store).map(|()| count)
}

#[cfg(test)]
mod tests {
    use super::super::mp4_profile::VideoCodec;
    use super::*;

    fn profile(sps: &[u8]) -> Profile {
        Profile {
            codec: VideoCodec::Hevc,
            video_parameter_sets: vec![vec![0x40, 0x01]],
            sequence_parameter_sets: vec![sps.to_vec()],
            picture_parameter_sets: vec![vec![0x44, 0x01]],
            picture_parameter_set_ids: vec![0],
            max_sample_size: 123_456,
            frame_rate: 29.97,
        }
    }

    #[test]
    fn a_store_survives_the_round_trip_to_disk() {
        let file = tempfile::NamedTempFile::new().expect("temp file");
        let store = Store {
            profiles: vec![profile(b"one"), profile(b"two")],
            mpeg4_headers: vec![vec![0, 0, 1, 0xB0, 0xF5]],
        };
        write(file.path(), &store).expect("writes");

        let read_back = read(file.path());
        assert_eq!(read_back.profiles.len(), 2);
        assert_eq!(read_back.profiles.first().map(|p| p.codec), Some(VideoCodec::Hevc));
        assert_eq!(read_back.profiles.first().map(|p| p.sequence_parameter_sets.clone()), Some(vec![b"one".to_vec()]));
        assert_eq!(read_back.mpeg4_headers, store.mpeg4_headers);
    }

    /// A cache one version out of date, or half-written by an interrupted run, must not stop a
    /// repair - it just means there is nothing remembered yet.
    #[test]
    fn an_unreadable_store_reads_as_an_empty_one() {
        let file = tempfile::NamedTempFile::new().expect("temp file");
        std::fs::write(file.path(), b"{ not json at all").expect("writes");
        assert!(read(file.path()).is_empty());

        assert!(read(Path::new("/nonexistent/profiles.json")).is_empty());
    }

    /// Runs are not nested: repairing one directory today must not forget what was scanned
    /// yesterday, so saving merges into whatever is already there.
    #[test]
    fn saving_keeps_what_an_earlier_run_stored() {
        let file = tempfile::NamedTempFile::new().expect("temp file");
        write(
            file.path(),
            &Store {
                profiles: vec![profile(b"from-yesterday")],
                mpeg4_headers: Vec::new(),
            },
        )
        .expect("writes");

        // `save` folds the in-memory library in; with nothing learned it must still preserve.
        let kept = save(file.path()).expect("saves");
        assert!(kept >= 1);
        assert!(
            read(file.path())
                .profiles
                .iter()
                .any(|found| found.sequence_parameter_sets == vec![b"from-yesterday".to_vec()])
        );
    }

    #[test]
    fn the_store_path_prefers_what_the_environment_names() {
        let named = |value: &str| Some(OsString::from(value));

        assert_eq!(
            resolve(named("/tmp/elsewhere.json"), named("/cache"), named("/home/a")),
            Some(PathBuf::from("/tmp/elsewhere.json"))
        );
        assert_eq!(resolve(None, named("/cache"), named("/home/a")), Some(PathBuf::from("/cache/file_fixer/profiles.json")));
        assert_eq!(resolve(None, None, named("/home/a")), Some(PathBuf::from("/home/a/.cache/file_fixer/profiles.json")));
        assert_eq!(resolve(None, None, None), None, "nowhere to put it is not an error");
    }
}
