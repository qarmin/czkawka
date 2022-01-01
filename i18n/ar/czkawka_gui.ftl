# Core
core_similarity_very_high = عالية جدا
core_similarity_high = مرتفع
core_similarity_medium = متوسط
core_similarity_small = صغير
core_similarity_very_small = صغير جدا
core_similarity_minimal = الحد الأدنى
core_cannot_open_dir = لا يمكن فتح dir { $dir }، لسبب { $reason }
core_cannot_read_entry_dir = لا يمكن قراءة الإدخال في dir { $dir }، السبب { $reason }
core_cannot_read_metadata_dir = لا يمكن قراءة البيانات الوصفية في dir { $dir }، السبب { $reason }
core_file_not_utf8_name = الملف { $name } لا يحتوي على اسم UTF-8 صالح (قد لا تظهر بعض الأحرف)
core_file_modified_before_epoch = يبدو أن الملف { $name } قد تم تعديله قبل يونكس Epoch
core_folder_modified_before_epoch = يبدو أن الملف { $name } قد تم تعديله قبل يونكس Epoch
core_file_no_modification_date = غير قادر على الحصول على تاريخ التعديل من الملف { $name }، السبب { $reason }
core_folder_no_modification_date = غير قادر على الحصول على تاريخ التعديل من المجلد { $name }، السبب { $reason }
# Window titles
window_settings_title = خيارات
window_main_title = Czkawka
window_progress_title = المسح
# General
general_ok_button = حسناً
general_close_button = أغلق
# Main window
music_title_checkbox = العنوان
music_artist_checkbox = الفنان
music_album_title_checkbox = عنوان الألبوم
music_album_artist_checkbox = فنان الألبوم
music_year_checkbox = السنة
music_comparison_checkbox = مقارنة تقريبية
music_comparison_checkbox_tooltip =
    يبحث عن ملفات موسيقية مماثلة باستخدام AI، الذي يستخدم تعلم الآلة لإزالة الأقواس من عبارة، على سبيل المثال. مع تمكين هذا الخيار, الملفات المعنية سوف تعتبر مكررة:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = الاسم
duplicate_mode_size_combo_box = الحجم
duplicate_mode_hash_combo_box = التجزئة
duplicate_hash_type_tooltip =
    يقدم Czkawka 3 أنواع من التجزئة التي يمكن استخدامها:
    
    Blake3 - دالة التجزئة المشفرة. يستخدم كخوارزمية تجزئة افتراضية، لأنه سريع جدا.
    
    CRC32 - دالة تجزئة بسيطة. ينبغي أن يكون أسرع من بليك 3، ولكن قد يكون هناك نادراً جداً بعض الاصطدام.
    
    XXH3 - في حالة الأداء وجودة التجزئة شبيهة جداً بالبحيرة 3، بحيث يمكن استخدام هذه الأساليب بسهولة.
duplicate_check_method_tooltip =
    في الوقت الحالي، يقدم Czkawka ثلاثة أنواع من الطرق للعثور على التكرارات بواسطة:
    
    الاسم - العثور على الملفات التي تحمل نفس الاسم.
    
    الحجم - العثور على الملفات التي لها نفس الحجم.
    
    Hash - Finds الملفات التي لها نفس المحتوى. هذا الوضع يقوم بتجزئة الملف ثم يقارن هذه التجزئة للعثور على التكرار. هذا الوضع هو أكثر الطرق أماناً للعثور على التكرار. الأدوات تستخدم التخزين المؤقت بكثافة، لذا فالمسح الثاني والمزيد لنفس البيانات يجب أن يكون أسرع من الأول.
image_hash_size_tooltip =
    يعرض Czkawka تغيير حجم التجزئة التي تم إنشاؤها لكل صور. سبب التجزئة الأكبر يسمح بالعثور على صور ذات كمية أقل من الاختلافات بين الصور، ولكن استخدامها أبطأ قليلا.
    
    القيمة الافتراضية للتجزئة هي 8 بايت، مما يسمح بالعثور على صور متشابهة ومختلفة جدا. وينبغي ألا تستخدم التجزئة 16 و32 إلا لصور متماثلة تقريبا. 64 بايت هاش لا ينبغي استخدامها، باستثناء الحالة التي تحتاج فيها إلى فروق صغيرة للعثور عليها
