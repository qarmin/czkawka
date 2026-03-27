# Window titles
window_settings_title = Cài đặt
window_main_title = Czkawka (Hiccup)
window_progress_title = Đang quét
window_compare_images = So sánh hình ảnh
# General
general_ok_button = OK
general_close_button = Đóng
# Krokiet info dialog
krokiet_info_title = Giới thiệu Krokiet - Phiên bản mới của Czkawka
krokiet_info_message = 
        Krokiet là phiên bản mới, cải tiến, nhanh hơn và đáng tin cậy hơn của giao diện người dùng Czkawka GTK!

        Nó dễ sử dụng hơn và ít bị ảnh hưởng bởi các thay đổi hệ thống hơn, vì nó chỉ phụ thuộc vào các thư viện cốt lõi có sẵn trên hầu hết các hệ thống theo mặc định.

        Krokiet cũng mang đến các tính năng mà Czkawka không có, bao gồm hình thu nhỏ trong chế độ so sánh video, một công cụ làm sạch EXIF, thanh tiến trình cho việc di chuyển/sao chép/xóa tệp hoặc các tùy chọn sắp xếp nâng cao.

        Hãy thử và cảm nhận sự khác biệt!

        Czkawka vẫn sẽ tiếp tục nhận được các bản sửa lỗi và các bản cập nhật nhỏ từ tôi, nhưng tất cả các tính năng mới sẽ chỉ được phát triển dành riêng cho Krokiet, và bất kỳ ai cũng có thể đóng góp các tính năng mới, thêm các chế độ bị thiếu hoặc mở rộng Czkawka hơn nữa.

        P.S: Thông báo này chỉ nên hiển thị một lần. Nếu nó xuất hiện lại, hãy đặt biến môi trường CZKAWKA_DONT_ANNOY_ME thành bất kỳ giá trị nào khác trống.
# Main window
music_title_checkbox = Tiêu đề
music_artist_checkbox = Nghệ sĩ
music_year_checkbox = Năm
music_bitrate_checkbox = Tốc độ bit
music_genre_checkbox = Thể loại
music_length_checkbox = Độ dài
music_comparison_checkbox = So sánh tương đối
music_checking_by_tags = Các thẻ
music_checking_by_content = Nội dung
same_music_seconds_label = Độ dài tối thiểu của đoạn thứ hai
same_music_similarity_label = Sự khác biệt lớn nhất
music_compare_only_in_title_group = So sánh trong các nhóm sách có tựa đề tương tự
music_compare_only_in_title_group_tooltip = 
        Khi được kích hoạt, các tệp sẽ được nhóm theo tiêu đề và sau đó so sánh với nhau.

        Với 10.000 tệp, thay vì thường phải thực hiện gần 100 triệu lần so sánh, con số này sẽ giảm xuống còn khoảng 20.000 lần so sánh.
same_music_tooltip = 
        Bạn có thể cấu hình việc tìm kiếm các tệp nhạc tương tự dựa trên nội dung của chúng bằng cách thiết lập:

        - Thời gian tối thiểu của một đoạn nhạc mà sau đó các tệp nhạc có thể được xác định là tương tự.
        - Sự khác biệt tối đa giữa hai đoạn nhạc được kiểm tra.

        Điều quan trọng để có kết quả tốt là tìm ra các kết hợp hợp lý của các tham số này, như đã được cung cấp.

        Việc đặt thời gian tối thiểu là 5 giây và sự khác biệt tối đa là 1.0 sẽ tìm kiếm các đoạn nhạc gần như giống hệt nhau trong các tệp.
        Ngược lại, việc đặt thời gian là 20 giây và sự khác biệt tối đa là 6.0 hoạt động tốt để tìm các bản remix/phiên bản live, v.v.

        Theo mặc định, mỗi tệp nhạc được so sánh với tất cả các tệp khác, và điều này có thể tốn rất nhiều thời gian khi kiểm tra nhiều tệp, vì vậy thường tốt hơn là sử dụng các thư mục tham chiếu và chỉ định các tệp nào cần được so sánh với nhau (với cùng số lượng tệp, việc so sánh dấu vân tay sẽ nhanh hơn ít nhất 4 lần so với việc không sử dụng thư mục tham chiếu).
music_comparison_checkbox_tooltip = 
        Nó sẽ tìm kiếm các tệp nhạc tương tự bằng trí tuệ nhân tạo, sử dụng học máy để loại bỏ dấu ngoặc khỏi một cụm từ. Ví dụ, với tùy chọn này được bật, các tệp được đề cập sẽ được coi là trùng lặp:

        Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Phân biệt chữ hoa chữ thường
duplicate_case_sensitive_name_tooltip = 
        Khi được bật, chỉ nhóm các bản ghi khi chúng có tên giống hệt nhau, ví dụ: Żołd <-> Żołd.

        Tắt tùy chọn này sẽ nhóm các tên mà không kiểm tra xem mỗi chữ có cùng kích thước hay không, ví dụ: żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Kích thước và Tên
