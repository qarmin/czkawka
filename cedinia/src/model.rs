use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::scan_runner::FileItem;
use crate::{AppState, FileEntry, MainWindow, SimilarGroupCard, SimilarImageItem};

pub fn make_file_model(items: Vec<FileItem>) -> ModelRc<FileEntry> {
    let entries: Vec<FileEntry> = items
        .into_iter()
        .map(|item| {
            let val_str: Vec<SharedString> = item.val_str.into_iter().map(SharedString::from).collect();
            let val_int: Vec<i32> = item.val_int;
            FileEntry {
                checked: false,
                is_header: item.is_header,
                is_reference: item.is_reference,
                val_str: ModelRc::new(VecModel::from(val_str)),
                val_int: ModelRc::new(VecModel::from(val_int)),
            }
        })
        .collect();

    ModelRc::new(VecModel::from(entries))
}

pub fn toggle_row(model: &ModelRc<FileEntry>, index: usize) {
    let mut entry = model
        .row_data(index)
        .unwrap_or_else(|| panic!("toggle_row: index {index} out of bounds (row_count={})", model.row_count()));
    if !entry.is_header && !entry.is_reference {
        entry.checked = !entry.checked;
        model.set_row_data(index, entry);
    }
}

pub fn count_checked(model: &ModelRc<FileEntry>) -> i32 {
    model.iter().filter(|e: &FileEntry| e.checked).count() as i32
}

pub(crate) fn build_gallery_groups(items: &[FileItem], placeholder: &slint::Image) -> Vec<SimilarGroupCard> {
    use slint::{ModelRc, SharedString, VecModel};

    use crate::common::{STR_IDX_NAME, STR_IDX_PATH, STR_IDX_SIZE};
    let mut groups: Vec<SimilarGroupCard> = Vec::new();
    let mut cur_label = String::new();
    let mut cur_items: Vec<SimilarImageItem> = Vec::new();

    for (flat_idx, item) in items.iter().enumerate() {
        if item.is_header {
            if !cur_items.is_empty() {
                groups.push(SimilarGroupCard {
                    label: SharedString::from(&cur_label),
                    items: ModelRc::new(VecModel::from(std::mem::take(&mut cur_items))),
                });
            }
            cur_label = item.val_str[STR_IDX_NAME].clone();
        } else {
            let name = &item.val_str[STR_IDX_NAME];
            let path = &item.val_str[STR_IDX_PATH];
            let size = &item.val_str[STR_IDX_SIZE];
            let full_path = if path.is_empty() { name.clone() } else { format!("{path}/{name}") };
            cur_items.push(SimilarImageItem {
                full_path: SharedString::from(full_path),
                name: SharedString::from(name),
                size: SharedString::from(size),
                val_str: ModelRc::new(VecModel::from(item.val_str.iter().map(|s| SharedString::from(s.as_str())).collect::<Vec<_>>())),
                flat_idx: flat_idx as i32,
                thumbnail: placeholder.clone(),
                checked: false,
                is_reference: item.is_reference,
            });
        }
    }
    if !cur_items.is_empty() {
        groups.push(SimilarGroupCard {
            label: SharedString::from(&cur_label),
            items: ModelRc::new(VecModel::from(cur_items)),
        });
    }
    groups
}

pub(crate) fn rebuild_similar_images_after_delete(win: &MainWindow, deleted: &std::collections::HashSet<String>) {
    let groups = win.get_similar_images_groups();
    let mut new_groups: Vec<SimilarGroupCard> = Vec::new();
    let mut new_flat: Vec<FileEntry> = Vec::new();

    for gi in 0..groups.row_count() {
        if let Some(group) = groups.row_data(gi) {
            let surviving: Vec<_> = (0..group.items.row_count())
                .filter_map(|ii| group.items.row_data(ii))
                .filter(|item| !deleted.contains(item.full_path.as_str()))
                .map(|mut item| {
                    item.checked = false;
                    item
                })
                .collect();

            if surviving.is_empty() {
                continue;
            }

            new_flat.push(FileEntry {
                checked: false,
                is_header: true,
                is_reference: false,
                val_str: ModelRc::new(VecModel::from(vec![
                    group.label.clone(),
                    SharedString::default(),
                    SharedString::default(),
                    SharedString::default(),
                ])),
                val_int: ModelRc::new(VecModel::from(Vec::new())),
            });

            let mut final_items: Vec<SimilarImageItem> = Vec::new();
            for mut item in surviving {
                item.flat_idx = new_flat.len() as i32;
                new_flat.push(FileEntry {
                    checked: false,
                    is_header: false,
                    is_reference: false,
                    val_str: item.val_str.clone(),
                    val_int: ModelRc::new(VecModel::from(Vec::new())),
                });
                final_items.push(item);
            }

            new_groups.push(SimilarGroupCard {
                label: group.label.clone(),
                items: ModelRc::new(VecModel::from(final_items)),
            });
        }
    }

    win.set_similar_images_model(ModelRc::new(VecModel::from(new_flat)));
    win.set_similar_images_groups(ModelRc::new(VecModel::from(new_groups)));
    win.global::<AppState>().set_selected_count(0);
}