image_resize_filter_tooltip = لحساب تجزئة الصورة، يجب على المكتبة أولاً تغيير حجمها. يجب أن ترصد على الخوارزمية المختارة، الصورة الناتجة لن تبدو مختلفة عن بعضها البعض. الخوارزمية الأسرع لاستخدامها، ولكن أيضا الخوارزمية التي تعطي أسوأ النتائج هي الخوارزمية الأنيقة.
image_hash_alg_tooltip = يمكن للمستخدمين اختيار واحد من العديد من الخوارزميات لحساب التجزئة. ولكل منها نقاط قوية وأضعف على حد سواء وستعطي أحيانا نتائج أفضل وأحيانا أسوأ لصور مختلفة. لذلك لاختيار الأفضل، الاختبار اليدوي مطلوب.
main_notebook_image_fast_compare = مقارنة سريعة
main_notebook_image_fast_compare_tooltip =
    تسريع البحث والمقارنة بين التجزئة.
    
    في عكس الوضع العادي حيث تتم مقارنة كل تجزئة ببعضها البعض x مرات، حيث x هو التشابه الذي يختاره المستخدم، في هذا الوضع دائمًا تستخدم مقارنة واحدة فقط.
    
    هذا الخيار ينصح به عند مقارنة أكثر من 10000 صورة مع تشابه غير كبير في 0(Very High).
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
main_tree_view_column_album_title = عنوان الألبوم
main_tree_view_column_album_artist = فنان الألبوم
main_tree_view_column_symlink_file_name = اسم ملف الرابط الرمزي
main_tree_view_column_symlink_folder = مجلد رملنك
main_tree_view_column_destination_path = مسار الوجهة
main_tree_view_column_type_of_error = نوع الخطأ
main_label_check_method = طريقة التحقق
main_label_hash_type = نوع التجزئة
main_label_hash_size = حجم التجزئة
main_label_size_bytes = الحجم (بايت)
main_label_min_size = الحد الأدنى
main_label_max_size = الحد الأقصى
main_label_shown_files = عدد الملفات المعروضة
main_label_resize_algorithm = تغيير حجم الخوارزمية
main_label_similarity = مشابهة{" " }
check_button_general_same_size = تجاهل نفس الحجم
check_button_general_same_size_tooltip = تجاهل من النتائج، الملفات التي لها حجم مطابق - عادة هذه المكررة 1:1
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
upper_manual_add_included_button_tooltip = يسمح بإضافة اسم الدليل للبحث باليد.
upper_add_included_button_tooltip = إضافة دليل جديد للبحث.
upper_remove_included_button_tooltip = حذف الدليل من البحث.
upper_manual_add_excluded_button_tooltip = يسمح بإضافة اسم الدليل المستبعد يدوياً.
upper_add_excluded_button_tooltip = إضافة دليل ليتم استبعاده في البحث.
upper_remove_excluded_button_tooltip = حذف الدليل من المستبعد.
upper_notebook_items_configuration = تكوين العناصر
upper_notebook_excluded_directories = المجلدات المستبعدة
upper_notebook_included_directories = المجلدات المضمنة
upper_allowed_extensions_tooltip =
    يجب أن تكون الملحقات المسموح بها مفصولة بفاصلة (بشكل افتراضي جميع متاحة).
    
    مايكوس الآخرون، فيديو، MUSIC، TEXT التي تضيف ملحقات متعددة في وقت واحد متوفرة أيضًا.
    
    مثال استخدام ".exe, IMAGE, VIDEO, .rar, 7z" - هذا يعني الصورة (e. . ملفات jpg, png) (مثلاً: avi, mp4), ex, rar and 7z سيتم مسحها.
upper_excluded_items_tooltip =
    يجب أن تحتوي العناصر المستبعدة على * بطاقة حرجة ويجب أن تكون مفصولة بفواصل.
    هذا أبطأ من المجلدات المستبعدة، لذلك استخدمها بحذر.
upper_excluded_items = البنود المستثناة:
upper_allowed_extensions = الإضافات المسموح بها:
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
    يسمح بتحديد السجلات عن طريق مسارها.
    
    مثال الاستخدام:
    /home/pimpek/rzecz.txt يمكن العثور عليه مع /home/pim*
