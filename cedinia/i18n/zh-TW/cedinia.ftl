# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = 重複項目
tool_empty_folders = 空資料夾
tool_similar_images = 相似影像
tool_empty_files = 空檔案
tool_temporary_files = 臨時檔案
tool_big_files = 最大檔案。
tool_broken_files = 損壞的檔案
tool_bad_extensions = 錯誤的副檔名
tool_same_music = 音樂重複
tool_bad_names = 糟糕的名字
tool_exif_remover = EXIF 數據
tool_directories = 目錄
tool_settings = 設定
# Home screen tool card descriptions
home_dup_description = 尋找內容相同的檔案。
home_empty_folders_description = 沒有內容的目錄。
home_similar_images_description = 尋找視覺上相似的照片。
home_empty_files_description = 檔案大小為零。
home_temp_files_description = 臨時檔案與快取檔案
home_big_files_description = 磁碟上最大的/最小的文件。
home_broken_files_description = PDF文件、音訊、圖片、檔案。
home_bad_extensions_description = 具有無效檔案擴充名的檔案。
home_same_music_description = 依標籤搜尋的相似音訊檔案。
home_bad_names_description = 檔案名稱含有問題字元。
home_exif_description = 帶有 EXIF 原始數據的圖片。
# Results list
scanning = 掃描中...
stopping = 暫停...
no_results = 沒有找到任何結果。
press_start = 請按「開始」按鈕以進行掃描。
select_label = 精選。.
deselect_label = Desel.
list_label = 清單。
gallery_label = 加拉太書。.
# Selection popup
selection_popup_title = 選擇
select_all = 選擇全部
select_except_one = 請選擇所有選項，除了其中一個。
select_except_largest = 請選擇所有項目，除了最大值。
select_except_smallest = 選擇除最小以外的全部影像
select_largest = 選擇最大值。
select_smallest = 選擇最小值。
select_except_highest_res = 請選擇所有選項，除了最高解析度之外。
select_except_lowest_res = 請選擇所有選項，除了解析度最低的那一個。
select_highest_res = 選擇最高解析度。
select_lowest_res = 選擇最低解析度。
invert_selection = 反向選擇
close = 關閉
# Deselection popup
deselection_popup_title = 取消選擇。
deselect_all = 取消全部
deselect_except_one = 取消所有選項，只保留一個。
# Confirm popup
cancel = 取消
delete = 刪除
rename = 重新命名
# Delete errors popup
delete_errors_title = 刪除部分檔案失敗：
ok = 好的。
# Stopping overlay
stopping_overlay_title = 暫停。
stopping_overlay_body = 
        掃描完成中...
        請稍候。.
