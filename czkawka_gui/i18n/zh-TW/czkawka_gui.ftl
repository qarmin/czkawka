# Window titles
window_settings_title = 設定
window_main_title = Czkawka
window_progress_title = 掃描中
window_compare_images = 比較影像
# General
general_ok_button = 確定
general_close_button = 關閉
# Main window
music_title_checkbox = 標題
music_artist_checkbox = 藝人
music_year_checkbox = 年份
music_bitrate_checkbox = 位元率
music_genre_checkbox = 類型
music_length_checkbox = 長度
music_comparison_checkbox = 近似比較
music_checking_by_tags = 標籤
music_checking_by_content = 內容
same_music_seconds_label = 最小片段秒數
same_music_similarity_label = 最大差異
music_compare_only_in_title_group = Compare only in title
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    透過以下設定，可以根據內容搜尋相似的音樂檔案：
    
    - 音樂檔案在超過最小片段時間後可以被識別為相似
    - 兩個測試片段之間允許的最大差異
    
    要得到理想的結果，關鍵是找到這些參數的合適組合。
    
    例如，將最小時間設定為 5 秒，最大差異設定為 1.0，會尋找檔案中幾乎相同的片段。
    而設定時間為 20 秒和最大差異為 6.0，則適用於尋找混音版本或現場版本等。
    
    預設情況下，每個音樂檔案都會與其他檔案彼此進行比較，這在測試大量檔案時會非常耗時。因此，通常更建議使用參考資料夾，並明確指定哪些檔案需要相互比較。如果檔案數量相同，使用參考資料夾進行指紋比較的速度至少會比不使用參考資料夾快 4 倍。
music_comparison_checkbox_tooltip =
    它利用 AI 搜尋相似的音樂檔案，該 AI 使用機器學習來去除句子中的括號。例如，啟用這個選項後，以下的檔案將被視為重複檔案：
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = 區分大小寫
duplicate_case_sensitive_name_tooltip =
    啟用後，只有在檔案名稱完全相同的情況下才會將其分組，例如 Żołd <-> Żołd。
    
    停用這個選項則會在不檢查每個字母大小是否相同的情況下進行分組，例如 żoŁD <-> Żołd。
duplicate_mode_size_name_combo_box = 大小和名稱
duplicate_mode_name_combo_box = 名稱
duplicate_mode_size_combo_box = 大小
duplicate_mode_hash_combo_box = 雜湊
duplicate_hash_type_tooltip =
    Czkawka 提供三種類型的雜湊：
    
    Blake3 - 這是一種加密雜湊函式，也是預設選項，主要因為它的計算速度非常快。
    
    CRC32 - 這是一種簡單的雜湊函式。理論上它比 Blake3 更快，雖然機率很低但有時可能會產生碰撞。
    
    XXH3 - 在效能和雜湊品質上與 Blake3 非常相似，但它不是加密型的。因此，這兩種模式可以輕易地互換使用。
duplicate_check_method_tooltip =
    目前，Czkawka 提供三種方法來找出重複檔案：
    
    名稱 - 找出名稱相同的檔案。
    
    大小 - 找出大小相同的檔案。
    
    雜湊 - 找出內容相同的檔案。這個模式會先對檔案進行雜湊運算，然後比較這些雜湊值來識別重複檔案。這是找出重複檔案最安全的方式。由於應用程式大量使用快取，對同一組資料進行的第二次及後續掃描會比第一次快得多。
image_hash_size_tooltip =
    每個檢查的圖片會產生一個可用來來互相比較的的特定的雜湊值，它們之間些微的差異則代表這些圖片是相似的。
    
    8 雜湊大小是相當不錯用以尋找與原版僅些微相似的圖片。對於更大規模的圖片組（>1000），則會產生大量的誤報，所以在此情形中推薦使用更大的雜湊大小。
    
    16 是預設的雜湊大小是相當不錯的折衷方案，對於尋找即使僅些微相似的圖片，並且只會有少量的雜湊衝突。
    
    32 與 64 雜湊值用於尋找非常相似的圖片，但應該幾乎沒有誤報（也許除了一些具有 Alpha 通道的圖片）。
