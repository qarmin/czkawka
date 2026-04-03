# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = 重复项
tool_empty_folders = 空文件夹
tool_similar_images = 相似图像
tool_empty_files = 空文件
tool_temporary_files = 临时文件
tool_big_files = 最大的文件
tool_broken_files = 损坏的文件
tool_bad_extensions = 错误的扩展
tool_same_music = 重复音乐
tool_bad_names = 名称错误
tool_exif_remover = EXIF 数据
tool_directories = 目录
tool_settings = 设置
# Home screen tool card descriptions
home_dup_description = 查找具有相同内容的文件
home_empty_folders_description = 没有内容的目录
home_similar_images_description = 寻找相似的照片
home_empty_files_description = 零大小的文件
home_temp_files_description = 临时文件和缓存文件
home_big_files_description = 磁盘上最大/最大文件
home_broken_files_description = PDF、音频、图像、存档
home_bad_extensions_description = 无效扩展名的文件
home_same_music_description = 按标签显示类似的音频文件
home_bad_names_description = 有问题字符名称的文件
home_exif_description = 使用 EXIF 元数据的图像
# Results list
scanning = 正在扫描...
stopping = 正在停止...
no_results = 没有结果
press_start = 点击开始扫描
select_label = 出售。.
deselect_label = Desel.
list_label = 列表
gallery_label = 加拉太书。.
# Selection popup
selection_popup_title = 选择
select_all = 选择所有
select_except_one = 选择除一个
select_except_largest = 选择除最大以外的所有
select_except_smallest = 选择除最小以外的所有
select_largest = 选择最大的
select_smallest = 选择最小值
select_except_highest_res = 选择除最高分辨率以外的所有选项
select_except_lowest_res = 选择除最低分辨率以外的所有选项
select_highest_res = 选择最高分辨率
select_lowest_res = 选择最低分辨率
invert_selection = 反转选择
close = 关闭
# Deselection popup
deselection_popup_title = 取消选择
deselect_all = 取消全选
deselect_except_one = 取消选择除一个
# Confirm popup
cancel = 取消
delete = 删除
rename = 重命名：
# Delete errors popup
delete_errors_title = 删除文件失败：
ok = 好的
# Stopping overlay
stopping_overlay_title = 正在停止
stopping_overlay_body = 
    正在完成当前扫描...
    请稍候。.