popover_custom_name_check_button_entry_tooltip =
    يسمح بتحديد السجلات حسب أسماء الملفات.
    
    مثال الاستخدام:
    /usr/ping/pong.txt يمكن العثور عليه مع *ong*
popover_custom_regex_check_button_entry_tooltip =
    يسمح بتحديد السجلات بواسطة Regex.
    
    مع هذا الوضع، النص الذي تم البحث عنه هو المسار بالاسم.
    
    مثال الاستخدام:
    /usr/bin/ziemniak. يمكن العثور على xt مع /ziem[a-z]+
    
    هذا الاستخدام الافتراضي لتطبيق Rust regex ، بحيث يمكنك قراءة المزيد عنه في https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    يمنع من اختيار كافة السجلات في المجموعة.
    
    يتم تمكين هذا بشكل افتراضي، لأنه في معظم الحالات لا يريد المستخدم حذف الملفات الأصلية والمتكررة، ولكن تريد أن تترك ملف واحد على الأقل.
    
    تحذير: هذا الإعداد لا يعمل إذا قام المستخدم بالفعل بتحديد جميع النتائج في المجموعة يدوياً.
popover_custom_regex_path_label = المسار
popover_custom_regex_name_label = الاسم
popover_custom_regex_regex_label = مسار Regex + اسم
popover_custom_all_in_group_label = عدم تحديد جميع السجلات في المجموعة
popover_custom_mode_unselect = إلغاء تحديد مخصص
popover_custom_mode_select = تحديد مخصص
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
bottom_search_button_tooltip = ابدأ في البحث عن الملفات/المجلدات.
bottom_select_button_tooltip = يحدد السجلات. يمكن معالجة الملفات/المجلدات المحددة في وقت لاحق.
bottom_delete_button_tooltip = حذف الملفات/المجلدات المحددة.
bottom_save_button_tooltip = حفظ البيانات حول البحث في الملف
bottom_symlink_button_tooltip =
    ينشئ روابط رمزية.
    يعمل فقط عندما يتم تحديد اثنين على الأقل من النتائج في المجموعة.
    أولا لم يتغير و الثاني و اللاحق مرتبطين بالأول.
bottom_hardlink_button_tooltip =
    ينشئ روابط صلبة.
    يعمل فقط عندما يتم تحديد 2 من النتائج على الأقل في المجموعة.
    أولا لم يتغير و الثاني و اللاحق متصلين بالأول.
bottom_move_button_tooltip =
    ينقل الملفات إلى المجلد المحدد.
    ينسخ جميع الملفات إلى المجلد دون الحفاظ على شجرة الدليل.
    عند محاولة نقل ملفين مع نفس الاسم إلى مجلد، سيتم فشل الثانية وإظهار الخطأ.
bottom_show_errors_tooltip = إظهار/إخفاء لوحة الأخطاء السفلية.
bottom_show_upper_notebook_tooltip = إظهار/إخفاء لوحة دفتر الملاحظات العلوية.
# Progress Window
progress_stop_button = توقف
# About Window
about_repository_button_tooltip = رابط لصفحة المستودع مع رمز المصدر.
about_donation_button_tooltip = رابط لصفحة التبرع.
about_instruction_button_tooltip = رابط لصفحة التعليمات.
about_translation_button_tooltip = رابط إلى صفحة كراودِن مع ترجمة التطبيق. بالبولندية الرسمية والإنجليزية مدعومة، ولكن أي مساعدة في لغة أخرى ستكون موضع تقدير.
about_repository_button = المستودع
about_donation_button = تبرع
about_instruction_button = تعليمات
about_translation_button = الترجمة
# Header
header_setting_button_tooltip = فتح مربع حوار الإعدادات.
header_about_button_tooltip = فتح مربع الحوار مع معلومات حول التطبيق.

# Settings


## General

settings_save_at_exit_button_tooltip = حفظ التكوين للملف عند إغلاق التطبيق.
settings_load_at_start_button_tooltip =
    تحميل عند بدء التكوين من الملف.
    
    عدم اختيار هذا الخيار سيحمل الإعدادات الافتراضية.
