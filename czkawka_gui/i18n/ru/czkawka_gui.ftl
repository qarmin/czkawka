# Window titles
window_settings_title = Настройки
window_main_title = Czkawka («Икота»)
window_progress_title = Сканирование
window_compare_images = Сравнить изображения
# General
general_ok_button = ОК
general_close_button = Закрыть
# Main window
music_title_checkbox = Заголовок
music_artist_checkbox = Исполнитель
music_year_checkbox = Год
music_bitrate_checkbox = Битрейт
music_genre_checkbox = Жанр
music_length_checkbox = Длительность
music_comparison_checkbox = Приблизительное сравнение
music_checking_by_tags = Теги
music_checking_by_content = Содержание
same_music_seconds_label = Минимальная длительность второго фрагмента
same_music_similarity_label = Максимальная разница
music_compare_only_in_title_group = Сравнить только в заголовке
music_compare_only_in_title_group_tooltip =
    Когда включено, файлы сгруппируются по заголовку, а затем сравниваются друг с другом.
    
    С 10000 файлов, вместо этого почти 100 миллионов сравнений обычно будет около 20000 сравнений.
same_music_tooltip =
    Поиск похожих музыкальных файлов по его содержимому может быть настроен с помощью настройки:
    
    - Минимальное время фрагмента, после которого музыкальные файлы можно определить как похожие
    - Максимальная разница между двумя проверенными фрагментами
    
    Ключ к хорошим результатам - найти разумные комбинации этих параметров, для предоставленных.
    
    Установка минимального времени на 5 секунд, а максимальная разница в 1.0, будет искать практически идентичные фрагменты файлов.
    Время 20 секунд и максимальная разница в 6,0, с другой стороны, хорошо подходит для поиска ремиксов/версий и т.д.
    
    По умолчанию, каждый музыкальный файл сравнивается друг с другом, и это может занять много времени при тестировании множества файлов, поэтому обычно лучше использовать справочные папки и указать, какие файлы следует сравнивать друг с другом (одинаковое количество файлов), сравнение отпечатков пальцев будет быстрее по крайней мере на 4х, чем без ссылочных папок).
music_comparison_checkbox_tooltip =
    Ищет похожие музыкальные файлы с помощью ИИ, использующего машинное обучение для удаления скобок из фраз. Например, если эта опция включена, следующие файлы будут считаться дубликатами:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = С учётом регистра
duplicate_case_sensitive_name_tooltip =
    При включённой опции записи группируются, только если у них полностью совпадают имена с точностью до каждого символа. Например, «ХИТ Дискотека» не совпадёт с «хит дискотека».
    
    При отключённой опции записи группируются вне зависимости от того, заглавные или строчные буквы использовались при написании. Например, «ХИТ Дискотека», «хит дискотека», «хИт ДиСкОтЕКа» будут эквивалентны.
duplicate_mode_size_name_combo_box = Размер и имя
duplicate_mode_name_combo_box = Имя
duplicate_mode_size_combo_box = Размер
duplicate_mode_hash_combo_box = Хэш
duplicate_hash_type_tooltip =
    В программе Czkawka можно использовать один из трёх алгоритмов хэширования:
     
    Blake3 — криптографическая хэш-функция. Используется по умолчанию, поскольку очень быстрый.
     
    CRC32 — простая хэш-функция. Ещё быстрее, чем Blake3, но возможны очень редкие совпадения хэшей неидентичных файлов.
     
    XXH3 — функция, похожая по производительности и надёжности хэша на Blake3, но не являющаяся криптографической, поэтому её можно использовать вместо Blake3.
duplicate_check_method_tooltip =
    На данный момент Czkawka предлагает три метода поиска дубликатов:
    
    Имя — ищет файлы с одинаковыми именами.
    
    Размер — ищет файлы одинакового размера.
    
    Хэш — ищет файлы с одинаковым содержимым. Этот режим хэширует файл, а затем сравнивает хэш для поиска дубликатов. Этот режим является самым надёжным способом поиска. Приложение активно использует кэш, поэтому второе и последующие сканирования одних и тех же данных должны быть намного быстрее, чем первое.
