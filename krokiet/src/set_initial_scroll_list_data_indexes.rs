use crate::common::StrDataDuplicateFiles;

fn set_initial_scroll_list_data_indexes(app: &MainWindow) {
    let gs = app.global::<GuiState>();

    // [Parent Idx, File Name Idx, (Additional)Preview Idx]
    // Preview Idx is set only if there is non-standard preview like video

    gs.set_duplicate_data_idx([
        StrDataDuplicateFiles::Path as i32,
        StrDataDuplicateFiles::Name as i32,
        -1,
    ])


}