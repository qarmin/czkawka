use std::path::PathBuf;

use crate::tools::disk_usage::category::{CATEGORY_COUNT, FileCategory};

pub type CategorySizes = [u64; CATEGORY_COUNT];

/// Position of a node in a [`DiskTree`]: index of the root, then index of the child at every level.
pub type NodeAddress = [u32];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeKind {
    File,
    Directory,
    Symlink,
    /// Directory or file whose metadata could not be read; counted as zero bytes.
    Inaccessible,
}

#[derive(Debug, Clone)]
pub struct DiskNode {
    /// File name; for a root node the full path as given by the user.
    pub name: Box<str>,
    pub kind: NodeKind,
    /// Category of a file, or the dominant category by size for a directory.
    pub category: FileCategory,
    /// Size of a file, or the sum of all descendants for a directory.
    pub size: u64,
    /// Sorted by size, descending, then by name, when built; [`DiskTree::remove`] can leave ancestors out of order.
    pub children: Vec<Self>,
    /// Bytes per category among all descendants; present only for directories.
    pub category_sizes: Option<Box<CategorySizes>>,
}

impl DiskNode {
    pub fn file(name: impl Into<Box<str>>, size: u64, category: FileCategory) -> Self {
        Self {
            name: name.into(),
            kind: NodeKind::File,
            category,
            size,
            children: Vec::new(),
            category_sizes: None,
        }
    }

    pub fn symlink(name: impl Into<Box<str>>) -> Self {
        Self::leaf(name, NodeKind::Symlink)
    }

    pub fn inaccessible(name: impl Into<Box<str>>) -> Self {
        Self::leaf(name, NodeKind::Inaccessible)
    }

    fn leaf(name: impl Into<Box<str>>, kind: NodeKind) -> Self {
        Self {
            name: name.into(),
            kind,
            category: FileCategory::Other,
            size: 0,
            children: Vec::new(),
            category_sizes: None,
        }
    }

    pub fn directory(name: impl Into<Box<str>>, mut children: Vec<Self>) -> Self {
        children.sort_unstable_by(|left, right| right.size.cmp(&left.size).then_with(|| left.name.cmp(&right.name)));
        let mut category_sizes = Box::new([0; CATEGORY_COUNT]);
        let mut size = 0_u64;
        for child in &children {
            size = size.saturating_add(child.size);
            child.add_category_sizes_to(&mut category_sizes);
        }
        Self {
            name: name.into(),
            kind: NodeKind::Directory,
            category: dominant_category(&category_sizes),
            size,
            children,
            category_sizes: Some(category_sizes),
        }
    }

    pub fn is_directory(&self) -> bool {
        self.kind == NodeKind::Directory
    }

    fn add_category_sizes_to(&self, target: &mut CategorySizes) {
        match &self.category_sizes {
            Some(sizes) => target.iter_mut().zip(sizes.iter()).for_each(|(sum, size)| *sum = sum.saturating_add(*size)),
            None => add_to_category(target, self.category, self.size),
        }
    }

    fn subtract_removed(&mut self, removed: &Self) {
        self.size = self.size.saturating_sub(removed.size);
        if let Some(sizes) = self.category_sizes.as_mut() {
            match &removed.category_sizes {
                Some(removed_sizes) => sizes.iter_mut().zip(removed_sizes.iter()).for_each(|(sum, size)| *sum = sum.saturating_sub(*size)),
                None => subtract_from_category(sizes, removed.category, removed.size),
            }
            self.category = dominant_category(sizes);
        }
    }
}

fn add_to_category(sizes: &mut CategorySizes, category: FileCategory, size: u64) {
    if let Some(slot) = sizes.get_mut(category.index()) {
        *slot = slot.saturating_add(size);
    }
}

fn subtract_from_category(sizes: &mut CategorySizes, category: FileCategory, size: u64) {
    if let Some(slot) = sizes.get_mut(category.index()) {
        *slot = slot.saturating_sub(size);
    }
}

fn dominant_category(sizes: &CategorySizes) -> FileCategory {
    FileCategory::ALL
        .iter()
        .zip(sizes)
        .filter(|(category, _)| **category != FileCategory::Folder)
        .max_by_key(|(_, size)| **size)
        .map_or(FileCategory::Other, |(category, _)| *category)
}

#[derive(Debug, Clone, Default)]
pub struct DiskTree {
    /// Sorted by size, descending, then by name, when built.
    pub roots: Vec<DiskNode>,
}

impl DiskTree {
    pub fn new(mut roots: Vec<DiskNode>) -> Self {
        roots.sort_unstable_by(|left, right| right.size.cmp(&left.size).then_with(|| left.name.cmp(&right.name)));
        Self { roots }
    }

    pub fn total_size(&self) -> u64 {
        self.roots.iter().map(|root| root.size).sum()
    }

    pub fn node(&self, address: &NodeAddress) -> Option<&DiskNode> {
        let (root, rest) = address.split_first()?;
        rest.iter().try_fold(self.roots.get(*root as usize)?, |node, child| node.children.get(*child as usize))
    }

    /// Full filesystem path of the node, built from the names of its ancestors.
    pub fn path(&self, address: &NodeAddress) -> Option<PathBuf> {
        let (root, rest) = address.split_first()?;
        let mut node = self.roots.get(*root as usize)?;
        let mut path = PathBuf::from(&*node.name);
        for child in rest {
            node = node.children.get(*child as usize)?;
            path.push(&*node.name);
        }
        Some(path)
    }

    /// Detaches the node from the tree and subtracts its size from all ancestors, without touching the disk.
    /// Addresses of siblings that follow the removed node shift down by one.
    pub fn remove(&mut self, address: &NodeAddress) -> Option<DiskNode> {
        let (root, rest) = address.split_first()?;
        let root_index = *root as usize;
        if rest.is_empty() {
            return (root_index < self.roots.len()).then(|| self.roots.remove(root_index));
        }
        remove_below(self.roots.get_mut(root_index)?, rest)
    }
}

fn remove_below(parent: &mut DiskNode, rest: &NodeAddress) -> Option<DiskNode> {
    let (child_index, deeper) = rest.split_first()?;
    let child_index = *child_index as usize;
    let removed = if deeper.is_empty() {
        (child_index < parent.children.len()).then(|| parent.children.remove(child_index))?
    } else {
        remove_below(parent.children.get_mut(child_index)?, deeper)?
    };
    parent.subtract_removed(&removed);
    Some(removed)
}
