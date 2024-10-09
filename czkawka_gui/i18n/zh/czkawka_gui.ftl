# Window titles
window_settings_title = 设置
window_main_title = Czkawka
window_progress_title = 正在扫描
window_compare_images = 比较图像
# General
general_ok_button = 确定
general_close_button = 关闭
# Main window
music_title_checkbox = 标题
music_artist_checkbox = 艺人
music_year_checkbox = 年份
music_bitrate_checkbox = 码率
music_genre_checkbox = 流派
music_length_checkbox = 长度
music_comparison_checkbox = 近似比较
music_checking_by_tags = 标签
music_checking_by_content = 内容
same_music_seconds_label = 最小分片秒持续时间
same_music_similarity_label = 最大差异
music_compare_only_in_title_group = 仅在标题中比较
music_compare_only_in_title_group_tooltip =
    如果启用，文件按标题分类，然后相互比较。
    
    如果有一亿文件，而不是几乎1亿文件的比较，通常就会有大约2000万份比较。
same_music_tooltip =
    通过设置以下内容，可以配置按其内容搜索类似的音乐文件：
    
    - 可以识别音乐文件为类似文件的最小碎片时间
    - 两个测试片段之间的最大差异度
    
    找到这些参数的合理组合是获得好结果的关键。
    
    将最小时间设置为5秒，最大差异度设置为1.0，将寻找几乎相同的文件碎片。
    另一方面，20秒的时间和6.0的最大差异度可以很好地找到混音/现场版本等内容。
    
    默认情况下，每个音乐文件都会与其他音乐文件进行比较，当测试许多文件时，这可能需要很长时间，因此通常最好使用参考文件夹并指定要相互比较的文件(如有相同数量的文件，则比较指纹至少比不使用参考文件夹快4倍)。
music_comparison_checkbox_tooltip =
    它使用AI搜索类似的音乐文件，利用机器学习从短语中移除括号。例如，启用此选项后，相关的文件将被视为重复文件：
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = 区分大小写
duplicate_case_sensitive_name_tooltip =
    启用时，仅当记录具有完全相同的名称时才进行分组，例如 Żołd <-> Żołd
    
    禁用此选项将不检查每个字母是否具有相同的大小写，例如 żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = 大小和名称
duplicate_mode_name_combo_box = 名称
duplicate_mode_size_combo_box = 大小
duplicate_mode_hash_combo_box = 哈希
duplicate_hash_type_tooltip =
    Czkawka提供3种哈希类型：
    
    Blake3 - 加密散列函数。这是默认选项，因为它非常快。
    
    CRC32 - 简单的散列函数。这应该比Blake3更快，但极少情况下可能会有一些冲突。
    
    XXH3 - 与Blake3非常相似，性能和哈希质量也很高（但不是加密的）。因此，这些模式可以很容易地互换使用。
duplicate_check_method_tooltip =
    目前，Czkawka提供三种方法来查找重复：
    
    名称 - 查找名称相同的文件。
    
    大小 - 查找大小相同的文件。
    
    哈希 - 查找内容相同的文件。 这种模式会对文件进行哈希计算，然后将哈希值进行比较以查找重复项。这种模式是查找重复项的最安全方法。应用程序大量使用缓存，因此对相同数据进行的第二次及更多次扫描应比第一次更快。
image_hash_size_tooltip =
    每张选中的图像都产生了一个可相互比较的特殊哈希。 两者之间的小差别意味着这些图像是相似的。
    
    8 散列尺寸非常适合于找到仅略类似于原始图像的图像。 有了更大的一组图像 (>1000)，这将产生大量虚假的正数， 所以我建议在这种情况下使用更大的散列尺寸。
    
    16是默认的散列尺寸，它是一个很好的折衷，既使找到了一些小相似的图像，又仅有少量的散列碰撞。
    
    32和64 散列只找到非常相似的图像，但是几乎不应该有假正数 (可能只有一些带着Alpha 通道的图像)。
image_resize_filter_tooltip =
    要计算图像散列，库必须首先调整大小。
    
    在选定的算法上花费, 用来计算散列的结果图像看起来有点不同。
    
    最快使用的算法，也是结果最差的算法，是Nearest。 默认情况下启用它，因为16x16散列大小较低的质量并不真正可见。
    
    使用 8x8 散列大小，建议使用不同于Nearest的算法来拥有更好的图像组。
