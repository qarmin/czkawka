use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

use czkawka_core::common::image::{ImgResizeOptions, get_dynamic_image_from_path};
use czkawka_core::re_exported::FirFilterType;
use image::imageops;
use log::error;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::{AppState, CompareImageData, MainWindow};

// Cancel token

thread_local! {
    static CANCEL_TOKEN: RefCell<Arc<AtomicBool>> =
        RefCell::new(Arc::new(AtomicBool::new(false)));
}

fn new_cancel_token() -> Arc<AtomicBool> {
    let token = Arc::new(AtomicBool::new(false));
    CANCEL_TOKEN.with(|t| *t.borrow_mut() = token.clone());
    token
}

fn request_cancel() {
    CANCEL_TOKEN.with(|t| t.borrow().store(true, Ordering::Relaxed));
}

// Diff generation counter

thread_local! {
    static DIFF_GEN: RefCell<Arc<AtomicU64>> =
        RefCell::new(Arc::new(AtomicU64::new(0)));
}

fn next_diff_gen() -> (Arc<AtomicU64>, u64) {
    DIFF_GEN.with(|g| {
        let arc = g.borrow().clone();
        let val = arc.fetch_add(1, Ordering::Relaxed) + 1;
        (arc, val)
    })
}

// Raw pixel buffer

struct RawPixels {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl RawPixels {
    fn into_slint_image(self) -> slint::Image {
        if self.width == 0 {
            return slint::Image::default();
        }
        let buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(&self.data, self.width, self.height);
        slint::Image::from_rgba8(buf)
    }
}

// Image loading

fn load_raw_image(path: &str, max_w: u32, max_h: u32) -> Option<RawPixels> {
    let meta = std::fs::metadata(path).ok()?;
    if !meta.is_file() {
        return None;
    }
    match get_dynamic_image_from_path(
        path,
        Some(ImgResizeOptions {
            max_width: max_w,
            max_height: max_h,
            filter: FirFilterType::Bilinear,
        }),
    ) {
        Ok(result) => {
            let buf = result.image.into_rgba8();
            let w = buf.width();
            let h = buf.height();
            Some(RawPixels {
                data: buf.into_raw(),
                width: w,
                height: h,
            })
        }
        Err(e) => {
            error!("compare: failed to load \"{path}\": {e}");
            None
        }
    }
}

// Diff computation

fn compute_diff_image(left_path: &str, right_path: &str) -> Option<RawPixels> {
    let left = load_raw_image(left_path, 1200, 900)?;
    let right = load_raw_image(right_path, 1200, 900)?;

    let (w, h) = (left.width, left.height);

    // If dimensions differ, resize right to match left.
    let right_data: Vec<u8> = if (right.width, right.height) != (w, h) {
        let right_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(right.width, right.height, right.data)?;
        let resized = imageops::resize(&right_buf, w, h, imageops::FilterType::Lanczos3);
        resized.into_raw()
    } else {
        right.data
    };

    // Per-pixel squared-difference → greyscale RGBA.
    let mut out: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);
    for (lp, rp) in left.data.chunks_exact(4).zip(right_data.chunks_exact(4)) {
        let dr = (lp[0] as f32 - rp[0] as f32).powi(2);
        let dg = (lp[1] as f32 - rp[1] as f32).powi(2);
        let db = (lp[2] as f32 - rp[2] as f32).powi(2);
        let val = ((dr + dg + db) / (3.0 * 255.0_f32 * 255.0_f32)).sqrt() * 255.0;
        let val = val.clamp(0.0, 255.0) as u8;
        out.extend_from_slice(&[val, val, val, 255]);
    }

    Some(RawPixels { data: out, width: w, height: h })
}

// Public wiring function

pub fn wire_compare(app: &MainWindow) {
    wire_compare_open(app);
    wire_compare_set_left(app);
    wire_compare_set_right(app);
    wire_compare_toggle_checkbox(app);
    wire_compare_next_group(app);
    wire_compare_prev_group(app);
    wire_compare_swap(app);
    wire_compare_cancel_load(app);
    wire_compare_compute_diff(app);
}

// Individual callback wires

fn wire_compare_open(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_open(move |group_idx| {
        let app = weak.upgrade().expect("wire_compare_open: upgrade failed");
        open_group(&app, group_idx as usize);
    });
}

fn wire_compare_set_left(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_set_left(move |compare_idx| {
        let app = weak.upgrade().expect("wire_compare_set_left: upgrade failed");
        let state = app.global::<AppState>();
        let idx = compare_idx as usize;
        if compare_idx == state.get_compare_right_idx() {
            return;
        }
        let images = state.get_compare_images();
        if idx >= images.row_count() {
            return;
        }
        let path = images
            .row_data(idx)
            .unwrap_or_else(|| panic!("wire_compare_set_left: invalid compare_idx {compare_idx}"))
            .path
            .to_string();
        state.set_compare_left_idx(compare_idx);
        state.set_compare_diff_image(slint::Image::default());
        if let Some(raw) = load_raw_image(&path, 1200, 900) {
            state.set_compare_left_image(raw.into_slint_image());
        }
    });
}

