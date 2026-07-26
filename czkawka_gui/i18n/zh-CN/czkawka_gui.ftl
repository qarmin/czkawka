# Window titles
window_settings_title = 设定
window_main_title = Czkawka (打嗝)
window_progress_title = 正在扫描
window_compare_images = 比较图像
# General
general_ok_button = 确定
general_close_button = 关闭
# Krokiet info dialog
krokiet_info_title = 弃用通知
krokiet_info_message =
    Czkawka GTK 12.0 是最后一个版本，不再计划任何更新、新功能或错误修复。
    
    Czkawka GTK 的大多数功能在 Krokiet 中都可用，而且通常更简单、更快、更稳定。Krokiet 还新增了 GTK 版本无法实现的新功能和改进。
    
    如果您仍在使用 Czkawka GTK，切换到 Krokiet 应该很容易，因为它界面相似、依赖更少、跨平台支持更好。
    
    PS：此消息应该只显示一次。如果它再次出现，请将 CZKAWKA_DONT_ANNOY_ME 环境变量设置为任意非空值。
# Main window
music_title_checkbox = 标题
music_artist_checkbox = 艺术家
music_year_checkbox = 年份
music_bitrate_checkbox = 比特率
music_genre_checkbox = 流派
music_length_checkbox = 长度
music_comparison_checkbox = 近似比较
music_checking_by_tags = 标签
music_checking_by_content = 内容
same_music_seconds_label = 最小片段时长（秒）
same_music_similarity_label = 最大差异
music_compare_only_in_title_group = 在相似标题的组中比较
music_compare_only_in_title_group_tooltip =
    启用后，文件会先按标题分组，然后组内相互比较。
    
    以 10000 个文件为例，通常只需约 20000 次比较，而不是近 1 亿次。
same_music_tooltip =
    可以通过以下设置来配置按内容搜索相似音乐文件：
    
    - 可将音乐文件判定为相似的最小片段时长
    - 两个被测片段之间的最大差异
    
    获得好结果的关键是找到这些参数的合理组合。
    
    将最小时长设为 5 秒、最大差异设为 1.0，将查找文件中几乎完全相同的片段。
    而 20 秒的时长和 6.0 的最大差异则很适合查找混音/现场版本等。
    
    默认情况下，每个音乐文件都会与其他所有文件两两比较，文件很多时可能非常耗时，因此通常最好使用参考文件夹并指定哪些文件相互比较（文件数量相同时，比较指纹至少比不使用参考文件夹快 4 倍）。
music_comparison_checkbox_tooltip =
    它使用 AI 搜索相似的音乐文件，利用机器学习从短语中移除括号。例如，启用此选项后，下列文件将被视为重复文件：
    
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
    Czkawka 提供 3 种哈希类型：
    
    Blake3 - 加密哈希函数。它是默认选项，因为它非常快。
    
    CRC32 - 简单的哈希函数。它应该比 Blake3 更快，但极少情况下可能发生碰撞。
    
    XXH3 - 性能和哈希质量与 Blake3 非常相似（但不是加密哈希），因此这些模式可以轻松互换使用。
duplicate_check_method_tooltip =
    目前，Czkawka 提供三种查找重复项的方法：
    
    名称 - 查找名称相同的文件。
    
    大小 - 查找大小相同的文件。
    
    哈希 - 查找内容相同的文件。此模式会对文件进行哈希计算，之后比较哈希值以查找重复项。此模式是查找重复项最安全的方式。应用程序大量使用缓存，因此对相同数据的第二次及以后的扫描会比第一次快得多。
image_hash_size_tooltip =
    每张检查过的图像都会生成一个可相互比较的特殊哈希，哈希间差异小意味着图像相似。
    
    哈希大小为 8 时，很适合查找与原图只有轻微相似的图像。图像较多（>1000）时会产生大量误报，此时建议使用更大的哈希。
    
    16 是默认哈希大小，在“能找到轻微相似的图像”与“哈希碰撞较少”之间取得了很好的平衡。
    
    32 和 64 的哈希只能找到非常相似的图像，但几乎没有误报（带 Alpha 通道的部分图像可能除外）。
