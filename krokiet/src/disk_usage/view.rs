use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use czkawka_core::tools::disk_usage::category::{CATEGORY_COUNT, FileCategory};
use czkawka_core::tools::disk_usage::layout::{Layout, LayoutKind, LayoutOptions};
use slint::{ComponentHandle, Image, Model, ModelRc, VecModel, Weak};

use crate::disk_usage::describe::{self, ItemInfo};
use crate::disk_usage::render::{ChartGeometry, RenderSettings, render_chart};
use crate::disk_usage::tree_view;
use crate::shared_models::SharedModels;
use crate::{DiskUsageCategory, DiskUsageMode, DiskUsageState, MainWindow, Settings, flk};

const RESIZE_DEBOUNCE: Duration = Duration::from_millis(40);
/// A wheel gesture produces many events; only one of them moves the view by a level.
const SCROLL_STEP_INTERVAL: Duration = Duration::from_millis(250);
const MIN_DEPTH: usize = 2;
const MAX_DEPTH: usize = 8;
/// Gap around the content of a directory when nested borders are on.
const NESTED_PADDING_PX: f64 = 2.0;
const MIN_ARC_ANGLE: f32 = 0.03;

#[derive(Default)]
pub(super) struct ViewState {
    pub size_px: (u32, u32),
    /// Address of the directory the chart is zoomed into; empty for the whole scan.
    pub zoom: Vec<u32>,
    pub selected: Option<Vec<u32>>,
    /// Directories opened in the tree list on the left of the rings.
    pub expanded: HashSet<Vec<u32>>,
    /// Address of the node shown in every row of the tree list.
    pub tree_addresses: Vec<Vec<u32>>,
    pub last_scroll_step: Option<Instant>,
    /// Last position of the pointer over the chart, to describe the item under it again after the chart is redrawn.
    pub last_hover: Option<(f32, f32)>,
}

pub(super) struct RenderedChart {
    layout: Arc<Layout>,
    geometry: ChartGeometry,
    zoom: Vec<u32>,
    total_size: u64,
}

pub(super) struct ChartContext {
    pub shared_models: Arc<Mutex<SharedModels>>,
    pub view: Mutex<ViewState>,
    pub chart: Mutex<Option<RenderedChart>>,
    pub generation: AtomicU64,
    /// Changes whenever the tree itself does (a new scan, a removed item), so a layout built before is not reused.
    pub tree_version: AtomicU64,
    pub requests: Sender<RenderRequest>,
}

/// What a layout depends on; while it stays the same, a new selection only needs the picture drawn again.
#[derive(PartialEq)]
struct LayoutKey {
    tree_version: u64,
    zoom: Vec<u32>,
    options: LayoutOptions,
}

pub(super) struct RenderRequest {
    generation: u64,
    tree_version: u64,
    zoom: Vec<u32>,
    selected: Option<Vec<u32>>,
    geometry: ChartGeometry,
    options: LayoutOptions,
    dark_theme: bool,
    nested_borders: bool,
}

pub(super) struct HitItem {
    /// Of the item, or of the directory holding it when the item is a block of omitted entries.
    pub address: Vec<u32>,
    size: u64,
    /// Number of entries when the item stands for entries too small to be drawn one by one.
    omitted_entries: Option<u32>,
}

pub(super) fn spawn_render_worker(receiver: Receiver<RenderRequest>, context: Arc<ChartContext>, weak: Weak<MainWindow>) {
    thread::Builder::new()
        .name("disk_usage_render".into())
        .spawn(move || {
            let mut cache: Option<(LayoutKey, Arc<Layout>)> = None;
            while let Ok(mut request) = receiver.recv() {
                // Resizing sends a request per step; a change of selection reuses the layout and is shown without waiting
                if cache.as_ref().is_none_or(|(key, _)| *key != layout_key(&request)) {
                    thread::sleep(RESIZE_DEBOUNCE);
                }
                while let Ok(newer) = receiver.try_recv() {
                    request = newer;
                }
                if let Some((chart, buffer)) = build_and_render(&context, &request, &mut cache) {
                    publish(&weak, &context, request.generation, chart, buffer);
                }
            }
        })
        .expect("Cannot start disk usage render thread - not much we can do here");
}

fn is_current(context: &ChartContext, generation: u64) -> bool {
    context.generation.load(Ordering::Relaxed) == generation
}

fn layout_key(request: &RenderRequest) -> LayoutKey {
    LayoutKey {
        tree_version: request.tree_version,
        zoom: request.zoom.clone(),
        options: request.options,
    }
}