fn wire_compare_set_right(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_set_right(move |compare_idx| {
        let app = weak.upgrade().expect("wire_compare_set_right: upgrade failed");
        let state = app.global::<AppState>();
        let idx = compare_idx as usize;
        if compare_idx == state.get_compare_left_idx() {
            return;
        }
        let images = state.get_compare_images();
        if idx >= images.row_count() {
            return;
        }
        let path = images
            .row_data(idx)
            .unwrap_or_else(|| panic!("wire_compare_set_right: invalid compare_idx {idx}"))
            .path
            .to_string();
        state.set_compare_right_idx(compare_idx);
        state.set_compare_diff_image(slint::Image::default());
        if let Some(raw) = load_raw_image(&path, 1200, 900) {
            state.set_compare_right_image(raw.into_slint_image());
        }
    });
}

fn wire_compare_toggle_checkbox(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_toggle_checkbox(move |compare_idx| {
        let app = weak.upgrade().expect("wire_compare_toggle_checkbox: upgrade failed");
        let state = app.global::<AppState>();
        let cidx = compare_idx as usize;
        let images_model = state.get_compare_images();

        if cidx >= images_model.row_count() {
            return;
        }

        let item = images_model
            .row_data(cidx)
            .unwrap_or_else(|| panic!("wire_compare_toggle_checkbox: invalid compare_idx {cidx}"));
        let new_checked = !item.checked;
        let flat_idx = item.flat_idx as usize;
        let group_idx = item.group_idx as usize;
        let item_idx = item.item_idx as usize;

        // Sync checked state back into the flat similar_images_model.
        let flat_model = app.get_similar_images_model();
        if let Some(mut row) = flat_model.row_data(flat_idx) {
            row.checked = new_checked;
            flat_model.set_row_data(flat_idx, row);
        }

        // Sync into similar_images_groups (gallery thumbnails reflect selection).
        let groups = app.get_similar_images_groups();
        if let Some(group) = groups.row_data(group_idx)
            && let Some(mut gi) = group.items.row_data(item_idx)
        {
            gi.checked = new_checked;
            group.items.set_row_data(item_idx, gi);
        }

        // Update compare_images model entry.
        update_compare_checked(&images_model, cidx, new_checked);

        // Keep the global selected_count in sync.
        let delta: i64 = if new_checked { 1 } else { -1 };
        let cur = state.get_selected_count() as i64 + delta;
        state.set_selected_count(cur.max(0) as i32);
    });
}

fn wire_compare_next_group(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_next_group(move || {
        let app = weak.upgrade().expect("wire_compare_next_group: upgrade failed");
        let state = app.global::<AppState>();
        let groups = app.get_similar_images_groups();
        let current = state.get_compare_current_group_idx() as usize;
        let next = current + 1;
        if next < groups.row_count() {
            open_group(&app, next);
        }
    });
}

fn wire_compare_prev_group(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_prev_group(move || {
        let app = weak.upgrade().expect("wire_compare_prev_group: upgrade failed");
        let state = app.global::<AppState>();
        let current = state.get_compare_current_group_idx() as usize;
        if current > 0 {
            open_group(&app, current - 1);
        }
    });
}

fn wire_compare_swap(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_swap(move || {
        let app = weak.upgrade().expect("wire_compare_swap: upgrade failed");
        let state = app.global::<AppState>();
        let li = state.get_compare_left_idx();
        let ri = state.get_compare_right_idx();
        let li_img = state.get_compare_left_image();
        let ri_img = state.get_compare_right_image();
        state.set_compare_left_idx(ri);
        state.set_compare_right_idx(li);
        state.set_compare_left_image(ri_img);
        state.set_compare_right_image(li_img);
        state.set_compare_diff_image(slint::Image::default());
    });
}

fn wire_compare_cancel_load(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_cancel_load(move || {
        let app = weak.upgrade().expect("wire_compare_cancel_load: upgrade failed");
        app.global::<AppState>().set_compare_cancelling(true);
        request_cancel();
    });
}

fn wire_compare_compute_diff(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<AppState>().on_compare_compute_diff(move || {
        let app = weak.upgrade().expect("wire_compare_compute_diff: upgrade failed");
        let state = app.global::<AppState>();
        let li = state.get_compare_left_idx() as usize;
        let ri = state.get_compare_right_idx() as usize;
        let images = state.get_compare_images();

        let left_path = images.row_data(li).map(|d| d.path.to_string()).unwrap_or_default();
        let right_path = images.row_data(ri).map(|d| d.path.to_string()).unwrap_or_default();
        if left_path.is_empty() || right_path.is_empty() {
            return;
        }

        state.set_compare_diff_image(slint::Image::default());
        let (gen_counter, gen_val) = next_diff_gen();
        let weak2 = weak.clone();

        thread::spawn(move || {
            let diff = compute_diff_image(&left_path, &right_path);
            if gen_counter.load(Ordering::Relaxed) != gen_val {
                return;
            }
            weak2
                .upgrade_in_event_loop(move |app| {
                    if gen_counter.load(Ordering::Relaxed) == gen_val
                        && let Some(raw) = diff
                    {
                        app.global::<AppState>().set_compare_diff_image(raw.into_slint_image());
                    }
                })
                .expect("compare_compute_diff: upgrade_in_event_loop failed");
        });
    });
}