image_resize_filter_tooltip =
    要计算图像哈希，库必须先调整图像大小。
    
    根据所选算法，用于计算哈希的图像会略有不同。
    
    速度最快但效果最差的算法是 Nearest。它默认启用，因为在 16x16 哈希大小下，质量降低并不明显。
    
    使用 8x8 哈希大小时，建议改用 Nearest 以外的算法，以获得更好的图像分组。
image_hash_alg_tooltip =
    用户可以从许多计算哈希值的算法中选择一种。
    
    每种算法都有强项和弱项，对于不同的图像，有时结果更好，有时结果更差。
    
    因此，为了确定最适合你的算法，需要进行人工测试。
image_geometric_invariance_tooltip = 同时比较每张图像的镜像/翻转以及可选的旋转变体。这会提升匹配效果，但会增加哈希计算时间。
big_files_mode_combobox_tooltip = 允许搜索最小/最大的文件
big_files_mode_label = 要检查的文件
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
main_notebook_bad_extensions = 错误的扩展名
main_tree_view_column_file_name = 文件名称
main_tree_view_column_folder_name = 文件夹名称
main_tree_view_column_path = 路径
main_tree_view_column_modification = 修改日期
main_tree_view_column_size = 大小
main_tree_view_column_similarity = 相似度
main_tree_view_column_dimensions = 尺寸
main_tree_view_column_title = 标题
main_tree_view_column_artist = 艺术家
main_tree_view_column_year = 年份
main_tree_view_column_bitrate = 比特率
main_tree_view_column_length = 长度
main_tree_view_column_genre = 流派
main_tree_view_column_symlink_file_name = 符号链接文件名
main_tree_view_column_symlink_folder = 符号链接文件夹
main_tree_view_column_destination_path = 目标路径
main_tree_view_column_type_of_error = 错误类型
main_tree_view_column_current_extension = 当前扩展名
main_tree_view_column_proper_extensions = 正确的扩展名
main_tree_view_column_fps = FPS
main_tree_view_column_codec = 编解码器
main_label_check_method = 检查方法
main_label_hash_type = 哈希类型
main_label_hash_size = 哈希大小
main_label_geometric_invariance = 几何不变性
main_label_size_bytes = 大小 (字节)
main_label_min_size = 最小值
main_label_max_size = 最大值
main_label_shown_files = 显示的文件数
main_label_resize_algorithm = 调整算法
main_label_similarity = 相似度{ "   " }
main_check_box_broken_files_audio = 音频
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = 压缩包
main_check_box_broken_files_image = 图像
main_check_box_broken_files_video = 视频
main_check_box_broken_files_video_tooltip = 使用 ffmpeg/ffprobe 验证视频文件。速度相当慢，即使文件能正常播放，也可能报告过于苛刻的错误。
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
    
    如需一次添加多个路径，请用 ; 分隔
    
    /home/roman;/home/rozkaz 将添加 /home/roman 和 /home/rozkaz 两个目录
upper_add_included_button_tooltip = 添加新目录进行搜索。
upper_remove_included_button_tooltip = 从搜索中删除目录。
upper_manual_add_excluded_button_tooltip =
    手动添加要排除的目录名。
    
    如需一次添加多个路径，请用 ; 分隔
    
    /home/roman;/home/krokiet 将添加 /home/roman 和 /home/krokiet 两个目录
upper_add_excluded_button_tooltip = 添加在搜索中排除的目录。
upper_remove_excluded_button_tooltip = 从排除中删除目录。
upper_notebook_items_configuration = 项目配置
upper_notebook_excluded_directories = 排除路径
upper_notebook_included_directories = 包含路径
upper_allowed_extensions_tooltip =
    允许的扩展名必须用逗号分隔（默认情况下所有扩展名都可用）。
    
    还可以使用以下可一次添加多个扩展名的宏：IMAGE、VIDEO、MUSIC、TEXT。
    
    用法示例“.exe, IMAGE, VIDEO, .rar, 7z”-这意味着将扫描图像（如 jpg、png）、视频（如 avi、mp4）、exe、rar 和 7z 文件。
upper_excluded_extensions_tooltip =
    扫描时将忽略的禁用扩展名列表。
    
    同时使用允许与禁用扩展名时，禁用扩展名的优先级更高，相应文件将不会被检查。