image_hash_alg_tooltip =
    用户可以从许多计算哈希值的算法中选择一种。
    
    每种算法都有强项和弱项，对于不同的图像，有时结果更好，有时结果更差。
    
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
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = 归档
main_check_box_broken_files_image = 图像
check_button_general_same_size = 忽略相同的大小
check_button_general_same_size_tooltip = 忽略结果中相同大小的文件 - 通常是 1:1 重复
main_label_size_bytes_tooltip = 将用于扫描的文件大小
# Upper window
upper_tree_view_included_folder_column_title = 要搜索的文件夹
upper_tree_view_included_reference_column_title = 参考文件夹
upper_recursive_button = 递归
upper_recursive_button_tooltip = 如果选中，也可以搜索未直接置于选定文件夹下的文件。
upper_manual_add_included_button = 手动添加
upper_add_included_button = 添加
upper_remove_included_button = 删除
upper_manual_add_excluded_button = 手动添加
upper_add_excluded_button = 添加
upper_remove_excluded_button = 删除
upper_manual_add_included_button_tooltip =
    手动添加目录名。
    
    如需一次性添加多个路径，请用分号;分隔它们
    
    填写 /home/roman;/home/krokiet 将添加 /home/roman 和 /home/kookiet 两个目录
upper_add_included_button_tooltip = 添加新目录进行搜索。
upper_remove_included_button_tooltip = 从搜索中删除目录。
upper_manual_add_excluded_button_tooltip =
    手动添加要排除的目录名称。
    
    如需一次性添加多个路径，请用分号;分隔它们
    
    填写 /home/roman;/home/krokiet 将添加 /home/roman 和 /home/kookiet 两个目录
upper_add_excluded_button_tooltip = 添加在搜索中排除的目录。
upper_remove_excluded_button_tooltip = 从排除中删除目录。
upper_notebook_items_configuration = 项目配置
upper_notebook_excluded_directories = 排除的目录
upper_notebook_included_directories = 包含的目录
upper_allowed_extensions_tooltip =
    允许的文件扩展名必须用逗号分隔（默认情况下全部可用）。
    
    还提供以下可同时添加多个扩展名的宏：IMAGE（图片）、VIDEO（视频）、MUSIC（音乐）、TEXT（文本）。
    
    使用示例：".exe, IMAGE, VIDEO, .rar, 7z"，这意味着将扫描图片文件（例如jpg、png）、视频文件（例如avi、mp4）、exe、rar和7z文件。
upper_excluded_extensions_tooltip =
    在扫描中忽略的已禁用文件列表。
    
    在使用允许和禁用的扩展时，这个扩展具有更高的优先级，所以文件将不会被检查。
upper_excluded_items_tooltip =
    被排除的项目必须包含 * 通配符，并用逗号分隔。
    与排除目录相比，这种方式较慢，因此请小心使用。
upper_excluded_items = 排除的项目：
upper_allowed_extensions = 允许的扩展：
upper_excluded_extensions = 已禁用扩展：
# Popovers
popover_select_all = 全部选择
popover_unselect_all = 取消全选
popover_reverse = 反向选择
popover_select_all_except_oldest = 选择除最旧以外的所有选项
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
    /home/pimpek/rzecz.txt 可以通过 /home/pim* 找到
popover_custom_name_check_button_entry_tooltip =
    按文件名选择记录。
    
    示例用法：
    /usr/ping/pong.txt 可以通过 *ong* 找到。
popover_custom_regex_check_button_entry_tooltip =
    按指定的正则表达式选择记录。
    
    使用此模式，搜索的文本是带有名称的路径。
    
    示例用法：
    可以使用 /ziem[a-z]+ 查找 /usr/bin/ziemniak.txt
    
    这使用默认的Rust正则表达式实现。 您可以在此处阅读有关它的更多信息：https://docs.rs/regex。