image_resize_filter_tooltip =
    要計算圖片的雜湊值，函式庫必須先對它進行調整大小。
    
    取決於選取的演算法，用於計算雜湊值的圖片將會看起來有些不同。
    
    最快的演算法是 Nearest，但也許會給出最差結果。預設為啟用，因為 16x16 雜湊大小並不是明顯可見的較低品質。
    
    對於 8x8 雜湊大小，建議使用不同於 Nearest 的演算法，以獲得更好的圖片分組。
image_hash_alg_tooltip =
    使用者可以從許多計算雜湊值的演算法中選擇一種。
    
    每種演算法都有強項和弱項，對於不同的圖片，有時會有更好的結果，有時會有更差的結果。
    
    因此，為了確定最適合你的演算法，需要進行人工測試。
big_files_mode_combobox_tooltip = 允許搜尋最小/最大的檔案
big_files_mode_label = 已檢查的檔案
big_files_mode_smallest_combo_box = 最小的
big_files_mode_biggest_combo_box = 最大的
main_notebook_duplicates = 重複檔案
main_notebook_empty_directories = 空目錄
main_notebook_big_files = 大檔案
main_notebook_empty_files = 空檔案
main_notebook_temporary = 臨時檔案
main_notebook_similar_images = 相似影像
main_notebook_similar_videos = 相似影片
main_notebook_same_music = 音樂重複
main_notebook_symlinks = 無效的符號連結
main_notebook_broken_files = 損壞的檔案
main_notebook_bad_extensions = 錯誤的副檔名
main_tree_view_column_file_name = 檔案名稱
main_tree_view_column_folder_name = 資料夾名稱
main_tree_view_column_path = 路徑
main_tree_view_column_modification = 修改日期
main_tree_view_column_size = 大小
main_tree_view_column_similarity = 相似度
main_tree_view_column_dimensions = 尺寸
main_tree_view_column_title = 標題
main_tree_view_column_artist = 藝人
main_tree_view_column_year = 年份
main_tree_view_column_bitrate = 位元率
main_tree_view_column_length = 長度
main_tree_view_column_genre = 類型
main_tree_view_column_symlink_file_name = 符號連結檔案名稱
main_tree_view_column_symlink_folder = 符號連結資料夾
main_tree_view_column_destination_path = 目標路徑
main_tree_view_column_type_of_error = 錯誤類型
main_tree_view_column_current_extension = 現有副檔名
main_tree_view_column_proper_extensions = 適當的副檔名
main_label_check_method = 檢查方法
main_label_hash_type = 雜湊類型
main_label_hash_size = 雜湊大小
main_label_size_bytes = 大小（位元組）
main_label_min_size = 最小
main_label_max_size = 最大
main_label_shown_files = 顯示的檔案數
main_label_resize_algorithm = 調整大小的演算法
main_label_similarity = 相似度：{ " " }
main_check_box_broken_files_audio = 音訊
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = 歸檔
main_check_box_broken_files_image = 影像
check_button_general_same_size = 忽略相同的大小
check_button_general_same_size_tooltip = 忽略在結果中具有完全相同大小的檔案 - 通常這些是 1:1 的重複
main_label_size_bytes_tooltip = 將用於掃描的檔案大小
# Upper window
upper_tree_view_included_folder_column_title = 要搜尋的資料夾
upper_tree_view_included_reference_column_title = 參考資料夾
upper_recursive_button = 遞迴
upper_recursive_button_tooltip = 如果選取，也會搜尋未直接放在選定資料夾下的檔案。
upper_manual_add_included_button = 手動新增
upper_add_included_button = 新增
upper_remove_included_button = 移除
upper_manual_add_excluded_button = 手動新增
upper_add_excluded_button = 新增
upper_remove_excluded_button = 移除
upper_manual_add_included_button_tooltip =
    手動新增目錄名稱。
    
    一次新增多個路徑，用分號(;)分隔它們
    
    /home/roman;/home/rozkaz 將新增兩個目錄 /home/roman 和 /home/rozkaz
upper_add_included_button_tooltip = 新增新目錄進行搜尋。
upper_remove_included_button_tooltip = 從搜尋中移除目錄。
upper_manual_add_excluded_button_tooltip =
    手動新增要排除的目錄名稱。
    
    一次新增多個路徑，請用分號(;)分隔它們
    
    /home/roman;/home/krokiet 將新增兩個目錄 /home/roman 和 /home/krokiet
