# Window titles
window_settings_title = الإعدادات
window_main_title = Czkawka
window_progress_title = المسح
window_compare_images = مقارنة الصور
# General
general_ok_button = حسناً
general_close_button = أغلق
# Main window
music_title_checkbox = العنوان
music_artist_checkbox = الفنان
music_year_checkbox = السنة
music_bitrate_checkbox = معدل
music_genre_checkbox = النوع
music_length_checkbox = طول
music_comparison_checkbox = مقارنة تقريبية
music_checking_by_tags = الوسوم
music_checking_by_content = محتوى
same_music_seconds_label = الحد الأدنى من مدة التجزئة الثانية
same_music_similarity_label = الفرق الأقصى
music_compare_only_in_title_group = قارن فقط في العنوان
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    يمكن تكوين البحث عن ملفات موسيقية مشابهة بواسطة محتواها عن طريق الإعداد:
    
    - الحد الأدنى لوقت الشظايا الذي يمكن بعدها تحديد ملفات الموسيقى على أنها
    - الحد الأقصى للفارق بين جزأين تم اختبارهما
    
    والمفتاح إلى النتائج الجيدة هو العثور على مجموعات معقولة من هذه المعلمات، عن تقديمه.
    
    تحديد الحد الأدنى من الوقت إلى 5 ثوان والحد الأقصى للفرق إلى 1.0، سيبحث عن أجزاء متطابقة تقريبا في الملفات.
    وقت 20 ثانية وفارق أقصى قدره 6.0، من ناحية أخرى، يعمل بشكل جيد من أجل العثور على تعديلات أو إصدارات حية وما إلى ذلك.
    
    بشكل افتراضي، يتم مقارنة كل ملف موسيقي بآخر وقد يستغرق ذلك الكثير من الوقت عند اختبار العديد من الملفات، لذلك من الأفضل عادة استخدام المجلدات المرجعية وتحديد الملفات التي يجب مقارنتها مع بعضها البعض (مع نفس كمية الملفات)، مقارنة بصمات الأصابع ستكون أسرع من 4 × على الأقل من دون مجلدات مرجعية).
music_comparison_checkbox_tooltip =
    يبحث عن ملفات موسيقية مماثلة باستخدام AI، الذي يستخدم تعلم الآلة لإزالة الأقواس من العبارة. على سبيل المثال، مع تمكين هذا الخيار، الملفات المعنية سوف تعتبر متكررة:
    
    S<unk> wie<unk> dziz<unk> L.o<unk> b --- S<unk> wie<unk> dziz<unk> L.o<unk> b (Remix Lato 2021)
duplicate_case_sensitive_name = حالة حساسة
duplicate_case_sensitive_name_tooltip =
    When enabled, group only records when they have exactly same name e.g. Żołd <-> Żołd
    
    Disabling such option will group names without checking if each letter is same size e.g. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = الحجم والاسم
duplicate_mode_name_combo_box = الاسم
duplicate_mode_size_combo_box = الحجم
duplicate_mode_hash_combo_box = التجزئة
duplicate_hash_type_tooltip =
    يقدم Czkawka 3 أنواع من التجزئة:
    
    Blake3 - دالة التجزئة المشفرة. هذا هو الافتراضي لأنه سريع جدا.
    
    CRC32 - دالة التجزئة البسيطة. وينبغي أن يكون هذا أسرع من بليك 3، ولكن نادرا ما تحدث بعض الاصطدام.
    
    XXH3 - مشابهة جدا في الأداء وجودة التجزئة للـ Blake3 (ولكن غير مشفرة). لذلك يمكن بسهولة تبادل مثل هذه الأوضاع.
duplicate_check_method_tooltip =
    في الوقت الحالي، تقدم Czkawka ثلاثة أنواع من الطرق للعثور على التكرارات:
    
    Name - Finds الملفات التي تحمل نفس الاسم.
    
    الحجم - العثور على الملفات التي لها نفس الحجم.
    
    Hash - العثور على الملفات التي لها نفس المحتوى. هذا الوضع يقوم بتجزئة الملف ثم يقارن هذا التجزئة للعثور على التكرار. هذا الوضع هو أكثر الطرق أماناً للعثور على التكرار. يستخدم التطبيق بكثافة ذاكرة التخزين المؤقت، لذا يجب أن تكون المسح الثاني والمزيد لنفس البيانات أسرع بكثير من الأول.