upper_excluded_items_tooltip =
    排除项目必须包含 * 并且应以逗号分隔。
    这比排除路径慢，因此请谨慎使用。
upper_excluded_items = 排除的项目：
upper_allowed_extensions = 允许的扩展名：
upper_excluded_extensions = 禁用的扩展名：
# Popovers
popover_select_all = 全部选择
popover_unselect_all = 取消全选
popover_reverse = 反向选择
popover_select_all_except_shortest_path = 选择除最短路径外的所有项
popover_select_all_except_longest_path = 选择除最长路径外的所有项
popover_select_all_except_oldest = 选择除最旧外的所有项
popover_select_all_except_newest = 选择除最新外的所有项
popover_select_one_oldest = 选择一个最旧的
popover_select_one_newest = 选择一个最新的
popover_select_custom = 选择自定义
popover_unselect_custom = 取消选择自定义
popover_select_all_images_except_biggest = 选择除最大外的所有项
popover_select_all_images_except_smallest = 选择除最小外的所有项
popover_custom_path_check_button_entry_tooltip =
    通过路径选择记录。
    
    示例用法：
    /home/pimpek/rzecz.txt 可以通过 /home/pim* 找到
popover_custom_name_check_button_entry_tooltip =
    按文件名选择记录。
    
    示例用法：
    /usr/ping/pong.txt 可以通过 *ong* 找到
popover_custom_regex_check_button_entry_tooltip =
    按指定的正则表达式选择记录。
    
    使用此模式，搜索的文本是带有名称的路径。
    
    示例用法：
    可以使用 /ziem[a-z]+ 查找 /usr/bin/ziemniak.txt
    
    这使用默认的Rust正则表达式实现。 您可以在此处阅读有关它的更多信息: https://docs.rs/regex。
