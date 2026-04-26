use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

use czkawka_core::common::image::{ImgResizeOptions, get_dynamic_image_from_path, resize_image_exact};
use czkawka_core::re_exported::FirFilterType;
use image::GenericImageView;
use log::error;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::StrDataSimilarImages;
use crate::connect_row_selection::checker::change_number_of_enabled_items;
use crate::shared_models::SharedModels;
use crate::{ActiveTab, Callabler, CompareImageData, GuiState, MainWindow, SingleMainListModel};

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

thread_local! {
    static DIFF_GENERATION: RefCell<Arc<AtomicU64>> =
        RefCell::new(Arc::new(AtomicU64::new(0)));
}

fn next_diff_gen() -> (Arc<AtomicU64>, u64) {
    DIFF_GENERATION.with(|g| {
        let arc = g.borrow().clone();
        let gen_val = arc.fetch_add(1, Ordering::Relaxed) + 1;
        (arc, gen_val)
    })
}

struct RawPixels {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl RawPixels {
    fn empty() -> Self {
        Self {
            data: vec![],
            width: 0,
            height: 0,
        }
    }
    fn into_slint_image(self) -> slint::Image {
        if self.width == 0 {
            return slint::Image::default();
        }
        let buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(&self.data, self.width, self.height);
        slint::Image::from_rgba8(buf)
    }
}

struct RawCompareItem {
    path: String,
    dir: String,
    name: String,
    size: String,
    resolution: String,
    modification_date: String,
    similarity: String,
    checked: bool,
    thumbnail: RawPixels,
    flat_idx: i32,
}

pub(crate) fn connect_compare(app: &MainWindow, _shared_models: Arc<std::sync::Mutex<SharedModels>>) {
    connect_compare_open(app);
    connect_compare_set_left(app);
    connect_compare_set_right(app);
    connect_compare_toggle_checkbox(app);
    connect_compare_next_group(app);
    connect_compare_prev_group(app);
    connect_compare_swap(app);
    connect_compare_cancel_load(app);
    connect_compare_compute_diff(app);
}

fn connect_compare_open(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_open(move || {
        let app = a.upgrade().expect("Failed to upgrade app");
        let model = app.get_similar_images_model();
        if model.row_count() == 0 {
            return;
        }
        let header_idx = find_group_header_for_current_selection(&model);
        open_group(&app, header_idx);
    });
}

fn connect_compare_set_left(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_set_left(move |compare_idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();
        let idx = compare_idx as usize;
        if compare_idx == gui_state.get_compare_right_idx() {
            return;
        }
        let images = gui_state.get_compare_images();
        if idx >= images.row_count() {
            return;
        }
        let path = images.row_data(idx).expect("compare_idx must be a valid index into compare_images").path.to_string();
        gui_state.set_compare_left_idx(idx as i32);
        gui_state.set_compare_diff_image(slint::Image::default());
        if let Some(img) = load_full_image(&path) {
            gui_state.set_compare_left_image(img);
        }
    });
}

fn connect_compare_set_right(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_set_right(move |compare_idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();
        let idx = compare_idx as usize;
        if compare_idx == gui_state.get_compare_left_idx() {
            return;
        }
        let images = gui_state.get_compare_images();
        if idx >= images.row_count() {
            return;
        }
        let path = images.row_data(idx).expect("compare_idx must be a valid index into compare_images").path.to_string();
        gui_state.set_compare_right_idx(idx as i32);
        gui_state.set_compare_diff_image(slint::Image::default());
        if let Some(img) = load_full_image(&path) {
            gui_state.set_compare_right_image(img);
        }
    });
}

fn connect_compare_toggle_checkbox(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_toggle_checkbox(move |compare_idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();
        let compare_idx = compare_idx as usize;
        let images_model = gui_state.get_compare_images();

        if compare_idx >= images_model.row_count() {
            return;
        }

        let item = images_model.row_data(compare_idx).expect("compare_idx must be a valid index into compare_images");
        let flat_idx = item.flat_idx as usize;
        let new_checked = !item.checked;

        let main_model = app.get_similar_images_model();
        if let Some(mut main_row) = main_model.row_data(flat_idx) {
            let old_main_checked = main_row.checked;
            main_row.checked = new_checked;
            main_model.set_row_data(flat_idx, main_row);

            if old_main_checked != new_checked {
                let delta: i64 = if new_checked { 1 } else { -1 };
                change_number_of_enabled_items(&app, ActiveTab::SimilarImages, delta);
            }
        }

        update_compare_checked(&gui_state, compare_idx, new_checked);
    });
}

