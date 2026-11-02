mod sunburst;
#[cfg(test)]
mod tests;
mod treemap;

use std::f32::consts::TAU;

use crate::tools::disk_usage::category::{CATEGORY_COUNT, FileCategory};
use crate::tools::disk_usage::tree::{CategorySizes, DiskNode, DiskTree, NodeAddress, NodeKind};

/// Share of the radius taken by the centre of the sunburst, where its root is shown.
pub const SUNBURST_CENTER_RADIUS: f32 = 0.38;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutKind {
    /// Every file and directory as nested rectangles; the most detailed and the slowest to draw.
    Treemap,
    /// A centre for the scanned directory and concentric rings around it, one per directory level.
    Sunburst,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutOptions {
    pub kind: LayoutKind,
    /// Directory levels to include (roots are level 1); the treemap honours it too.
    pub max_depth: usize,
    /// Width divided by height of the area the treemap will be drawn in; keeps its rectangles close to squares.
    /// Ignored when `canvas_size_px` is set.
    pub aspect_ratio: f32,
    /// Size of the area the layout will be drawn in; `(0.0, 0.0)` when unknown.
    pub canvas_size_px: (f32, f32),
    /// Rectangles with a side shorter than this many pixels are left out; needs `canvas_size_px`.
    pub min_item_size_px: f32,
    /// Arcs of the sunburst narrower than this angle in radians are left out.
    pub min_arc_angle: f32,
    /// Entries smaller than this share of the directory that holds them are grouped into one block of small entries.
    pub min_size_fraction: f64,
    /// A directory whose rectangle has a side shorter than this many pixels is drawn as one block without its content;
    /// `0.0` never does this. Needs `canvas_size_px`.
    pub min_expanded_directory_px: f32,
    /// Gap kept around the content of a directory in the treemap, as a share of the height of the area;
    /// `0.0` lets the content fill the directory exactly, so nested directories do not add borders.
    pub nested_padding: f64,
    /// Only the bytes of enabled categories are measured; indexed by [`FileCategory::index`].
    pub enabled_categories: [bool; CATEGORY_COUNT],
}

impl LayoutOptions {
    pub const UNLIMITED_DEPTH: usize = usize::MAX;

    pub fn new(kind: LayoutKind) -> Self {
        Self {
            kind,
            max_depth: match kind {
                LayoutKind::Treemap => Self::UNLIMITED_DEPTH,
                LayoutKind::Sunburst => 4,
            },
            aspect_ratio: 1.0,
            canvas_size_px: (0.0, 0.0),
            min_item_size_px: 0.0,
            min_arc_angle: 0.0,
            min_size_fraction: 0.000_001,
            min_expanded_directory_px: 0.0,
            nested_padding: 0.002,
            enabled_categories: [true; CATEGORY_COUNT],
        }
    }

    /// Aspect ratio of the drawing area, from its pixel size when known.
    pub(crate) fn effective_aspect_ratio(&self) -> f64 {
        let (width, height) = self.canvas_size_px;
        let aspect = if width > 0.0 && height > 0.0 { width / height } else { self.aspect_ratio };
        f64::from(aspect.max(0.01))
    }

    fn is_enabled(&self, category: FileCategory) -> bool {
        self.enabled_categories.get(category.index()).copied().unwrap_or(false)
    }

    fn all_categories_enabled(&self) -> bool {
        self.enabled_categories.iter().all(|enabled| *enabled)
    }

    /// Size of the node counting only enabled categories; `0` when nothing in it would be shown.
    pub fn measured_size(&self, node: &DiskNode) -> u64 {
        if self.all_categories_enabled() {
            return node.size;
        }
        match (&node.category_sizes, node.kind) {
            (Some(sizes), _) => sizes.iter().zip(&self.enabled_categories).filter(|(_, enabled)| **enabled).map(|(size, _)| *size).sum(),
            (None, NodeKind::File) if self.is_enabled(node.category) => node.size,
            (None, _) => 0,
        }
    }
}

/// Position of a layout item on a unit square.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    /// Origin in the top left corner, all values in `0.0..=1.0`.
    Rect { x: f32, y: f32, width: f32, height: f32 },
    /// Centred in the square; angles in radians clockwise from twelve o'clock, radii where `1.0` reaches the edge of the square.
    Arc {
        start_angle: f32,
        end_angle: f32,
        inner_radius: f32,
        outer_radius: f32,
    },
}

