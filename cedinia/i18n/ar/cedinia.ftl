# Cedinia - English (fallback)

# App / top bar titles
tool_duplicate_files = التكرارات
tool_empty_folders = مجلدات فارغة
tool_similar_images = صور مشابهة
tool_empty_files = ملفات فارغة
tool_temporary_files = الملفات المؤقتة
tool_big_files = أكبر الملفات
tool_broken_files = الملفات التالفة
tool_bad_extensions = ملحقات سيئة
tool_same_music = تكرارات الموسيقى
tool_bad_names = أسماء سيئة
tool_exif_remover = بيانات EXIF
tool_similar_videos = فيديوهات مشابهة (صوت)
tool_directories = المجلدات
tool_settings = الإعدادات
# Home screen tool card descriptions
home_dup_description = البحث عن ملفات بنفس المحتوى
home_empty_folders_description = المجلدات بدون محتوى
home_similar_images_description = العثور على صور مشابهة بصرياً
home_empty_files_description = ملفات بحجم صفر
home_temp_files_description = الملفات المؤقتة والمخبأة
home_big_files_description = أكبر/أصغر الملفات على القرص
home_broken_files_description = PDF، الصوت، الصور، الأرشيف
home_bad_extensions_description = ملفات ذات امتداد غير صالح
home_same_music_description = ملفات صوتية مشابهة بواسطة الوسوم
home_bad_names_description = الملفات ذات الأحرف الإشكالية في الاسم
home_exif_description = صور مع بيانات التعريف EXIF
home_similar_videos_description = العثور على مقاطع فيديو ذات صوت مشابه
# Results list
scanning = جاري الفحص...
stopping = جارٍ التوقف...
no_results = لا توجد نتائج
press_start = اضغط على «ابدأ» لبدء الفحص
select_label = تحد.
deselect_label = إلغ.
list_label = قائمة
gallery_label = معرض
# Selection popup
selection_popup_title = حدد
select_all = حدد الكل
select_except_one = حدد الكل باستثناء واحد
select_except_largest = حدد الكل باستثناء الأكبر
select_except_smallest = حدد الكل باستثناء الأصغر
select_largest = حدد أكبر
select_smallest = حدد أصغر
select_except_highest_res = حدد الكل باستثناء أعلى دقة
select_except_lowest_res = حدد الكل باستثناء أدنى دقة
select_highest_res = حدد أعلى دقة
select_lowest_res = حدد أدنى دقة
invert_selection = عكس التحديد
close = أغلق
# Deselection popup
deselection_popup_title = إلغاء التحديد
deselect_all = إلغاء تحديد الكل
deselect_except_one = إلغاء تحديد الكل باستثناء واحد
# Confirm popup
cancel = إلغاء
delete = حذف
rename = إعادة تسمية
# Delete errors popup
delete_errors_title = فشل في حذف بعض الملفات:
ok = حسناً
# Stopping overlay
stopping_overlay_title = التوقف
stopping_overlay_body =
    جارٍ إنهاء الفحص الحالي...
    الرجاء الانتظار.