#[cfg(test)]
mod tests {
    use slint::Model;

    use super::*;
    use crate::scan_runner::FileItem;

    fn make_entry(checked: bool, is_header: bool, is_reference: bool) -> FileEntry {
        FileEntry {
            checked,
            is_header,
            is_reference,
            val_str: ModelRc::new(VecModel::from(vec![SharedString::from("test")])),
            val_int: ModelRc::new(VecModel::from(Vec::new())),
        }
    }

    fn make_model(entries: Vec<FileEntry>) -> ModelRc<FileEntry> {
        ModelRc::new(VecModel::from(entries))
    }

    #[test]
    fn count_checked_empty_model() {
        assert_eq!(count_checked(&make_model(Vec::new())), 0);
    }

    #[test]
    fn count_checked_none_selected() {
        let model = make_model(vec![make_entry(false, false, false), make_entry(false, false, false)]);
        assert_eq!(count_checked(&model), 0);
    }

    #[test]
    fn count_checked_some_selected() {
        let model = make_model(vec![make_entry(true, false, false), make_entry(false, false, false), make_entry(true, false, false)]);
        assert_eq!(count_checked(&model), 2);
    }

    #[test]
    fn count_checked_headers_are_included_if_checked() {
        let model = make_model(vec![make_entry(true, true, false), make_entry(true, false, false)]);
        assert_eq!(count_checked(&model), 2);
    }

    #[test]
    fn toggle_row_checks_unchecked_entry() {
        let model = make_model(vec![make_entry(false, false, false)]);
        toggle_row(&model, 0);
        assert!(model.as_any().downcast_ref::<VecModel<FileEntry>>().unwrap().row_data(0).unwrap().checked);
    }

    #[test]
    fn toggle_row_unchecks_checked_entry() {
        let model = make_model(vec![make_entry(true, false, false)]);
        toggle_row(&model, 0);
        assert!(!model.as_any().downcast_ref::<VecModel<FileEntry>>().unwrap().row_data(0).unwrap().checked);
    }

    #[test]
    fn toggle_row_skips_header() {
        let model = make_model(vec![make_entry(false, true, false)]);
        toggle_row(&model, 0);
        assert!(!model.as_any().downcast_ref::<VecModel<FileEntry>>().unwrap().row_data(0).unwrap().checked);
    }

    #[test]
    fn toggle_row_skips_reference() {
        let model = make_model(vec![make_entry(false, false, true)]);
        toggle_row(&model, 0);
        assert!(!model.as_any().downcast_ref::<VecModel<FileEntry>>().unwrap().row_data(0).unwrap().checked);
    }

    #[test]
    #[should_panic(expected = "toggle_row: index 99 out of bounds")]
    fn toggle_row_out_of_bounds_panics() {
        let model = make_model(vec![make_entry(false, false, false)]);
        toggle_row(&model, 99);
    }

    #[test]
    fn make_file_model_converts_items_correctly() {
        let items = vec![
            FileItem {
                is_header: true,
                is_reference: false,
                val_str: vec!["Group 1".to_string()],
                val_int: Vec::new(),
            },
            FileItem {
                is_header: false,
                is_reference: false,
                val_str: vec!["file.txt".to_string(), "/home/user".to_string()],
                val_int: vec![42],
            },
        ];
        let model = make_file_model(items);
        assert_eq!(model.row_count(), 2);
        let header = model.row_data(0).unwrap();
        assert!(header.is_header);
        assert!(!header.checked);
        let file = model.row_data(1).unwrap();
        assert!(!file.is_header);
        assert!(!file.checked);
        assert_eq!(file.val_str.row_data(0).unwrap().as_str(), "file.txt");
        assert_eq!(file.val_int.row_data(0).unwrap(), 42);
    }
}