image_hash_size_tooltip =
    كل صورة تم فحصها تنتج تجزئة خاصة يمكن مقارنتها مع بعضها البعض، والاختلاف الصغير بينهما يعني أن هذه الصور متشابهة.
    
    8 حجم التجزئة جيد جدا للعثور على صور تشبه قليلا فقط الصور الأصلية. مع مجموعة أكبر من الصور (>1000)، هذا سوف ينتج كمية كبيرة من الإيجابيات الكاذبة، لذا أوصي باستخدام حجم تجزئة أكبر في هذه الحالة.
    
    16 هو حجم التجزئة الافتراضي الذي يمثل حلاً وسطاً جيداً بين العثور على صور مشابهة قليلاً فقط وبين حدوث عدد صغير من تصادم التجزئة.
    
    32 و64 تجزئة لا تجد سوى صور مشابهة جداً، ولكن ينبغي ألا يكون لها تقريباً إيجابيات كاذبة (ربما باستثناء بعض الصور مع قناة ألفا).
image_resize_filter_tooltip =
    To compute hash of image, the library must first resize it.
    
    Depend on chosen algorithm, the resulting image used to calculate hash will looks a little different.
    
    The fastest algorithm to use, but also the one which gives the worst results, is Nearest. It is enabled by default, because with 16x16 hash size lower quality it is not really visible.
    
    With 8x8 hash size it is recommended to use a different algorithm than Nearest, to have better groups of images.
image_hash_alg_tooltip =
    يمكن للمستخدمين الاختيار من واحدة من خوارزميات عديدة لحساب التجزئة.
    
    لكل منها نقاط قوية وأضعف وسوف تعطي أحيانا نتائج أفضل وأحيانا أسوأ لصور مختلفة.
    
    لذلك ، لتحديد أفضل واحد لك، يتطلب الاختبار اليدوي.
big_files_mode_combobox_tooltip = يسمح بالبحث عن ملفات أصغر/أكبر
big_files_mode_label = الملفات المحددة
big_files_mode_smallest_combo_box = الأصغر حجماً
big_files_mode_biggest_combo_box = الاكبر
main_notebook_duplicates = الملفات المكررة
main_notebook_empty_directories = دلائل فارغة
main_notebook_big_files = الملفات الكبيرة
main_notebook_empty_files = الملفات الفارغة
main_notebook_temporary = ملفات مؤقتة
main_notebook_similar_images = صور مشابهة
main_notebook_similar_videos = مقاطع فيديو مماثلة
main_notebook_same_music = مكرر الموسيقى
main_notebook_symlinks = الروابط الرمزية غير صالحة
main_notebook_broken_files = الملفات المكسورة
main_notebook_bad_extensions = ملحقات سيئة
main_tree_view_column_file_name = اسم الملف
main_tree_view_column_folder_name = اسم المجلد
main_tree_view_column_path = المسار
main_tree_view_column_modification = تاريخ التعديل
main_tree_view_column_size = الحجم
main_tree_view_column_similarity = تماثل
main_tree_view_column_dimensions = الأبعاد
main_tree_view_column_title = العنوان
main_tree_view_column_artist = الفنان
main_tree_view_column_year = السنة
main_tree_view_column_bitrate = معدل
main_tree_view_column_length = طول
main_tree_view_column_genre = النوع
main_tree_view_column_symlink_file_name = اسم ملف الرابط الرمزي
main_tree_view_column_symlink_folder = مجلد الرابط الرمزي
main_tree_view_column_destination_path = مسار الوجهة
main_tree_view_column_type_of_error = نوع الخطأ
main_tree_view_column_current_extension = التمديد الحالي
main_tree_view_column_proper_extensions = التمديد الصحيح
main_label_check_method = طريقة التحقق
main_label_hash_type = نوع التجزئة
main_label_hash_size = حجم التجزئة
main_label_size_bytes = الحجم (بايت)
main_label_min_size = الحد الأدنى
main_label_max_size = الحد الأقصى
main_label_shown_files = عدد الملفات المعروضة
main_label_resize_algorithm = تغيير حجم الخوارزمية
main_label_similarity = مشابهة{" " }
main_check_box_broken_files_audio = الصوت
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = أرشيف
main_check_box_broken_files_image = صورة
check_button_general_same_size = تجاهل نفس الحجم
check_button_general_same_size_tooltip = تجاهل الملفات ذات الحجم المتطابق في النتائج - عادة ما تكون هذه المكررة 1:1
main_label_size_bytes_tooltip = حجم الملفات التي سيتم استخدامها في المسح
# Upper window
upper_tree_view_included_folder_column_title = مجلدات للبحث
upper_tree_view_included_reference_column_title = المجلدات المرجعية
upper_recursive_button = متكرر
upper_recursive_button_tooltip = إذا تم تحديده، ابحث أيضا عن الملفات التي لم توضع مباشرة تحت المجلدات المختارة.
upper_manual_add_included_button = إضافة يدوي
upper_add_included_button = إضافة
upper_remove_included_button = إزالة
upper_manual_add_excluded_button = إضافة يدوي
upper_add_excluded_button = إضافة
upper_remove_excluded_button = إزالة
upper_manual_add_included_button_tooltip =
    إضافة اسم الدليل للبحث باليد.
    
    لإضافة مسارات متعددة في وقت واحد، قم بفصلها بواسطة ؛
    
    /home/rozkaz سيضيف دليلين /home/rozkaz و /home/rozkaz
