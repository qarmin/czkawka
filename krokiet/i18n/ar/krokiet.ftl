# In Rust translations
rust_init_error_title = خطأ حرج أثناء بدء التطبيق
rust_init_error_message = 
        حدث خطأ حرج أثناء بدء التطبيق:

        { $error_message }

        قد يكون ذلك بسبب نقص أو تلف تعريفات OpenGL/Vulkan، أو تشغيل التطبيق في جهاز افتراضي أو خلل في Krokiet أو أحد مكتباته.

        يمكنك محاولة تشغيل إصدارات مختلفة (skia_opengl، skia_vulkan، femtovg_opengl - الافتراضي) أو مع مُسرِّع رسومي للبرامج لتحديد ما إذا كان ذلك يحل المشكلة.
rust_loaded_preset = تم تحميل الإعداد المسبق { $preset_idx }
rust_file_already_exists = الملف "{ $file }" موجود بالفعل، ولن يتم الكتابة فوقه
rust_error_removing_file_after_copy = خطأ أثناء حذف الملف "{ $file }" (بعد نسخه إلى قسم مختلف)، السبب: { $reason }
rust_error_copying_file = خطأ أثناء نسخ "{ $input }" إلى "{ $output }"، السبب: { $reason }
rust_loading_tags_cache = تحميل ذاكرة التخزين المؤقت للعلامات
rust_loading_fingerprints_cache = تحميل ذاكرة التخزين المؤقت لبصمات الأصابع
rust_saving_tags_cache = حفظ ذاكرة التخزين المؤقت للعلامات
rust_saving_fingerprints_cache = حفظ ذاكرة التخزين المؤقت لبصمات الأصابع
rust_loading_prehash_cache = تحميل ذاكرة التخزين المؤقت
rust_saving_prehash_cache = حفظ ذاكرة التخزين المؤقت
rust_loading_hash_cache = تحميل ذاكرة التخزين المؤقت للتجزئة
rust_saving_hash_cache = حفظ ذاكرة التخزين المؤقت
rust_loading_exif_cache = تحميل ذاكرة التخزين المؤقت EXIF
rust_saving_exif_cache = حفظ ذاكرة التخزين المؤقت EXIF
rust_scanning_name = فحص اسم الملف { $entries_checked }
rust_scanning_size_name = حجم واسم ملف { $entries_checked }
rust_scanning_size = حجم مسح الملف { $entries_checked }
rust_scanning_file = فحص الملف { $entries_checked }
rust_scanning_folder = فحص { $entries_checked } مجلد
rust_checked_tags = تم التحقق من العلامات { $items_stats }
rust_checked_content = المحتوى المختار من { $items_stats } ({ $size_stats })
rust_compared_tags = مقارنة العلامات { $items_stats }
rust_compared_content = مقارنة محتوى { $items_stats }
rust_hashed_images = تجزئة { $items_stats } صور ({ $size_stats })
rust_compared_image_hashes = مقارنة تجزئة الصور من { $items_stats }
rust_hashed_videos = مجزأة { $items_stats } مقاطع فيديو
rust_created_thumbnails = أنشئ مصغرات لـ { $items_stats } مقاطع فيديو
rust_checked_files = تم تحديد الملف { $items_stats } ({ $size_stats })
rust_checked_files_bad_extensions = تم التحقق من الملف { $items_stats }
rust_checked_files_bad_names = تم التحقق من الملف { $items_stats }
rust_checked_videos = تم التحقق من { $items_stats } مقاطع فيديو ({ $size_stats })
rust_analyzed_partial_hash = تم تحليل التجزئة الجزئية للملفات { $items_stats } ({ $size_stats })
rust_analyzed_full_hash = تم تحليل التجزئة الكاملة من ملفات { $items_stats } ({ $size_stats })
rust_failed_to_rename_file = فشل في إعادة تسمية الملف { $old_path } إلى { $new_path }، الخطأ: { $error }
rust_no_included_paths = لا يمكن بدء المسح عند عدم تحديد المسارات المضمنة.
rust_all_paths_referenced = لا يمكن بدء المسح عندما تكون جميع المسارات المضمنة مضبوطة كمسارات مرجعية، تحتاج إلى تعطيل مربع الاختيار بجوار المسار المدخل.
rust_found_empty_folders = تم العثور على { $items_found } مجلدات فارغة في { $time }
rust_found_empty_files = تم العثور على { $items_found } ملفات فارغة في { $time }
rust_found_similar_images = تم العثور على { $items_found } ملفات صور مماثلة في { $groups } مجموعات في { $time }
rust_found_similar_videos = تم العثور على { $items_found } ملفات فيديو مماثلة في { $groups } مجموعات في { $time }
rust_found_similar_music_files = تم العثور على { $items_found } ملفات موسيقية مماثلة في { $groups } مجموعات في { $time }
rust_found_invalid_symlinks = تم العثور على { $items_found } روابط رموز غير صالحة في { $time }
rust_found_temporary_files = تم العثور على { $items_found } ملفات مؤقتة في { $time }
rust_no_file_type_selected = لا يمكن العثور على الملفات المكسورة بدون أي نوع من الملفات المحددة.
rust_found_broken_files = تم العثور على { $items_found } ملفات مكسورة أخذت { $size } في { $time }
rust_found_bad_extensions = تم العثور على { $items_found } ملفات ذات ملحقات سيئة في { $time }
rust_found_bad_names = تم العثور على { $items_found } ملفات بأسماء سيئة في { $time }
rust_found_video_optimizer = تم العثور على { $items_found } ملفات لتحسينها في { $time }
rust_found_duplicate_files = تم العثور على { $items_found } ملفات مكررة في { $groups } مجموعات أخذت { $size } في { $time }
rust_found_duplicate_files_no_lost_space = تم العثور على { $items_found } ملفات مكررة في { $groups } مجموعات في { $time }
rust_found_big_files = تم العثور على { $items_found } ملفات كبيرة بحجم { $size } في { $time }
rust_found_exif_files = تم العثور على { $items_found } ملفات مع بيانات EXIF في { $time }
rust_cannot_load_preset = لا يمكن تغيير وتحميل الإعداد المسبق { $preset_idx } - السبب { $reason }، باستخدام الإعدادات الافتراضية بدلاً من ذلك
rust_saved_preset = تم الحفظ مسبقا { $preset_idx }
rust_cannot_save_preset = لا يمكن حفظ الإعداد المسبق { $preset_idx } - السبب { $reason }
rust_reset_preset = استرجع التعيين المسبق { $preset_idx }
rust_cannot_create_output_folder = لا يمكن إنشاء مجلد الإخراج { $output_folder }، السبب: { $error }
rust_delete_summary = حذف { $deleted } عناصر ، فشل في إزالة { $failed } عناصر ، من أصل { $total } عناصر
rust_rename_summary = إعادة تسمية العناصر { $renamed } ، فشل في إعادة تسمية العناصر { $failed } ، من أصل { $total } عناصر
rust_move_summary = نقل { $moved } عناصر, فشل في نقل { $failed } عناصر, من { $total } عناصر
rust_hardlink_summary = مرتبط بالرابط { $hardlinked } عناصر، فشل ربط الرابط { $failed } عناصر، من أصل { $total } عناصر
rust_symlink_summary = ربط رمزي { $symlinked } عناصر، فشل ربط رمزي { $failed } عناصر، من أصل { $total } عناصر
rust_optimize_video_summary = مقاطع فيديو مُحسّنة { $optimized }، وفشلت في تحسين { $failed }، وخرجت من { $total } مقاطع فيديو
rust_clean_exif_summary = تمت إزالة EXIF المُنظَّفة من { $cleaned } ملفات، وفشلت في تنظيف { $failed } ملفات، من أصل { $total } ملفات
rust_deleting_files = حذف ملف { $items_stats } ({ $size_stats })
rust_deleting_no_size_files = حذف ملف { $items_stats }
rust_renaming_files = إعادة تسمية الملف { $items_stats }
rust_moving_files = نقل الملف { $items_stats } ({ $size_stats })
rust_moving_no_size_files = نقل ملف { $items_stats }
rust_hardlinking_files = الرابط الصلب { $items_stats } الملف ({ $size_stats })
rust_hardlinking_no_size_files = الرابط الصلب { $items_stats } ملف
rust_symlinking_files = الرابط الرمزية { $items_stats } الملف ({ $size_stats })
rust_symlinking_no_size_files = الرابط الرمزية { $items_stats } ملف
rust_optimizing_videos = مُحسَّن { $items_stats } فيديو ({ $size_stats })
rust_optimizing_no_size_videos = مُحسَّن { $items_stats } فيديو
rust_cleaning_exif = تنظيف EXIF من ملف { $items_stats } ({ $size_stats })
rust_cleaning_no_size_exif = تنظيف EXIF من ملف { $items_stats }
rust_no_files_deleted = لا توجد ملفات أو مجلدات محددة للحذف
rust_no_files_renamed = لا توجد ملفات أو مجلدات محددة لإعادة التسمية
rust_no_files_moved = لا توجد ملفات أو مجلدات محددة للانتقال
rust_no_files_hardlinked = لا توجد ملفات أو مجلدات محددة لإنشاء الروابط الصلبة
rust_no_files_symlinked = لا توجد ملفات أو مجلدات محددة لإنشاء الروابط الرمزية
rust_no_videos_optimized = لا توجد فيديوهات مُحدَّدة للتحسين
rust_no_exif_cleaned = لا توجد ملفات مُحدَّدة لتنظيف EXIF
rust_extracted_exif_tags = تم استخراج علامات EXIF من ملفات { $items_stats } ({ $size_stats })
rust_delete_confirmation = هل أنت متأكد من أنك تريد حذف العناصر المحددة؟
rust_delete_confirmation_number_simple = { $items } العناصر المحددة.
rust_delete_confirmation_number_groups = { $items } العناصر المحددة في { $groups } مجموعات.
rust_delete_confirmation_selected_all_in_group = جميع العناصر المحددة في مجموعات { $groups }.
rust_move_confirmation = هل أنت متأكد من أنك تريد نقل العناصر المحددة؟
rust_move_confirmation_number_simple = { $items } عناصر محددة.
rust_clean_exif_confirmation = هل أنت متأكد من أنك تريد إزالة بيانات EXIF من العناصر المحددة؟
rust_clean_exif_confirmation_number_simple = { $items } عناصر محددة.
clean_exif_overwrite_files_text = استبدل الملفات
rust_optimize_video_confirmation = هل أنت متأكد من أنك تريد تحسين مقاطع الفيديو المحددة؟
rust_optimize_video_confirmation_number_simple = { $items } عناصر محددة.
rust_hardlink_confirmation = هل أنت متأكد من أنك تريد إنشاء روابط صلبة للعناصر المحددة؟
rust_hardlink_confirmation_number_simple = { $items } عناصر محددة.
rust_symlink_confirmation = هل أنت متأكد من أنك تريد إنشاء روابط رمزية للعناصر المحددة؟
rust_symlink_confirmation_number_simple = { $items } عناصر محددة.
rust_rename_confirmation = هل أنت متأكد من أنك تريد إعادة تسمية العناصر المحددة؟
rust_rename_confirmation_number_simple = { $items } عناصر محددة.
rust_cache_processed_files = تمت معالجة ملفات التخزين المؤقت { $files }
rust_cache_entries_stats = تمت إزالة { $removed } من جميع { $all }، { $left } متبقية
rust_cache_size_reduced = تم تقليل حجم ملفات التخزين المؤقت بنسبة { $size }
rust_cache_time_elapsed = الوقت المنقضي: { $time }
rust_symlink_failed = Failed to symlink { $name } to { $target }, reason { $reason }
rust_hardlink_failed = فشل ربط الروابط الصلبة { $name } بـ { $target }، والسبب { $reason }