fn build_layout(context: &ChartContext, request: &RenderRequest) -> Option<Layout> {
    let models = context.shared_models.lock().expect("Mutex poisoned");
    let tree = models.shared_disk_usage_state.as_ref()?.get_tree();
    Some(if request.zoom.is_empty() {
        Layout::build_from_roots(&tree.roots, &request.options)
    } else {
        Layout::build_from_roots(std::slice::from_ref(tree.node(&request.zoom)?), &request.options)
    })
}

fn build_and_render(
    context: &ChartContext,
    request: &RenderRequest,
    cache: &mut Option<(LayoutKey, Arc<Layout>)>,
) -> Option<(RenderedChart, slint::SharedPixelBuffer<slint::Rgba8Pixel>)> {
    let key = layout_key(request);
    let layout = match cache.as_ref() {
        Some((cached_key, layout)) if *cached_key == key => Arc::clone(layout),
        _ => {
            let layout = Arc::new(build_layout(context, request)?);
            *cache = Some((key, Arc::clone(&layout)));
            layout
        }
    };
    if !is_current(context, request.generation) {
        return None;
    }

    let selected = request.selected.as_deref().and_then(|address| find_item_in_view(&layout, &request.zoom, address));
    let settings = RenderSettings {
        geometry: request.geometry,
        levels: request.options.max_depth,
        selected,
        dark_theme: request.dark_theme,
        nested_borders: request.nested_borders,
    };
    let buffer = render_chart(&layout, &settings, &|| is_current(context, request.generation))?;
    let total_size = layout.items.iter().filter(|item| item.parent.is_none()).map(|item| item.size).sum();
    let chart = RenderedChart {
        layout,
        geometry: request.geometry,
        zoom: request.zoom.clone(),
        total_size,
    };
    Some((chart, buffer))
}

fn publish(weak: &Weak<MainWindow>, context: &Arc<ChartContext>, generation: u64, chart: RenderedChart, buffer: slint::SharedPixelBuffer<slint::Rgba8Pixel>) {
    let context = Arc::clone(context);
    let delivered = weak.upgrade_in_event_loop(move |app| {
        if !is_current(&context, generation) {
            return;
        }
        let state = app.global::<DiskUsageState>();
        state.set_chart_image(Image::from_rgba8(buffer));
        state.set_rendering(false);
        state.set_center_size(describe::format_total(chart.total_size).into());
        let title = context
            .shared_models
            .lock()
            .expect("Mutex poisoned")
            .shared_disk_usage_state
            .as_ref()
            .map(|tool| describe::root_label(tool.get_tree(), &chart.zoom));
        state.set_center_title(title.unwrap_or_default().into());
        *context.chart.lock().expect("Chart mutex poisoned") = Some(chart);
        // The pointer may be resting over a different item now, e.g. after zooming with the wheel
        let last_hover = context.view.lock().expect("View mutex poisoned").last_hover;
        if let Some((x, y)) = last_hover {
            show_hovered_item(&app, &context, x, y);
        }
    });
    if delivered.is_err() {
        log::debug!("Main window is gone, dropping rendered disk usage chart");
    }
}

/// Address of the item in the full tree, given its address inside a layout that may start at the zoomed directory.
fn absolute_address(zoom: &[u32], relative: &[u32]) -> Vec<u32> {
    if zoom.is_empty() {
        relative.to_vec()
    } else {
        zoom.iter().chain(relative.iter().skip(1)).copied().collect()
    }
}

fn find_item_in_view(layout: &Layout, zoom: &[u32], address: &[u32]) -> Option<usize> {
    if zoom.is_empty() {
        return layout.find_item(address);
    }
    let inside_zoom = address.strip_prefix(zoom)?;
    let relative = std::iter::once(0).chain(inside_zoom.iter().copied()).collect::<Vec<_>>();
    layout.find_item(&relative)
}

