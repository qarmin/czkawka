
window_settings_title = تنظیمات
window_main_title = چکاواک (همیپ)
window_progress_title = سkenنگ
window_compare_images = مقایسه تصاویر
general_close_button = بسته شود
music_title_checkbox = عنوان
music_artist_checkbox = هنرپرداز
music_year_checkbox = سال
music_bitrate_checkbox = بیت‌روتpedoze
music_genre_checkbox = نوع
music_length_checkbox = طول
music_comparison_checkbox = مقایسه تقریبی
music_checking_by_tags = برچسب‌ها
music_checking_by_content = محتوا
same_music_seconds_label = کسری مینیمال دوم طول دومین جفت
same_music_similarity_label = مکانیمم تفاضل
music_compare_only_in_title_group = مقایسه در گروه‌های با عنوان‌های مشابه
music_compare_only_in_title_group_tooltip = 
        هنگامی که فعال است، فایل‌ها بر اساس عنوان گروه‌بندی می‌شوند و سپس با هم مقایسه می‌شوند.

        با 10000 فایل، به جای تقریباً 100 میلیون مقایسه، حدوداً 20000 مقایسه خواهد بود.
same_music_tooltip = 
        جستجوی مسایل موسیقی مشابه بر اساس محتوا می‌تواند با تنظیم موارد زیر پیشنهاد شود:

        - زمان حداقل فragment بعد از آن مسایل موسیقی می‌توانند به عنوان مشابه شناسایی شوند
        - تفاوت بینی مرکب دو fragment مورد تست

        کلید خوب کسب نتایج است این پارامترها را به صورت منطقی تر از طریق تنظیم‌های ارائه شده جستجو کنید.

        تنظیم زمان حداقل ۵ ثانیه و تفاوت بینی مرکب ۱.۰، برای پیدا کردن فرگمنت‌های معمولاً مشابه در فایل‌ها استفاده خواهد شد.
        در حالی که زمان ۲۰ ثانیه و تفاوت بینی مرکب ۶.۰ برای پیدا کردن remixات/نسخه‌های live بهتر کار می‌کند.

        به طور پیش فرض، هر فایل موسیقی با هر فایل دیگری مقایسه می‌شود و این زمان خیلی زمان‌بر خواهد بود وقتی که تعداد بسیاری از فایل‌ها را تست کنید، بنابراین به طور گسترده‌تر استفاده از پوشه‌های مرجع و مشخص شدن کدام فایل‌ها را که باید با هم مقایسه شوند (با تعداد برابر فایل‌ها، مقایسه خلاک‌های ضخامت چندگانه حداقل ۴ برابر از بدون پوشه‌های مرجع سریعتر خواهد بود) کارآمد است.
music_comparison_checkbox_tooltip = 
        آنچه با هوش مصنوعی جستجو می‌کند و از یادگیری ماشین برای حذف پرانتز‌ها از جمله استفاده می‌کند. به عنوان مثال، با فعال سازی این گزینه، فایل‌های مورد سوال نظرات همگون در نظر گرفته خواهند شد:

        Świędziżłób     ---     Świędziżłób ( Remix Lato 2021)
duplicate_case_sensitive_name = متن حساس به بیانیه
duplicate_case_sensitive_name_tooltip = 
        وقتی فعال است، تنها رکوردهایی را در گروه گذاری می‌کند که دقیقاً نام آن‌ها مشابه هستند مانند Żołd <-> Żołd

        停用 اینگونde برای گروه‌بندی نام‌ها در نظر نمی‌گیرد که هر حرف چقدر تقریب زدن دارد مانند żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = حجم و نام
duplicate_mode_name_combo_box = نام
duplicate_mode_size_combo_box = grootه
duplicate_hash_type_tooltip = 
        Czkawka ۳ نوع هاش می‌پذیرد:

        بلک ۳ - تابع چربی کriتوگرافیک. این آن به طور پیش فرض است زیرا خیلی سریع است.

        CRC32 - تابع چربی ساده. این سریع‌تر از بلک ۳ باشد، اما در نسخه‌ای نادر ممکن است برخورد کامل داشته باشد.

        XXH3 - در عملکرد و کیفیت چربی تقریباً شبیه به بلک ۳ (اما کriتوگرافیک نیست). بنابراین، این حالت‌ها می‌توانند خلک ساده جایگزین شوند.
duplicate_check_method_tooltip = 
        این لحظه، Czkawka سه روش از پشتیبان برای پیدا کردن فایل‌های تکراری دارد:

        نام - فایل‌هایی را که نام آنها یکسان است را پیدا می‌کند.

        حجم - فایل‌هایی را که حجم آنها یکسان است را پیدا می‌کند.

        هش - فایل‌هایی را که محتوای آنها یکسان است را پیدا می‌کند. این مód هش فایل را بررسی می‌کند و بعد با مقایسه این هش، تکراری‌ها را پیدا می‌کند. این مód روش امن‌تری برای پیدا کردن تکراری‌ها است. اپلیکیشن به شدت از کاش استفاده می‌کند، بنابراین دومین و فراخانی‌های بعدی داده‌های یکسان بسیار سریع‌تر از اولین بار خواهند بود.
