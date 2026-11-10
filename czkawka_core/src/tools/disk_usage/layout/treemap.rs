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

/// Work left to do; kept on an explicit stack, so a deeply nested tree cannot overflow the call stack.
enum Task<'a> {
    Node {
        child: MeasuredChild<'a>,
        parent: Option<u32>,
        depth: usize,
        rect: Rect,
    },
    Other {
        omitted: Omitted,
        parent: Option<u32>,
        depth: usize,
        rect: Rect,
    },
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
    let mut stack = Vec::new();
    builder.schedule(roots, None, 0, area, &mut stack);
    while let Some(task) = stack.pop() {
        match task {
            Task::Node { child, parent, depth, rect } => builder.place_node(child, parent, depth, rect, &mut stack),
            Task::Other { omitted, parent, depth, rect } => builder.items.push(LayoutItem::omitted_block(builder.unit_shape(rect), depth, parent, omitted)),
        }
    }
    builder.items
}

impl<'a> Builder<'a> {
    /// Divides `area` between the siblings and queues them, the first one on top of the stack so that items come out
    /// parents first and every subtree in one piece. Returns `false` when nothing could be shown inside `area`,
    /// so the caller can draw the directory as a plain block.
    fn schedule(&self, siblings: &'a [DiskNode], parent: Option<u32>, depth: usize, area: Rect, stack: &mut Vec<Task<'a>>) -> bool {
        let measured = measured_children(siblings, self.options);
        let mut entries = measured.kept.into_iter().map(Entry::Node).collect::<Vec<_>>();
        if measured.omitted.count > 0 {
            entries.push(Entry::Other(measured.omitted));
        }
        let mut rects = arrange(&mut entries, area, &|rect| self.is_too_small(rect));

        if entries.iter().zip(&rects).any(|(entry, rect)| matches!(entry, Entry::Node(_)) && self.is_too_small(*rect)) {
            entries = self.merge_too_small_into_other(entries, &rects);
            rects = arrange(&mut entries, area, &|rect| self.is_too_small(rect));
        }
        if !entries.iter().any(|entry| matches!(entry, Entry::Node(_))) {
            return false;
        }

        let first_new_task = stack.len();
        for (entry, rect) in entries.into_iter().zip(rects) {
            if self.is_too_small(rect) {
                continue;
            }
            stack.push(match entry {
                Entry::Node(child) => Task::Node { child, parent, depth, rect },
                Entry::Other(omitted) => Task::Other { omitted, parent, depth, rect },
            });
        }
        if let Some(new_tasks) = stack.get_mut(first_new_task..) {
            new_tasks.reverse();
        }
        true
    }

    fn place_node(&mut self, child: MeasuredChild<'a>, parent: Option<u32>, depth: usize, rect: Rect, stack: &mut Vec<Task<'a>>) {
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
            && self.schedule(&node.children, Some(item_index), depth + 1, rect.shrink(self.options.nested_padding), stack);
        if !has_content && let Some(item) = self.items.get_mut(item_index as usize) {
            item.composition = measured_composition(node, self.options);
        }
    }

    /// Entries that came out too small to see are counted together with those that were already left out.
    fn merge_too_small_into_other(&self, entries: Vec<Entry<'a>>, rects: &[Rect]) -> Vec<Entry<'a>> {
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

/// Sorts the entries biggest first and gives each a rectangle, in that order. Entries inside a rectangle that is `negligible`
/// are not divided any further and get that whole rectangle, which is just as negligible as theirs would be, because they lie inside it.
fn arrange(entries: &mut [Entry], area: Rect, negligible: &dyn Fn(Rect) -> bool) -> Vec<Rect> {
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.size()));
    let total = entries.iter().map(|entry| entry.size() as f64).sum::<f64>();
    if total <= 0.0 || area.area() <= 0.0 {
        return Vec::new();
    }
    let areas = entries.iter().map(|entry| area.area() * entry.size() as f64 / total).collect::<Vec<_>>();
    let mut rects = Vec::with_capacity(areas.len());
    split_part(&areas, || areas.iter().sum(), area, negligible, &mut rects);
    rects
}

/// `total` is only added up when the rectangle is divided, so that a negligible one costs nothing but its entries' places in `rects`.
fn split_part(areas: &[f64], total: impl FnOnce() -> f64, rect: Rect, negligible: &dyn Fn(Rect) -> bool, rects: &mut Vec<Rect>) {
    if negligible(rect) {
        rects.extend(std::iter::repeat_n(rect, areas.len()));
    } else {
        split(areas, total(), rect, negligible, rects);
    }
}

/// Binary split: cuts the rectangle across its longer side where the two halves of the list are the closest in total area,
/// which keeps every rectangle near a square, unlike filling rows one after another. `areas` must be sorted descending
/// and `total` is their sum, added up from the first one, which is what the halves would be measured with anyway.
fn split(areas: &[f64], total: f64, rect: Rect, negligible: &dyn Fn(Rect) -> bool, rects: &mut Vec<Rect>) {
    match areas {
        [] => {}
        [_] => rects.push(rect),
        _ => {
            let (count, first_total) = closest_to_half(areas, total);
            let Some((first, rest)) = areas.split_at_checked(count) else {
                return;
            };
            let share = (first_total / total).clamp(MIN_SPLIT_SHARE, 1.0 - MIN_SPLIT_SHARE);
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
            split_part(first, || first_total, left, negligible, rects);
            split_part(rest, || rest.iter().sum(), right, negligible, rects);
        }
    }
}

/// Number of leading areas whose sum is the closest to half of `total`, with that sum; at least one, and not all.
fn closest_to_half(areas: &[f64], total: f64) -> (usize, f64) {
    let target = total / 2.0;
    let mut cumulative = 0.0;
    let mut best = (1, f64::INFINITY, 0.0);
    for (count, area) in areas.iter().take(areas.len() - 1).enumerate() {
        cumulative += area;
        let difference = (cumulative - target).abs();
        if difference >= best.1 {
            break;
        }
        best = (count + 1, difference, cumulative);
    }
    (best.0, best.2)
}

#[cfg(test)]
mod tests {
    use super::{Rect, split};

