use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::tools::folder_compare::hashing::{ContentHash, hash_text};
use crate::tools::folder_compare::scan::FsNode;
use crate::tools::folder_compare::{EntryKind, FolderCompareParameters, FolderGroup, FolderMatch, GroupRelation, MatchRole};

type Key = u128;

pub(super) struct DirInfo {
    pub path: PathBuf,
    pub parent: Option<usize>,
    pub hash: ContentHash,
    pub file_count: u64,
    pub size: u64,
    /// Every file below shares its size and content with another scanned file.
    pub all_shared: bool,
    /// Sorted, deduplicated identities of the shared files and symlinks below, relative to this folder.
    pub keys: Vec<Key>,
}

/// Files whose size occurs more than once; the only ones that can have a duplicate.
pub(super) fn collect_candidate_files(roots: &[(PathBuf, FsNode)]) -> Vec<(PathBuf, u64)> {
    let mut size_counts: HashMap<u64, u32> = HashMap::new();
    for (_, node) in roots {
        count_sizes(node, &mut size_counts);
    }
    let mut candidates = Vec::new();
    for (path, node) in roots {
        gather_candidates(node, path, &size_counts, &mut candidates);
    }
    candidates
}

fn count_sizes(node: &FsNode, size_counts: &mut HashMap<u64, u32>) {
    for child in &node.children {
        match child.kind {
            EntryKind::File => *size_counts.entry(child.size).or_default() += 1,
            EntryKind::Directory => count_sizes(child, size_counts),
            EntryKind::Symlink => {}
        }
    }
}

fn gather_candidates(node: &FsNode, path: &Path, size_counts: &HashMap<u64, u32>, candidates: &mut Vec<(PathBuf, u64)>) {
    for child in &node.children {
        match child.kind {
            EntryKind::File if size_counts.get(&child.size).is_some_and(|count| *count > 1) => candidates.push((path.join(&*child.name), child.size)),
            EntryKind::Directory => gather_candidates(child, &path.join(&*child.name), size_counts, candidates),
            EntryKind::File | EntryKind::Symlink => {}
        }
    }
}

pub(super) struct DirIndexBuilder<'a> {
    shared_hashes: &'a HashMap<PathBuf, ContentHash>,
    ignore_names: bool,
    dirs: Vec<DirInfo>,
}

impl<'a> DirIndexBuilder<'a> {
    pub fn new(shared_hashes: &'a HashMap<PathBuf, ContentHash>, ignore_names: bool) -> Self {
        Self {
            shared_hashes,
            ignore_names,
            dirs: Vec::new(),
        }
    }

    pub fn build(mut self, roots: &[(PathBuf, FsNode)]) -> Vec<DirInfo> {
        for (path, node) in roots {
            self.add_directory(node, path, None);
        }
        self.dirs
    }

    fn add_directory(&mut self, node: &FsNode, path: &Path, parent: Option<usize>) -> usize {
        let index = self.dirs.len();
        self.dirs.push(DirInfo {
            path: path.to_path_buf(),
            parent,
            hash: ContentHash::default(),
            file_count: node.file_count,
            size: node.size,
            all_shared: true,
            keys: Vec::new(),
        });

        let mut digests: Vec<(u8, &str, ContentHash)> = Vec::with_capacity(node.children.len());
        let mut keys = Vec::new();
        let mut all_shared = true;
        for child in &node.children {
            let child_path = path.join(&*child.name);
            match child.kind {
                EntryKind::Directory => {
                    let child_index = self.add_directory(child, &child_path, Some(index));
                    let Some(child_dir) = self.dirs.get(child_index) else { continue };
                    digests.push((2, &child.name, child_dir.hash));
                    all_shared &= child_dir.all_shared;
                    keys.extend(child_dir.keys.iter().map(|key| self.nested_key(&child.name, *key)));
                }
                EntryKind::File => {
                    let hash = self.shared_hashes.get(&child_path).copied();
                    all_shared &= hash.is_some();
                    let hash = hash.unwrap_or_else(|| unique_hash(&child_path, child.size));
                    digests.push((0, &child.name, hash));
                    keys.extend(self.shared_hashes.contains_key(&child_path).then(|| self.nested_key(&child.name, truncate(&hash))));
                }
                EntryKind::Symlink => {
                    let hash = symlink_hash(&child_path);
                    digests.push((1, &child.name, hash));
                    keys.push(self.nested_key(&child.name, truncate(&hash)));
                }
            }
        }
        keys.sort_unstable();
        keys.dedup();

        let hash = self.folder_hash(&mut digests);
        if let Some(dir) = self.dirs.get_mut(index) {
            dir.hash = hash;
            dir.all_shared = all_shared;
            dir.keys = keys;
        }
        index
    }

