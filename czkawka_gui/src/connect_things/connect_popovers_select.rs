use czkawka_core::common::items::new_excluded_item;
use czkawka_core::common::regex_check;
use gtk4::prelude::*;
use gtk4::TreeIter;
use log::error;
use regex::Regex;

use crate::gui_structs::common_tree_view::SubView;
use crate::help_functions::{MAIN_ROW_COLOR, SAME_SIZE_ROW_COLOR, change_dimension_to_krotka, get_full_name_from_path_name};
use crate::helpers::model_iter::iter_list;

pub(crate) fn exec_select_all(sv: &SubView) {
    let model = sv.get_model();
    if let Some(mut iter) = model.iter_first() {
        if let Some(column_header) = sv.nb_object.column_header {
            loop {
                if !model.get::<bool>(&iter, column_header) {
                    model.set_value(&iter, sv.nb_object.column_selection as u32, &true.to_value());
                }
                if !model.iter_next(&mut iter) {
                    break;
                }
            }
        } else {
            loop {
                model.set_value(&iter, sv.nb_object.column_selection as u32, &true.to_value());
                if !model.iter_next(&mut iter) {
                    break;
                }
            }
        }
    }
}

pub(crate) fn exec_unselect_all(sv: &SubView) {
    let model = sv.get_model();
    iter_list(&model, |m, i| {
        m.set_value(i, sv.nb_object.column_selection as u32, &false.to_value());
    });
}

pub(crate) fn exec_reverse(sv: &SubView) {
    let model = sv.get_model();
    if let Some(mut iter) = model.iter_first() {
        if let Some(column_header) = sv.nb_object.column_header {
            loop {
                if !model.get::<bool>(&iter, column_header) {
                    let cur: bool = model.get::<bool>(&iter, sv.nb_object.column_selection);
                    model.set_value(&iter, sv.nb_object.column_selection as u32, &(!cur).to_value());
                }
                if !model.iter_next(&mut iter) {
                    break;
                }
            }
        } else {
            loop {
                let cur: bool = model.get::<bool>(&iter, sv.nb_object.column_selection);
                model.set_value(&iter, sv.nb_object.column_selection as u32, &(!cur).to_value());
                if !model.iter_next(&mut iter) {
                    break;
                }
            }
        }
    }
}

/// `except_longest=true`  → keep the item with SHORTEST path, select all others ("select all except shortest path").
/// `except_longest=false` → keep the item with LONGEST path, select all others ("select all except longest path").
pub(crate) fn exec_all_except_longest_shortest_path(sv: &SubView, except_longest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("AES/AEL needs header column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut path_extreme: usize = if except_longest { usize::MAX } else { 0 };

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let path_length = model.get::<String>(&iter, sv.nb_object.column_path).len();
                if except_longest {
                    if path_length < path_extreme {
                        path_extreme = path_length;
                        used_index = Some(current_index);
                    }
                } else if path_length > path_extreme {
                    path_extreme = path_length;
                    used_index = Some(current_index);
                }
                current_index += 1;
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if let Some(used_index) = used_index {
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(index != used_index).to_value());
                }
            }
            if end {
                break;
            }
        }
    }
}

/// `except_oldest=true`  → keep oldest, select all others.
/// `except_oldest=false` → keep newest, select all others.
pub(crate) fn exec_all_except_oldest_newest(sv: &SubView, except_oldest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("AEO/AEN needs header column");
    let column_modification_as_secs = sv.nb_object.column_modification_as_secs.expect("AEO/AEN needs modification column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut modification_extreme: u64 = if except_oldest { u64::MAX } else { 0 };
            let mut file_length: usize = 0;

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let modification = model.get::<u64>(&iter, column_modification_as_secs);
                let current_file_length = model.get::<String>(&iter, sv.nb_object.column_name).len();
                if except_oldest {
                    if modification < modification_extreme || (modification == modification_extreme && current_file_length < file_length) {
                        file_length = current_file_length;
                        modification_extreme = modification;
                        used_index = Some(current_index);
                    }
                } else if modification > modification_extreme || (modification == modification_extreme && current_file_length < file_length) {
                    file_length = current_file_length;
                    modification_extreme = modification;
                    used_index = Some(current_index);
                }
                current_index += 1;
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if let Some(used_index) = used_index {
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(index != used_index).to_value());
                }
            }
            if end {
                break;
            }
        }
    }
}