fn connect_compare_next_group(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_next_group(move || {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();
        let model = app.get_similar_images_model();
        let current = gui_state.get_compare_current_group_header_flat_idx() as usize;
        if let Some(next) = find_adjacent_group_header(&model, current, true) {
            open_group(&app, next);
        }
    });
}

fn connect_compare_prev_group(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_prev_group(move || {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();
        let model = app.get_similar_images_model();
        let current = gui_state.get_compare_current_group_header_flat_idx() as usize;
        if let Some(prev) = find_adjacent_group_header(&model, current, false) {
            open_group(&app, prev);
        }
    });
}

fn connect_compare_swap(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_swap(move || {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();
        let left_idx = gui_state.get_compare_left_idx();
        let right_idx = gui_state.get_compare_right_idx();
        let left_image = gui_state.get_compare_left_image();
        let right_image = gui_state.get_compare_right_image();
        gui_state.set_compare_left_idx(right_idx);
        gui_state.set_compare_right_idx(left_idx);
        gui_state.set_compare_left_image(right_image);
        gui_state.set_compare_right_image(left_image);
        gui_state.set_compare_diff_image(slint::Image::default());
    });
}

fn connect_compare_cancel_load(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_cancel_load(move || {
        let app = a.upgrade().expect("Failed to upgrade app");
        app.global::<GuiState>().set_compare_cancelling(true);
        request_cancel();
    });
}

fn connect_compare_compute_diff(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_compare_compute_diff(move || {
        let app = a.upgrade().expect("Failed to upgrade app");
        let gui_state = app.global::<GuiState>();

        let left_idx = gui_state.get_compare_left_idx() as usize;
        let right_idx = gui_state.get_compare_right_idx() as usize;
        let images = gui_state.get_compare_images();

        let left_path = images.row_data(left_idx).map(|d| d.path.to_string()).expect("path_idx must be a valid column index");
        let right_path = images.row_data(right_idx).map(|d| d.path.to_string()).expect("path_idx must be a valid column index");

        if left_path.is_empty() || right_path.is_empty() {
            return;
        }

        gui_state.set_compare_diff_image(slint::Image::default());
        let (gen_counter, gen_val) = next_diff_gen();

        let weak = a.clone();
        thread::spawn(move || {
            let diff = compute_diff_image(&left_path, &right_path);
            if gen_counter.load(Ordering::Relaxed) != gen_val {
                return;
            }
            weak.upgrade_in_event_loop(move |app| {
                if gen_counter.load(Ordering::Relaxed) == gen_val
                    && let Some(raw) = diff
                {
                    app.global::<GuiState>().set_compare_diff_image(raw.into_slint_image());
                }
            })
            .expect("Failed to upgrade app :(");
        });
    });
}