popover_custom_case_sensitive_check_button_tooltip =
    启用大小写检测。
    
    该选项禁用时，/home/* 将同时找到 /HoMe/roman 和 /home/roman。
popover_custom_not_all_check_button_tooltip =
    禁止在分组中选择所有记录。
    
    这是默认启用的，因为在大多数情况下， 您不想删除原始文件和重复文件，而是想留下至少一个文件。
    
    警告：如果您已经手动选择了一个组中的所有结果，则此设置不起作用。
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
bottom_symlink_button = 软链接
bottom_hardlink_button = 硬链接
bottom_move_button = 移动
bottom_sort_button = 排序
bottom_search_button_tooltip = 开始搜索
bottom_select_button_tooltip = 选择记录。只能稍后处理选定的文件/文件夹。
bottom_delete_button_tooltip = 删除选中的文件/文件夹。
bottom_save_button_tooltip = 保存搜索数据到文件
bottom_symlink_button_tooltip =
    创建软链接。
    只有在至少选择了一组中的两个结果时才起作用。
    第一个结果保持不变，第二个及后续结果都会被软链接到第一个结果上。
bottom_hardlink_button_tooltip =
    创建硬链接。
    只有在至少选择了一组中的两个结果时才起作用。
    第一个结果保持不变，第二个及后续结果都会被硬链接到第一个结果上。
bottom_hardlink_button_not_available_tooltip =
    创建硬链接。
    按钮已禁用，因为无法创建硬链接。
    在 Windows 上，只有使用管理员权限才能使用硬链接，所以请确保以管理员身份运行该应用程序。
    如果应用程序已经具有管理员权限，请在 Github 上查找类似的问题。
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
settings_number_of_threads_tooltip = 使用的线程数，0表示所有可用线程都将被使用。
settings_use_rust_preview = 使用外部库来加载预览
settings_use_rust_preview_tooltip =
    使用 gtk 预览有时会更快，支持更多格式，但有时恰恰相反。
    
    如果您在加载预览时遇到问题，您可以尝试更改此设置。
    
    关于非Linux系统，建议使用此选项。 因为gtk-pixbuf 并不总是可用的，所以禁用此选项不会加载某些图像的预览。
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
settings_use_trash_button_tooltip = 将文件移至回收站，而不是将其永久删除。
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
    删除指向不存在文件的过期缓存结果。
    
    当启用时，应用程序确保在加载记录时所有记录都指向有效文件（无法访问的文件将被忽略）。
    
    禁用此功能将有助于扫描外部驱动器上的文件时，避免在下一次扫描时清除与其相关的缓存条目。
    
    如果缓存中有数十万条记录，则建议启用此功能，这将加快扫描开始/结束时的缓存加载/保存速度。
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
settings_multiple_clear_cache_button = 从缓存中删除过时的结果。

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    隐藏除一个以外的所有文件，如果所有文件都指向同一数据（即为硬链接）。
    
    例如，在磁盘上有七个文件都硬链接到特定数据，并且还有一个不同的文件，其数据相同，但索引节点不同，那么在重复文件查找器中，只会显示一个唯一的文件和一个硬链接文件。
settings_duplicates_minimal_size_entry_tooltip =
    设置将被缓存的最小文件大小。
    
    选择较小的值将会生成更多的记录。这将加快搜索速度，但会减慢缓存的加载/保存速度。
settings_duplicates_prehash_checkbutton_tooltip =
    启用预散列缓存（从文件的一小部分计算出的哈希），以便更早地排除非重复结果。
    
    默认情况下禁用它，因为在某些情况下可能会导致减慢速度。
    
    强烈建议在扫描数十万或100万个文件时使用它，因为它可以将搜索加速多倍。
settings_duplicates_prehash_minimal_entry_tooltip = 缓存条目的最小尺寸。
settings_duplicates_hide_hard_link_button = 隐藏硬链接 (仅限Linux 和 macOS)
settings_duplicates_prehash_checkbutton = 使用捕捉缓存
settings_duplicates_minimal_size_cache_label = 保存到缓存的最小文件大小 (字节)
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
    打开存储缓存的txt文件的文件夹。
    
    修改缓存文件可能会导致显示无效的结果。然而，当将大量文件移动到另一个位置时，修改路径可能会节省时间。
    
    您可以在计算机之间复制这些文件，以节省再次扫描文件的时间（当然，如果它们具有相似的目录结构）。
    
    如果出现缓存问题，可以删除这些文件。该应用程序将自动重新生成它们。
settings_folder_settings_open_tooltip =
    打开保存Czkawka配置的文件夹。
    
    警告：手动修改配置可能会破坏您的工作流程。
settings_folder_cache_open = 打开缓存文件夹
settings_folder_settings_open = 打开设置文件夹
# Compute results
compute_stopped_by_user = 搜索已被用户停止
compute_found_duplicates_hash_size = 在 { $number_groups } 个组中发现 { $number_files } 个重复项，占用了 { $size } 的空间
compute_found_duplicates_name = 在 { $number_groups } 个组中发现 { $number_files } 个重复项
compute_found_empty_folders = 找到 { $number_files } 个空文件夹
compute_found_empty_files = 找到 { $number_files } 个空文件
compute_found_big_files = 找到 { $number_files } 个大文件
compute_found_temporary_files = 找到 { $number_files } 个临时文件
compute_found_images = 在 { $number_groups } 个组中找到 { $number_files } 个相似图像
compute_found_videos = 在 { $number_groups } 个组中找到 { $number_files } 个相似视频
compute_found_music = 在 { $number_groups } 个组中找到 { $number_files } 个类似的音乐文件
compute_found_invalid_symlinks = 找到 { $number_files } 个无效的符号链接
compute_found_broken_files = 找到 { $number_files } 个损坏的文件
compute_found_bad_extensions = 找到 { $number_files } 个文件，其扩展名无效
# Progress window
progress_scanning_general_file = 正在扫描 { $file_number } 个文件
progress_scanning_extension_of_files = 正在检查 { $file_checked }/{ $all_files } 个文件的扩展名
progress_scanning_broken_files = 正在检查 { $file_checked }/{ $all_files } 个文件
progress_scanning_video = 散列 { $file_checked }/{ $all_files } 视频
progress_scanning_image = 散列 { $file_checked }/{ $all_files } 图像
progress_comparing_image_hashes = 正在比较 { $file_checked }/{ $all_files } 个图像的哈希
progress_scanning_music_tags_end = 正在对比 { $file_checked }/{ $all_files } 个音乐文件标签
progress_scanning_music_tags = 正在读取 { $file_checked }/{ $all_files } 个音乐文件标签
progress_scanning_music_content_end = 正在比较 { $file_checked }/{ $all_files } 个音乐文件指纹
progress_scanning_music_content = 正在计算 { $file_checked }/{ $all_files } 个音乐文件指纹
progress_scanning_empty_folders = 正在扫描 { $folder_number } 个文件夹
progress_scanning_size = 正在扫描 { $file_number } 个文件的大小
progress_scanning_size_name = 正在扫描 { $file_number } 个文件的名称和大小
progress_scanning_name = 正在扫描 { $file_number } 个文件的名称
progress_analyzed_partial_hash = 分析了 { $file_checked }/{ $all_files } 个文件的部分哈希
progress_analyzed_full_hash = 分析了 { $file_checked }/{ $all_files } 个文件的完整哈希值
progress_prehash_cache_loading = 正在加载逮捕缓存
progress_prehash_cache_saving = 正在保存抓取缓存
progress_hash_cache_loading = 加载散列缓存
progress_hash_cache_saving = 保存哈希缓存
progress_cache_loading = 加载缓存
progress_cache_saving = 正在保存缓存
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
saving_loading_orphan_data = 在行 “{ $line }” 中发现了孤立数据 “{ $data }”。
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
about_window_motto = 本程序永久免费。
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
delete_items_groups_label = 来自 { $groups } 个组中的 { $items } 个文件将被删除。
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
move_stats = 正确移动 { $num_files }/{ $all_files } 个项目
save_results_to_file = 将结果保存到 txt 和 json 文件到 { $name } 文件夹。
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
preview_image_resize_failure = 调整图像 { $name } 的大小失败
preview_image_opening_failure = 打开镜像 { $name } 失败，原因 { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = 组 { $current_group }/{ $all_groups } ({ $images_in_group } 图像)
compare_move_left_button = L
compare_move_right_button = R
