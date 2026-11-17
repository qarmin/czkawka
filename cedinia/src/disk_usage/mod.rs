mod actions;
mod list_model;
mod scan;

use std::cell::RefCell;
use std::sync::Arc;

use czkawka_core::tools::disk_usage::Info;
use czkawka_core::tools::disk_usage::tree::DiskTree;
pub(crate) use scan::scan_disk_usage;
use slint::{Color, ComponentHandle, ModelRc, SharedString, VecModel};

use crate::disk_usage::list_model::{Row, RowKind, address_at_level, breadcrumb, build_rows};
use crate::scan_runner::fmt_size;
use crate::{DiskUsageRow, DiskUsageState, MainWindow, flc};

#[derive(Default)]
struct View {
    tree: Option<Arc<DiskTree>>,
    info: Info,
    current: Vec<u32>,
    rows: Vec<Row>,
    menu_target: Option<Vec<u32>>,
}

thread_local! {
    static VIEW: RefCell<View> = RefCell::new(View::default());
}

pub(crate) fn file_count() -> usize {
    VIEW.with_borrow(|view| view.info.number_of_files)
}

pub(crate) fn show_results(win: &MainWindow, tree: Arc<DiskTree>, info: &Info) {
    let has_results = info.number_of_files > 0;
    VIEW.with_borrow_mut(|view| {
        *view = View {
            tree: has_results.then_some(tree),
            info: info.clone(),
            ..View::default()
        };
    });
    win.global::<DiskUsageState>().set_has_results(has_results);
    refresh(win);
}

pub(crate) fn clear_results(win: &MainWindow) {
    VIEW.with_borrow_mut(|view| {
        *view = View::default();
    });
    let state = win.global::<DiskUsageState>();
    state.set_has_results(false);
    state.set_rows(ModelRc::new(VecModel::<DiskUsageRow>::default()));
    state.set_breadcrumb(ModelRc::new(VecModel::<SharedString>::default()));
}

pub(crate) fn wire_disk_usage(win: &MainWindow) {
    let state = win.global::<DiskUsageState>();

    let weak = win.as_weak();
    state.on_row_tapped(move |index| {
        let win = weak.upgrade().expect("MainWindow dropped in on_row_tapped");
        let (kind, address) = VIEW.with_borrow(|view| {
            let row = view
                .rows
                .get(index as usize)
                .unwrap_or_else(|| panic!("row_tapped: index {index} out of bounds (rows={})", view.rows.len()));
            (row.kind, row.address.clone())
        });
        match (kind, address) {
            (RowKind::Directory, Some(address)) => navigate_to(&win, address),
            (RowKind::File, _) => actions::show_menu(&win, index),
            _ => {}
        }
    });

    let weak = win.as_weak();
    state.on_row_menu_requested(move |index| {
        let win = weak.upgrade().expect("MainWindow dropped in on_row_menu_requested");
        actions::show_menu(&win, index);
    });

    let weak = win.as_weak();
    state.on_delete_confirmed(move || {
        let win = weak.upgrade().expect("MainWindow dropped in on_delete_confirmed");
        actions::delete_target(&win);
    });

    let weak = win.as_weak();
    state.on_breadcrumb_tapped(move |level| {
        let win = weak.upgrade().expect("MainWindow dropped in on_breadcrumb_tapped");
        let target = VIEW.with_borrow(|view| address_at_level(&view.current, level as usize));
        navigate_to(&win, target);
    });
}

fn navigate_to(win: &MainWindow, address: Vec<u32>) {
    let changed = VIEW.with_borrow_mut(|view| {
        let changed = view.current != address;
        view.current = address;
        changed
    });
    if changed {
        refresh(win);
    }
}

fn refresh(win: &MainWindow) {
    let state = win.global::<DiskUsageState>();
    VIEW.with_borrow_mut(|view| {
        let Some(tree) = view.tree.clone() else {
            return;
        };
        view.rows = build_rows(&tree, &view.current);
        let rows = view.rows.iter().map(to_slint_row).collect::<Vec<_>>();
        state.set_rows(ModelRc::new(VecModel::from(rows)));

        let segments = breadcrumb(&tree, &view.current, &flc!("disk_usage_all"))
            .into_iter()
            .map(SharedString::from)
            .collect::<Vec<_>>();
        state.set_breadcrumb(ModelRc::new(VecModel::from(segments)));
        state.set_summary_text(flc!("disk_usage_summary", size = fmt_size(tree.total_size()), files = view.info.number_of_files).into());
    });
}

fn to_slint_row(row: &Row) -> DiskUsageRow {
    let [red, green, blue] = row.category.color();
    let name = match row.kind {
        RowKind::Others => flc!("disk_usage_others", count = row.hidden_entries),
        _ => row.name.clone(),
    };
    DiskUsageRow {
        name: name.into(),
        size_text: fmt_size(row.size).into(),
        detail_text: format!("{:.1}%", row.fraction * 100.0).into(),
        fraction: row.fraction,
        color: Color::from_rgb_u8(red, green, blue),
        is_directory: row.kind == RowKind::Directory,
        has_menu: matches!(row.kind, RowKind::Directory | RowKind::File),
        category: if row.kind == RowKind::Directory { 0 } else { row.category as i32 },
    }
}