image_hash_size_tooltip =
    Каждое проверяемое изображение производит специальный хэш, который можно сравнить друг с другом, и небольшая разница между ними означает, что эти изображения аналогичны.
    
    8 размер хэша достаточно хорош, чтобы найти изображения, которые немного похожи на оригинал. С большим набором изображений (>1000), это приведет к большому количеству ложных срабатываний, поэтому в данном случае я рекомендую использовать больший размер хэша.
    
    16 - это размер хэша по умолчанию, который является хорошим компромиссом между нахождением даже немного похожих изображений и наличием лишь небольшого количества хэш-коллизий.
    
    32 и 64 хэши находят только очень похожие изображения, но не должны иметь ложных срабатываний (может быть, за исключением некоторых изображений с альфа-каналом).
image_resize_filter_tooltip =
    Чтобы вычислить хэш изображения, библиотека должна сначала его перемасштабировать.
    
    В зависимости от выбранного алгоритма полученное изображение, используемое при хэшировании, может выглядеть немного другим.
    
    Самый быстрый алгоритм с низким качеством — это метод ближайших соседей, Nearest. Он включён по умолчанию, потому при размере хэша 16x16 плохое качество не замечается.
    
    Если размер хэша 8x8, рекомендуется любой алгоритм, кроме Nearest, чтобы лучше отличать похожие изображения в группах.
image_hash_alg_tooltip =
    Пользователи могут выбрать один из многих алгоритмов вычисления хэша.
    
    Каждый имеет сильные и слабые точки и иногда даёт более качественные и иногда хуже результаты для разных изображений.
    
    Поэтому для определения наилучшего из вас, требуется ручное тестирование.
big_files_mode_combobox_tooltip = Поиск наименьших/наибольших файлов
big_files_mode_label = Проверенные файлы
big_files_mode_smallest_combo_box = Самый маленький
big_files_mode_biggest_combo_box = Крупнейший
main_notebook_duplicates = Файлы-дубликаты
main_notebook_empty_directories = Пустые папки
main_notebook_big_files = Большие файлы
main_notebook_empty_files = Пустые файлы
main_notebook_temporary = Временные файлы
main_notebook_similar_images = Похожие изображения
main_notebook_similar_videos = Похожие видео
main_notebook_same_music = Музыкальные дубликаты
main_notebook_symlinks = Битые симв. ссылки
main_notebook_broken_files = Битые файлы
main_notebook_bad_extensions = Плохие расширения
main_tree_view_column_file_name = Имя файла
main_tree_view_column_folder_name = Имя папки
main_tree_view_column_path = Путь
main_tree_view_column_modification = Дата изменения
main_tree_view_column_size = Размер
main_tree_view_column_similarity = Сходство
main_tree_view_column_dimensions = Размеры
main_tree_view_column_title = Заголовок
main_tree_view_column_artist = Исполнитель
main_tree_view_column_year = Год
main_tree_view_column_bitrate = Битрейт
main_tree_view_column_length = Длительность
main_tree_view_column_genre = Жанр
main_tree_view_column_symlink_file_name = Имя файла символьной ссылки
main_tree_view_column_symlink_folder = Папка Symlink
main_tree_view_column_destination_path = Путь назначения
main_tree_view_column_type_of_error = Тип ошибки
main_tree_view_column_current_extension = Текущее расширение
main_tree_view_column_proper_extensions = Правильное расширение
main_label_check_method = Метод проверки
main_label_hash_type = Тип хэша
main_label_hash_size = Размер хэша
main_label_size_bytes = Размер (байт)
main_label_min_size = Мин
main_label_max_size = Макс
main_label_shown_files = Количество отображаемых файлов
main_label_resize_algorithm = Алгоритм масштабирования
main_label_similarity = Сходство{"   "}
main_check_box_broken_files_audio = Звук
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Архивировать
main_check_box_broken_files_image = Изображение
check_button_general_same_size = Игнорировать одинаковый размер
check_button_general_same_size_tooltip = Игнорировать файлы с одинаковым размером в результатах - обычно это 1:1 дубликаты
main_label_size_bytes_tooltip = Размер файлов, которые будут просканированы
# Upper window
upper_tree_view_included_folder_column_title = Папки для поиска
upper_tree_view_included_reference_column_title = Содержит оригиналы
upper_recursive_button = В подпапках
upper_recursive_button_tooltip = При включённой опции будут также искаться файлы, не находящиеся непосредственно в корне выбранной папки, т. е. в других подпапках данной папки и их подпапках.
upper_manual_add_included_button = Прописать вручную
upper_add_included_button = Добавить
upper_remove_included_button = Удалить
upper_manual_add_excluded_button = Ручное добавление
upper_add_excluded_button = Добавить
upper_remove_excluded_button = Удалить
upper_manual_add_included_button_tooltip =
    Добавьте имя каталога для поиска вручную.
    
    Чтобы добавить несколько путей одновременно, разделите их на ;
    
    /home/roman;/home/rozkaz добавит два каталога /home/roman и /home/rozkaz
