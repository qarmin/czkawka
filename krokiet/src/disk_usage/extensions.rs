use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;

use czkawka_core::tools::disk_usage::extensions::{ExtensionStat, extension_stats};
use humansize::{BINARY, format_size};
use log::error;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::disk_usage::view::{ChartContext, request_render};
use crate::{DiskUsageExtensionRow, DiskUsageState, MainWindow, flk};

const MAX_ROWS: usize = 300;

pub(super) fn toggled(app: &MainWindow, context: &Arc<ChartContext>) {
    if app.global::<DiskUsageState>().get_extensions_open() {
        refresh(app, context);
        return;
    }
    let had_highlight = context.view.lock().expect("View mutex poisoned").highlighted_extension.take().is_some();
    if had_highlight {
        request_render(app, context);
    }
}

/// Counts the extensions below the zoomed directory on a worker thread, because a big scan has millions of files.
pub(super) fn refresh(app: &MainWindow, context: &Arc<ChartContext>) {
    if !app.global::<DiskUsageState>().get_extensions_open() {
        return;
    }
    let generation = context.extensions_generation.fetch_add(1, Ordering::Relaxed) + 1;
    let zoom = context.view.lock().expect("View mutex poisoned").zoom.clone();
    let (weak, context) = (app.as_weak(), Arc::clone(context));
    let spawned = thread::Builder::new().name("disk-usage-extensions".into()).spawn(move || {
        let stats = {
            let models = context.shared_models.lock().expect("Mutex poisoned");
            models.shared_disk_usage_state.as_ref().map_or_else(Vec::new, |tool| {
                let tree = tool.get_tree();
                if zoom.is_empty() {
                    extension_stats(&tree.roots)
                } else {
                    tree.node(&zoom).map_or_else(Vec::new, |node| extension_stats(std::slice::from_ref(node)))
                }
            })
        };
        let published = weak.upgrade_in_event_loop(move |app| publish(&app, &context, generation, &stats));
        if let Err(e) = published {
            error!("Failed to show the extension list: {e}");
        }
    });
    if let Err(e) = spawned {
        error!("Failed to start the thread counting extensions: {e}");
    }
}

fn publish(app: &MainWindow, context: &ChartContext, generation: u64, stats: &[ExtensionStat]) {
    if context.extensions_generation.load(Ordering::Relaxed) != generation {
        return;
    }
    let total = stats.iter().map(|stat| stat.size).sum::<u64>().max(1);
    let mut view = context.view.lock().expect("View mutex poisoned");
    let shown = &stats[..stats.len().min(MAX_ROWS)];
    view.extension_names = shown.iter().map(|stat| stat.extension.to_string()).collect();
    let rows = shown
        .iter()
        .map(|stat| {
            let [red, green, blue] = stat.category.color();
            DiskUsageExtensionRow {
                name: display_name(&stat.extension).into(),
                size_text: format_size(stat.size, BINARY).into(),
                count_text: stat.count.to_string().into(),
                percent: (stat.size as f64 * 100.0 / total as f64) as f32,
                selected: view.highlighted_extension.as_deref() == Some(&*stat.extension),
                color: slint::Color::from_rgb_u8(red, green, blue),
            }
        })
        .collect::<Vec<_>>();
    drop(view);
    app.global::<DiskUsageState>().set_extension_rows(ModelRc::new(VecModel::from(rows)));
}

fn display_name(extension: &str) -> String {
    if extension.is_empty() {
        flk!("disk_usage_no_extension")
    } else {
        format!(".{extension}")
    }
}

pub(super) fn clicked(app: &MainWindow, context: &Arc<ChartContext>, row: i32) {
    let Ok(row) = usize::try_from(row) else {
        return;
    };
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        let Some(name) = view.extension_names.get(row).cloned() else {
            return;
        };
        view.highlighted_extension = if view.highlighted_extension.as_deref() == Some(name.as_str()) {
            None
        } else {
            Some(name)
        };
        let highlighted = view.highlighted_extension.clone();
        let rows = app.global::<DiskUsageState>().get_extension_rows();
        for (index, extension) in view.extension_names.iter().enumerate() {
            if let Some(mut data) = rows.row_data(index) {
                data.selected = highlighted.as_deref() == Some(extension.as_str());
                rows.set_row_data(index, data);
            }
        }
    }
    request_render(app, context);
}