    fn folder_hash(&self, digests: &mut [(u8, &str, ContentHash)]) -> ContentHash {
        if self.ignore_names {
            digests.sort_unstable_by_key(|left| (left.0, left.2));
        }
        let mut hasher = blake3::Hasher::new();
        for (tag, name, hash) in digests.iter() {
            hasher.update(&[*tag]);
            if !self.ignore_names {
                hasher.update(&(name.len() as u64).to_le_bytes());
                hasher.update(name.as_bytes());
            }
            hasher.update(hash);
        }
        *hasher.finalize().as_bytes()
    }

    fn nested_key(&self, name: &str, key: Key) -> Key {
        if self.ignore_names {
            return key;
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(name.as_bytes());
        hasher.update(&key.to_le_bytes());
        truncate(hasher.finalize().as_bytes())
    }
}

fn truncate(hash: &[u8; 32]) -> Key {
    let mut bytes = [0; 16];
    bytes.copy_from_slice(hash.first_chunk::<16>().unwrap_or(&[0; 16]));
    Key::from_le_bytes(bytes)
}

fn unique_hash(path: &Path, size: u64) -> ContentHash {
    hash_text(&format!("unique:{}:{size}", path.to_string_lossy()))
}

fn symlink_hash(path: &Path) -> ContentHash {
    let target = std::fs::read_link(path).map(|target| target.to_string_lossy().into_owned()).unwrap_or_default();
    hash_text(&format!("symlink:{target}"))
}

pub(super) fn find_groups(dirs: &[DirInfo], params: &FolderCompareParameters) -> Vec<FolderGroup> {
    let finder = GroupFinder { dirs, params };
    let identical = finder.identical_classes();
    let mut groups = finder.identical_groups(&identical);
    if params.find_subsets {
        groups.extend(finder.subset_groups());
    }
    groups.sort_by(|left, right| {
        right
            .wasted_size()
            .cmp(&left.wasted_size())
            .then_with(|| left.folders.first().map(|f| &f.path).cmp(&right.folders.first().map(|f| &f.path)))
    });
    groups
}

struct GroupFinder<'a> {
    dirs: &'a [DirInfo],
    params: &'a FolderCompareParameters,
}