settings_confirm_deletion_button_tooltip = إظهار مربع حوار التأكيد عند النقر على زر الحذف.
settings_confirm_link_button_tooltip = يظهر مربع حوار التأكيد عند النقر على زر الربط الصلب/التفضيل.
settings_confirm_group_deletion_button_tooltip = يظهر مربع حوار عند محاولة إزالة جميع السجلات من المجموعة.
settings_show_text_view_button_tooltip = إظهار لوحة الخطأ في الأسفل.
settings_use_cache_button_tooltip = الخيار الذي يسمح له بعدم استخدام ميزة ذاكرة التخزين المؤقت.
settings_use_trash_button_tooltip = عند تمكينه ينقل الملفات إلى سلة المهملات بدلاً من حذفها بشكل دائم.
settings_language_label_tooltip = يسمح باختيار لغة الواجهة من الواجهة المتاحة.
settings_save_at_exit_button = حفظ التكوين عند الخروج
settings_load_at_start_button = تحميل التكوين في البداية
settings_confirm_deletion_button = إظهار تأكيد مربع الحوار عند حذف أي ملفات
settings_confirm_link_button = إظهار مربع حوار تأكيد عند ربط أي ملفات بصعوبة/رموز
settings_confirm_group_deletion_button = إظهار تأكيد مربع الحوار عند حذف جميع الملفات في المجموعة
settings_show_text_view_button = إظهار لوحة النص السفلي
settings_use_cache_button = استخدام ذاكرة التخزين المؤقت
settings_use_trash_button = نقل الملفات المحذوفة إلى سلة المهملات
settings_language_label = اللغة
settings_multiple_delete_outdated_cache_checkbutton = حذف إدخالات ذاكرة التخزين المؤقت القديمة تلقائياً
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    يسمح بحذف نتائج ذاكرة التخزين المؤقت العتيقة التي تشير إلى ملفات غير موجودة.
    
    عند التمكين، يتأكد التطبيق عند تحميل السجلات، من أن كل ما يشير إلى الملفات الصالحة وتجاهل الملفات المكسورة.
    
    سيؤدي تعطيل هذا الخيار، إلى المساعدة في مسح الملفات على الأقراص الخارجية، لذلك لن يتم إزالة إدخالات ذاكرة التخزين المؤقت عنها في الفحص التالي.
    
    في حالة وجود مئات الآلاف من السجلات في المخبئ، من المقترح تمكين هذا الخيار، لتسريع تحميل ذاكرة التخزين المؤقت والحفظ في بداية ونهاية عملية المسح.
settings_notebook_general = عمومي
settings_notebook_duplicates = مكرر
settings_notebook_images = صور مشابهة
settings_notebook_videos = فيديو مشابه

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = يعرض المعاينة على الجانب الأيمن، عند تحديد ملف الصورة.
settings_multiple_image_preview_checkbutton = عرض معاينة الصورة
settings_multiple_clear_cache_button_tooltip =
    مسح ذاكرة التخزين المؤقت يدويا من الإدخالات العتيقة.
    يجب أن تستخدم فقط إذا تم تعطيل المسح التلقائي.
settings_multiple_clear_cache_button = إزالة النتائج القديمة من ذاكرة التخزين المؤقت للصور

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    يخفي جميع الملفات باستثناء واحدة، إذا كانت نقاط إلى نفس البيانات(متصلة).
    
    على سبيل المثال. في حالة وجود 7 ملفات على القرص مرتبطة ارتباطاً وثيقاً ببيانات محددة وملف واحد مختلف يحتوي على بيانات واحدة ولكن مختلفة, بعد ذلك في المكرر سيظهر ملف فريد واحد فقط وملف واحد من الملفات المرتبطة بشدة.
settings_duplicates_minimal_size_entry_tooltip =
    يسمح بتعيين الحد الأدنى من حجم الملف، والذي سيتم تخزينه.
    
    اختيار قيمة أصغر، سيولد المزيد من السجلات التي ستسرع البحث، ولكن سيبطئ التحميل/حفظ ذاكرة التخزين المؤقت.
