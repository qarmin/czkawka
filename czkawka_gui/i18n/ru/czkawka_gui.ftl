# Window titles
window_settings_title = Опции
window_main_title = Czkawka (икота)
window_progress_title = Сканирование
window_compare_images = Сравнить изображения
# General
general_ok_button = Ок
general_close_button = Закрыть
# Main window
music_title_checkbox = Заголовок
music_artist_checkbox = Художник
music_album_title_checkbox = Название альбома
music_album_artist_checkbox = Альбом исполнителя
music_year_checkbox = Год
music_comparison_checkbox = Приблизительное сравнение
music_comparison_checkbox_tooltip =
    It is designed to help you to learn foreign words and phrases quickly and easily. с включенной опцией, эти файлы будут рассматриваться как дубликаты:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Наименование
duplicate_mode_size_combo_box = Размер
duplicate_mode_hash_combo_box = Хэш
duplicate_hash_type_tooltip =
    Чкавка предлагает 3 вида хэшей, которые можно использовать:
    
    Blake3 - криптографический хэш. Используется в качестве хеш-алгоритма по умолчанию, потому что он очень быстрый.
    
    CRC32 - простая хэш-функция. Это должно быть быстрее, чем Blake3, но вероятно, очень редко некоторые столкновения.
    
    XXH3 - очень похожие в случае производительности и качества хэша с Blake3, поэтому такие режимы можно легко использовать.
duplicate_check_method_tooltip =
    На данный момент Чокавка предлагает три типа методов, чтобы найти дубликаты:
    
    Имя - Найди файлы, имеющие одинаковое имя.
    
    Размер - находит файлы, которые имеют одинаковый размер.
    
    Хэш - находит файлы, которые имеют одинаковое содержимое. В этом режиме хэши и затем сравнивают эти хэши для поиска дубликатов. Этот режим является самым безопасным способом поиска дубликатов. Инструмент сильно использует кэш, поэтому второй и последующие сканирования одних и тех же данных должны быть намного быстрее, чем в первую очередь.
image_hash_size_tooltip =
    Чкавка предлагает изменить размер генерируемого хэша для каждого изображения. Чем больше хэш причины позволяет находить изображения с меньшим количеством различий между изображениями, но и немного медленнее в использовании.
    
    Значение по умолчанию для хэша - 8 байт, что позволяет найти очень похожие и разные изображения. 16 и 32 хэшей следует использовать только для почти идентичных изображений. Хэш 64 байт не должен использоваться, кроме случаев, когда для поиска необходимы действительно небольшие различия
image_resize_filter_tooltip = Чтобы вычислить хэш изображения, библиотека должна изменить размер изображения. В зависимости от выбранного алгоритма получившееся изображение будет выглядеть немного по-разному. Самым быстрым алгоритмом для использования, но также является алгоритм, который дает наихудшие результаты.
image_hash_alg_tooltip = Пользователи могут выбрать один из множества алгоритмов вычисления хэша. Каждый имеет сильные и слабые точки и может давать иногда более качественные и иногда хуже результаты для различных изображений, так что для выбора лучшего, требуется ручное тестирование.
main_notebook_image_fast_compare = Быстрое сравнение
main_notebook_image_fast_compare_tooltip =
    Ускорить поиск и сравнение хэшей.
    
    В противоположность нормальному режиму, где каждый хэш сравнивается друг с другом х раз, где x - схожесть, которую пользователь выбирает, в этом режиме используется всегда только одно сравнение.
    
    Эта опция рекомендуется при сравнении >10000 изображений с подобием не 0(очень высоким).