/// `check_oldest=true`  → mark the one oldest file per group as selected.
/// `check_oldest=false` → mark the one newest file per group as selected.
pub(crate) fn exec_one_oldest_newest(sv: &SubView, check_oldest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("OO/ON needs header column");
    let column_modification_as_secs = sv.nb_object.column_modification_as_secs.expect("OO/ON needs modification column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut modification_extreme: u64 = if check_oldest { u64::MAX } else { 0 };
            let mut file_length: usize = 0;

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let modification = model.get::<u64>(&iter, column_modification_as_secs);
                let current_file_length = model.get::<String>(&iter, sv.nb_object.column_name).len();
                if check_oldest {
                    if modification < modification_extreme || (modification == modification_extreme && current_file_length > file_length) {
                        file_length = current_file_length;
                        modification_extreme = modification;
                        used_index = Some(current_index);
                    }
                } else if modification > modification_extreme || (modification == modification_extreme && current_file_length > file_length) {
                    file_length = current_file_length;
                    modification_extreme = modification;
                    used_index = Some(current_index);
                }
                current_index += 1;
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if let Some(used_index) = used_index {
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(index == used_index).to_value());
                }
            }
            if end {
                break;
            }
        }
    }
}

/// Like `exec_one_oldest_newest` but skips groups where file sizes differ.
pub(crate) fn exec_one_oldest_newest_same_size(sv: &SubView, check_oldest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("OO/ON same size needs header column");
    let column_modification_as_secs = sv.nb_object.column_modification_as_secs.expect("OO/ON same size needs modification column");
    let column_size_as_bytes = sv.nb_object.column_size_as_bytes.expect("OO/ON same size needs size column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut modification_extreme: u64 = if check_oldest { u64::MAX } else { 0 };
            let mut file_length: usize = 0;
            let mut first_size: Option<u64> = None;
            let mut sizes_match = true;

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let size = model.get::<u64>(&iter, column_size_as_bytes);
                match first_size {
                    None => first_size = Some(size),
                    Some(s) if s != size => sizes_match = false,
                    _ => {}
                }
                let modification = model.get::<u64>(&iter, column_modification_as_secs);
                let current_file_length = model.get::<String>(&iter, sv.nb_object.column_name).len();
                if check_oldest {
                    if modification < modification_extreme || (modification == modification_extreme && current_file_length > file_length) {
                        file_length = current_file_length;
                        modification_extreme = modification;
                        used_index = Some(current_index);
                    }
                } else if modification > modification_extreme || (modification == modification_extreme && current_file_length > file_length) {
                    file_length = current_file_length;
                    modification_extreme = modification;
                    used_index = Some(current_index);
                }
                current_index += 1;
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if sizes_match {
                if let Some(used_index) = used_index {
                    for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                        model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(index == used_index).to_value());
                    }
                }
            }
            if end {
                break;
            }
        }
    }
}

/// Like `exec_one_oldest_newest` but skips groups where file paths differ.
pub(crate) fn exec_one_oldest_newest_same_path(sv: &SubView, check_oldest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("OO/ON same path needs header column");
    let column_modification_as_secs = sv.nb_object.column_modification_as_secs.expect("OO/ON same path needs modification column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut modification_extreme: u64 = if check_oldest { u64::MAX } else { 0 };
            let mut file_length: usize = 0;
            let mut first_path: Option<String> = None;
            let mut paths_match = true;

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let path = model.get::<String>(&iter, sv.nb_object.column_path);
                match &first_path {
                    None => first_path = Some(path.clone()),
                    Some(p) if *p != path => paths_match = false,
                    _ => {}
                }
                let modification = model.get::<u64>(&iter, column_modification_as_secs);
                let current_file_length = model.get::<String>(&iter, sv.nb_object.column_name).len();
                if check_oldest {
                    if modification < modification_extreme || (modification == modification_extreme && current_file_length > file_length) {
                        file_length = current_file_length;
                        modification_extreme = modification;
                        used_index = Some(current_index);
                    }
                } else if modification > modification_extreme || (modification == modification_extreme && current_file_length > file_length) {
                    file_length = current_file_length;
                    modification_extreme = modification;
                    used_index = Some(current_index);
                }
                current_index += 1;
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if paths_match {
                if let Some(used_index) = used_index {
                    for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                        model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(index == used_index).to_value());
                    }
                }
            }
            if end {
                break;
            }
        }
    }
}