upper_add_included_button_tooltip = إضافة دليل جديد للبحث.
upper_remove_included_button_tooltip = حذف الدليل من البحث.
upper_manual_add_excluded_button_tooltip =
    إضافة اسم الدليل المستبعد يدوياً.
    
    لإضافة مسارات متعددة في وقت واحد، قم بفصلها بواسطة ؛
    
    /home/roman;/home/krokiet سيضيف دليلين / home/roman و /home/keokiet
upper_add_excluded_button_tooltip = إضافة دليل ليتم استبعاده في البحث.
upper_remove_excluded_button_tooltip = حذف الدليل من المستبعد.
upper_notebook_items_configuration = تكوين العناصر
upper_notebook_excluded_directories = المجلدات المستبعدة
upper_notebook_included_directories = المجلدات المضمنة
upper_allowed_extensions_tooltip =
    يجب أن تكون الملحقات المسموح بها مفصولة بفواصل (بشكل افتراضي كلها متاحة).
    
    أجهزة الماكرو التالية، التي تضيف ملحقات متعددة في وقت واحد، متاحة أيضا: IMAGE، VIDEO، MUSIC، TEXT.
    
    مثال استخدام ".exe, IMAGE, VIDEO, .rar, 7z" - وهذا يعني أن الصور (e. .jpg, png) الفيديوهات (مثلاً: avi, mp4) و ex, rar و 7z سيتم مسح الملفات.
upper_excluded_extensions_tooltip =
    قائمة الملفات المعطلة التي سيتم تجاهلها في المسح.
    
    عند استخدام الملحقات المسموح بها والمعطلة على حد سواء، هذه واحدة لها أولوية أعلى، لذلك لن يتم تحديد الملف.
upper_excluded_items_tooltip =
    Excluded items must contain * wildcard and should be separated by commas.
    This is slower than Excluded Directories, so use it carefully.
upper_excluded_items = البنود المستثناة:
upper_allowed_extensions = الإضافات المسموح بها:
upper_excluded_extensions = الملحقات المعطّلة:
# Popovers
popover_select_all = حدد الكل
popover_unselect_all = إلغاء تحديد الكل
popover_reverse = الاختيار العكسي
popover_select_all_except_oldest = حدد الكل باستثناء الأقدم
popover_select_all_except_newest = حدد الكل باستثناء الأحدث
popover_select_one_oldest = حدد أقدم واحد
popover_select_one_newest = حدد واحد أحدث
popover_select_custom = تحديد مخصص
popover_unselect_custom = إلغاء تحديد مخصص
popover_select_all_images_except_biggest = حدد الكل باستثناء أكبر
popover_select_all_images_except_smallest = حدد الكل باستثناء الأصغر
popover_custom_path_check_button_entry_tooltip =
    Select records by path.
    
    Example usage:
    /home/pimpek/rzecz.txt can be found with /home/pim*
popover_custom_name_check_button_entry_tooltip =
    حدد السجلات حسب أسماء الملفات.
    
    استخدام مثال:
    /usr/ping/pong.txt يمكن العثور عليه مع *ong*
