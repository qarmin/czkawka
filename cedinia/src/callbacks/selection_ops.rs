use slint::{Model, ModelRc, VecModel};

use crate::FileEntry;
use crate::common::{INT_IDX_SIZE_HI, INT_IDX_SIZE_LO, IntDataSimilarImages, STR_IDX_NAME, STR_IDX_PATH};

pub(crate) fn vm_of(model: &ModelRc<FileEntry>) -> &VecModel<FileEntry> {
    model.as_any().downcast_ref::<VecModel<FileEntry>>().expect("FileEntry model must be backed by a VecModel")
}

pub(crate) fn size_from_entry(e: &FileEntry) -> u64 {
    let hi = get_val_int(e, INT_IDX_SIZE_HI) as u64;
    let lo = get_val_int(e, INT_IDX_SIZE_LO) as u64;
    (hi << 32) | (lo & 0xFFFF_FFFF)
}

pub(crate) fn get_val_str(e: &FileEntry, idx: usize) -> String {
    e.val_str
        .row_data(idx)
        .unwrap_or_else(|| panic!("get_val_str: val_str[{idx}] missing, full val_str={:?}", e.val_str.iter().collect::<Vec<_>>()))
        .to_string()
}

pub(crate) fn get_val_int(e: &FileEntry, idx: usize) -> i32 {
    e.val_int
        .row_data(idx)
        .unwrap_or_else(|| panic!("get_val_int: val_int[{idx}] missing, full val_int={:?}", e.val_int.iter().collect::<Vec<_>>()))
}

pub(crate) fn full_path_of(e: &FileEntry) -> String {
    let name = get_val_str(e, STR_IDX_NAME);
    let path = get_val_str(e, STR_IDX_PATH);
    if path.is_empty() { name } else { format!("{path}/{name}") }
}

pub(crate) fn set_all_checked(model: &ModelRc<FileEntry>, state: bool) {
    let vm = vm_of(model);
    let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
    for e in &mut items {
        if !e.is_header && !e.is_reference {
            e.checked = state;
        }
    }
    vm.set_vec(items);
}

pub(crate) fn select_except_one_per_group(model: &ModelRc<FileEntry>, select: bool) {
    let vm = vm_of(model);
    let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
    let has_headers = items.iter().any(|e| e.is_header);

    if !has_headers {
        for e in &mut items {
            if !e.is_header {
                e.checked = select;
            }
        }
        vm.set_vec(items);
        return;
    }

    if select {
        let mut first_non_ref_in_group = false;
        for e in &mut items {
            if e.is_header {
                first_non_ref_in_group = true;
                continue;
            }
            if e.is_reference {
                continue;
            }
            e.checked = !std::mem::take(&mut first_non_ref_in_group);
        }
    } else {
        let mut i = 0;
        while i < items.len() {
            if items[i].is_header {
                let group_end = items[i + 1..].iter().position(|e| e.is_header).map_or(items.len(), |p| i + 1 + p);
                let checked_count = items[i + 1..group_end].iter().filter(|e| e.checked).count();
                if checked_count >= 2 {
                    let mut kept = false;
                    for j in i + 1..group_end {
                        if items[j].checked {
                            if kept {
                                items[j].checked = false;
                            } else {
                                kept = true;
                            }
                        }
                    }
                }
                i = group_end;
                continue;
            }
            i += 1;
        }
    }

    vm.set_vec(items);
}

pub(crate) fn select_largest_per_group(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, true, true);
}

pub(crate) fn select_all_except_largest(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, true, false);
}

pub(crate) fn select_smallest_per_group(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, false, true);
}

pub(crate) fn select_all_except_smallest(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, false, false);
}

fn select_by_size_per_group(model: &ModelRc<FileEntry>, largest: bool, select_target: bool) {
    let vm = vm_of(model);
    let mut items: Vec<FileEntry> = vm.iter().collect();

    let mut i = 0;
    while i < items.len() {
        if items[i].is_header {
            let group_end = items[i + 1..].iter().position(|e| e.is_header).map_or(items.len(), |p| i + 1 + p);

            let target_idx = if largest {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .max_by_key(|(_, e)| size_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            } else {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .min_by_key(|(_, e)| size_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            };

            for j in i + 1..group_end {
                if items[j].is_reference {
                    continue;
                }
                let is_target = target_idx == Some(j);
                items[j].checked = if select_target { is_target } else { !is_target };
            }

            i = group_end;
            continue;
        }
        i += 1;
    }

    vm.set_vec(items);
}

fn resolution_from_entry(e: &FileEntry) -> u64 {
    let w = get_val_int(e, IntDataSimilarImages::Width as usize) as u64;
    let h = get_val_int(e, IntDataSimilarImages::Height as usize) as u64;
    w * h
}

fn select_by_resolution_per_group(model: &ModelRc<FileEntry>, highest: bool, select_target: bool) {
    let vm = vm_of(model);
    let mut items: Vec<FileEntry> = vm.iter().collect();

    let mut i = 0;
    while i < items.len() {
        if items[i].is_header {
            let group_end = items[i + 1..].iter().position(|e| e.is_header).map_or(items.len(), |p| i + 1 + p);

            let target_idx = if highest {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .max_by_key(|(_, e)| resolution_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            } else {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .min_by_key(|(_, e)| resolution_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            };

            for j in i + 1..group_end {
                if items[j].is_reference {
                    continue;
                }
                let is_target = target_idx == Some(j);
                items[j].checked = if select_target { is_target } else { !is_target };
            }

            i = group_end;
            continue;
        }
        i += 1;
    }
    vm.set_vec(items);
}

pub(crate) fn select_highest_resolution_per_group(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, true, true);
}
pub(crate) fn select_all_except_highest_resolution(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, true, false);
}
pub(crate) fn select_lowest_resolution_per_group(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, false, true);
}
pub(crate) fn select_all_except_lowest_resolution(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, false, false);
}