upper_add_included_button_tooltip = Добавить новый каталог для поиска.
upper_remove_included_button_tooltip = Исключить каталог из поиска.
upper_manual_add_excluded_button_tooltip =
    Добавьте вручную исключенное имя каталога.
    
    Чтобы добавить несколько путей одновременно, разделите их на ;
    
    /home/roman;/home/krokiet добавит два каталога /home/roman и /home/keokiet
upper_add_excluded_button_tooltip = Добавить каталог, исключаемый из поиска.
upper_remove_excluded_button_tooltip = Убрать каталог из исключенных.
upper_notebook_items_configuration = Параметры поиска
upper_notebook_excluded_directories = Исключённые каталоги
upper_notebook_included_directories = Включённые каталоги
upper_allowed_extensions_tooltip =
    Включаемые расширения должны быть разделены запятыми (по умолчанию ищутся файлы с любыми расширениями).
        
    Макросы IMAGE, VIDEO, MUSIC, TEXT добавляют сразу несколько расширений.
        
    Пример использования: «.exe, IMAGE, VIDEO, .rar, 7z» — это означает, что будут сканироваться файлы изображений (напр. jpg, png), видео (напр. avi, mp4), exe, rar и 7z.
upper_excluded_extensions_tooltip =
    Список отключенных файлов, которые будут игнорироваться в сканировании.
    
    При использовании разрешенных и отключенных расширений этот файл имеет более высокий приоритет, поэтому файл не будет проверяться.
upper_excluded_items_tooltip =
    Исключаемые элементы должны содержать маску «*» и быть разделены запятыми.
    Это медленнее, чем «Исключённые каталоги», поэтому используйте осторожно.
upper_excluded_items = Исключённые элементы:
upper_allowed_extensions = Допустимые расширения:
upper_excluded_extensions = Отключенные расширения:
# Popovers
popover_select_all = Выбрать все
popover_unselect_all = Снять выделение
popover_reverse = Обратить выделение
popover_select_all_except_oldest = Выделить все, кроме старых
popover_select_all_except_newest = Выделить все, кроме новых
popover_select_one_oldest = Выбрать один старый
popover_select_one_newest = Выбрать один новый
popover_select_custom = Выбрать произвольный
popover_unselect_custom = Снять выбор
popover_select_all_images_except_biggest = Выделить все, кроме наибольшего
popover_select_all_images_except_smallest = Выделить все, кроме наименьшего
popover_custom_path_check_button_entry_tooltip =
    Выбор записей на основе пути.
    
    Пример:
    /home/pimpek/rzecz.txt можно найти с помощью /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Выбор записей по именам файлов.
    
    Пример:
    /usr/ping/pong.txt можно найти с помощью *ong*