duplicate_mode_name_combo_box = Tên
duplicate_mode_size_combo_box = Kích thước
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip = 
        Czkawka cung cấp 3 loại hàm băm:

        Blake3: Hàm băm mật mã. Đây là mặc định vì nó rất nhanh.

        CRC32: Hàm băm đơn giản. Hàm này có thể nhanh hơn Blake3, nhưng có thể rất hiếm khi xảy ra xung đột.

        XXH3: Hiệu suất và chất lượng băm tương tự như Blake3 (nhưng không phải là hàm băm mật mã). Do đó, các chế độ này có thể dễ dàng thay đổi cho nhau.
duplicate_check_method_tooltip = 
        Hiện tại, Czkawka cung cấp ba loại phương pháp để tìm các bản sao trùng lặp:

        Tên: Tìm các tệp có cùng tên.

        Kích thước: Tìm các tệp có cùng kích thước.

        Hash: Tìm các tệp có cùng nội dung. Chế độ này tính toán giá trị hash của tệp và sau đó so sánh giá trị hash này để tìm các bản sao trùng lặp. Đây là cách an toàn nhất để tìm các bản sao trùng lặp. Ứng dụng sử dụng bộ nhớ đệm (cache) rất nhiều, vì vậy các lần quét tiếp theo của cùng một dữ liệu sẽ nhanh hơn đáng kể so với lần quét đầu tiên.
image_hash_size_tooltip = 
        Mỗi ảnh được kiểm tra sẽ tạo ra một mã hash đặc biệt, có thể so sánh với nhau. Sự khác biệt nhỏ giữa các mã hash này cho thấy các ảnh đó tương tự nhau.

        Kích thước mã hash 8 hoạt động khá tốt để tìm các ảnh chỉ hơi giống với ảnh gốc. Tuy nhiên, với một tập ảnh lớn hơn (>1000 ảnh), điều này có thể tạo ra rất nhiều kết quả sai, vì vậy tôi khuyên bạn nên sử dụng kích thước mã hash lớn hơn trong trường hợp này.

        Kích thước mã hash 16 là giá trị mặc định, và đây là một sự cân bằng tốt giữa việc tìm các ảnh tương tự, ngay cả khi chỉ hơi giống, và giảm thiểu số lượng va chạm mã hash.

        Mã hash 32 và 64 chỉ tìm các ảnh rất giống nhau, và dự kiến sẽ có rất ít hoặc không có kết quả sai (có thể ngoại trừ một số ảnh có kênh alpha).
image_resize_filter_tooltip = 
        Để tính toán giá trị hash của ảnh, thư viện phải điều chỉnh kích thước ảnh trước.

        Tùy thuộc vào thuật toán được chọn, hình ảnh kết quả được sử dụng để tính toán giá trị hash sẽ có vẻ hơi khác nhau.

        Thuật toán nhanh nhất để sử dụng, nhưng cũng là thuật toán cho kết quả tệ nhất, là thuật toán "Nearest". Nó được bật theo mặc định, vì với kích thước giá trị hash là 16x16, sự khác biệt về chất lượng không thực sự đáng chú ý.

        Với kích thước giá trị hash là 8x8, nên sử dụng một thuật toán khác ngoài "Nearest" để có các nhóm ảnh tốt hơn.
image_hash_alg_tooltip = 
        Người dùng có thể lựa chọn từ nhiều thuật toán khác nhau để tính toán giá trị băm.

        Mỗi thuật toán có những ưu điểm và nhược điểm riêng, và đôi khi sẽ cho kết quả tốt hơn, đôi khi lại cho kết quả tệ hơn, tùy thuộc vào từng hình ảnh.

        Do đó, để xác định thuật toán tốt nhất cho bạn, cần phải thực hiện kiểm tra thủ công.