fn layout_options(app: &MainWindow, size_px: (u32, u32)) -> LayoutOptions {
    let state = app.global::<DiskUsageState>();
    let kind = match state.get_mode() {
        DiskUsageMode::Treemap => LayoutKind::Treemap,
        DiskUsageMode::Sunburst => LayoutKind::Sunburst,
    };
    let mut options = LayoutOptions::new(kind);
    options.min_size_fraction = 10_f64.powf(-f64::from(state.get_group_exponent().round()));
    options.min_item_size_px = state.get_min_item_px().round();
    options.min_expanded_directory_px = state.get_folder_block_px().round();
    if kind == LayoutKind::Treemap {
        options.aspect_ratio = size_px.0 as f32 / size_px.1.max(1) as f32;
        options.nested_padding = if state.get_nested_borders() { NESTED_PADDING_PX / size_px.1.max(1) as f64 } else { 0.0 };
    } else {
        // Level 0 is the centre, so the rings come on top of it
        options.max_depth = (state.get_depth().round() as usize).clamp(MIN_DEPTH, MAX_DEPTH) + 1;
    }
    for (slot, category) in options.enabled_categories.iter_mut().skip(1).zip(state.get_categories().iter()) {
        *slot = category.enabled;
    }
    options.canvas_size_px = (size_px.0 as f32, size_px.1 as f32);
    options.min_arc_angle = MIN_ARC_ANGLE;
    options
}

pub(super) fn request_render(app: &MainWindow, context: &Arc<ChartContext>) {
    let state = app.global::<DiskUsageState>();
    if !state.get_has_results() {
        return;
    }
    let (size_px, zoom, selected) = {
        let view = context.view.lock().expect("View mutex poisoned");
        (view.size_px, view.zoom.clone(), view.selected.clone())
    };
    if size_px.0 == 0 || size_px.1 == 0 {
        return;
    }
    let options = layout_options(app, size_px);
    let request = RenderRequest {
        generation: context.generation.fetch_add(1, Ordering::Relaxed) + 1,
        tree_version: context.tree_version.load(Ordering::Relaxed),
        zoom,
        selected,
        geometry: ChartGeometry {
            width: size_px.0,
            height: size_px.1,
            kind: options.kind,
        },
        options,
        dark_theme: app.global::<Settings>().get_dark_theme(),
        nested_borders: state.get_nested_borders(),
    };
    state.set_rendering(true);
    if context.requests.send(request).is_err() {
        log::error!("Disk usage render thread is not running");
    }
}

pub(super) fn reset_view(app: &MainWindow, context: &Arc<ChartContext>) {
    context.tree_version.fetch_add(1, Ordering::Relaxed);
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        view.zoom.clear();
        view.selected = None;
    }
    refresh_after_zoom(app, context);
}

/// Brings the labels, the tree list and the chart in line with the zoom stored in the view.
pub(super) fn refresh_after_zoom(app: &MainWindow, context: &Arc<ChartContext>) {
    update_zoom_state(app, context);
    tree_view::reset_expanded(context);
    tree_view::rebuild(app, context);
    request_render(app, context);
}

fn hit_item(context: &ChartContext, x: f32, y: f32) -> Option<HitItem> {
    let chart = context.chart.lock().expect("Chart mutex poisoned");
    let chart = chart.as_ref()?;
    let (unit_x, unit_y) = chart.geometry.to_unit(x, y)?;
    let item_index = chart.layout.hit_test(unit_x, unit_y)?;
    let item = chart.layout.items.get(item_index)?;
    Some(HitItem {
        address: absolute_address(&chart.zoom, &chart.layout.address(item_index)?),
        size: item.size,
        omitted_entries: item.is_omitted_block().then_some(item.omitted_entries),
    })
}

fn hit_info(context: &ChartContext, hit: &HitItem) -> Option<ItemInfo> {
    let total = context.chart.lock().expect("Chart mutex poisoned").as_ref().map_or(0, |chart| chart.total_size);
    let models = context.shared_models.lock().expect("Mutex poisoned");
    let tree = models.shared_disk_usage_state.as_ref()?.get_tree();
    match hit.omitted_entries {
        Some(count) => describe::omitted_info(tree, &hit.address, count, hit.size, total),
        None => describe::node_info(tree, &hit.address, hit.size, total),
    }
}

pub(super) fn show_hovered_item(app: &MainWindow, context: &Arc<ChartContext>, x: f32, y: f32) {
    context.view.lock().expect("View mutex poisoned").last_hover = Some((x, y));
    let info = hit_item(context, x, y).and_then(|hit| hit_info(context, &hit));
    describe::show_hover(app, info.as_ref());
}

/// Description of a node picked in the tree list, as a share of the whole scan.
pub(super) fn describe_address(context: &ChartContext, address: &[u32]) -> Option<ItemInfo> {
    let models = context.shared_models.lock().expect("Mutex poisoned");
    let tree = models.shared_disk_usage_state.as_ref()?.get_tree();
    describe::node_info(tree, address, tree.node(address)?.size, tree.total_size())
}

