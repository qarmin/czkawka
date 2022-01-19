# Window titles
window_settings_title = Опції
window_main_title = Czkawka (гикавка)
window_progress_title = Сканування
window_compare_images = Порівняння зображень
# General
general_ok_button = Гаразд
general_close_button = Закрити
# Main window
music_title_checkbox = Найменування
music_artist_checkbox = Художник
music_album_title_checkbox = Назва альбому
music_album_artist_checkbox = Виконавець альбому
music_year_checkbox = Рік
music_comparison_checkbox = Приблизне порівняння
music_comparison_checkbox_tooltip =
    Він шукає схожі музичні файли використовуючи AI, який використовує машинне навчання для видалення дужок з фрази, наприклад за допомогою цієї опції увімкнено, файли у питанні будуть розглядатися повтори:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Ім'я
duplicate_mode_size_combo_box = Розмір
duplicate_mode_hash_combo_box = Хеш
duplicate_hash_type_tooltip =
    Czkawka пропонує 3 типи хешу, які можуть бути використані:
    
    Blake3 - криптографічна геш-функція. Він використовується як хеш-алгоритм за замовчуванням, тому що він дуже швидкий.
    
    CRC32 - проста геш-функція. Він повинен бути швидшим, ніж Blake3, але, ймовірно, має дуже рідко деякі зіткнення.
    
    XXH3 - дуже схожий на випадок продуктивності та якості хешування для Blake3, тому такі режими можна легко використовувати.
duplicate_check_method_tooltip =
    На даний момент Czkawka пропонує три типи методу для пошуку дублікатів:
    
    Ім'я - Пошук файлів, які мають однакове ім'я.
    
    Розмір - видаляє файли, які мають однаковий розмір.
    
    Hash - Шукає файли, які мають той самий вміст. Цей режим геш-файл, і пізніше порівняйте цей хеш, щоб знайти дублікати. Цей режим є найбезпечнішим способом пошуку дублікатів. Додатково використання кешу утилізається, тому подальше сканування тих самих даних мають бути набагато швидшими першими.
image_hash_size_tooltip =
    Чкавка пропонує змінювати розмір згенерованої хешу для кожного зображення. Big-хеш дозволяє знаходити зображення з меншою кількістю відмінностей між зображеннями, але також він є трохи повільнішим.
    
    Значення за замовчуванням для хешу - 8 байт, що дозволяє знайти дуже подібні та різні зображення. 16 і 32 хешів слід використовувати тільки для майже ідентичних образів. 64 байти не слід використовувати, за винятком ситуації, коли потрібно дуже мало відмінностей
image_resize_filter_tooltip = Для обчислення хеш-зображення бібліотека повинна спочатку змінити розмір. Залежно від обраного алгоритму, результатом цього зображення буде виглядати зовсім інакше. Найшвидший алгоритм для використання, але також той, який дає найгірші результати є Найближчими.
image_hash_alg_tooltip = Користувачі можуть вибирати один з багатьох алгоритмів обчислення. Кожен має і сильні і слабкі точки, і іноді дасть кращі результати для різних зображень, таким чином, щоб обрати найкраще, необхідно ручне тестування.
main_notebook_image_fast_compare = Швидко порівняти
main_notebook_image_fast_compare_tooltip =
    Пошук попелу пришвидшення і порівняння хешів.
    
    У протилежному напрямку до нормального режиму, де кожен хеш порівнюються між собою x рази, де x подібність вибирається користувачем в цьому режимі завжди використовується лише одне порівняння.
    
    Рекомендується порівнювати >10000 зображень з нульовою (Дуже високою) схожістю.