# Permission popup
permission_title = الوصول إلى الملف
permission_body = لفحص الملفات، يحتاج التطبيق إلى الوصول إلى تخزين الجهاز. بدون هذا الإذن، لن يكون الفحص ممكناً.
grant = منح
no_permission_scan_warning = لا يوجد وصول إلى الملف - امنح الإذن بالفحص
# Settings screen tabs
settings_tab_general = عام
settings_tab_tools = أدوات
settings_tab_diagnostics = معلومات
# Settings - General tab
settings_use_cache = استخدام ذاكرة التخزين المؤقت
settings_use_cache_desc = تسريع الفحص اللاحق (التجزئة/الصور)
settings_ignore_hidden = تجاهل الملفات المخفية
settings_ignore_hidden_desc = الملفات والمجلدات التي تبدأ بـ '.'
settings_show_notification = إشعار عند الانتهاء من الفحص
settings_show_notification_desc = إظهار إشعار النظام عند الانتهاء من الفحص
settings_notify_only_background = فقط عندما يكون التطبيق في الخلفية
settings_notify_only_background_desc = تخطي الإشعار إذا كان التطبيق مرئيًا
notifications_disabled_banner = تم تعطيل الإشعارات
notifications_enable_button = تمكين
settings_scan_label = الفحص
settings_filters_label = المرشحات (بعض الأدوات)
settings_min_file_size = الحد الأدنى لحجم الملف
settings_max_file_size = الحد الأقصى لحجم الملف
settings_language = اللغة
settings_language_restart = يتطلب إعادة تشغيل التطبيق
settings_common_label = الإعدادات العامة
settings_excluded_items = العناصر المستبعدة (أنماط glob، مفصولة بفواصل)
settings_excluded_items_placeholder = مثال: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = الامتدادات المسموح بها (فارغة = الكل)
settings_allowed_extensions_placeholder = مثل jpg, png, mp4
settings_excluded_extensions = الامتدادات المستبعدة
settings_excluded_extensions_placeholder = مثل bak، tmp، سجل
# Settings - Tools section labels
settings_duplicates_header = التكرارات
settings_check_method_label = طريقة المقارنة
settings_check_method = الطريقة
settings_hash_type_label = نوع التجزئة
settings_hash_type = نوع التجزئة
settings_hash_type_desc = Blake3 هو الخيار الموصى به؛ خوارزمية CRC32 لديها احتمال ضئيل للإيجابيات الكاذبة
settings_similar_images_header = الصور المتشابهة
settings_similarity_preset = عتبة التشابه
settings_similarity_desc = عالية جدا = شبه متطابقة فقط
settings_hash_size = حجم التجزئة
settings_hash_size_desc = أحجام أكبر، لها إيجابيات خاطئة أقل، ولكنها أيضا تجد صورا أقل تشابه
settings_hash_alg = خوارزمية التجزئة
settings_image_filter = فلتر تغيير الحجم
settings_geometric_invariance = ثبات هندسي
settings_ignore_same_size = تجاهل الصور ذات الأبعاد نفسها
settings_gallery_image_fit_cover = المعرض: اقتصاص إلى مربع
settings_gallery_image_fit_cover_desc = ملء المربع؛ تعطيل للحفاظ على نسبة العرض الأصلية
settings_big_files_header = أكبر الملفات
settings_search_mode = وضع البحث
settings_file_count = عدد الملفات
settings_same_music_header = تكرارات الموسيقى
settings_music_check_method = وضع المقارنة
settings_music_compare_tags_label = الوسوم المقارنة
settings_music_title = العنوان
settings_music_artist = الفنان
settings_music_year = السنة
settings_music_length = الطول
settings_music_genre = النوع
settings_music_bitrate = معدل البت
settings_music_approx = مقارنة تقريبية للوسوم
settings_temporary_files_header = الملفات المؤقتة
settings_temporary_files_extensions_label = الامتدادات
settings_temporary_files_extensions_placeholder = مثال: .tmp، .bak، ~
settings_temporary_files_reset = إعادة التعيين إلى الافتراضي
settings_broken_files_header = الملفات التالفة
settings_broken_files_note = المسح الكثيف للموارد. للحصول على أفضل أداء استخدم Krokiet على سطح المكتب.
settings_broken_files_types_label = الأنواع المفحوصة
settings_broken_audio = الصوت
settings_broken_pdf = ملف PDF
settings_broken_archive = أرشيف
settings_broken_image = صورة
settings_broken_font = الخط
settings_broken_markup = علامات التنسيق (JSON/XML/TOML)
settings_similar_videos_header = فيديوهات مشابهة (صوت)
settings_similar_videos_audio_preset = الإعداد المسبق لتشابه الصوت
settings_similar_videos_audio_preset_desc = يتحكم في مدى الدقة المطلوبة لتطابق الصوت
settings_bad_names_header = أسماء سيئة
settings_bad_names_checks_label = الفحوصات
settings_bad_names_uppercase_ext = الامتداد بأحرف كبيرة
settings_bad_names_emoji = الرموز التعبيرية في الاسم
settings_bad_names_space = المسافات في البداية/النهاية
settings_bad_names_non_ascii = الأحرف غير المستخدمة في معيار ASCII
settings_bad_names_duplicated = أحرف متكررة
settings_ignore_same_resolution = تجاهل الصور بنفس الدقة
# Settings - Appearance section
settings_appearance_label = مظهر
settings_dark_theme = السمة المظلمة
settings_dark_theme_desc = استخدام نظام الألوان المظلم
# Settings - Diagnostics tab
diagnostics_header = التشخيص
diagnostics_thumbnails = ذاكرة تخزين مؤقت للصور المصغرة
diagnostics_app_cache = ذاكرة التخزين المؤقت للتطبيق
diagnostics_refresh = تحديث
diagnostics_clear_thumbnails = مسح الصور المصغرة
diagnostics_open_thumbnails_folder = فتح المجلد
diagnostics_clear_cache = مسح ذاكرة التخزين المؤقت
diagnostics_open_cache_folder = فتح المجلد
diagnostics_export_logs = تصدير السجلات
logs_label = سجلات
logs_export_title = تصدير السجلات
logs_export_saved = تم نسخ السجلات إلى:
logs_export_failed = تعذر تصدير السجلات
diagnostics_collect_test = اختبار الوصول إلى الملف
diagnostics_collect_test_desc = تحقق من عدد الملفات التي يمكن الوصول إليها
diagnostics_collect_test_run = تشغيل
diagnostics_collect_test_stop = توقف
collect_test_cancelled = توقف من قبل المستخدم
diag_confirm_clear_thumbnails = مسح جميع ذاكرة التخزين المؤقت للصور المصغرة؟
diag_confirm_clear_cache = مسح ذاكرة التخزين المؤقت للتطبيق؟
about_repo = المستودع
about_translate = الترجمات
about_donate = تبرع
# Collect-test result popup
collect_test_title = نتائج الاختبار
collect_test_volumes = وحدات التخزين:
collect_test_folders = المجلدات:
collect_test_files = الملفات:
collect_test_time = الوقت:
# Licenses
licenses_label = الترخيص
third_party_licenses = تراخيص طرف ثالث
licenses_popup_title = تراخيص الطرف الثالث
# Directories screen
directories_include_header = تضمين
directories_included = متضمن
directories_exclude_header = استبعاد
directories_excluded_header = مستبعد
directories_add = تضمين
no_paths = لا توجد مسارات - أضف أدناه
directories_volume_header = وحدات التخزين
directories_volume_refresh = تحديث
directories_volume_add = إضافة
# Bottom navigation
nav_home = ابدأ
nav_dirs = المجلدات
nav_settings = الإعدادات
# Status messages set from Rust
status_ready = مستعد
status_stopped = توقفت
status_no_results = لا توجد نتائج
status_deleted_selected = تم حذف المحدد
status_deleted_with_errors = تم الحذف مع الأخطاء
scan_not_started = لم يبدأ الفحص
found_items_prefix = موجود
found_items_suffix = عناصر
deleted_items_prefix = محذوف
deleted_items_suffix = عناصر
deleted_errors_suffix = أخطاء
renamed_prefix = أُعيد تسميته
renamed_files_suffix = ملفات
renamed_errors_suffix = أخطاء
cleaned_exif_prefix = مسح EXIF من
cleaned_exif_suffix = ملفات
cleaned_exif_errors_suffix = أخطاء
rename_error_read_file_name = لا يمكن قراءة اسم الملف
rename_error_read_directory = لا يمكن قراءة الدليل
and_more_prefix = ...و
and_more_suffix = المزيد
# Gallery / delete popups
gallery_delete_button = حذف
gallery_back = الرجوع
gallery_confirm_delete = نعم، حذف
deleting_files = جارٍ حذف الملفات...
stop = توقف
scanning_fallback = جارٍ الفحص...
app_subtitle = تكريما لمعركة سيدينيا (972 CE)
app_license = الواجهة لنواة Czkawka - GPL-3.0
about_app_label = حول
cache_label = ذاكرة التخزين المؤقت
# Notification
scan_completed_notification = اكتمل الفحص - تم العثور على { $file_count } عنصر
# Confirm popups (set from Rust)
confirm_clean_exif = هل أنت متأكد من أنك تريد تنظيف وسوم EXIF من الملفات المحددة ({ $n })؟
confirm_delete_items = هل أنت متأكد من أنك تريد حذف { $n } عنصر محدد؟
gallery_confirm_delete_msg = أنت على وشك حذف { $total_images } صورة في { $total_groups } مجموعة.
gallery_confirm_delete_warning = تم تحديد جميع العناصر في { $unsafe_groups } من المجموعات!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = ويوصى باستخدام Krokiet على نظام حاسوب مكتبي لهذه المهمة.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } ملفات x { $per_file } / الملف = { $total } المجموع
similar_images_group_header = { $count } صور مشابهة
same_music_group_header = { $count } مسارات مشابهة
similar_videos_group_header = { $count } مقاطع فيديو مشابهة
# Rename confirmation
confirm_rename_items = هل أنت متأكد من أنك تريد إعادة تسمية { $n } من الملفات المحددة؟
# Combo-box option labels (translatable display names)
option_search_mode_biggest = أكبر
option_search_mode_smallest = أصغر
option_similarity_very_high = عالٍ جدًا
option_similarity_high = مرتفع
option_similarity_medium = متوسط
option_similarity_low = منخفض
option_similarity_very_low = منخفض جدًا
option_similarity_minimal = الحد الأدنى
option_check_method_hash = التجزئة
option_check_method_name = الاسم
option_check_method_size_and_name = الحجم+الاسم
option_check_method_size = الحجم
option_music_method_tags = الوسوم
option_music_method_audio = الصوت
option_min_size_none = لا
option_max_size_unlimited = غير محدود
option_audio_preset_identical = متطابق
option_audio_preset_clip = مقطع ضمن الأطول
option_audio_preset_similar = مشابه
# Volume labels (shown in the directories screen)
volume_internal_storage = التخزين الداخلي
volume_sd_card = بطاقة الذاكرة (بطاقة SD)
volume_storage = وحدة التخزين
# Directories screen
directories_referenced_tooltip = المشار إليها (غير محذوفة)
directories_include_section_header = المضمّنة
directories_exclude_section_header = المستبعدة
directories_custom_paths = مسارات مخصصة
directories_check_button = تحليل
directories_check_popup_title = إحصائيات الدليل
directories_check_label_included = المسارات المضمنة:
directories_check_label_excluded = المسارات المستبعدة:
directories_check_label_referenced = المسارات المرجعية:
directories_check_label_would_scan = الملفات المراد فحصها:
directories_check_label_processable = الملفات القابلة للمعالجة:
directories_check_scanning = جارٍ الفحص...
directories_check_warning_no_processable = لم يتم العثور على ملفات قابلة للمعالجة - التحقق من المجلدات المشمولة/المستبعدة
path_edit_title_include = إضافة إلى تضمين
path_edit_title_exclude = إضافة إلى استبعاد
path_edit_placeholder = أدخل المسار...
path_edit_not_exists = المسار غير موجود
path_edit_is_dir = دليل
path_edit_is_file = ملف
path_edit_no_newlines = المسارات لا يمكن أن تحتوي على خطوط جديدة - مفتاح الإدخال غير مسموح به
ctx_menu_title = فتح
ctx_open_file = فتح العنصر
ctx_open_folder = فتح المجلد الأصل
dir_open_folder = فتح المجلد
# Compare view
compare_label = قارن
compare_loading = تحميل الصور...
compare_cancelling = إلغاء...
compare_computing = جارٍ حساب الفرق...
compare_mode_normal = جانب
compare_mode_split = تقسيم
compare_mode_overlay = تراكب
compare_mode_diff = فرق
compare_res_mismatch = درجات دقة مختلفة - قد يكون الفرق غير دقيق