/// Among files in a group with the longest/shortest path, mark the oldest/newest as selected.
/// `check_longest=true`  → among files with the LONGEST path.
/// `check_longest=false` → among files with the SHORTEST path.
pub(crate) fn exec_one_longest_shortest_path_oldest_newest(sv: &SubView, check_longest: bool, check_oldest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("path+date select needs header column");
    let column_modification_as_secs = sv.nb_object.column_modification_as_secs.expect("path+date select needs modification column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut entries: Vec<(TreeIter, usize, u64, usize)> = Vec::new();

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                let path_len = model.get::<String>(&iter, sv.nb_object.column_path).len();
                let modification = model.get::<u64>(&iter, column_modification_as_secs);
                let name_len = model.get::<String>(&iter, sv.nb_object.column_name).len();
                entries.push((iter, path_len, modification, name_len));
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if !entries.is_empty() {
                let extreme_path_len = if check_longest {
                    entries.iter().map(|e| e.1).max().unwrap_or(0)
                } else {
                    entries.iter().map(|e| e.1).min().unwrap_or(usize::MAX)
                };

                let mut best_index: Option<usize> = None;
                let mut best_modification: u64 = if check_oldest { u64::MAX } else { 0 };
                let mut best_name_len: usize = 0;

                for (idx, (_, path_len, modification, name_len)) in entries.iter().enumerate() {
                    if *path_len != extreme_path_len {
                        continue;
                    }
                    if check_oldest {
                        if *modification < best_modification || (*modification == best_modification && *name_len > best_name_len) {
                            best_modification = *modification;
                            best_name_len = *name_len;
                            best_index = Some(idx);
                        }
                    } else if *modification > best_modification || (*modification == best_modification && *name_len > best_name_len) {
                        best_modification = *modification;
                        best_name_len = *name_len;
                        best_index = Some(idx);
                    }
                }

                if let Some(best_index) = best_index {
                    for (idx, (tree_iter, _, _, _)) in entries.iter().enumerate() {
                        model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(idx == best_index).to_value());
                    }
                }
            }

            if end {
                break;
            }
        }
    }
}

/// Selects the one file with the longest/shortest path per group.
pub(crate) fn exec_one_longest_shortest_path(sv: &SubView, check_longest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("select-one-by-path needs header column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut entries: Vec<(TreeIter, usize)> = Vec::new();

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                let path_len = model.get::<String>(&iter, sv.nb_object.column_path).len();
                entries.push((iter, path_len));
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if let Some(best) = extreme_path_index(&entries, check_longest) {
                for (idx, (ti, _)) in entries.iter().enumerate() {
                    model.set_value(ti, sv.nb_object.column_selection as u32, &(idx == best).to_value());
                }
            }
            if end {
                break;
            }
        }
    }
}

/// Selects the one file with the longest/shortest path per group, skipping groups where file sizes differ.
pub(crate) fn exec_one_longest_shortest_path_same_size(sv: &SubView, check_longest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("select-one-by-path(size) needs header column");
    let column_size_as_bytes = sv.nb_object.column_size_as_bytes.expect("select-one-by-path(size) needs size column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut entries: Vec<(TreeIter, usize)> = Vec::new();
            let mut first_size: Option<u64> = None;
            let mut sizes_match = true;

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                let path_len = model.get::<String>(&iter, sv.nb_object.column_path).len();
                let size = model.get::<u64>(&iter, column_size_as_bytes);
                match first_size {
                    None => first_size = Some(size),
                    Some(s) if s != size => sizes_match = false,
                    _ => {}
                }
                entries.push((iter, path_len));
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if sizes_match {
                if let Some(best) = extreme_path_index(&entries, check_longest) {
                    for (idx, (ti, _)) in entries.iter().enumerate() {
                        model.set_value(ti, sv.nb_object.column_selection as u32, &(idx == best).to_value());
                    }
                }
            }
            if end {
                break;
            }
        }
    }
}