main_notebook_duplicates = Дублировать файлы
main_notebook_empty_directories = Пустые каталоги
main_notebook_big_files = Большие файлы
main_notebook_empty_files = Пустые файлы
main_notebook_temporary = Временные файлы
main_notebook_similar_images = Похожие изображения
main_notebook_similar_videos = Похожие видео
main_notebook_same_music = Музыкальные дубликаты
main_notebook_symlinks = Недопустимые символические ссылки
main_notebook_broken_files = Сломанные файлы
main_tree_view_column_file_name = Имя файла
main_tree_view_column_folder_name = Имя папки
main_tree_view_column_path = Путь
main_tree_view_column_modification = Дата изменения
main_tree_view_column_size = Размер
main_tree_view_column_similarity = Схожесть
main_tree_view_column_dimensions = Размеры
main_tree_view_column_title = Заголовок
main_tree_view_column_artist = Художник
main_tree_view_column_year = Год
main_tree_view_column_album_title = Название альбома
main_tree_view_column_album_artist = Альбом исполнителя
main_tree_view_column_symlink_file_name = Имя файла Symlink
main_tree_view_column_symlink_folder = Папка Symlnik
main_tree_view_column_destination_path = Путь назначения
main_tree_view_column_type_of_error = Тип ошибки
main_label_check_method = Метод проверки
main_label_hash_type = Тип хэша
main_label_hash_size = Размер хэша
main_label_size_bytes = Размер(байт)
main_label_min_size = Мин
main_label_max_size = Макс
main_label_shown_files = Количество отображаемых файлов
main_label_resize_algorithm = Изменить размер алгоритма
main_label_similarity = Похожей{ " " }
check_button_general_same_size = Игнорировать одинаковый размер
check_button_general_same_size_tooltip = Игнорировать из результатов, файлы, имеющие одинаковый размер - обычно это 1:1 дубликаты
main_label_size_bytes_tooltip = Размер файлов, которые будут использоваться в сканировании
# Upper window
upper_tree_view_included_folder_column_title = Папки для поиска
upper_tree_view_included_reference_column_title = Папки ссылок
upper_recursive_button = Рекурсивный
upper_recursive_button_tooltip = Если выбрано, то найдите файлы, которые не помещаются непосредственно в выбранные папки.
upper_manual_add_included_button = Ручное добавление
upper_add_included_button = Добавить
upper_remove_included_button = Удалить
upper_manual_add_excluded_button = Ручное добавление
upper_add_excluded_button = Добавить
upper_remove_excluded_button = Удалить
upper_manual_add_included_button_tooltip = Позволяет вручную добавить имя каталога для поиска.
upper_add_included_button_tooltip = Добавить новый каталог для поиска.
upper_remove_included_button_tooltip = Удалить каталог из поиска.
upper_manual_add_excluded_button_tooltip = Позволяет вручную добавить исключенное имя каталога.
upper_add_excluded_button_tooltip = Добавить каталог, исключаемый из поиска.
upper_remove_excluded_button_tooltip = Удалить каталог из исключенных.
upper_notebook_items_configuration = Конфигурация пунктов
upper_notebook_excluded_directories = Исключенные каталоги
upper_notebook_included_directories = Включенные каталоги
upper_allowed_extensions_tooltip =
    Допустимые расширения должны быть разделены запятыми (по умолчанию доступны все расширения).
    
    Макрос IMAGE, VIDEO, MUSIC, TEXT, который добавляет сразу несколько расширений.
    
    Пример использования ".exe, IMAGE, VIDEO, .rar, 7z" - это означает, что image(e. . jpg, png), video(e.g. avi, mp4), exe, rar и 7z файлы будут сканированы.
upper_excluded_items_tooltip =
    Исключенные элементы должны содержать * звездочку и должны быть разделены запятыми.
    Это медленнее, чем Исключенные Директории, поэтому используйте его внимательно.
upper_excluded_items = Исключенные пункты:
upper_allowed_extensions = Допустимые расширения:
# Popovers
popover_select_all = Выбрать все
popover_unselect_all = Снять выделение
popover_reverse = Обратный выбор
popover_select_all_except_oldest = Выделить все, кроме старых
popover_select_all_except_newest = Выделить все кроме новых
popover_select_one_oldest = Выберите старое
popover_select_one_newest = Выберите один новый
popover_select_custom = Выбрать пользовательский
popover_unselect_custom = Снять выбор
popover_select_all_images_except_biggest = Выделить все, кроме наибольшего
popover_select_all_images_except_smallest = Выделить все, кроме самых маленьких
popover_custom_path_check_button_entry_tooltip =
    Позволяет выбрать записи по его пути.
    
    Пример использования:
    /home/pimpek/rzecz.txt может быть найден с /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Позволяет выбрать записи по именам.
    
    Пример использования:
    /usr/ping/pong.txt можно найти с *ong*
