# Window titles
window_settings_title = 设置
window_main_title = Czkawka
window_progress_title = 正在扫描
window_compare_images = 比较图像
# General
general_ok_button = 好的
general_close_button = 关闭
# Main window
music_title_checkbox = 标题
music_artist_checkbox = 艺人
music_year_checkbox = 年份
music_bitrate_checkbox = 码率
music_genre_checkbox = 流派
music_length_checkbox = 长度
music_comparison_checkbox = 近似比较
music_comparison_checkbox_tooltip =
    它使用 AI搜索类似的音乐文件，它使用机器学习从短语中删除括号。 例如，启用此选项， 所涉文件将被视为重复：
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = 区分大小写
duplicate_case_sensitive_name_tooltip =
    启用时，仅当记录具有完全相同的名称时分组，例如 Żołd <-> Żołd
    
    禁用这种选项将不会检查每封字母是否相同的大小，例如 żoŁD <-> Żołd
duplicate_mode_name_combo_box = 名称
duplicate_mode_size_combo_box = 大小
duplicate_mode_hash_combo_box = 哈希
duplicate_hash_type_tooltip =
    Czkawka提供3种哈希类型：
    
    Blake3 - 加密散列函数。 这是默认值，因为它很快。
    
    CRC32 - 简单的散列函数。 这应该比Blake3更快，但可能很少发生碰撞。
    
    XXH3 - 在性能和散列质量方面非常相似的 Blake3 (但非加密法) 。所以这种模式可以很容易地进行交流。
duplicate_check_method_tooltip =
    目前，Czkawka提供三种方法来查找重复：
    
    名称 - 查找具有相同名称的文件。
    
    大小 - 查找大小相同的文件。
    
    哈希-查找内容相同的文件。 此模式将文件哈希，然后比较此哈希以找到重复。此模式是找到重复的最安全方法。 应用大量使用缓存，所以第二和进一步扫描同一数据应该比第一个更快一些。
image_hash_size_tooltip =
    每张选中的图像都会产生可相互比较的特殊哈希值，而它们之间的小差意味着这张图像是相似的。
    
    8 个散列尺寸非常适合于找到与原始相似的图像。 随着更大的图像集(>1000) 将产生大量的虚假正数，因此我建议使用这种更大的散列大小。
    
    16 是默认的散列大小，它很好地影响了找到哪怕是略类似的图像和少量散列碰撞。
    
    32和64 哈希只找到非常相似的图像，但几乎不应该有任何假正数(可能只有一些透明通道的图像)。
image_resize_filter_tooltip =
    要计算图像散列，库必须首先调整大小。
    
    在选定的算法上花费钱，用来计算散列的结果图像可能看起来没有什么不同。
    
    最快使用的算法，但结果最差的算法是Nearest。 默认启用它，因为16x16散列尺寸较低的质量并不真正可见。
    
    建议使用 8x8 散列大小，使用不同于Nearest的算法来拥有更好的图像组。
image_hash_alg_tooltip =
    用户可以从许多计算哈希值的算法中选择一种。
    
    每种算法都有强项和弱项，对于不同的图像，有时会有更好的结果，有时会有更差的结果。
    
    因此，为了确定最适合你的算法，需要进行人工测试。
