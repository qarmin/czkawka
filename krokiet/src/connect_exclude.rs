use std::path::PathBuf;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::connect_directories_changes::add_excluded_paths;
use crate::connect_row_selection::checker::set_number_of_enabled_items;
use crate::connect_row_selection::reset_selection;
use crate::model_operations::remove_single_items_in_groups;
use crate::{Callabler, GuiState, MainWindow, Settings, SingleMainListModel, flk};

pub(crate) fn connect_exclude(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_exclude_items(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);

        let excluded_paths = collect_checked_full_paths(&model, active_tab.get_str_path_idx(), active_tab.get_str_name_idx());
        if excluded_paths.is_empty() {
            return;
        }
        add_excluded_paths(&app.global::<Settings>(), &excluded_paths);

        let kept_items: Vec<SingleMainListModel> = model.iter().filter(|row| row.header_row || !row.checked).collect();
        let cleaned = remove_single_items_in_groups(kept_items, active_tab.get_is_header_mode());

        active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
        reset_selection(&app, active_tab, true);
        // Every checked row was just removed, so nothing stays selected
        set_number_of_enabled_items(&app, active_tab, 0);
        app.global::<GuiState>().set_info_text(flk!("rust_exclude_summary", items = excluded_paths.len()).into());
    });
}

fn collect_checked_full_paths(model: &ModelRc<SingleMainListModel>, path_idx: usize, name_idx: usize) -> Vec<String> {
    model
        .iter()
        .filter(|row| row.checked && !row.header_row)
        .map(|row| {
            let values = row.val_str.iter().collect::<Vec<_>>();
            let path = values.get(path_idx).unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={values:?}"));
            let name = values.get(name_idx).unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={values:?}"));
            PathBuf::from(path.as_str()).join(name.as_str()).to_string_lossy().to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::MAIN_SEPARATOR;

    use crate::common::create_model_from_model_vec;
    use crate::connect_exclude::collect_checked_full_paths;
    use crate::test_common::get_model_vec;

    #[test]
    fn collect_checked_full_paths_takes_only_checked_non_header_rows() {
        let mut model = get_model_vec(4);
        model[0].header_row = true;
        model[0].val_str = create_model_from_model_vec(&["".into(), "header".into(), "/group".into()]);
        model[1].checked = true;
        model[1].val_str = create_model_from_model_vec(&["".into(), "checked.jpg".into(), "/home/pictures".into()]);
        model[2].checked = false;
        model[2].val_str = create_model_from_model_vec(&["".into(), "unchecked.jpg".into(), "/home/pictures".into()]);
        model[3].checked = true;
        model[3].val_str = create_model_from_model_vec(&["".into(), "other.jpg".into(), "/home/other".into()]);
        let model = create_model_from_model_vec(&model);

        let paths = collect_checked_full_paths(&model, 2, 1);

        assert_eq!(
            paths,
            vec![
                format!("{MAIN_SEPARATOR}home{MAIN_SEPARATOR}pictures{MAIN_SEPARATOR}checked.jpg"),
                format!("{MAIN_SEPARATOR}home{MAIN_SEPARATOR}other{MAIN_SEPARATOR}other.jpg")
            ]
        );
    }

    #[test]
    fn collect_checked_full_paths_returns_empty_when_nothing_checked() {
        let mut model = get_model_vec(2);
        model[0].val_str = create_model_from_model_vec(&["".into(), "a.jpg".into(), "/home".into()]);
        model[1].val_str = create_model_from_model_vec(&["".into(), "b.jpg".into(), "/home".into()]);
        let model = create_model_from_model_vec(&model);

        assert!(collect_checked_full_paths(&model, 2, 1).is_empty());
    }
}