fn extreme_path_index(entries: &[(TreeIter, usize)], check_longest: bool) -> Option<usize> {
    let extreme = if check_longest {
        entries.iter().map(|e| e.1).max()?
    } else {
        entries.iter().map(|e| e.1).min()?
    };
    entries.iter().position(|e| e.1 == extreme)
}

/// Highlight groups where all files have the same size in a distinct color.
pub(crate) fn exec_mark_same_size(sv: &SubView) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("mark same size needs header column");
    let column_size_as_bytes = sv.nb_object.column_size_as_bytes.expect("mark same size needs size column");
    let column_color = sv.nb_object.column_color.expect("mark same size needs color column") as u32;

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut first_size: Option<u64> = None;
            let mut sizes_match = true;

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let size = model.get::<u64>(&iter, column_size_as_bytes);
                match first_size {
                    None => first_size = Some(size),
                    Some(s) if s != size => sizes_match = false,
                    _ => {}
                }
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            let highlight = if sizes_match && !tree_iter_array.is_empty() { SAME_SIZE_ROW_COLOR } else { MAIN_ROW_COLOR };
            for tree_iter in &tree_iter_array {
                model.set_value(tree_iter, column_color, &highlight.to_value());
            }
            if end {
                break;
            }
        }
    }
}

/// `except_biggest=true`  → keep the item with the BIGGEST size/resolution, select all others.
/// `except_biggest=false` → keep the item with the SMALLEST size/resolution, select all others.
pub(crate) fn exec_all_except_biggest_smallest(sv: &SubView, except_biggest: bool) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("AEB/AES needs header column");
    let column_size_as_bytes = sv.nb_object.column_size_as_bytes.expect("AEB/AES needs size column");

    if let Some(mut iter) = model.iter_first() {
        let mut end = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut size_extreme: u64 = if except_biggest { 0 } else { u64::MAX };
            let mut pixels_extreme: u64 = if except_biggest { 0 } else { u64::MAX };

            loop {
                if model.get::<bool>(&iter, column_header) {
                    if !model.iter_next(&mut iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter);
                let size_as_bytes = model.get::<u64>(&iter, column_size_as_bytes);

                if let Some(column_dimensions) = sv.nb_object.column_dimensions {
                    let dim_str = model.get::<String>(&iter, column_dimensions);
                    let dim = change_dimension_to_krotka(&dim_str);
                    let pixels = dim.0 * dim.1;
                    if except_biggest {
                        if pixels > pixels_extreme || (pixels == pixels_extreme && size_as_bytes > size_extreme) {
                            pixels_extreme = pixels;
                            size_extreme = size_as_bytes;
                            used_index = Some(current_index);
                        }
                    } else if pixels < pixels_extreme || (pixels == pixels_extreme && size_as_bytes < size_extreme) {
                        pixels_extreme = pixels;
                        size_extreme = size_as_bytes;
                        used_index = Some(current_index);
                    }
                } else if except_biggest {
                    if size_as_bytes > size_extreme {
                        size_extreme = size_as_bytes;
                        used_index = Some(current_index);
                    }
                } else if size_as_bytes < size_extreme {
                    size_extreme = size_as_bytes;
                    used_index = Some(current_index);
                }
                current_index += 1;
                if !model.iter_next(&mut iter) {
                    end = true;
                    break;
                }
            }

            if let Some(used_index) = used_index {
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    model.set_value(tree_iter, sv.nb_object.column_selection as u32, &(index != used_index).to_value());
                }
            }
            if end {
                break;
            }
        }
    }
}