settings_duplicates_prehash_checkbutton_tooltip =
    تمكين التخزين المؤقت للباباش (حساب التجزئة من جزء صغير من الملف) الذي يسمح بإلقاء نتائج غير مكررة في وقت سابق.
    
    يتم تعطيله بشكل افتراضي لأنه يمكن أن يسبب تباطؤا في بعض الحالات.
    
    من المستحسن بشدة استخدامه عند مسح مئات الآلاف أو المليون من الملفات، لأنه يمكن تسريع البحث عدة مرات.
settings_duplicates_prehash_minimal_entry_tooltip = الحجم الأدنى للإدخال المخبئ.
settings_duplicates_hide_hard_link_button = إخفاء الروابط الصلبة (فقط لينكس وماكوس)
settings_duplicates_prehash_checkbutton = استخدام ذاكرة التخزين المؤقت
settings_duplicates_minimal_size_cache_label = الحد الأدنى لحجم الملفات في البايت المحفوظة في ذاكرة التخزين المؤقت
settings_duplicates_minimal_size_cache_prehash_label = الحد الأدنى لحجم الملفات في بايت المحفوظة في ذاكرة التخزين المؤقت

## Saving/Loading settings

settings_saving_button_tooltip = حفظ الإعدادات الحالية إلى الملف.
settings_loading_button_tooltip = تحميل الإعدادات من الملف واستبدل الإعدادات الحالية بها.
settings_reset_button_tooltip = إعادة تعيين الإعدادات الحالية إلى الإعدادات الافتراضية.
settings_saving_button = حفظ التكوين
settings_loading_button = تحميل التكوين
settings_reset_button = إعادة ضبط الإعدادات

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    يفتح المجلد حيث يتم تخزين ملفات txt مع ذاكرة التخزين المؤقت.
    
    تعديلها قد يتسبب في إظهار نتائج غير صحيحة ولكن أيضا تعديل e.. قد يوفر المسار الوقت عند نقل كمية كبيرة من الملفات إلى مكان مختلف.
    
    يمكنك نسخ هذه الملفات بين أجهزة الكمبيوتر لتوفير الوقت على المسح مرة أخرى للملفات (بالطبع إذا كان لديهم بنية دليل مماثلة).
    
    في حالة وجود مشاكل مع ذاكرة التخزين المؤقت، يمكن إزالة هذه الملفات، لذا سيقوم التطبيق بتجديدها.