# Slint translations, but in arrays

column_selection = التحديد
column_size = الحجم
column_file_name = اسم الملف
column_path = المسار
column_modification_date = تاريخ التعديل
column_similarity = تماثل
column_dimensions = الأبعاد
column_new_dimensions = أبعاد جديدة
column_title = العنوان
column_artist = الفنان
column_year = السنة
column_bitrate = معدل
column_length = طول
column_genre = النوع
column_type_of_error = نوع الخطأ
column_symlink_name = اسم الرابط الرمزي
column_symlink_folder = مجلد الرابط الرمزي
column_destination_path = مسار الوجهة
column_current_extension = التمديد الحالي
column_proper_extension = التمديد الصحيح
column_fps = fps
column_codec = ترميز
column_duration = المدة
column_exif_tags = وسوم EXIF
column_new_name = اسم جديد
# Slint translations
ok_button = حسناً
cancel_button = إلغاء
do_you_want_to_continue = هل تريد المتابعة؟
main_window_title = كروكييت - منظف البيانات
scan_button = فحص
stop_button = توقف
stop_text = توقف
select_button = حدد
move_button = نقل
delete_button = حذف
save_button = حفظ
sort_button = فرز
rename_button = إعادة تسمية
motto = هذا البرنامج حر في الاستخدام وسوف يكون دائما.\nراجع رخصة MIT/GPL للحصول على التفاصيل.
unicorn = قد لا تنظر إلى وحيد القرن، ولكن وحيد القرن ينظر إليك دائما.
repository = المستودع
instruction = تعليمات
donation = تبرع
translation = الترجمة
included_paths = المسارات المضمنة
excluded_paths = المسارات المستبعدة
ref = مرجع
path = المسار
tool_duplicate_files = تكرار الملفات
tool_empty_folders = مجلدات فارغة
tool_big_files = ملفات كبيرة
tool_empty_files = ملفات فارغة
tool_temporary_files = الملفات المؤقتة
tool_similar_images = صور مشابهة
tool_similar_videos = مقاطع فيديو مماثلة
tool_music_duplicates = مكرر الموسيقى
tool_invalid_symlinks = الروابط الرمزية غير صالحة
tool_broken_files = الملفات المكسورة
tool_bad_extensions = ملحقات سيئة
tool_bad_names = أسماء سيئة
tool_video_optimizer = مُحسِّن الفيديو
tool_exif_remover = مزيل إكسيف
sort_by_full_name = الترتيب حسب الاسم الكامل
sort_by_selection = الترتيب حسب التحديد
sort_reverse = عكس الترتيب
selection_all = حدد الكل
selection_deselect_all = إلغاء تحديد الكل
selection_invert_selection = عكس التحديد
selection_the_biggest_size = حدد أكبر حجم
selection_the_biggest_resolution = حدد أكبر دقة
selection_the_smallest_size = حدد أصغر حجم
selection_the_smallest_resolution = حدد أصغر دقة
selection_newest = حدد الأحدث
selection_oldest = حدد الأقدم
selection_shortest_path = اختر أقصر مسار
selection_longest_path = اختر أطول مسار
stage_current = المرحلة الحالية:
stage_all = جميع المراحل:
subsettings = الإعدادات الفرعية
subsettings_images_hash_size = حجم التجزئة
subsettings_images_resize_algorithm = تغيير حجم الخوارزمية
subsettings_images_ignore_same_size = تجاهل الصور بنفس الحجم
subsettings_images_max_difference = الفرق الأقصى
subsettings_images_duplicates_hash_type = نوع التجزئة
subsettings_duplicates_check_method = طريقة التحقق
subsettings_duplicates_name_case_sensitive = حالة حساسة (طرق الاسم فقط)
subsettings_biggest_files_sub_method = الطريقة
subsettings_biggest_files_sub_number_of_files = عدد الملفات
subsettings_videos_max_difference = الفرق الأقصى
subsettings_videos_ignore_same_size = تجاهل مقاطع الفيديو بنفس الحجم
subsettings_music_audio_check_type = نوع التحقق من الصوت
subsettings_music_approximate_comparison = مقارنة العلامات التقريبية
subsettings_music_compared_tags = مقارنة العلامات
subsettings_music_title = العنوان
subsettings_music_artist = الفنان
subsettings_music_bitrate = معدل
subsettings_music_genre = النوع
subsettings_music_year = السنة
subsettings_music_length = طول
subsettings_music_max_difference = الفرق الأقصى
subsettings_music_minimal_fragment_duration = الحد الأدنى من مدة الشظايا
subsettings_music_compare_fingerprints_only_with_similar_titles = مقارنة داخل مجموعات من العناوين المتشابهة
subsettings_broken_files_type = نوع الملفات المراد التحقق منها
subsettings_broken_files_audio = الصوت
subsettings_broken_files_pdf = بي دي إف
subsettings_broken_files_archive = أرشيف
subsettings_broken_files_image = صورة
subsettings_broken_files_video = فيديو
subsettings_broken_files_video_info = يستخدم ffmpeg/ffprobe. بطيء جداً وقد يكتشف أخطاءً تافهة حتى لو كان الملف يعمل بشكل جيد.
subsettings_bad_names_issues = فحص أسماء الملفات
subsettings_bad_names_uppercase_extension = توسيع علوي
subsettings_bad_names_uppercase_extension_hint = يجد الملفات التي تحتوي على حروف كبيرة في الامتداد (مثل .JPG، .Mp3) ويقترح النسخة الصغيرة
subsettings_bad_names_emoji_used = إيموجي في الاسم
subsettings_bad_names_emoji_used_hint = يجد الملفات التي تحتوي على أحرف تعبيرية (😀، 🎉، إلخ) في الاسم ويقترح حذفها
subsettings_bad_names_space_at_start_end = مسافات بادئة / مسافات لاحقة
subsettings_bad_names_space_at_start_end_hint = يجد الملفات التي تحتوي على مسافات في بداية أو نهاية الاسم ويقترح قصها
subsettings_bad_names_non_ascii = أحرف غير ASCII
subsettings_bad_names_non_ascii_hint = يجد أحرفًا غير ASCII (ą، ć، ñ، إلخ) ويقترح استبدالها بمرادفاتها ASCII (أ، ج، ن) أو إزالتها إذا لم يكن هناك تعيين
subsettings_bad_names_restricted_charset = مجموعة أحرف محدودة
subsettings_bad_names_restricted_charset_hint = يحول إلى ASCII الأحرف غير ASCII غير القابلة للطباعة، ثم يجد الملفات التي تحتوي على أحرف خارج 0-9أ-ي-ز و أحرف مسموح بها محددة من قبل المستخدم
subsettings_bad_names_allowed_chars = السماح بحروف
subsettings_bad_names_remove_duplicated = أحرف مكررة
subsettings_bad_names_remove_duplicated_hint = يجد الأحرف غير الحرفية المتكررة المتجاورة (مثل "ملف---اسم..txt") ويقترح إزالة التكرارات
settings_global_settings = الإعدادات العامة
settings_dark_theme = السمة المظلمة
settings_show_only_icons = إظهار الأيقونات فقط
settings_excluded_items = البند المستبعد:
settings_allowed_extensions = الإضافات المسموح بها:
settings_excluded_extensions = الإضافات المستبعدة:
settings_file_size = حجم الملف (كيلوبايتات)
settings_minimum_file_size = دقيقة:
settings_maximum_file_size = الحد الأقصى:
settings_recursive_search = البحث المتكرر
settings_use_cache = استخدام ذاكرة التخزين المؤقت
settings_save_as_json = حفظ ذاكرة التخزين المؤقت أيضا كملف JSON
settings_move_to_trash = نقل الملفات المحذوفة إلى سلة المهملات
settings_ignore_other_filesystems = تجاهل نظم الملفات الأخرى (Linux)
settings_delete_outdated_cache_entries = حذف إدخالات ذاكرة التخزين المؤقت القديمة تلقائيًا
settings_delete_outdated_cache_entries_hint = عند التفعيل، ستقوم التطبيق بالتحقق أثناء تحميل ذاكرة التخزين المؤقت (بحد أقصى مرة واحدة في الأسبوع) لمعرفة ما إذا كانت السجلات المخزنة لا تزال تشير إلى ملفات/بيانات موجودة وغير معدلة
settings_hide_hard_links = إخفاء الروابط الصلبة
settings_hide_hard_links_hint = إخفاء الروابط الصلبة للملفات نفسها في النتائج
settings_thread_number = رقم الموضوع
settings_restart_required = ---أنت بحاجة إلى إعادة تشغيل التطبيق لتطبيق التغييرات في رقم الموضوع --
settings_duplicate_image_preview = معاينة الصورة
settings_duplicate_minimal_hash_cache_size = الحجم الأدنى للملفات المخزنة مؤقتاً - هاش (KB)
settings_duplicate_use_prehash = استخدام ما قبل التجزئة
settings_duplicate_minimal_prehash_cache_size = الحجم الأدنى للملفات المخزنة مؤقتاً - بريهاش (KB)
settings_similar_images_show_image_preview = معاينة الصورة
settings_application_scale_text = تطبيق النطاق
settings_application_scale_hint_text = عند تفعيل المقياس اليدوي، يتيح لك ذلك اختيار عامل مقياس مخصص، ولكنه يعطل تمامًا التوسيع التلقائي بناءً على دقة الشاشة (DPI).
settings_restart_required_scale_text = ---يجب إعادة تشغيل التطبيق لتطبيق التغييرات في المقياس---
settings_use_manual_application_scale_text = استخدم مقياس تطبيق يدوي
settings_video_thumbnails_preview = معاينة الصورة
settings_open_config_folder = فتح مجلد التكوين
settings_open_cache_folder = فتح مجلد ذاكرة التخزين المؤقت
settings_language = اللغة
settings_current_preset = المسبق الحالي:
settings_edit_name = تحرير الاسم
settings_choose_name_for_prefix = اختر اسم البادئة
settings_save = حفظ
settings_load = تحميل
settings_reset = إعادة تعيين
settings_similar_videos_tool = أداة فيديو مشابهة
settings_video_thumbnails_clear_unused_thumbnails = حذف صورthumbnails للفيديو غير المستخدمة التي يزيد عمرها عن 7 أيام عند بدء تشغيل التطبيق
settings_video_thumbnails_header = صورة مصغرة للفيديو
settings_video_thumbnails_generate = إنشاء صور مصغرة
settings_video_thumbnails_position = موضع الصورة المصغرة في الفيديو (%)
settings_video_thumbnails_generate_grid = إنشاء شبكة صور مصغرة بدلاً من صورة واحدة
settings_video_thumbnails_generate_grid_hint = إن إنشاء صور متعددة في شبكة أبطأ بكثير من إنشاء صورة مصغرة واحدة
settings_video_thumbnails_grid_tiles_per_side = عدد البلاطات في كل جانب في شبكة الصورة المصغرة
settings_video_thumbnails_grid_tiles_per_side_hint = عدد مربعات الصور المصغرة في كل جانب من الشبكة. على سبيل المثال، تحديد 2 ينشئ شبكة 2 × 2، مما ينتج عنه صورة مصغرة واحدة تتكون من 4 صور.
settings_similar_images_tool = أداة مشابهة للصور
settings_general_settings = الإعدادات العامة
settings_cache_header_text = إعدادات التخزين المؤقت
settings_clean_cache_button_text = امسح ذاكرة التخزين المؤقت القديمة
settings_settings = الإعدادات
settings_load_tabs_sizes_at_startup = تحميل أحجام علامات التبويب عند بدء التشغيل
settings_load_windows_size_at_startup = تحميل حجم النوافذ عند بدء التشغيل
settings_limit_lines_of_messages = قصر الرسائل على 500 سطر (العمل على أداة تحرير نص بطيئ)
settings_play_audio_on_scan_completion_text = تشغيل الصوت عند اكتمال المسح بنجاح
settings_audio_feature_hint_text = متاح فقط عند التجميع مع الميزة الصوتية
settings_audio_env_variable_hint_text = يمكن تغيير الصوت عن طريق تعيين متغير البيئة KROKIET_AUDIO_STOP_FILE إلى مسار ملف صوتي صالح
popup_save_title = حفظ النتائج
popup_save_message = سيؤدي هذا إلى حفظ النتائج إلى 3 ملفات مختلفة
popup_rename_title = إعادة تسمية الملفات
popup_new_paths_title = أضف مسارات سطرًا واحدًا لكل سطر
popup_move_title = نقل الملفات
popup_move_copy_checkbox = نسخ الملفات بدلاً من النقل
popup_move_preserve_folder_checkbox = الحفاظ على هيكل المجلد
move_confirmation_text = هل أنت متأكد من أنك تريد نقل العناصر المحددة؟
rename_confirmation_text = هل أنت متأكد من أنك تريد إعادة تسمية العناصر المحددة؟
delete = حذف العناصر
stopping_scan = إيقاف المسح، الرجاء الانتظار...
searching = يبحث...
subsettings_videos_crop_detect = طريقة الكشف عن المحاصيل
subsettings_videos_skip_forward_amount = تخطي المدة [s]
subsettings_videos_vid_hash_duration = مدة تجزئة الفيديو
settings_cache_number_size_text = حجم ملفات التخزين المؤقت: { $size }، عدد الملفات: { $number }
settings_video_thumbnails_number_size_text = حجم الصور المصغرة للفيديو: { $size }، عدد الملفات: { $number }
settings_log_number_size_text = حجم ملفات السجل: { $size }، عدد الملفات: { $number }
popup_clean_cache_title_text = مسح ذاكرة التخزين المؤقت القديمة
popup_clean_cache_confirmation_text = هل أنت متأكد من أنك تريد مسح إدخالات ذاكرة التخزين المؤقت القديمة؟ سيؤدي ذلك إلى إزالة إدخالات ذاكرة التخزين المؤقت للملفات التي لم تعد موجودة أو تم تعديلها.
popup_clean_cache_progress_text = جاري معالجة ملف ذاكرة التخزين المؤقت:
popup_clean_cache_current_file_text = الملف الحالي:
popup_clean_cache_file_progress_text = التقدم الحالي للملف:
popup_clean_cache_overall_progress_text = التقدم العام:
popup_clean_cache_stopped_by_user_text = تم إيقاف تنظيف ذاكرة التخزين المؤقت بواسطة المستخدم
popup_clean_cache_finished_text = تم تنظيف ذاكرة التخزين المؤقت بنجاح!
popup_clean_cache_error_details_text = تفاصيل الخطأ:
popup_clean_cache_files_with_errors = ملفات بها أخطاء:
subsettings_video_optimizer_mode = وضع
subsettings_video_optimizer_crop_type = نوع المحصول
subsettings_video_optimizer_black_pixel_threshold = حد\_السطوع\_الأسود
subsettings_video_optimizer_black_pixel_threshold_hint = القيمة القصوى لـ RGB لكل قناة بكسل لاعتبارها سوداء (0-128). القيمة الافتراضية: 20
subsettings_video_optimizer_black_bar_min_percentage = شريط أسود الحد الأدنى للنسبة المئوية
subsettings_video_optimizer_black_bar_min_percentage_hint = الحد الأدنى لنسبة بكسلات سوداء في صف/عمود لاعتبارها شريطًا أسود (50-100). القيمة الافتراضية: 90
subsettings_video_optimizer_max_samples = أقصى عينات
subsettings_video_optimizer_max_samples_hint = الحد الأقصى لعدد الإطارات لتحليلها لكل فيديو (5-1000). القيمة الافتراضية: 60
subsettings_video_optimizer_min_crop_size = من Crop Size
subsettings_video_optimizer_min_crop_size_hint = الحد الأدنى لعدد وحدات البكسل التي يتم القص فيها على أي جانب (1-1000). يتم تجاهل القصص الأصغر. القيمة الافتراضية: 5
subsettings_video_optimizer_video_codec = فيديو كودك
subsettings_video_optimizer_excluded_codecs = محذوفات الترميز
subsettings_video_optimizer_video_quality = جودة الفيديو (CRF)
subsettings_reset = إعادة تعيين
subsettings_exif_ignored_tags_text = تجاهل العلامات:
subsettings_exif_ignored_tags_hint_text = قائمة مفرغة بفواصل من العلامات المستبعدة من الفحص (مثل GPS، Thumbnail). بعض العلامات، مثل ImageWidth في ملفات TIFF، مخفية لمنع كسر الصورة.
clean_button_text = نظيف
clean_text = بيانات EXIF ​​النظيفة
clean_confirmation_text = هل أنت متأكد من أنك تريد إزالة بيانات EXIF من العناصر المحددة؟
crop_videos_text = قص الفيديو
crop_video_confirmation_text = هل أنت متأكد من أنك تريد اقتطاف الفيديوهات المحددة؟
crop_reencode_video_text = إعادة ترميز الفيديو
reencode_videos_text = إعادة ترميز الفيديوهات
optimize_button_text = التحسين
optimize_confirmation_text = هل أنت متأكد من أنك تريد إعادة ترميز الفيديوهات المحددة؟
optimize_fail_if_bigger_text = فشل إذا كان الملف المحسن أكبر
optimize_overwrite_files_text = استبدل الملفات
optimize_limit_video_size_text = حدّ حجم الفيديو
optimize_max_width_text = الحد الأقصى للعرض:
optimize_max_height_text = الحد الأقصى للارتفاع:
hardlink_button_text = رابط صلب
hardlink_text = إنشاء روابط صلبة
hardlink_confirmation_text = هل أنت متأكد من أنك تريد إنشاء روابط صلبة للعناصر المحددة؟
softlink_button_text = سولت لينك
softlink_text = إنشاء روابط رمزية
softlink_confirmation_text = هل أنت متأكد من أنك تريد إنشاء روابط رمزية (symlinks) للعناصر المحددة؟