# Permission popup
permission_title = 檔案存取權限
permission_body = 為了掃描檔案，應用程式需要存取裝置儲存空間。若沒有此授權，將無法進行掃描。.
grant = 補助金；授予；（人名）格兰特。
no_permission_scan_warning = 無法存取檔案 - 授權進行掃描。
# Settings screen tabs
settings_tab_general = 一般
settings_tab_tools = 工具
settings_tab_diagnostics = 資訊。
# Settings - General tab
settings_use_cache = 使用快取
settings_use_cache_desc = 加快後續掃描速度（例如：雜湊碼/圖片）。
settings_ignore_hidden = 忽略隱藏檔案。
settings_ignore_hidden_desc = 以「.」開頭的檔案和資料夾。
settings_show_notification = 請在掃描完成時通知。
settings_show_notification_desc = 在掃描完成時，顯示系統通知。
settings_notify_only_background = 僅在背景時顯示。
settings_notify_only_background_desc = 如果應用程式視窗已顯示，則跳過通知。
notifications_disabled_banner = 通知已禁用。
notifications_enable_button = 啟用
settings_scan_label = 掃描
settings_filters_label = 篩選器 (部分工具)
settings_min_file_size = 最小文件大小。
settings_max_file_size = 最大檔案大小。
settings_language = 語言
settings_language_restart = 需要重新啟動應用程式。
settings_common_label = 常用設定
settings_excluded_items = 排除項目 (全域模式，以逗號分隔)
settings_excluded_items_placeholder = 例如：*.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = 允許的檔案類型 (若為空白，則代表所有類型)
settings_allowed_extensions_placeholder = 例如：jpg、png、mp4。
settings_excluded_extensions = 排除的擴展功能。
settings_excluded_extensions_placeholder = 例如：bak、tmp、log。
# Settings - Tools section labels
settings_duplicates_header = 重複項目
settings_check_method_label = 比較方法
settings_check_method = 方法
settings_hash_type_label = 雜湊類型
settings_hash_type = 雜湊類型
settings_hash_type_desc = Blake3 – 這是推薦的選項，CRC32 存在極小的誤判機率。
settings_similar_images_header = 相似圖片
settings_similarity_preset = 相似度閾值
settings_similarity_desc = 極高 = 僅接近完全相同。
settings_hash_size = 雜湊大小
settings_hash_size_desc = 較大的尺寸，雖然能減少誤判的機率，但同時也可能找到的相似圖片數量較少。
settings_hash_alg = 哈希算法
settings_image_filter = 調整過濾器大小。
settings_ignore_same_size = 忽略尺寸相同的圖片。
settings_gallery_image_fit_cover = 相簿：裁剪至正方形。
settings_gallery_image_fit_cover_desc = 填滿圖格；禁用此選項以保留原始比例。
settings_big_files_header = 檔案大小：巨 كبير
settings_search_mode = 搜尋模式
settings_file_count = 文件數量
settings_same_music_header = 音樂複製品
settings_music_check_method = 比較模式
settings_music_compare_tags_label = 相關標籤
settings_music_title = 標題
settings_music_artist = 藝人
settings_music_year = 年份
settings_music_length = 長度
settings_music_genre = 類型
settings_music_bitrate = 位元率
settings_music_approx = 近似標籤比較
settings_broken_files_header = 損毀檔案
settings_broken_files_note = 資源佔用較高的掃描操作。如需最佳效能，請在桌面上使用 Krokiet。.
settings_broken_files_types_label = 已選取的類型。
settings_broken_audio = 音訊
settings_broken_pdf = PDF 文件。
settings_broken_archive = 歸檔
settings_broken_image = 影像
settings_bad_names_header = 不良名稱
settings_bad_names_checks_label = 支票
settings_bad_names_uppercase_ext = 大寫延伸
settings_bad_names_emoji = 表情符號在名字裡
settings_bad_names_space = 開頭/結尾的空格。
settings_bad_names_non_ascii = 非ASCII字元
settings_bad_names_duplicated = 重複字元
# Settings - Diagnostics tab
diagnostics_header = 診斷儀器
diagnostics_thumbnails = 縮圖快取。
diagnostics_app_cache = 應用程式快取。
diagnostics_refresh = 重新整理
diagnostics_clear_thumbnails = 清晰的縮圖。
diagnostics_open_thumbnails_folder = 開啟資料夾
diagnostics_clear_cache = 清除快取。
diagnostics_open_cache_folder = 開啟資料夾
diagnostics_collect_test = 檔案存取測試
diagnostics_collect_test_desc = 請確認可以存取的檔案數量。
diagnostics_collect_test_run = 執行。
diagnostics_collect_test_stop = 停止
collect_test_cancelled = 由用戶停止。
diag_confirm_clear_thumbnails = 是否清除所有縮圖快取？
diag_confirm_clear_cache = 是否清除所有應用程式快取？
about_repo = 儲存庫
about_translate = 翻譯
about_donate = 支持
# Collect-test result popup
collect_test_title = 測試結果
collect_test_volumes = 篇數：
collect_test_folders = 資料夾：
collect_test_files = 檔案：
collect_test_time = 時間：
# Licenses
licenses_label = 許可證
third_party_licenses = 第三方授權條款。
licenses_popup_title = 第三方授權條款
# Directories screen
directories_include_header = 包含。
directories_included = 包含於內。
directories_exclude_header = 排除。
directories_excluded_header = 已排除。
directories_add = 請包含。
no_paths = 沒有任何路徑 - 請在下方新增。
directories_volume_header = 冊數
directories_volume_refresh = 刷新
directories_volume_add = 新增
# Bottom navigation
nav_home = 開始。
nav_dirs = 目錄
nav_settings = 設定
# Status messages set from Rust
status_ready = 準備好了。
status_stopped = 已停止。
status_no_results = 沒有找到任何結果。
status_deleted_selected = 已刪除所選項目。
status_deleted_with_errors = 已刪除，但可能存在錯誤。
scan_not_started = 掃描尚未開始。
found_items_prefix = 已找到。
found_items_suffix = 商品。
deleted_items_prefix = 已刪除。
deleted_items_suffix = 商品。
deleted_errors_suffix = 錯誤。
renamed_prefix = 已重新命名。
renamed_files_suffix = 檔案
renamed_errors_suffix = 錯誤。
cleaned_exif_prefix = 已清除來自以下來源的 EXIF 資訊：
cleaned_exif_suffix = 檔案
cleaned_exif_errors_suffix = 錯誤。
and_more_prefix = ...以及
and_more_suffix = 更多。
# Gallery / delete popups
gallery_delete_button = 刪除
gallery_back = 返回
gallery_confirm_delete = 是的，刪除。
deleting_files = 正在刪除檔案...
stop = 停止
files_suffix = 檔案
scanning_fallback = 掃描中...
app_subtitle = 為紀念塞迪尼亞戰役 (西元 972 年) 而作。
app_license = Czkawka Core 的前端 - GPL-3.0 协议。
about_app_label = 關於
cache_label = 快取
# Notification
scan_completed_notification = 掃描完成 - 發現 { $file_count } 個項目。
# Confirm popups (set from Rust)
confirm_clean_exif = 您確定要清除所選 { $n } 個檔案中的 EXIF 標籤嗎？
confirm_delete_items = 您確定要刪除 { $n } 個已選取的項目嗎？
gallery_confirm_delete_msg = 您即將刪除 { $total_images } 張圖片，這些圖片位於 { $total_groups } 個群組中。.
gallery_confirm_delete_warning = 所有項目都已從 { $unsafe_groups } 個群組中選取！
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = 計算和比較音頻指紋需要消耗大量資源，而且可能需要很長時間。 建議在桌面系統上使用 Krokiet 來執行此項任務。.
# Scan stage labels (shown during scan progress)
stage_collecting_files = 收集檔案中
stage_scanning_name = 按姓名掃描。
stage_scanning_size_name = 以名稱和大小進行掃描。
stage_scanning_size = 依尺寸掃描。
stage_pre_hash = 預先哈希處理。
stage_full_hash = 雜湊演算法 (Hashing)
stage_loading_cache = 正在載入快取
stage_saving_cache = 正在儲存快取
stage_calculating_image_hashes = 計算影像的雜湊值。
stage_comparing_images = 比較圖片。
stage_calculating_video_hashes = 計算影片的雜湊值。
stage_checking_files = 正在檢查檔案
stage_checking_extensions = 正在檢查擴充功能。
stage_checking_names = 檢查姓名。
stage_reading_music_tags = 閱讀音樂標籤。
stage_comparing_tags = 比較標籤。
stage_calculating_music_fingerprints = 計算音樂指紋。
stage_comparing_fingerprints = 比對指紋。
stage_extracting_exif = 讀取 EXIF 標籤。
stage_creating_video_thumbnails = 製作影片縮圖。
stage_processing_videos = 正在處理影片。
stage_deleting = 刪除檔案
stage_renaming = 重新命名檔案
stage_moving = 移動檔案
stage_hardlinking = 建立硬連結。
stage_symlinking = 建立符號連結。
stage_optimizing_videos = 優化影片。
stage_cleaning_exif = 清除 EXIF 資訊。
# Group headers in scan results
duplicates_group_header = { $count } 個檔案  x  { $per_file } / 個檔案  =  總計 { $total }
similar_images_group_header = { $count } 張相似圖片
same_music_group_header = { $count } 相似曲目
# Rename confirmation
confirm_rename_items = 您確定要將選取的 { $n } 個檔案重新命名嗎？
# Combo-box option labels (translatable display names)
option_search_mode_biggest = 最大。
option_search_mode_smallest = 最小的
option_similarity_very_high = 極高
option_similarity_high = 高
option_similarity_medium = 中等
option_similarity_low = 低
option_similarity_very_low = V. 低
option_similarity_minimal = 最小.
option_check_method_hash = 雜湊
option_check_method_name = 名稱
option_check_method_size_and_name = 尺寸 + 姓名
option_check_method_size = 大小
option_music_method_tags = 標籤
option_music_method_audio = 音訊
option_min_size_none = 無。
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = 無限大。
# Volume labels (shown in the directories screen)
volume_internal_storage = 內部儲存空間
volume_sd_card = 記憶卡 (SD 卡)
volume_storage = 儲存空間容量
# Directories screen
directories_referenced_tooltip = 已參閱（未刪除）。
directories_include_section_header = 內含項目
directories_exclude_section_header = 已排除。
directories_custom_paths = 自定義路徑
directories_check_button = 分析。
directories_check_popup_title = 目錄統計資訊
directories_check_label_included = 包含的路径：
directories_check_label_excluded = 排除路徑：
directories_check_label_referenced = 參考路徑：
directories_check_label_would_scan = 需要掃描的檔案：
directories_check_label_processable = 可處理的文件：
directories_check_scanning = 掃描中...
directories_check_warning_no_processable = 未找到可處理的文件 - 請確認您包含/排除的資料夾。
path_edit_title_include = 新增包含項目。
path_edit_title_exclude = 新增至排除列表
path_edit_placeholder = 請輸入路徑...
path_edit_not_exists = 路徑不存在。
path_edit_is_dir = 目錄
path_edit_is_file = 檔案
path_edit_no_newlines = 路徑名稱中不能包含換行符號 — 不允許使用Enter鍵。
ctx_menu_title = 開啟。
ctx_open_file = 開啟項目。
ctx_open_folder = 開啟父目錄。
