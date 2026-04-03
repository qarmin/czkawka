# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Дублює
tool_empty_folders = Порожні теки
tool_similar_images = Схожі зображення
tool_empty_files = Порожні файли
tool_temporary_files = Тимчасові файли
tool_big_files = Найбільші файли
tool_broken_files = Пошкоджені файли
tool_bad_extensions = Помилкові розширення
tool_same_music = Музичні дублікати
tool_bad_names = Помилкові імена
tool_exif_remover = Дані EXIF
tool_directories = Каталоги
tool_settings = Налаштування
# Home screen tool card descriptions
home_dup_description = Знайти файли з тим самим вмістом
home_empty_folders_description = Каталоги без вмісту
home_similar_images_description = Пошук візуально схожих фотографій
home_empty_files_description = Файли з нульовим розміром
home_temp_files_description = Тимчасові і кешовані файли
home_big_files_description = Найгірші файли на диску
home_broken_files_description = PDF, аудіо та зображення, архіви
home_bad_extensions_description = Файли з неправильним розширенням
home_same_music_description = Схожі аудіо файли за тегами
home_bad_names_description = Файли з логічними символами в імені
home_exif_description = Зображення з EXIF метаданими
# Results list
scanning = Виконується сканування...
stopping = Зупинення...
no_results = Немає результатів
press_start = Натисніть СТАРТ для сканування
select_label = Від.
deselect_label = Desel.
list_label = Список
gallery_label = Галь.
# Selection popup
selection_popup_title = Вибрати
select_all = Виділити все
select_except_one = Вибрати всі крім однієї
select_except_largest = Виділити всі, крім найбільшого
select_except_smallest = Вибрати всі крім найменших
select_largest = Виберіть найбільший
select_smallest = Виберіть найменший
select_except_highest_res = Вибрати всі крім найвищої роздільної здатності
select_except_lowest_res = Вибрати всі крім найнижчої роздільної здатності
select_highest_res = Виберіть найкращу роздільну здатність
select_lowest_res = Виберіть найнижчу роздільну здатність
invert_selection = Інвертувати виділення
close = Закрити
# Deselection popup
deselection_popup_title = Зняти виділення
deselect_all = Зняти всі виділення
deselect_except_one = Зняти виділення з усього крім одного
# Confirm popup
cancel = Скасувати
delete = Видалити
rename = Перейменувати
# Delete errors popup
delete_errors_title = Не вдалося видалити деякі файли:
ok = Гаразд
# Stopping overlay
stopping_overlay_title = Зупиняю
stopping_overlay_body =
    Завершення поточного сканування...
    Будь ласка, зачекайте.