    /// The way the split was done before it passed the sums down: every call added the areas up again.
    fn reference_split(areas: &[f64], rect: Rect, rects: &mut Vec<Rect>) {
        match areas {
            [] => {}
            [_] => rects.push(rect),
            _ => {
                let total = areas.iter().sum::<f64>();
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
                let (first, rest) = areas.split_at(best.0);
                let share = (first.iter().sum::<f64>() / total).clamp(super::MIN_SPLIT_SHARE, 1.0 - super::MIN_SPLIT_SHARE);
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
                reference_split(first, left, rects);
                reference_split(rest, right, rects);
            }
        }
    }

    fn assert_same_rects(areas: &[f64]) {
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 1.7,
            height: 1.0,
        };
        let (mut expected, mut actual) = (Vec::new(), Vec::new());
        reference_split(areas, rect, &mut expected);
        split(areas, areas.iter().sum(), rect, &|_| false, &mut actual);
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.iter().zip(&actual) {
            assert_eq!(
                [expected.x.to_bits(), expected.y.to_bits(), expected.width.to_bits(), expected.height.to_bits()],
                [actual.x.to_bits(), actual.y.to_bits(), actual.width.to_bits(), actual.height.to_bits()],
                "areas {areas:?}"
            );
        }
    }

    #[test]
    fn passing_sums_down_gives_bit_for_bit_the_same_rectangles() {
        let mut seed = 99_u64;
        let mut next = move |limit: u64| {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            (seed >> 33) % limit + 1
        };
        for count in [2, 3, 5, 10, 37, 200] {
            let mut sizes = (0..count).map(|_| next(100_000) as f64 / 7.0).collect::<Vec<_>>();
            sizes.sort_by(|a, b| b.total_cmp(a));
            assert_same_rects(&sizes);
        }
        // Equal entries decide ties between the places to cut, and a long tail after a few big entries is the uneven case
        for count in [2, 3, 4, 9, 10, 100, 1001] {
            assert_same_rects(&vec![0.1; count]);
        }
        let zipf = (1..=500).map(|rank| 1.0 / f64::from(rank)).collect::<Vec<_>>();
        assert_same_rects(&zipf);
    }

    #[test]
    fn leaving_negligible_rectangles_undivided_changes_neither_what_is_left_out_nor_what_is_kept() {
        let rect = Rect {
            x: 0.0,
            y: 0.0,
            width: 1.5,
            height: 1.0,
        };
        let too_small = |rect: Rect| (rect.width / 1.5 * 300.0).min(rect.height * 200.0) < 1.0;
        let mut seed = 5_u64;
        let mut next = move |limit: u64| {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            (seed >> 33) % limit + 1
        };
        let mut lists = vec![vec![1.0; 5000], (1..=3000).map(|rank| 1.0 / f64::from(rank)).collect::<Vec<_>>()];
        for count in [3, 40, 700, 9000] {
            let mut sizes = (0..count).map(|_| (next(1_000_000) as f64).powi(2)).collect::<Vec<_>>();
            sizes.sort_by(|a, b| b.total_cmp(a));
            lists.push(sizes);
        }
        for sizes in lists {
            let total = sizes.iter().sum::<f64>();
            let areas = sizes.iter().map(|size| size / total).collect::<Vec<_>>();
            let (mut exact, mut pruned) = (Vec::new(), Vec::new());
            split(&areas, areas.iter().sum(), rect, &|_| false, &mut exact);
            split(&areas, areas.iter().sum(), rect, &too_small, &mut pruned);

            assert_eq!(exact.len(), pruned.len());
            assert!(exact.iter().any(|rect| !too_small(*rect)), "something should be left");
            assert!(
                sizes.len() < 700 || sizes.first() == sizes.last() || exact.iter().any(|rect| too_small(*rect)),
                "a long list of different entries should have some that are left out"
            );
            for (exact, pruned) in exact.iter().zip(&pruned) {
                assert_eq!(too_small(*exact), too_small(*pruned));
                if !too_small(*exact) {
                    assert_eq!(
                        [exact.x.to_bits(), exact.y.to_bits(), exact.width.to_bits(), exact.height.to_bits()],
                        [pruned.x.to_bits(), pruned.y.to_bits(), pruned.width.to_bits(), pruned.height.to_bits()]
                    );
                }
            }
        }
    }
}