popover_custom_regex_check_button_entry_tooltip =
    Позволяет выбрать записи по указанному Regex.
    
    В этом режиме искаженный текст является Путь с именем.
    
    Пример:
    /usr/bin/ziemniak. xt можно найти с помощью /ziem[a-z]+
    
    Эта реализация по умолчанию Rust regex, так что вы можете прочитать больше об этом в https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Предотвращает выбор всех записей в группе.
    
    Это включено по умолчанию, потому что в большинстве ситуаций пользователь не хочет удалять как оригиналы, так и дубликаты, но хотите оставить хотя бы один файл.
    
    Предупреждение: Эта настройка не работает, если пользователь уже выбрал все результаты в группе вручную.
popover_custom_regex_path_label = Путь
popover_custom_regex_name_label = Наименование
popover_custom_regex_regex_label = Путь к выражению + имя
popover_custom_all_in_group_label = Не выбирать все записи в группе
popover_custom_mode_unselect = Снять выбор
popover_custom_mode_select = Выбрать пользовательский
popover_invalid_regex = Некорректный выраж
popover_valid_regex = Regex действителен
# Bottom buttons
bottom_search_button = Искать
bottom_select_button = Выбрать
bottom_delete_button = Удалить
bottom_save_button = Сохранить
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Переместить
bottom_search_button_tooltip = Начать поиск файлов/папок.
bottom_select_button_tooltip = Выбор записей. Только выбранные файлы/папки могут быть обработаны позже.
bottom_delete_button_tooltip = Удалить выбранные файлы/папки.
bottom_save_button_tooltip = Сохранить данные о поиске в файл
bottom_symlink_button_tooltip =
    Создает символические ссылки.
    Работает только когда выбраны не менее 2 результатов в группе.
    Первый не изменен, второй и более поздние символизируют первый.
bottom_hardlink_button_tooltip =
    Создает хардлинки.
    Работает только когда выбраны как минимум 2 результата в группе.
    Первый не изменен, а второй и более поздние тесно связаны с первым.
bottom_move_button_tooltip =
    Перемещает файлы в выбранную папку.
    Копирует все файлы в папку без сохранения дерева каталогов.
    При попытке переместить 2 файла с одинаковым именем в папку, второй не будет выполнён и появится ошибка.
bottom_show_errors_tooltip = Показать/скрыть нижнюю панель ошибок.
bottom_show_upper_notebook_tooltip = Показать/скрыть верхнюю панель блокнота.
# Progress Window
progress_stop_button = Остановить
# About Window
about_repository_button_tooltip = Ссылка на страницу репозитория с исходным кодом.
about_donation_button_tooltip = Ссылка на страницу пожертвования.
about_instruction_button_tooltip = Ссылка на страницу инструкций.
about_translation_button_tooltip = Ссылка на страницу Crowdin с переводами приложений. Официально польский и английский языки, но любая помощь с другим языком будет признательна.
about_repository_button = Репозиторий
about_donation_button = Пожертвование
about_instruction_button = Инструкция
about_translation_button = Перевод
# Header
header_setting_button_tooltip = Открыть диалог настроек.
header_about_button_tooltip = Открывает диалог с информацией о приложении.

# Settings


## General

settings_save_at_exit_button_tooltip = Сохраняет конфигурацию в файл при закрытии приложения.
settings_load_at_start_button_tooltip =
    Загрузка конфигурации при запуске из файла.
    
    Не выбрав этот параметр, будут загружены настройки по умолчанию.