image_hash_size_tooltip = 
        هر تصویری که تایید شده یک هش خاص ایجاد می‌کند که می‌توان آن را با هم مقایسه کرد، و اختلاف زیادی بین آنها به این معناست که این تصاویر مشابه هستند.

        8 باره سایز هش خیلی خوبی است تا تصاویری را که تنها ممکن است شبیه به اصلی باشند پیدا کنیم. با یک مجموعه بزرگتر از تصاویر (>1000)، این موضوع تعداد زیادی از مقادیر نادرست را تولید خواهد کرد، بنابراین بهتر است در این صورت سایز هش بزرگ‌تری را استفاده کنید.

        16 پوزیشن سایز هش پیش‌فرض است که یک ترادف خوبی بین پیدا کردن تصاویری که حتی شبیه به اصلی هستند و داشتن تنوع کمتری از تنش‌های هش، محسوب می‌شود.

        32 و 64 هش فقط تصاویر بسیار مشابه را پیدا می‌کنند، اما بایستی اچ helt نسبتاً کمترین مقادیر نادرست داشته باشند (ربات‌ها با کانال alpha ممکن است غیرمعمول باشند).
image_resize_filter_tooltip = 
        برای حساب کردن هش تصویر، بиблиا می‌تواند ابتدا آن را نرم‌زد کند.

        به گونه‌ای که الگوریتم انتخاب شده است، تصویری که برای محاسبه هش استفاده می‌شود به صورت کمی متفاوت خواهد بود.

        الگوریتم سریع‌تر برای استفاده وجود دارد، ولی نتایج آن کسب‌کننده پایین‌تری هستند. الگوریتم نزدیک‌تر (Nearest) به طور پیش‌فرض فعال است زیرا با حجم 16x16 کمترین کیفیت این کسیزه که نظار قابل توجهی برای بین‌داشت ندارد.

        با حجم 8x8 کسیزه، پیشنهاد می‌شود به جای الگوریتم نزدیک‌تر (Nearest) از یک الگوریتم دیگر استفاده شود تا گروه‌های تصویر بگذرند.
image_hash_alg_tooltip = 
        کاربران می‌توانند از یکی از دستورالعمل‌های بسیاری برای محاسبه هاش که در اختیار آن‌ها قرار داده شده است، پیشنهاد دستورالعمل خود را انتخاب کنند.

        هر یک این‌ها نقطه قوت و ضعف کلیدی دارند و برای تصاویر مختلف همگرایتر یا نامطلوبتر می‌توانند عملکرد خود را نشان دهند.

        بنابراین، برای تعیین بهترین یکی برای شما، آزمون دستی ضروری است.
big_files_mode_combobox_tooltip = می‌پذیرد جستجوی فایل‌های کوچکتر/بزرگتر را
big_files_mode_label = فایل‌های بررسی شده
big_files_mode_smallest_combo_box = تازیست
big_files_mode_biggest_combo_box = بزرگsteamaño_placeholder
main_notebook_duplicates = فایل‌های تکراری
main_notebook_empty_directories = دایرکتوری خالی
main_notebook_big_files = فایل‌های بزرگ
main_notebook_empty_files = فایل‌های خالی
main_notebook_temporary = فایل‌های موقت
main_notebook_similar_images = تصورهای مشابه
main_notebook_similar_videos = فیلم‌های مشابه
main_notebook_same_music = 拷贝的音乐
main_notebook_symlinks = سینمک های معتبر ناامن
main_notebook_broken_files = فایل‌های ضارب
main_notebook_bad_extensions = برچسب‌های بد
main_tree_view_column_file_name = نام فایل
main_tree_view_column_folder_name = نام پوشه
main_tree_view_column_path = پادکست
main_tree_view_column_modification = تاریخ بروزرسانی
main_tree_view_column_size = حجم
main_tree_view_column_similarity = شبیهسازی
main_tree_view_column_dimensions = بعدیون
main_tree_view_column_title = عنوان
main_tree_view_column_artist = کارگردان
main_tree_view_column_year = سال
main_tree_view_column_bitrate = بریتیتی کدگذاری
main_tree_view_column_length = طول
main_tree_view_column_genre = ژانر
main_tree_view_column_symlink_file_name = لینک ناشی فایل
main_tree_view_column_symlink_folder = تیم‌پیکس فولدر
main_tree_view_column_destination_path = پیشانته مسیر
main_tree_view_column_type_of_error = نوع خطا
main_tree_view_column_current_extension = بازگشت بسته شده
main_tree_view_column_proper_extensions = توضیح معتبر
main_tree_view_column_codec = کدکodb
main_label_check_method = Métودا را بررسی کنید
main_label_hash_type = نوع هاش
main_label_hash_size = سایز هاش
main_label_size_bytes = حجم (بایت)
main_label_min_size = مین
main_label_max_size = مکس
main_label_shown_files = تعداد فایل‌های نمایش‌یافته
main_label_resize_algorithm = الگوریتم سایز بندی
main_label_similarity = سхожگی{"   "}
main_check_box_broken_files_audio = آوتویډ
main_check_box_broken_files_archive = اریکرج
main_check_box_broken_files_image = عکس
check_button_general_same_size = تنها سایز‌های مختلف را تغییر ندهید
check_button_general_same_size_tooltip = فایل‌های مشابه سایز آن‌ها در نتایج را ترک کنید - معمولاً این فایل‌ها دوباره تولید شده‌اند (1:1)
main_label_size_bytes_tooltip = حجم فایل‌هایی که در طول سcan استفاده خواهند شد
upper_tree_view_included_folder_column_title = پوشه‌هایی که جستجو کنید
upper_tree_view_included_reference_column_title = دایرکتوری های مرجع
upper_recursive_button = پیچیده‌ترین
upper_recursive_button_tooltip = اگر انتخاب شد، به دنبال فایل‌هایی نیز جستجو کنید که قطعاً در پوشه‌های گرفته‌شده مستقیماً وجود ندارند.
upper_manual_add_included_button = افضalan手工添加（保留原文格式）
upper_add_included_button = افزودن
upper_remove_included_button = حذف
upper_manual_add_excluded_button = متن دستی اضافه
upper_add_excluded_button = افضال
upper_remove_excluded_button = حذف
upper_manual_add_included_button_tooltip = 
        درآماد نام دایرکتوری را برای جستجو با دست اضافه کنید.

        برای بارگذاری چندین مسیر به طور همزمان، آنها را با یک ';' جدا کنید;

        /home/رومان;/home/روزکaz خواهد باعث اضافه شدن دو دایرکتوری /home/رومان و /home/روزکaz می‌شود
