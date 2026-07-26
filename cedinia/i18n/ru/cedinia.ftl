# Cedinia - English (fallback)

# App / top bar titles
tool_duplicate_files = Дубликаты
tool_empty_folders = Пустые папки
tool_similar_images = Похожие картинки
tool_empty_files = Пустые файлы
tool_temporary_files = Временные файлы
tool_big_files = Самые большие файлы
tool_broken_files = Битые файлы
tool_bad_extensions = Неверные расширения
tool_same_music = Дубликаты музыки
tool_bad_names = Неверные имена
tool_exif_remover = Данные EXIF
tool_similar_videos = Похожие видео (аудио)
tool_directories = Каталоги
tool_settings = Настройки
# Home screen tool card descriptions
home_dup_description = Поиск файлов с одинаковым содержимым
home_empty_folders_description = Каталоги без содержимого
home_similar_images_description = Найти визуально похожие фото
home_empty_files_description = Файлы с нулевым размером
home_temp_files_description = Временные и кэшированные файлы
home_big_files_description = Самые большие/маленькие файлы на диске
home_broken_files_description = PDF, аудио, изображения, архивы
home_bad_extensions_description = Файлы с недопустимым расширением
home_same_music_description = Похожие аудиофайлы по тегам
home_bad_names_description = Файлы с проблемными символами в имени
home_exif_description = Изображения с метаданными EXIF
home_similar_videos_description = Найти видео с похожим аудио
# Results list
scanning = Идет сканирование...
stopping = Остановка...
no_results = Нет результатов
press_start = Нажмите НАЧАТЬ для сканирования
select_label = Выбр.
deselect_label = Невыб.
list_label = Список
gallery_label = Гал
# Selection popup
selection_popup_title = Выбрать
select_all = Выбрать все
select_except_one = Выбрать все, кроме одного
select_except_largest = Выбрать все, кроме наибольшего
select_except_smallest = Выделить все, кроме самых маленьких
select_largest = Выбрать наибольшие
select_smallest = Выбрать наименьшие
select_except_highest_res = Выделить все кроме самого высокого разрешения
select_except_lowest_res = Выделить все кроме минимального разрешения
select_highest_res = Выбрать с наибольшим разрешением
select_lowest_res = Выбрать с наименьшим разрешением
invert_selection = Обратить выделение
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
    Пожалуйста, подождите.