settings_confirm_deletion_button_tooltip = Показывает диалоговое окно подтверждения при нажатии на кнопку удаления.
settings_confirm_link_button_tooltip = Показывает диалоговое окно подтверждения при нажатии на кнопку хард/символьная ссылка.
settings_confirm_group_deletion_button_tooltip = Показывает диалог при попытке удалить все записи из группы.
settings_show_text_view_button_tooltip = Показывает панель ошибок внизу.
settings_use_cache_button_tooltip = Параметр, который позволяет не использовать функцию кэша.
settings_save_also_as_json_button_tooltip = Сохраните кэш для чтения по формату JSON. Можно изменять его содержимое. Кэш из этого файла будет прочитан автоматически приложением, если кэш двоичного формата (с расширением) не будет установлен.
settings_use_trash_button_tooltip = При включении перемещает файлы в корзину вместо их удаления навсегда.
settings_language_label_tooltip = Позволяет выбрать язык интерфейса из доступных языков.
settings_save_at_exit_button = Сохранить конфигурацию при выходе
settings_load_at_start_button = Загрузить конфигурацию при запуске
settings_confirm_deletion_button = Показывать подтверждение при удалении любых файлов
settings_confirm_link_button = Показывать окно подтверждения, когда все файлы/символьные ссылки
settings_confirm_group_deletion_button = Показывать подтверждение при удалении всех файлов в группе
settings_show_text_view_button = Показывать нижнюю текстовую панель
settings_use_cache_button = Использовать кэш
settings_save_also_as_json_button = Сохранить кэш в файл JSON
settings_use_trash_button = Переместить удаленные файлы в корзину
settings_language_label = Язык
settings_multiple_delete_outdated_cache_checkbutton = Автоматически удалять устаревшие записи кэша
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Позволяет удалять устаревшие результаты кэша, указывающие на несуществующие файлы.
    
    Когда включено, приложение проверяет при загрузке записей, что все указывает на допустимые файлы и игнорирует поврежденные файлы.
    
    Отключение этой опции, поможет сканировать файлы на внешних дисках, так что записи о них не будут очищены при следующем сканировании.
    
    В случае наличия сотни тысяч записей в кэше, включить эту опцию, ускорить загрузку и сохранение кэша в начале и конце сканирования.
settings_notebook_general = Общие положения
settings_notebook_duplicates = Дубликаты
settings_notebook_images = Похожие изображения
settings_notebook_videos = Похожие видео

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Показывает предварительный просмотр справа при выборе файла изображения.
settings_multiple_image_preview_checkbutton = Предпросмотр изображения
settings_multiple_clear_cache_button_tooltip =
    Очистка кэша вручную из устаревших записей.
    Должно быть использовано, только если автоматическая очистка была отключена.
settings_multiple_clear_cache_button = Удалить устаревшие результаты из кэша изображений

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Скрывает все файлы, кроме одного, если они указывают на одну и ту же информацию (жестко привязанные).
    
    Например, в случае, если на диске есть 7 файлов, которые связаны с определенными данными и один другой файл с одними и теми же данными, но другой inode,, то в дублирующем искателе будет виден только один уникальный файл и один файл из привязанных.
settings_duplicates_minimal_size_entry_tooltip =
    Позволяет установить минимальный размер файла, который будет кэширован.
    
    Выбор меньшего значения позволит создать больше записей, которые ускорят поиск, но замедляют загрузку/сохранение кэша.
settings_duplicates_prehash_checkbutton_tooltip =
    Включает кэширование prehash(хэша, вычисляемого из небольшой части файла), что позволяет раньше бросать неповторяющиеся результаты.
    
    По умолчанию отключено, потому что может вызвать замедление в некоторых ситуациях.
    
    Настоятельно рекомендуется использовать его при сканировании сотен тысяч или миллионов файлов, так как он может несколько раз ускорить поиск.
settings_duplicates_prehash_minimal_entry_tooltip = Минимальный размер кешированной записи.
settings_duplicates_hide_hard_link_button = Скрыть жёсткие ссылки (только Linux и MacOS)
settings_duplicates_prehash_checkbutton = Использовать кэш prehash
settings_duplicates_minimal_size_cache_label = Минимальный размер файлов в байтах, сохраненных в кэш
settings_duplicates_minimal_size_cache_prehash_label = Минимальный размер файлов в байтах, сохраненных в кэш prehash

## Saving/Loading settings

settings_saving_button_tooltip = Сохранить текущую конфигурацию настроек в файл.
settings_loading_button_tooltip = Загрузить настройки из файла и заменить текущую конфигурацию.
settings_reset_button_tooltip = Восстановить текущую конфигурацию по умолчанию.
settings_saving_button = Сохранить конфигурацию
settings_loading_button = Загрузить конфигурацию
settings_reset_button = Сбросить настройки

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Открывает папку, где хранятся файлы txt с кэшированием.
    
    Изменение их может привести к появлению некорректных результатов, но также и к изменениям.. Путь может сэкономить время при перемещении большого количества файлов в другое место.
    
    Вы можете скопировать эти файлы между компьютерами, чтобы сэкономить время на повторном сканировании для файлов (конечно, если они имеют аналогичную структуру каталогов).
    
    В случае проблем с кэшем эти файлы могут быть удалены, поэтому приложение автоматически сгенерирует их.
