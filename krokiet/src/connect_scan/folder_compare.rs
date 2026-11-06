use std::path::PathBuf;
use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::model::HashType;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::common::{format_time, split_path};
use czkawka_core::tools::folder_compare::{DiffNode, DiffStatus, EntryKind, FolderCompare, FolderCompareMode, FolderCompareParameters, FolderGroup, Info, MatchRole};
use humansize::{BINARY, format_size};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{MAX_INT_DATA_FOLDER_COMPARE, MAX_STR_DATA_FOLDER_COMPARE, split_u64_into_i32s};
use crate::connect_scan::{MessagesData, ScanData, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::{ActiveTab, GuiState, MainWindow, flk};

const MIN_FILES_CHOICES: [u64; 6] = [1, 2, 3, 5, 10, 50];

#[derive(Debug, Clone, PartialEq, Eq)]
enum RowStatus {
    Identical,
    Contained,
    Container { extra_files: u64 },
    OnlyLeft,
    OnlyRight,
    DifferentLeft,
    DifferentRight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FolderRow {
    status: RowStatus,
    path: PathBuf,
    size: u64,
    files: u64,
}

pub(crate) fn scan_folder_compare(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let min_files = MIN_FILES_CHOICES
                .get(sd.folder_compare_min_files_index)
                .copied()
                .unwrap_or(FolderCompareParameters::DEFAULT_MIN_FILES);
            let params = FolderCompareParameters {
                ignore_names: sd.folder_compare_ignore_names,
                find_subsets: sd.folder_compare_find_subsets,
                min_files,
                ..FolderCompareParameters::new(HashType::Blake3)
            };
            let mut tool = FolderCompare::new(params);

            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);
            tool.search(&sd.stop_flag, Some(&sd.progress_sender));
            let (critical, messages) = get_text_messages(&tool);

            let groups = match tool.get_mode() {
                FolderCompareMode::Duplicates => duplicate_group_rows(tool.get_groups()),
                FolderCompareMode::Diff => diff_group_rows(&tool.get_diff().entries),
            };
            let mode = tool.get_mode();
            let info = tool.get_information();
            let wasted_size = info.wasted_size;
            let stopped_search = tool.get_stopped_search();
            sd.shared_models.lock().expect("Mutex poisoned").shared_folder_compare_state = Some(tool);

            let messages_data = MessagesData { critical, messages };

            a.upgrade_in_event_loop(move |app| {
                write_folder_compare_results(&app, groups, messages_data, info, mode, wasted_size, sd, stopped_search);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}

fn write_folder_compare_results(
    app: &MainWindow,
    groups: Vec<Vec<FolderRow>>,
    messages_data: MessagesData,
    info: Info,
    mode: FolderCompareMode,
    wasted_size: u64,
    sd: ScanData,
    stopped_search: bool,
) {
    let scanning_time_str = format_time(info.scanning_time);

    let minimal_rows = ModelRc::new(VecModel::from_slice(&[minimal_group_rows(mode)]));
    let items = Rc::new(VecModel::default());
    for group in groups {
        insert_data_to_model(&items, ModelRc::new(VecModel::default()), minimal_rows.clone(), Some(false));
        for row in group {
            let (data_model_str, data_model_int) = prepare_data_model_folder_compare(&row);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_folder_compare_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        let result_message = match mode {
            FolderCompareMode::Duplicates => flk!(
                "rust_found_folder_compare_groups",
                folders = info.number_of_folders,
                groups = info.number_of_groups,
                size = format_size(wasted_size, BINARY),
                time = scanning_time_str
            ),
            FolderCompareMode::Diff => flk!("rust_found_folder_compare_diff", items_found = info.number_of_different_entries, time = scanning_time_str),
        };
        if !stopped_search && sd.basic_settings.show_notification_on_scan_completion {
            crate::notification_manager::send_scan_completed_notification("Folder Compare", &result_message);
        }
        app.invoke_scan_ended(result_message.into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::FolderCompare);
}

/// Rows a group needs to stay in the results after some of its folders are deleted.
const fn minimal_group_rows(mode: FolderCompareMode) -> i32 {
    match mode {
        FolderCompareMode::Duplicates => 2,
        FolderCompareMode::Diff => 1,
    }
}

fn duplicate_group_rows(groups: &[FolderGroup]) -> Vec<Vec<FolderRow>> {
    groups
        .iter()
        .map(|group| {
            group
                .folders
                .iter()
                .map(|folder| FolderRow {
                    status: match folder.role {
                        MatchRole::Identical => RowStatus::Identical,
                        MatchRole::Contained => RowStatus::Contained,
                        MatchRole::Container => RowStatus::Container { extra_files: folder.extra_files },
                    },
                    path: folder.path.clone(),
                    size: folder.size,
                    files: folder.file_count,
                })
                .collect()
        })
        .collect()
}

/// One group per difference: a lone row for an entry on one side, a left and right row for entries that differ.
fn diff_group_rows(nodes: &[DiffNode]) -> Vec<Vec<FolderRow>> {
    let mut groups = Vec::new();
    for node in nodes {
        collect_diff_groups(node, &mut groups);
    }
    groups
}

fn collect_diff_groups(node: &DiffNode, groups: &mut Vec<Vec<FolderRow>>) {
    match node.status {
        DiffStatus::Identical => {}
        DiffStatus::OnlyLeft => groups.extend(side_row(node, RowStatus::OnlyLeft, true).map(|row| vec![row])),
        DiffStatus::OnlyRight => groups.extend(side_row(node, RowStatus::OnlyRight, false).map(|row| vec![row])),
        DiffStatus::Different | DiffStatus::LeftHasMore | DiffStatus::RightHasMore => {
            if node.children.is_empty() || node.kind != EntryKind::Directory {
                let rows = [side_row(node, RowStatus::DifferentLeft, true), side_row(node, RowStatus::DifferentRight, false)];
                groups.push(rows.into_iter().flatten().collect());
            } else {
                node.children.iter().for_each(|child| collect_diff_groups(child, groups));
            }
        }
    }
}

fn side_row(node: &DiffNode, status: RowStatus, left: bool) -> Option<FolderRow> {
    let (path, size, files) = if left {
        (node.left_path.clone()?, node.left_size, node.left_files)
    } else {
        (node.right_path.clone()?, node.right_size, node.right_files)
    };
    Some(FolderRow { status, path, size, files })
}

fn status_text(status: &RowStatus) -> String {
    match status {
        RowStatus::Identical => flk!("folder_compare_relation_identical"),
        RowStatus::Contained => flk!("folder_compare_relation_contained"),
        RowStatus::Container { extra_files } => format!("{} (+{extra_files})", flk!("folder_compare_relation_container")),
        RowStatus::OnlyLeft => flk!("folder_compare_relation_only_left"),
        RowStatus::OnlyRight => flk!("folder_compare_relation_only_right"),
        RowStatus::DifferentLeft => flk!("folder_compare_relation_different_left"),
        RowStatus::DifferentRight => flk!("folder_compare_relation_different_right"),
    }
}

fn prepare_data_model_folder_compare(row: &FolderRow) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, name) = split_path(&row.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_FOLDER_COMPARE] = [
        status_text(&row.status).into(),
        format_size(row.size, BINARY).into(),
        row.files.to_string().into(),
        name.into(),
        directory.into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let size_split = split_u64_into_i32s(row.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_FOLDER_COMPARE] = [size_split.0, size_split.1, row.files.min(i32::MAX as u64) as i32];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(name: &str, kind: EntryKind, status: DiffStatus, children: Vec<DiffNode>) -> DiffNode {
        DiffNode {
            name: name.to_string(),
            kind,
            status,
            left_path: Some(PathBuf::from(format!("/l/{name}"))),
            right_path: Some(PathBuf::from(format!("/r/{name}"))),
            left_size: 1,
            right_size: 2,
            left_files: 1,
            right_files: 1,
            children,
        }
    }

    fn paths(group: &[FolderRow]) -> Vec<&str> {
        group.iter().map(|row| row.path.to_str().unwrap()).collect()
    }

    #[test]
    fn one_sided_entries_become_single_row_groups() {
        let mut only_left = node("a", EntryKind::File, DiffStatus::OnlyLeft, Vec::new());
        only_left.right_path = None;
        let mut only_right = node("b", EntryKind::Directory, DiffStatus::OnlyRight, Vec::new());
        only_right.left_path = None;

        let groups = diff_group_rows(&[only_left, only_right]);

        assert_eq!(groups.len(), 2);
        assert_eq!(paths(&groups[0]), ["/l/a"]);
        assert_eq!(groups[0][0].status, RowStatus::OnlyLeft);
        assert_eq!(paths(&groups[1]), ["/r/b"]);
        assert_eq!(groups[1][0].status, RowStatus::OnlyRight);
    }

    #[test]
    fn different_files_become_left_and_right_row() {
        let groups = diff_group_rows(&[node("a", EntryKind::File, DiffStatus::Different, Vec::new())]);

        assert_eq!(groups.len(), 1);
        assert_eq!(paths(&groups[0]), ["/l/a", "/r/a"]);
    }

    #[test]
    fn folders_with_differences_are_replaced_by_their_children() {
        let identical = node("same", EntryKind::File, DiffStatus::Identical, Vec::new());
        let changed = node("changed", EntryKind::File, DiffStatus::Different, Vec::new());
        let folder = node("dir", EntryKind::Directory, DiffStatus::Different, vec![identical, changed]);

        let groups = diff_group_rows(&[folder]);

        assert_eq!(groups.len(), 1);
        assert_eq!(paths(&groups[0]), ["/l/changed", "/r/changed"]);
    }

    #[test]
    fn identical_entries_are_skipped() {
        assert!(diff_group_rows(&[node("a", EntryKind::Directory, DiffStatus::Identical, Vec::new())]).is_empty());
    }
}