upper_add_excluded_button_tooltip = 新增要在搜尋中排除的目錄。
upper_remove_excluded_button_tooltip = 從排除中移除目錄。
upper_notebook_items_configuration = 項目設定
upper_notebook_excluded_directories = 排除的目錄
upper_notebook_included_directories = 包含的目錄
upper_allowed_extensions_tooltip =
    允許的副檔名必須用逗號分隔（預設所有可用）。
    
    以下的巨集也可用，可以一次新增多個副檔名：IMAGE, VIDEO, MUSIC, TEXT。
    
    使用範例 ".exe, IMAGE, VIDEO, .rar, 7z" - 這表示將影像檔案（例如 .jpg, .png)、影片檔案（例如 .avi, .mp4)、.exe、.rar 和 .7z 檔案。
upper_excluded_extensions_tooltip =
    在掃描中將會被忽略的禁用檔案清單。
    
    當使同時使用允許與禁用兩者時，此項擁有更高的優先等級，所以檔案將不會被檢查。
upper_excluded_items_tooltip =
    要排除的項目必須包含 * 萬用字元，並且使用逗號分隔。
    這比排除目錄的速度來的更慢，請謹慎使用。
upper_excluded_items = 排除的項目：
upper_allowed_extensions = 允許的副檔名：
upper_excluded_extensions = 禁用的副檔名：
# Popovers
popover_select_all = 選擇全部
popover_unselect_all = 取消選擇全部
popover_reverse = 反向選擇
popover_select_all_except_oldest = 選擇除最舊以外的全部
popover_select_all_except_newest = 選擇除最新以外的全部
popover_select_one_oldest = 選擇一個最舊的
popover_select_one_newest = 選擇一個最新的
popover_select_custom = 選擇自訂
popover_unselect_custom = 取消選擇自訂
popover_select_all_images_except_biggest = 選擇除最大以外的全部影像
popover_select_all_images_except_smallest = 選擇除最小以外的全部影像
popover_custom_path_check_button_entry_tooltip =
    透過路徑選擇記錄。
    
    範例用法：
    /home/pimpek/rzecz.txt 可以透過 /home/pim* 找到
popover_custom_name_check_button_entry_tooltip =
    透過檔名選擇記錄。
    
    範例用法：
    /usr/ping/pong.txt 可以在 *ong* 中找到。
popover_custom_regex_check_button_entry_tooltip =
    透過指定的正規表達式（Regex）來選擇記錄。
    
    在這個模式下，被搜尋的文字是「路徑」加上「名稱」。
    
    範例用法：
    使用 /ziem[a-z]+ 可以找到 /usr/bin/ziemniak.txt。
    
    這個功能使用的是 Rust 語言預設的正規表達式實作。更多相關資訊，您可以參考這個網址： https://docs.rs/regex。
