mod actions;
mod describe;
mod render;
mod selection;
mod tree_view;
mod view;

use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use slint::ComponentHandle;
pub(crate) use view::refresh_category_names;

use crate::disk_usage::view::{ChartContext, ViewState, spawn_render_worker};
use crate::shared_models::SharedModels;
use crate::{DiskUsageState, MainWindow, flk};

pub(crate) fn connect_disk_usage(app: &MainWindow, shared_models: Arc<Mutex<SharedModels>>) {
    let (requests, receiver) = std::sync::mpsc::channel();
    let context = Arc::new(ChartContext {
        shared_models,
        view: Mutex::new(ViewState::default()),
        chart: Mutex::new(None),
        generation: AtomicU64::new(0),
        tree_version: AtomicU64::new(0),
        requests,
    });
    spawn_render_worker(receiver, Arc::clone(&context), app.as_weak());
    refresh_category_names(app);
    app.global::<DiskUsageState>().set_zoom_text(flk!("disk_usage_whole_scan").into());

    connect_view_callbacks(app, &context);
    connect_item_actions(app, &context);
}

fn connect_view_callbacks(app: &MainWindow, context: &Arc<ChartContext>) {
    let state = app.global::<DiskUsageState>();

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_chart_resized(move |width, height| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        ctx.view.lock().expect("View mutex poisoned").size_px = (width.max(0) as u32, height.max(0) as u32);
        view::request_render(&app, &ctx);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_options_changed(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::request_render(&app, &ctx);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_results_changed(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::reset_view(&app, &ctx);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_chart_hovered(move |x, y| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::show_hovered_item(&app, &ctx, x, y);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_chart_left_clicked(move |x, y| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::select_item_at(&app, &ctx, x, y);
    });

    let ctx = Arc::clone(context);
    state.on_chart_double_clicked(move |x, y| {
        if let Some(address) = view::address_at(&ctx, x, y) {
            actions::open_address(&ctx, &address, false);
        }
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_chart_right_clicked(move |x, y| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::select_item_at(&app, &ctx, x, y);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_chart_scrolled(move |delta| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        if delta != 0.0 {
            view::scroll_step(&app, &ctx, delta > 0.0);
        }
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_zoom_out(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::zoom_out(&app, &ctx);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_tree_row_toggled(move |row| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        tree_view::toggle_row(&app, &ctx, row);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_tree_row_clicked(move |row| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        tree_view::select_row(&app, &ctx, row);
    });

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_tree_row_right_clicked(move |row| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        tree_view::select_row(&app, &ctx, row);
    });

    let ctx = Arc::clone(context);
    state.on_tree_row_activated(move |row| tree_view::activate_row(&ctx, row));

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_category_toggled(move |index, enabled| {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        view::set_category_enabled(&app, index, enabled);
        view::after_category_change(&app, &ctx);
    });
}

fn connect_item_actions(app: &MainWindow, context: &Arc<ChartContext>) {
    let state = app.global::<DiskUsageState>();

    let ctx = Arc::clone(context);
    state.on_open_selected_item(move || actions::open_selected(&ctx, false));

    let ctx = Arc::clone(context);
    state.on_open_selected_parent(move || actions::open_selected(&ctx, true));

    let ctx = Arc::clone(context);
    state.on_copy_path(move || actions::copy_selected_path(&ctx));

    let (weak, ctx) = (app.as_weak(), Arc::clone(context));
    state.on_move_to_trash(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        actions::move_selected_to_trash(&app, &ctx);
    });
}
