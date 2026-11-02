use crate::tools::disk_usage::layout::{LayoutItem, LayoutOptions, MeasuredChild, Omitted, Shape, measured_children, measured_composition};
use crate::tools::disk_usage::tree::DiskNode;

/// Only keeps a split away from the very edge of a rectangle, so no item gets a degenerate zero-sized one.
const MIN_SPLIT_SHARE: f64 = 1e-9;

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    fn area(self) -> f64 {
        self.width * self.height
    }

    fn shrink(self, padding: f64) -> Self {
        let padding = padding.min(self.width / 2.0).min(self.height / 2.0);
        Self {
            x: self.x + padding,
            y: self.y + padding,
            width: self.width - 2.0 * padding,
            height: self.height - 2.0 * padding,
        }
    }
}

enum Entry<'a> {
    Node(MeasuredChild<'a>),
    Other(Omitted),
}

impl Entry<'_> {
    fn size(&self) -> u64 {
        match self {
            Self::Node(child) => child.size,
            Self::Other(omitted) => omitted.size,
        }
    }
}

struct Builder<'a> {
    options: &'a LayoutOptions,
    items: Vec<LayoutItem>,
}

pub(super) fn layout(roots: &[DiskNode], options: &LayoutOptions) -> Vec<LayoutItem> {
    let mut builder = Builder { options, items: Vec::new() };
    // Laid out in a space as wide as the target area, so that squares stay squares once stretched back to the unit square
    let area = Rect {
        x: 0.0,
        y: 0.0,
        width: options.effective_aspect_ratio(),
        height: 1.0,
    };
    builder.place(roots, None, 0, area);
    builder.items
}

impl Builder<'_> {
    /// Returns `false` when nothing could be shown inside `area`, so the caller can draw the directory as a plain block.
    fn place(&mut self, siblings: &[DiskNode], parent: Option<u32>, depth: usize, area: Rect) -> bool {
        let measured = measured_children(siblings, self.options);
        let mut entries = measured.kept.into_iter().map(Entry::Node).collect::<Vec<_>>();
        if measured.omitted.count > 0 {
            entries.push(Entry::Other(measured.omitted));
        }
        let mut rects = arrange(&mut entries, area);

        if entries.iter().zip(&rects).any(|(entry, rect)| matches!(entry, Entry::Node(_)) && self.is_too_small(*rect)) {
            entries = self.merge_too_small_into_other(entries, &rects);
            rects = arrange(&mut entries, area);
        }
        if !entries.iter().any(|entry| matches!(entry, Entry::Node(_))) {
            return false;
        }

        for (entry, rect) in entries.into_iter().zip(rects) {
            if self.is_too_small(rect) {
                continue;
            }
            match entry {
                Entry::Node(child) => self.place_node(child, parent, depth, rect),
                Entry::Other(omitted) => self.items.push(LayoutItem::omitted_block(self.unit_shape(rect), depth, parent, omitted)),
            }
        }
        true
    }

    fn place_node(&mut self, child: MeasuredChild, parent: Option<u32>, depth: usize, rect: Rect) {
        let MeasuredChild { index, node, size } = child;
        let item_index = self.items.len() as u32;
        self.items.push(LayoutItem {
            shape: self.unit_shape(rect),
            depth: depth as u16,
            parent,
            index_in_parent: index,
            size,
            kind: node.kind,
            category: node.category,
            omitted_entries: 0,
            composition: None,
        });
        if !node.is_directory() {
            return;
        }
        let has_content = depth + 1 < self.options.max_depth
            && !self.is_too_small_to_open(rect)
            && self.place(&node.children, Some(item_index), depth + 1, rect.shrink(self.options.nested_padding));
        if !has_content && let Some(item) = self.items.get_mut(item_index as usize) {
            item.composition = measured_composition(node, self.options);
        }
    }

    /// Entries that came out too small to see are counted together with those that were already left out.
    fn merge_too_small_into_other<'a>(&self, entries: Vec<Entry<'a>>, rects: &[Rect]) -> Vec<Entry<'a>> {
        let mut omitted = Omitted::default();
        let mut shown = Vec::new();
        for (entry, rect) in entries.into_iter().zip(rects) {
            match entry {
                Entry::Node(child) if self.is_too_small(*rect) => omitted.add(child.size, 1),
                Entry::Node(child) => shown.push(Entry::Node(child)),
                Entry::Other(other) => omitted.add(other.size, other.count),
            }
        }
        if omitted.count > 0 {
            shown.push(Entry::Other(omitted));
        }
        shown
    }

    fn is_too_small_to_open(&self, rect: Rect) -> bool {
        self.pixel_side(rect) < f64::from(self.options.min_expanded_directory_px)
    }

    fn is_too_small(&self, rect: Rect) -> bool {
        self.pixel_side(rect) < f64::from(self.options.min_item_size_px)
    }

    /// The shorter side of the rectangle in pixels of the canvas.
    fn pixel_side(&self, rect: Rect) -> f64 {
        let (canvas_width, canvas_height) = self.options.canvas_size_px;
        let width_px = rect.width / self.options.effective_aspect_ratio() * f64::from(canvas_width);
        let height_px = rect.height * f64::from(canvas_height);
        width_px.min(height_px)
    }

    fn unit_shape(&self, rect: Rect) -> Shape {
        let width = self.options.effective_aspect_ratio();
        Shape::Rect {
            x: (rect.x / width) as f32,
            y: rect.y as f32,
            width: (rect.width / width) as f32,
            height: rect.height as f32,
        }
    }
}