popover_custom_case_sensitive_check_button_tooltip =
    啟用區分大小寫的偵測。
    
    當此選項停用時，「/home/*」會同時找到「/HoMe/roman」和「/home/roman」。
popover_custom_not_all_check_button_tooltip =
    防止在同一群組中全選所有記錄。
    
    這個選項預設是啟用的，主要是因為在多數情況下，您不會想要同時刪除原始檔案和其重複檔，而是會希望至少保留一個檔案。
    
    警告：如果您已經手動全選了某一群組中的所有結果，這個設定將不會生效。
popover_custom_regex_path_label = 路徑
popover_custom_regex_name_label = 名稱
popover_custom_regex_regex_label = 正規表達式路徑 + 名稱
popover_custom_case_sensitive_check_button = 區分大小寫
popover_custom_all_in_group_label = 不要選取群組中的所有記錄
popover_custom_mode_unselect = 取消選擇自訂
popover_custom_mode_select = 選擇自訂
popover_sort_file_name = 檔案名稱
popover_sort_folder_name = 資料夾名稱
popover_sort_full_name = 完整名稱
popover_sort_size = 大小
popover_sort_selection = 選擇
popover_invalid_regex = 正規表達式無效
popover_valid_regex = 正規表達式有效
# Bottom buttons
bottom_search_button = 搜尋
bottom_select_button = 選擇
bottom_delete_button = 刪除
bottom_save_button = 儲存
bottom_symlink_button = 符號連結
bottom_hardlink_button = 永久連結
bottom_move_button = 移動
bottom_sort_button = 排序
bottom_compare_button = Compare
bottom_search_button_tooltip = 開始搜尋
bottom_select_button_tooltip = 選擇記錄。只能稍後處理選定的檔案/資料夾。
bottom_delete_button_tooltip = 刪除選取的檔案/資料夾。
bottom_save_button_tooltip = 儲存搜尋資料到檔案
bottom_symlink_button_tooltip =
    建立符號連結。
    只有在一個群組中至少選擇了兩個結果時才會生效。
    第一個檔案保持不變，第二個以及之後的檔案會建立為指向第一個檔案的符號連結。
bottom_hardlink_button_tooltip =
    建立永久連結。
    只有在一個群組中至少選擇了兩個結果時才會生效。
    第一個檔案保持不變，第二個以及之後的檔案會建立為與第一個檔案的永久連結。
bottom_hardlink_button_not_available_tooltip =
    建立永久連結。
    此按鈕已被停用，因為無法建立永久連結。
    在 Windows 上，只有擁有管理員權限才能建立永久連結，請確保以管理員身份執行應用程式。
    如果應用程式已經具有對應的權限，請在 GitHub 上查詢相關問題。
bottom_move_button_tooltip =
    將檔案移動到指定目錄。
    會將所有檔案複製到目錄中，但不會保留原始的目錄結構。
    如果試圖將兩個同名檔案移動到同一資料夾，第二個檔案將無法移動並會顯示錯誤。
bottom_sort_button_tooltip = 根據選定的方法排序檔案/資料夾。
bottom_compare_button_tooltip = Compare images in the group.
bottom_show_errors_tooltip = 顯示/隱藏底部文字面板。
bottom_show_upper_notebook_tooltip = 顯示/隱藏主筆記本面板。
# Progress Window
progress_stop_button = 停止
progress_stop_additional_message = 已請求停止
# About Window
about_repository_button_tooltip = 連結到原始碼的專案。
about_donation_button_tooltip = 連結到贊助頁面。
about_instruction_button_tooltip = 連結到指令頁面。
about_translation_button_tooltip = 連結到帶有應用程式翻譯的 Crowdin 頁面。官方支援波蘭語和英語。
about_repository_button = 儲存庫
about_donation_button = 贊助
about_instruction_button = 說明
about_translation_button = 翻譯
# Header
header_setting_button_tooltip = 開啟設定對話方塊。
header_about_button_tooltip = 開啟包含應用程式資訊的對話方塊。

# Settings


## General

settings_number_of_threads = 使用的執行緒數
settings_number_of_threads_tooltip = 使用的執行緒數，0 表示所有可用執行緒都將被使用。
settings_use_rust_preview = Use external libraries instead gtk to load previews
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = 您需要重新啟動應用程式才能套用設定！
settings_ignore_other_filesystems = 忽略其它檔案系統（僅限 Linux）
settings_ignore_other_filesystems_tooltip =
    忽略與搜尋的目錄不在同一個檔案系統中的檔案。
    
    在 Linux 上查詢命令時類似 -xdev 選項
settings_save_at_exit_button_tooltip = 關閉應用程式時將設定儲存到檔案。
settings_load_at_start_button_tooltip =
    開啟應用程式時從檔案載入設定。
    
    如果未啟用，將使用預設設定。
settings_confirm_deletion_button_tooltip = 點選刪除按鈕時顯示確認對話方塊。
settings_confirm_link_button_tooltip = 點選永久連結/符號連結按鈕時顯示確認對話方塊。
settings_confirm_group_deletion_button_tooltip = 嘗試從群組中刪除所有記錄時顯示警告對話方塊。
settings_show_text_view_button_tooltip = 在使用者介面底部顯示文字面板。
settings_use_cache_button_tooltip = 使用檔案快取。
settings_save_also_as_json_button_tooltip = 儲存快取為（人類可讀）JSON 格式。可以修改其內容。 如果缺少二進位制格式快取（帶bin extensional)，此檔案的快取將被應用程式自動讀取。
settings_use_trash_button_tooltip = 將檔案移至回收桶，而將其永久刪除。
settings_language_label_tooltip = 使用者介面的語言。
settings_save_at_exit_button = 關閉應用程式時儲存設定
settings_load_at_start_button = 開啟應用程式時載入設定
settings_confirm_deletion_button = 刪除任何檔案時顯示確認對話方塊
settings_confirm_link_button = 硬/符號連結任何檔案時顯示確認對話方塊
settings_confirm_group_deletion_button = 刪除群組中所有檔案時顯示確認對話方塊
settings_show_text_view_button = 顯示底部文字面板
settings_use_cache_button = 使用快取
settings_save_also_as_json_button = 同時將快取儲存為 JSON 檔案
settings_use_trash_button = 移動已刪除的檔案到回收桶
settings_language_label = 語言
settings_multiple_delete_outdated_cache_checkbutton = 自動刪除過時的快取項目
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    刪除指向不存在檔案的過時快取結果。
    
    啟用後，應用程式在載入記錄時會確保所有記錄都指向有效的檔案（無效的檔案會被忽略）。
    
    停用此選項將有助於掃描外部硬碟上的檔案，這樣下次掃描時有關這些檔案的快取項目不會被清除。
    
    若快取中有數十萬條記錄，建議啟用此選項，這將加速掃描開始和結束時的快取載入和儲存。
settings_notebook_general = 一般
settings_notebook_duplicates = 重複項目
settings_notebook_images = 相似影像
settings_notebook_videos = 相似影片

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = 在右側顯示預覽（當選擇影像檔案時）。
settings_multiple_image_preview_checkbutton = 顯示影像預覽
settings_multiple_clear_cache_button_tooltip =
    手動清除過時項目的快取。
    僅在停用自動清除時才應使用。
settings_multiple_clear_cache_button = 從快取中移除過時結果

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    如果所有檔案都指向相同的資料（即為永久連結），則隱藏除一個以外的所有檔案。
    
    例如：在有七個檔案與特定資料有永久連結，以及一個具有相同資料但不同 inode 的不同檔案的情況下，重複檔案檢查工具只會顯示一個獨特的檔案和一個來自永久連結的檔案。
settings_duplicates_minimal_size_entry_tooltip =
    設定將被快取的最小檔案大小。
    
    選擇較小的值會產生更多記錄。這會加速搜尋，但會減慢快取的載入和儲存。
settings_duplicates_prehash_checkbutton_tooltip =
    啟用預先計算的雜湊（從檔案的一小部分計算出來）的快取，這允許更早地排除非重複的結果。
    
    這個選項預設是停用的，因為在某些情況下它可能會造成減速。
    
    當掃描數十萬或百萬個檔案時，強烈建議使用此選項，因為它可以多倍加速搜尋。
settings_duplicates_prehash_minimal_entry_tooltip = 快取項目的最小大小。
settings_duplicates_hide_hard_link_button = 隱藏永久連結（僅限 Linux 和 macOS）
settings_duplicates_prehash_checkbutton = 使用捕捉快取
settings_duplicates_minimal_size_cache_label = 儲存到快取的檔案最小大小（位元組）
settings_duplicates_minimal_size_cache_prehash_label = 檔案最小大小（位元組）儲存到逮捕快取

## Saving/Loading settings

settings_saving_button_tooltip = 儲存目前設定設定到檔案。
settings_loading_button_tooltip = 從檔案載入設定並替換目前設定。
settings_reset_button_tooltip = 重設目前設定為預設設定。
settings_saving_button = 儲存設定
settings_loading_button = 載入設定
settings_reset_button = 重設設定

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    開啟儲存快取 txt 檔案的資料夾。
    
    修改快取檔案可能會導致顯示無效的結果。然而，如果需要將大量檔案移動到不同位置，修改路徑可能會節省時間。
    
    如果兩台電腦有類似的目錄結構，您可以在它們之間複製這些檔案，以節省重新掃描檔案的時間。
    
    如果快取有問題，這些檔案可以被移除。應用程式會自動重新產生它們。
settings_folder_settings_open_tooltip =
    開啟儲存 Czkawka 設定的資料夾。
    
    警告：手動修改設定可能會影響您的工作流程。
settings_folder_cache_open = 開啟快取資料夾
settings_folder_settings_open = 開啟設定資料夾
# Compute results
compute_stopped_by_user = 搜尋已被使用者停止
compute_found_duplicates_hash_size = 在 { $number_groups } 群組中找到 { $number_files } 重複，被帶去了 { $size }
compute_found_duplicates_name = 找到 { $number_files } 重複的 { $number_groups } 群組
compute_found_empty_folders = 找到 { $number_files } 空資料夾
compute_found_empty_files = 找到 { $number_files } 空檔案
compute_found_big_files = 找到 { $number_files } 大檔案
compute_found_temporary_files = 找到 { $number_files } 臨時檔案
compute_found_images = 在 { $number_groups } 群組中找到 { $number_files } 相似影像
compute_found_videos = 在 { $number_groups } 群組中找到 { $number_files } 相似影片
compute_found_music = 在 { $number_groups } 群組中找到 { $number_files } 相似的音樂檔案
compute_found_invalid_symlinks = 找到 { $number_files } 無效的符號連結
compute_found_broken_files = 找到 { $number_files } 損壞的檔案
compute_found_bad_extensions = 找到 { $number_files } 檔案，其副檔名無效
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] Scanned { $file_number } file
       *[other] Scanned { $file_number } files
    }
progress_scanning_extension_of_files = Checked extension of { $file_checked }/{ $all_files } file
progress_scanning_broken_files = Checked { $file_checked }/{ $all_files } file ({ $data_checked }/{ $all_data })
progress_scanning_video = Hashed of { $file_checked }/{ $all_files } video
progress_scanning_image = Hashed of { $file_checked }/{ $all_files } image ({ $data_checked }/{ $all_data })
progress_comparing_image_hashes = Compared { $file_checked }/{ $all_files } image hash
progress_scanning_music_tags_end = Compared tags of { $file_checked }/{ $all_files } music file
progress_scanning_music_tags = Read tags of { $file_checked }/{ $all_files } music file
progress_scanning_music_content_end = Compared fingerprint of { $file_checked }/{ $all_files } music file
progress_scanning_music_content = Calculated fingerprint of { $file_checked }/{ $all_files } music file ({ $data_checked }/{ $all_data })
progress_scanning_empty_folders =
    { $folder_number ->
        [one] Scanned { $folder_number } folder
       *[other] Scanned { $folder_number } folders
    }
progress_scanning_size = Scanned size of { $file_number } file
progress_scanning_size_name = Scanned name and size of { $file_number } file
progress_scanning_name = Scanned name of { $file_number } file
progress_analyzed_partial_hash = Analyzed partial hash of { $file_checked }/{ $all_files } files ({ $data_checked }/{ $all_data })
progress_analyzed_full_hash = Analyzed full hash of { $file_checked }/{ $all_files } files ({ $data_checked }/{ $all_data })
progress_prehash_cache_loading = 正在載入 PreHash 快取
progress_prehash_cache_saving = 正在儲存 PreHash 快取
progress_hash_cache_loading = 正在載入雜湊快取
progress_hash_cache_saving = 正在儲存雜湊快取
progress_cache_loading = 正在載入快取
progress_cache_saving = 正在儲存快取
progress_current_stage = 目前階段：{ "  " }
progress_all_stages = 所有階段：{ " " }
# Saving loading 
saving_loading_saving_success = 設定儲存到檔案 { $name }。
saving_loading_saving_failure = 無法將設定資料儲存到檔案 { $name }
saving_loading_reset_configuration = 目前設定已被清除。
saving_loading_loading_success = 正確載入應用程式設定。
saving_loading_invalid_string = 對於金鑰"{ $key }" 發現無效的結果 - "{ $result }"不是字串。
saving_loading_invalid_int = 對於金鑰"{ $key }" 發現無效的結果 - "{ $result } 不是整數。
saving_loading_invalid_bool = 對於金鑰"{ $key }" 發現無效的結果 - "{ $result } 不是布林值。
saving_loading_decode_problem_bool = 無法從金鑰 "{ $key }" 解碼布林值，找到的是 "{ $result }" ，允許的值為 0, 1, true 或 false。
saving_loading_saving_same_keys = 嘗試用重複的金鑰儲存設定 "{ $key }".
saving_loading_failed_to_get_home_directory = 無法取得開啟/儲存設定檔案的主目錄。
saving_loading_folder_config_instead_file = 無法在路徑 "{ $path } 中建立或開啟設定檔案，因為已經有一個資料夾。
saving_loading_failed_to_create_configuration_folder = 無法建立設定資料夾 "{ $path }", 原因"{ $reason }".
saving_loading_failed_to_create_config_file = 無法建立設定檔案 "{ $path }", 原因"{ $reason }".
saving_loading_failed_to_read_config_file = 無法從 "{ $path }" 載入設定，因為它不存在或不是檔案。
saving_loading_failed_to_read_data_from_file = 無法從檔案讀取資料"{ $path }", 原因"{ $reason }".
saving_loading_orphan_data = 在 "{ $line }" 行中發現了孤兒資料 "{ $data }"。
saving_loading_not_valid = 設定 "{ $data }" 在目前應用程式的版本中不存在。
# Invalid symlinks
invalid_symlink_infinite_recursion = 無限遞迴
invalid_symlink_non_existent_destination = 目標檔案不存在
# Other
selected_all_reference_folders = 當所有目錄被設定為參考資料夾時，無法開始搜尋
searching_for_data = 正在搜尋資料，可能需要一段時間，請稍候...
text_view_messages = 訊息
text_view_warnings = 警告
text_view_errors = 錯誤
about_window_motto = 這個程式可以永遠自由使用。
# Various dialog
dialogs_ask_next_time = 下次詢問
delete_file_failed = 刪除檔案 { $name } 失敗，原因 { $reason }
delete_title_dialog = 刪除確認
delete_question_label = 您確定要刪除檔案嗎？
delete_all_files_in_group_title = 確認刪除群組中的所有檔案
delete_all_files_in_group_label1 = 在某些群組中，所有記錄都被選取。
delete_all_files_in_group_label2 = 您確定要刪除它們嗎？
delete_folder_failed = 無法刪除資料夾 { $dir } ，因為資料夾不存在，您沒有權限，或者資料夾不是空的。
delete_items_label = { $items } 檔案將被刪除。
delete_items_groups_label = { $items } 檔案來自 { $groups } 群組將被刪除。
hardlink_failed = 永久連結失敗
hard_sym_invalid_selection_title_dialog = 對某些群組的選擇無效
hard_sym_invalid_selection_label_1 = 在某些群組中，只選擇了一個記錄，它將被忽略。
hard_sym_invalid_selection_label_2 = 要能夠連結到這些檔案，至少需要選擇兩個群組的結果。
hard_sym_invalid_selection_label_3 = 第一個群組被承認為原始群組，沒有改變，但是第二個群組後來被修改。
hard_sym_link_title_dialog = 連結確認
hard_sym_link_label = 您確定要連結這些檔案嗎？
move_folder_failed = 無法移動資料夾 { $name }, 原因 { $reason }
move_file_failed = 移動檔案 { $name } 失敗，原因 { $reason }
move_files_title_dialog = 選擇要移動重複檔案的資料夾
move_files_choose_more_than_1_path = 只能選擇一個路徑來複製重複的檔案，選擇 { $path_number }。
move_stats = 正確移動 { $num_files }/{ $all_files } 專案
save_results_to_file = Saved results both to txt and json files into { $name } folder.
search_not_choosing_any_music = 錯誤：您必須選擇至少一個帶有音樂搜尋類型的核取方塊。
search_not_choosing_any_broken_files = 錯誤：您必須選擇至少一個帶有選取檔案類型的核取方塊。
include_folders_dialog_title = 要包含的資料夾
exclude_folders_dialog_title = 要排除的資料夾
include_manually_directories_dialog_title = 手動新增目錄
cache_properly_cleared = 已正確清除快取
cache_clear_duplicates_title = 清除重複快取
cache_clear_similar_images_title = 清除相似影像快取
cache_clear_similar_videos_title = 正在清除相似影片快取
cache_clear_message_label_1 = 您想要清除過時項目的快取嗎？
cache_clear_message_label_2 = 此操作將刪除所有指向無效檔案的快取項。
cache_clear_message_label_3 = 這可能會稍微加速載入/儲存到快取。
cache_clear_message_label_4 = 警告：操作將從未接入的外部硬碟中移除所有快取資料。所以每個雜湊都需要重新產生。
# Show preview
preview_image_resize_failure = 調整影像大小失敗 { $name }
preview_image_opening_failure = 開啟影像 { $name } 失敗，原因 { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = 組 { $current_group }/{ $all_groups } ({ $images_in_group } 影像）
compare_move_left_button = L
compare_move_right_button = R