fn open_group(app: &MainWindow, header_idx: usize) {
    let model = app.get_similar_images_model();
    let gui_state = app.global::<GuiState>();

    let data_indices = collect_group_data_indices(&model, header_idx);
    if data_indices.is_empty() {
        return;
    }

    let total = data_indices.len() as i32;

    let rows: Vec<(usize, bool, Vec<String>)> = data_indices
        .iter()
        .map(|&flat_idx| {
            let row = model.row_data(flat_idx).expect("data_indices must contain valid model indices");
            let strs: Vec<String> = (0..row.val_str.row_count())
                .map(|i| row.val_str.row_data(i).map(|s| s.to_string()).expect("SimilarImages row must have enough columns"))
                .collect();
            (flat_idx, row.checked, strs)
        })
        .collect();

    gui_state.set_compare_current_group_header_flat_idx(header_idx as i32);
    gui_state.set_compare_loading_current(0);
    gui_state.set_compare_loading_total(total);

    gui_state.set_compare_images(Rc::new(VecModel::from(vec![])).into());
    gui_state.set_compare_left_image(slint::Image::default());
    gui_state.set_compare_right_image(slint::Image::default());
    gui_state.set_compare_diff_image(slint::Image::default());
    gui_state.set_compare_loading(true);
    gui_state.set_compare_cancelling(false);
    gui_state.set_compare_visible(true);

    let cancel = new_cancel_token();
    let weak = app.as_weak();

    thread::spawn(move || {
        let mut raw_items: Vec<RawCompareItem> = Vec::with_capacity(rows.len());

        for (i, (flat_idx, checked, strs)) in rows.iter().enumerate() {
            if cancel.load(Ordering::Relaxed) {
                let weak_c = weak.clone();
                weak_c
                    .upgrade_in_event_loop(move |app| {
                        let gs = app.global::<GuiState>();
                        gs.set_compare_loading(false);
                        gs.set_compare_cancelling(false);
                        gs.set_compare_visible(false);
                    })
                    .expect("Failed to upgrade app :(");
                return;
            }

            let dir = strs
                .get(StrDataSimilarImages::Path as usize)
                .cloned()
                .expect("SimilarImages row must contain a Path column");
            let name = strs
                .get(StrDataSimilarImages::Name as usize)
                .cloned()
                .expect("SimilarImages row must contain a Name column");
            let full_path = format!("{dir}/{name}");
            let thumbnail = load_raw_thumbnail(&full_path);

            raw_items.push(RawCompareItem {
                path: full_path,
                dir: dir.clone(),
                name,
                size: strs
                    .get(StrDataSimilarImages::Size as usize)
                    .cloned()
                    .expect("SimilarImages row must contain a Size column"),
                resolution: strs
                    .get(StrDataSimilarImages::Resolution as usize)
                    .cloned()
                    .expect("SimilarImages row must contain a Resolution column"),
                modification_date: strs
                    .get(StrDataSimilarImages::ModificationDate as usize)
                    .cloned()
                    .expect("SimilarImages row must contain a ModificationDate column"),
                similarity: strs
                    .get(StrDataSimilarImages::Similarity as usize)
                    .cloned()
                    .expect("SimilarImages row must contain a Similarity column"),
                checked: *checked,
                thumbnail,
                flat_idx: *flat_idx as i32,
            });

            let current = i as i32 + 1;
            let weak_c = weak.clone();
            weak_c
                .upgrade_in_event_loop(move |app| {
                    app.global::<GuiState>().set_compare_loading_current(current);
                })
                .expect("Failed to upgrade app :(");
        }

        let left_idx_slint = 0i32;
        let right_idx_slint = (raw_items.len() as i32 - 1).min(1);

        if cancel.load(Ordering::Relaxed) {
            weak.upgrade_in_event_loop(move |app| {
                let gs = app.global::<GuiState>();
                gs.set_compare_loading(false);
                gs.set_compare_cancelling(false);
                gs.set_compare_visible(false);
            })
            .expect("Failed to upgrade app :(");
            return;
        }

        let left_path = raw_items
            .get(left_idx_slint as usize)
            .map(|r| r.path.clone())
            .expect("left_idx_slint is 0 and raw_items is guaranteed to have at least 1 item");
        let right_path = raw_items
            .get(right_idx_slint as usize)
            .map(|r| r.path.clone())
            .expect("right_idx_slint is either 0 or 1 and raw_items is guaranteed to have at least 1 item");

        let left_raw = load_raw_full_image(&left_path);
        let right_raw = load_raw_full_image(&right_path);

        weak.upgrade_in_event_loop(move |app| {
            let gui_state = app.global::<GuiState>();

            let main_model = app.get_similar_images_model();
            let compare_data: Vec<CompareImageData> = raw_items
                .into_iter()
                .map(|r| {
                    let current_checked = main_model.row_data(r.flat_idx as usize).map_or(r.checked, |row| row.checked);
                    CompareImageData {
                        path: r.path.into(),
                        dir: r.dir.into(),
                        name: r.name.into(),
                        size: r.size.into(),
                        resolution: r.resolution.into(),
                        modification_date: r.modification_date.into(),
                        similarity: r.similarity.into(),
                        checked: current_checked,
                        thumbnail: r.thumbnail.into_slint_image(),
                        flat_idx: r.flat_idx,
                    }
                })
                .collect();

            gui_state.set_compare_images(Rc::new(VecModel::from(compare_data)).into());
            gui_state.set_compare_left_idx(left_idx_slint);
            gui_state.set_compare_right_idx(right_idx_slint);

            if let Some(raw) = left_raw {
                gui_state.set_compare_left_image(raw.into_slint_image());
            }
            if let Some(raw) = right_raw {
                gui_state.set_compare_right_image(raw.into_slint_image());
            }

            gui_state.set_compare_cancelling(false);
            gui_state.set_compare_loading(false);
        })
        .expect("Failed to upgrade app :(");
    });
}

