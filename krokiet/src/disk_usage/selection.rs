use std::f32::consts::TAU;
use std::sync::Arc;

use czkawka_core::tools::disk_usage::layout::{Layout, LayoutKind, Shape};
use slint::ComponentHandle;

use crate::disk_usage::render::ChartGeometry;
use crate::disk_usage::view::{ChartContext, RenderedChart};
use crate::{DiskUsageSelection, DiskUsageState, MainWindow};

/// Outline of an arc lies this far inside its edge, in pixels, to keep it clear of the gap between neighbours.
const ARC_INSET_PX: f32 = 1.25;
const DIRECTORY_BLOCK_BORDER_PX: f32 = 1.0;
const FRAME_BORDER_PX: f32 = 2.0;

/// Moves the outline of the selected item over the chart; the picture itself does not change with the selection.
pub(super) fn update_overlay(app: &MainWindow, context: &Arc<ChartContext>) {
    let selected = context.view.lock().expect("View mutex poisoned").selected.clone();
    let selection = {
        let chart = context.chart.lock().expect("Chart mutex poisoned");
        selected.zip(chart.as_ref()).and_then(|(address, chart)| overlay_for(chart, &address)).unwrap_or_default()
    };
    app.global::<DiskUsageState>().set_selection(selection);
}

fn overlay_for(chart: &RenderedChart, address: &[u32]) -> Option<DiskUsageSelection> {
    let index = find_item_in_view(&chart.layout, &chart.zoom, address)?;
    let item = chart.layout.items.get(index)?;
    let geometry = chart.geometry;
    match item.shape {
        Shape::Rect { x, y, width, height } => {
            let (w, h) = (geometry.width as f32, geometry.height as f32);
            let (left, top) = ((x * w).round(), (y * h).round());
            let (right, bottom) = (((x + width) * w).round(), ((y + height) * h).round());
            (right > left && bottom > top).then(|| DiskUsageSelection {
                visible: true,
                is_arc: false,
                x: left,
                y: top,
                width: right - left,
                height: bottom - top,
                border: if item.composition.is_some() { DIRECTORY_BLOCK_BORDER_PX } else { FRAME_BORDER_PX },
                path: Default::default(),
            })
        }
        Shape::Arc {
            start_angle,
            end_angle,
            inner_radius,
            outer_radius,
        } => (item.depth > 0)
            .then(|| arc_overlay(geometry, start_angle, end_angle, inner_radius, outer_radius))
            .flatten(),
    }
}

fn arc_overlay(geometry: ChartGeometry, start: f32, end: f32, inner: f32, outer: f32) -> Option<DiskUsageSelection> {
    debug_assert_eq!(geometry.kind, LayoutKind::Sunburst);
    let (left, top, side, _) = geometry.drawing_area();
    let radius_px = side as f32 / 2.0;
    let (center_x, center_y) = (left as f32 + radius_px, top as f32 + radius_px);
    let (inner_r, outer_r) = (inner * radius_px + ARC_INSET_PX, outer * radius_px - ARC_INSET_PX);
    if outer_r <= inner_r {
        return None;
    }
    let inner_span = (start + ARC_INSET_PX / inner_r, end - ARC_INSET_PX / inner_r);
    let outer_span = (start + ARC_INSET_PX / outer_r, end - ARC_INSET_PX / outer_r);
    if inner_span.1 <= inner_span.0 || outer_span.1 <= outer_span.0 {
        return None;
    }
    let point = |radius: f32, angle: f32| (center_x + radius * angle.sin(), center_y - radius * angle.cos());
    let large = i32::from(outer_span.1 - outer_span.0 > TAU / 2.0);
    let (outer_start, outer_end) = (point(outer_r, outer_span.0), point(outer_r, outer_span.1));
    let (inner_start, inner_end) = (point(inner_r, inner_span.0), point(inner_r, inner_span.1));
    let path = format!(
        "M {} {} A {outer_r} {outer_r} 0 {large} 1 {} {} L {} {} A {inner_r} {inner_r} 0 {large} 0 {} {} Z",
        outer_start.0, outer_start.1, outer_end.0, outer_end.1, inner_end.0, inner_end.1, inner_start.0, inner_start.1,
    );
    Some(DiskUsageSelection {
        visible: true,
        is_arc: true,
        x: 0.0,
        y: 0.0,
        width: geometry.width as f32,
        height: geometry.height as f32,
        border: 1.5,
        path: path.into(),
    })
}

/// Index in the layout of the item with `address` in the full tree; a zoomed layout starts at the zoomed directory.
fn find_item_in_view(layout: &Layout, zoom: &[u32], address: &[u32]) -> Option<usize> {
    if zoom.is_empty() {
        return layout.find_item(address);
    }
    let inside_zoom = address.strip_prefix(zoom)?;
    let relative = std::iter::once(0).chain(inside_zoom.iter().copied()).collect::<Vec<_>>();
    layout.find_item(&relative)
}