/// Sorts the entries biggest first and gives each a rectangle, in that order.
fn arrange(entries: &mut [Entry], area: Rect) -> Vec<Rect> {
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.size()));
    let total = entries.iter().map(|entry| entry.size() as f64).sum::<f64>();
    if total <= 0.0 || area.area() <= 0.0 {
        return Vec::new();
    }
    let areas = entries.iter().map(|entry| area.area() * entry.size() as f64 / total).collect::<Vec<_>>();
    let mut rects = Vec::with_capacity(areas.len());
    split(&areas, area, &mut rects);
    rects
}

/// Binary split: cuts the rectangle across its longer side where the two halves of the list are the closest in total area,
/// which keeps every rectangle near a square, unlike filling rows one after another. `areas` must be sorted descending.
fn split(areas: &[f64], rect: Rect, rects: &mut Vec<Rect>) {
    match areas {
        [] => {}
        [_] => rects.push(rect),
        _ => {
            let total = areas.iter().sum::<f64>();
            let Some((first, rest)) = areas.split_at_checked(closest_to_half(areas, total)) else {
                return;
            };
            let share = (first.iter().sum::<f64>() / total).clamp(MIN_SPLIT_SHARE, 1.0 - MIN_SPLIT_SHARE);
            let (left, right) = if rect.width >= rect.height {
                let width = rect.width * share;
                (
                    Rect { width, ..rect },
                    Rect {
                        x: rect.x + width,
                        width: rect.width - width,
                        ..rect
                    },
                )
            } else {
                let height = rect.height * share;
                (
                    Rect { height, ..rect },
                    Rect {
                        y: rect.y + height,
                        height: rect.height - height,
                        ..rect
                    },
                )
            };
            split(first, left, rects);
            split(rest, right, rects);
        }
    }
}

/// Number of leading areas whose sum is the closest to half of `total`; at least one, and not all.
fn closest_to_half(areas: &[f64], total: f64) -> usize {
    let target = total / 2.0;
    let mut cumulative = 0.0;
    let mut best = (1, f64::INFINITY);
    for (count, area) in areas.iter().take(areas.len() - 1).enumerate() {
        cumulative += area;
        let difference = (cumulative - target).abs();
        if difference >= best.1 {
            break;
        }
        best = (count + 1, difference);
    }
    best.0
}