upper_add_included_button_tooltip = دایرکتوری جدید به جستجو اضافه کنید.
upper_remove_included_button_tooltip = پوشه را از جستجو حذف کنید.
upper_manual_add_excluded_button_tooltip = 
        مدیریت نام دایرکتوری استراحت شده را به صورت håد良心化处理结果：
        保持相同的语气和风格。保留任何特殊格式或占位符。
        仅返回翻译文本，不提供解释或其他额外文本。

        ترجمه:
        نام دایرکتوری استراحت شده را به صورت håد و يدی اضافه کنید.

        برای اضافه کردن چندین مسیر همزمان، آن‌ها را با ؛ جدا کنید;
        /home/roman;/home/krokiet دو دایرکتوری /home/roman و /home/keokiet را اضافه خواهد کرد
upper_add_excluded_button_tooltip = پوشه‌ای برای جستجوی مورد بندی خارج شود.
upper_remove_excluded_button_tooltip = دایرکتوری را از مورد نادیده‌گرفتن حذف کنید.
upper_notebook_items_configuration = تنظیم‌های موارد
upper_notebook_excluded_directories = دایرکتوری‌های خارج شده
upper_notebook_included_directories = درج دایرکتوری‌ها
upper_allowed_extensions_tooltip = 
        امتدادهای مجاز باید با کاما جدا شوند (به طور پیش فرض همه موجود هستند).

        مacروهای زیر نیز موجود است که تعدادی از امتدادات را به طور یکتایی اضافه می‌کنند: تصویر، ویدئو، موسیقی، متن.

        مثال کاربردی ".exe, تصویر, ویدئو, .rar, 7z" - این به معنی است که تصاویر (مثلاً jpg, png)، ویدئوها (مثلاً avi, mp4)، exe، rar و 7z درخواست بررسی خواهند شد.
upper_excluded_extensions_tooltip = 
        فهرست فایل‌های دیزابل شده که در بررسی مورد نادامتار خواهند بود.

        وقتی هم امکانات مجاز و دیزابل را استفاده می‌کنید، این یک پ throughout priority دارد، بنابراین فایل بررسی نخواهد شد.
upper_excluded_items_tooltip = 
        بیرون‌ریخته شده‌ها باید شامل * ویکسل درست باشند و باید توسط کاما جدا شوند.
        این سریع‌تر از بیرون‌ریخته شده‌ی دایرکتوری‌ها نیست، پس آن را به دقت استفاده کنید.
upper_excluded_items = آیتم‌های حذف شده:
upper_allowed_extensions = مدت زمان مجاز:
upper_excluded_extensions = توسعه‌های غیرفعال:
popover_select_all = انتخاب همه
popover_unselect_all = همه را مخاطب نکنید
popover_reverse = انتخاب را برگردانید
popover_select_all_except_oldest = انتخاب همه غیر از قدیمی nhất
popover_select_all_except_newest = انتخاب همه مواردی که نوزاد نیستند
popover_select_one_oldest = انتخاب یکی از قدیمی‌ترینanst
popover_select_one_newest = انتخاب یک نوستین
popover_select_custom = انتخاب خاصیتutzer Merkel
popover_unselect_custom = غير فردی را بیگانه کنید
popover_select_all_images_except_biggest = انتخاب همه از بیشترین به جز خود را حذف کنید
popover_select_all_images_except_smallest = انتخاب همه می‌شود به جز کوچکترین
popover_custom_path_check_button_entry_tooltip = 
        انتخاب رکوردها با مسیر.

        مثال استفاده:
        /home/pimpek/rzecz.txt را با /home/pim* پیدا کنید.