fn update_compare_checked(gui_state: &GuiState, compare_idx: usize, new_checked: bool) {
    let images_model = gui_state.get_compare_images();
    let vec_model = images_model
        .as_any()
        .downcast_ref::<VecModel<CompareImageData>>()
        .expect("compare_images is always a VecModel<CompareImageData>");
    let mut item = vec_model.row_data(compare_idx).expect("compare_idx must be a valid index into compare_images");
    item.checked = new_checked;
    vec_model.set_row_data(compare_idx, item);
}

fn collect_group_data_indices(model: &ModelRc<SingleMainListModel>, header_idx: usize) -> Vec<usize> {
    let count = model.row_count();
    let mut indices = Vec::new();
    let h = model.row_data(header_idx).expect("header_idx must be a valid model row index");
    if h.header_row && h.filled_header_row {
        indices.push(header_idx);
    }

    let mut i = header_idx + 1;
    while i < count {
        let row = model.row_data(i).expect("i must be within row_count bounds");
        if row.header_row {
            break;
        }
        indices.push(i);
        i += 1;
    }

    indices
}

fn find_group_header_for_current_selection(model: &ModelRc<SingleMainListModel>) -> usize {
    if let Some(sel_idx) = model.iter().enumerate().find(|(_, r)| r.focused_row && !r.header_row).map(|(i, _)| i) {
        for i in (0..=sel_idx).rev() {
            if model.row_data(i).expect("i is within sel_idx bounds which is within row_count").header_row {
                return i;
            }
        }
    }

    model.iter().enumerate().find(|(_, r)| r.header_row).map_or(0, |(i, _)| i)
}

fn find_adjacent_group_header(model: &ModelRc<SingleMainListModel>, current_header_idx: usize, forward: bool) -> Option<usize> {
    let count = model.row_count();
    if forward {
        for i in (current_header_idx + 1)..count {
            if model.row_data(i).expect("i is within row_count bounds").header_row {
                return Some(i);
            }
        }
    } else if current_header_idx > 0 {
        for i in (0..current_header_idx).rev() {
            if model.row_data(i).expect("i is within row_count bounds").header_row {
                return Some(i);
            }
        }
    }
    None
}

fn load_raw_thumbnail(path: &str) -> RawPixels {
    load_raw_image(path, 200, 150).unwrap_or_else(RawPixels::empty)
}

fn load_raw_full_image(path: &str) -> Option<RawPixels> {
    load_raw_image(path, 1200, 900)
}

fn load_raw_image(path: &str, max_w: u32, max_h: u32) -> Option<RawPixels> {
    let p = Path::new(path);
    if !p.is_file() {
        return None;
    }
    match get_dynamic_image_from_path(
        &p.to_string_lossy(),
        Some(ImgResizeOptions {
            max_width: max_w,
            max_height: max_h,
            filter: FirFilterType::Bilinear,
        }),
    ) {
        Ok(result) => {
            let buf = result.image.into_rgba8();
            Some(RawPixels {
                data: buf.as_raw().clone(),
                width: buf.width(),
                height: buf.height(),
            })
        }
        Err(e) => {
            error!("Failed to load compare image \"{path}\": {e}");
            None
        }
    }
}

fn load_full_image(path: &str) -> Option<slint::Image> {
    load_raw_full_image(path).map(RawPixels::into_slint_image)
}