settings_folder_settings_open_tooltip =
    يفتح المجلد حيث يتم تخزين تكوين Czkawka
    
    تعديله باليد، قد يؤدي إلى كسر سير العمل الخاص بك.
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
# Progress window
progress_scanning_general_file = فحص ملف { $file_number }
progress_scanning_broken_files = فحص { $file_checked }/{ $all_files } ملف
progress_scanning_video = تجزئة الفيديو { $file_checked }/{ $all_files }
progress_scanning_image = تجزئة الصورة { $file_checked }/{ $all_files }
progress_comparing_image_hashes = مقارنة { $file_checked }/{ $all_files } تجزئة الصور
progress_scanning_music_tags_end = مقارنة علامات { $file_checked }/{ $all_files } ملف الموسيقى
progress_scanning_music_tags = قراءة وسوم { $file_checked }/{ $all_files } ملف الموسيقى
progress_scanning_empty_folders = يتم فحص { $folder_number } مجلد
progress_scanning_size = حجم البحث لـ { $file_number } ملف
progress_scanning_name = فحص اسم ملف { $file_number }
progress_analyzed_partial_hash = تم تحليل تجزئة جزئية ل { $file_checked }/{ $all_files } ملفات
progress_analyzed_full_hash = تم تحليل التجزئة الكاملة لـ { $file_checked }/{ $all_files } ملفات
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
saving_loading_not_valid = إعداد "{ $data }" غير موجود في إصدار التطبيق الحالي.
# Invalid symlinks
invalid_symlink_infinite_recursion = التكرار اللامتناهي
invalid_symlink_non_existent_destination = ملف الوجهة غير موجود
# Other
searching_for_data = البحث عن البيانات، قد يستغرق بعض الوقت، يرجى الانتظار...
text_view_messages = الرسائل
text_view_warnings = التحذيرات
text_view_errors = أخطاء
about_window_motto = هذا البرنامج حر في الاستخدام وسوف يكون دائماً.
# Various dialog
dialogs_ask_next_time = اسأل المرة القادمة
delete_file_failed = فشل في إزالة الملف { $name }، السبب { $reason }
delete_title_dialog = تأكيد حذف
delete_question_label = هل أنت متأكد من أنك تريد حذف الملفات؟
delete_all_files_in_group_title = تأكيد حذف جميع الملفات في المجموعة
delete_all_files_in_group_label1 = في بعض المجموعات هناك جميع السجلات المحددة.
delete_all_files_in_group_label2 = هل أنت متأكد من أنك تريد حذفهم؟
delete_folder_failed = فشل في إزالة المجلد { $dir } لأن المجلد غير موجود، ليس لديك أذونات أو ليست فارغة.
delete_items_label = { $items } سيتم إزالة الملفات.
delete_items_groups_label = { $items } ملفات من { $groups } سيتم إزالة المجموعات.
hardlink_failed = فشل في الربط
hard_sym_invalid_selection_title_dialog = إختيار غير صالح مع بعض المجموعات
hard_sym_invalid_selection_label_1 = في بعض المجموعات هناك سجل واحد فقط تم اختياره وسيتم تجاهله.
hard_sym_invalid_selection_label_2 = لتتمكن من ربط هذه الملفات، تحتاج إلى تحديد نتيجتين على الأقل في المجموعة.
hard_sym_invalid_selection_label_3 = الأول في المجموعة معترف به على أنه أصلي ولا يتغير ولكن الثاني ثم يتم تعديله.
hard_sym_link_title_dialog = تأكيد الرابط
hard_sym_link_label = هل أنت متأكد من أنك تريد ربط هذه الملفات؟
move_folder_failed = فشل في نقل المجلد { $name }، السبب { $reason }
move_file_failed = فشل نقل الملف { $name }، السبب { $reason }
move_files_title_dialog = اختر مجلد تريد نقل الملفات المكررة إليه
move_files_choose_more_than_1_path = يجب تحديد مسار واحد فقط لتكون قادرة على نسخ الملفات المكررة، المحددة { $path_number }.
move_stats = نقل بشكل صحيح { $num_files }/{ $all_files } عناصر
save_results_to_file = تم حفظ النتائج إلى الملف { $name }
search_not_choosing_any_music = خطأ: يجب عليك تحديد مربع اختيار واحد على الأقل مع أنواع البحث عن الموسيقى.
include_folders_dialog_title = مجلدات لتضمينها
exclude_folders_dialog_title = مجلدات للاستبعاد
include_manually_directories_dialog_title = إضافة دليل يدوياً
cache_properly_cleared = مسح ذاكرة التخزين المؤقت بشكل صحيح
cache_clear_duplicates_title = مسح ذاكرة التخزين المؤقت التكراري
cache_clear_similar_images_title = مسح ذاكرة التخزين المؤقت مشابهة للصور
cache_clear_similar_videos_title = مسح ذاكرة التخزين المؤقت المماثلة للفيديوهات
cache_clear_message_label_1 = هل تريد مسح ذاكرة التخزين المؤقت من المواد العتيقة؟
cache_clear_message_label_2 = هذه العملية ستزيل جميع إدخالات ذاكرة التخزين المؤقت التي تشير إلى الملفات غير صالحة.
cache_clear_message_label_3 = قد يؤدي هذا إلى تسريع التحميل/الحفظ إلى ذاكرة التخزين المؤقت.
cache_clear_message_label_4 = تحذير: العملية ستزيل جميع البيانات المخزنة مؤقتاً من الأقراص الخارجية الغير موصولة، لذا سيتعين إنشاء التجزئة مرة أخرى.
# Show preview
preview_temporary_file = فشل في فتح ملف صورة مؤقت { $name }، لسبب { $reason }.
preview_0_size = لا يمكن إنشاء معاينة للصورة { $name }، بعرض أو ارتفاع 0 .
preview_temporary_image_save = فشل في حفظ ملف الصورة المؤقت إلى { $name }، السبب { $reason }.
preview_temporary_image_remove = فشل في حذف ملف الصورة المؤقت { $name }، السبب { $reason }.
preview_failed_to_create_cache_dir = فشل في إنشاء dir { $name } الذي تحتاجه معاينة الصورة، السبب { $reason }.