# Permission popup
permission_title = 文件访问权限
permission_body = 要扫描文件，应用程序需要访问设备存储。没有此权限，扫描将无法进行。.
grant = 授权
no_permission_scan_warning = 没有文件访问权限 - 允许扫描
# Settings screen tabs
settings_tab_general = A. 概况
settings_tab_tools = 工具
settings_tab_diagnostics = 信息
# Settings - General tab
settings_use_cache = 使用缓存
settings_use_cache_desc = 加速随后的扫描(哈希/图像)
settings_ignore_hidden = 忽略隐藏文件
settings_ignore_hidden_desc = 以“.”开头的文件和文件夹
settings_show_notification = 扫描结束时通知
settings_show_notification_desc = 扫描完成时显示系统通知
settings_notify_only_background = 仅在后台显示
settings_notify_only_background_desc = 如果应用程序可见则跳过通知
notifications_disabled_banner = 通知已禁用
notifications_enable_button = 启用
settings_scan_label = 已扫描的
settings_filters_label = FILTERS (一些工具)
settings_min_file_size = 最小文件大小
settings_max_file_size = 最大文件大小
settings_language = 语言
settings_language_restart = 需要重启应用程序
settings_common_label = COMMON设置
settings_excluded_items = EXCLUDED ITEMS (glob 模式, 逗号分隔)
settings_excluded_items_placeholder = 例如：*.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = 允许延长时间(空=所有)
settings_allowed_extensions_placeholder = 例如：jpg, png, mp4
settings_excluded_extensions = ExcluDED Extensions
settings_excluded_extensions_placeholder = 例如：bak、tmp、日志
# Settings - Tools section labels
settings_duplicates_header = 脱机
settings_check_method_label = 竞技模式
settings_check_method = 方法
settings_hash_type_label = HASH 类型
settings_hash_type = 哈希类型
settings_hash_type_desc = Blake3 - 推荐选项，CRC32 几率较小的假阳性
settings_similar_images_header = 缩放图像
settings_similarity_preset = 相似度阈值
settings_similarity_desc = 非常高 = 仅几乎完全相同
settings_hash_size = 哈希大小
settings_hash_size_desc = 较大的尺寸，较小的正直性，但也发现较不相似的图像
settings_hash_alg = 哈希算法
settings_image_filter = 调整滤镜大小
settings_ignore_same_size = 忽略同一尺寸的图像
settings_gallery_image_fit_cover = 相册：裁剪到正方形
settings_gallery_image_fit_cover_desc = 填充电量；禁用以保持原始宽高比
settings_big_files_header = BIGES文件
settings_search_mode = 搜索模式
settings_file_count = 文件计数
settings_same_music_header = 我的大家都知道这个问题
settings_music_check_method = 比较模式
settings_music_compare_tags_label = 美容战术师
settings_music_title = 标题
settings_music_artist = 艺人
settings_music_year = 年份
settings_music_length = 长度
settings_music_genre = 流派数
settings_music_bitrate = 位速率
settings_music_approx = 近似标签比较
settings_broken_files_header = 蓝色文件
settings_broken_files_note = 资源密集扫描。为了最佳性能，请在桌面上使用 Krokiet。.
settings_broken_files_types_label = 选择类型
settings_broken_audio = 音频
settings_broken_pdf = PDF
settings_broken_archive = 存档
settings_broken_image = 图片
settings_bad_names_header = 备用名称
settings_bad_names_checks_label = 搜索
settings_bad_names_uppercase_ext = 大写扩展
settings_bad_names_emoji = 表情符号名称
settings_bad_names_space = 开始/结束时的空间
settings_bad_names_non_ascii = 非ASCII字符
settings_bad_names_duplicated = 重复字符
# Settings - Diagnostics tab
diagnostics_header = 恐龙座
diagnostics_thumbnails = 缩略图缓存
diagnostics_app_cache = 应用缓存
diagnostics_refresh = 刷新
diagnostics_clear_thumbnails = 清除缩略图
diagnostics_open_thumbnails_folder = 打开文件夹
diagnostics_clear_cache = 清除缓存
diagnostics_open_cache_folder = 打开文件夹
diagnostics_collect_test = 文件访问测试
diagnostics_collect_test_desc = 检查可以访问多少个文件
diagnostics_collect_test_run = 运行
diagnostics_collect_test_stop = 停止
collect_test_cancelled = 被用户停止
diag_confirm_clear_thumbnails = 清除所有缩略图缓存？
diag_confirm_clear_cache = 清除所有应用缓存？
about_repo = 存储库
about_translate = 翻译
about_donate = 支持
# Collect-test result popup
collect_test_title = 测试结果
collect_test_volumes = 卷：
collect_test_folders = 文件夹：
collect_test_files = 文件：
collect_test_time = 时间：
# Licenses
licenses_label = 许可证
third_party_licenses = 第三方许可证
licenses_popup_title = 第三方许可证
# Directories screen
directories_include_header = 包含
directories_included = 包含
directories_exclude_header = 不包含
directories_excluded_header = 不包含
directories_add = 包含
no_paths = 没有路径 - 在下方添加
directories_volume_header = 卷
directories_volume_refresh = 刷新
directories_volume_add = 添加
# Bottom navigation
nav_home = 开始
nav_dirs = 目录
nav_settings = 设置
# Status messages set from Rust
status_ready = 准备好
status_stopped = 已停止
status_no_results = 没有结果
status_deleted_selected = 已删除选中项
status_deleted_with_errors = 删除时出现错误
scan_not_started = 扫描未开始
found_items_prefix = 找到了
found_items_suffix = 项目
deleted_items_prefix = 已删除
deleted_items_suffix = 项目
deleted_errors_suffix = 错误
renamed_prefix = 重命名的
renamed_files_suffix = 文件
renamed_errors_suffix = 错误
cleaned_exif_prefix = 清除 EXIF 来自
cleaned_exif_suffix = 文件
cleaned_exif_errors_suffix = 错误
and_more_prefix = ...及
and_more_suffix = 更多
# Gallery / delete popups
gallery_delete_button = 删除
gallery_back = 后退
gallery_confirm_delete = 是，删除
deleting_files = 正在删除文件...
stop = 停止
files_suffix = 文件
scanning_fallback = 正在扫描...
app_subtitle = 为纪念Cedynia战役（972 CE）
app_license = Czkawka Core - GPL-3.0 的前端
about_app_label = 关于
cache_label = 缓存
# Notification
scan_completed_notification = 扫描完成 - 找到 { $file_count } 个文件。
# Confirm popups (set from Rust)
confirm_clean_exif = 您确定要从 { $n } 个选定的文件中清除 EXIF 标签吗？
confirm_delete_items = 您确定要删除 { $n } 个选中的项目吗？
gallery_confirm_delete_msg = 您即将删除 { $total_images } 张图片，这些图片位于 { $total_groups } 个分组中。.
gallery_confirm_delete_warning = 所有商品均已从 { $unsafe_groups } 个组别中挑选！
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = 计算和比较音频指纹耗费大量资源，可能需要很长时间。 建议在桌面系统上使用Krokiet来完成这项任务。.
# Scan stage labels (shown during scan progress)
stage_collecting_files = 正在收集文件
stage_scanning_name = 按名称扫描
stage_scanning_size_name = 按名称和大小扫描
stage_scanning_size = 按大小扫描
stage_pre_hash = 预散列
stage_full_hash = 正在散列
stage_loading_cache = 加载缓存
stage_saving_cache = 正在保存缓存
stage_calculating_image_hashes = 正在计算图像哈希值
stage_comparing_images = 比较图像
stage_calculating_video_hashes = 计算视频哈希值
stage_checking_files = 正在检查文件
stage_checking_extensions = 正在检查扩展
stage_checking_names = 正在检查名称
stage_reading_music_tags = 读取音乐标签
stage_comparing_tags = 标签比较
stage_calculating_music_fingerprints = 正在计算音乐指纹
stage_comparing_fingerprints = 比较指纹
stage_extracting_exif = 读取 EXIF 标签
stage_creating_video_thumbnails = 创建视频缩略图
stage_processing_videos = 正在处理视频
stage_deleting = 正在删除文件
stage_renaming = 重命名文件
stage_moving = 正在移动文件
stage_hardlinking = 创建硬链接
stage_symlinking = 创建符号链接
stage_optimizing_videos = 优化视频
stage_cleaning_exif = 正在清理 EXIF
# Group headers in scan results
duplicates_group_header = { $count } 个文件 x { $per_file } / 个文件 = 共有 { $total } 个文件
similar_images_group_header = { $count } 张相似图片
same_music_group_header = { $count } 首相似歌曲
# Rename confirmation
confirm_rename_items = 您确定要重命名所选中的 { $n } 个文件吗？
# Combo-box option labels (translatable display names)
option_search_mode_biggest = 最大的
option_search_mode_smallest = 最小值
option_similarity_very_high = 高
option_similarity_high = 高
option_similarity_medium = 中
option_similarity_low = 低
option_similarity_very_low = 低
option_similarity_minimal = 最小值.
option_check_method_hash = 哈希
option_check_method_name = 名称
option_check_method_size_and_name = 大小+名称
option_check_method_size = 大小
option_music_method_tags = 标签
option_music_method_audio = 音频
option_min_size_none = 无
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = 无限制
# Volume labels (shown in the directories screen)
volume_internal_storage = 内部存储
volume_sd_card = 内存卡 (SD 卡)
volume_storage = 存储音量
# Directories screen
directories_referenced_tooltip = 引用(未删除)
directories_include_section_header = 包含
directories_exclude_section_header = 外观
directories_custom_paths = 自定义路径
directories_check_button = 分析
directories_check_popup_title = 目录统计
directories_check_label_included = 包含的路径：
directories_check_label_excluded = 排除的路径：
directories_check_label_referenced = 参考路径：
directories_check_label_would_scan = 要扫描的文件：
directories_check_label_processable = 可处理的文件：
directories_check_scanning = 正在扫描...
directories_check_warning_no_processable = 未找到可处理的文件 - 请验证您所包含/排除的文件夹
path_edit_title_include = 添加到包含
path_edit_title_exclude = 添加到排除
path_edit_placeholder = 输入路径...
path_edit_not_exists = 路径不存在
path_edit_is_dir = 目录
path_edit_is_file = 文件
path_edit_no_newlines = 路径不能包含换行符 - 输入密钥是不允许的
ctx_menu_title = 打开
ctx_open_file = 打开项目
ctx_open_folder = 打开父文件夹