big_files_mode_combobox_tooltip = 允许搜索最小/最大的文件
big_files_mode_label = 已检查的文件
big_files_mode_smallest_combo_box = 最小的
big_files_mode_biggest_combo_box = 最大的
main_notebook_duplicates = 重复文件
main_notebook_empty_directories = 空目录
main_notebook_big_files = 大文件
main_notebook_empty_files = 空文件
main_notebook_temporary = 临时文件
main_notebook_similar_images = 相似图像
main_notebook_similar_videos = 相似视频
main_notebook_same_music = 重复音乐
main_notebook_symlinks = 无效的符号链接
main_notebook_broken_files = 损坏的文件
main_notebook_bad_extensions = 错误的扩展
main_tree_view_column_file_name = 文件名称
main_tree_view_column_folder_name = 文件夹名称
main_tree_view_column_path = 路径
main_tree_view_column_modification = 修改日期
main_tree_view_column_size = 大小
main_tree_view_column_similarity = 相似度
main_tree_view_column_dimensions = 尺寸
main_tree_view_column_title = 标题
main_tree_view_column_artist = 艺人
main_tree_view_column_year = 年份
main_tree_view_column_bitrate = 码率
main_tree_view_column_length = 长度
main_tree_view_column_genre = 流派
main_tree_view_column_symlink_file_name = 符号链接文件名
main_tree_view_column_symlink_folder = 符号链接文件夹
main_tree_view_column_destination_path = 目标路径
main_tree_view_column_type_of_error = 错误类型
main_tree_view_column_current_extension = 当前扩展
main_tree_view_column_proper_extensions = 合适的扩展
main_label_check_method = 检查方法
main_label_hash_type = 哈希类型
main_label_hash_size = 哈希大小
main_label_size_bytes = 大小 (字节)
main_label_min_size = 最小值
main_label_max_size = 最大值
main_label_shown_files = 显示的文件数
main_label_resize_algorithm = 调整算法
main_label_similarity = 相似性:{" "}
main_check_box_broken_files_audio = 音频
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = 归档
main_check_box_broken_files_image = 图像
check_button_general_same_size = 忽略相同的大小
check_button_general_same_size_tooltip = 从结果忽略相同大小的文件 - 通常是 1:1 重复
main_label_size_bytes_tooltip = 将用于扫描的文件大小
# Upper window
upper_tree_view_included_folder_column_title = 要搜索的文件夹
upper_tree_view_included_reference_column_title = 参考文件夹
upper_recursive_button = 递归的
upper_recursive_button_tooltip = 如果选中，也可以搜索未直接置于选定文件夹下的文件。
upper_manual_add_included_button = 手动添加
upper_add_included_button = 添加
upper_remove_included_button = 删除
upper_manual_add_excluded_button = 手动添加
upper_add_excluded_button = 添加
upper_remove_excluded_button = 删除
upper_manual_add_included_button_tooltip =
    手动添加目录名。
    
    一次性添加多个路径，用……分隔它们；
    
    /home/roman；/home/rozkaz 将添加两个目录/home/roman和/home/rozkaz
upper_add_included_button_tooltip = 添加新目录进行搜索。
upper_remove_included_button_tooltip = 从搜索中删除目录。
upper_manual_add_excluded_button_tooltip =
    手动添加排除的目录名称。
    
    一次要添加多个路径，请将它们分开。
    
    /home/roman;/home/krokiet 将添加两个目录/home/roman 和 /home/kookiet
upper_add_excluded_button_tooltip = 添加要排除在搜索中的目录。
upper_remove_excluded_button_tooltip = 从排除中删除目录。
upper_notebook_items_configuration = 项目配置
upper_notebook_excluded_directories = 排除的目录
upper_notebook_included_directories = 包含的目录
upper_allowed_extensions_tooltip =
    允许的扩展必须用逗号分隔(默认所有可用)。
    
    同时添加多个扩展的下列宏也可用: IMAGE, VIDEO, MUSIC, TEXT
    
    使用示例".exe, IMAGE, VIDEO, .rar, 7z" - 这意味着图像(e). . jpg、png、视频(例如avi、mp4)、ex、rar和7z 文件将被扫描。
upper_excluded_items_tooltip =
    排除的项目必须包含 * 通配符，并且应该用逗号分隔。
    这比排除的目录慢，所以请仔细使用。
upper_excluded_items = 排除的项目：
upper_allowed_extensions = 允许的扩展：
# Popovers
popover_select_all = 选择所有
popover_unselect_all = 取消选择所有
popover_reverse = 反向选择
popover_select_all_except_oldest = 选择除最老以外的所有选项
popover_select_all_except_newest = 选择除最新以外的所有选项
popover_select_one_oldest = 选择一个最旧的
popover_select_one_newest = 选择一个最新的
popover_select_custom = 选择自定义
popover_unselect_custom = 取消选择自定义
popover_select_all_images_except_biggest = 选择除最大以外的所有选项
popover_select_all_images_except_smallest = 选择除最小以外的所有
popover_custom_path_check_button_entry_tooltip =
    通过路径选择记录。
    
    示例用法：
    /home/pimpek/rzecz.txt可以通过 /home/pim* 找到
popover_custom_name_check_button_entry_tooltip =
    按文件名选择记录。
    
    示例用法：
    /usr/ping/pong.txt可以在 *ong* 中找到。