big_files_mode_combobox_tooltip = Cho phép tìm kiếm các tập tin có kích thước nhỏ nhất/lớn nhất
big_files_mode_label = Các tệp đã được kiểm tra
big_files_mode_smallest_combo_box = Nhỏ nhất
big_files_mode_biggest_combo_box = Lớn nhất
main_notebook_duplicates = Tệp trùng lặp
main_notebook_empty_directories = Thư mục trống
main_notebook_big_files = Các tập tin lớn
main_notebook_empty_files = Các tệp trống
main_notebook_temporary = Tệp tạm
main_notebook_similar_images = Hình ảnh tương tự
main_notebook_similar_videos = Các video tương tự
main_notebook_same_music = Bản sao nhạc
main_notebook_symlinks = Liên kết tượng trưng không hợp lệ
main_notebook_broken_files = Tệp tin bị lỗi
main_notebook_bad_extensions = Các tiện ích mở rộng kém chất lượng
main_tree_view_column_file_name = Tên tệp
main_tree_view_column_folder_name = Tên thư mục
main_tree_view_column_path = Đường đi
main_tree_view_column_modification = Ngày sửa đổi
main_tree_view_column_size = Kích thước
main_tree_view_column_similarity = Tính tương đồng
main_tree_view_column_dimensions = Kích thước
main_tree_view_column_title = Tiêu đề
main_tree_view_column_artist = Nghệ sĩ
main_tree_view_column_year = Năm
main_tree_view_column_bitrate = Tốc độ bit
main_tree_view_column_length = Độ dài
main_tree_view_column_genre = Thể loại
main_tree_view_column_symlink_file_name = Tên tệp liên kết tượng trưng
main_tree_view_column_symlink_folder = Thư mục liên kết tượng trưng
main_tree_view_column_destination_path = Đường dẫn đích
main_tree_view_column_type_of_error = Loại lỗi
main_tree_view_column_current_extension = Phiên bản mở rộng hiện tại
main_tree_view_column_proper_extensions = Độ mở rộng phù hợp
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Codec
main_label_check_method = Kiểm tra phương pháp
main_label_hash_type = Loại hàm băm
main_label_hash_size = Kích thước hàm
main_label_size_bytes = Kích thước (byte)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Số lượng tệp hiển thị
main_label_resize_algorithm = Thuật toán thay đổi kích thước
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = Âm thanh
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Lưu trữ
main_check_box_broken_files_image = Hình ảnh
main_check_box_broken_files_video = Video
main_check_box_broken_files_video_tooltip = Sử dụng ffmpeg/ffprobe để kiểm tra tính hợp lệ của các tệp video. Quá trình này khá chậm và có thể phát hiện ra những lỗi nhỏ nhặt ngay cả khi tệp video vẫn hoạt động bình thường.
check_button_general_same_size = Bỏ qua kích thước giống nhau
check_button_general_same_size_tooltip = Bỏ qua các tệp có kích thước giống nhau trong kết quả - thường thì đây là các bản sao giống hệt nhau
main_label_size_bytes_tooltip = Kích thước của các tệp sẽ được sử dụng trong quá trình quét
# Upper window
upper_tree_view_included_folder_column_title = Các thư mục cần tìm kiếm
upper_tree_view_included_reference_column_title = Thư mục tham khảo
upper_recursive_button = Đệ quy
upper_recursive_button_tooltip = Nếu được chọn, tìm kiếm sẽ bao gồm cả các tệp không nằm trực tiếp trong các thư mục đã chọn.
upper_manual_add_included_button = Thêm thủ công
upper_add_included_button = Thêm
upper_remove_included_button = Xóa
upper_manual_add_excluded_button = Thêm thủ công
upper_add_excluded_button = Thêm
upper_remove_excluded_button = Xóa
upper_manual_add_included_button_tooltip = 
        Thêm tên thư mục để tìm kiếm thủ công.

        Để thêm nhiều đường dẫn cùng lúc, hãy phân tách chúng bằng dấu ";".

        /home/roman;/home/rozkaz sẽ thêm hai thư mục /home/roman và /home/rozkaz
upper_add_included_button_tooltip = Thêm thư mục mới vào danh sách tìm kiếm.
upper_remove_included_button_tooltip = Xóa thư mục khỏi kết quả tìm kiếm.
upper_manual_add_excluded_button_tooltip = 
        Thêm tên thư mục bị loại trực tiếp.

        Để thêm nhiều đường dẫn cùng lúc, hãy phân tách chúng bằng dấu ";".

        Ví dụ: "/home/roman;/home/krokiet" sẽ thêm hai thư mục /home/roman và /home/keokiet
upper_add_excluded_button_tooltip = Thêm thư mục cần loại trừ khỏi quá trình tìm kiếm.
upper_remove_excluded_button_tooltip = Xóa thư mục khỏi danh sách bị loại trừ.
upper_notebook_items_configuration = Cấu hình mục
upper_notebook_excluded_directories = Các đường dẫn bị loại trừ
upper_notebook_included_directories = Các đường dẫn được bao gồm
upper_allowed_extensions_tooltip = 
        Các phần mở rộng được phép phải được phân tách bằng dấu phẩy (mặc định là tất cả đều được cho phép).

        Các Macro sau đây, cho phép thêm nhiều phần mở rộng cùng lúc, cũng có sẵn: IMAGE, VIDEO, MUSIC, TEXT.

        Ví dụ sử dụng: ".exe, IMAGE, VIDEO, .rar, 7z" - điều này có nghĩa là các tệp hình ảnh (ví dụ: jpg, png), video (ví dụ: avi, mp4), tệp .exe, .rar và .7z sẽ được quét.
upper_excluded_extensions_tooltip = 
        Danh sách các tệp bị vô hiệu hóa, những tệp này sẽ bị bỏ qua trong quá trình quét.

        Khi sử dụng cả danh sách các phần mở rộng được phép và danh sách các phần mở rộng bị vô hiệu hóa, danh sách này (danh sách các phần mở rộng bị vô hiệu hóa) sẽ được ưu tiên hơn, do đó tệp sẽ không được kiểm tra.
upper_excluded_items_tooltip = 
        Các mục bị loại trừ phải chứa ký tự "*wildcard*" và được phân tách bằng dấu phẩy.
        Cách này chậm hơn so với "Excluded Paths", vì vậy hãy sử dụng nó một cách cẩn thận.