popover_custom_regex_check_button_entry_tooltip =
    حدد السجلات بواسطة Regex.
    
    مع هذا الوضع، النص الذي تم البحث عنه هو المسار بالاسم.
    
    مثال الاستخدام:
    /usr/bin/ziemniak. يمكن العثور على xt مع /ziem[a-z]+
    
    يستخدم هذا التطبيق الافتراضي Rust regex . يمكنك قراءة المزيد عنه هنا: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    تمكين الكشف الحساس لحالة الأحرف.
    
    عند تعطيل / المنزل/* يجد كلا من /HoMe/roman و /home/roman.
popover_custom_not_all_check_button_tooltip =
    Prevents selecting all records in group.
    
    This is enabled by default, because in most situations, you don't want to delete both original and duplicates files, but want to leave at least one file.
    
    WARNING: This setting doesn't work if you have already manually selected all results in a group.
popover_custom_regex_path_label = المسار
popover_custom_regex_name_label = الاسم
popover_custom_regex_regex_label = مسار Regex + اسم
popover_custom_case_sensitive_check_button = حساسية الحالة
popover_custom_all_in_group_label = عدم تحديد جميع السجلات في المجموعة
popover_custom_mode_unselect = إلغاء تحديد مخصص
popover_custom_mode_select = تحديد مخصص
popover_sort_file_name = اسم الملف
popover_sort_folder_name = اسم المجلد
popover_sort_full_name = الاسم الكامل
popover_sort_size = الحجم
popover_sort_selection = التحديد
popover_invalid_regex = Regex غير صحيح
popover_valid_regex = Regex صالح
# Bottom buttons
bottom_search_button = البحث
bottom_select_button = حدد
bottom_delete_button = حذف
bottom_save_button = حفظ
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = نقل
bottom_sort_button = فرز
bottom_search_button_tooltip = بدء البحث
bottom_select_button_tooltip = حدد السجلات. يمكن معالجة الملفات/المجلدات المحددة في وقت لاحق.
bottom_delete_button_tooltip = حذف الملفات/المجلدات المحددة.
bottom_save_button_tooltip = حفظ البيانات حول البحث في الملف
bottom_symlink_button_tooltip =
    إنشاء روابط رمزية.
    يعمل فقط عندما يتم تحديد نتيجتين على الأقل في المجموعة.
    أولا لم يتغير و الثاني و اللاحق مرتبطين بالأول.
bottom_hardlink_button_tooltip =
    إنشاء روابط صلبة.
    يعمل فقط عندما يتم تحديد نتيجتين على الأقل في المجموعة.
    أولا لم يتغير و الثاني و اللاحق متصلين بالأول.
bottom_hardlink_button_not_available_tooltip =
    Create hardlinks.
    Button is disabled, because hardlinks cannot be created.
    Hardlinks only works with administrator privileges on Windows, so be sure to run app as administrator.
    If app already works with such privileges check for similar issues on Github.
bottom_move_button_tooltip =
    ينقل الملفات إلى الدليل المختار.
    ينسخ جميع الملفات إلى الدليل دون الحفاظ على شجرة الدليل.
    عند محاولة نقل ملفين مع نفس الاسم إلى مجلد، سيتم فشل الثانية وإظهار الخطأ.
bottom_sort_button_tooltip = ترتيب الملفات/المجلدات وفقا للطريقة المحددة.
bottom_show_errors_tooltip = إظهار/إخفاء لوحة النص السفلية.
bottom_show_upper_notebook_tooltip = إظهار/إخفاء لوحة دفتر الملاحظات العلوية.
# Progress Window
progress_stop_button = توقف
progress_stop_additional_message = إيقاف الطلب
# About Window
about_repository_button_tooltip = رابط لصفحة المستودع مع رمز المصدر.
about_donation_button_tooltip = رابط لصفحة التبرع.
about_instruction_button_tooltip = رابط لصفحة التعليمات.
about_translation_button_tooltip = رابط إلى صفحة كراودِن مع ترجمة التطبيق. يتم دعم البولندية الرسمية والإنجليزية.
about_repository_button = المستودع
about_donation_button = تبرع
about_instruction_button = تعليمات
about_translation_button = الترجمة
# Header
header_setting_button_tooltip = فتح مربع حوار الإعدادات.
header_about_button_tooltip = فتح مربع الحوار مع معلومات حول التطبيق.

# Settings


## General

settings_number_of_threads = عدد المواضيع المستخدمة
settings_number_of_threads_tooltip = عدد المواضيع المستخدمة، 0 يعني أن جميع المواضيع المتاحة سيتم استخدامها.
settings_use_rust_preview = استخدام المكتبات الخارجية بدلاً من gtk لتحميل المعاينات
settings_use_rust_preview_tooltip =
    وفي بعض الأحيان سيكون استخدام معاينات gtk أسرع ويدعم صيغا أكثر، ولكن في بعض الأحيان قد يكون الأمر على العكس تماما.
    
    إذا كان لديك مشاكل في تحميل المعاينات، فيمكنك محاولة تغيير هذا الإعداد.
    
    على أنظمة غير لينوكس، يوصى باستخدام هذا الخيار، لأن gtk-pixbuf غير متوفر دائمًا هناك لذلك فإن تعطيل هذا الخيار لن يقوم بتحميل المعاينات لبعض الصور.
settings_label_restart = تحتاج إلى إعادة تشغيل التطبيق لتطبيق الإعدادات!
settings_ignore_other_filesystems = تجاهل نظم الملفات الأخرى (Linux)
settings_ignore_other_filesystems_tooltip =
    يتجاهل الملفات التي ليست في نفس نظام الملفات مثل الدلائل التي تم بحثها. يعمل
    
    مثل خيار -xdev في العثور على أمر على Linux
settings_save_at_exit_button_tooltip = حفظ التكوين إلى الملف عند إغلاق التطبيق.
settings_load_at_start_button_tooltip =
    تحميل التكوين من الملف عند فتح التطبيق.
    
    إذا لم يتم تمكينه، سيتم استخدام الإعدادات الافتراضية.
settings_confirm_deletion_button_tooltip = إظهار مربع حوار التأكيد عند النقر على زر الحذف.
settings_confirm_link_button_tooltip = إظهار مربع حوار التأكيد عند النقر على زر الارتباط الصلب/الرمزي.
settings_confirm_group_deletion_button_tooltip = إظهار مربع حوار التحذير عند محاولة حذف جميع السجلات من المجموعة.
settings_show_text_view_button_tooltip = إظهار لوحة النص في أسفل واجهة المستخدم.
settings_use_cache_button_tooltip = استخدام ذاكرة التخزين المؤقت للملف.
settings_save_also_as_json_button_tooltip = حفظ ذاكرة التخزين المؤقت إلى تنسيق JSON (قابل للقراءة البشرية). من الممكن تعديل محتواه. الذاكرة المؤقتة من هذا الملف سيتم قراءتها تلقائيًا بواسطة التطبيق إذا كان مخبأ تنسيق ثنائي (مع امتداد بن ) مفقود.
settings_use_trash_button_tooltip = نقل الملفات إلى سلة المهملات بدلاً من حذفها بشكل دائم.
settings_language_label_tooltip = لغة واجهة المستخدم.
settings_save_at_exit_button = حفظ التكوين عند إغلاق التطبيق
settings_load_at_start_button = تحميل التكوين عند فتح التطبيق
settings_confirm_deletion_button = إظهار تأكيد مربع الحوار عند حذف أي ملفات
settings_confirm_link_button = إظهار مربع حوار تأكيد عند ربط أي ملفات بصعوبة/رموز
settings_confirm_group_deletion_button = إظهار تأكيد مربع الحوار عند حذف جميع الملفات في المجموعة
settings_show_text_view_button = إظهار لوحة النص السفلي
settings_use_cache_button = استخدام ذاكرة التخزين المؤقت
settings_save_also_as_json_button = حفظ ذاكرة التخزين المؤقت أيضا كملف JSON
settings_use_trash_button = نقل الملفات المحذوفة إلى سلة المهملات
settings_language_label = اللغة
settings_multiple_delete_outdated_cache_checkbutton = حذف إدخالات ذاكرة التخزين المؤقت القديمة تلقائياً
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Delete outdated cache results which point to non-existent files.
    
    When enabled, app makes sure when loading records, that all records point to valid files (broken ones are ignored).
    
    Disabling this will help when scanning files on external drives, so cache entries about them will not be purged in the next scan.
    
    In the case of having hundred of thousands records in cache, it is suggested to enable this, which will speedup cache loading/saving at start/end of the scan.
settings_notebook_general = عمومي
settings_notebook_duplicates = مكرر
settings_notebook_images = صور مشابهة
settings_notebook_videos = فيديو مشابه

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = عرض المعاينة على الجانب الأيمن (عند تحديد ملف صورة).
settings_multiple_image_preview_checkbutton = عرض معاينة الصورة
settings_multiple_clear_cache_button_tooltip =
    Manually clear the cache of outdated entries.
    This should only be used if automatic clearing has been disabled.
settings_multiple_clear_cache_button = إزالة النتائج القديمة من ذاكرة التخزين المؤقت.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Hides all files except one, if all point to the same data (are hardlinked).
    
    Example: In the case where there are (on disk) seven files which are hardlinked to specific data and one different file with same data but a different inode, then in duplicate finder, only one unique file and one file from hardlinked ones will be shown.
settings_duplicates_minimal_size_entry_tooltip =
    Set the minimal file size which will be cached.
    
    Choosing a smaller value will generate more records. This will speedup search, but slowdown cache loading/saving.
settings_duplicates_prehash_checkbutton_tooltip =
    تمكين التخزين المؤقت للتجزئة (تجزئة محسوبة من جزء صغير من الملف) مما يسمح برفض النتائج غير المكررة في وقت سابق.
    
    يتم تعطيله بشكل افتراضي لأنه يمكن أن يتسبب في تباطؤ في بعض الحالات.
    
    يوصى بشدة باستخدامها عند مسح مئات الألوف أو الملايين من الملفات، لأنه يمكن تسريع البحث عدة مرات.
settings_duplicates_prehash_minimal_entry_tooltip = الحجم الأدنى للإدخال المخبئ.
settings_duplicates_hide_hard_link_button = إخفاء الروابط الصلبة (فقط Linux و macOS)
settings_duplicates_prehash_checkbutton = استخدام ذاكرة التخزين المؤقت
settings_duplicates_minimal_size_cache_label = الحجم الأدنى للملفات (بالبايت) المحفوظة إلى ذاكرة التخزين المؤقت
settings_duplicates_minimal_size_cache_prehash_label = الحجم الأدنى للملفات (بالبايت) المحفوظة في ذاكرة التخزين المؤقت

## Saving/Loading settings

settings_saving_button_tooltip = حفظ الإعدادات الحالية إلى الملف.
settings_loading_button_tooltip = تحميل الإعدادات من الملف واستبدل الإعدادات الحالية بها.
settings_reset_button_tooltip = إعادة تعيين الإعدادات الحالية إلى الإعدادات الافتراضية.
settings_saving_button = حفظ التكوين
settings_loading_button = تحميل التكوين
settings_reset_button = إعادة ضبط الإعدادات

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Opens the folder where the cache txt files are stored.
    
    Modifying the cache files may cause invalid results to be shown. However, modifying path may save time when moving a big amount of files to a different location.
    
    You can copy these files between computers to save time on scanning again for files (of course if they have similar directory structure).
    
    In the case of problems with the cache, these files can be removed. The app will automatically regenerate them.
settings_folder_settings_open_tooltip =
    Opens the folder where the Czkawka config is stored.
    
    WARNING: Manually modifying the config may break your workflow.
settings_folder_cache_open = فتح مجلد التخزين المؤقت
settings_folder_settings_open = فتح مجلد الإعدادات
# Compute results
compute_stopped_by_user = تم إيقاف البحث من قبل المستخدم
compute_found_duplicates_hash_size = تم العثور على { $number_files } مكررة في { $number_groups } مجموعات أخذت { $size }
compute_found_duplicates_name = تم العثور على { $number_files } مكررة في { $number_groups } مجموعات
compute_found_empty_folders = تم العثور على { $number_files } مجلدات فارغة
compute_found_empty_files = تم العثور على { $number_files } ملفات فارغة
compute_found_big_files = تم العثور على { $number_files } ملف كبير
compute_found_temporary_files = تم العثور على { $number_files } ملف مؤقت
compute_found_images = تم العثور على { $number_files } صورة مشابهة في { $number_groups } مجموعات
compute_found_videos = تم العثور على { $number_files } مقاطع فيديو مماثلة في { $number_groups } مجموعات
compute_found_music = تم العثور على { $number_files } ملفات موسيقية مماثلة في { $number_groups } مجموعة
compute_found_invalid_symlinks = تم العثور على { $number_files } روابط رموزية غير صالحة
compute_found_broken_files = تم العثور على { $number_files } ملفات محطمة
compute_found_bad_extensions = تم العثور على { $number_files } ملفات ذات ملحقات غير صالحة
# Progress window
progress_scanning_general_file = فحص ملف { $file_number }
progress_scanning_extension_of_files = التحقق من امتداد الملف { $file_checked }/{ $all_files }
progress_scanning_broken_files = فحص { $file_checked }/{ $all_files } ملف
progress_scanning_video = تجزئة الفيديو { $file_checked }/{ $all_files }
progress_scanning_image = تجزئة الصورة { $file_checked }/{ $all_files }
progress_comparing_image_hashes = مقارنة { $file_checked }/{ $all_files } تجزئة الصور
progress_scanning_music_tags_end = مقارنة علامات { $file_checked }/{ $all_files } ملف الموسيقى
progress_scanning_music_tags = قراءة وسوم { $file_checked }/{ $all_files } ملف الموسيقى
progress_scanning_music_content_end = مقارنة بصمة الإصبع { $file_checked }/{ $all_files } ملف الموسيقى
progress_scanning_music_content = حساب بصمة الإصبع { $file_checked }/{ $all_files } ملف الموسيقى
progress_scanning_empty_folders = يتم فحص { $folder_number } مجلد
progress_scanning_size = حجم البحث لـ { $file_number } ملف
progress_scanning_size_name = فحص اسم وحجم ملف { $file_number }
progress_scanning_name = فحص اسم ملف { $file_number }
progress_analyzed_partial_hash = تم تحليل تجزئة جزئية ل { $file_checked }/{ $all_files } ملفات
progress_analyzed_full_hash = تم تحليل التجزئة الكاملة لـ { $file_checked }/{ $all_files } ملفات
progress_prehash_cache_loading = تحميل ذاكرة التخزين المؤقت
progress_prehash_cache_saving = حفظ ذاكرة التخزين المؤقت
progress_hash_cache_loading = تحميل ذاكرة التخزين المؤقت للتجزئة
progress_hash_cache_saving = حفظ ذاكرة التخزين المؤقت
progress_cache_loading = تحميل ذاكرة التخزين المؤقت
progress_cache_saving = حفظ ذاكرة التخزين المؤقت
progress_current_stage = المرحلة الحالية:{ "" }
progress_all_stages = جميع المراحل:{ " " }
# Saving loading 
saving_loading_saving_success = حفظ التكوين إلى ملف { $name }.
saving_loading_saving_failure = فشل حفظ بيانات التكوين في الملف { $name }.
saving_loading_reset_configuration = تم مسح التكوين الحالي.
saving_loading_loading_success = تم تحميل إعدادات التطبيق بشكل صحيح.
saving_loading_invalid_string = للمفتاح "{ $key }" وجد نتيجة غير صالحة - "{ $result }" الذي ليس سلسلة.
saving_loading_invalid_int = للمفتاح "{ $key }" وجد نتيجة غير صالحة - "{ $result }" الذي ليس عددا صحيحا.
saving_loading_invalid_bool = للمفتاح "{ $key }" وجد نتيجة غير صالحة - "{ $result }" الذي ليس برعيا.
saving_loading_decode_problem_bool = فشل في فك شفرة البول من المفتاح"{ $key }" وجد{ $result }" ولكن القيم المسموح بها هي 0، 1، صحيحة أو خاطئة.
saving_loading_saving_same_keys = محاولة حفظ الإعداد مع مفتاح مكرر"{ $key }".
saving_loading_failed_to_get_home_directory = فشل في الحصول على الدليل الرئيسي لفتح/حفظ ملف الإعداد.
saving_loading_folder_config_instead_file = لا يمكن إنشاء أو فتح ملف التكوين في المسار "{ $path }" لأنه يوجد مجلد بالفعل.
saving_loading_failed_to_create_configuration_folder = فشل تكوين إنشاء مجلد الإعدادات"{ $path }"، السبب"{ $reason }".
saving_loading_failed_to_create_config_file = فشل في إنشاء ملف الإعداد"{ $path }"، السبب"{ $reason }".
saving_loading_failed_to_read_config_file = لا يمكن تحميل التكوين من "{ $path }" لأنه غير موجود أو ليس ملفا.
saving_loading_failed_to_read_data_from_file = لا يمكن قراءة البيانات من الملف"{ $path }"، السبب"{ $reason }".
saving_loading_orphan_data = تم العثور على بيانات يتيمة"{ $data }" في السطر "{ $line }".
saving_loading_not_valid = الإعداد "{ $data }" غير موجود في الإصدار الحالي للتطبيق.
# Invalid symlinks
invalid_symlink_infinite_recursion = التكرار اللامتناهي
invalid_symlink_non_existent_destination = ملف الوجهة غير موجود
# Other
selected_all_reference_folders = لا يمكن بدء البحث، عندما يتم تعيين جميع الدلائل كمجلدات مرجعية
searching_for_data = البحث عن البيانات، قد يستغرق بعض الوقت، يرجى الانتظار...
text_view_messages = الرسائل
text_view_warnings = التحذيرات
text_view_errors = أخطاء
about_window_motto = هذا البرنامج حر في الاستخدام وسوف يكون دائماً.
# Various dialog
dialogs_ask_next_time = اسأل المرة القادمة
delete_file_failed = فشل في حذف الملف { $name }، السبب { $reason }
delete_title_dialog = تأكيد حذف
delete_question_label = هل أنت متأكد من أنك تريد حذف الملفات؟
delete_all_files_in_group_title = تأكيد حذف جميع الملفات في المجموعة
delete_all_files_in_group_label1 = ويتم اختيار جميع السجلات في بعض المجموعات.
delete_all_files_in_group_label2 = هل أنت متأكد من أنك تريد حذفهم؟
delete_folder_failed = فشل في حذف المجلد { $dir } لأن المجلد غير موجود، ليس لديك إذن أو المجلد غير فارغ.
delete_items_label = { $items } سيتم حذف الملفات.
delete_items_groups_label = { $items } ملفات من { $groups } سيتم حذف المجموعات.
hardlink_failed = فشل في الربط
hard_sym_invalid_selection_title_dialog = إختيار غير صالح مع بعض المجموعات
hard_sym_invalid_selection_label_1 = في بعض المجموعات هناك رقم قياسي واحد تم اختياره وسيتم تجاهله.
hard_sym_invalid_selection_label_2 = لتتمكن من صلابة / ربط هذه الملفات، يجب اختيار نتيجتين على الأقل في المجموعة.
hard_sym_invalid_selection_label_3 = الأول في المجموعة معترف به على أنه أصلي ولا يتغير ولكن الثاني ثم يتم تعديله.
hard_sym_link_title_dialog = تأكيد الرابط
hard_sym_link_label = هل أنت متأكد من أنك تريد ربط هذه الملفات؟
move_folder_failed = فشل في نقل المجلد { $name }، السبب { $reason }
move_file_failed = فشل نقل الملف { $name }، السبب { $reason }
move_files_title_dialog = اختر مجلد تريد نقل الملفات المكررة إليه
move_files_choose_more_than_1_path = يمكن تحديد مسار واحد فقط لتكون قادرة على نسخ الملفات المكررة، المحددة { $path_number }.
move_stats = نقل بشكل صحيح { $num_files }/{ $all_files } عناصر
save_results_to_file = تم حفظ النتائج إلى ملفات txt و json في مجلد { $name }.
search_not_choosing_any_music = خطأ: يجب عليك تحديد مربع اختيار واحد على الأقل مع أنواع البحث عن الموسيقى.
search_not_choosing_any_broken_files = خطأ: يجب عليك تحديد مربع اختيار واحد على الأقل مع نوع الملفات المحددة المكسورة.
include_folders_dialog_title = مجلدات لتضمينها
exclude_folders_dialog_title = مجلدات للاستبعاد
include_manually_directories_dialog_title = إضافة دليل يدوياً
cache_properly_cleared = مسح ذاكرة التخزين المؤقت بشكل صحيح
cache_clear_duplicates_title = مسح ذاكرة التخزين المؤقت التكراري
cache_clear_similar_images_title = مسح ذاكرة التخزين المؤقت مشابهة للصور
cache_clear_similar_videos_title = مسح ذاكرة التخزين المؤقت المماثلة للفيديوهات
cache_clear_message_label_1 = هل تريد مسح ذاكرة التخزين المؤقت للإدخالات العتيقة؟
cache_clear_message_label_2 = هذه العملية ستزيل جميع إدخالات ذاكرة التخزين المؤقت التي تشير إلى ملفات غير صالحة.
cache_clear_message_label_3 = قد يؤدي هذا إلى تسريع التحميل/الحفظ إلى ذاكرة التخزين المؤقت.
cache_clear_message_label_4 = تحذير: العملية ستزيل جميع البيانات المخزنة مؤقتاً من الأقراص الخارجية الغير موصولة. لذلك سوف تحتاج كل تجزئة إلى التجديد.
# Show preview
preview_image_resize_failure = فشل تغيير حجم الصورة { $name }.
preview_image_opening_failure = فشل في فتح الصورة { $name }، السبب { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = المجموعة { $current_group }/{ $all_groups } ({ $images_in_group } صورة)
compare_move_left_button = ل
compare_move_right_button = ر
