# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Các bản sao
tool_empty_folders = Thư mục trống
tool_similar_images = Hình ảnh tương tự
tool_empty_files = Các tệp trống
tool_temporary_files = Các tệp tạm
tool_big_files = Các tệp lớn nhất
tool_broken_files = Tệp tin bị lỗi
tool_bad_extensions = Các tiện ích mở rộng kém chất lượng
tool_same_music = Bản sao nhạc
tool_bad_names = Tên nghe tệ
tool_exif_remover = Dữ liệu EXIF
tool_directories = Thư mục
tool_settings = Cài đặt
# Home screen tool card descriptions
home_dup_description = Tìm các tệp có nội dung giống nhau
home_empty_folders_description = Các thư mục không có nội dung
home_similar_images_description = Tìm các ảnh có hình ảnh tương tự
home_empty_files_description = Các tệp có kích thước bằng không
home_temp_files_description = Các tệp tạm thời và tệp được lưu trong bộ nhớ đệm
home_big_files_description = Các tệp lớn nhất/nhỏ nhất trên ổ đĩa
home_broken_files_description = PDF, âm thanh, hình ảnh, các tệp lưu trữ
home_bad_extensions_description = Các tệp có phần mở rộng không hợp lệ
home_same_music_description = Các tệp âm thanh tương tự theo thẻ
home_bad_names_description = Các tệp có chứa các ký tự không hợp lệ trong tên
home_exif_description = Hình ảnh có siêu dữ liệu EXIF
# Results list
scanning = Đang quét...
stopping = Đang dừng...
no_results = Không có kết quả
press_start = Nhấn nút BẮT ĐẦU để quét
select_label = Chọn.
deselect_label = Desel.
list_label = Danh sách
gallery_label = Ga-la.
# Selection popup
selection_popup_title = Chọn
select_all = Chọn tất cả
select_except_one = Chọn tất cả trừ một
select_except_largest = Chọn tất cả các mục trừ mục lớn nhất
select_except_smallest = Chọn tất cả trừ phần nhỏ nhất
select_largest = Chọn phần lớn nhất
select_smallest = Chọn phần nhỏ nhất
select_except_highest_res = Chọn tất cả các tùy chọn, ngoại trừ độ phân giải cao nhất
select_except_lowest_res = Chọn tất cả các tùy chọn, trừ độ phân giải thấp nhất
select_highest_res = Chọn độ phân giải cao nhất
select_lowest_res = Chọn độ phân giải thấp nhất
invert_selection = Đảo ngược lựa chọn
close = Đóng
# Deselection popup
deselection_popup_title = Bỏ chọn
deselect_all = Bỏ chọn tất cả
deselect_except_one = Bỏ chọn tất cả, chỉ giữ lại một
# Confirm popup
cancel = Hủy bỏ
delete = Xóa
rename = Đổi tên
# Delete errors popup
delete_errors_title = Không thể xóa một số tập tin:
ok = Được
# Stopping overlay
stopping_overlay_title = Dừng lại
stopping_overlay_body = 
        Đang hoàn thành quá trình quét hiện tại...
        Vui lòng chờ.
