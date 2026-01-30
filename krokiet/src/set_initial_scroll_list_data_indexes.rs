use slint::ComponentHandle;

use crate::common::{
    IntDataVideoOptimizer, StrDataBadExtensions, StrDataBadNames, StrDataBigFiles, StrDataBrokenFiles, StrDataDuplicateFiles, StrDataEmptyFiles, StrDataEmptyFolders,
    StrDataExifRemover, StrDataInvalidSymlinks, StrDataSimilarImages, StrDataSimilarMusic, StrDataSimilarVideos, StrDataTemporaryFiles, StrDataVideoOptimizer,
    create_model_from_model_vec,
};
use crate::{GuiState, MainWindow};

type DataType = [i32; 6];

pub(crate) fn set_initial_scroll_list_data_indexes(app: &MainWindow) {
    let gs = app.global::<GuiState>();

    // [Parent Idx, File Name Idx, (Additional)Preview Idx, Rect Left Idx, Width Idx, Height Idx]
    // Preview Idx is set only if there is non-standard preview like video

    let duplicate_data: DataType = [StrDataDuplicateFiles::Path as i32, StrDataDuplicateFiles::Name as i32, -1, -1, -1, -1];
    gs.set_duplicate_data_idx(create_model_from_model_vec(&duplicate_data));

    let empty_folders_data: DataType = [StrDataEmptyFolders::Path as i32, StrDataEmptyFolders::Name as i32, -1, -1, -1, -1];
    gs.set_empty_folders_data_idx(create_model_from_model_vec(&empty_folders_data));

    let big_files_data: DataType = [StrDataBigFiles::Path as i32, StrDataBigFiles::Name as i32, -1, -1, -1, -1];
    gs.set_big_files_data_idx(create_model_from_model_vec(&big_files_data));

    let empty_files_data: DataType = [StrDataEmptyFiles::Path as i32, StrDataEmptyFiles::Name as i32, -1, -1, -1, -1];
    gs.set_empty_files_data_idx(create_model_from_model_vec(&empty_files_data));

    let temporary_files_data: DataType = [StrDataTemporaryFiles::Path as i32, StrDataTemporaryFiles::Name as i32, -1, -1, -1, -1];
    gs.set_temporary_files_data_idx(create_model_from_model_vec(&temporary_files_data));

    let similar_images_data: DataType = [StrDataSimilarImages::Path as i32, StrDataSimilarImages::Name as i32, -1, -1, -1, -1];
    gs.set_similar_images_data_idx(create_model_from_model_vec(&similar_images_data));

    let similar_videos_data: DataType = [
        StrDataSimilarVideos::Path as i32,
        StrDataSimilarVideos::Name as i32,
        StrDataSimilarVideos::PreviewPath as i32,
        -1,
        -1,
        -1,
    ];
    gs.set_similar_videos_data_idx(create_model_from_model_vec(&similar_videos_data));

    let similar_music_data: DataType = [StrDataSimilarMusic::Path as i32, StrDataSimilarMusic::Name as i32, -1, -1, -1, -1];
    gs.set_similar_music_data_idx(create_model_from_model_vec(&similar_music_data));

    let invalid_symlink_data: DataType = [StrDataInvalidSymlinks::SymlinkFolder as i32, StrDataInvalidSymlinks::SymlinkName as i32, -1, -1, -1, -1];
    gs.set_invalid_symlink_data_idx(create_model_from_model_vec(&invalid_symlink_data));

    let broken_files_data: DataType = [StrDataBrokenFiles::Path as i32, StrDataBrokenFiles::Name as i32, -1, -1, -1, -1];
    gs.set_broken_files_data_idx(create_model_from_model_vec(&broken_files_data));

    let bad_extensions_data: DataType = [StrDataBadExtensions::Path as i32, StrDataBadExtensions::Name as i32, -1, -1, -1, -1];
    gs.set_bad_extensions_data_idx(create_model_from_model_vec(&bad_extensions_data));

    let exif_remover_data: DataType = [StrDataExifRemover::Path as i32, StrDataExifRemover::Name as i32, -1, -1, -1, -1];
    gs.set_exif_remover_data_idx(create_model_from_model_vec(&exif_remover_data));

    let video_optimizer_data: DataType = [
        StrDataVideoOptimizer::Path as i32,
        StrDataVideoOptimizer::Name as i32,
        StrDataVideoOptimizer::PreviewPath as i32,
        IntDataVideoOptimizer::RectLeft as i32,
        IntDataVideoOptimizer::Width as i32,
        IntDataVideoOptimizer::Height as i32,
    ];
    gs.set_video_optimizer_data_idx(create_model_from_model_vec(&video_optimizer_data));

    let bad_names_data: DataType = [StrDataBadNames::Path as i32, StrDataBadNames::Name as i32, -1, -1, -1, -1];
    gs.set_bad_names_data_idx(create_model_from_model_vec(&bad_names_data));
}