# Permission popup
permission_title = Доступ к файлам
permission_body = Для сканирования файлов приложению требуется доступ к хранилищу устройства. Без этого разрешения сканирование будет невозможно.
grant = Предоставить
no_permission_scan_warning = Нет доступа к файлам - предоставьте разрешение для сканирования
# Settings screen tabs
settings_tab_general = Основные
settings_tab_tools = Инструменты
settings_tab_diagnostics = Сведения
# Settings - General tab
settings_use_cache = Использовать кэш
settings_use_cache_desc = Ускоряет последующие сканирования (хэш/изображения)
settings_ignore_hidden = Игнорировать скрытые файлы
settings_ignore_hidden_desc = Файлы и папки, начинающиеся с «.»
settings_show_notification = Уведомлять о завершении сканирования
settings_show_notification_desc = Показывать системное уведомление при сканировании
settings_notify_only_background = Только когда в фоне
settings_notify_only_background_desc = Пропустить уведомление, если приложение видно
notifications_disabled_banner = Уведомления отключены
notifications_enable_button = Включить
settings_scan_label = СКАН
settings_filters_label = ФИЛЬТРЫ (некоторые инструменты)
settings_min_file_size = Мин. размер файла
settings_max_file_size = Макс. размер файла
settings_language = Язык
settings_language_restart = Требуется перезапуск приложения
settings_common_label = ОБЩИЕ НАСТРОЙКИ
settings_excluded_items = ИСКЛЮЧЁННЫЕ ОБЪЕКТЫ (шаблоны glob, через запятую)
settings_excluded_items_placeholder = напр., *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = РАЗРЕШЁННЫЕ РАСШИРЕНИЯ (пусто = все)
settings_allowed_extensions_placeholder = напр. jpg, png, mp4
settings_excluded_extensions = ИСКЛЮЧЁННЫЕ РАСШИРЕНИЯ
settings_excluded_extensions_placeholder = напр., bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = ДУБЛИКАТЫ
settings_check_method_label = МЕТОД СРАВНЕНИЯ
settings_check_method = Метод
settings_hash_type_label = ТИП ХЭША
settings_hash_type = Тип хэша
settings_hash_type_desc = Blake3 - рекомендуется, CRC32 имеет малые шансы на ложное срабатывание
settings_similar_images_header = ПОХОЖИЕ КАРТИНКИ
settings_similarity_preset = Порог сходства
settings_similarity_desc = Очень высокий = только почти идентичный
settings_hash_size = Размер хэша
settings_hash_size_desc = Большие размеры, имеют меньше ложных срабатываний, но и находят менее похожие изображения
settings_hash_alg = Алгоритм хеша
settings_image_filter = Фильтр изменения размера
settings_geometric_invariance = Геометрическая инвариантность
settings_ignore_same_size = Игнорировать изображения с одинаковыми размерами
settings_gallery_image_fit_cover = Галерея: обрезать до квадрата
settings_gallery_image_fit_cover_desc = Заполнить плитку; отключите для сохранения исходных пропорций
settings_big_files_header = САМЫЕ БОЛЬШИЕ ФАЙЛЫ
settings_search_mode = Режим поиска
settings_file_count = Количество файлов
settings_same_music_header = ДУБЛИКАТЫ МУЗЫКИ
settings_music_check_method = Режим сравнения
settings_music_compare_tags_label = СРАВНИВАЕМЫЕ ТЕГИ
settings_music_title = Заголовок
settings_music_artist = Артист
settings_music_year = Год
settings_music_length = Длина
settings_music_genre = Жанр
settings_music_bitrate = Битрейт
settings_music_approx = Приблизительное сравнение тегов
settings_temporary_files_header = ВРЕМЕННЫЕ ФАЙЛЫ
settings_temporary_files_extensions_label = РАСШИРЕНИЯ
settings_temporary_files_extensions_placeholder = напр., .tmp, .bak, ~
settings_temporary_files_reset = Сброс по умолчанию
settings_broken_files_header = ПОВРЕЖДЁННЫЕ ФАЙЛЫ
settings_broken_files_note = Ресурсоёмкое сканирование. Для лучшей производительности используйте Krokiet на ПК.
settings_broken_files_types_label = ВЫБРАННЫЕ ТИПЫ
settings_broken_audio = Аудио
settings_broken_pdf = PDF
settings_broken_archive = Архив
settings_broken_image = Изображение
settings_broken_font = Шрифт
settings_broken_markup = Разметка (JSON/XML/TOML)
settings_similar_videos_header = ПОХОЖИЕ ВИДЕО (АУДИО)
settings_similar_videos_audio_preset = Предустановка сходства аудио
settings_similar_videos_audio_preset_desc = Управляет, насколько строго звук должен соответствовать
settings_bad_names_header = НЕВЕРНЫЕ ИМЕНА
settings_bad_names_checks_label = ПОИСК
settings_bad_names_uppercase_ext = Расширение в верхнем регистре
settings_bad_names_emoji = Эмодзи в имени
settings_bad_names_space = Пробелы в начале/конце
settings_bad_names_non_ascii = Не ASCII символы
settings_bad_names_duplicated = Повторяющиеся символы
settings_ignore_same_resolution = Игнорировать изображения с одинаковым разрешением
# Settings - Appearance section
settings_appearance_label = ВНЕШНИЙ ВИД
settings_dark_theme = Тёмная тема
settings_dark_theme_desc = Использовать тёмную цветовую схему
# Settings - Diagnostics tab
diagnostics_header = ДИАГНОСТИКА
diagnostics_thumbnails = Кэш миниатюр
diagnostics_app_cache = Кэш приложения
diagnostics_refresh = Обновить
diagnostics_clear_thumbnails = Очистить эскизы
diagnostics_open_thumbnails_folder = Открыть папку
diagnostics_clear_cache = Очистить кэш
diagnostics_open_cache_folder = Открыть папку
diagnostics_export_logs = Экспорт журналов
logs_label = ЖУРНАЛЫ
logs_export_title = Экспорт журналов
logs_export_saved = Журналы скопированы в:
logs_export_failed = Не удалось экспортировать журналы
diagnostics_collect_test = Проверка доступа к файлам
diagnostics_collect_test_desc = Узнать количество доступных файлов
diagnostics_collect_test_run = Запустить
diagnostics_collect_test_stop = Остановить
collect_test_cancelled = Остановлено пользователем
diag_confirm_clear_thumbnails = Очистить кэш эскизов?
diag_confirm_clear_cache = Очистить весь кэш приложения?
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
no_paths = Нет путей - добавьте ниже
directories_volume_header = Разделы
directories_volume_refresh = Обновить
directories_volume_add = Добавить
# Bottom navigation
nav_home = Начать
nav_dirs = Каталоги
nav_settings = Настройки
# Status messages set from Rust
status_ready = Готов к работе
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
cleaned_exif_prefix = Очищен EXIF из
cleaned_exif_suffix = файлы
cleaned_exif_errors_suffix = ошибки
rename_error_read_file_name = Не удается прочитать имя файла
rename_error_read_directory = Не удается прочитать каталог
and_more_prefix = ...и
and_more_suffix = больше
# Gallery / delete popups
gallery_delete_button = Удалить
gallery_back = Назад
gallery_confirm_delete = Да, удалить
deleting_files = Удаление файлов...
stop = Остановить
scanning_fallback = Сканирование...
app_subtitle = В память о Битве под Цедыней (972 г. н.э.)
app_license = Фронтенд для Czkawka Core - лицензия GPL-3.0
about_app_label = О программе
cache_label = ОЧИСТЬ
# Notification
scan_completed_notification = Сканирование завершено - { $file_count } элементов найдено
# Confirm popups (set from Rust)
confirm_clean_exif = Уверены, что хотите удалить теги EXIF из { $n } выбранных файлов?
confirm_delete_items = Уверены, что хотите удалить { $n } выбранных элементов?
gallery_confirm_delete_msg = Вы собираетесь удалить { $total_images } изображений в { $total_groups } группах.
gallery_confirm_delete_warning = Все элементы выбраны в { $unsafe_groups } группах!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Вычисление и сравнение отпечатков аудио требует много ресурсов и может занять много времени. Для этой задачи рекомендуется использовать Krokiet на настольном ПК.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } файлов x { $per_file } / файл = { $total } всего
similar_images_group_header = { $count } похожих изображений
same_music_group_header = { $count } похожие треки
similar_videos_group_header = { $count } похожие видео
# Rename confirmation
confirm_rename_items = Уверены, что хотите переименовать { $n } выбранных файлов?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Крупнейший
option_search_mode_smallest = Наименьший
option_similarity_very_high = Высокий V
option_similarity_high = Высокий
option_similarity_medium = Средний
option_similarity_low = Низкий
option_similarity_very_low = V.Низкий
option_similarity_minimal = Мин
option_check_method_hash = Хэш
option_check_method_name = Имя
option_check_method_size_and_name = Размер+Имя
option_check_method_size = Размер
option_music_method_tags = Теги
option_music_method_audio = Аудио
option_min_size_none = Нет
option_max_size_unlimited = Неограниченный
option_audio_preset_identical = Идентичные
option_audio_preset_clip = Фрагмент в более длинном
option_audio_preset_similar = Похожие
# Volume labels (shown in the directories screen)
volume_internal_storage = Внутреннее хранилище
volume_sd_card = Карта памяти (SD карта)
volume_storage = Раздел хранилища
# Directories screen
directories_referenced_tooltip = Ссылаемый (не удален)
directories_include_section_header = ВКЛЮЧЕНО
directories_exclude_section_header = ИСКЛЮЧЕНО
directories_custom_paths = Свои пути
directories_check_button = Анализ
directories_check_popup_title = Статистика каталогов
directories_check_label_included = Включённые пути:
directories_check_label_excluded = Исключённые пути:
directories_check_label_referenced = Эталонные пути:
directories_check_label_would_scan = Файлы для сканирования:
directories_check_label_processable = Обрабатываемые файлы:
directories_check_scanning = Сканирование...
directories_check_warning_no_processable = Не найдены файлы для обработки - проверьте ваши включённые/исключённые папки
path_edit_title_include = Добавить в Включить
path_edit_title_exclude = Добавить к исключению
path_edit_placeholder = Введите путь...
path_edit_not_exists = Путь не существует
path_edit_is_dir = Каталог
path_edit_is_file = Файл
path_edit_no_newlines = Пути не могут содержать новые строки - Ввод ключа не допускается
ctx_menu_title = Открыть
ctx_open_file = Открыть объект
ctx_open_folder = Открыть родительскую папку
dir_open_folder = Открыть папку
# Compare view
compare_label = Сравнить
compare_loading = Загрузка изображений...
compare_cancelling = Отмена...
compare_computing = Вычисление разницы...
compare_mode_normal = Грань
compare_mode_split = Разделить
compare_mode_overlay = Наложение
compare_mode_diff = Разница
compare_res_mismatch = Разные разрешения - разница может быть неточной
