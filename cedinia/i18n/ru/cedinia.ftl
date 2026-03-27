# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Дубликаты
tool_empty_folders = Пустые папки
tool_similar_images = Похожие изображения
tool_empty_files = Пустые файлы
tool_temporary_files = Временные файлы
tool_big_files = Крупнейшие файлы
tool_broken_files = Сломанные файлы
tool_bad_extensions = Плохие расширения
tool_same_music = Музыкальные дубликаты
tool_bad_names = Неверные имена
tool_exif_remover = Данные EXIF
tool_directories = Каталоги
tool_settings = Настройки
# Home screen tool card descriptions
home_dup_description = Найти файлы с одним и тем же контентом
home_empty_folders_description = Директории без содержания
home_similar_images_description = Найти похожие на визуальное фото
home_empty_files_description = Файлы с нулевым размером
home_temp_files_description = Временные и кэшированные файлы
home_big_files_description = Крупнейшие/маленькие файлы на диске
home_broken_files_description = PDF, аудио, изображения, архивы
home_bad_extensions_description = Файлы с недопустимым расширением
home_same_music_description = Похожие аудио файлы по тегам
home_bad_names_description = Файлы с проблемными символами в имени
home_exif_description = Изображения с метаданными EXIF
# Results list
scanning = Идет сканирование...
stopping = Остановка...
no_results = Нет результатов
press_start = Нажмите НАЧАТЬ для сканирования
select_label = Сел.
deselect_label = Desel.
list_label = Список
gallery_label = Гал.
# Selection popup
selection_popup_title = Выбрать
select_all = Выбрать все
select_except_one = Выделить все, кроме одного
select_except_largest = Выбрать все, кроме наибольшего
select_except_smallest = Выделить все, кроме самых маленьких
select_largest = Выберите наибольший
select_smallest = Выберите наименьший
select_except_highest_res = Выделить все кроме самого высокого разрешения
select_except_lowest_res = Выделить все кроме минимального разрешения
select_highest_res = Выберите наибольшее разрешение
select_lowest_res = Выберите минимальное разрешение
invert_selection = Инвертировать выделение
close = Закрыть
# Deselection popup
deselection_popup_title = Отменить выбор
deselect_all = Отменить выбор
deselect_except_one = Отменить выбор всех кроме одного
# Confirm popup
cancel = Отмена
delete = Удалить
rename = Переименовать
# Delete errors popup
delete_errors_title = Не удалось удалить некоторые файлы:
ok = ОК
# Stopping overlay
stopping_overlay_title = Остановка
stopping_overlay_body =
    Завершение сканирования...
    Пожалуйста подождите.