upper_excluded_items = Các mục bị loại trừ:
upper_allowed_extensions = Các định dạng tệp được phép:
upper_excluded_extensions = Các tiện ích bị tắt:
# Popovers
popover_select_all = Chọn tất cả
popover_unselect_all = Bỏ chọn tất cả
popover_reverse = Chọn ngược
popover_select_all_except_shortest_path = Chọn tất cả các tùy chọn, ngoại trừ "đường đi ngắn nhất"
popover_select_all_except_longest_path = Chọn tất cả các đường dẫn, ngoại trừ đường dẫn dài nhất
popover_select_all_except_oldest = Chọn tất cả trừ mục cũ nhất
popover_select_all_except_newest = Chọn tất cả trừ mục mới nhất
popover_select_one_oldest = Chọn một sản phẩm lâu đời nhất
popover_select_one_newest = Chọn một sản phẩm mới nhất
popover_select_custom = Chọn tùy chỉnh
popover_unselect_custom = Bỏ chọn tùy chỉnh
popover_select_all_images_except_biggest = Chọn tất cả các mục trừ mục lớn nhất
popover_select_all_images_except_smallest = Chọn tất cả các mục trừ mục nhỏ nhất
popover_custom_path_check_button_entry_tooltip = 
        Chọn các bản ghi theo đường dẫn.

        Ví dụ sử dụng:
        Tệp /home/pimpek/rzecz.txt có thể được tìm thấy với đường dẫn /home/pim*
popover_custom_name_check_button_entry_tooltip = 
        Chọn các bản ghi theo tên tệp.

        Ví dụ sử dụng:
        Tệp /usr/ping/pong.txt có thể được tìm thấy với cú pháp *ong*
popover_custom_regex_check_button_entry_tooltip = 
        Chọn các bản ghi theo biểu thức chính quy (Regex) đã chỉ định.

        Ở chế độ này, văn bản được tìm kiếm là đường dẫn có tên.

        Ví dụ sử dụng:
        Tệp "/usr/bin/ziemniak.txt" có thể được tìm thấy bằng biểu thức chính quy "/ziem[a-z]+"

        Điều này sử dụng triển khai biểu thức chính quy mặc định của Rust. Bạn có thể tìm hiểu thêm về nó tại đây: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip = 
        Cho phép phát hiện phân biệt chữ hoa chữ thường.

        Khi tính năng này bị tắt, `/home/*` sẽ tìm thấy cả `/HoMe/roman` và `/home/roman`.
popover_custom_not_all_check_button_tooltip = 
        Ngăn chặn việc chọn tất cả các bản ghi trong một nhóm.

        Tính năng này được bật theo mặc định, vì trong hầu hết các trường hợp, bạn không muốn xóa cả tệp gốc và các bản sao, mà muốn giữ lại ít nhất một tệp.

        CẢNH BÁO: Cài đặt này không hoạt động nếu bạn đã chọn thủ công tất cả các kết quả trong một nhóm.
popover_custom_regex_path_label = Đường dẫn
popover_custom_regex_name_label = Tên
popover_custom_regex_regex_label = Đường dẫn Regex + Tên
popover_custom_case_sensitive_check_button = Phân biệt chữ hoa chữ thường
popover_custom_all_in_group_label = Không chọn tất cả các bản ghi trong nhóm
popover_custom_mode_unselect = Bỏ chọn "Tùy chỉnh"
popover_custom_mode_select = Chọn Tùy chỉnh
popover_sort_file_name = Tên tệp
popover_sort_folder_name = Tên thư mục
popover_sort_full_name = Tên đầy đủ
popover_sort_size = Kích thước
popover_sort_selection = Lựa chọn
popover_invalid_regex = Biểu thức chính quy không hợp lệ
popover_valid_regex = Biểu thức chính quy hợp lệ
# Bottom buttons
bottom_search_button = Tìm kiếm
bottom_select_button = Chọn
bottom_delete_button = Xóa
bottom_save_button = Lưu lại
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Di chuyển
bottom_sort_button = Sắp xếp
bottom_compare_button = So sánh
bottom_search_button_tooltip = Bắt đầu tìm kiếm
bottom_select_button_tooltip = Chọn các bản ghi. Chỉ các tệp/thư mục đã chọn mới có thể được xử lý sau.
bottom_delete_button_tooltip = Xóa các tệp/thư mục đã chọn.
bottom_save_button_tooltip = Lưu dữ liệu tìm kiếm vào file
bottom_symlink_button_tooltip = 
        Tạo các liên kết tượng trưng.
        Chỉ hoạt động khi ít nhất hai kết quả trong một nhóm được chọn.
        Kết quả đầu tiên không thay đổi, và các kết quả thứ hai trở đi sẽ được tạo liên kết tượng trưng đến kết quả đầu tiên.
bottom_hardlink_button_tooltip = 
        Tạo liên kết cứng.
        Chỉ hoạt động khi có ít nhất hai kết quả được chọn trong một nhóm.
        Kết quả đầu tiên không thay đổi, và các kết quả thứ hai trở đi sẽ được liên kết cứng với kết quả đầu tiên.