impl GroupFinder<'_> {
    fn dir(&self, index: usize) -> Option<&DirInfo> {
        self.dirs.get(index)
    }

    fn is_eligible(&self, dir: &DirInfo) -> bool {
        dir.file_count >= self.params.min_files.max(1) && dir.size >= self.params.min_size
    }

    /// Folders with equal content hashes, only classes of at least two folders.
    fn identical_classes(&self) -> Vec<Vec<usize>> {
        let mut classes: HashMap<ContentHash, Vec<usize>> = HashMap::new();
        for (index, dir) in self.dirs.iter().enumerate().filter(|(_, dir)| self.is_eligible(dir)) {
            classes.entry(dir.hash).or_default().push(index);
        }
        classes.into_values().filter(|class| class.len() > 1).collect()
    }

    fn identical_groups(&self, classes: &[Vec<usize>]) -> Vec<FolderGroup> {
        let class_of: HashMap<usize, usize> = classes.iter().enumerate().flat_map(|(id, class)| class.iter().map(move |index| (*index, id))).collect();
        classes
            .iter()
            .filter(|class| !self.is_nested_in_one_class(class, &class_of))
            .map(|class| FolderGroup {
                relation: GroupRelation::Identical,
                folders: self.matches(class, MatchRole::Identical, 0),
            })
            .collect()
    }

    /// A class whose members all sit inside the members of one other class only repeats that class.
    fn is_nested_in_one_class(&self, class: &[usize], class_of: &HashMap<usize, usize>) -> bool {
        let parent_classes: Vec<Option<usize>> = class
            .iter()
            .map(|index| self.dir(*index).and_then(|dir| dir.parent).and_then(|parent| class_of.get(&parent).copied()))
            .collect();
        parent_classes
            .first()
            .copied()
            .flatten()
            .is_some_and(|first| parent_classes.iter().all(|id| *id == Some(first)))
    }

    fn matches(&self, indices: &[usize], role: MatchRole, extra_files: u64) -> Vec<FolderMatch> {
        let mut matches: Vec<FolderMatch> = indices
            .iter()
            .filter_map(|index| self.dir(*index))
            .map(|dir| FolderMatch {
                path: dir.path.clone(),
                role,
                file_count: dir.file_count,
                size: dir.size,
                extra_files,
            })
            .collect();
        matches.sort_by(|left, right| left.path.cmp(&right.path));
        matches
    }

    fn is_ancestor(&self, ancestor: usize, descendant: usize) -> bool {
        let mut current = self.dir(descendant).and_then(|dir| dir.parent);
        while let Some(index) = current {
            if index == ancestor {
                return true;
            }
            current = self.dir(index).and_then(|dir| dir.parent);
        }
        false
    }

    fn is_related(&self, first: usize, second: usize) -> bool {
        first == second || self.is_ancestor(first, second) || self.is_ancestor(second, first)
    }

    fn subset_groups(&self) -> Vec<FolderGroup> {
        let index = self.key_index();
        let mut contained_classes: HashMap<ContentHash, Vec<usize>> = HashMap::new();
        for (dir_index, dir) in self
            .dirs
            .iter()
            .enumerate()
            .filter(|(_, dir)| self.is_eligible(dir) && dir.all_shared && !dir.keys.is_empty())
        {
            contained_classes.entry(dir.hash).or_default().push(dir_index);
        }

        let mut found: HashMap<ContentHash, (Vec<usize>, Vec<usize>)> = HashMap::new();
        for class in contained_classes.into_values() {
            let containers = self.find_containers(&class, &index);
            if let (Some(first), false) = (class.first().and_then(|i| self.dir(*i)), containers.is_empty()) {
                found.insert(first.hash, (class, containers));
            }
        }
        let containers_of_class: HashMap<ContentHash, &Vec<usize>> = found.iter().map(|(hash, (_, containers))| (*hash, containers)).collect();
        found
            .values()
            .filter(|(class, containers)| !self.repeats_parent_group(class, containers, &containers_of_class))
            .map(|(class, containers)| self.subset_group(class, containers))
            .collect()
    }

    fn key_index(&self) -> HashMap<Key, Vec<usize>> {
        let mut index: HashMap<Key, Vec<usize>> = HashMap::new();
        for (dir_index, dir) in self.dirs.iter().enumerate() {
            for key in &dir.keys {
                index.entry(*key).or_default().push(dir_index);
            }
        }
        index
    }

    fn find_containers(&self, class: &[usize], index: &HashMap<Key, Vec<usize>>) -> Vec<usize> {
        let Some(representative) = class.first().and_then(|i| self.dir(*i)) else {
            return Vec::new();
        };
        let rarest_holders = representative.keys.iter().filter_map(|key| index.get(key)).min_by_key(|holders| holders.len());
        let mut containers: Vec<usize> = rarest_holders
            .into_iter()
            .flatten()
            .copied()
            .filter(|candidate| {
                self.dir(*candidate)
                    .is_some_and(|dir| dir.hash != representative.hash && dir.file_count > representative.file_count && is_sorted_superset(&dir.keys, &representative.keys))
            })
            .filter(|candidate| !class.iter().any(|member| self.is_related(*member, *candidate)))
            .collect();
        containers.sort_unstable();
        containers
    }

    /// When the parents of the contained folders are already contained in the parents of the containers, this group says nothing new.
    fn repeats_parent_group(&self, class: &[usize], containers: &[usize], containers_of_class: &HashMap<ContentHash, &Vec<usize>>) -> bool {
        class.iter().all(|member| {
            let parent_containers = self
                .dir(*member)
                .and_then(|dir| dir.parent)
                .and_then(|parent| self.dir(parent))
                .and_then(|parent| containers_of_class.get(&parent.hash));
            parent_containers.is_some_and(|parent_containers| {
                containers
                    .iter()
                    .all(|container| self.dir(*container).and_then(|dir| dir.parent).is_some_and(|parent| parent_containers.contains(&parent)))
            })
        })
    }

    fn subset_group(&self, class: &[usize], containers: &[usize]) -> FolderGroup {
        let contained_files = class.first().and_then(|i| self.dir(*i)).map_or(0, |dir| dir.file_count);
        let mut folders = self.matches(class, MatchRole::Contained, 0);
        let mut container_matches: Vec<FolderMatch> = containers
            .iter()
            .filter_map(|i| self.dir(*i))
            .map(|dir| FolderMatch {
                path: dir.path.clone(),
                role: MatchRole::Container,
                file_count: dir.file_count,
                size: dir.size,
                extra_files: dir.file_count.saturating_sub(contained_files),
            })
            .collect();
        container_matches.sort_by(|left, right| left.path.cmp(&right.path));
        folders.extend(container_matches);
        FolderGroup {
            relation: GroupRelation::Subset,
            folders,
        }
    }
}

fn is_sorted_superset(superset: &[Key], subset: &[Key]) -> bool {
    let mut remaining = superset.iter();
    subset.iter().all(|wanted| remaining.by_ref().any(|key| key == wanted))
}