settings_folder_settings_open_tooltip =
    Открывает папку, в которой хранится конфигурация Czkawk.
    
    Изменение их вручную, может привести к нарушению рабочего процесса.
settings_folder_cache_open = Открыть папку кэша
settings_folder_settings_open = Открыть папку настроек
# Compute results
compute_stopped_by_user = Поиск был остановлен пользователем
compute_found_duplicates_hash_size = Найдено { $number_files } дубликатов в { $number_groups } группах, которые взяли { $size }
compute_found_duplicates_name = Найдено { $number_files } дубликатов в { $number_groups } группах
compute_found_empty_folders = Найдено { $number_files } пустых папок
compute_found_empty_files = Найдено { $number_files } пустых файлов
compute_found_big_files = Найдено { $number_files } больших файлов
compute_found_temporary_files = Найдено { $number_files } временных файлов
compute_found_images = Найдено { $number_files } похожие изображения в { $number_groups } группах
compute_found_videos = Найдено { $number_files } похожие видео в { $number_groups } группах
compute_found_music = Найдено { $number_files } похожие музыкальные файлы в { $number_groups } группах
compute_found_invalid_symlinks = Найдено { $number_files } неверные символические ссылки
compute_found_broken_files = Найдено { $number_files } сломанных файлов
# Progress window
progress_scanning_general_file = Сканирование { $file_number } файла
progress_scanning_broken_files = Проверка { $file_checked }/{ $all_files } файла
progress_scanning_video = Хэширование { $file_checked }/{ $all_files } видео
progress_scanning_image = Хэширование { $file_checked }/{ $all_files } изображения
progress_comparing_image_hashes = Сравнение { $file_checked }/{ $all_files } хэша изображений
progress_scanning_music_tags_end = Сравнение тегов { $file_checked }/{ $all_files } музыкального файла
progress_scanning_music_tags = Чтение тегов { $file_checked }/{ $all_files } музыкального файла
progress_scanning_empty_folders = Сканирование папки { $folder_number }
progress_scanning_size = Сканирование размера { $file_number } файла
progress_scanning_name = Сканирование имени { $file_number } файла
progress_analyzed_partial_hash = Частичный хэш { $file_checked }/{ $all_files } файлов
progress_analyzed_full_hash = Полный хэш { $file_checked }/{ $all_files } файлов
progress_current_stage = Текущая стадия:{ " " }
progress_all_stages = Все этапы:{ " " }
# Saving loading 
saving_loading_saving_success = Конфигурация сохранена в файл { $name }.
saving_loading_saving_failure = Не удалось сохранить данные конфигурации в файл { $name }.
saving_loading_reset_configuration = Текущая конфигурация была удалена.
saving_loading_loading_success = Правильно загруженные настройки приложения.
saving_loading_invalid_string = Для ключа "{ $key }" найден недопустимый результат - "{ $result }", который не является строкой.
saving_loading_invalid_int = Для ключа "{ $key }" найден недопустимый результат - "{ $result }", который не является целым числом.
saving_loading_invalid_bool = Для ключа "{ $key }" найден недопустимый результат - "{ $result }", который не является щитом.
saving_loading_decode_problem_bool = Не удалось декодировать бул из ключа "{ $key }" найдено "{ $result }", но допустимые значения равны 0, 1, true или false.
saving_loading_saving_same_keys = Попытка сохранить настройки с дублированным ключом "{ $key }".
saving_loading_failed_to_get_home_directory = Не удалось получить домашний каталог для открытия/сохранения конфигурационного файла.
saving_loading_folder_config_instead_file = Невозможно создать или открыть файл конфигурации в пути "{ $path }", потому что уже есть папка.
saving_loading_failed_to_create_configuration_folder = Не удалось создать папку конфигурации "{ $path }", причина "{ $reason }".
saving_loading_failed_to_create_config_file = Не удалось создать файл конфигурации "{ $path }", причина "{ $reason }".
saving_loading_failed_to_read_config_file = Невозможно загрузить конфигурацию из "{ $path }" потому что не существует или не является файлом.
saving_loading_failed_to_read_data_from_file = Невозможно прочитать данные из файла "{ $path }", причина "{ $reason }".
saving_loading_orphan_data = Найдены осиротевшие данные "{ $data }" в строке "{ $line }".
saving_loading_not_valid = Установка "{ $data }" не существует в текущей версии приложения.
# Invalid symlinks
invalid_symlink_infinite_recursion = Бесконечная рекурсия
invalid_symlink_non_existent_destination = Не существующий файл назначения
# Other
searching_for_data = Поиск данных, это может занять некоторое время, пожалуйста, подождите...
text_view_messages = СООБЩЕНИЯ
text_view_warnings = ПРЕДУПРЕЖДЕНИЯ
text_view_errors = ОШИБКИ
about_window_motto = Эта программа бесплатна для использования и всегда будет.
# Various dialog
dialogs_ask_next_time = Спросить в следующий раз
delete_file_failed = Не удалось удалить файл { $name }, причина { $reason }
delete_title_dialog = Подтверждение удаления
delete_question_label = Вы уверены, что хотите удалить файлы?
delete_all_files_in_group_title = Подтверждение удаления всех файлов в группе
delete_all_files_in_group_label1 = В некоторых группах все записи выделены.
delete_all_files_in_group_label2 = Вы уверены, что хотите удалить их?
delete_folder_failed = Не удалось удалить папку { $dir } , потому что папка не существует, у вас нет прав или не пуста.
delete_items_label = { $items } файлы будут удалены.
delete_items_groups_label = { $items } файлов из { $groups } групп будут удалены.
hardlink_failed = Не удалось связать
hard_sym_invalid_selection_title_dialog = Неверный выбор с некоторыми группами
hard_sym_invalid_selection_label_1 = В некоторых группах выбрана только одна запись, которая будет проигнорирована.
hard_sym_invalid_selection_label_2 = Чтобы иметь возможность использовать жесткий/симм, необходимо выбрать не менее 2 результатов в группе.
hard_sym_invalid_selection_label_3 = Первый в группе признан как оригинал и не изменен, но второй и более позднее модифицирован.
hard_sym_link_title_dialog = Подтверждение ссылки
hard_sym_link_label = Вы уверены, что хотите связать эти файлы?
move_folder_failed = Не удалось переместить папку { $name }, причина { $reason }
move_file_failed = Не удалось переместить файл { $name }, причина { $reason }
move_files_title_dialog = Выберите папку, в которую вы хотите переместить дублирующиеся файлы
move_files_choose_more_than_1_path = Необходимо выбрать только 1 путь, чтобы иметь возможность копировать там дубликаты, выбрано { $path_number }.
move_stats = Правильно перемещено { $num_files }/{ $all_files } предметов
save_results_to_file = Результаты сохранены в файл { $name }
search_not_choosing_any_music = ОШИБКА: Необходимо выбрать по крайней мере один флажок с типами поиска музыки.
include_folders_dialog_title = Папки для включения
exclude_folders_dialog_title = Папки для исключения
include_manually_directories_dialog_title = Добавить папку вручную
cache_properly_cleared = Правильно очищенный кэш
cache_clear_duplicates_title = Очистка кэша дубликатов
cache_clear_similar_images_title = Очистка кэша аналогичных изображений
cache_clear_similar_videos_title = Очистка кэша похожих видео
cache_clear_message_label_1 = Вы хотите очистить кэш от устаревших записей?
cache_clear_message_label_2 = Эта операция удалит все записи кэша, которые указывают на недопустимые файлы.
cache_clear_message_label_3 = Это может ускорить загрузку/сохранение в кэш.
cache_clear_message_label_4 = ВНИМАНИЕ: Операция удалит все кэшированные данные с неподключенных внешних накопителей, поэтому хэш необходимо сгенерировать снова.
# Show preview
preview_temporary_file = Не удалось открыть временный файл изображения { $name }, причина { $reason }.
preview_0_size = Невозможно создать предварительный просмотр изображения { $name }, ширина или высота.
preview_temporary_image_save = Не удалось сохранить временный файл в { $name }, причина { $reason }.
preview_temporary_image_remove = Не удалось удалить временный файл изображения { $name }, причина { $reason }.
preview_failed_to_create_cache_dir = Не удалось создать каталог { $name } , необходимый для просмотра изображения, причина { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Группа { $current_group }/{ $all_groups } ({ $images_in_group } изображения)
compare_move_left_button = L
compare_move_right_button = R