bottom_hardlink_button_not_available_tooltip = 
        Tạo liên kết cứng.
        Nút này bị tắt, vì không thể tạo liên kết cứng.
        Liên kết cứng chỉ hoạt động với quyền quản trị viên trên Windows, vì vậy hãy đảm bảo chạy ứng dụng với quyền quản trị viên.
        Nếu ứng dụng đã chạy với quyền như vậy, hãy kiểm tra các vấn đề tương tự trên Github.
bottom_move_button_tooltip = 
        Di chuyển các tệp đến thư mục được chọn.
        Nó sao chép tất cả các tệp vào thư mục mà không giữ lại cấu trúc thư mục gốc.
        Khi cố gắng di chuyển hai tệp có cùng tên vào một thư mục, thao tác với tệp thứ hai sẽ thất bại và hiển thị thông báo lỗi.
bottom_sort_button_tooltip = Sắp xếp các tệp/thư mục theo phương pháp đã chọn.
bottom_compare_button_tooltip = So sánh các hình ảnh trong nhóm.
bottom_show_errors_tooltip = Hiện/Ẩn bảng điều khiển văn bản phía dưới.
bottom_show_upper_notebook_tooltip = Hiện/Ẩn bảng điều khiển phía trên của sổ.
# Progress Window
progress_stop_button = Dừng lại
progress_stop_additional_message = Yêu cầu dừng đã được thực hiện
# About Window
about_repository_button_tooltip = Liên kết đến trang lưu trữ mã nguồn.
about_donation_button_tooltip = Liên kết đến trang quyên góp.
about_instruction_button_tooltip = Liên kết đến trang hướng dẫn.
about_translation_button_tooltip = Liên kết đến trang Crowdin với các bản dịch ứng dụng. Hiện tại, tiếng Ba Lan và tiếng Anh là các ngôn ngữ được hỗ trợ chính thức.
about_repository_button = Kho lưu trữ
about_donation_button = Quyên góp
about_instruction_button = Hướng dẫn
about_translation_button = Dịch thuật
# Header
header_setting_button_tooltip = Mở hộp thoại cài đặt.
header_about_button_tooltip = Mở hộp thoại hiển thị thông tin về ứng dụng.

# Settings


## General

settings_number_of_threads = Số lượng luồng đang được sử dụng
settings_number_of_threads_tooltip = Số lượng luồng được sử dụng, 0 có nghĩa là tất cả các luồng khả dụng sẽ được sử dụng.
settings_use_rust_preview = Sử dụng các thư viện bên ngoài thay vì gtk để tải các hình xem trước
settings_use_rust_preview_tooltip = 
        Việc sử dụng bản xem trước của gtk đôi khi có thể nhanh hơn và hỗ trợ nhiều định dạng hơn, nhưng đôi khi điều này có thể hoàn toàn ngược lại.

        Nếu bạn gặp vấn đề khi tải bản xem trước, bạn có thể thử thay đổi cài đặt này.

        Trên các hệ thống không phải Linux, nên sử dụng tùy chọn này, vì các tệp gtk-pixbuf không phải lúc nào cũng có sẵn, vì vậy việc tắt tùy chọn này sẽ không tải bản xem trước của một số hình ảnh.
settings_label_restart = Bạn cần khởi động lại ứng dụng để áp dụng các thiết lập!
settings_ignore_other_filesystems = Bỏ qua các hệ thống tập tin khác (chỉ dành cho Linux)
settings_ignore_other_filesystems_tooltip = 
        Bỏ qua các tệp không nằm trong cùng hệ thống tệp với các thư mục được tìm kiếm.

        Hoạt động tương tự như tùy chọn `-xdev` trong lệnh `find` trên Linux
settings_save_at_exit_button_tooltip = Lưu cấu hình vào file khi đóng ứng dụng.
settings_load_at_start_button_tooltip = 
        Tải cấu hình từ tệp khi mở ứng dụng.

        Nếu tính năng này chưa được bật, các cài đặt mặc định sẽ được sử dụng.
