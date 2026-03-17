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