# Permission popup
permission_title = Доступ к файлу
permission_body = Для сканирования файлов приложению требуется доступ к хранилищу устройства. Без этого разрешения сканирование будет невозможно.
grant = Предоставить
no_permission_scan_warning = Нет доступа к файлу - предоставить разрешение для сканирования
# Settings screen tabs
settings_tab_general = Общие положения
settings_tab_tools = Инструменты
settings_tab_diagnostics = Инфо
# Settings - General tab
settings_use_cache = Использовать кэш
settings_use_cache_desc = Ускоряет последующие сканирования (хэш/изображения)
settings_ignore_hidden = Игнорировать скрытые файлы
settings_ignore_hidden_desc = Файлы и папки, начинающиеся с '.'
settings_show_notification = Уведомлять о завершении сканирования
settings_show_notification_desc = Показывать системное уведомление при сканировании
settings_notify_only_background = Только когда в фоне
settings_notify_only_background_desc = Пропустить уведомление, если приложение видно
notifications_disabled_banner = Уведомления отключены
notifications_enable_button = Включить
settings_scan_label = СКАН
settings_filters_label = ФИЛЬТРЫ (некоторые инструменты)
settings_min_file_size = Минимальный размер файла
settings_max_file_size = Макс. размер файла
settings_language = Язык
settings_language_restart = Требуется перезапуск приложения
settings_common_label = ОБЩИЕ НАСТРОЙКИ
settings_excluded_items = ВЫПОЛНЕННЫЕ ПРЕДМЕТЫ (мрачные шаблоны, разделённые запятыми)
settings_excluded_items_placeholder = например, *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = РАЗРЕШЕННЫЕ ВЫХОДЫ (пусто = все)
settings_allowed_extensions_placeholder = напр. jpg, png, mp4
settings_excluded_extensions = РАСПРОСТРАНЕНИЕ ВЫШЕНИЙ
settings_excluded_extensions_placeholder = например, bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = ПОДРАЗУМЕВАНИЯ
settings_check_method_label = МЕТОД КОМПАРИЗНА
settings_check_method = Метод
settings_hash_type_label = ТИП ЛЕГКОГО
settings_hash_type = Тип хэша
settings_hash_type_desc = Blake3 - рекомендуется, CRC32 имеет малые шансы на ложное срабатывание
settings_similar_images_header = СИЛИЛАРНЫЕ ИМЕНИЯ
settings_similarity_preset = Порог сходства
settings_similarity_desc = Очень высокий = только почти идентичный
settings_hash_size = Размер хэша
settings_hash_size_desc = Большие размеры, имеют меньше ложных срабатываний, но и находят менее похожие изображения
settings_hash_alg = Алгоритм хеша
settings_image_filter = Изменить размер фильтра
settings_ignore_same_size = Игнорировать изображения с одинаковыми размерами
settings_gallery_image_fit_cover = Галерея: обрезать на квадрат
settings_gallery_image_fit_cover_desc = Заполните плитку; отключите для сохранения оригинального соотношения сторон
settings_big_files_header = БЫСТРЫЕ ФАЙЛЫ
settings_search_mode = Режим поиска
settings_file_count = Количество файлов
settings_same_music_header = МУЗЫЧЕСКИЕ ДВИГАТЕЛИ
settings_music_check_method = Режим сравнения
settings_music_compare_tags_label = Тэги КОМПАНИИ
settings_music_title = Заголовок
settings_music_artist = Художник
settings_music_year = Год
settings_music_length = Длина
settings_music_genre = Жанр
settings_music_bitrate = Битрейт
settings_music_approx = Приблизительное сравнение тегов
settings_broken_files_header = БРОКЕН ФАЙЛЫ
settings_broken_files_note = Ресурсоемкое сканирование. Для оптимальной производительности используйте Krokiet на рабочем столе.
settings_broken_files_types_label = ИЗМЕНЕННЫЕ ТИПЫ
settings_broken_audio = Аудио
settings_broken_pdf = PDF
settings_broken_archive = Архивировать
settings_broken_image = Изображение
settings_bad_names_header = НАЗВАНИЯ BAD
settings_bad_names_checks_label = ПОИСК
settings_bad_names_uppercase_ext = Расширение прописных букв
settings_bad_names_emoji = Эмодзи в имени
settings_bad_names_space = Пробелы в начале/конце
settings_bad_names_non_ascii = Не ASCII символы
settings_bad_names_duplicated = Повторяющиеся символы
# Settings - Diagnostics tab
diagnostics_header = ДИАГНОСТИКА
diagnostics_thumbnails = Кэш миниатюр
diagnostics_app_cache = Кэш приложений
diagnostics_refresh = Обновить
diagnostics_clear_thumbnails = Очистить эскизы
diagnostics_open_thumbnails_folder = Открыть папку
diagnostics_clear_cache = Очистить кэш
diagnostics_open_cache_folder = Открыть папку
diagnostics_collect_test = Тест доступа к файлу
diagnostics_collect_test_desc = Проверьте, сколько файлов доступно
diagnostics_collect_test_run = Запустить
diagnostics_collect_test_stop = Остановить
collect_test_cancelled = Остановлено пользователем
diag_confirm_clear_thumbnails = Очистить кэш эскизов?
diag_confirm_clear_cache = Очистить весь кэш приложений?
about_repo = Репозиторий
about_translate = Переводы
about_donate = Поддержка
# Collect-test result popup
collect_test_title = Результаты испытания
collect_test_volumes = Объем:
collect_test_folders = Папки:
collect_test_files = Файлы:
collect_test_time = Время:
# Licenses
licenses_label = ЛИЦЕНЗИЯ
third_party_licenses = Сторонние лицензии
licenses_popup_title = Сторонние лицензии
# Directories screen
directories_include_header = Включить
directories_included = Включено
directories_exclude_header = Исключить
directories_excluded_header = Исключено
directories_add = Включить
no_paths = Нет путей - добавить ниже
directories_volume_header = Громкость
directories_volume_refresh = Обновить
directories_volume_add = Добавить
# Bottom navigation
nav_home = Начать
nav_dirs = Каталоги
nav_settings = Настройки
# Status messages set from Rust
status_ready = Готово
status_stopped = Остановлено
status_no_results = Нет результатов
status_deleted_selected = Удалено
status_deleted_with_errors = Удалено с ошибками
scan_not_started = Сканирование не начато
found_items_prefix = Найдено
found_items_suffix = элементы
deleted_items_prefix = Удалено
deleted_items_suffix = элементы
deleted_errors_suffix = ошибки
renamed_prefix = Переименовано
renamed_files_suffix = файлы
renamed_errors_suffix = ошибки
cleaned_exif_prefix = Удалено EXIF из
cleaned_exif_suffix = файлы
cleaned_exif_errors_suffix = ошибки
and_more_prefix = ...и
and_more_suffix = больше
# Gallery / delete popups
gallery_delete_button = Удалить
gallery_back = Назад
gallery_confirm_delete = Да, удалить
deleting_files = Удаление файлов...
stop = Остановить
files_suffix = файлы
scanning_fallback = Сканирование...
app_subtitle = В честь Сединии (972 CE)
app_license = Фронтенд для Czkawka Core - лицензия GPL-3.0
about_app_label = О программе
cache_label = ОЧИСТЬ
# Notification
scan_completed_notification = Сканирование завершено - { $file_count } элементов найдено
# Confirm popups (set from Rust)
confirm_clean_exif = Вы уверены, что хотите удалить EXIF теги из { $n } выбранных файлов?
confirm_delete_items = Вы уверены, что хотите удалить { $n } выбранных элементов?
gallery_confirm_delete_msg = Вы собираетесь удалить { $total_images } изображений в { $total_groups } группах.
gallery_confirm_delete_warning = Все элементы выбраны в { $unsafe_groups } группах!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Вычисление и сравнение аудио отпечатков пальцев очень ресурсоемкое и может занять много времени. Для этого рекомендуется использовать Krokiet на настольной системе.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Сбор файлов
stage_scanning_name = Сканирование по имени
stage_scanning_size_name = Сканирование по имени и размеру
stage_scanning_size = Сканирование по размеру
stage_pre_hash = Пре-хеширование
stage_full_hash = Хэширование
stage_loading_cache = Загрузка кэша
stage_saving_cache = Сохранение кэша
stage_calculating_image_hashes = Расчет хэшей изображений
stage_comparing_images = Сравнение изображений
stage_calculating_video_hashes = Вычисление хэшей видео
stage_checking_files = Проверка файлов
stage_checking_extensions = Проверка расширений
stage_checking_names = Проверка имен
stage_reading_music_tags = Чтение музыкальных тегов
stage_comparing_tags = Сравнение тегов
stage_calculating_music_fingerprints = Вычисление отпечатков музыки
stage_comparing_fingerprints = Сравнение отпечатков пальцев
stage_extracting_exif = Чтение тегов EXIF
stage_creating_video_thumbnails = Создание миниатюр видео
stage_processing_videos = Обработка видео
stage_deleting = Удаление файлов
stage_renaming = Переименование файлов
stage_moving = Перемещение файлов
stage_hardlinking = Создание жёстких ссылок
stage_symlinking = Создание символьных ссылок
stage_optimizing_videos = Оптимизация видео
stage_cleaning_exif = Очистка EXIF
# Group headers in scan results
duplicates_group_header = { $count } файлов x { $per_file } / file = { $total } всего
similar_images_group_header = { $count } похожие изображения
same_music_group_header = { $count } похожие треки
# Rename confirmation
confirm_rename_items = Вы уверены, что хотите переименовать { $n } выбранных файлов?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Крупнейший
option_search_mode_smallest = Наименьший
option_similarity_very_high = Высокий V
option_similarity_high = Высокий
option_similarity_medium = Средний
option_similarity_low = Низкий
option_similarity_very_low = V.Низкий
option_similarity_minimal = Мин.
option_check_method_hash = Хэш
option_check_method_name = Наименование
option_check_method_size_and_name = Размер+Имя
option_check_method_size = Размер
option_music_method_tags = Теги
option_music_method_audio = Аудио
option_min_size_none = Нет
option_min_size_1kb = 1 КБ
option_min_size_8kb = 8 КБ
option_min_size_64kb = 64 КБ
option_min_size_1mb = 1 МБ
option_max_size_16kb = 16 КБ
option_max_size_1mb = 1 МБ
option_max_size_10mb = 10 МБ
option_max_size_100mb = 100 МБ
option_max_size_unlimited = Неограниченный
# Volume labels (shown in the directories screen)
volume_internal_storage = Внутреннее хранилище
volume_sd_card = Карта памяти (SD карта)
volume_storage = Громкость хранения
# Directories screen
directories_referenced_tooltip = Ссылаемый (не удален)
directories_include_section_header = ВКЛЮЧЕНО
directories_exclude_section_header = ВЫКЛЮЧЕНО
directories_custom_paths = Пользовательские пути
directories_check_button = Анализ
directories_check_popup_title = Статистика каталога
directories_check_label_included = Включенные пути:
directories_check_label_excluded = Исключенные пути:
directories_check_label_referenced = Путь к ссылке:
directories_check_label_would_scan = Файлы для сканирования:
directories_check_label_processable = Обрабатываемые файлы:
directories_check_scanning = Сканирование...
directories_check_warning_no_processable = Не найдено обрабатываемых файлов - проверьте ваши включные/исключенные папки
path_edit_title_include = Добавить в Включить
path_edit_title_exclude = Добавить к исключению
path_edit_placeholder = Введите путь...
path_edit_not_exists = Путь не существует
path_edit_is_dir = Каталог
path_edit_is_file = Файл
path_edit_no_newlines = Пути не могут содержать новые строки — Ввод ключа не допускается
ctx_menu_title = Открыть
ctx_open_file = Открыть предмет
ctx_open_folder = Открыть родительскую папку