settings_confirm_deletion_button_tooltip = Hiển thị hộp thoại xác nhận khi người dùng nhấp vào nút xóa.
settings_confirm_link_button_tooltip = Hiển thị hộp thoại xác nhận khi người dùng nhấp vào nút "liên kết cứng/liên kết tượng trưng".
settings_confirm_group_deletion_button_tooltip = Hiển thị hộp thoại cảnh báo khi người dùng cố gắng xóa tất cả các bản ghi trong nhóm.
settings_show_text_view_button_tooltip = Hiển thị bảng điều khiển văn bản ở phía dưới giao diện người dùng.
settings_use_cache_button_tooltip = Sử dụng bộ nhớ đệm tệp.
settings_save_also_as_json_button_tooltip = Lưu bộ nhớ đệm dưới định dạng JSON (dễ đọc). Bạn có thể chỉnh sửa nội dung của nó. Ứng dụng sẽ tự động đọc bộ nhớ đệm từ tệp này nếu không tìm thấy bộ nhớ đệm ở định dạng nhị phân (có phần mở rộng .bin).
settings_use_trash_button_tooltip = Di chuyển các tệp vào thùng rác thay vì xóa chúng vĩnh viễn.
settings_language_label_tooltip = Ngôn ngữ cho giao diện người dùng.
settings_save_at_exit_button = Lưu cấu hình khi đóng ứng dụng
settings_load_at_start_button = Tải cấu hình khi mở ứng dụng
settings_confirm_deletion_button = Hiển thị hộp thoại xác nhận khi xóa bất kỳ tệp nào
settings_confirm_link_button = Hiển thị hộp thoại xác nhận khi tạo liên kết cứng hoặc liên kết tượng trưng cho bất kỳ tệp nào
settings_confirm_group_deletion_button = Hiển thị hộp thoại xác nhận khi xóa tất cả các tệp trong một nhóm
settings_show_text_view_button = Hiển thị bảng điều khiển văn bản ở dưới cùng
settings_use_cache_button = Sử dụng bộ nhớ đệm
settings_save_also_as_json_button = Ngoài ra, hãy lưu bộ nhớ đệm dưới dạng tệp JSON
settings_use_trash_button = Di chuyển các tệp đã xóa đến thùng rác
settings_language_label = Ngôn ngữ
settings_multiple_delete_outdated_cache_checkbutton = Tự động xóa các mục bộ nhớ đệm đã cũ
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Xóa các kết quả lưu trữ cũ trỏ đến các tệp không tồn tại.

        Khi tính năng này được bật, ứng dụng sẽ đảm bảo rằng khi tải các bản ghi, tất cả các bản ghi đều trỏ đến các tệp hợp lệ (các tệp bị lỗi sẽ bị bỏ qua).

        Tắt tính năng này sẽ hữu ích khi quét các tệp trên ổ đĩa ngoài, để các mục nhập lưu trữ liên quan đến chúng sẽ không bị xóa trong lần quét tiếp theo.

        Trong trường hợp có hàng trăm nghìn bản ghi trong bộ nhớ cache, nên bật tính năng này, điều này sẽ giúp tăng tốc độ tải/lưu bộ nhớ cache ở đầu/cuối quá trình quét.
settings_notebook_general = Tổng quan
settings_notebook_duplicates = Bản sao
settings_notebook_images = Hình ảnh tương tự
settings_notebook_videos = Video tương tự

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Hiển thị bản xem trước ở phía bên phải (khi chọn một tệp hình ảnh).
settings_multiple_image_preview_checkbutton = Hiển thị hình ảnh xem trước
settings_multiple_clear_cache_button_tooltip = 
        Xóa bộ nhớ cache thủ công để loại bỏ các mục đã hết hạn.
        Chỉ nên sử dụng tùy chọn này khi tính năng xóa tự động đã bị tắt.
settings_multiple_clear_cache_button = Xóa các kết quả đã cũ khỏi bộ nhớ đệm.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip = 
        Ẩn tất cả các tệp, ngoại trừ một, nếu tất cả chúng đều trỏ đến cùng một dữ liệu (được liên kết cứng).

        Ví dụ: Trong trường hợp có bảy tệp trên ổ đĩa được liên kết cứng đến cùng một dữ liệu, và một tệp khác có cùng dữ liệu nhưng inode khác, thì trong trình tìm kiếm bản sao, chỉ hiển thị một tệp duy nhất và một tệp từ các tệp được liên kết cứng.
settings_duplicates_minimal_size_entry_tooltip = 
        Thiết lập kích thước tệp tối thiểu sẽ được lưu trong bộ nhớ đệm.

        Việc chọn giá trị nhỏ hơn sẽ tạo ra nhiều bản ghi hơn. Điều này sẽ tăng tốc độ tìm kiếm, nhưng làm chậm quá trình tải/lưu bộ nhớ đệm.
settings_duplicates_prehash_checkbutton_tooltip = 
        Cho phép lưu trữ tạm thời (cache) giá trị prehash (một giá trị băm được tính từ một phần nhỏ của tệp), điều này cho phép loại bỏ nhanh chóng hơn các kết quả không trùng lặp.

        Tính năng này bị tắt theo mặc định vì nó có thể gây ra tình trạng chậm lại trong một số trường hợp.

        Nên bật tính năng này khi quét hàng trăm nghìn hoặc hàng triệu tệp, vì nó có thể tăng tốc độ tìm kiếm lên nhiều lần.
settings_duplicates_prehash_minimal_entry_tooltip = Kích thước tối thiểu của mục được lưu trong bộ nhớ đệm.
settings_duplicates_hide_hard_link_button = Ẩn các liên kết cứng
settings_duplicates_prehash_checkbutton = Sử dụng bộ nhớ đệm trước khi tính toán hash
settings_duplicates_minimal_size_cache_label = Kích thước tối thiểu của các tệp (tính bằng byte) được lưu vào bộ nhớ cache
settings_duplicates_minimal_size_cache_prehash_label = Kích thước tối thiểu của các tệp (tính bằng byte) được lưu vào bộ nhớ đệm trước khi tính toán hash