# Permission popup
permission_title = Доступ до файлу
permission_body = Для сканування файлів потрібен доступ до пам'яті пристрою. Без цього дозволу сканування буде неможливим.
grant = Грант
no_permission_scan_warning = Немає доступу до файлу - надайте дозвіл для сканування
# Settings screen tabs
settings_tab_general = Загальні налаштування
settings_tab_tools = Інструменти
settings_tab_diagnostics = Інформація
# Settings - General tab
settings_use_cache = Використання кешу
settings_use_cache_desc = Прискорює наступні скани (хеш/зображення)
settings_ignore_hidden = Ігнорувати приховані файли
settings_ignore_hidden_desc = Файли та теки починаються з '.'
settings_show_notification = Сповіщати при завершенні сканування
settings_show_notification_desc = Показувати системні сповіщення при завершенні сканування
settings_notify_only_background = Тільки коли у фоновому режимі
settings_notify_only_background_desc = Пропустити сповіщення, якщо програма видима
notifications_disabled_banner = Сповіщення вимкнено
notifications_enable_button = Увімкнено
settings_scan_label = СКАНУВАТИ
settings_filters_label = FILTERS (деякі інструменти)
settings_min_file_size = Мінімальний розмір файлу
settings_max_file_size = Макс. розмір файлу
settings_language = Мова:
settings_language_restart = Потрібно перезапустити програму
settings_common_label = ЗАГАЛЬНІ НАЛАШТУВАННЯ
settings_excluded_items = ЕКСКЛАДЕНІ РЕЧІ (glob шаблони, кома розділені)
settings_excluded_items_placeholder = напр. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = ДОЗВОЛЕНІ ДОЗВОЛЕННЯ (порожнє = всі)
settings_allowed_extensions_placeholder = напр. jpg, png, mp4
settings_excluded_extensions = РОЗШИРЕНІ ДОДАТКОВІ ЗАПИСИ
settings_excluded_extensions_placeholder = напр. bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = ВИМКНУТИ
settings_check_method_label = КОМЕЙНИЙ СПАРИЖ
settings_check_method = Метод
settings_hash_type_label = ТИП ГОЛОВУ
settings_hash_type = Тип хешу
settings_hash_type_desc = Blake3 - рекомендована опція, CRC32 має невеликий шанс помилкових додатніх
settings_similar_images_header = ЗОБРАЖЕННЯ SIM
settings_similarity_preset = Поріг подібності
settings_similarity_desc = Дуже висока = тільки майже ідентична
settings_hash_size = Розмір хешу
settings_hash_size_desc = Великі розміри мають менше помилкових додатніх, але також менш схожі зображення
settings_hash_alg = Алгоритм хеша
settings_image_filter = Змінити розмір фільтру
settings_ignore_same_size = Ігнорувати зображення з такими ж розмірами
settings_gallery_image_fit_cover = Галерея: обрізати в квадрат
settings_gallery_image_fit_cover_desc = Заповніть плитку; вимкнути щоб зберегти оригінальне співвідношення сторін
settings_big_files_header = СВІТЛОВІ ФАЙЛИ
settings_search_mode = Режим пошуку
settings_file_count = Кількість файлів
settings_same_music_header = ОПИС МУЗИКУ
settings_music_check_method = Режим порівняння
settings_music_compare_tags_label = ПОЗНАЧЕНІ ТАГИ (ДЛЯ ПОРІВНЯННЯ)
settings_music_title = Найменування
settings_music_artist = Художник
settings_music_year = Рік
settings_music_length = Довжина
settings_music_genre = Жанр
settings_music_bitrate = Бітрейт
settings_music_approx = Приблизне порівняння тегів
settings_broken_files_header = АКТИВНІ ФАЙЛИ
settings_broken_files_note = Відстеження ресурсів. Для кращої продуктивності використовуйте Krokiet на робочому столі.
settings_broken_files_types_label = РЕЄСТРАЦІЯ
settings_broken_audio = Аудіо
settings_broken_pdf = Файл pdf
settings_broken_archive = Архів
settings_broken_image = Зображення
settings_bad_names_header = НАЗВА БАД
settings_bad_names_checks_label = ПЕРЕВІРИТИ
settings_bad_names_uppercase_ext = Верхнє розширення
settings_bad_names_emoji = Назва емодзі
settings_bad_names_space = Пробіли на початку/кінці
settings_bad_names_non_ascii = Не-ASCII символи
settings_bad_names_duplicated = Символи, що повторюються
# Settings - Diagnostics tab
diagnostics_header = ДІАГНОСТИКА
diagnostics_thumbnails = Кеш мініатюр
diagnostics_app_cache = Кеш додатків
diagnostics_refresh = Оновити
diagnostics_clear_thumbnails = Очистити мініатюри
diagnostics_open_thumbnails_folder = Відкрити папку
diagnostics_clear_cache = Очистити кеш
diagnostics_open_cache_folder = Відкрити папку
diagnostics_collect_test = Тестування доступу до файлу
diagnostics_collect_test_desc = Перевірте кількість доступних файлів
diagnostics_collect_test_run = Ран
diagnostics_collect_test_stop = Зупинити
collect_test_cancelled = Зупинено користувачем
diag_confirm_clear_thumbnails = Очистити кеш мініатюр ?
diag_confirm_clear_cache = Очистити кеш усіх застосунків?
about_repo = Репозиторій
about_translate = Переклади
about_donate = Підтримка
# Collect-test result popup
collect_test_title = Результати тестування
collect_test_volumes = Гучність:
collect_test_folders = Теки:
collect_test_files = Файли:
collect_test_time = Час:
# Licenses
licenses_label = ЛІЦЕНЗІЯ
third_party_licenses = Сторонні ліцензії
licenses_popup_title = Сторонні ліцензії
# Directories screen
directories_include_header = Включити
directories_included = Включено
directories_exclude_header = Не включати
directories_excluded_header = Виключені
directories_add = Включити
no_paths = Відсутні шляхи - додайте нижче
directories_volume_header = Томи
directories_volume_refresh = Оновити
directories_volume_add = Додати
# Bottom navigation
nav_home = Старт
nav_dirs = Каталоги
nav_settings = Налаштування
# Status messages set from Rust
status_ready = Готовий
status_stopped = Зупинено
status_no_results = Немає результатів
status_deleted_selected = Видалено
status_deleted_with_errors = Видалено з помилками
scan_not_started = Сканування не почалося
found_items_prefix = Знайдено
found_items_suffix = елементи
deleted_items_prefix = Видалено
deleted_items_suffix = елементи
deleted_errors_suffix = помилки
renamed_prefix = Перейменовано
renamed_files_suffix = файлів
renamed_errors_suffix = помилки
cleaned_exif_prefix = Очищено EXIF від
cleaned_exif_suffix = файлів
cleaned_exif_errors_suffix = помилки
and_more_prefix = ... і
and_more_suffix = більше
# Gallery / delete popups
gallery_delete_button = Видалити
gallery_back = Відмінити
gallery_confirm_delete = Так, видалити
deleting_files = Видалення файлів...
stop = Зупинити
files_suffix = файлів
scanning_fallback = Сканування...
app_subtitle = Пошанування битви в Сединії (972 р.)
app_license = Інтерфейс для Czkawka Core - GPL-3.0
about_app_label = Про
cache_label = КЕШ
# Notification
scan_completed_notification = Сканування завершено - { $file_count } знайдено елементів
# Confirm popups (set from Rust)
confirm_clean_exif = Ви дійсно хочете очистити EXIF мітки { $n } вибраних файлів?
confirm_delete_items = Ви впевнені, що хочете видалити { $n } виділених елементів?
gallery_confirm_delete_msg = Ви збираєтесь видалити { $total_images } зображень у { $total_groups } групі.
gallery_confirm_delete_warning = Всі елементи обрані в { $unsafe_groups } групи!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Обчислення та порівняння аудіо-відбитків є дуже ресурсоємними і може зайняти тривалий час. Рекомендується використовувати Krokiet на настільному комп'ютері для цього завдання.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Збирання файлів
stage_scanning_name = Сканування по імені
stage_scanning_size_name = Сканування по імені та розміру
stage_scanning_size = Сканування за розміром
stage_pre_hash = Попереднє хешування
stage_full_hash = Хеш
stage_loading_cache = Завантаження кешу
stage_saving_cache = Збереження кешу
stage_calculating_image_hashes = Обчислення хешів зображення
stage_comparing_images = Порівняння зображень
stage_calculating_video_hashes = Обчислення хешів відео
stage_checking_files = Перевірка файлів
stage_checking_extensions = Перевірка розширень
stage_checking_names = Перевірка імен
stage_reading_music_tags = Читання музичних тегів
stage_comparing_tags = Порівняння міток
stage_calculating_music_fingerprints = Обчислення відбитків музики
stage_comparing_fingerprints = Порівняння відбитків
stage_extracting_exif = Читання EXIF тегів
stage_creating_video_thumbnails = Створення мініатюр відео
stage_processing_videos = Обробка відео
stage_deleting = Видалення файлів
stage_renaming = Перейменування файлів
stage_moving = Переміщення файлів
stage_hardlinking = Створення жорстких посилань
stage_symlinking = Створення символічних посилань
stage_optimizing_videos = Оптимізація відео
stage_cleaning_exif = Очищення EXIF
# Group headers in scan results
duplicates_group_header = { $count } файлів x { $per_file } / файл = { $total } всього
similar_images_group_header = { $count } схожих зображень
same_music_group_header = { $count } схожих маршрутів
# Rename confirmation
confirm_rename_items = Ви впевнені, що хочете перейменувати { $n } вибраних файлів?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Найбільший
option_search_mode_smallest = Найменший
option_similarity_very_high = Вертикальний
option_similarity_high = Високий
option_similarity_medium = Медіум
option_similarity_low = Низька
option_similarity_very_low = В.Низька
option_similarity_minimal = Мін.
option_check_method_hash = Хеш
option_check_method_name = Ім'я
option_check_method_size_and_name = Розмір+Ім'я
option_check_method_size = Розмір
option_music_method_tags = Мітки
option_music_method_audio = Аудіо
option_min_size_none = Без ефекту
option_min_size_1kb = 1 КБ
option_min_size_8kb = 8 Кб
option_min_size_64kb = 64 КБ
option_min_size_1mb = 1 Мб
option_max_size_16kb = 16 Кб
option_max_size_1mb = 1 Мб
option_max_size_10mb = 10 Мб
option_max_size_100mb = 100 МБ
option_max_size_unlimited = Необмежено
# Volume labels (shown in the directories screen)
volume_internal_storage = Внутрішня пам’ять
volume_sd_card = Карта пам'яті (SD карта)
volume_storage = Гучність сховища
# Directories screen
directories_referenced_tooltip = Посилання (не видалено)
directories_include_section_header = ВКЛЮЧЕНО
directories_exclude_section_header = ЗАКЛЮЧЕНО
directories_custom_paths = Користувацькі шляхи
directories_check_button = Аналізувати
directories_check_popup_title = Статистика каталогів
directories_check_label_included = Включені шляхи:
directories_check_label_excluded = Виключені шляхи:
directories_check_label_referenced = Шлях до посилання:
directories_check_label_would_scan = Файлів для сканування:
directories_check_label_processable = Файли для обробки:
directories_check_scanning = Сканування...
directories_check_warning_no_processable = Не знайдено жодного файлу, що обробляється - перевірте ваші включені/виключені теки
path_edit_title_include = Додати до включення
path_edit_title_exclude = Додати до виключення
path_edit_placeholder = Введіть шлях...
path_edit_not_exists = Шлях не існує
path_edit_is_dir = Каталог
path_edit_is_file = Файл
path_edit_no_newlines = Шляхи не можуть містити нових рядків — Enter не дозволено
ctx_menu_title = Відкриті
ctx_open_file = Відкрити елемент
ctx_open_folder = Відкрити батьківську теку
