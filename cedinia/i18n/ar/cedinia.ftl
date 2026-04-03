# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = التكرارات
tool_empty_folders = مجلدات فارغة
tool_similar_images = صور مشابهة
tool_empty_files = ملفات فارغة
tool_temporary_files = الملفات المؤقتة
tool_big_files = أكبر الملفات
tool_broken_files = الملفات المكسورة
tool_bad_extensions = ملحقات سيئة
tool_same_music = مكرر الموسيقى
tool_bad_names = أسماء سيئة
tool_exif_remover = بيانات EXIF
tool_directories = المجلدات
tool_settings = الإعدادات
# Home screen tool card descriptions
home_dup_description = البحث عن ملفات بنفس المحتوى
home_empty_folders_description = المجلدات بدون محتوى
home_similar_images_description = العثور على صور مشابهة بصرياً
home_empty_files_description = ملفات بحجم صفر
home_temp_files_description = الملفات المؤقتة والمخزونة
home_big_files_description = ملفات Biggest/Mallallest على القرص
home_broken_files_description = PDF، السمع، الصور، الأرشيف
home_bad_extensions_description = ملفات ذات ملحق غير صالح
home_same_music_description = ملفات صوتية مشابهة بواسطة العلامات
home_bad_names_description = الملفات ذات الأحرف الإشكالية في الاسم
home_exif_description = صور مع بيانات التعريف EXIF
# Results list
scanning = جاري الفحص...
stopping = التوقف...
no_results = لا توجد نتائج
press_start = اضغط على START للمسح
select_label = سيّل.
deselect_label = ديزل.
list_label = قائمة
gallery_label = غلا.
# Selection popup
selection_popup_title = حدد
select_all = حدد الكل
select_except_one = حدد الكل باستثناء واحد
select_except_largest = حدد الكل باستثناء الاهم
select_except_smallest = حدد الكل باستثناء الأصغر
select_largest = حدد أكبر
select_smallest = اختيار أصغر
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
permission_body = للمسح الضوئي للملفات، يحتاج التطبيق إلى الوصول إلى تخزين الجهاز. بدون هذا الإذن، لن يكون المسح الضوئي ممكنا.
grant = منح
no_permission_scan_warning = لا يوجد وصول إلى الملف - منح الإذن بالمسح
# Settings screen tabs
settings_tab_general = عام
settings_tab_tools = أدوات
settings_tab_diagnostics = معلومات
# Settings - General tab
settings_use_cache = استخدام ذاكرة التخزين المؤقت
settings_use_cache_desc = تسريع المسح اللاحق (هاش/صور)
settings_ignore_hidden = تجاهل الملفات المخفية
settings_ignore_hidden_desc = الملفات والمجلدات التي تبدأ بـ '.'
settings_show_notification = إشعار عند الانتهاء من المسح
settings_show_notification_desc = إظهار إشعار النظام عند الانتهاء من المسح
settings_notify_only_background = فقط عندما تكون في الخلفية
settings_notify_only_background_desc = تخطي الإشعار إذا كان التطبيق مرئيًا
notifications_disabled_banner = تم تعطيل الإشعارات
notifications_enable_button = تمكين
settings_scan_label = سكون
settings_filters_label = FILTERS (بعض الأدوات)
settings_min_file_size = الحد الأدنى لحجم الملف
settings_max_file_size = الحد الأقصى لحجم الملف
settings_language = اللغة
settings_language_restart = يتطلب إعادة تشغيل التطبيق
settings_common_label = إعدادات الاتصال
settings_excluded_items = اكتشف الصور (أنماط القذائف، مفصولة بفاصلة)
settings_excluded_items_placeholder = مثال: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = اختصارات محجوبة (فارغة = الكل)
settings_allowed_extensions_placeholder = مثل jpg, png, mp4
settings_excluded_extensions = نسخ مستخرجة
settings_excluded_extensions_placeholder = مثل الخبز ، tmp، سجل
# Settings - Tools section labels
settings_duplicates_header = المحتويات
settings_check_method_label = التعليق على الطريقة
settings_check_method = الطريقة
settings_hash_type_label = نوع هاش
settings_hash_type = نوع التجزئة
settings_hash_type_desc = Blake3 - الخيار الموصى به، اتفاقية CRC32 لديها فرصة صغيرة للإيجابيات الكاذبة
settings_similar_images_header = الآثار الخاصة
settings_similarity_preset = عتبة التشابه
settings_similarity_desc = عالية جدا = شبه متطابقة فقط
settings_hash_size = حجم التجزئة
settings_hash_size_desc = أحجام أكبر، لها إيجابيات خاطئة أقل، ولكنها أيضا تجد صورا أقل تشابه
settings_hash_alg = خوارزمية التجزئة
settings_image_filter = تغيير حجم الفلتر
settings_ignore_same_size = تجاهل الصور ذات الأبعاد نفسها
settings_gallery_image_fit_cover = المعرض: محصول إلى مربع
settings_gallery_image_fit_cover_desc = ملء الرتبة؛ تعطيل للحفاظ على نسبة العرض الأصلية
settings_big_files_header = خلل كبير
settings_search_mode = وضع البحث
settings_file_count = عدد الملفات
settings_same_music_header = التعاريف الرئيسية
settings_music_check_method = وضع المقارنة
settings_music_compare_tags_label = التسلسل الشامل
settings_music_title = العنوان
settings_music_artist = الفنان
settings_music_year = السنة
settings_music_length = طول
settings_music_genre = النوع
settings_music_bitrate = معدل
settings_music_approx = مقارنة الوسوم التقريبية
settings_broken_files_header = الحفلات المدفوعه
settings_broken_files_note = المسح الكثيف للموارد. للحصول على أفضل أداء استخدم Krokiet على سطح المكتب.
settings_broken_files_types_label = التحقق من اللعبة
settings_broken_audio = الصوت
settings_broken_pdf = ملف PDF
settings_broken_archive = أرشيف
settings_broken_image = صورة
settings_bad_names_header = أسماء العملاء
settings_bad_names_checks_label = التدوين
settings_bad_names_uppercase_ext = ملحق الحالة العلوية
settings_bad_names_emoji = الرموز التعبيرية باسم
settings_bad_names_space = المسافات عند البدء/النهاية
settings_bad_names_non_ascii = الأحرف غير المستخدمة في معيار ASCII
settings_bad_names_duplicated = أحرف متكررة
# Settings - Diagnostics tab
diagnostics_header = الأجهزة،
diagnostics_thumbnails = ذاكرة تخزين مؤقت للصور المصغرة
diagnostics_app_cache = ذاكرة التخزين المؤقت للتطبيق
diagnostics_refresh = تحديث
diagnostics_clear_thumbnails = مسح الصور المصغرة
diagnostics_open_thumbnails_folder = فتح المجلد
diagnostics_clear_cache = مسح ذاكرة التخزين المؤقت
diagnostics_open_cache_folder = فتح المجلد
diagnostics_collect_test = اختبار الوصول إلى الملف
diagnostics_collect_test_desc = تحقق من عدد الملفات التي يمكن الوصول إليها
diagnostics_collect_test_run = تشغيل
diagnostics_collect_test_stop = توقف
collect_test_cancelled = توقف من قبل المستخدم
diag_confirm_clear_thumbnails = مسح جميع ذاكرة التخزين المؤقت المصغرة؟
diag_confirm_clear_cache = مسح ذاكرة التخزين المؤقت للتطبيق؟
about_repo = المستودع
about_translate = الترجمات
about_donate = الدعم
# Collect-test result popup
collect_test_title = نتائج الاختبار
collect_test_volumes = المجلدات:
collect_test_folders = المجلدات:
collect_test_files = الملفات:
collect_test_time = الوقت:
# Licenses
licenses_label = ليسانس
third_party_licenses = تراخيص طرف ثالث
licenses_popup_title = تراخيص الطرف الثالث
# Directories screen
directories_include_header = تضمين
directories_included = متضمن
directories_exclude_header = استبعاد
directories_excluded_header = مستبعد
directories_add = تضمين
no_paths = لا توجد مسارات - أضف أدناه
directories_volume_header = الصوت
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
found_items_suffix = العناصر
deleted_items_prefix = محذوف
deleted_items_suffix = العناصر
deleted_errors_suffix = أخطاء
renamed_prefix = أُعيد تسميته
renamed_files_suffix = الملفات
renamed_errors_suffix = أخطاء
cleaned_exif_prefix = مسح EXIF من
cleaned_exif_suffix = الملفات
cleaned_exif_errors_suffix = أخطاء
and_more_prefix = ...و
and_more_suffix = المزيد
# Gallery / delete popups
gallery_delete_button = حذف
gallery_back = الرجوع
gallery_confirm_delete = نعم، حذف
deleting_files = حذف الملفات...
stop = توقف
files_suffix = الملفات
scanning_fallback = جارٍ الفحص...
app_subtitle = تكريما لمعركة سيدينيا (972 CE)
app_license = الواجهة لنواة Czkawka - GPL-3.0
about_app_label = من
cache_label = التوقف
# Notification
scan_completed_notification = تم مسح العناصر - { $file_count } الموجودة
# Confirm popups (set from Rust)
confirm_clean_exif = هل أنت متأكد من أنك تريد تنظيف وسوم EXIF من الملفّات المختارة{ $n}؟
confirm_delete_items = هل أنت متأكد من أنك تريد حذف { $n } العناصر المحددة؟
gallery_confirm_delete_msg = أنت على وشك حذف صور { $total_images } في مجموعات { $total_groups}.
gallery_confirm_delete_warning = تم تحديد جميع العناصر في مجموعة { $unsafe_groups}!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = إن حساب ومقارنة بصمات الصوت كثيف الموارد وقد يستغرق وقتا طويلا. ويوصى باستخدام كروكييت على نظام حاسوب مكتبي لهذه المهمة.
# Scan stage labels (shown during scan progress)
stage_collecting_files = جمع الملفات
stage_scanning_name = البحث عن طريق الاسم
stage_scanning_size_name = البحث حسب الاسم والحجم
stage_scanning_size = البحث حسب الحجم
stage_pre_hash = التجزئة السابقة
stage_full_hash = التجزئة
stage_loading_cache = تحميل ذاكرة التخزين المؤقت
stage_saving_cache = حفظ ذاكرة التخزين المؤقت
stage_calculating_image_hashes = حساب تجزئة الصور
stage_comparing_images = مقارنة الصور
stage_calculating_video_hashes = حساب تجزئة الفيديو
stage_checking_files = التحقق من الملفات
stage_checking_extensions = التحقق من الإضافات
stage_checking_names = التحقق من الأسماء
stage_reading_music_tags = قراءة علامات الموسيقى
stage_comparing_tags = مقارنة العلامات
stage_calculating_music_fingerprints = حساب بصمات الموسيقى
stage_comparing_fingerprints = مقارنة بصمات الأصابع
stage_extracting_exif = قراءة علامات EXIF
stage_creating_video_thumbnails = إنشاء مصغرات الفيديو
stage_processing_videos = معالجة الفيديوهات
stage_deleting = حذف الملفات
stage_renaming = إعادة تسمية الملفات
stage_moving = نقل الملفات
stage_hardlinking = إنشاء روابط صلبة
stage_symlinking = إنشاء الروابط الرمزية
stage_optimizing_videos = تحسين الفيديوهات
stage_cleaning_exif = تنظيف EXIF
# Group headers in scan results
duplicates_group_header = { $count } ملفات x { $per_file } / الملف = { $total } المجموع
similar_images_group_header = { $count } صور مشابهة
same_music_group_header = { $count } مسارات مشابهة
# Rename confirmation
confirm_rename_items = هل أنت متأكد من أنك تريد إعادة تسمية { $n } من الملفات المحددة؟
# Combo-box option labels (translatable display names)
option_search_mode_biggest = أكبر
option_search_mode_smallest = أصغر
option_similarity_very_high = خامساً - عالي
option_similarity_high = مرتفع
option_similarity_medium = متوسط
option_similarity_low = منخفض
option_similarity_very_low = خامساً - انخفاض
option_similarity_minimal = دقيقة.
option_check_method_hash = التجزئة
option_check_method_name = الاسم
option_check_method_size_and_name = الحجم+الاسم
option_check_method_size = الحجم
option_music_method_tags = الوسوم
option_music_method_audio = الصوت
option_min_size_none = لا
option_min_size_1kb = 1 كيلوبايت
option_min_size_8kb = 8 كيلوبايت
option_min_size_64kb = 64 كيلوبايت
option_min_size_1mb = 1 ميغابايت
option_max_size_16kb = 16 كيلوبايت
option_max_size_1mb = 1 ميغابايت
option_max_size_10mb = 10 ميغابايت
option_max_size_100mb = 100 ميغابايت
option_max_size_unlimited = غير محدود
# Volume labels (shown in the directories screen)
volume_internal_storage = التخزين الداخلي
volume_sd_card = بطاقة الذاكرة (بطاقة SD)
volume_storage = حجم التخزين
# Directories screen
directories_referenced_tooltip = المشار إليها (غير محذوفة)
directories_include_section_header = مشاهدة
directories_exclude_section_header = أكمل
directories_custom_paths = مسارات مخصصة
directories_check_button = تحليل
directories_check_popup_title = إحصائيات الدليل
directories_check_label_included = المسارات المضمنة:
directories_check_label_excluded = المسارات المستثناة:
directories_check_label_referenced = المسارات المرجعية:
directories_check_label_would_scan = الملفات المراد فحصها:
directories_check_label_processable = الملفات القابلة للتطبيق:
directories_check_scanning = جارٍ الفحص...
directories_check_warning_no_processable = لم يتم العثور على ملفات قابلة للمعالجة - التحقق من المجلدات المشمولة/المستبعدة
path_edit_title_include = إضافة إلى تضمين
path_edit_title_exclude = إضافة إلى استبعاد
path_edit_placeholder = أدخل المسار...
path_edit_not_exists = المسار غير موجود
path_edit_is_dir = دليل
path_edit_is_file = ملف
path_edit_no_newlines = المسارات لا يمكن أن تحتوي على خطوط جديدة — مفتاح الإدخال غير مسموح به
ctx_menu_title = فتح
ctx_open_file = فتح العنصر
ctx_open_folder = فتح المجلد الأصل