impl Shape {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        match *self {
            Self::Rect { x: left, y: top, width, height } => x >= left && x < left + width && y >= top && y < top + height,
            Self::Arc {
                start_angle,
                end_angle,
                inner_radius,
                outer_radius,
            } => {
                let (dx, dy) = (x - 0.5, y - 0.5);
                let radius = dx.hypot(dy) * 2.0;
                let angle = dx.atan2(-dy).rem_euclid(TAU);
                radius >= inner_radius && radius < outer_radius && angle >= start_angle && angle < end_angle
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutItem {
    pub shape: Shape,
    /// `0` for roots.
    pub depth: u16,
    /// Index of the parent in [`Layout::items`]; `None` for roots.
    pub parent: Option<u32>,
    /// Index of the node among its siblings in the [`DiskTree`], i.e. the last element of its address.
    pub index_in_parent: u32,
    /// Measured size, honouring the enabled categories.
    pub size: u64,
    pub kind: NodeKind,
    pub category: FileCategory,
    /// Non-zero for the block that stands for entries too small to be shown on their own; it has no node in the tree.
    pub omitted_entries: u32,
    /// Measured bytes per category, only for a directory that is drawn with nothing inside it.
    pub composition: Option<Box<CategorySizes>>,
}

impl LayoutItem {
    pub const fn is_omitted_block(&self) -> bool {
        self.omitted_entries > 0
    }

    fn omitted_block(shape: Shape, depth: usize, parent: Option<u32>, omitted: Omitted) -> Self {
        Self {
            shape,
            depth: depth as u16,
            parent,
            index_in_parent: u32::MAX,
            size: omitted.size,
            kind: NodeKind::File,
            category: FileCategory::Other,
            omitted_entries: omitted.count,
            composition: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Layout {
    /// Parents always come before their children, so drawing in order puts children on top.
    pub items: Vec<LayoutItem>,
}

impl Layout {
    pub fn build(tree: &DiskTree, options: &LayoutOptions) -> Self {
        Self::build_from_roots(&tree.roots, options)
    }

    /// Lays out the given nodes as the roots, e.g. a single directory to zoom into; addresses are then relative to them.
    pub fn build_from_roots(roots: &[DiskNode], options: &LayoutOptions) -> Self {
        let items = match options.kind {
            LayoutKind::Treemap => treemap::layout(roots, options),
            LayoutKind::Sunburst => sunburst::layout(roots, options),
        };
        Self { items }
    }

    /// Topmost item containing the point, i.e. the deepest one.
    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        self.items.iter().rposition(|item| item.shape.contains(x, y))
    }

    /// Address of the item in the [`DiskTree`] it was built from; for a block of omitted entries, of the directory that holds them.
    pub fn address(&self, item_index: usize) -> Option<Vec<u32>> {
        let item = self.items.get(item_index)?;
        let mut address = Vec::new();
        let mut current = if item.is_omitted_block() {
            item.parent.map(|parent| parent as usize)
        } else {
            Some(item_index)
        };
        while let Some(index) = current {
            let item = self.items.get(index)?;
            address.push(item.index_in_parent);
            current = item.parent.map(|parent| parent as usize);
        }
        address.reverse();
        Some(address)
    }

    pub fn find_item(&self, address: &NodeAddress) -> Option<usize> {
        let (&root, rest) = address.split_first()?;
        let mut current = self
            .items
            .iter()
            .position(|item| item.parent.is_none() && !item.is_omitted_block() && item.index_in_parent == root)?;
        for &child in rest {
            let parent = u32::try_from(current).ok()?;
            current = self
                .items
                .iter()
                .position(|item| item.parent == Some(parent) && !item.is_omitted_block() && item.index_in_parent == child)?;
        }
        Some(current)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MeasuredChild<'a> {
    pub index: u32,
    pub node: &'a DiskNode,
    pub size: u64,
}

/// Entries that are too small to be shown one by one, counted as a whole.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Omitted {
    pub size: u64,
    pub count: u32,
}

impl Omitted {
    pub(crate) fn add(&mut self, size: u64, count: u32) {
        self.size = self.size.saturating_add(size);
        self.count = self.count.saturating_add(count);
    }
}

pub(crate) struct Measured<'a> {
    /// Biggest first; equal sizes are ordered by name so that a rescan gives the same layout.
    pub kept: Vec<MeasuredChild<'a>>,
    pub omitted: Omitted,
}

/// Splits the children with a non-zero measured size into those that are big enough compared to their siblings and the rest.
pub(crate) fn measured_children<'a>(children: &'a [DiskNode], options: &LayoutOptions) -> Measured<'a> {
    let sizes = children.iter().map(|node| options.measured_size(node)).collect::<Vec<_>>();
    let min_size = (sizes.iter().sum::<u64>() as f64 * options.min_size_fraction) as u64;
    let mut kept = Vec::new();
    let mut omitted = Omitted::default();
    for ((index, node), size) in children.iter().enumerate().zip(sizes) {
        if size == 0 {
            continue;
        }
        if size >= min_size {
            kept.push(MeasuredChild { index: index as u32, node, size });
        } else {
            omitted.add(size, 1);
        }
    }
    kept.sort_unstable_by(|left, right| right.size.cmp(&left.size).then_with(|| left.node.name.cmp(&right.node.name)));
    Measured { kept, omitted }
}

/// Measured bytes per category of a directory; `None` for anything else.
pub(crate) fn measured_composition(node: &DiskNode, options: &LayoutOptions) -> Option<Box<CategorySizes>> {
    let sizes = node.category_sizes.as_ref()?;
    let mut composition = Box::new(**sizes);
    composition.iter_mut().zip(&options.enabled_categories).for_each(|(size, enabled)| {
        if !enabled {
            *size = 0;
        }
    });
    Some(composition)
}