popover_custom_regex_check_button_entry_tooltip =
    Выбор записей с помощью регулярного выражения.
    
    В этом режиме искомый текст представляет собой путь с именем.
     
    Пример:
    /usr/bin/ziemniak.txt можно найти с помощью выражения /ziem[a-z]+
     
    По умолчанию используется синтаксис регулярных выражений Rust. Подробнее об этом можно прочитать здесь: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Включает регистрозависимый поиск.
    
    При отключённой опции «/home/*» будет соответствовать как «/home/roman», так и «/HoMe/roman».
popover_custom_not_all_check_button_tooltip =
    Запрет выбора всех записей в группе.
    
    Эта опция включена по умолчанию, потому что в большинстве ситуаций вам не надо удалять и оригиналы, и дубликаты — обычно оставляют хотя бы один файл.
    
    ВНИМАНИЕ. Этот параметр не работает, если вы уже вручную выбрали все результаты в группе.
popover_custom_regex_path_label = Путь
popover_custom_regex_name_label = Имя
popover_custom_regex_regex_label = Путь с рег. выраж. + имя
popover_custom_case_sensitive_check_button = С учётом регистра
popover_custom_all_in_group_label = Не выбирать все записи в группе
popover_custom_mode_unselect = Снять выбор
popover_custom_mode_select = Выбрать произвольный
popover_sort_file_name = Имя файла
popover_sort_folder_name = Название папки
popover_sort_full_name = Полное имя
popover_sort_size = Размер
popover_sort_selection = Выбранные объекты
popover_invalid_regex = Некорректное регулярное выражение
popover_valid_regex = Корректное регулярное выражение
# Bottom buttons
bottom_search_button = Искать
bottom_select_button = Выбрать
bottom_delete_button = Удалить
bottom_save_button = Сохранить
bottom_symlink_button = Симв. ссылка
bottom_hardlink_button = Жёст. ссылка
bottom_move_button = Переместить
bottom_sort_button = Сортировать
bottom_search_button_tooltip = Начать поиск
bottom_select_button_tooltip = Выберите записи. Только выбранные файлы/папки будут доступны для последующей обработки.
bottom_delete_button_tooltip = Удалить выбранные файлы/папки.
bottom_save_button_tooltip = Сохранить данные о поиске в файл
bottom_symlink_button_tooltip =
    Создать символьные ссылки.
    Работает, только когда выбрано не менее двух результатов в группе.
    Первый результат оставляется, а второй и последующие делаются символьными ссылками на первый.
bottom_hardlink_button_tooltip =
    Создать жёсткие ссылки.
    Работает, только когда выбрано не менее двух результатов в группе.
    Первый результат оставляется, а второй и последующие делаются жёсткими ссылками на первый.
bottom_hardlink_button_not_available_tooltip =
    Создание жестких ссылок.
    Кнопка отключена, так как невозможно создать жёсткие ссылки.
    Связи работают только с правами администратора в Windows, поэтому не забудьте запустить приложение от имени администратора.
    Если приложение уже работает с такими привилегиями, проверьте аналогичные проблемы на Github.
bottom_move_button_tooltip =
    Перемещение файлов в выбранный каталог.
    Копирует все файлы в папку без сохранения структуры дерева каталогов.
    При попытке переместить два файла с одинаковым именем в одну и ту же папку второй не будет перемещён и появится сообщение об ошибке.
bottom_sort_button_tooltip = Сортировка файлов/папок по выбранному методу.
bottom_show_errors_tooltip = Показать/скрыть нижнюю текстовую панель.
bottom_show_upper_notebook_tooltip = Показать/скрыть верхнюю панель блокнота.
# Progress Window
progress_stop_button = Остановить
progress_stop_additional_message = Стоп запрошен
# About Window
about_repository_button_tooltip = Ссылка на страницу репозитория с исходным кодом.
about_donation_button_tooltip = Ссылка на страницу пожертвований.
about_instruction_button_tooltip = Ссылка на страницу инструкций.
about_translation_button_tooltip = Ссылка на страницу Crowdin с переводами приложений. Официально поддерживаются английский и польский языки.
about_repository_button = Репозиторий
about_donation_button = Пожертвование
about_instruction_button = Инструкция
about_translation_button = Перевод
# Header
header_setting_button_tooltip = Открыть окно настроек.
header_about_button_tooltip = Открыть окно с информацией о приложении.

# Settings


## General

settings_number_of_threads = Количество использованных потоков
settings_number_of_threads_tooltip = Количество используемых потоков. Установите 0, чтобы использовать все доступные потоки.
settings_use_rust_preview = Использовать внешние библиотеки вместо gtk для загрузки предпросмотра
settings_use_rust_preview_tooltip =
    Использование превью gtk иногда будет быстрее и поддерживать больше форматов, но иногда это может быть и наоборот.
    
    Если у вас возникли проблемы с загрузкой предпросмотра, вы можете попробовать изменить эту настройку.
    
    На не-linux системах рекомендуется использовать эту опцию, потому что gtk-pixbuf не всегда доступен там, поэтому отключение этой опции не будет загружать превью некоторых изображений.
settings_label_restart = Вам нужно перезапустить приложение, чтобы применить настройки!
settings_ignore_other_filesystems = Игнорировать другие файловые системы (только Linux)
settings_ignore_other_filesystems_tooltip =
    игнорирует файлы, которые находятся в той же файловой системе, что и поисковые директории.
    
    Работает так же, как и команда 'xdev' в команде 'находить'
settings_save_at_exit_button_tooltip = Сохранить конфигурацию в файл при закрытии приложения.
settings_load_at_start_button_tooltip =
    Загрузить конфигурацию из файла при открытии приложения.
    
    Если не включено, будут использоваться настройки по умолчанию.
settings_confirm_deletion_button_tooltip = Показать окно подтверждения при нажатии на кнопку удаления.
settings_confirm_link_button_tooltip = Показывать окно подтверждения при нажатии кнопки жесткой/символической ссылки.
settings_confirm_group_deletion_button_tooltip = Показывать окно предупреждения при попытке удалить все записи из группы.
settings_show_text_view_button_tooltip = Показать текстовую панель в нижней части интерфейса.
settings_use_cache_button_tooltip = Использовать файловый кэш.
settings_save_also_as_json_button_tooltip = Сохранять кэш в формат JSON (человекочитаемый). Его содержимое можно изменять. Кэш из этого файла будет автоматически прочитан приложением, если бинарный кэш (с расширением bin) отсутствует.
settings_use_trash_button_tooltip = Перемещать файлы в корзину вместо их безвозвратного удаления.
settings_language_label_tooltip = Язык пользовательского интерфейса.
settings_save_at_exit_button = Сохранять конфигурацию при закрытии приложения
settings_load_at_start_button = Загружать конфигурацию при открытии приложения
settings_confirm_deletion_button = Показывать подтверждение при удалении любых файлов
settings_confirm_link_button = Показывать окно подтверждения при создании жёстких или символьных ссылок на файлы
settings_confirm_group_deletion_button = Показывать подтверждение при удалении всех файлов в группе
settings_show_text_view_button = Показывать нижнюю текстовую панель
settings_use_cache_button = Использовать кэш
settings_save_also_as_json_button = Также сохранять кэш в файл JSON
settings_use_trash_button = Перемещать удаляемые файлы в корзину
settings_language_label = Язык
settings_multiple_delete_outdated_cache_checkbutton = Автоматически удалять устаревшие записи кэша
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Удалить устаревшие результаты кеша, указывающие на несуществующие файлы.
    
    Когда опция включена, приложение проверяет при загрузке записей, указывают ли они на доступные файлы (недостающие файлы игнорируются).
    
    Отключение этой опции помогает при сканировании файлов на внешних носителях, чтобы информация о них не была очищена при следующем сканировании.
    
    При наличии сотен тысяч записей в кэше рекомендуется включить эту опцию, чтобы ускорить загрузку и сохранение кэша в начале и конце сканирования.
settings_notebook_general = Общие настройки
settings_notebook_duplicates = Дубликаты
settings_notebook_images = Похожие изображения
settings_notebook_videos = Похожие видео

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Показывать предварительный просмотр справа (при выборе файла изображения).
settings_multiple_image_preview_checkbutton = Показывать предпросмотр изображения
settings_multiple_clear_cache_button_tooltip =
    Очистка устаревших записей кэша вручную.
    Следует использовать только в том случае, если автоматическая очистка отключена.
settings_multiple_clear_cache_button = Удалить устаревшие результаты из кэша.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Скрыть все файлы, кроме первого, если все они указывают на одни и те же данные (связаны жёсткой ссылкой).
    
    Пример: если (на диске) семь файлов связаны жёсткой ссылкой с определёнными данными, а ещё один файл содержат те же данные, но на другом inode, то в средстве поиска дубликатов будут показаны только этот последний уникальный файл и один файл из являющихся жёсткой ссылкой.
settings_duplicates_minimal_size_entry_tooltip =
    Установить минимальный размер кэшируемого файла.
    
    Выбор меньшего значения приведёт к созданию большего количества записей. Это ускорит поиск, но замедлит загрузку/сохранение кэша.
settings_duplicates_prehash_checkbutton_tooltip =
    Включает кэширование предварительного хэша (предхэша), вычисляемого из небольшой части файла, что позволяет быстрее исключать из анализа отличающиеся файлы.
    
    По умолчанию отключено, так как в некоторых ситуациях может замедлять работу.
    
    Настоятельно рекомендуется использовать его при сканировании сотен тысяч или миллионов файлов, так как это может ускорить поиск в разы.
settings_duplicates_prehash_minimal_entry_tooltip = Минимальный размер кэшируемого элемента.
settings_duplicates_hide_hard_link_button = Скрыть жёсткие ссылки (только для Linux и macOS)
settings_duplicates_prehash_checkbutton = Кэшировать предхэш
settings_duplicates_minimal_size_cache_label = Минимальный размер (байт) кэшируемых файлов
settings_duplicates_minimal_size_cache_prehash_label = Минимальный размер (байт) файлов для кэша предхэша

## Saving/Loading settings

settings_saving_button_tooltip = Сохранить текущую конфигурацию настроек в файл.
settings_loading_button_tooltip = Загрузить настройки из файла и заменить ими текущую конфигурацию.
settings_reset_button_tooltip = Сбросить текущую конфигурацию на конфигурацию по умолчанию.
settings_saving_button = Сохранить конфигурацию
settings_loading_button = Загрузить конфигурацию
settings_reset_button = Сбросить настройки

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Открыть папку, в которой хранятся текстовые файлы кеша.
    
    Изменение файлов кэша может привести к отображению неверных результатов, однако изменение пути может сэкономить время при перемещении большого количества файлов в другое место.
    
    Вы можете копировать эти файлы между компьютерами, чтобы сэкономить время на повторном сканировании файлов (конечно, если они имеют схожую структуру каталогов).
    
    В случае возникновения проблем с кэшем эти файлы можно удалить. Приложение автоматически пересоздаст их.
settings_folder_settings_open_tooltip =
    Открывает папку, в которой хранится конфигурация Czkawka.
    
    ВНИМАНИЕ. Ручное изменение конфигурации может нарушить функционирование программы.
settings_folder_cache_open = Открыть папку кэша
settings_folder_settings_open = Открыть папку настроек
# Compute results
compute_stopped_by_user = Поиск был остановлен пользователем
compute_found_duplicates_hash_size = Найдено дубликатов: { $number_files } (групп: { $number_groups }), размер: { $size }
compute_found_duplicates_name = Найдено: { $number_files } дубликат(а/ов) (групп: { $number_groups })
compute_found_empty_folders = Найдено пустых папок: { $number_files }
compute_found_empty_files = Найдено пустых файлов: { $number_files }
compute_found_big_files = Найдено больших файлов: { $number_files }
compute_found_temporary_files = Найдено временных файлов: { $number_files }
compute_found_images = Найдено похожих изображений: { $number_files } (групп: { $number_groups })
compute_found_videos = Найдено похожих видео: { $number_files } (групп: { $number_groups })
compute_found_music = Найдено похожих музыкальных файлов: { $number_files } (групп: { $number_groups })
compute_found_invalid_symlinks = Найдено битых символьных ссылок: { $number_files }
compute_found_broken_files = Найдено битых файлов: { $number_files }
compute_found_bad_extensions = Найдено { $number_files } файлов с недопустимыми расширениями
# Progress window
progress_scanning_general_file = Сканирование файла: { $file_number }
progress_scanning_extension_of_files = Проверка расширения { $file_checked }/{ $all_files } файла
progress_scanning_broken_files = Проверка файла: { $file_checked }/{ $all_files }
progress_scanning_video = Хэширование видео: { $file_checked }/{ $all_files }
progress_scanning_image = Хэширование изображения: { $file_checked }/{ $all_files }
progress_comparing_image_hashes = Сравнение хэша изображений: { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Сравнение тегов { $file_checked }/{ $all_files } музыкального файла
progress_scanning_music_tags = Чтение тэгов музыкальных файлов: { $file_checked }/{ $all_files }
progress_scanning_music_content_end = Сравнение отпечатка пальца из { $file_checked }/{ $all_files } музыкального файла
progress_scanning_music_content = Вычисление отпечатка пальца из { $file_checked }/{ $all_files } музыкального файла
progress_scanning_empty_folders = Сканирование папки { $folder_number }
progress_scanning_size = Сканирование размера файла { $file_number }
progress_scanning_size_name = Сканирование имени и размера файла { $file_number }
progress_scanning_name = Сканирование имени файла { $file_number }
progress_analyzed_partial_hash = Анализ частичного хэша файла { $file_checked }/{ $all_files }
progress_analyzed_full_hash = Анализ полного хэша файла { $file_checked }/{ $all_files }
progress_prehash_cache_loading = Загрузка кэша prehash
progress_prehash_cache_saving = Сохранение кэша prehash
progress_hash_cache_loading = Загрузка хеш-кэша
progress_hash_cache_saving = Сохранение хэша
progress_cache_loading = Загрузка кэша
progress_cache_saving = Сохранение кэша
progress_current_stage = Текущий этап:{ " " }
progress_all_stages = Все этапы:{ " " }
# Saving loading 
saving_loading_saving_success = Конфигурация сохранена в файл { $name }.
saving_loading_saving_failure = Не удалось сохранить данные конфигурации в файл { $name }.
saving_loading_reset_configuration = Текущая конфигурация была удалена.
saving_loading_loading_success = Настройки приложения корректно загружены.
saving_loading_invalid_string = Для ключа «{ $key }» найден недопустимый результат: «{ $result }» не является строковым выражением.
saving_loading_invalid_int = Для ключа «{ $key }» найден недопустимый результат: «{ $result }» не является целым числом.
saving_loading_invalid_bool = Для ключа «{ $key }» найден недопустимый результат: «{ $result }» не является булевым значением.
saving_loading_decode_problem_bool = Не удалось декодировать булево значение из ключа «{ $key }»: найдено «{ $result }», но допустимы лишь значения 0, 1, true или false.
saving_loading_saving_same_keys = Попытка сохранить настройки с дублирующимся ключом «{ $key }».
saving_loading_failed_to_get_home_directory = Не удалось получить домашний каталог для открытия/сохранения конфигурационного файла.
saving_loading_folder_config_instead_file = Невозможно создать или открыть файл конфигурации в пути «{ $path }», потому что уже есть такая папка.
saving_loading_failed_to_create_configuration_folder = Не удалось создать папку конфигурации «{ $path }». Причина: «{ $reason }».
saving_loading_failed_to_create_config_file = Не удалось создать файл конфигурации «{ $path }». Причина: «{ $reason }».
saving_loading_failed_to_read_config_file = Невозможно загрузить конфигурацию из «{ $path }», так как или такого файла не существует, или это не файл.
saving_loading_failed_to_read_data_from_file = Невозможно прочитать данные из файла «{ $path }». Причина: «{ $reason }».
saving_loading_orphan_data = Найдены ничему не принадлежащие данные «{ $data }» в строке «{ $line }».
saving_loading_not_valid = Параметра «{ $data }» не существует в текущей версии приложения.
# Invalid symlinks
invalid_symlink_infinite_recursion = Бесконечная рекурсия
invalid_symlink_non_existent_destination = Не найден конечный файл
# Other
selected_all_reference_folders = Невозможно начать поиск, когда все каталоги установлены как папки со ссылками
searching_for_data = Поиск данных может занять некоторое время — пожалуйста, подождите...
text_view_messages = СООБЩЕНИЯ
text_view_warnings = ПРЕДУПРЕЖДЕНИЯ
text_view_errors = ОШИБКИ
about_window_motto = Эта программа бесплатна для использования и всегда будет оставаться таковой.
# Various dialog
dialogs_ask_next_time = Всегда спрашивать
delete_file_failed = Не удалось удалить файл { $name }. Причина: { $reason }
delete_title_dialog = Подтверждение удаления
delete_question_label = Вы уверены, что хотите удалить файлы?
delete_all_files_in_group_title = Подтверждение удаления всех файлов в группе
delete_all_files_in_group_label1 = В некоторых группах были выбраны все записи.
delete_all_files_in_group_label2 = Вы уверены, что хотите удалить их?
delete_folder_failed = Не удалось удалить папку { $dir }, так как или папки не существует, или у вас нет разрешения на изменение, или папка не пуста.
delete_items_label = Будет удалено файлов: { $items }.
delete_items_groups_label = Будет удалено файлов: { $items } (групп: { $groups }).
hardlink_failed = Не удалось создать жёсткую ссылку
hard_sym_invalid_selection_title_dialog = Неверный выбор в некоторых группах
hard_sym_invalid_selection_label_1 = В некоторых группах выбрана только одна запись — они будут проигнорированы.
hard_sym_invalid_selection_label_2 = Чтобы жёстко или символьно связать эти файлы, необходимо выбрать как минимум два результата в группе.
hard_sym_invalid_selection_label_3 = Первый в группе признан в качестве оригинала и не будет изменён, но второй и последующие модифицированы.
hard_sym_link_title_dialog = Подтверждение связывания ссылкой
hard_sym_link_label = Вы уверены, что хотите связать эти файлы?
move_folder_failed = Не удалось переместить папку { $name }. Причина: { $reason }
move_file_failed = Не удалось переместить файл { $name }. Причина: { $reason }
move_files_title_dialog = Выберите папку, в которую вы хотите переместить дублирующиеся файлы
move_files_choose_more_than_1_path = Можно выбрать только один путь для копирования дубликатов файлов, но выбрано { $path_number }.
move_stats = Удалось переместить без ошибок элементов: { $num_files }/{ $all_files }
save_results_to_file = Результаты сохранены в txt и json файлах в папке { $name }.
search_not_choosing_any_music = ОШИБКА: Необходимо выбрать как минимум один флажок с типами поиска музыки.
search_not_choosing_any_broken_files = ОШИБКА: Вы должны выбрать хотя бы один флажок с типом проверенных ошибочных файлов.
include_folders_dialog_title = Папки для включения
exclude_folders_dialog_title = Папки для исключения
include_manually_directories_dialog_title = Добавить папку вручную
cache_properly_cleared = Кэш успешно очищен
cache_clear_duplicates_title = Очистка кэша дубликатов
cache_clear_similar_images_title = Очистка кэша похожих изображений
cache_clear_similar_videos_title = Очистка кэша похожих видео
cache_clear_message_label_1 = Убрать из кэша устаревшие записи?
cache_clear_message_label_2 = Это действие удалит все записи кэша, указывающие на недоступные файлы.
cache_clear_message_label_3 = Это может немного ускорить загрузку/сохранение кэша.
cache_clear_message_label_4 = ВНИМАНИЕ. Это действие удалит все кэшированные данные с отключённых внешних дисков. Хэши для файлов на этих носителях будет необходимо сгенерировать заново.
# Show preview
preview_image_resize_failure = Не удалось изменить размер изображения { $name }.
preview_image_opening_failure = Не удалось открыть изображение { $name }. Причина: { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Группа { $current_group }/{ $all_groups } (изображений: { $images_in_group })
compare_move_left_button = L
compare_move_right_button = R