## Saving/Loading settings

settings_saving_button_tooltip = Lưu cấu hình cài đặt hiện tại vào tệp.
settings_loading_button_tooltip = Tải cài đặt từ tệp và thay thế cấu hình hiện tại bằng các cài đặt đó.
settings_reset_button_tooltip = Khôi phục cấu hình hiện tại về cấu hình mặc định.
settings_saving_button = Lưu cấu hình
settings_loading_button = Tải cấu hình
settings_reset_button = Đặt lại cấu hình

## Opening cache/config folders

settings_folder_cache_open_tooltip = 
        Mở thư mục chứa các file txt cache.

        Việc sửa đổi các file cache có thể gây ra kết quả không chính xác. Tuy nhiên, việc thay đổi đường dẫn có thể tiết kiệm thời gian khi di chuyển một lượng lớn file đến một vị trí khác.

        Bạn có thể sao chép các file này giữa các máy tính để tiết kiệm thời gian khi phải quét lại các file (tất nhiên là nếu chúng có cấu trúc thư mục tương tự).

        Trong trường hợp có vấn đề với cache, bạn có thể xóa các file này. Ứng dụng sẽ tự động tạo lại chúng.
settings_folder_settings_open_tooltip = 
        Mở thư mục chứa cấu hình của Czkawka.

        CẢNH BÁO: Việc chỉnh sửa cấu hình thủ công có thể làm gián đoạn quy trình làm việc của bạn.
settings_folder_cache_open = Mở thư mục bộ nhớ đệm
settings_folder_settings_open = Mở thư mục cài đặt
# Compute results
compute_stopped_by_user = Quá trình tìm kiếm đã bị người dùng dừng lại
compute_found_duplicates_hash_size = Đã tìm thấy { $number_files } bản sao trong { $number_groups } nhóm, chiếm dung lượng { $size } và mất { $time }
compute_found_duplicates_name = Đã tìm thấy { $number_files } bản sao trong { $number_groups } nhóm trong { $time }
compute_found_empty_folders = Đã tìm thấy { $number_files } thư mục trống trong { $time }
compute_found_empty_files = Đã tìm thấy { $number_files } tệp trống trong { $time }
compute_found_big_files = Đã tìm thấy { $number_files } tệp lớn trong { $time }
compute_found_temporary_files = Đã tìm thấy { $number_files } tệp tạm thời trong { $time }
compute_found_images = Đã tìm thấy { $number_files } hình ảnh tương tự trong { $number_groups } nhóm, mất { $time }
compute_found_videos = Đã tìm thấy { $number_files } video tương tự trong { $number_groups } nhóm, mất { $time } để hoàn thành
compute_found_music = Đã tìm thấy { $number_files } tệp nhạc tương tự trong { $number_groups } nhóm, mất { $time }
compute_found_invalid_symlinks = Đã tìm thấy { $number_files } liên kết tượng trưng không hợp lệ trong { $time }
compute_found_broken_files = Đã tìm thấy { $number_files } tệp bị lỗi trong { $time }
compute_found_bad_extensions = Đã tìm thấy { $number_files } tệp có phần mở rộng không hợp lệ trong { $time }
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] Scanned { $file_number } file
       *[other] Scanned { $file_number } files
    }