popover_custom_case_sensitive_check_button_tooltip =
    启用区分大小写的检测。
    
    禁用时，/home/* 会同时找到 /HoMe/roman 和 /home/roman。
popover_custom_not_all_check_button_tooltip =
    禁止在分组中选择所有记录。
    
    这是默认启用的，因为在大多数情况下， 您不想删除原始文件和重复文件，而是想留下至少一个文件。
    
    警告：如果您已经手动选择了一个组中的所有结果，则此设置不起作用。
popover_custom_regex_path_label = 路径
popover_custom_regex_name_label = 名称
popover_custom_regex_regex_label = 正则表达式路径 + 名称
popover_custom_case_sensitive_check_button = 区分大小写
popover_custom_all_in_group_label = 不选中组内的所有记录
popover_custom_mode_unselect = 取消选择自定义
popover_custom_mode_select = 选择自定义
popover_sort_file_name = 文件名称
popover_sort_folder_name = 文件夹名称
popover_sort_full_name = 完整名称
popover_sort_size = 大小
popover_sort_selection = 选择
popover_invalid_regex = 正则表达式无效
popover_valid_regex = 正则表达式有效
# Bottom buttons
bottom_search_button = 搜索
bottom_select_button = 选择
bottom_delete_button = 删除
bottom_save_button = 保存
bottom_symlink_button = 符号链接
bottom_hardlink_button = 硬链接
bottom_move_button = 移动
bottom_sort_button = 排序
bottom_compare_button = 比较
bottom_search_button_tooltip = 开始搜索
bottom_select_button_tooltip = 选择记录。只有选中的文件/文件夹才能进行后续处理。
bottom_delete_button_tooltip = 删除选中的文件/文件夹。
bottom_save_button_tooltip = 保存搜索数据到文件
bottom_symlink_button_tooltip =
    创建符号链接。
    仅当组内至少选中两个结果时才有效。
    第一个结果保持不变，第二个及之后的结果将符号链接到第一个。
bottom_hardlink_button_tooltip =
    创建硬链接。
    仅当组内至少选中两个结果时才有效。
    第一个结果保持不变，第二个及之后的结果将硬链接到第一个。
bottom_hardlink_button_not_available_tooltip =
    创建硬链接。
    按钮已禁用，因为无法创建硬链接。
    在 Windows 上，只有使用管理员权限才能使用硬链接，所以请确保以管理员身份运行该应用程序。
    如果应用程序已经具有管理员权限，请在 Github 上查找类似的问题。
bottom_move_button_tooltip =
    将文件移动到选定的目录。
    它会把所有文件复制到该目录，不保留目录树结构。
    当尝试将两个同名文件移动到同一文件夹时，第二个会失败并显示错误。
bottom_sort_button_tooltip = 根据选定的方法排序文件/文件夹。
bottom_compare_button_tooltip = 比较群组中的图像。
bottom_show_errors_tooltip = 显示/隐藏底部文本面板。
bottom_show_upper_notebook_tooltip = 显示/隐藏上方面板。
# Progress Window
progress_stop_button = 停止
progress_stop_additional_message = 已请求停止
# About Window
about_repository_button_tooltip = 链接到源代码的仓库页面。
about_donation_button_tooltip = 链接到捐赠页面。
about_instruction_button_tooltip = 使用说明页面的链接。
about_translation_button_tooltip = 链接到带有应用程序翻译的 Crowdin 页面。官方支持波兰语和英语。
about_repository_button = 存储库
about_donation_button = 捐助
about_instruction_button = 说明
about_translation_button = 翻译
about_other_apps_button = 其他应用程序
about_other_apps_dialog_title = qarmin 的其他应用
about_other_apps_open_source_note = 列出的所有应用程序都是免费和开源的。
about_other_apps_open_button = 打开
about_other_apps_szyszka_desc = 快速且强大的文件重命名工具。
about_other_apps_mykrut_desc = 简单、快速、有主见的 Linux 文件管理器。
about_other_apps_dcmki_viewer_desc = 简单的 DICOM 查看器。
about_other_apps_video_thumbnailer_desc = Czkawka 所用视频缩略图生成器的封装工具。
about_other_apps_space_finder_desc = 用于查找系统中最大文件的简单工具。
about_other_apps_system_info_collector_desc = 从操作系统收集 RAM/CPU 使用情况并以图表显示。
# Header
header_setting_button_tooltip = 打开设置对话框。
header_about_button_tooltip = 打开包含应用程序信息的对话框。
header_krokiet_button_tooltip = 试试 Krokiet--全新改进版！
# Krokiet promo dialog
krokiet_promo_title = 认识一下 Krokiet！
krokiet_promo_message =
    你好，勇敢的 Czkawka 用户！
    
    原力显然与你同在，但 Krokiet 还没有--它是一款更新、更快、更轻量、也明显更英俊（假设应用程序真能称得上英俊）的重复文件清理工具。
    
    Krokiet 保留了 Czkawka 深受喜爱的一切。它完全免费、开源，拥有独特而简洁的 UI（有人盛赞也有人痛恨），引入了大量新功能，依赖更少，并且在不同平台上的运行可靠得多。
    
    另外，如果你还没注意到，现在已经有比 Krokiet 更新的应用--Cedinia，主要为安卓设备和触摸屏设计。
    
    Czkawka GTK 曾很好地为我们服务，但它的守望已经结束。
krokiet_promo_link_download = 下载 Krokiet/Cedinia
krokiet_promo_link_project = 项目页面

# Settings


## General

settings_number_of_threads = 使用的线程数
settings_number_of_threads_tooltip = 使用的线程数，0表示所有可用线程都将被使用。
settings_use_rust_preview = 使用外部库（而非 GTK）加载预览
settings_use_rust_preview_tooltip =
    使用 GTK 预览有时更快、支持的格式更多，但有时恰恰相反。
    
    如果加载预览时遇到问题，可以尝试更改此设置。
    
    在非 Linux 系统上建议启用此选项，因为 gtk-pixbuf 并不总是可用，禁用此选项将无法加载某些图像的预览。
settings_label_restart = 您需要重新启动应用才能应用设置！
settings_ignore_other_filesystems = 忽略其它文件系统 (仅限Linux)
settings_ignore_other_filesystems_tooltip =
    忽略与所搜索目录不在同一文件系统中的文件。
    
    作用与 Linux 上 find 命令的 -xdev 选项相同
settings_save_at_exit_button_tooltip = 关闭应用时将配置保存到文件。
settings_load_at_start_button_tooltip =
    打开应用程序时从文件加载配置。
    
    如果未启用，将使用默认设置。
settings_confirm_deletion_button_tooltip = 点击删除按钮时显示确认对话框。
settings_confirm_link_button_tooltip = 点击硬链接/符号链接按钮时显示确认对话框。
settings_confirm_group_deletion_button_tooltip = 尝试从群组中删除所有记录时显示警告对话框。
settings_show_text_view_button_tooltip = 在用户界面底部显示文本面板。
settings_use_cache_button_tooltip = 使用文件缓存。
settings_save_also_as_json_button_tooltip = 将缓存保存为（人类可读的）JSON 格式。可以修改其内容。如果二进制格式缓存（扩展名为 bin）缺失，应用程序会自动读取此文件中的缓存。
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
    
    当启用时，应用程序确保在加载记录时所有记录都指向有效文件 (无法访问的文件将被忽略)。
    
    禁用此功能将有助于扫描外部驱动器上的文件时，避免在下一次扫描时清除与其相关的缓存条目。
    
    如果缓存中有数十万条记录，则建议启用此功能。这将加快扫描开始/结束时的缓存加载/保存速度。
settings_notebook_general = 常规
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
    
    示例：如果（磁盘上）有七个文件硬链接到特定数据，而一个不同文件具有相同数据但不同 inode，则在重复查找器中，将仅显示一个唯一文件和一个来自硬链接文件的文件。
settings_duplicates_minimal_size_entry_tooltip =
    设置将被缓存的最小文件大小。
    
    选择较小的值将会生成更多的记录。这将加快搜索速度，但会减慢缓存的加载/保存速度。
settings_duplicates_prehash_checkbutton_tooltip =
    启用预哈希（从文件的一小部分计算出的哈希）缓存，以便更早排除非重复结果。
    
    默认禁用，因为在某些情况下可能导致变慢。
    
    强烈建议在扫描数十万或上百万个文件时使用，因为它可将搜索速度提高数倍。
settings_duplicates_prehash_minimal_entry_tooltip = 缓存条目的最小尺寸。
settings_duplicates_hide_hard_link_button = 隐藏硬链接
settings_duplicates_prehash_checkbutton = 使用预哈希缓存
settings_duplicates_minimal_size_cache_label = 保存到缓存的最小文件大小 (字节)
settings_duplicates_minimal_size_cache_prehash_label = 保存到预哈希缓存的最小文件大小（字节）

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
    
    您可以在计算机之间复制这些文件，以节省再次扫描文件的时间 (当然，如果它们具有相似的目录结构)。
    
    如果出现缓存问题，可以删除这些文件。该应用程序将自动重新生成它们。
settings_folder_settings_open_tooltip =
    打开保存Czkawka配置的文件夹。
    
    警告：手动修改配置可能会破坏您的工作流程。
settings_folder_cache_open = 打开缓存文件夹
settings_folder_settings_open = 打开设置文件夹
# Compute results
compute_stopped_by_user = 搜索已被用户停止
compute_found_duplicates_hash_size = 在 { $time } 内找到 { $number_groups } 组共 { $number_files } 个重复文件，占用 { $size }
compute_found_duplicates_name = 在 { $time } 内找到 { $number_groups } 组共 { $number_files } 个重复文件
compute_found_empty_folders = 在 { $time } 内找到 { $number_files } 个空文件夹
compute_found_empty_files = 在 { $time } 内找到 { $number_files } 个空文件
compute_found_big_files = 在 { $time } 内找到 { $number_files } 个大文件
compute_found_temporary_files = 在 { $time } 内找到 { $number_files } 个临时文件
compute_found_images = 在 { $time } 内找到 { $number_groups } 组共 { $number_files } 个相似图像
compute_found_videos = 在 { $time } 内找到 { $number_groups } 组共 { $number_files } 个相似视频
compute_found_music = 在 { $time } 内找到 { $number_groups } 组共 { $number_files } 个相似音乐文件
compute_found_invalid_symlinks = 在 { $time } 内找到 { $number_files } 个无效的符号链接
compute_found_broken_files = 在 { $time } 内找到 { $number_files } 个损坏的文件
compute_found_bad_extensions = 在 { $time } 内找到 { $number_files } 个扩展名无效的文件
# Progress window
progress_current_stage = 当前阶段：{ "  " }
progress_all_stages = 所有阶段：{ "  " }
# Saving loading 
saving_loading_saving_success = 已将配置保存到文件 { $name }。
saving_loading_saving_failure = 无法将配置数据保存到文件 { $name }，原因 { $reason }。
saving_loading_reset_configuration = 当前配置已被清除。
saving_loading_loading_success = 已成功加载应用程序配置。
saving_loading_no_config_file = 未找到配置文件，使用默认设置。
saving_loading_failed_to_create_config_file = 无法创建配置文件 "{ $path }"，原因 "{ $reason }"。
saving_loading_failed_to_read_config_file = 无法从 "{ $path }" 加载配置，因为它不存在或不是文件。
saving_loading_failed_to_read_data_from_file = 无法从文件 "{ $path }" 读取数据，原因 "{ $reason }"。
# Other
selected_all_reference_folders = 当所有目录被设置为参考文件夹时，无法开始搜索
searching_for_data = 正在搜索数据，可能需要一段时间，请稍候...
text_view_messages = 消息
text_view_warnings = 警告
text_view_errors = 错误
about_window_motto = 本程序永久免费。
krokiet_new_app = 自版本 12 起，Czkawka 的 GTK 版本已不再开发。如需新功能和活跃开发，请使用更稳定、性能更好的 Krokiet。
# Various dialog
dialogs_ask_next_time = 下次询问
symlink_failed = 无法将 { $name } 符号链接到 { $target }，原因 { $reason }
delete_title_dialog = 删除确认
delete_question_label = 您确定要删除文件吗？
delete_all_files_in_group_title = 确认删除组中的所有文件
delete_all_files_in_group_label1 = 在某些组中，所有记录都被选中。
delete_all_files_in_group_label2 = 您确定要删除它们吗？
delete_items_label = { $items } 文件将被删除。
delete_items_groups_label = 来自 { $groups } 个组中的 { $items } 个文件将被删除。
hardlink_failed = 无法将 { $name } 硬链接到 { $target }，原因 { $reason }
hard_sym_invalid_selection_title_dialog = 部分组的选择无效
hard_sym_invalid_selection_label_1 = 在某些组中，只选择了一个记录，它将被忽略。
hard_sym_invalid_selection_label_2 = 要对这些文件创建硬链接/符号链接，组内至少需要选中两个结果。
hard_sym_invalid_selection_label_3 = 组中第一个文件被视为原始文件，不会被更改；第二个及之后的文件会被修改。
hard_sym_link_title_dialog = 链接确认
hard_sym_link_label = 您确定要链接这些文件吗？
move_folder_failed = 无法移动文件夹 { $name }，原因 { $reason }
move_file_failed = 移动文件 { $name } 失败，原因 { $reason }
move_files_title_dialog = 选择要移动重复文件的文件夹
move_files_choose_more_than_1_path = 只能选择一个路径来复制重复文件，当前选择了 { $path_number } 个。
move_stats = 已成功移动 { $num_files }/{ $all_files } 个项目
save_results_to_file = 已将结果以 txt 和 json 文件保存到 "{ $name }" 文件夹。
search_not_choosing_any_music = 错误：您必须至少选择一个音乐搜索类型的复选框。
search_not_choosing_any_broken_files = 错误：您必须至少选择一种要检查的损坏文件类型。
include_folders_dialog_title = 要包含的文件夹
exclude_folders_dialog_title = 要排除的文件夹
include_manually_directories_dialog_title = 手动添加目录
cache_properly_cleared = 已成功清除缓存
cache_clear_duplicates_title = 清除重复缓存
cache_clear_similar_images_title = 清除相似图像缓存
cache_clear_similar_videos_title = 清除相似视频缓存
cache_clear_message_label_1 = 您想要清除过时条目的缓存吗？
cache_clear_message_label_2 = 此操作将删除所有指向无效文件的缓存项。
cache_clear_message_label_3 = 这可能会稍微加速加载/保存到缓存。
cache_clear_message_label_4 = 警告：操作将从未接入的外部驱动器中移除所有缓存数据。所以每个散列都需要重新生成。
# Show preview
preview_image_resize_failure = 调整图像 { $name } 的大小失败。
preview_image_opening_failure = 打开图像 { $name } 失败，原因 { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = 组 { $current_group }/{ $all_groups }（{ $images_in_group } 张图像）
compare_move_left_button = L
compare_move_right_button = R
