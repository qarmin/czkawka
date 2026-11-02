use std::f32::consts::TAU;

use crate::tools::disk_usage::layout::{LayoutItem, LayoutOptions, Measured, MeasuredChild, SUNBURST_CENTER_RADIUS, Shape, measured_children};
use crate::tools::disk_usage::tree::DiskNode;

/// Every level splits the span of its parent proportionally to the sizes of the children; level 0 is the centre, the rest are rings.
pub(super) fn layout(roots: &[DiskNode], options: &LayoutOptions) -> Vec<LayoutItem> {
    let levels = options.max_depth.max(1);
    let mut builder = Builder {
        options,
        levels,
        items: Vec::new(),
    };
    builder.place(roots, None, 0, 0.0, 1.0);
    builder.items
}

struct Builder<'a> {
    options: &'a LayoutOptions,
    levels: usize,
    items: Vec<LayoutItem>,
}

impl Builder<'_> {
    /// `span_start` and `span_end` are shares of the whole circle in `0.0..=1.0`.
    /// Returns `false` when nothing was shown, so the caller can draw the directory as a plain block.
    fn place(&mut self, siblings: &[DiskNode], parent: Option<u32>, depth: usize, span_start: f64, span_end: f64) -> bool {
        let Measured { kept, mut omitted } = measured_children(siblings, self.options);
        let total = kept.iter().map(|child| child.size as f64).sum::<f64>() + omitted.size as f64;
        let mut next_start = span_start;
        let mut placed_any = false;
        let mut kept = kept.into_iter();
        for MeasuredChild { index, node, size } in kept.by_ref() {
            let start = next_start;
            let end = start + (span_end - span_start) * size as f64 / total;
            if self.is_too_small(start, end) {
                // Children are sorted by size, so this one and all that follow join the omitted block
                omitted.add(size, 1);
                break;
            }
            next_start = end;
            placed_any = true;
            let item_index = self.items.len() as u32;
            self.items.push(LayoutItem {
                shape: self.shape(depth, start, end),
                depth: depth as u16,
                parent,
                index_in_parent: index,
                size,
                kind: node.kind,
                category: node.category,
                omitted_entries: 0,
                composition: None,
            });
            if node.is_directory() && depth + 1 < self.levels {
                self.place(&node.children, Some(item_index), depth + 1, start, end);
            }
        }
        kept.for_each(|child| omitted.add(child.size, 1));

        if placed_any && omitted.count > 0 && !self.is_too_small(next_start, span_end) {
            self.items.push(LayoutItem::omitted_block(self.shape(depth, next_start, span_end), depth, parent, omitted));
        }
        placed_any
    }

    fn is_too_small(&self, span_start: f64, span_end: f64) -> bool {
        (span_end - span_start) * f64::from(TAU) < f64::from(self.options.min_arc_angle)
    }

    /// The centre takes [`SUNBURST_CENTER_RADIUS`] and the rings share what is left evenly.
    fn shape(&self, depth: usize, span_start: f64, span_end: f64) -> Shape {
        let (inner_radius, outer_radius) = match (depth, self.levels) {
            (0, 1) => (0.0, 1.0),
            (0, _) => (0.0, SUNBURST_CENTER_RADIUS),
            _ => {
                let ring_thickness = (1.0 - SUNBURST_CENTER_RADIUS) / (self.levels - 1) as f32;
                (
                    SUNBURST_CENTER_RADIUS + (depth - 1) as f32 * ring_thickness,
                    SUNBURST_CENTER_RADIUS + depth as f32 * ring_thickness,
                )
            }
        };
        Shape::Arc {
            start_angle: span_start as f32 * TAU,
            end_angle: span_end as f32 * TAU,
            inner_radius,
            outer_radius,
        }
    }
}