main_notebook_duplicates = Дублікати файлів
main_notebook_empty_directories = Порожні каталоги
main_notebook_big_files = Великі файли
main_notebook_empty_files = Порожні файли
main_notebook_temporary = Тимчасові файли
main_notebook_similar_images = Схожі зображення
main_notebook_similar_videos = Схожі відео
main_notebook_same_music = Музичні дублікати
main_notebook_symlinks = Неприпустимі символічні посилання
main_notebook_broken_files = Пошкоджені файли
main_tree_view_column_file_name = Назва файлу
main_tree_view_column_folder_name = Ім'я папки
main_tree_view_column_path = Шлях
main_tree_view_column_modification = Дата зміни
main_tree_view_column_size = Розмір
main_tree_view_column_similarity = Подібність
main_tree_view_column_dimensions = Розміри
main_tree_view_column_title = Найменування
main_tree_view_column_artist = Художник
main_tree_view_column_year = Рік
main_tree_view_column_album_title = Назва альбому
main_tree_view_column_album_artist = Виконавець альбому
main_tree_view_column_symlink_file_name = Ім'я файла символьного посилання
main_tree_view_column_symlink_folder = Симлнік
main_tree_view_column_destination_path = Шлях призначення
main_tree_view_column_type_of_error = Тип помилки
main_label_check_method = Перевірити метод
main_label_hash_type = Тип хешу
main_label_hash_size = Розмір хешу
main_label_size_bytes = Розмір(байт)
main_label_min_size = Мін
main_label_max_size = Макс
main_label_shown_files = Кількість показаних файлів
main_label_resize_algorithm = Змінити розмір алгоритму
main_label_similarity = Подібність{ " " }
check_button_general_same_size = Ігнорувати той самий розмір
check_button_general_same_size_tooltip = Ігнорувати від результатів, файли, які мають ідентичний розмір - зазвичай це 1:1 дублікатів
main_label_size_bytes_tooltip = Розмір файлів, які будуть використовуватися в скануванні
# Upper window
upper_tree_view_included_folder_column_title = Знайти теки
upper_tree_view_included_reference_column_title = Теки посилань
upper_recursive_button = Рекурсивний
upper_recursive_button_tooltip = Якщо вибрано, шукати також файли, які не розміщені безпосередньо під вибраними теками.
upper_manual_add_included_button = Ручне додавання
upper_add_included_button = Додати
upper_remove_included_button = Видалити
upper_manual_add_excluded_button = Ручне додавання
upper_add_excluded_button = Додати
upper_remove_excluded_button = Видалити
upper_manual_add_included_button_tooltip = Дозволяє додати назву каталогу для пошуку вручну.
upper_add_included_button_tooltip = Додати нову директорію для пошуку.
upper_remove_included_button_tooltip = Видалити теку з пошуку.
upper_manual_add_excluded_button_tooltip = Дозволяє додати виключені назви каталогу вручну.
upper_add_excluded_button_tooltip = Додати теку щоб бути виключеною для пошуку.
upper_remove_excluded_button_tooltip = Видалити каталог з виключених.
upper_notebook_items_configuration = Налаштування елементів
upper_notebook_excluded_directories = Виключені каталоги
upper_notebook_included_directories = Включені каталоги
upper_allowed_extensions_tooltip =
    Дозволені розширення повинні бути розділені комами (по замовчуванню всі доступні).
    
    Macros IMAGE, VIDEO, MUSIC, TEXT який одночасно додає кілька розширень.
    
    приклад ".exe, IMAGE, VIDEO, .rar, 7z" - це означає, що зображення (наприклад . jpg, png), video(напр. avi, mp4), exe, rar та 7z файли буде відскановано.
upper_excluded_items_tooltip =
    Виключені елементи мають містити символ *, який має бути розділений комами.
    Це число є повільнішим, ніж виключені каталоги, тому використовуйте його обережно.
upper_excluded_items = Виключені елементи:
upper_allowed_extensions = Дозволені розширення:
# Popovers
popover_select_all = Виділити все
popover_unselect_all = Прибрати всі
popover_reverse = Зворотній вибір
popover_select_all_except_oldest = Вибрати всі крім старіших
popover_select_all_except_newest = Вибрати всі крім найновіших
popover_select_one_oldest = Виберіть один найстаріший
popover_select_one_newest = Оберіть один найновіший
popover_select_custom = Вибрати власний
popover_unselect_custom = Скасувати вибір
popover_select_all_images_except_biggest = Вибрати всі, крім найбільших
popover_select_all_images_except_smallest = Вибрати всі крім найменших
popover_custom_path_check_button_entry_tooltip =
    Дозволяє вибрати записи за його шляхом.
    
    Приклад використання:
    /home/pimpek/rzech.txt можна знайти з /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Дозволяє вибрати записи за іменами файлів.
    
    Приклад використання:
    /usr/ping/pong.txt можна знайти в *ong*