pub(super) fn select_item_at(app: &MainWindow, context: &Arc<ChartContext>, x: f32, y: f32) {
    let hit = hit_item(context, x, y);
    context.view.lock().expect("View mutex poisoned").selected = hit.as_ref().map(|hit| hit.address.clone());
    // A block of small entries selects the directory that holds them, so it is that directory the menu and the status line describe
    let info = hit.as_ref().and_then(|hit| {
        if hit.omitted_entries.is_some() {
            describe_address(context, &hit.address)
        } else {
            hit_info(context, hit)
        }
    });
    describe::show_selection(app, info.as_ref());
    tree_view::expand_ancestors_of_selection(context);
    tree_view::rebuild(app, context);
    request_render(app, context);
}

/// Address of the item under the cursor, or of the directory that holds a block of omitted entries.
pub(super) fn address_at(context: &ChartContext, x: f32, y: f32) -> Option<Vec<u32>> {
    hit_item(context, x, y).map(|hit| hit.address)
}

/// Moves the view by one level: towards the selected item when scrolling up, back out when scrolling down.
pub(super) fn scroll_step(app: &MainWindow, context: &Arc<ChartContext>, towards_selection: bool) {
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        let now = Instant::now();
        if view.last_scroll_step.is_some_and(|last| now.duration_since(last) < SCROLL_STEP_INTERVAL) {
            return;
        }
        view.last_scroll_step = Some(now);
    }
    if towards_selection {
        zoom_towards_selection(app, context);
    } else {
        zoom_out(app, context);
    }
}

fn zoom_towards_selection(app: &MainWindow, context: &Arc<ChartContext>) {
    let Some(selected) = selected_address(context) else {
        return;
    };
    let zoom = context.view.lock().expect("View mutex poisoned").zoom.clone();
    let (selected_is_directory, single_root) = {
        let models = context.shared_models.lock().expect("Mutex poisoned");
        let Some(tool) = models.shared_disk_usage_state.as_ref() else {
            return;
        };
        let Some(node) = tool.get_tree().node(&selected) else {
            return;
        };
        (node.is_directory(), tool.get_tree().roots.len() == 1)
    };
    if let Some(target) = next_zoom_towards(&zoom, &selected, selected_is_directory, single_root) {
        context.view.lock().expect("View mutex poisoned").zoom = target;
        refresh_after_zoom(app, context);
    }
}

/// The zoom one level deeper than `zoom` on the way to `selected`; `None` when it is already as deep as it can be.
fn next_zoom_towards(zoom: &[u32], selected: &[u32], selected_is_directory: bool, single_root: bool) -> Option<Vec<u32>> {
    // With a single root, zooming into it changes nothing visible, so that level is skipped
    let current = if zoom.is_empty() && single_root { &[0][..] } else { zoom };
    if !selected.starts_with(current) {
        return None;
    }
    let deepest_directory = if selected_is_directory { selected.len() } else { selected.len().checked_sub(1)? };
    let target_length = (current.len() + 1).min(deepest_directory);
    (target_length > current.len()).then(|| selected.get(..target_length).map(<[u32]>::to_vec)).flatten()
}

pub(super) fn zoom_out(app: &MainWindow, context: &Arc<ChartContext>) {
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        if view.zoom.pop().is_none() {
            return;
        }
    }
    refresh_after_zoom(app, context);
}

pub(super) fn update_zoom_state(app: &MainWindow, context: &ChartContext) {
    let zoom = context.view.lock().expect("View mutex poisoned").zoom.clone();
    let text = if zoom.is_empty() {
        flk!("disk_usage_whole_scan")
    } else {
        let models = context.shared_models.lock().expect("Mutex poisoned");
        models
            .shared_disk_usage_state
            .as_ref()
            .and_then(|tool| tool.get_tree().path(&zoom))
            .map_or_else(String::new, |path| path.to_string_lossy().into_owned())
    };
    let state = app.global::<DiskUsageState>();
    state.set_zoom_text(text.into());
    state.set_can_zoom_out(!zoom.is_empty());
    describe::show_hover(app, None);
}

/// The deepest part of `zoom`, counted from the whole scan, that still has something to show; `has_content` says that for an address.
fn visible_zoom(zoom: &[u32], has_content: impl Fn(&[u32]) -> bool) -> Vec<u32> {
    (0..=zoom.len())
        .rev()
        .filter_map(|length| zoom.get(..length))
        .find(|prefix| prefix.is_empty() || has_content(prefix))
        .map(<[u32]>::to_vec)
        .unwrap_or_default()
}

