# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Дубликати
tool_empty_folders = Празни папки
tool_similar_images = Сходни изображения
tool_empty_files = Празни файлове
tool_temporary_files = Временни файлове
tool_big_files = Най-големите файлове
tool_broken_files = Повредени файлове
tool_bad_extensions = Повредени разширения
tool_same_music = Музикални дубликати
tool_bad_names = Лоши имена
tool_exif_remover = EXIF данни
tool_directories = Директории
tool_settings = Настройки
# Home screen tool card descriptions
home_dup_description = Намерете файлове със същото съдържание
home_empty_folders_description = Директории без съдържание
home_similar_images_description = Намерете визуално подобни снимки
home_empty_files_description = Файлове с нулев размер
home_temp_files_description = Временни и кеширани файлове
home_big_files_description = Най-големите/Най-малките файлове на диска
home_broken_files_description = PDF, аудио, изображения, архиви
home_bad_extensions_description = Файлове с невалидни разширения
home_same_music_description = Подобни аудио файлове, подредени по тагове
home_bad_names_description = Файлове с проблемни символи в името
home_exif_description = Изображения с EXIF метаданни
# Results list
scanning = Сканирането е в процес...
stopping = Спиране...
no_results = Няма намерени резултати
press_start = Натиснете START, за да сканирате
select_label = Изберете.
deselect_label = Desel.
list_label = Списък
gallery_label = Гал.
# Selection popup
selection_popup_title = Избери
select_all = Избери всички
select_except_one = Изберете всички, с изключение на един
select_except_largest = Изберете всички, с изключение на най-големия
select_except_smallest = Избери всички освен най-малкия
select_largest = Изберете най-големия
select_smallest = Изберете най-малкия
select_except_highest_res = Изберете всички, с изключение на опцията с най-висока резолюция
select_except_lowest_res = Изберете всички, с изключение на опцията с най-ниска резолюция
select_highest_res = Изберете най-високата резолюция
select_lowest_res = Изберете най-ниското разрешение
invert_selection = Инверсия на избора
close = Затвори
# Deselection popup
deselection_popup_title = Премахнете селекцията
deselect_all = Отключи всичко
deselect_except_one = Премахнете всички опции, с изключение на една
# Confirm popup
cancel = Отказ
delete = Изтрий
rename = Преименувай
# Delete errors popup
delete_errors_title = Неуспешно изтрити някои файлове:
ok = Добре
# Stopping overlay
stopping_overlay_title = Спиране
stopping_overlay_body = 
        Завършване на текущото сканиране...
        Моля, изчакайте.