fn compute_diff_image(left_path: &str, right_path: &str) -> Option<RawPixels> {
    let resize_opts = Some(ImgResizeOptions {
        max_width: 1200,
        max_height: 900,
        filter: FirFilterType::Bilinear,
    });

    let left = get_dynamic_image_from_path(left_path, resize_opts).ok()?.image;
    let right = get_dynamic_image_from_path(right_path, resize_opts).ok()?.image;

    let (w, h) = (left.width(), left.height());

    let right = if (right.width(), right.height()) != (w, h) {
        resize_image_exact(&right, w, h, FirFilterType::Lanczos3)
    } else {
        right
    };

    assert_eq!(left.dimensions(), right.dimensions());

    let left_rgba = left.to_rgba8();
    let right_rgba = right.to_rgba8();

    let mut data: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);

    for y in 0..h {
        for x in 0..w {
            let lp = left_rgba.get_pixel(x, y).0;
            let rp = right_rgba.get_pixel(x, y).0;

            let dr = (lp[0] as f32 - rp[0] as f32).powi(2);
            let dg = (lp[1] as f32 - rp[1] as f32).powi(2);
            let db = (lp[2] as f32 - rp[2] as f32).powi(2);

            let val = ((dr + dg + db) / (3.0 * 255.0_f32 * 255.0_f32)).sqrt() * 255.0;
            let val = val.clamp(0.0, 255.0) as u8;

            data.extend_from_slice(&[val, val, val, 255]);
        }
    }

    Some(RawPixels { data, width: w, height: h })
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use slint::{ModelRc, VecModel};

    use super::{collect_group_data_indices, find_adjacent_group_header, find_group_header_for_current_selection};
    use crate::SingleMainListModel;

    fn row(header: bool, filled: bool, selected: bool) -> SingleMainListModel {
        SingleMainListModel {
            header_row: header,
            filled_header_row: filled,
            focused_row: selected,
            ..Default::default()
        }
    }

    fn make_model(rows: Vec<SingleMainListModel>) -> ModelRc<SingleMainListModel> {
        Rc::new(VecModel::from(rows)).into()
    }

    #[test]
    fn test_collect_group_data_indices() {
        // [H*] [D] [D] | filled header → all 3 included
        let model = make_model(vec![row(true, true, false), row(false, false, false), row(false, false, false)]);
        assert_eq!(collect_group_data_indices(&model, 0), vec![0, 1, 2]);

        // unfilled header → only data rows
        let model = make_model(vec![row(true, false, false), row(false, false, false), row(false, false, false)]);
        assert_eq!(collect_group_data_indices(&model, 0), vec![1, 2]);

        // two groups: first group stops at second header
        let model = make_model(vec![row(true, true, false), row(false, false, false), row(true, true, false), row(false, false, false)]);
        assert_eq!(collect_group_data_indices(&model, 0), vec![0, 1]);
        assert_eq!(collect_group_data_indices(&model, 2), vec![2, 3]);
    }

    #[test]
    fn test_find_group_header_for_current_selection() {
        // no selection → fall back to first header (idx 0)
        let model = make_model(vec![row(true, true, false), row(false, false, false)]);
        assert_eq!(find_group_header_for_current_selection(&model), 0);

        // selected data row belongs to first group
        let model = make_model(vec![
            row(true, true, false),
            row(false, false, true), // selected
            row(true, true, false),
            row(false, false, false),
        ]);
        assert_eq!(find_group_header_for_current_selection(&model), 0);

        // selected data row belongs to second group
        let model = make_model(vec![
            row(true, true, false),
            row(false, false, false),
            row(true, true, false),
            row(false, false, true), // selected
        ]);
        assert_eq!(find_group_header_for_current_selection(&model), 2);
    }

    #[test]
    fn test_find_adjacent_group_header() {
        let model = make_model(vec![
            row(true, true, false),   // 0
            row(false, false, false), // 1
            row(true, true, false),   // 2
            row(false, false, false), // 3
            row(true, true, false),   // 4
        ]);

        assert_eq!(find_adjacent_group_header(&model, 0, true), Some(2));
        assert_eq!(find_adjacent_group_header(&model, 2, true), Some(4));
        assert_eq!(find_adjacent_group_header(&model, 4, true), None);

        assert_eq!(find_adjacent_group_header(&model, 4, false), Some(2));
        assert_eq!(find_adjacent_group_header(&model, 2, false), Some(0));
        assert_eq!(find_adjacent_group_header(&model, 0, false), None);
    }
}