/// After a file type was switched on or off: when the directory the chart is in has nothing left to show,
/// the view jumps up to the closest directory above that has.
pub(super) fn after_category_change(app: &MainWindow, context: &Arc<ChartContext>) {
    let size_px = context.view.lock().expect("View mutex poisoned").size_px;
    let options = layout_options(app, size_px);
    let zoom = context.view.lock().expect("View mutex poisoned").zoom.clone();
    let new_zoom = {
        let models = context.shared_models.lock().expect("Mutex poisoned");
        let Some(tool) = models.shared_disk_usage_state.as_ref() else {
            return;
        };
        let tree = tool.get_tree();
        visible_zoom(&zoom, |prefix| tree.node(prefix).is_some_and(|node| options.measured_size(node) > 0))
    };
    if new_zoom == zoom {
        request_render(app, context);
        return;
    }
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        view.zoom = new_zoom;
        view.selected = None;
    }
    describe::show_selection(app, None);
    refresh_after_zoom(app, context);
}

pub(super) fn set_category_enabled(app: &MainWindow, index: i32, enabled: bool) {
    let categories = app.global::<DiskUsageState>().get_categories();
    let Ok(row) = usize::try_from(index) else {
        return;
    };
    let Some(mut category) = categories.row_data(row) else {
        return;
    };
    category.enabled = enabled;
    categories.set_row_data(row, category);
}

/// Rebuilds the list of file types after a language change, keeping which ones are enabled.
pub(crate) fn refresh_category_names(app: &MainWindow) {
    let state = app.global::<DiskUsageState>();
    let previous = state.get_categories();
    let categories = FileCategory::ALL
        .iter()
        .skip(1)
        .enumerate()
        .map(|(index, category)| {
            let [red, green, blue] = category.color();
            DiskUsageCategory {
                name: describe::category_name(*category).into(),
                color: slint::Color::from_rgb_u8(red, green, blue),
                enabled: previous.row_data(index).is_none_or(|old| old.enabled),
            }
        })
        .collect::<Vec<_>>();
    debug_assert_eq!(categories.len(), CATEGORY_COUNT - 1);
    state.set_categories(ModelRc::new(VecModel::from(categories)));
}

pub(super) fn selected_address(context: &ChartContext) -> Option<Vec<u32>> {
    context.view.lock().expect("View mutex poisoned").selected.clone()
}

#[cfg(test)]
mod tests {
    use super::{next_zoom_towards, visible_zoom};

    #[test]
    fn zoom_that_still_has_content_stays() {
        assert_eq!(visible_zoom(&[0, 4, 2], |_| true), [0, 4, 2]);
    }

    #[test]
    fn empty_directory_sends_the_view_to_the_closest_directory_above_with_content() {
        let has_content = |prefix: &[u32]| prefix.len() <= 2;

        assert_eq!(visible_zoom(&[0, 4, 2, 7], has_content), [0, 4]);
    }

    #[test]
    fn view_goes_back_to_the_whole_scan_when_nothing_below_has_content() {
        assert_eq!(visible_zoom(&[0, 4], |_| false), Vec::<u32>::new());
        assert_eq!(visible_zoom(&[], |_| false), Vec::<u32>::new());
    }

    #[test]
    fn steps_one_level_at_a_time_towards_the_selection() {
        let selected = [0, 4, 2, 7];

        assert_eq!(next_zoom_towards(&[], &selected, false, false), Some(vec![0]));
        assert_eq!(next_zoom_towards(&[0], &selected, false, false), Some(vec![0, 4]));
        assert_eq!(next_zoom_towards(&[0, 4], &selected, false, false), Some(vec![0, 4, 2]));
    }

    #[test]
    fn stops_at_the_directory_that_holds_a_selected_file() {
        assert_eq!(next_zoom_towards(&[0, 4, 2], &[0, 4, 2, 7], false, false), None);
        assert_eq!(next_zoom_towards(&[0, 4, 2], &[0, 4, 2, 7], true, false), Some(vec![0, 4, 2, 7]));
    }

    #[test]
    fn single_root_is_skipped_because_zooming_into_it_looks_the_same() {
        assert_eq!(next_zoom_towards(&[], &[0, 4, 2], true, true), Some(vec![0, 4]));
    }

    #[test]
    fn selection_outside_the_zoomed_directory_gives_nothing() {
        assert_eq!(next_zoom_towards(&[0, 5], &[0, 4, 2], true, false), None);
    }
}