popover_custom_regex_check_button_entry_tooltip =
    通过指定的 Regex选择记录。
    
    使用这种模式，搜索的文本是名的路径。
    
    示例用法：
    /usr/bin/ziemniak。 xt 可以通过 /ziem[a-z]+
    
    找到。这使用默认的 Rust regex 实现。你可以在这里阅读更多关于它的信息：https://docs.rs/regex。
popover_custom_case_sensitive_check_button_tooltip =
    启用区分大小写的检测。
    
    当禁用/home/* 同时找到 /HoMe/roman 和 /home/roman。
popover_custom_not_all_check_button_tooltip =
    禁止在分组中选择所有记录。
    
    这是默认启用的，因为在大多数情况下， 您不想删除原始文件和重复文件，但想保留至少一个文件。
    
    警告：如果您已经手动选择了一个组中的所有结果，此设置将无法工作。
popover_custom_regex_path_label = 路径
popover_custom_regex_name_label = 名称
popover_custom_regex_regex_label = 正则表达式路径 + 名称
popover_custom_case_sensitive_check_button = 区分大小写
popover_custom_all_in_group_label = 不在组中选择所有记录
popover_custom_mode_unselect = 取消选择自定义
popover_custom_mode_select = 选择自定义
popover_sort_file_name = 文件名称
popover_sort_folder_name = 文件夹名称
popover_sort_full_name = 全名
popover_sort_size = 大小
popover_sort_selection = 选择
popover_invalid_regex = 正则表达式无效
popover_valid_regex = 正则表达式有效
# Bottom buttons
bottom_search_button = 搜索
bottom_select_button = 选择
bottom_delete_button = 删除
bottom_save_button = 保存
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = 移动
bottom_sort_button = 排序
bottom_search_button_tooltip = 开始搜索
bottom_select_button_tooltip = 选择记录。只能稍后处理选定的文件/文件夹。
bottom_delete_button_tooltip = 删除选中的文件/文件夹。
bottom_save_button_tooltip = 保存搜索数据到文件
bottom_symlink_button_tooltip =
    创建符号链接。
    只能在选择组中至少两个结果时生效。
    第一个不变，第二个和以后两个都是对称的。
bottom_hardlink_button_tooltip =
    创建硬链接。
    只在选定组中至少两个结果时有效。
    第一个不变，第二个和第二个后来是与第一个联系在一起的。
bottom_hardlink_button_not_available_tooltip =
    创建硬链接。
    按钮已禁用，因为无法创建硬链接。
    硬链接仅适用于管理员在Windows上的权限，所以必须以管理员身份运行应用程序。
    如果应用程序已经使用了这种权限，请检查类似的 Github。
bottom_move_button_tooltip =
    移动文件到选定的目录。
    它复制所有文件到目录，而不保留目录树。
    试图将两个具有相同名称的文件移动到文件夹时，第二个将失败并显示错误。
bottom_sort_button_tooltip = 根据选定的方法排序文件/文件夹。
bottom_show_errors_tooltip = 显示/隐藏底部文本面板。
bottom_show_upper_notebook_tooltip = 显示/隐藏主笔记本面板。
# Progress Window
progress_stop_button = 停止
progress_stop_additional_message = 停止请求
# About Window
about_repository_button_tooltip = 链接到源代码的仓库页面。
about_donation_button_tooltip = 链接到捐赠页面。
about_instruction_button_tooltip = 链接到指令页面。
about_translation_button_tooltip = 链接到带有应用程序翻译的 Crowdin 页面。官方支持波兰语和英语。
about_repository_button = 存储库
about_donation_button = 捐助
about_instruction_button = 说明
about_translation_button = 翻译
# Header
header_setting_button_tooltip = 打开设置对话框。
header_about_button_tooltip = 打开包含应用程序信息的对话框。

# Settings


## General

settings_number_of_threads = 使用的线程数
settings_number_of_threads_tooltip = 用过的线程数，0表示所有可用线程都将被使用。
settings_label_restart = 您需要重新启动应用才能应用设置！
settings_ignore_other_filesystems = 忽略其它文件系统 (仅限Linux)
settings_ignore_other_filesystems_tooltip =
    忽略与搜索的目录不在同一个文件系统中的文件。
    
    在 Linux 上查找命令时类似-xdev选项
settings_save_at_exit_button_tooltip = 关闭应用时将配置保存到文件。
settings_load_at_start_button_tooltip =
    打开应用程序时从文件加载配置。
    
    如果未启用，将使用默认设置。
settings_confirm_deletion_button_tooltip = 点击删除按钮时显示确认对话框。
settings_confirm_link_button_tooltip = 点击硬链接/符号链接按钮时显示确认对话框。
settings_confirm_group_deletion_button_tooltip = 尝试从群组中删除所有记录时显示警告对话框。
settings_show_text_view_button_tooltip = 在用户界面底部显示文本面板。
settings_use_cache_button_tooltip = 使用文件缓存。
settings_save_also_as_json_button_tooltip = 保存缓存为 (人类可读) JSON 格式。可以修改其内容。 如果缺少二进制格式缓存(带bin extensional)，此文件的缓存将被应用自动读取。
settings_use_trash_button_tooltip = 将文件移至回收站，而将其永久删除。
settings_language_label_tooltip = 用户界面的语言。
settings_save_at_exit_button = 关闭应用时保存配置
settings_load_at_start_button = 打开应用程序时加载配置
settings_confirm_deletion_button = 删除任何文件时显示确认对话框
settings_confirm_link_button = 硬/符号链接任何文件时显示确认对话框
settings_confirm_group_deletion_button = 删除组中所有文件时显示确认对话框
settings_show_text_view_button = 显示底部文本面板
settings_use_cache_button = 使用缓存
settings_save_also_as_json_button = 同时将缓存保存为 JSON 文件
settings_use_trash_button = 移动已删除的文件到回收站
settings_language_label = 语言
settings_multiple_delete_outdated_cache_checkbutton = 自动删除过时的缓存条目
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    删除指向不存在文件的过时缓存结果。
    
    如果启用，应用会确保当加载记录时，所有记录都指向有效的文件(被忽略的文件)。
    
    禁用这将有助于扫描外部驱动器上的文件，所以在下次扫描中不会清除有关它们的缓存条目。
    
    在缓存中拥有成千上万条记录的情况下， 建议启用这个选项，这将加速扫描开始/结束时的缓存加载/保存。
settings_notebook_general = 概况
settings_notebook_duplicates = 重复项
settings_notebook_images = 相似图像
settings_notebook_videos = 相似视频

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = 在右侧显示预览 (当选择图像文件时)。
settings_multiple_image_preview_checkbutton = 显示图像预览
settings_multiple_clear_cache_button_tooltip =
    手动清除过时条目的缓存。
    仅在禁用自动清除时才使用。
settings_multiple_clear_cache_button = 从图像缓存中删除过时的结果

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    隐藏除一个之外的所有文件，如果所有文件都指向相同的数据(是硬链接)。
    
    示例：在存在(磁盘) 的情况下，七个文件与特定数据有硬链接，一个不同的文件具有相同的数据但不同的内涵， 然后在重复的查找器中，将只显示一个唯一的文件和一个来自硬链接的文件。
settings_duplicates_minimal_size_entry_tooltip =
    设置将缓存的最小文件大小。
    
    选择一个较小的值将生成更多的记录。 这将加速搜索，但减缓缓缓存加载/保存。
settings_duplicates_prehash_checkbutton_tooltip =
    启用封存(从文件的一小部分计算的散列) 的缓存，允许提早撤销不重复的结果。
    
    默认情况下禁用它，因为它会导致某些情况下的减速。
    
    强烈建议在扫描数十万或100万个文件时使用它，因为它可以多次加速搜索。
settings_duplicates_prehash_minimal_entry_tooltip = 缓存条目的最小尺寸。
settings_duplicates_hide_hard_link_button = 隐藏硬链接 (仅限Linux 和 macOS)
settings_duplicates_prehash_checkbutton = 使用捕捉缓存
settings_duplicates_minimal_size_cache_label = 保存到缓存的文件最小大小 (字节)
settings_duplicates_minimal_size_cache_prehash_label = 文件最小尺寸(字节) 保存到逮捕缓存

## Saving/Loading settings

settings_saving_button_tooltip = 保存当前设置配置到文件。
settings_loading_button_tooltip = 从文件加载设置并替换当前配置。
settings_reset_button_tooltip = 重置当前配置为默认设置。
settings_saving_button = 保存配置
settings_loading_button = 加载配置
settings_reset_button = 重置配置

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    打开存储缓存txt文件的文件夹。
    
    修改缓存文件可能会导致显示无效结果。 然而，当将大量文件移动到另一个位置时，修改路径可能会节省时间。
    
    您可以在计算机之间复制这些文件以便在扫描文件时节省时间(当然，如果它们具有类似的目录结构)。
    
    如果缓存出现问题，这些文件可以被删除。应用程序将自动重新生成它们。
settings_folder_settings_open_tooltip =
    打开保存Czkawka配置的文件夹。
    
    警告：手动修改配置可能会破坏您的工作流。
settings_folder_cache_open = 打开缓存文件夹
settings_folder_settings_open = 打开设置文件夹
# Compute results
compute_stopped_by_user = 搜索已被用户停止
compute_found_duplicates_hash_size = 在 { $number_groups } 组中找到 { $number_files } 重复，被带去了 { $size }
compute_found_duplicates_name = 找到 { $number_files } 重复的 { $number_groups } 组
compute_found_empty_folders = 找到 { $number_files } 空文件夹
compute_found_empty_files = 找到 { $number_files } 空文件
compute_found_big_files = 找到 { $number_files } 大文件
compute_found_temporary_files = 找到 { $number_files } 临时文件
compute_found_images = 在 { $number_groups } 组中找到 { $number_files } 相似图像
compute_found_videos = 在 { $number_groups } 组中找到 { $number_files } 相似视频
compute_found_music = 在 { $number_groups } 组中找到 { $number_files } 类似的音乐文件
compute_found_invalid_symlinks = 找到 { $number_files } 无效的符号链接
compute_found_broken_files = 找到 { $number_files } 损坏的文件
compute_found_bad_extensions = 找到 { $number_files } 文件，其扩展名无效
# Progress window
progress_scanning_general_file = 正在扫描 { $file_number } 文件
progress_scanning_extension_of_files = 检查扩展名 { $file_checked }/{ $all_files } 文件
progress_scanning_broken_files = 正在检查 { $file_checked }/{ $all_files } 文件
progress_scanning_video = 散列 { $file_checked }/{ $all_files } 视频
progress_scanning_image = 散列 { $file_checked }/{ $all_files } 图像
progress_comparing_image_hashes = 比较 { $file_checked }/{ $all_files } 图像哈希
progress_scanning_music_tags_end = 对比标签 { $file_checked }/{ $all_files } 音乐文件
progress_scanning_music_tags = 正在读取标签 { $file_checked }/{ $all_files } 音乐文件
progress_scanning_empty_folders = 正在扫描 { $folder_number } 文件夹
progress_scanning_size = 正在扫描文件大小 { $file_number }
progress_scanning_name = 正在扫描 { $file_number } 文件的名称
progress_analyzed_partial_hash = 分析了 { $file_checked }/{ $all_files } 文件的部分哈希
progress_analyzed_full_hash = 分析了 { $file_checked }/{ $all_files } 文件的完整哈希值
progress_current_stage = 当前阶段:{ "  " }
progress_all_stages = 所有阶段:{ " " }
# Saving loading 
saving_loading_saving_success = 配置保存到文件 { $name }。
saving_loading_saving_failure = 无法将配置数据保存到文件 { $name }
saving_loading_reset_configuration = 当前配置已被清除。
saving_loading_loading_success = 正确加载应用程序配置。
saving_loading_invalid_string = 对于密钥"{ $key }" 发现无效的结果 - "{ $result }"不是字符串。
saving_loading_invalid_int = 对于密钥"{ $key }" 发现无效的结果 - "{ $result } 不是整数。
saving_loading_invalid_bool = 对于密钥"{ $key }" 发现无效的结果 - "{ $result } 不是布尔值。
saving_loading_decode_problem_bool = 无法解码来自密钥的 "{ $key }" 找到 "{ $result }" ，但允许的值为 0, 1, true 或 false。
saving_loading_saving_same_keys = 尝试用重复的密钥保存设置 "{ $key }".
saving_loading_failed_to_get_home_directory = 无法获取打开/保存配置文件的主目录。
saving_loading_folder_config_instead_file = 无法在路径 "{ $path } 中创建或打开配置文件，因为已经有一个文件夹。
saving_loading_failed_to_create_configuration_folder = 无法创建配置文件夹 "{ $path }", 原因"{ $reason }".
saving_loading_failed_to_create_config_file = 无法创建配置文件 "{ $path }", 原因"{ $reason }".
saving_loading_failed_to_read_config_file = 无法从 "{ $path }" 加载配置，因为它不存在或不是文件。
saving_loading_failed_to_read_data_from_file = 无法从文件读取数据"{ $path }", 原因"{ $reason }".
saving_loading_orphan_data = 在行中发现了孤儿数据“{ $data }”，{ $line }"。
saving_loading_not_valid = 设置“{ $data }”在当前应用版本中不存在。
# Invalid symlinks
invalid_symlink_infinite_recursion = 无限递归性
invalid_symlink_non_existent_destination = 目标文件不存在
# Other
selected_all_reference_folders = 当所有目录被设置为参考文件夹时，无法开始搜索
searching_for_data = 正在搜索数据，可能需要一段时间，请稍候...
text_view_messages = 消息
text_view_warnings = 警告
text_view_errors = 错误
about_window_motto = 这个程序可以自由使用，并将永远使用。
# Various dialog
dialogs_ask_next_time = 下次询问
delete_file_failed = 删除文件 { $name } 失败，原因 { $reason }
delete_title_dialog = 删除确认
delete_question_label = 您确定要删除文件吗？
delete_all_files_in_group_title = 确认删除组中的所有文件
delete_all_files_in_group_label1 = 在某些组中，所有记录都被选中。
delete_all_files_in_group_label2 = 您确定要删除它们吗？
delete_folder_failed = 无法删除文件夹 { $dir } ，因为文件夹不存在，您没有权限，或者文件夹不是空的。
delete_items_label = { $items } 文件将被删除。
delete_items_groups_label = { $items } 文件来自 { $groups } 组将被删除。
hardlink_failed = 硬链接失败
hard_sym_invalid_selection_title_dialog = 对某些组的选择无效
hard_sym_invalid_selection_label_1 = 在某些组中，只选择了一个记录，它将被忽略。
hard_sym_invalid_selection_label_2 = 要能够链接到这些文件，至少需要选择两个组的结果。
hard_sym_invalid_selection_label_3 = 第一个组被承认为原始组别，没有改变，但是第二个组别后来被修改。
hard_sym_link_title_dialog = 链接确认
hard_sym_link_label = 您确定要链接这些文件吗？
move_folder_failed = 无法移动文件夹 { $name }, 原因 { $reason }
move_file_failed = 移动文件 { $name } 失败，原因 { $reason }
move_files_title_dialog = 选择要移动重复文件的文件夹
move_files_choose_more_than_1_path = 只能选择一个路径来复制重复的文件，选择 { $path_number }。
move_stats = 正确移动 { $num_files }/{ $all_files } 项目
save_results_to_file = 结果保存到文件 { $name }
search_not_choosing_any_music = 错误：您必须选择至少一个带有音乐搜索类型的复选框。
search_not_choosing_any_broken_files = 错误：您必须选择至少一个带有选中文件类型的复选框。
include_folders_dialog_title = 要包含的文件夹
exclude_folders_dialog_title = 要排除的文件夹
include_manually_directories_dialog_title = 手动添加目录
cache_properly_cleared = 已正确清除缓存
cache_clear_duplicates_title = 清除重复缓存
cache_clear_similar_images_title = 清除相似图像缓存
cache_clear_similar_videos_title = 正在清除类似视频缓存
cache_clear_message_label_1 = 您想要清除过时条目的缓存吗？
cache_clear_message_label_2 = 此操作将删除所有指向无效文件的缓存项。
cache_clear_message_label_3 = 这可能会稍微加速加载/保存到缓存。
cache_clear_message_label_4 = 警告：操作将从未接入的外部驱动器中移除所有缓存数据。所以每个散列都需要重新生成。
# Show preview
preview_image_resize_failure = 调整图像大小失败 { $name }
preview_image_opening_failure = 打开镜像 { $name } 失败，原因 { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = 组 { $current_group }/{ $all_groups } ({ $images_in_group } 图像)
compare_move_left_button = L
compare_move_right_button = R