// open_group

fn open_group(app: &MainWindow, group_idx: usize) {
    let groups = app.get_similar_images_groups();
    let state = app.global::<AppState>();

    if group_idx >= groups.row_count() {
        return;
    }
    let group = groups.row_data(group_idx).unwrap_or_else(|| panic!("open_group: invalid group_idx {group_idx}"));
    if group.items.row_count() == 0 {
        return;
    }

    let flat_model = app.get_similar_images_model();
    let mut compare_data: Vec<CompareImageData> = Vec::with_capacity(group.items.row_count());

    for i in 0..group.items.row_count() {
        let item = group.items.row_data(i).unwrap_or_else(|| panic!("open_group: invalid item_idx {i} in group {group_idx}"));

        // Use the live checked state from the flat model (may have changed since
        // the gallery group was last built).
        let live_checked = flat_model.row_data(item.flat_idx as usize).map_or(item.checked, |r| r.checked);

        let get_str = |idx: usize| -> SharedString {
            item.val_str
                .row_data(idx)
                .unwrap_or_else(|| panic!("open_group: val_str[{idx}] missing, full val_str={:?}", item.val_str.iter().collect::<Vec<_>>()))
        };

        compare_data.push(CompareImageData {
            path: item.full_path.clone(),
            dir: get_str(crate::common::STR_IDX_PATH),
            name: item.name.clone(),
            size: item.size.clone(),
            dims: get_str(crate::common::StrDataSimilarImages::DimsDisplay as usize),
            modified: get_str(crate::common::STR_IDX_MODIFIED),
            checked: live_checked,
            thumbnail: item.thumbnail.clone(),
            flat_idx: item.flat_idx,
            group_idx: group_idx as i32,
            item_idx: i as i32,
        });
    }

    let total = compare_data.len() as i32;
    let left_idx = 0i32;
    let right_idx = (total - 1).min(1);
    let left_path = compare_data.first().map(|s| s.path.to_string()).unwrap_or_default();
    let right_path = compare_data.get(right_idx as usize).map(|s| s.path.to_string()).unwrap_or_default();

    // Set up the full data model and show the overlay on the UI thread.
    state.set_compare_current_group_idx(group_idx as i32);
    state.set_compare_images(ModelRc::new(VecModel::from(compare_data)));
    state.set_compare_left_idx(left_idx);
    state.set_compare_right_idx(right_idx);
    state.set_compare_left_image(slint::Image::default());
    state.set_compare_right_image(slint::Image::default());
    state.set_compare_diff_image(slint::Image::default());
    state.set_compare_loading(true);
    state.set_compare_cancelling(false);
    state.set_compare_loading_current(0);
    state.set_compare_loading_total(total);
    state.set_compare_visible(true);

    // Load full-size images in the background (only RawPixels are sent – they are Send).
    let cancel = new_cancel_token();
    let weak = app.as_weak();

    thread::spawn(move || {
        if cancel.load(Ordering::Relaxed) {
            weak.upgrade_in_event_loop(|app| finish_cancel(&app)).expect("open_group cancel1: upgrade failed");
            return;
        }

        let left_full = load_raw_image(&left_path, 1200, 900);
        let right_full = if left_path != right_path { load_raw_image(&right_path, 1200, 900) } else { None };

        if cancel.load(Ordering::Relaxed) {
            weak.upgrade_in_event_loop(|app| finish_cancel(&app)).expect("open_group cancel2: upgrade failed");
            return;
        }

        weak.upgrade_in_event_loop(move |app| {
            let state = app.global::<AppState>();
            if let Some(raw) = left_full {
                state.set_compare_left_image(raw.into_slint_image());
            }
            if let Some(raw) = right_full {
                state.set_compare_right_image(raw.into_slint_image());
            }
            state.set_compare_loading(false);
            state.set_compare_cancelling(false);
        })
        .expect("open_group finish: upgrade_in_event_loop failed");
    });
}

fn finish_cancel(app: &MainWindow) {
    let state = app.global::<AppState>();
    state.set_compare_loading(false);
    state.set_compare_cancelling(false);
    state.set_compare_visible(false);
}

fn update_compare_checked(images_model: &ModelRc<CompareImageData>, compare_idx: usize, new_checked: bool) {
    if let Some(vm) = images_model.as_any().downcast_ref::<VecModel<CompareImageData>>()
        && let Some(mut item) = vm.row_data(compare_idx)
    {
        item.checked = new_checked;
        vm.set_row_data(compare_idx, item);
    }
}