popover_custom_regex_check_button_entry_tooltip =
    Дозволяє вибрати записи за вказаним Regex.
    
    За допомогою цього режиму пошук в тексті є шлях до імені.
    
    Приклад використання:
    /usr/bin/ziemniak. xt можна знайти за допомогою /ziem[a-z]+
    
    Це використовується за замовчуванням regex, щоб прочитати більше про це в https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Забороняє вибрати всі записи в групі.
    
    Це увімкнено за замовчуванням, оскільки у більшості ситуацій користувач не хоче видаляти і оригінальні, і дублювати файли, але хочете залишити принаймні один файл.
    
    Увага: це налаштування не працює, якщо користувач не вибрав усі результати у групі вручну.
popover_custom_regex_path_label = Шлях
popover_custom_regex_name_label = Ім'я
popover_custom_regex_regex_label = Шлях до регулярного виразу + ім'я
popover_custom_all_in_group_label = Не вибирати усі записи в групі
popover_custom_mode_unselect = Зняти вибір з користувацьких
popover_custom_mode_select = Вибрати користувача
popover_invalid_regex = Неприпустимий регулярний вираз
popover_valid_regex = Значення регулярного виразу дійсний
# Bottom buttons
bottom_search_button = Пошук
bottom_select_button = Вибрати
bottom_delete_button = Видалити
bottom_save_button = Зберегти
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Пересунути
bottom_search_button_tooltip = Почніть пошук файлів/тек.
bottom_select_button_tooltip = Вибирає записи. Лише обрані файли/теки можуть бути пізніше оброблені.
bottom_delete_button_tooltip = Видалити вибрані файли/папки.
bottom_save_button_tooltip = Зберегти дані про пошук у файл
bottom_symlink_button_tooltip =
    Створює символічні посилання.
    Працює тільки тоді, коли вибрано щонайменше 2 результати в групі.
    Перший є незміненим, другий і пізніше є символічними для першого.
bottom_hardlink_button_tooltip =
    Створює жорсткі посилання.
    працює лише тоді, коли вибрано щонайменше 2 результати в групі.
    Перший без змін і потім важко з'єднати на перший.
bottom_move_button_tooltip =
    Переміщує файли до вибраної папки.
    Вони копіюють всі файли в папку без збереження дерева каталогів.
    При спробі перемістити 2 файли з однаковим іменем до теки, друге - невдача та показати помилку.
bottom_show_errors_tooltip = Показати/приховати панель помилки.
bottom_show_upper_notebook_tooltip = Показати/приховати верхню панель блокнота.
# Progress Window
progress_stop_button = Зупинити
# About Window
about_repository_button_tooltip = Посилання на сторінку репозиторію з вихідним кодом.
about_donation_button_tooltip = Посилання на сторінку пожертвування.
about_instruction_button_tooltip = Посилання на сторінку інструкцій.
about_translation_button_tooltip = Посилання на сторінку Crowdin з перекладами додатків. Підтримуються офіційно польські та англійські мови, але будь-яка допомога з іншими мовами буде цінується.
about_repository_button = Репозиторій
about_donation_button = Пожертва
about_instruction_button = Інструкція
about_translation_button = Переклад
# Header
header_setting_button_tooltip = Відкриває вікно налаштувань.
header_about_button_tooltip = Відкриває діалогове вікно з інформацією про додаток.

# Settings


## General

settings_save_at_exit_button_tooltip = Зберігає конфігурацію в файл під час закриття додатку.
settings_load_at_start_button_tooltip =
    Завантаження при запуску конфігурації з файлу.
    
    При виборі цього параметра буде завантажувати стандартні налаштування.