progress_scanning_extension_of_files = Checked extension of { $file_checked }/{ $all_files } file
progress_scanning_broken_files = Checked { $file_checked }/{ $all_files } file ({ $data_checked }/{ $all_data })
progress_scanning_video = Hashed of { $file_checked }/{ $all_files } video
progress_creating_video_thumbnails = Created thumbnails of { $file_checked }/{ $all_files } video
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
progress_prehash_cache_loading = Đang tải bộ nhớ đệm trước khi tính toán hash
progress_prehash_cache_saving = Lưu bộ nhớ đệm trước khi tính toán hash
progress_hash_cache_loading = Đang tải bộ nhớ đệm hash
progress_hash_cache_saving = Lưu bộ nhớ đệm hash
progress_cache_loading = Đang tải bộ nhớ đệm
progress_cache_saving = Lưu bộ nhớ đệm
progress_current_stage = Current Stage:{ "  " }
progress_all_stages = All Stages:{ "  " }
# Saving loading 
saving_loading_saving_success = Đã lưu cấu hình vào file { $name }.
saving_loading_saving_failure = Không thể lưu dữ liệu cấu hình vào tệp { $name }, lý do: { $reason }.
saving_loading_reset_configuration = Cấu hình hiện tại đã được xóa.
saving_loading_loading_success = Cấu hình ứng dụng đã được tải đúng.
saving_loading_failed_to_create_config_file = Không thể tạo file cấu hình "{ $path }", lý do: "{ $reason }".
saving_loading_failed_to_read_config_file = Không thể tải cấu hình từ "{ $path }" vì nó không tồn tại hoặc không phải là một tệp.
saving_loading_failed_to_read_data_from_file = Không thể đọc dữ liệu từ tệp "{ $path }", lý do: "{ $reason }".
# Other
selected_all_reference_folders = Không thể bắt đầu tìm kiếm nếu tất cả các thư mục được đặt làm thư mục tham chiếu
searching_for_data = Đang tìm kiếm dữ liệu, quá trình này có thể mất một chút thời gian, vui lòng chờ...
text_view_messages = TIN NHẮN
text_view_warnings = CẢNH BÁO
text_view_errors = LỖI
about_window_motto = Chương trình này hoàn toàn miễn phí để sử dụng và sẽ luôn như vậy.
krokiet_new_app = Czkawka hiện đang ở chế độ bảo trì, điều này có nghĩa là chỉ các lỗi nghiêm trọng mới được sửa và không có tính năng mới nào được thêm vào. Để trải nghiệm các tính năng mới, vui lòng sử dụng ứng dụng Krokiet mới, ứng dụng này ổn định và hiệu suất tốt hơn, đồng thời vẫn đang được phát triển tích cực.
# Various dialog
dialogs_ask_next_time = Hỏi lần sau nhé
symlink_failed = Failed to symlink { $name } to { $target }, reason { $reason }
delete_title_dialog = Xác nhận xóa
delete_question_label = Bạn có chắc chắn rằng bạn muốn xóa các tệp này không?
delete_all_files_in_group_title = Xác nhận xóa tất cả các tệp trong nhóm
delete_all_files_in_group_label1 = Trong một số nhóm, tất cả các bản ghi đều được chọn.
delete_all_files_in_group_label2 = Bạn có chắc chắn rằng bạn muốn xóa chúng không?
delete_items_label = { $items } tập tin sẽ bị xóa.
delete_items_groups_label = { $items } tệp từ { $groups } nhóm sẽ bị xóa.
hardlink_failed = Không thể tạo liên kết cứng từ { $name } đến { $target }, lý do: { $reason }
hard_sym_invalid_selection_title_dialog = Lựa chọn không hợp lệ với một số nhóm
hard_sym_invalid_selection_label_1 = Trong một số nhóm, chỉ có một bản ghi được chọn và nó sẽ bị bỏ qua.
hard_sym_invalid_selection_label_2 = Để có thể tạo liên kết cứng hoặc liên kết tượng trưng cho các tệp này, bạn cần chọn ít nhất hai kết quả trong nhóm.
hard_sym_invalid_selection_label_3 = Trong một nhóm, phần tử đầu tiên được công nhận là bản gốc và không được thay đổi, nhưng các phần tử thứ hai trở đi sẽ được sửa đổi.
hard_sym_link_title_dialog = Xác nhận liên kết
hard_sym_link_label = Bạn có chắc chắn rằng bạn muốn liên kết các tệp này không?
move_folder_failed = Failed to move folder { $name }, reason { $reason }
move_file_failed = Failed to move file { $name }, reason { $reason }
move_files_title_dialog = Chọn thư mục mà bạn muốn chuyển các tập tin trùng lặp đến
move_files_choose_more_than_1_path = Only one path may be selected to be able to copy their duplicated files, selected { $path_number }.
move_stats = Properly moved { $num_files }/{ $all_files } items
save_results_to_file = Saved results both to txt and json files into "{ $name }" folder.
search_not_choosing_any_music = LỖI: Bạn phải chọn ít nhất một tùy chọn tìm kiếm nhạc.
search_not_choosing_any_broken_files = LỖI: Bạn phải chọn ít nhất một ô kiểm với loại là các tệp bị lỗi.
include_folders_dialog_title = Các thư mục cần đưa vào
exclude_folders_dialog_title = Các thư mục cần loại trừ
include_manually_directories_dialog_title = Thêm thư mục một cách thủ công
cache_properly_cleared = Đã xóa bộ nhớ cache thành công
cache_clear_duplicates_title = Xóa bộ nhớ đệm chứa các mục trùng lặp
cache_clear_similar_images_title = Xóa bộ nhớ đệm chứa các ảnh tương tự
cache_clear_similar_videos_title = Xóa bộ nhớ đệm của các video tương tự
cache_clear_message_label_1 = Bạn có muốn xóa bộ nhớ đệm chứa các mục đã cũ không?
cache_clear_message_label_2 = Thao tác này sẽ xóa tất cả các mục trong bộ nhớ cache trỏ đến các tệp không hợp lệ.
cache_clear_message_label_3 = Điều này có thể giúp tăng tốc độ tải/lưu vào bộ nhớ đệm một chút.
cache_clear_message_label_4 = CẢNH BÁO: Quá trình này sẽ xóa tất cả dữ liệu đã được lưu trữ tạm thời trên các ổ đĩa ngoài không được kết nối. Do đó, mỗi giá trị băm (hash) sẽ cần được tạo lại.
# Show preview
preview_image_resize_failure = Failed to resize image { $name }.
preview_image_opening_failure = Failed to open image { $name }, reason { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Nhóm { $current_group }/{ $all_groups } ({ $images_in_group } ảnh)
compare_move_left_button = L
compare_move_right_button = R