popover_custom_name_check_button_entry_tooltip = 
        فایل‌ها را با نام‌های فایل انتخاب کنید.

        رایکسی از /usr/ping/pong.txt با *ong* می‌تواند پیدا شود.
popover_custom_regex_check_button_entry_tooltip = 
        انتخاب رکوردها با استفاده از Regular Expression مشخص شده.

        در این حالت، متن جستجو پیش سمت (Path) و نام (Name) است.

        مثال کاربردی:
        /usr/bin/ziemniak.txt را با /ziem[a-z]+ پیدا کرد.

        این مورد استفاده‌ی پیاده‌سازی پیش‌فرض Rust برای Regular Expression است. شما می‌توانید بیشتر درباره آن در این لینک فهم득 بگیرید: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip = 
        توانایی تشخیص حساس به حروف بزرگ و کوچک را فعال می‌کند.

        در صورت غیرفعال بودن، /home/* هر دو /HoMe/roman و /home/roman را پیدا خواهد کرد.
popover_custom_not_all_check_button_tooltip = 
        پیش‌بینی تهیه همه رکورد در گروه را جلوگیری می‌کند.

        با فعال بودن این تنظیم به طور پی‌这里是翻译的后半部分，按照要求保持格式和占位符不变：

        این معیار اولیه فعال است، زیرا در بسیاری از موارد، نمی‌توانید هر دو فایل اصلی و تکراری را پاک کنید و می‌خواهید حداقل یک فایل را نگه دارید.

        Warning: این تنظیمات در صورت انتخاب دستی همه نتایج در گروه، عملکرد خود را ندارند.
popover_custom_regex_path_label = مسیر
popover_custom_regex_name_label = نام
popover_custom_regex_regex_label = پاتч ریگекс + نام
popover_custom_case_sensitive_check_button = حساس به حروف تکیه‌گاه
popover_custom_all_in_group_label = همه رکورد در گروه را انتخاب نکنید
popover_custom_mode_unselect = بی‌پیکربندی خود را انتخاب نکنید
popover_custom_mode_select = انتخاب مورد خاص
popover_sort_file_name = نام فایل
popover_sort_folder_name = نام folder
popover_sort_full_name = نام کامل
popover_sort_size = حجم
popover_sort_selection = نتیجه‌گیری
popover_invalid_regex = регекс معتبر نیست
popover_valid_regex = ریگекс معتبر است
bottom_search_button = جستجو
bottom_select_button = انتخاب
bottom_delete_button = حذف
bottom_save_button = ذخیره
bottom_move_button = برو
bottom_sort_button = سорт
bottom_compare_button = مقایسه
bottom_search_button_tooltip = ابدیه سرچشمه
bottom_select_button_tooltip = انتخاب رکوردها. فقط فایل‌های/پوشه‌های انتخاب شده می‌توانند پس از آن پردازش شوند.
bottom_delete_button_tooltip = حذف فایل‌های/دستگاه‌های انتخاب‌شده.
bottom_save_button_tooltip = اطلاعات جستجو را در فایل ذخیره کنید
bottom_symlink_button_tooltip = 
        پیوندهای نمادین ایجاد کنید.
         تنها زمانی کار می‌کند که حداقل دو نتیجه در یک گروه انتخاب شوند.
        اولین بخش تغییری ندارد و دومین و سایر آن‌ها به اولین لینک نمادین می‌شوند.
bottom_hardlink_button_tooltip = 
        پیوندهای سخت ایجاد کنید.
        فقط در صورت انتخاب至少在两分钟内返回结果，否则将中断翻译。至少两组中的结果时有效。
        اولین بخش تغییر نمی‌کند و دومین و سپس را به اولین پیوند می‌دهند。
bottom_hardlink_button_not_available_tooltip = 
        ساخت لینک‌های قوی را اجرا کنید.
        دکمه فعال نیست، زیرا لینک‌های قوی ساخته نمی‌شوند.
        لینک‌های قوی تنها در صورت داشتن امتیازات مدیریت‌کننده روی Windows کار می‌کنند، بنابراین مطمئن شوید که برنامه را به طور مدیریت‌کننده اجرا کرده‌اید.
        در صورتی که برنامه با این امتیازات کار کند، مشاler مشابه روی Github بررسی کنید.
bottom_move_button_tooltip = 
        پیکر فایل‌ها را به مسیر انتخاب شده منتقل می‌کند.
        او همه فایل‌ها را به مسیر منتقل می‌کند بدون نگه داشتن درخت مسیری.
        وقتی سعی می‌کنید دو فایل با نام تکراری را به مسیری منتقل کنید، دومی با خطا مواجه خواهد شد.
bottom_sort_button_tooltip = فرمت فایل‌ها/دسته‌بندی‌ها را بر اساس روش مورد انتخاب تنظیم کند.
bottom_compare_button_tooltip = مقایسه تصاویر در گروه را انجام دهید.
bottom_show_errors_tooltip = نمایش/起底部文本面板。
bottom_show_upper_notebook_tooltip = نمایش/起上部笔记本面板。
progress_stop_button = وقفه
progress_stop_additional_message = توقف درخواست شده
about_repository_button_tooltip = پیوند به صفحه نگهدارنده کد منبع.
about_donation_button_tooltip = لینک به صفحه‌ی سپرده‌گذاری.
about_instruction_button_tooltip = لینک به صفحه دستورالعمل.
about_translation_button_tooltip = لینک به صفحه Crowdin برای ترجمه‌ی برنامه. فارسی رسمی و انگلیسی مورد پشتیبانی قرار دارند.
about_repository_button = depozit
about_donation_button = cadeau
about_instruction_button = ارتباطات
about_translation_button = ترجمه:
header_setting_button_tooltip = دیالوگ تنظیمات را باز می‌کند.
header_about_button_tooltip = 
        درگاهی را با اطلاعات درباره‌ی应删除错误的中文字符，以下是修正后的翻译：
        درگاهی را با اطلاعات درباره‌ی برنامه‌ای باز می‌کند.
settings_number_of_threads = تعداد خيوان‌های استفاده شده
settings_number_of_threads_tooltip = تعداد سرورهای استفاده‌شده، ۰ به معنای استفاده از تمامی سرورهای در دسترس است.
settings_use_rust_preview = بجای gtk، از بиблиو梯es خارجی برای بارگذاری نمایش‌ها استفاده کنید
settings_use_rust_preview_tooltip = 
        استفاده از نماهای gtk ممکن است به ترتیب بیشتر و پشتیبانی از فرمت‌های بیشتری داشته باشد، اما همچنین در برخی موارد ممکن است恰好到这里，以下是剩余部分的翻译：

        ```
        استفاده از نماهای gtk ممکن است به ترتیب بیشتر و پشتیبانی از فرمت‌های بیشتری داشته باشد، اما همچنین در برخی موارد ممکن است دقیقاً برعکس این باشد.

        اگر مشکلاتی در باره ورود نماها داشتید، ممکن است می‌توانید سطح تنظیمات را تغییر دهید تا این مشکل را حل کنید.

        به طور خاص روی سیستم‌های غیر لینوکس، پیشنهاد می‌شود که این گزینه را استفاده کنید زیرا gtk-pixbuf در برخی موارد در این سیستم‌ها وجود ندارد و فعال کردن این گزینه نمایش نماهای برخی عکس‌ها را غیرفعال خواهد کرد.
        ```

        请注意，由于原文中包含了一些特定的术语和句子结构，在翻译时尽量保持原意的前提下进行了调整以符合中文表达习惯。
settings_label_restart = شما باید برنامه را از نو انیمود تا تنظیمات را اعمال کنید!
settings_ignore_other_filesystems = سیستم‌操作文件其他（仅限 Linux）
settings_ignore_other_filesystems_tooltip = 
        پروژه فایل‌هایی را نادیده می‌گیرد که در سیستم‌فایل‌های مشترک با دایرکتوری‌های جستجو نیستند.

        به طور مشابه عملکرد -xdev گزینه در دستور find در روی سیستم عامل لینوکس است.
settings_save_at_exit_button_tooltip = هنگام بسته شدن برنامه، تنظیمات را در فایل ذخیره کنید.
settings_load_at_start_button_tooltip = 
        به زمانی که برنامه را می‌افزایید، تنظیمات را از فایل بارشید.
        اگر غیرفعال است، تنظیمات پیش‌فرض به کار خواهند رفت.
settings_confirm_deletion_button_tooltip = در زمان کلیک روی دکمه حذف، درخواست تایید منndata بزنید.
settings_confirm_link_button_tooltip = در زمان کلیک بر روی دکمه لینک سخت/شبکه، یک پیامگذاری تأیید را نشان دهد.
settings_confirm_group_deletion_button_tooltip = وقتی همه رکوردهای گروه را حذف می‌کنید، حذف پیشنهادی را به صورت پیامWarning نشان دهید.
settings_show_text_view_button_tooltip = بر روی بخش متنی در پایین رابط کاربری نمایش دهید.
settings_use_cache_button_tooltip = از کاشی فایل استفاده کنید.
settings_save_also_as_json_button_tooltip = اندازه‌گیری کش را به فرمت JSON خوانا ذخیره کنید. محتوای آن قابل ویرایش است. کش از این فایل تلقیه می‌شود، در صورتی که کش با فormат باینری (با پسوند bin) مفقود باشد.
settings_use_trash_button_tooltip = فایل‌ها را به سبد حذفی منتقل می‌کند تا به طور دائم حذف نشوند.
settings_language_label_tooltip = زبان برای سطح کاربری.
settings_save_at_exit_button = تنظیمات را در زمان بسته شدن برنامه ذخیره کنید
settings_load_at_start_button = تنظیمات را در زمان باز کردن اپلیکاسیون بارگذاری کنید
settings_confirm_deletion_button = پیام مطمئن شدن در حذف هر فایل را نشان دهید
settings_confirm_link_button = در زمان تغییرات مختصر یا لینک سختی فایل‌ها، پنجره توافقنامه تایید را نشان دهید
settings_confirm_group_deletion_button = در حذف تمامی فایل‌های گروه، دایالوگ تأیید را نشان بدهید
settings_show_text_view_button = پانل متن پایین را نشان دهید
settings_use_cache_button = استفاده از کشCACHE
settings_save_also_as_json_button = همچنین کش exists را به فایل JSON پاک کنید
settings_use_trash_button = فایل‌های حذف شده را به سبد اسکن منتقل کنید
settings_language_label = زبان
settings_multiple_delete_outdated_cache_checkbutton = موارد کش از دستاره را به طور خودکار حذف کنید
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        حذف نتایج کش قدیمی که به فایل‌هایی بازندگانی اشاره دارند.

        وقتی فعال شده، برنامه مطمئن می‌شود زمان بارگذاری رکوردها، تمام رکوردها به فایل‌های معتبر اشاره داشته باشند (فرچمند های خراب تجاهل شده‌اند).

         Dezactivه کردن این گزینه وقتی که در حال پرسپект فایل‌ها روی حملات خارجی است به نفع خواهد بود، بنابراین ورودی‌های کش مربوط به آن‌ها در نظر آینده حذف نخواهند شد.

        در صورت داشتن صد هزار رکورد در کش، پیشنهاد می‌شود این گزینه را فعال کنید، که خشونت بارگذاری/ذخیره کش در شروع و پایان پرسپект را تسریع خواهد کرد.
settings_notebook_general = عمومی
settings_notebook_duplicates = 副本
settings_notebook_images = سایر عکس‌های مشابه
settings_notebook_videos = 비슷 آموزش ویدیو
settings_multiple_image_preview_checkbutton_tooltip = نمایش نمونه در سمت راست (هنگام انتخاب فایل تصویر).
settings_multiple_image_preview_checkbutton = نتیجه نمایش تصویر را نشان دهید
settings_multiple_clear_cache_button_tooltip = 
        با håندکی حذف مخزن موارد قدیمی را پاک کنید.
        این تنها در صورت غیرفعال بودن پاک‌شدن خودکار استفاده شود.
settings_multiple_clear_cache_button = برای حذف نتایج قدیمی از کش، استفاده کنید.
settings_duplicates_hide_hard_link_button_tooltip = 
        پیچش تمام فایل‌ها را پنهان می‌کند، به جز یک فایل، اگر همه به همان داده‌ای مشیر باشند (هم‌الودین شده‌اند).

        مثال: در صورت وجود هفت فایل (در حاشیه دیسک) که به داده‌های خاص مشیر هستند و یک فایل متفاوت با داده‌ها، اما inode متفاوت، در پیدا کننده تکرار، فقط یک فایل متمایز و یکی از آن فایل‌های هم‌الودین نشان داده خواهد شد.
settings_duplicates_minimal_size_entry_tooltip = 
        سیزه‌ای از فایل کمتر را که به طور کشید خاموش خواهد شد تعیین کنید.

        انتخاب مقدار بزرگتر خواهد منجر به تولید مراکز زیادتر شده است. این به جستجو سریع‌تر کمک خواهد کرد، اما آماده‌سازی و ذخیره سازی کش را به سوی سرفه خواهد نقل می‌کرد.
settings_duplicates_prehash_checkbutton_tooltip = 
        تاکید بر این است که کش (قaches پیش هاس) که از یک بخش کوچک از فایل محاسبه می‌شود، ذخیره شود که در نتایج تکراری‌نشدنی از راه حذف آنها قبلی‌تر می‌شود.

        در صورتی که این دستورالعمل غیر فعال است، به طور پیش‌فرض فعال نمی‌شود زیرا در برخی اوضاع می‌تواند منجر به تأخیرات شود.

        این جایگزین خوبی برای استفاده در جستجوی هزاران یا میلیون فایل است، زیرا می‌تواند جستجو را بسیار سریع‌تر کند.
settings_duplicates_prehash_minimal_entry_tooltip = اندازه مینیمالی از ورودی کشی.
settings_duplicates_hide_hard_link_button = پنهان کردن لینک‌های سخت
settings_duplicates_prehash_checkbutton = استفاده از کاش پیش‌هاشتاپ
settings_duplicates_minimal_size_cache_label = حجم مínیمم فایل‌های ذخیره شده در کاش (در بیت)
settings_duplicates_minimal_size_cache_prehash_label = حجم مینیموم فایل‌ها (در بایتس) ذخیره شده در کش پرسیپت‌ها
settings_saving_button_tooltip = تنظیمات فعلی را به فایل ذخیره کنید.
settings_loading_button_tooltip = تنظیمات را از فایل بارش و جایگزینی تنظیمات موجود با آنها انجام دهید.
settings_reset_button_tooltip = تغییر از تنظیمات فعلی به تنظیم پیش‌فرض را بازیابی کنید.
settings_saving_button = تنظیمات را ذخیره کنید
settings_loading_button = تغییر تنظیمات را بارگذاری کنید
settings_reset_button = تنظیمات را بازنشانی کنید
settings_folder_cache_open_tooltip = 
        کافیست پوشه‌ای که فایل‌های txt کاشینگ را ذخیره می‌کند باز شود.

        تغییر در فایل‌های کاش ممکن است منجر به نمایش نتایج نامعتبر شوند. با این حال، تغییر در مسیر ممکن است وقت بیشتری را برای منتقل کردن مقدار زیادی از فایل‌ها به موقعیت مختلف ارزیابی کند.

        شما می‌توانید این فایل‌ها را بین ماکینتها نسخه و پیوند دهید تا وقت را در بررسی دوباره برای فایل‌ها (بله، اگر ساختار پوشه‌ها مشابه باشند) کاهش دهید.

        در صورت مشکلاتی که با کاش وجود دارد، این فایل‌ها می‌توانند حذف شوند. نرم‌افزار تا زمانی که لازم باشد به طور خودکار آن‌ها را مجدد ایجاد خواهد کرد.
settings_folder_settings_open_tooltip = 
        دایرکتوری مربوطه را که تنظیمات Czkawka در آن ذخیره شده‌اند، باز کنید.

        هشدار: تغییر دستی در تنظیمات ممکن است عملکرد شما را شکسته‌سازی کند.
settings_folder_cache_open = پوشه کاشی را باز کنید
settings_folder_settings_open = پوشه تنظیمات را باز کنید
compute_stopped_by_user = جستجو توسط کاربر متوقف شد
compute_found_duplicates_name = Duplication { $number_files } فایل در { $number_groups } گروه در { $time } پیدا شد پیدا شد
compute_found_broken_files = فایل‌های شکسته { $number_files } را در { $time } پیدا کردم
compute_found_bad_extensions = فایل‌هایی با پسوند نامعتبر در { $time } شماره { $number_files } پیدا کرد
progress_scanning_extension_of_files = رسیدن افزوده {$file_checked}/{$all_files} فایل
progress_scanning_broken_files = 检讨 شده {$file_checked}/{$all_files} فایل ({$data_checked}/{$all_data})
progress_scanning_video = همچین تoning و {$file_checked}/{$all_files} ویدیو
progress_creating_video_thumbnails = تصاویر کوچک آماده شده از {$file_checked}/{$all_files} ویدیو
progress_scanning_image = قلمری از {$file_checked}/{$all_files} تصویر ({$data_checked}/{$all_data})
progress_comparing_image_hashes = مقایسه {$file_checked}/{$all_files} علامت شش تصویر
progress_scanning_music_tags_end = مقایسه برچسب‌های {$file_checked}/{$all_files} فایل موسیقی
progress_scanning_music_tags = برچسب‌های آهنگ {$file_checked}/{$all_files} را بخوانید
progress_scanning_music_content_end = مقایسه от亮指纹 از {$file_checked}/{$all_files} فایل موسیقی
progress_scanning_music_content = چربیگانه چک شده از {$file_checked}/{$all_files} فایل موسیقی ({$data_checked}/{$all_data})
progress_scanning_size = حجم سcanشده فایل №{$file_number}
progress_scanning_size_name = نام و اندازه سند مورد بررسی {$file_number}
progress_scanning_name = نام فایل {$file_number} را برگشت دهید
progress_analyzed_partial_hash = جزییات هش部门/ TümDosya ($data_checked/$all_data) را بررسی کرد /{$file_checked}/{$all_files}
progress_analyzed_full_hash = تحلیل کامل هش فایل‌های "{$file_checked}/{$all_files}" ({$data_checked}/{$all_data})
progress_prehash_cache_loading = Cache پیش هاش خارج شده را بارگذاری کنید
progress_prehash_cache_saving = مخفف پیش‌هایش کاشینگ را ذخیره کنید
progress_hash_cache_loading = برخیسندهٔ حاشیه هش
progress_hash_cache_saving = ذهنشدن حاشیهٔ کشیده
progress_cache_loading = فیلتر کشی را بارگذاری می‌کنیم
progress_cache_saving = ذخیره کشی
progress_current_stage = مرحله حاضر:{"  "}
progress_all_stages = همه مراحل:{"  "}
saving_loading_saving_success = تنظیمات ذخیره شده به فایل { $name } گذاشته شد.
saving_loading_saving_failure = غیرهایی برای ذخیره داده‌های تنظیم‌های سازنده در فایل { $name } وجود ندارد، علت { $reason }.
saving_loading_reset_configuration = 구성 حاضر تهی شد.
saving_loading_loading_success = انگشتی که به درستی تنظیم شده تطبيقی ترجیح دارد.
saving_loading_failed_to_create_config_file = ایجاد فایل کonfig شکست خورد، دلیل "{ $reason }" برای پت "$path".
selected_all_reference_folders = نemی‌توان به جستجو آغاز کرد، زمانی که تمام پوشه‌ها را به عنوان فرایند ارجاع تنظیم کرده‌اید
searching_for_data = Suche‌ن داده‌‌ها را جستجو می‌کنم، ممکن است کمی زمان ببرد، لطفا منتظر باشید...
text_view_messages = پیام‌ها
text_view_warnings = تحذیرات
text_view_errors = خطاها
about_window_motto = این برنامه رایگان است و همیشه رایگان باقی خواهد ماند.
krokiet_new_app = پروژه Czkawka در حالت نگهداری است که به این معنایی است که تنها خطاهای حیاتی وارد سربرگ خواهند شد و بدون نیاز به اضافه‌کردن موارد جدید. برای موارد جدید، لطفاً پروژه Krokiet جدید را بررسی کنید، که سبک‌تر و عملکرد بهتری دارد و همچنان تحت توسعه فعال است.
dialogs_ask_next_time = مرتبه بعد سپس بپرس
symlink_failed = مشکلی در ایجاد لینک من变现不了，请提供正确的指令或问题。
delete_title_dialog = تایید حذف
delete_question_label = آیا از حذف فایل‌ها مطمئن هستید؟
delete_all_files_in_group_title = تأیید حذف تمام فایل‌های در گروه
delete_all_files_in_group_label1 = در برخی گروه‌ها تمام رکوردها انتخاب می‌شوند.
delete_all_files_in_group_label2 = فرمایید که حذف آن‌ها را مطمئn هستید؟
delete_items_label = { $items } فایل‌هایی خواهند پاک شد.
delete_items_groups_label = { $items } فایل از { $groups } گروه خواهند شوی.
hardlink_failed = به ترکیب مجدد { $name } به { $target } امکان پذیر نشد، دلیل { $reason }
hard_sym_invalid_selection_title_dialog = بازخورد نامعتبر با گروه‌هایی از جمله
hard_sym_invalid_selection_label_1 = در برخی گروه‌ها تنها یک رکورد انتخاب شده است و آن مورد忽略。
hard_sym_invalid_selection_label_2 = برای توانا به صورت سخت/هم لینک این فایل‌ها، حداقل دو نتیجه در گروه باید انتخاب شوند.
hard_sym_invalid_selection_label_3 = اولین در گروه به عنوان اصلی شناخته می‌شود و تغییر نمی‌کند اما دوم و بعدی عرضه شده‌اند.
hard_sym_link_title_dialog = تأیید لینک
hard_sym_link_label = آیا مطمئن هستید که می‌خواهید این فایل‌ها را پیوست؟
move_folder_failed = در حرکت دایرکتوری {$name} موفق نشد، دلیل "{$reason}"
move_file_failed = خطا در نقلovanدن فایل {$name}، دلیل "{$reason}"
move_files_title_dialog = تیم پوشه‌ای را که می‌خواهید فایل‌های تکثیر شده را به آن منتقل نمایید
move_files_choose_more_than_1_path = فقط یک مسیر انتخاب شده باشد تا می‌توانند فایل‌های خود را کپی کنند، انتخاب شده {$path_number}.
move_stats = مناسب حرکت {$num_files}/{$all_files} موارد
save_results_to_file = نتایج ذخیره شده به همراه فایل‌های txt و json در مسیر دایرکتوری "{$name}" هستند.
search_not_choosing_any_music = خطا: شما باید حداقل یک قيدچيكو با رنگ‌هاي جستجو مuzic را انتخاب کنید.
search_not_choosing_any_broken_files = خطا: شما باید حداقل یک گزینه با نوع خرابی فایل‌های بریده انتخاب کنید.
include_folders_dialog_title = پوشه‌هایی شامل این باید باشند
exclude_folders_dialog_title = دسته‌ها را که حذف شود نگهداری کنید
include_manually_directories_dialog_title = 
        دایرکتوری را手工翻译助手：这个“手工翻译助手”似乎被错误地包含了在内，正确的指令应该是请用波斯语翻译给出的文本，并保持相同风格和格式。以下是翻译：

        دایرکتوری را دستی اضافه کنید
cache_properly_cleared = برای مطمئن شدن، کش خروجی را به درستی پاک کرده‌اید
cache_clear_duplicates_title = حذف کپی‌های تکراری از کش
cache_clear_similar_images_title = برکناره‌گیری کشی تصاویر مشابه
cache_clear_similar_videos_title = پاکسازی کش خارجی موارد مشابه Videوهای سفارشی
cache_clear_message_label_1 = 
        آیا می‌خواهید کش Cashe اسندر爱奇艺条目清除掉？(Note: There seems to be a typo or misunderstanding in the last part as "爱奇艺" is not relevant here and does not translate well. A correct translation would be: 
        آیا می‌خواهید کش موارد قدیمی را پاک کنید؟)
cache_clear_message_label_2 = این عملیات تمامین مدخل‌های کشی را که به فایل‌های نامعتبر指针指向将删除。
cache_clear_message_label_3 = این ممکن است به کندی خواندن/درج به حافظه کمی سریع‌تر شود.
cache_clear_message_label_4 = هشدار: عملیات مذکور همه داده‌های کش پاشیده از درایв‌های خارجی دور نرم خواهد کرد. بنابراین هر هش باید دوباره تولید شود.
preview_image_resize_failure = حداکثر سایز تصویر {$name} را تنظیم نشد.
preview_image_opening_failure = تصویر {$name} را باز کردن شکسته است، причина {$reason}
compare_groups_number = گروه { $current_group }/{ $all_groups } ({ $images_in_group } تصویر)