settings_confirm_deletion_button_tooltip = Показувати вікно підтвердження при натисканні на кнопку Видалити.
settings_confirm_link_button_tooltip = Показувати діалогове вікно підтвердження при натисканні на кнопку твердого/символічного посилання.
settings_confirm_group_deletion_button_tooltip = Показує діалогове вікно при спробі видалення всіх записів з групи.
settings_show_text_view_button_tooltip = Показує панель помилки знизу.
settings_use_cache_button_tooltip = Опція, яка дозволяє не використовувати функцію кешу.
settings_save_also_as_json_button_tooltip = Зберегти кеш для читуваного людського JSON формату. Можна змінити його вміст. Кеш з цього файлу буде автоматично читати додатком, якщо буде відсутній бінарний формат (з розширенням bit).
settings_use_trash_button_tooltip = Якщо увімкнено, то він буде видаляти файли в смітник, а не постійно.
settings_language_label_tooltip = Вибір мови інтерфейсу з доступних інтерфейсів.
settings_save_at_exit_button = Зберегти конфігурацію при виході
settings_load_at_start_button = Завантажувати конфігурацію при старті
settings_confirm_deletion_button = Показувати вікно підтвердження при видаленні будь-якого файлу
settings_confirm_link_button = Показувати вікно підтвердження при складній/символьних посилань будь-які файли
settings_confirm_group_deletion_button = Показувати вікно підтвердження при видаленні всіх файлів групи
settings_show_text_view_button = Показувати нижню текстову панель
settings_use_cache_button = Використання кешу
settings_save_also_as_json_button = Зберегти кеш також у файл JSON
settings_use_trash_button = Перемістити видалені файли в смітник
settings_language_label = Мова
settings_multiple_delete_outdated_cache_checkbutton = Автоматично видаляти застарілі записи кешу
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Видалення застарілих результатів кешу, які вказують на неіснуючі файли.
    
    При включеному додатку впевніться що програма завантажує записи, що всі точки зможуть проігнорувати пошкоджені файли.
    
    Вимкнення цієї опції допоможе сканувати файли на зовнішньому диску, щоб кеш записів про них не буде очищено у наступному скануванні.
    
    Якщо у вас є сотні тисяч записів в кеші, запропоновано включити цю опцію, пришвидшити завантаження кешу і зберегти з початку та кінця сканування.
settings_notebook_general = Загальні налаштування
settings_notebook_duplicates = Дублює
settings_notebook_images = Схожі зображення
settings_notebook_videos = Схожий відео

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Показує попередній перегляд з правої сторони, при виборі файлу зображення.
settings_multiple_image_preview_checkbutton = Показати попередній перегляд зображення
settings_multiple_clear_cache_button_tooltip =
    Вручну очистити кеш із застарілих записів.
    Слід використовувати тільки якщо автоматичне очищення було вимкнено.
settings_multiple_clear_cache_button = Видалити застарілі результати з кешу зображень

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Приховує всі файли, крім одних, якщо дані знаходяться за однаковими даними(ускладнені).
    
    E.g. у випадку, якщо на диску є 7 файлів, які жорстко пов'язані з конкретними даними і іншим файлом з тими ж даними, але різними в той же час, тоді в дублікаті шукач буде видно лише один унікальний файл і один файл з працьовитостей.
settings_duplicates_minimal_size_entry_tooltip =
    Дозволяє встановити мінімальний розмір файлу, який буде кешовано.
    
    Вибір меншого значення, згенерує більше записів, що дозволить прискорити пошук, але сповільнити завантаження кешу/збереження.
settings_duplicates_prehash_checkbutton_tooltip =
    Дозволяє кешування інформації з невеликої частини файлу, що дозволяє раніше отримувати не дубльовані результати.
    
    Його вимкнено за замовчуванням, оскільки може призвести до уповільнення деяких ситуацій.
    
    Рекомендується використовувати його при скануванні сотень тисяч або мільйонів файлів, тому що можна прискорити пошук кілька разів.
settings_duplicates_prehash_minimal_entry_tooltip = Мінімальний розмір кешованого запису.
settings_duplicates_hide_hard_link_button = Приховати жорсткі посилання (лише Linux та MacOS)
settings_duplicates_prehash_checkbutton = Використовувати вбудований кеш
settings_duplicates_minimal_size_cache_label = Мінімальний розмір файлів в байтах, збережених до кешу
settings_duplicates_minimal_size_cache_prehash_label = Мінімальний розмір файлів в байтах, збережених для кешу

## Saving/Loading settings

settings_saving_button_tooltip = Зберегти поточні налаштування у файл.
settings_loading_button_tooltip = Завантажити параметри з файлу і замінити поточні налаштування.
settings_reset_button_tooltip = Скидання поточної конфігурації до стандартних.
settings_saving_button = Зберегти конфігурацію
settings_loading_button = Завантажити конфігурацію
settings_reset_button = Скидання налаштувань

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Відкриває папку, де зберігається txt-файли з кешем.
    
    Змінення їх може призвести до відображення недійсних результатів, але також зміна бази даних.. Шлях може зберегти час при переміщенні великої кількості файлів до іншого місця.
    
    Ви можете копіювати файли між комп'ютерами, щоб зберегти час на сканування знову файлів (звичайно, якщо вони мають подібну структуру каталогу).
    
    У разі проблем з кешем, ці файли можна видалити, а тому програма автоматично відновить їх.