/// Execute custom select/unselect with filter parameters read directly from the dialog.
pub(crate) fn exec_custom_filter(
    sv: &SubView,
    select_things: bool,
    name_text: &str,
    path_text: &str,
    regex_text: &str,
    check_name: bool,
    check_path: bool,
    check_regex: bool,
    case_sensitive: bool,
    check_all_selected: bool,
) {
    if !check_path && !check_name && !check_regex {
        return;
    }

    let name_wildcard = {
        let s = name_text.trim().to_string();
        #[cfg(target_family = "windows")]
        let s = s.replace("/", "\\");
        s
    };
    let path_wildcard = {
        let s = path_text.trim().to_string();
        #[cfg(target_family = "windows")]
        let s = s.replace("/", "\\");
        s
    };
    let regex_wildcard = regex_text.trim().to_string();

    let name_wildcard_excluded = new_excluded_item(&name_wildcard);
    let name_wildcard_lowercase_excluded = new_excluded_item(&name_wildcard.to_lowercase());
    let path_wildcard_excluded = new_excluded_item(&path_wildcard);
    let path_wildcard_lowercase_excluded = new_excluded_item(&path_wildcard.to_lowercase());

    let compiled_regex = if check_regex {
        match Regex::new(&regex_wildcard) {
            Ok(r) => r,
            Err(_) => {
                error!("Custom filter regex failed to compile.");
                return;
            }
        }
    } else {
        #[expect(clippy::trivial_regex)]
        Regex::new("").expect("Empty regex should compile properly.")
    };

    let model = sv.get_model();
    let Some(mut iter) = model.iter_first() else {
        return;
    };
    let using_reference_folders = sv.nb_object.column_header.is_some_and(|e| model.get::<bool>(&iter, e)) && !model.get::<String>(&iter, sv.nb_object.column_name).is_empty();

    let mut number_of_all_things = 0;
    let mut number_of_already_selected_things = 0;
    let mut vec_of_iters: Vec<TreeIter> = Vec::new();

    loop {
        if let Some(column_header) = sv.nb_object.column_header
            && model.get::<bool>(&iter, column_header)
        {
            if select_things {
                if !using_reference_folders && check_all_selected && (number_of_all_things - number_of_already_selected_things == vec_of_iters.len()) {
                    vec_of_iters.pop();
                }
                for iter in vec_of_iters {
                    model.set_value(&iter, sv.nb_object.column_selection as u32, &true.to_value());
                }
            } else {
                for iter in vec_of_iters {
                    model.set_value(&iter, sv.nb_object.column_selection as u32, &false.to_value());
                }
            }
            if !model.iter_next(&mut iter) {
                break;
            }
            number_of_all_things = 0;
            number_of_already_selected_things = 0;
            vec_of_iters = Vec::new();
            continue;
        }

        let is_selected = model.get::<bool>(&iter, sv.nb_object.column_selection);
        let path = model.get::<String>(&iter, sv.nb_object.column_path);
        let name = model.get::<String>(&iter, sv.nb_object.column_name);
        let path_and_name = get_full_name_from_path_name(&path, &name);
        let mut need_to_change = false;

        number_of_all_things += 1;
        if check_regex && compiled_regex.find(&path_and_name).is_some() {
            need_to_change = true;
        } else {
            if check_name {
                if case_sensitive {
                    if regex_check(&name_wildcard_excluded, &name) {
                        need_to_change = true;
                    }
                } else if regex_check(&name_wildcard_lowercase_excluded, &name.to_lowercase()) {
                    need_to_change = true;
                }
            }
            if check_path {
                if case_sensitive {
                    if regex_check(&path_wildcard_excluded, &path) {
                        need_to_change = true;
                    }
                } else if regex_check(&path_wildcard_lowercase_excluded, &path.to_lowercase()) {
                    need_to_change = true;
                }
            }
        }

        if select_things {
            if is_selected {
                number_of_already_selected_things += 1;
            } else if need_to_change {
                vec_of_iters.push(iter);
            }
        } else if need_to_change {
            vec_of_iters.push(iter);
        }

        if !model.iter_next(&mut iter) {
            if select_things {
                if !using_reference_folders && check_all_selected && (number_of_all_things - number_of_already_selected_things == vec_of_iters.len()) {
                    vec_of_iters.pop();
                }
                for iter in vec_of_iters {
                    model.set_value(&iter, sv.nb_object.column_selection as u32, &true.to_value());
                }
            } else {
                for iter in vec_of_iters {
                    model.set_value(&iter, sv.nb_object.column_selection as u32, &false.to_value());
                }
            }
            break;
        }
    }
}
