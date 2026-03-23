use slint::{Model, ModelRc, SharedString, VecModel};

use crate::FileEntry;
use crate::scan_runner::FileItem;

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
    if let Some(vm) = model.as_any().downcast_ref::<VecModel<FileEntry>>() {
        let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
        if let Some(entry) = items.get_mut(index)
            && !entry.is_header
            && !entry.is_reference
        {
            entry.checked = !entry.checked;
        }
        vm.set_vec(items);
    }
}

pub fn count_checked(model: &ModelRc<FileEntry>) -> i32 {
    model
        .as_any()
        .downcast_ref::<VecModel<FileEntry>>()
        .map_or(0, |vm| vm.iter().filter(|e: &FileEntry| e.checked).count() as i32)
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
    fn toggle_row_out_of_bounds_is_noop() {
        let model = make_model(vec![make_entry(false, false, false)]);
        toggle_row(&model, 99);
        assert!(!model.as_any().downcast_ref::<VecModel<FileEntry>>().unwrap().row_data(0).unwrap().checked);
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