settings_folder_settings_open_tooltip =
    Відкрита папка, де зберігаються налаштування Czkawkka вручну.
    
    Зміна їх вручну може призвести до переривання вашого робочого процесу.
settings_folder_cache_open = Відкрити теку кешу
settings_folder_settings_open = Відкрити папку налаштувань
# Compute results
compute_stopped_by_user = Пошук був зупинений користувачем
compute_found_duplicates_hash_size = Знайдено { $number_files } дублікатів у { $number_groups } групах за допомогою { $size }
compute_found_duplicates_name = Знайдено { $number_files } дублікатів у { $number_groups } групах
compute_found_empty_folders = Знайдено { $number_files } порожніх папок
compute_found_empty_files = Знайдено { $number_files } порожніх файлів
compute_found_big_files = Знайдено { $number_files } великих файлів
compute_found_temporary_files = Знайдено { $number_files } тимчасових файлів
compute_found_images = Знайдено { $number_files } аналогічних зображень в { $number_groups } групах
compute_found_videos = Знайдено { $number_files } схожих відео в { $number_groups } групах
compute_found_music = Знайдено { $number_files } аналогічних музичних файлів в { $number_groups } групах
compute_found_invalid_symlinks = Знайдено { $number_files } неприпустимі символічні посилання
compute_found_broken_files = Знайдено { $number_files } зламаних файлів
# Progress window
progress_scanning_general_file = Сканування файлу { $file_number }
progress_scanning_broken_files = Перевірка { $file_checked }/{ $all_files } файлу
progress_scanning_video = Завдяки { $file_checked }/{ $all_files } відео
progress_scanning_image = Через { $file_checked }/{ $all_files } зображення
progress_comparing_image_hashes = Порівнювати { $file_checked }/{ $all_files } хеш зображення
progress_scanning_music_tags_end = Порівняння тегів { $file_checked }/{ $all_files } музичного файлу
progress_scanning_music_tags = Читання тегів { $file_checked }/{ $all_files } музичний файл
progress_scanning_empty_folders = Сканування { $folder_number } папки
progress_scanning_size = Сканування розміру файлу { $file_number }
progress_scanning_name = Сканування імені файлу { $file_number }
progress_analyzed_partial_hash = Проаналізовано частковий хеш { $file_checked }/{ $all_files } файлів
progress_analyzed_full_hash = Проаналізовано повний хеш { $file_checked }/{ $all_files } файлів
progress_current_stage = Поточна зірочка:{ " "  }
progress_all_stages = Всі стаги:{ " " }
# Saving loading 
saving_loading_saving_success = Збережено конфігурацію в файл { $name }.
saving_loading_saving_failure = Не вдалося зберегти дані конфігурації до файлу { $name }.
saving_loading_reset_configuration = Поточна конфігурація була очищена.
saving_loading_loading_success = Конфігурація завантажених об'єктів.
saving_loading_invalid_string = Для ключа "{ $key }" знайдено неприпустимий результат - "{ $result }", який не є рядком.
saving_loading_invalid_int = Для ключа "{ $key }" знайдено неприпустимий результат - "{ $result }", який не є цілим числом.
saving_loading_invalid_bool = Для ключа "{ $key }" знайдено неприпустимий результат - "{ $result }", який не є булом.
saving_loading_decode_problem_bool = Не вдалося декодувати bool з ключа "{$key}", знайдено "{$result}", але дозволені значення: 0, 1, true або false.
saving_loading_saving_same_keys = Спроба зберегти налаштування за допомогою дубльованого ключа "{ $key }".
saving_loading_failed_to_get_home_directory = Не вдалося отримати домашній каталог для відкриття і збереження файлу конфігурації.
saving_loading_folder_config_instead_file = Не вдалося створити або відкрити файл конфігурації в шляху"{ $path }", оскільки папка вже існує.
saving_loading_failed_to_create_configuration_folder = Не вдалося налаштувати теку конфігурації "{ $path }", причиною "{ $reason }".
saving_loading_failed_to_create_config_file = Не вдалося створити файл налаштувань "{ $path }", причина "{ $reason }".
saving_loading_failed_to_read_config_file = Не вдалося завантажити конфігурацію з "{ $path }", оскільки не існує або не є файлом.
saving_loading_failed_to_read_data_from_file = Не вдалося прочитати дані з файлу "{ $path }", причина "{ $reason }".
saving_loading_orphan_data = Знайдено застарілі дані "{ $data }" в рядку "{ $line }".
saving_loading_not_valid = Опція "{ $data }" не існує в поточній версії додатку.
# Invalid symlinks
invalid_symlink_infinite_recursion = Нескінченна рекурсія
invalid_symlink_non_existent_destination = Неіснуючий файл призначення
# Other
searching_for_data = Пошук даних може зайняти деякий час, зачекайте...
text_view_messages = ПОВІДОМЛЕННЯ
text_view_warnings = ПОПЕРЕДЖЕННЯ
text_view_errors = ПОМИЛКИ
about_window_motto = Ця програма безкоштовна для використання та буде завжди.
# Various dialog
dialogs_ask_next_time = Запитати наступного разу
delete_file_failed = Не вдалося видалити файл { $name }, причина { $reason }
delete_title_dialog = Підтвердження видалення
delete_question_label = Ви впевнені, що бажаєте видалити файли?
delete_all_files_in_group_title = Підтвердження видалення всіх файлів у групі
delete_all_files_in_group_label1 = У деяких групах є обрані всі записи.
delete_all_files_in_group_label2 = Ви впевнені, що хочете видалити їх?
delete_folder_failed = Не вдалося видалити папку { $dir } , тому що папка не існує, у вас немає дозволів, або вона не порожня.
delete_items_label = { $items } файлів буде видалено.
delete_items_groups_label = { $items } файлів з групи { $groups } буде видалено.
hardlink_failed = Не вдалося встановити hardlink
hard_sym_invalid_selection_title_dialog = Неправильний вибір з деякими групами
hard_sym_invalid_selection_label_1 = У деяких групах є лише 1 запис обраний і його буде проігноровано.
hard_sym_invalid_selection_label_2 = Щоб мати можливість зв'язати ці файли, потрібно вибрати принаймні 2 результати в групі.
hard_sym_invalid_selection_label_3 = Перший у групі визнаний оригінальним, і тому він не змінюється, а потім змінюється.
hard_sym_link_title_dialog = Підтвердження посилання
hard_sym_link_label = Ви впевнені, що хочете зв'язати ці файли?
move_folder_failed = Не вдалося перемістити папку { $name }, причина { $reason }
move_file_failed = Не вдалося перемістити файл { $name }, причина { $reason }
move_files_title_dialog = Виберіть теку, в яку хочете перемістити дубльовані файли
move_files_choose_more_than_1_path = Потрібно вибрати тільки 1 шлях, щоб скопіювати там дубльовані файли, вибрані { $path_number }.
move_stats = Очевидно переміщено { $num_files }/{ $all_files } елементів
save_results_to_file = Збережені результати до файлу { $name }
search_not_choosing_any_music = Помилка: Ви повинні вибрати принаймні один прапорець з типами пошуку музики.
include_folders_dialog_title = Папки для включення
exclude_folders_dialog_title = Теки для виключення
include_manually_directories_dialog_title = Додати теку вручну
cache_properly_cleared = Правильно очищено кеш
cache_clear_duplicates_title = Очищення кешу дублікатів
cache_clear_similar_images_title = Очищення кешу подібних зображень
cache_clear_similar_videos_title = Очищення кеша схожих відео
cache_clear_message_label_1 = Ви хочете очистити кеш із застарілих записів?
cache_clear_message_label_2 = Ця операція видалить всі записи кешу, які вказують на неприпустимі файли.
cache_clear_message_label_3 = Це може пришвидшити завантаження/збереження для кешу.
cache_clear_message_label_4 = УВАГА: Операція видалить всі кешовані дані з відключених зовнішніх дисків, тому хеш буде потрібно згенерувати знову.
# Show preview
preview_temporary_file = Не вдалося відкрити файл тимчасового зображення { $name }, причина { $reason }.
preview_0_size = Неможливо створити попередній перегляд зображення { $name } з нульовою шириною або висотою.
preview_temporary_image_save = Не вдалося зберегти файл тимчасового зображення до { $name }, причина { $reason }.
preview_temporary_image_remove = Не вдалося видалити файл тимчасового зображення { $name }, причина { $reason }.
preview_failed_to_create_cache_dir = Не вдалося створити каталог { $name } потрібного при попередньому перегляді зображення. Причина { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Група { $current_group }/{ $all_groups } ({ $images_in_group } зображення)
compare_move_left_button = L
compare_move_right_button = R