# Permission popup
permission_title = Truy cập tệp
permission_body = Để ứng dụng có thể quét các tệp, ứng dụng cần quyền truy cập vào bộ nhớ của thiết bị. Nếu không có quyền này, việc quét sẽ không thể thực hiện được.
grant = Trao tặng
no_permission_scan_warning = Không thể truy cập tệp - vui lòng cấp quyền để quét
# Settings screen tabs
settings_tab_general = Tổng quan
settings_tab_tools = Công cụ
settings_tab_diagnostics = Thông tin
# Settings - General tab
settings_use_cache = Sử dụng bộ nhớ đệm
settings_use_cache_desc = Tăng tốc độ quét sau (băm/hình ảnh)
settings_ignore_hidden = Bỏ qua các tệp ẩn
settings_ignore_hidden_desc = Các tệp và thư mục bắt đầu bằng dấu '.'
settings_show_notification = Thông báo khi quá trình quét hoàn tất
settings_show_notification_desc = Hiển thị thông báo hệ thống khi quá trình quét hoàn tất
settings_notify_only_background = Chỉ khi đang chạy ngầm
settings_notify_only_background_desc = Bỏ qua thông báo nếu ứng dụng đang hiển thị
notifications_disabled_banner = Thông báo đã bị tắt
notifications_enable_button = Bật
settings_scan_label = QUÉT
settings_filters_label = LỌC (một số công cụ)
settings_min_file_size = Kích thước tệp tối thiểu
settings_max_file_size = Kích thước tệp tối đa
settings_language = Ngôn ngữ
settings_language_restart = Yêu cầu khởi động lại ứng dụng
settings_common_label = CÁC CÀI ĐẶT CHUNG
settings_excluded_items = CÁC MỤC ĐƯỢC LOẠI TRỪ (mẫu toàn cục, phân tách bằng dấu phẩy)
settings_excluded_items_placeholder = Ví dụ: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = CÁC ĐỊNH DẠNG TỆP ĐƯỢC PHÉP (để trống = tất cả)
settings_allowed_extensions_placeholder = Ví dụ: jpg, png, mp4
settings_excluded_extensions = CÁC TIỆN ÍCH KHÔNG ĐƯỢC HỖ TRỢ
settings_excluded_extensions_placeholder = Ví dụ: bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = BẢN SAO CHÉ
settings_check_method_label = PHƯƠNG PHÁP SO SÁNH
settings_check_method = Phương pháp
settings_hash_type_label = LOẠI MÃ HÓA
settings_hash_type = Loại hàm băm
settings_hash_type_desc = Blake3 - là lựa chọn được khuyến nghị, trong khi CRC32 có khả năng tạo ra kết quả dương tính giả
settings_similar_images_header = CÁC HÌNH ẢNH TƯƠNG TỰ
settings_similarity_preset = Ngưỡng tương đồng
settings_similarity_desc = Rất cao = chỉ gần giống hệt
settings_hash_size = Kích thước hàm băm
settings_hash_size_desc = Các kích thước lớn hơn có xu hướng tạo ra ít kết quả dương tính giả hơn, nhưng đồng thời cũng tìm thấy ít hình ảnh tương tự hơn
settings_hash_alg = Thuật toán băm
settings_image_filter = Điều chỉnh kích thước bộ lọc
settings_ignore_same_size = Bỏ qua các hình ảnh có cùng kích thước
settings_gallery_image_fit_cover = Thư viện ảnh: Cắt ảnh thành hình vuông
settings_gallery_image_fit_cover_desc = Điền đầy ô; tắt tùy chọn để giữ nguyên tỷ lệ khung hình gốc
settings_big_files_header = TỆP LỚN NHẤT
settings_search_mode = Chế độ tìm kiếm
settings_file_count = Số lượng file
settings_same_music_header = BẢN SAO NHẠC
settings_music_check_method = Chế độ so sánh
settings_music_compare_tags_label = NHỮNG THẺ SO SÁNH
settings_music_title = Tiêu đề
settings_music_artist = Nghệ sĩ
settings_music_year = Năm
settings_music_length = Độ dài
settings_music_genre = Thể loại
settings_music_bitrate = Bitrate
settings_music_approx = So sánh thẻ (ước tính)
settings_broken_files_header = TỆP BỊ HƯ HỎNG
settings_broken_files_note = Quá trình quét này đòi hỏi nhiều tài nguyên. Để đạt hiệu suất tốt nhất, hãy sử dụng Krokiet trên máy tính để bàn.
settings_broken_files_types_label = LOẠI ĐÃ KIỂM TRA
settings_broken_audio = Âm thanh
settings_broken_pdf = PDF
settings_broken_archive = Lưu trữ
settings_broken_image = Hình ảnh
settings_bad_names_header = TÊN XẤU
settings_bad_names_checks_label = KIỂM TRA
settings_bad_names_uppercase_ext = Ký hiệu mở rộng viết hoa
settings_bad_names_emoji = Biểu tượng cảm xúc trong tên
settings_bad_names_space = Các khoảng trắng ở đầu/cuối
settings_bad_names_non_ascii = Ký tự không thuộc bảng ASCII
settings_bad_names_duplicated = Ký tự lặp lại
# Settings - Diagnostics tab
diagnostics_header = CHẨN ĐOÁN
diagnostics_thumbnails = Bộ nhớ đệm ảnh thu nhỏ
diagnostics_app_cache = Bộ nhớ đệm của ứng dụng
diagnostics_refresh = Tươi mới
diagnostics_clear_thumbnails = Hình thu nhỏ rõ ràng
diagnostics_open_thumbnails_folder = Mở thư mục
diagnostics_clear_cache = Xóa bộ nhớ cache
diagnostics_open_cache_folder = Mở thư mục
diagnostics_collect_test = Kiểm tra quyền truy cập tệp
diagnostics_collect_test_desc = Kiểm tra xem có bao nhiêu tệp có thể truy cập được
diagnostics_collect_test_run = Chạy
diagnostics_collect_test_stop = Dừng lại
collect_test_cancelled = Đã bị người dùng chặn
diag_confirm_clear_thumbnails = Xóa toàn bộ bộ nhớ đệm hình thu nhỏ?
diag_confirm_clear_cache = Xóa toàn bộ bộ nhớ cache của ứng dụng?
about_repo = Kho lưu trữ
about_translate = Dịch thuật
about_donate = Hỗ trợ
# Collect-test result popup
collect_test_title = Kết quả kiểm tra
collect_test_volumes = Số lượng:
collect_test_folders = Thư mục:
collect_test_files = Tệp:
collect_test_time = Thời gian:
# Licenses
licenses_label = GIẤY PHÉP SỬ DỤNG
third_party_licenses = Giấy phép của bên thứ ba
licenses_popup_title = Giấy phép của bên thứ ba
# Directories screen
directories_include_header = Bao gồm
directories_included = Đã bao gồm
directories_exclude_header = Loại trừ
directories_excluded_header = Loại trừ
directories_add = Bao gồm
no_paths = Không có đường dẫn - thêm vào phần dưới đây
directories_volume_header = Số lượng
directories_volume_refresh = Làm mới
directories_volume_add = Thêm
# Bottom navigation
nav_home = Bắt đầu
nav_dirs = Thư mục
nav_settings = Cài đặt
# Status messages set from Rust
status_ready = Sẵn sàng
status_stopped = Đã dừng
status_no_results = Không có kết quả
status_deleted_selected = Đã xóa mục đã chọn
status_deleted_with_errors = Đã xóa, có lỗi
scan_not_started = Quá trình quét chưa bắt đầu
found_items_prefix = Đã tìm thấy
found_items_suffix = các mục
deleted_items_prefix = Đã xóa
deleted_items_suffix = các mục
deleted_errors_suffix = lỗi
renamed_prefix = Đã đổi tên
renamed_files_suffix = các tệp
renamed_errors_suffix = lỗi
cleaned_exif_prefix = Đã loại bỏ thông tin EXIF từ
cleaned_exif_suffix = các tệp
cleaned_exif_errors_suffix = lỗi
and_more_prefix = ...và
and_more_suffix = thêm nữa
# Gallery / delete popups
gallery_delete_button = Xóa
gallery_back = Quay lại
gallery_confirm_delete = Vâng, xóa
deleting_files = Đang xóa các tệp...
stop = Dừng lại
files_suffix = các tệp
scanning_fallback = Đang quét...
app_subtitle = Để tưởng nhớ trận chiến Cedynia (năm 972 CN)
app_license = Giao diện người dùng cho Czkawka Core - Giấy phép GPL-3.0
about_app_label = VỀ
cache_label = BỘ NHỚ ĐỆM
# Notification
scan_completed_notification = Quá trình quét đã hoàn tất - Tìm thấy { $file_count } mục
# Confirm popups (set from Rust)
confirm_clean_exif = Bạn có chắc chắn muốn xóa các thẻ EXIF khỏi { $n } tập tin đã chọn không?
confirm_delete_items = Bạn có chắc chắn muốn xóa { $n } mục đã chọn không?
gallery_confirm_delete_msg = Bạn sắp xóa { $total_images } ảnh trong { $total_groups } nhóm.
gallery_confirm_delete_warning = Tất cả các mục đã được chọn trong { $unsafe_groups } nhóm!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Việc tính toán và so sánh dấu vân âm thanh đòi hỏi rất nhiều tài nguyên và có thể mất nhiều thời gian. Chúng tôi khuyến nghị nên sử dụng Krokiet trên một hệ thống máy tính để bàn để thực hiện tác vụ này.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Đang thu thập các tệp
stage_scanning_name = Quét theo tên
stage_scanning_size_name = Quét theo tên và kích thước
stage_scanning_size = Quét theo kích thước
stage_pre_hash = Tiền xử lý băm
stage_full_hash = Mã hóa hàm
stage_loading_cache = Đang tải bộ nhớ đệm
stage_saving_cache = Lưu bộ nhớ đệm
stage_calculating_image_hashes = Tính toán giá trị băm của ảnh
stage_comparing_images = So sánh hình ảnh
stage_calculating_video_hashes = Tính toán giá trị băm của video
stage_checking_files = Đang kiểm tra các tệp
stage_checking_extensions = Đang kiểm tra các tiện ích mở rộng
stage_checking_names = Kiểm tra tên
stage_reading_music_tags = Đọc thông tin thẻ nhạc
stage_comparing_tags = So sánh các thẻ
stage_calculating_music_fingerprints = Tính toán dấu vân tay âm nhạc
stage_comparing_fingerprints = So sánh dấu vân tay
stage_extracting_exif = Đọc các thẻ EXIF
stage_creating_video_thumbnails = Tạo hình thu nhỏ cho video
stage_processing_videos = Đang xử lý video
stage_deleting = Xóa các tệp
stage_renaming = Đổi tên tệp
stage_moving = Di chuyển các tập tin
stage_hardlinking = Tạo liên kết cứng
stage_symlinking = Tạo liên kết tượng trưng
stage_optimizing_videos = Tối ưu hóa video
stage_cleaning_exif = Loại bỏ thông tin EXIF
# Group headers in scan results
duplicates_group_header = { $count } tệp tin x { $per_file } / tệp tin = { $total } tổng cộng
similar_images_group_header = { $count } hình ảnh tương tự
same_music_group_header = { $count } bản nhạc tương tự
# Rename confirmation
confirm_rename_items = Bạn có chắc chắn muốn đổi tên { $n } tệp đã chọn không?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Lớn nhất
option_search_mode_smallest = Nhỏ nhất
option_similarity_very_high = V. Cực cao
option_similarity_high = Cao
option_similarity_medium = Trung bình
option_similarity_low = Thấp
option_similarity_very_low = Rất thấp
option_similarity_minimal = Tối thiểu.
option_check_method_hash = Mã băm
option_check_method_name = Tên
option_check_method_size_and_name = Kích thước + Tên
option_check_method_size = Kích thước
option_music_method_tags = Các thẻ
option_music_method_audio = Âm thanh
option_min_size_none = Không có
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Không giới hạn
# Volume labels (shown in the directories screen)
volume_internal_storage = Bộ nhớ trong
volume_sd_card = Thẻ nhớ (thẻ SD)
volume_storage = Dung lượng lưu trữ
# Directories screen
directories_referenced_tooltip = Đã tham chiếu (chưa bị xóa)
directories_include_section_header = ĐÃ BAO GỒM
directories_exclude_section_header = LOẠI TRỪ
directories_custom_paths = Đường dẫn tùy chỉnh
directories_check_button = Phân tích
directories_check_popup_title = Thống kê thư mục
directories_check_label_included = Các đường dẫn được bao gồm:
directories_check_label_excluded = Các đường dẫn bị loại trừ:
directories_check_label_referenced = Đường dẫn tham khảo:
directories_check_label_would_scan = Các tệp cần quét:
directories_check_label_processable = Các loại tệp có thể xử lý:
directories_check_scanning = Đang quét...
directories_check_warning_no_processable = Không tìm thấy tệp nào có thể xử lý - hãy kiểm tra lại các thư mục đã được bao gồm/loại trừ
path_edit_title_include = Thêm vào để bao gồm
path_edit_title_exclude = Thêm vào danh sách loại trừ
path_edit_placeholder = Nhập đường dẫn...
path_edit_not_exists = Đường dẫn không tồn tại
path_edit_is_dir = Thư mục
path_edit_is_file = Tệp
path_edit_no_newlines = Đường dẫn không được chứa ký tự xuống dòng — Không được sử dụng phím Enter
ctx_menu_title = Mở
ctx_open_file = Mục đang mở
ctx_open_folder = Mở thư mục cha