# Permission popup
permission_title = Достъп до файлове
permission_body = За да сканира файлове, приложението се нуждае от достъп до паметта на устройството. Без това разрешение, сканирането няма да бъде възможно.
grant = Безвъзмездна помощ/Безвъзмездно финансиране
no_permission_scan_warning = Липсва достъп до файлове - предоставете разрешение за сканиране
# Settings screen tabs
settings_tab_general = Общи
settings_tab_tools = Инструменти
settings_tab_diagnostics = Информация
# Settings - General tab
settings_use_cache = Използвай кеш
settings_use_cache_desc = Ускорява следващите сканирания (на хешове/изображения)
settings_ignore_hidden = Игнорирайте скритите файлове
settings_ignore_hidden_desc = Файлове и папки, чиито имена започват с '.'
settings_show_notification = Уведомете, когато сканирането приключи
settings_show_notification_desc = Показвайте системно известие при завършване на сканирането
settings_notify_only_background = Показвайте само когато е на заден план
settings_notify_only_background_desc = Пропуснете известието, ако приложението е видимо
notifications_disabled_banner = Известията са деактивирани
notifications_enable_button = Активирай
settings_scan_label = СКАНИРАЙ
settings_filters_label = ФИЛТРИ (някои инструменти)
settings_min_file_size = Минимален размер на файла
settings_max_file_size = Макс. размер на файла
settings_language = Език
settings_language_restart = Изисква рестартиране на приложението
settings_common_label = ОБЩИ НАСТРОЙКИ
settings_excluded_items = ИЗКЛЮЧЕНИ ЕЛЕМЕНТИ (глобални шаблони, разделени със запетая)
settings_excluded_items_placeholder = Например: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = ПОЗВОЛЕНИТЕ РАЗШИРЕНИЯ (празно = всички)
settings_allowed_extensions_placeholder = Например: jpg, png, mp4
settings_excluded_extensions = ИЗКЛЮЧЕНИ РАЗШИРЕНИЯ
settings_excluded_extensions_placeholder = Например: bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = ДУБЛИРАНИ ОБЕКТИ
settings_check_method_label = МЕТОД НА СРАВНЕНИЕ
settings_check_method = Метод
settings_hash_type_label = ТИП НА ХЕША
settings_hash_type = Хеш тип
settings_hash_type_desc = Blake3 – това е препоръчителният вариант, тъй като CRC32 има малък шанс за фалшиво положителни резултати
settings_similar_images_header = ПОДОБНИ СНИМКИ
settings_similarity_preset = Праг на сходство
settings_similarity_desc = Много високо качество = само почти идентични
settings_hash_size = Хеш размер
settings_hash_size_desc = По-големите размери имат по-малко фалшиви положителни резултати, но също така намират по-малко подобни изображения
settings_hash_alg = Хеш алгоритъм
settings_image_filter = Промяна на размера на филтъра
settings_ignore_same_size = Пренебрегвайте изображенията със същите размери
settings_gallery_image_fit_cover = Галерия: изрязване до квадратен формат
settings_gallery_image_fit_cover_desc = Запълнете плочката; деактивирайте тази опция, за да запазите оригиналното съотношение на страните
settings_big_files_header = НАЙ-ГОЛЕМИТЕ ФАЙЛОВЕ
settings_search_mode = Режим на търсене
settings_file_count = Брой файлове
settings_same_music_header = МУЗИКАЛНИ КОПИЯ
settings_music_check_method = Режим на сравнение
settings_music_compare_tags_label = СВЪРЗАНИ ЕТИКЕТИ
settings_music_title = Заглавие
settings_music_artist = Изпълнител
settings_music_year = Година
settings_music_length = Дължина
settings_music_genre = Жанр
settings_music_bitrate = Битрейт
settings_music_approx = Приблизително сравнение на тагове
settings_broken_files_header = ПОЩУПЕНИ ФАЙЛОВЕ
settings_broken_files_note = Сканирането изисква значителни ресурси. За оптимална производителност използвайте Krokiet на настолен компютър.
settings_broken_files_types_label = ПРОВЕРЕНИ ТИПОВЕ
settings_broken_audio = Аудио
settings_broken_pdf = PDF
settings_broken_archive = Архив
settings_broken_image = Изображение
settings_bad_names_header = ЛОШИ ИМЕНА
settings_bad_names_checks_label = ЧЕКОВЕ
settings_bad_names_uppercase_ext = Голяма буква
settings_bad_names_emoji = Емоджи в име
settings_bad_names_space = Пространства в началото/края
settings_bad_names_non_ascii = Не-ASCII символи
settings_bad_names_duplicated = Повтарящи се символи
# Settings - Diagnostics tab
diagnostics_header = ДИАГНОСТИКА
diagnostics_thumbnails = Кеш за миниатюри
diagnostics_app_cache = Кеш на приложението
diagnostics_refresh = Обновете
diagnostics_clear_thumbnails = Ясни миниатюри
diagnostics_open_thumbnails_folder = Отворете папката
diagnostics_clear_cache = Изчистване на кеша
diagnostics_open_cache_folder = Отворете папката
diagnostics_collect_test = Тест за достъп до файлове
diagnostics_collect_test_desc = Проверете колко файлове са достъпни
diagnostics_collect_test_run = Стартирай
diagnostics_collect_test_stop = Спри
collect_test_cancelled = Спиране, инициирано от потребител
diag_confirm_clear_thumbnails = Искате ли да изтриете всички кеширани миниатюри?
diag_confirm_clear_cache = Искате ли да изтриете кеша на всички приложения?
about_repo = Хранилище
about_translate = Преводи
about_donate = Поддръжка
# Collect-test result popup
collect_test_title = Резултати от тестовете
collect_test_volumes = Томове:
collect_test_folders = Папки:
collect_test_files = Файлове:
collect_test_time = Време:
# Licenses
licenses_label = ЛИЦЕНЗ
third_party_licenses = Лицензи на трети страни
licenses_popup_title = Лицензи от трети страни
# Directories screen
directories_include_header = Включете
directories_included = Включено
directories_exclude_header = Изключете
directories_excluded_header = Изключени
directories_add = Включете
no_paths = Няма пътища - добавете отдолу
directories_volume_header = Томове
directories_volume_refresh = Обновете
directories_volume_add = Добави
# Bottom navigation
nav_home = Старт
nav_dirs = Директории
nav_settings = Настройки
# Status messages set from Rust
status_ready = Готови
status_stopped = Спряно
status_no_results = Няма намерени резултати
status_deleted_selected = Избраното е изтрито
status_deleted_with_errors = Изтрито с грешки
scan_not_started = Сканирането не е започнало
found_items_prefix = Открито
found_items_suffix = артикули
deleted_items_prefix = Изтрито
deleted_items_suffix = продукти/стоки
deleted_errors_suffix = грешки
renamed_prefix = Преименувано
renamed_files_suffix = файлове
renamed_errors_suffix = грешки
cleaned_exif_prefix = Премахната EXIF информация от
cleaned_exif_suffix = файлове
cleaned_exif_errors_suffix = грешки
and_more_prefix = ...и
and_more_suffix = още
# Gallery / delete popups
gallery_delete_button = Изтрий
gallery_back = Назад
gallery_confirm_delete = Да, изтрий
deleting_files = Изтриване на файлове...
stop = Спри
files_suffix = файлове
scanning_fallback = Сканиране...
app_subtitle = В чест на битката при Чединя (972 г. сл. Хр.)
app_license = Интерфейс за потребителя за Czkawka Core - Лиценз GPL-3.0
about_app_label = ЗА
cache_label = КЕШ
# Notification
scan_completed_notification = Сканирането приключи - намерени { $file_count } елемента
# Confirm popups (set from Rust)
confirm_clean_exif = Сигурни ли сте, че искате да изтриете EXIF данните от { $n } избраните файлове?
confirm_delete_items = Сигурни ли сте, че искате да изтриете { $n } избрани елемента?
gallery_confirm_delete_msg = Вие ще изтриете { $total_images } изображения от { $total_groups } групи.
gallery_confirm_delete_warning = Всички елементи са избрани от { $unsafe_groups } групи!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Изчисляването и сравняването на аудио отпечатъци е много ресурсоемък процес и може да отнеме доста време. Препоръчва се използването на Krokiet на настолна система за тази задача.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Събиране на файлове
stage_scanning_name = Сканиране по име
stage_scanning_size_name = Сканиране по име и размер
stage_scanning_size = Сканиране по размер
stage_pre_hash = Предварително хеширане
stage_full_hash = Хеширане
stage_loading_cache = Зарежда кеш
stage_saving_cache = Запазва кеш
stage_calculating_image_hashes = Изчисляване на хешове за изображения
stage_comparing_images = Сравняване на изображения
stage_calculating_video_hashes = Изчисляване на хешове за видео
stage_checking_files = Проверка на файловете
stage_checking_extensions = Проверка на разширенията
stage_checking_names = Проверка на имена
stage_reading_music_tags = Четене на музикални тагове
stage_comparing_tags = Сравняване на тагове
stage_calculating_music_fingerprints = Изчисляване на музикални отпечатъци
stage_comparing_fingerprints = Сравнение на отпечатъци от пръсти
stage_extracting_exif = Четене на EXIF тагове
stage_creating_video_thumbnails = Създаване на миниатюри за видеа
stage_processing_videos = Обработка на видеоклипове
stage_deleting = Изтриване на файлове
stage_renaming = Преименуване на файлове
stage_moving = Преместване на елементи
stage_hardlinking = Създаване на твърди връзки
stage_symlinking = Създаване на символни връзки
stage_optimizing_videos = Оптимизиране на видеа
stage_cleaning_exif = Почистване на EXIF данните
# Group headers in scan results
duplicates_group_header = { $count } файла x { $per_file } / файл = { $total } общо
similar_images_group_header = { $count } подобни изображения
same_music_group_header = { $count } подобни записи
# Rename confirmation
confirm_rename_items = Сигурни ли сте, че искате да преименувате избраните { $n } файла?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Най-голям
option_search_mode_smallest = Най-малък
option_similarity_very_high = V. Много висока
option_similarity_high = Високо
option_similarity_medium = Средно
option_similarity_low = Ниско
option_similarity_very_low = Много ниско
option_similarity_minimal = Мин.
option_check_method_hash = Хеш
option_check_method_name = Име
option_check_method_size_and_name = Размер + Име
option_check_method_size = Размер
option_music_method_tags = Етикети
option_music_method_audio = Аудио
option_min_size_none = Няма
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Неограничен
# Volume labels (shown in the directories screen)
volume_internal_storage = Вътрешна памет
volume_sd_card = Карта памет (SD карта)
volume_storage = Обем на паметта
# Directories screen
directories_referenced_tooltip = Препратено (не е изтрито)
directories_include_section_header = ВКЛЮЧЕНО
directories_exclude_section_header = ИЗКЛЮЧЕНО
directories_custom_paths = Персонализирани пътища
directories_check_button = Анализирайте
directories_check_popup_title = Статистика на директорията
directories_check_label_included = Включени пътища:
directories_check_label_excluded = Изключени пътища:
directories_check_label_referenced = Референтни пътища:
directories_check_label_would_scan = Файлове за сканиране:
directories_check_label_processable = Обработваеми файлове:
directories_check_scanning = Сканиране...
directories_check_warning_no_processable = Не са открити файлове, които могат да бъдат обработени - проверете включените и изключените папки
path_edit_title_include = Добави към "Включени"
path_edit_title_exclude = Добави към списъка за изключване
path_edit_placeholder = Въведете пътя...
path_edit_not_exists = Пътят не съществува
path_edit_is_dir = Директория
path_edit_is_file = Файл
path_edit_no_newlines = Пътищата не могат да съдържат нови редове – натискането на клавиша "Enter" не е разрешено
ctx_menu_title = Отворено
ctx_open_file = Отворена точка
ctx_open_folder = Отворете родителската папка
