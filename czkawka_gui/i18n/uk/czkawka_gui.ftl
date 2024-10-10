# Window titles
window_settings_title = Налаштування
window_main_title = Czkawka («Гикавка»)
window_progress_title = Сканування
window_compare_images = Порівняння зображень
# General
general_ok_button = Гаразд
general_close_button = Закрити
# Main window
music_title_checkbox = Найменування
music_artist_checkbox = Виконавець
music_year_checkbox = Рік
music_bitrate_checkbox = Бітрейт
music_genre_checkbox = Жанр
music_length_checkbox = Тривалість
music_comparison_checkbox = Приблизне порівняння
music_checking_by_tags = Мітки
music_checking_by_content = Зміст
same_music_seconds_label = Мінімальна тривалість фрагменту
same_music_similarity_label = Максимальна різниця
music_compare_only_in_title_group = Порівняти лише в заголовку
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    Пошук подібних музичних файлів за його вмістом може бути налаштований за налаштуванням:
    
    - Мінімальний час фрагменту, після якого музичні файли можна визначити як схожий
    - Максимальна різниця між двома тестовими фрагментами
    
    —Що ключові з хороших результатів - знайти розумні комбінації цих параметрів, за умов.
    
    Встановлення мінімального часу на 5 сек і максимальна різниця складає 1.0, буде шукати майже однакові фрагменти у файлах.
    Час 20 і максимальна різниця в 6.0, з іншого боку, добре працює для пошуку реміксиксів/живу версії і т. д.
    
    За замовчуванням, кожен музичний файл порівнюється один з одним, і це може зайняти багато часу при тестуванні багатьох файлів, так що використовувати референтні папки і вказати, які файли слід порівнювати один з одним (з тією ж кількістю файлів, порівняння відбитків пальців буде швидше 4x, ніж без стандартних папок).
music_comparison_checkbox_tooltip =
    Шукає схожі музичні файли за допомогою ШІ, що використовує машинне навчання для видалення дужок із фраз. Наприклад, якщо ця опція увімкнена, наступні файли будуть вважатися дублікатами:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = З урахуванням регістру
duplicate_case_sensitive_name_tooltip =
    Коли увімкнено, записи групуються, тільки якщо вони повністю збігаються імена з точністю до кожного символу. Наприклад, «ХІТ Дискотека» не збігається з "хіт дискотека".
    
    Коли вимкнено, записи групуються незалежно від того, великі або малі літери використовувалися при написанні. Наприклад, «ХІТ Дискотека», «хіт дискотека», «хІт ДиСкОтЕКа» будуть еквівалентні.
duplicate_mode_size_name_combo_box = Розмір і ім'я
duplicate_mode_name_combo_box = Ім'я
duplicate_mode_size_combo_box = Розмір
duplicate_mode_hash_combo_box = Хеш
duplicate_hash_type_tooltip =
    У програмі Czkawka можна використовувати один із трьох алгоритмів хешування:
     
    Blake3 — криптографічна хеш-функція. Використовується за замовчуванням, оскільки дуже швидка.
     
    CRC32 — проста хеш-функція. Ще швидше, ніж Blake3, але можливі рідкісні колізії хешів різних файлів.
     
    XXH3 — функція, схожа за продуктивністю і надійністю хеша на Blake3 (але вона не криптографічна), тому її можна використовувати замість Blake3.
duplicate_check_method_tooltip =
    На цей час Czkawka пропонує три методи пошуку дублікатів:
    
    Ім'я – шукає файли з однаковими іменами.
    
    Розмір – шукає файли однакового розміру.
    
    Хеш – шукає файли з однаковим вмістом. Цей режим хешує файл, а потім порівнює хеш для пошуку дублікатів. Цей режим є найнадійнішим способом пошуку. Додаток активно використовує кеш, тому друге та подальші сканування одних і тих же даних повинні бути набагато швидшими, ніж перше.
image_hash_size_tooltip =
    Кожне перевірене зображення видає спеціальний хеш, який можна порівнювати один з одним, і невелика різниця між ними означає, що ці зображення є схожими.
    
    8 хешів дуже добре знайти зображення, які є трохи схожими на оригінал. При великому наборі зображень (>1000) значення дає велику кількість хибних позитивних результатів, так що я рекомендую використовувати більший розмір хешу у цьому випадку.
    
    16 - це хеш за замовчуванням, який є досить хороший компроміс між пошуком навіть мало схожих зображень і маючи тільки невелику кількість хеш-зіткнень.
    
    32 і 64 пеші знайдуть лише дуже схожі зображення, але не повинні мати практично неправильних позитивних результатів (можливо, окрім деяких зображень з альфа каналом).
image_resize_filter_tooltip =
    Щоб обчислити хеш зображення, бібліотека має спочатку його перемасштабувати.
    
    Залежно від обраного алгоритму отримане зображення, яке використовується при хешуванні, може виглядати трохи інакше.
    
    Найшвидший алгоритм з низькою якістю — це метод найближчого сусіда, Nearest. Він увімкнений за замовчуванням, тому при розмірі хешу 16×16 погана якість не помітна.
    
    Якщо розмір хешу 8×8, рекомендується будь-який алгоритм, крім Nearest, щоб краще відрізняти подібні зображення в групах.
image_hash_alg_tooltip =
    Користувачі можуть вибирати з одного з багатьох алгоритмів обчислення хешу.
    
    Кожен з них має і сильні і слабкі точки, і іноді може призвести до кращого і іноді гірших результатів для різних зображень.
    
    Таким чином, щоб визначити найкращу для вас, потрібно ручне тестування.
big_files_mode_combobox_tooltip = Дозволяє шукати найменші або найбільші файли
big_files_mode_label = Перевірені файли
big_files_mode_smallest_combo_box = Найменший
big_files_mode_biggest_combo_box = Найбільший
main_notebook_duplicates = Файли-дублікати
main_notebook_empty_directories = Порожні каталоги
main_notebook_big_files = Великі файли
main_notebook_empty_files = Порожні файли
main_notebook_temporary = Тимчасові файли
main_notebook_similar_images = Схожі зображення
main_notebook_similar_videos = Схожі відео
main_notebook_same_music = Музичні дублікати
main_notebook_symlinks = Пошкоджені симв. посилання
main_notebook_broken_files = Пошкоджені файли
main_notebook_bad_extensions = Помилкові розширення
main_tree_view_column_file_name = Ім'я файлу
main_tree_view_column_folder_name = Ім'я теки
main_tree_view_column_path = Шлях
main_tree_view_column_modification = Дата зміни
main_tree_view_column_size = Розмір
main_tree_view_column_similarity = Подібність
main_tree_view_column_dimensions = Розміри
main_tree_view_column_title = Найменування
main_tree_view_column_artist = Виконавець
main_tree_view_column_year = Рік
main_tree_view_column_bitrate = Бітрейт
main_tree_view_column_length = Тривалість
main_tree_view_column_genre = Жанр
main_tree_view_column_symlink_file_name = Ім'я файла символьного посилання
main_tree_view_column_symlink_folder = Тека символічного посилання
main_tree_view_column_destination_path = Шлях призначення
main_tree_view_column_type_of_error = Тип помилки
main_tree_view_column_current_extension = Поточне розширення
main_tree_view_column_proper_extensions = Належне розширення
main_label_check_method = Метод перевірки
main_label_hash_type = Тип хешу
main_label_hash_size = Розмір хешу
main_label_size_bytes = Розмір (байт)
main_label_min_size = Мін
main_label_max_size = Макс
main_label_shown_files = Кількість показаних файлів
main_label_resize_algorithm = Алгоритм масштабування
main_label_similarity = Подібність{ "   " }
main_check_box_broken_files_audio = Звук
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Архів
main_check_box_broken_files_image = Зображення
check_button_general_same_size = Ігнорувати однаковий розмір
check_button_general_same_size_tooltip = Ігнорувати файли з однаковим розміром у результаті - зазвичай це 1:1 дублікатів
main_label_size_bytes_tooltip = Розмір файлів, які будуть проскановані
# Upper window
upper_tree_view_included_folder_column_title = Папки для пошуку
upper_tree_view_included_reference_column_title = Містить оригінали
upper_recursive_button = Рекурсивний
upper_recursive_button_tooltip = Коли увімкнено, будуть шукатися файли, що не знаходяться безпосередньо в корені вибраної папки, тобто в інших підпапках даної папки та їх підпапках.
upper_manual_add_included_button = Прописати вручну
upper_add_included_button = Додати
upper_remove_included_button = Видалити
upper_manual_add_excluded_button = Ручне додавання
upper_add_excluded_button = Додати
upper_remove_excluded_button = Видалити
upper_manual_add_included_button_tooltip =
    Додавати назву теки для пошуку вручну.
    
    Додайте декілька шляхів відразу, розділіть їх
    
    /home/roman;/home/rozkaz буде додано дві папки/home/roman and /home/rozkaz
upper_add_included_button_tooltip = Додати нову директорію для пошуку.
upper_remove_included_button_tooltip = Видалити директорію з пошуку.
upper_manual_add_excluded_button_tooltip =
    Додавати виключені назви директорії.
    
    Додайте декілька шляхів одночасно, відокремте їх ;
    
    /home/roman;/home/krokiet додасть дві папки/home/roman та /home/keokiet
upper_add_excluded_button_tooltip = Додати каталог, який виключається з пошуку.
upper_remove_excluded_button_tooltip = Видалити каталог з виключених.
upper_notebook_items_configuration = Параметри пошуку
upper_notebook_excluded_directories = Виключені каталоги
upper_notebook_included_directories = Включені каталоги
upper_allowed_extensions_tooltip =
    Розширення, що включаються, повинні бути розділені комами (за замовчуванням шукаються файли з будь-якими розширеннями).
        
    Макроси IMAGE, VIDEO, MUSIC, TEXT додають одразу кілька розширень.
        
    Приклад використання: «.exe, IMAGE, VIDEO, .rar, 7z» — це означає, що будуть скануватися файли зображень (напр. jpg, png), відео (напр. avi, mp4), exe, rar і 7z.
upper_excluded_extensions_tooltip =
    Список вимкнених файлів, які будуть ігноруватися при скануванні.
    
    При використанні дозволених і вимкнених розширень, цей файл має більший пріоритет, тому файл не буде відмітити.
upper_excluded_items_tooltip =
    Виключені елементи повинні містити маску «*» і бути розділені комами.
    Це повільніше, ніж «Виключені каталоги», тому використовуйте обережно.
upper_excluded_items = Виключені елементи:
upper_allowed_extensions = Дозволені розширення:
upper_excluded_extensions = Вимкнені розширення:
# Popovers
popover_select_all = Виділити все
popover_unselect_all = Прибрати всі
popover_reverse = Зворотній вибір
popover_select_all_except_oldest = Вибрати всі, крім старіших
popover_select_all_except_newest = Вибрати всі, крім найновіших
popover_select_one_oldest = Вибрати один найстаріший
popover_select_one_newest = Вибрати один найновіший
popover_select_custom = Вибрати власний
popover_unselect_custom = Скасувати вибір
popover_select_all_images_except_biggest = Вибрати всі, крім найбільшого
popover_select_all_images_except_smallest = Вибрати всі, крім найменших
popover_custom_path_check_button_entry_tooltip =
    Вибір записів з урахуванням шляху.
    
    Приклад:
    /home/pimpek/rzecz.txt можна знайти за допомогою /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Вибір записів на ім'я файлів.
    
    Приклад:
    /usr/ping/pong.txt можна знайти за допомогою *ong*
popover_custom_regex_check_button_entry_tooltip =
    Вибір записів за допомогою регулярних виразів.
    
    У цьому режимі шуканий текст є шлях з ім'ям.
     
    Приклад:
    /usr/bin/ziemniak.txt можна знайти за допомогою виразу /ziem[a-z]+
     
    За замовчуванням використається синтаксис регулярних виразів Rust. Докладніше про це можна прочитати тут: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Вмикає пошук з урахуванням регістру.
    
    При відключеній опції «/home/*» буде відповідати як «/home/roman», так і «/HoMe/roman».
popover_custom_not_all_check_button_tooltip =
    Заборона вибору всіх записів у групі.
    
    Ця опція включена за замовчуванням, тому що в більшості ситуацій вам не треба видаляти і оригінали, і дублікати — зазвичай залишають хоча б один файл.
    
    УВАГА. Цей параметр не працює, якщо ви вже вручну вибрали всі результати групи.
popover_custom_regex_path_label = Шлях
popover_custom_regex_name_label = Ім'я
popover_custom_regex_regex_label = Шлях із рег. виразом + ім'я
popover_custom_case_sensitive_check_button = З урахуванням регістру
popover_custom_all_in_group_label = Не вибирати усі записи в групі
popover_custom_mode_unselect = Зняти вибір
popover_custom_mode_select = Вибрати власний
popover_sort_file_name = Ім'я файлу
popover_sort_folder_name = Назва папки
popover_sort_full_name = Повне ім'я
popover_sort_size = Розмір
popover_sort_selection = Вибрані об'єкти
popover_invalid_regex = Неприпустимий регулярний вираз
popover_valid_regex = Коректний регулярний вираз
# Bottom buttons
bottom_search_button = Пошук
bottom_select_button = Вибрати
bottom_delete_button = Видалити
bottom_save_button = Зберегти
bottom_symlink_button = Симв. посилання
bottom_hardlink_button = Жорст. посилання
bottom_move_button = Перемістити
bottom_sort_button = Сортування
bottom_search_button_tooltip = Почати пошук
bottom_select_button_tooltip = Виберіть запис. Тільки вибрані файли/папки будуть доступні для подальшої обробки.
bottom_delete_button_tooltip = Видалити вибрані файли/папки.
bottom_save_button_tooltip = Зберегти дані про пошук у файл
bottom_symlink_button_tooltip =
    Створити символьні посилання.
    Працює лише тоді, коли вибрано не менше двох результатів у групі.
    Перший результат залишається, а другий та наступні робляться символьними посиланнями на перший.
bottom_hardlink_button_tooltip =
    Створити жорсткі посилання.
    Працює лише тоді, коли вибрано не менше двох результатів у групі.
    Перший результат залишається, а другий та наступні робляться жорсткими посиланнями на перший.
bottom_hardlink_button_not_available_tooltip =
    Створити Жорсткі посилання.
    кнопка вимкнена, тому що не може бути створена.
    Жорсткі посилання працюють тільки з правами адміністратора на Windows, тому не забудьте запустити додаток як адміністратор.
    Якщо додаток вже працює з такими привілеями для подібних проблем на GitHub.
bottom_move_button_tooltip =
    Переміщення файлів до вибраного каталогу.
    Копіює всі файли в теку без збереження структури дерева каталогів.
    При спробі перемістити два файли з однаковим ім'ям в ту саму теку другий не буде переміщений, і з'явиться повідомлення про помилку.
bottom_sort_button_tooltip = Сортує файли/теки відповідно до вибраного методу.
bottom_show_errors_tooltip = Показати/приховати нижню текстову панель.
bottom_show_upper_notebook_tooltip = Показати/приховати верхню панель блокнота.
# Progress Window
progress_stop_button = Зупинити
progress_stop_additional_message = Припинити запит
# About Window
about_repository_button_tooltip = Посилання на сторінку репозиторію з вихідним кодом.
about_donation_button_tooltip = Посилання на сторінку пожертви.
about_instruction_button_tooltip = Посилання на сторінку інструкцій.
about_translation_button_tooltip = Посилання на сторінку Crowdin із перекладами додатків. Офіційно підтримуються англійська та польська мови.
about_repository_button = Репозиторій
about_donation_button = Пожертва
about_instruction_button = Інструкція
about_translation_button = Переклад
# Header
header_setting_button_tooltip = Відкриває вікно налаштувань.
header_about_button_tooltip = Відкриває діалогове вікно з інформацією про додаток.

# Settings


## General

settings_number_of_threads = Кількість використаних тем
settings_number_of_threads_tooltip = Кількість використаних потоків; встановіть 0, щоб використовувати всі доступні потоки.
settings_use_rust_preview = Використовувати зовнішні бібліотеки gtk для завантаження прев'ю
settings_use_rust_preview_tooltip =
    Використання gtk прев'ю іноді може бути швидшим і підтримувати більше форматів, але іноді це може бути зовсім навпаки.
    
    Якщо у вас виникли проблеми з завантаженням превью, можна змінити цей параметр.
    
    У нелінійних системах рекомендується використовувати цей параметр, тому, що gtk-pixbuf не завжди доступні, тому вимкнення цього параметру не буде завантажувати попередній перегляд деяких зображень.
settings_label_restart = Щоб застосувати параметри, необхідно перезапустити додаток!
settings_ignore_other_filesystems = Ігнорувати інші файлові системи (лише Linux)
settings_ignore_other_filesystems_tooltip =
    ігнорує файли, які не знаходяться в одній файловій системі, як пошукові каталоги.
    
    Працює те саме, що і параметр -xdev у пошуку команди на Linux
settings_save_at_exit_button_tooltip = Зберегти конфігурацію в файл під час закриття додатку.
settings_load_at_start_button_tooltip =
    Завантажити конфігурацію з файлу під час відкриття програми.
    
    Якщо не ввімкнено, будуть використовуватися налаштування за замовчуванням.
settings_confirm_deletion_button_tooltip = Показувати вікно підтвердження при натисканні на кнопку видалення.
settings_confirm_link_button_tooltip = Показувати діалогове вікно підтвердження при натисканні на кнопку твердого/символічного посилання.
settings_confirm_group_deletion_button_tooltip = Показувати діалогове вікно при спробі видалення всіх записів з групи.
settings_show_text_view_button_tooltip = Показувати панель тексту в нижній частині інтерфейсу користувача.
settings_use_cache_button_tooltip = Використовувати кеш файлів.
settings_save_also_as_json_button_tooltip = Зберігати кеш у формат JSON (читабельний). Його вміст можна змінювати. Кеш із цього файлу буде автоматично прочитаний програмою, якщо бінарний кеш (з розширенням bin) відсутній.
settings_use_trash_button_tooltip = Перемістити файли в смітник, а не видаляти їх назавжди.
settings_language_label_tooltip = Мова інтерфейсу користувача.
settings_save_at_exit_button = Зберегти конфігурацію під час закриття додатку
settings_load_at_start_button = Завантажити конфігурацію при відкритті програми
settings_confirm_deletion_button = Показувати вікно підтвердження при видаленні будь-якого файлу
settings_confirm_link_button = Показувати вікно підтвердження при складній/символьних посилань будь-які файли
settings_confirm_group_deletion_button = Показувати вікно підтвердження при видаленні всіх файлів групи
settings_show_text_view_button = Показувати нижню текстову панель
settings_use_cache_button = Використовувати кеш
settings_save_also_as_json_button = Також зберегти кеш у файл JSON
settings_use_trash_button = Перемістити видалені файли в смітник
settings_language_label = Мова
settings_multiple_delete_outdated_cache_checkbutton = Автоматично видаляти застарілі записи кешу
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Видалити застарілі результати кеша, що вказують на неіснуючі файли.
    
    Коли опція увімкнена, програма перевіряє під час завантаження записів, чи вказують вони на доступні файли (недостачі файли ігноруються).
    
    Вимкнення цієї опції допомагає при скануванні файлів на зовнішніх носіях, щоб інформація про них не була очищена під час наступного сканування.
    
    За наявності сотень тисяч записів у кеші рекомендується увімкнути цю опцію, щоб прискорити завантаження та збереження кешу на початку та в кінці сканування.
settings_notebook_general = Загальні налаштування
settings_notebook_duplicates = Дублікати
settings_notebook_images = Схожі зображення
settings_notebook_videos = Схожий відео

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Показувати попередній перегляд праворуч (якщо вибрано файл зображення).
settings_multiple_image_preview_checkbutton = Показати попередній перегляд зображення
settings_multiple_clear_cache_button_tooltip =
    Очищення застарілих записів кешу вручну.
    Слід використовувати лише в тому випадку, якщо автоматичне очищення вимкнено.
settings_multiple_clear_cache_button = Видалити застарілі результати з кешу.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Приховати всі файли, крім першого, якщо всі вони вказують на ті самі дані (пов'язані жорстким посиланням).
    
    Приклад: якщо (на диску) сім файлів пов'язані жорстким посиланням з певними даними, а ще один файл містить ті ж дані, але на іншому inode, то в засобі пошуку дублікатів будуть показані тільки цей останній унікальний файл і один файл, що є жорстким посиланням.
settings_duplicates_minimal_size_entry_tooltip =
    Встановити мінімальний розмір файлу, що кешується.
    
    Вибір меншого значення призведе до створення більшої кількості записів. Це прискорить пошук, але сповільнить завантаження/збереження кешу.
settings_duplicates_prehash_checkbutton_tooltip =
    Включає кешування попереднього хеша (prehash), що обчислюється з невеликої частини файлу, що дозволяє швидше виключати з аналізу файли, що відрізняються.
    
    За замовчуванням вимкнено, оскільки в деяких ситуаціях може сповільнюватися.
    
    Рекомендується використовувати його при скануванні сотень тисяч або мільйонів файлів, оскільки це може прискорити пошук в рази.
settings_duplicates_prehash_minimal_entry_tooltip = Мінімальний розмір кешованого запису.
settings_duplicates_hide_hard_link_button = Приховати жорсткі посилання (лише Linux та macOS)
settings_duplicates_prehash_checkbutton = Кешувати попередній хеш
settings_duplicates_minimal_size_cache_label = Мінімальний розмір (байт) файлів, що кешуються
settings_duplicates_minimal_size_cache_prehash_label = Мінімальний розмір (байт) файлів для кешування попереднього хешу

## Saving/Loading settings

settings_saving_button_tooltip = Зберегти поточні налаштування у файл.
settings_loading_button_tooltip = Завантажити параметри з файлу і замінити поточні налаштування.
settings_reset_button_tooltip = Скинути поточні налаштування до стандартних значень.
settings_saving_button = Зберегти конфігурацію
settings_loading_button = Завантажити конфігурацію
settings_reset_button = Скидання налаштувань

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Відкрити папку, де зберігаються текстові файли кеша.
    
    Зміна файлів кешу може призвести до відображення неправильних результатів, однак зміна шляху може заощадити час, коли переміщується велика кількість файлів в інше місце.
    
    Ви можете копіювати ці файли між комп'ютерами, щоб заощадити час на повторному скануванні файлів (звичайно якщо вони мають схожу структуру каталогів).
    
    У разі виникнення проблем із кешем ці файли можна видалити. Програма автоматично перестворить їх.
settings_folder_settings_open_tooltip =
    Відкриває теку, де зберігається конфігурація Czkawka.
    
    УВАГА. Ручна зміна конфігурації може порушити функціонування програми.
settings_folder_cache_open = Відкрити теку кешу
settings_folder_settings_open = Відкрити папку налаштувань
# Compute results
compute_stopped_by_user = Пошук був зупинений користувачем
compute_found_duplicates_hash_size = Знайдено дублікатів: { $number_files } (груп: { $number_groups }), розмір: { $size }
compute_found_duplicates_name = Знайдено: { $number_files } дубликат(и/ів) (груп: { $number_groups })
compute_found_empty_folders = Знайдено порожніх папок: { $number_files }
compute_found_empty_files = Знайдено порожніх файлів: { $number_files }
compute_found_big_files = Знайдено великих файлів: { $number_files }
compute_found_temporary_files = Знайдено тимчасових файлів: { $number_files }
compute_found_images = Знайдено аналогічних зображень: { $number_files } (груп: { $number_groups })
compute_found_videos = Знайдено схожих відео: { $number_files } (груп: { $number_groups })
compute_found_music = Знайдено аналогічних музичних файлів: { $number_files } (груп: { $number_groups })
compute_found_invalid_symlinks = Знайдено { $number_files } неприпустимі символічні посилання
compute_found_broken_files = Знайдено зламаних файлів: { $number_files }
compute_found_bad_extensions = Знайдено { $number_files } файлів з недійсними розширеннями
# Progress window
progress_scanning_general_file = Сканування файлу: { $file_number }
progress_scanning_extension_of_files = Перевірка розширення { $file_checked }/{ $all_files } файлу
progress_scanning_broken_files = Перевірка файлу: { $file_checked }/{ $all_files }
progress_scanning_video = Хешування відео: { $file_checked }/{ $all_files }
progress_scanning_image = Хешування зображення: { $file_checked }/{ $all_files }
progress_comparing_image_hashes = Порівняння хешу зображення: { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Порівняння тегів музичного файлу: { $file_checked }/{ $all_files }
progress_scanning_music_tags = Читання тегів музичного файлу: { $file_checked }/{ $all_files }
progress_scanning_music_content_end = Порівняння відбитку пальця { $file_checked }/{ $all_files } музичного файлу
progress_scanning_music_content = Розрахунок відбитку пальця { $file_checked }/{ $all_files } музичного файлу
progress_scanning_empty_folders = Сканування теки { $folder_number }
progress_scanning_size = Сканування розміру файлу { $file_number }
progress_scanning_size_name = Сканування назви і розміру файлу { $file_number }
progress_scanning_name = Сканування імені файлу { $file_number }
progress_analyzed_partial_hash = Аналіз часткового хешу файлу { $file_checked }/{ $all_files }
progress_analyzed_full_hash = Аналіз повного хешу файлу { $file_checked }/{ $all_files }
progress_prehash_cache_loading = Завантаження цільового кешу
progress_prehash_cache_saving = Збереження цілковитого кешу
progress_hash_cache_loading = Завантаження схованки
progress_hash_cache_saving = Збереження кешу кешу
progress_cache_loading = Завантаження кешу
progress_cache_saving = Збереження кешу
progress_current_stage = Поточний етап: { "  " }
progress_all_stages = Усі етапи: { "  " }
# Saving loading 
saving_loading_saving_success = Збережено конфігурацію в файл { $name }.
saving_loading_saving_failure = Не вдалося зберегти дані конфігурації до файлу { $name }.
saving_loading_reset_configuration = Поточна конфігурація була очищена.
saving_loading_loading_success = Установки програми коректно завантажені.
saving_loading_invalid_string = Для ключа «{ $key }» знайдено неприпустимий результат: «{ $result }» не є рядком.
saving_loading_invalid_int = Для ключа «{ $key }» знайдено неприпустимий результат: «{ $result }» не є цілим числом.
saving_loading_invalid_bool = Для ключа «{ $key }» знайдено неприпустимий результат: «{ $result }» не є булевим.
saving_loading_decode_problem_bool = Не вдалося декодувати булеве значення з ключа «{$key}»: знайдено «{$result}», але дозволені значення 0, 1, true або false.
saving_loading_saving_same_keys = Спроба зберегти налаштування за допомогою дубльованого ключа "{ $key }".
saving_loading_failed_to_get_home_directory = Не вдалося отримати домашній каталог для відкриття і збереження файлу конфігурації.
saving_loading_folder_config_instead_file = Не вдалося створити або відкрити файл конфігурації в шляху"{ $path }", оскільки папка вже існує.
saving_loading_failed_to_create_configuration_folder = Не вдалося налаштувати теку конфігурації "{ $path }", причиною "{ $reason }".
saving_loading_failed_to_create_config_file = Не вдалося створити файл налаштувань "{ $path }", причина "{ $reason }".
saving_loading_failed_to_read_config_file = Неможливо завантажити конфігурацію з «{$path}», тому що або такого файлу не існує, або це не файл.
saving_loading_failed_to_read_data_from_file = Не вдалося прочитати дані з файлу "{ $path }", причина "{ $reason }".
saving_loading_orphan_data = Знайдені дані «{ $data }», що нічого не належать, у рядку «{ $line }».
saving_loading_not_valid = Параметр «{ $data }» не існує в поточній версії додатку.
# Invalid symlinks
invalid_symlink_infinite_recursion = Нескінченна рекурсія
invalid_symlink_non_existent_destination = Неіснуючий файл призначення
# Other
selected_all_reference_folders = Неможливо почати пошук, коли всі каталоги встановлені як папки з орієнтирами
searching_for_data = Пошук даних може зайняти деякий час — будь ласка, зачекайте...
text_view_messages = ПОВІДОМЛЕННЯ
text_view_warnings = ПОПЕРЕДЖЕННЯ
text_view_errors = ПОМИЛКИ
about_window_motto = Ця програма безкоштовна для використання і завжди буде такою.
# Various dialog
dialogs_ask_next_time = Завжди запитувати
delete_file_failed = Не вдалося видалити файл { $name }. Причина: { $reason }
delete_title_dialog = Підтвердження видалення
delete_question_label = Ви впевнені, що бажаєте видалити файли?
delete_all_files_in_group_title = Підтвердження видалення всіх файлів у групі
delete_all_files_in_group_label1 = У деяких групах обрані всі записи.
delete_all_files_in_group_label2 = Ви впевнені, що хочете видалити їх?
delete_folder_failed = Не вдалося видалити папку { $dir }, оскільки папки не існує, або у вас немає дозволу на зміну, або папка не порожня.
delete_items_label = Буде видалено файлів: { $items }
delete_items_groups_label = Буде видалено файлів: { $items } (груп: { $groups })
hardlink_failed = Не вдалося створити жорстке посилання
hard_sym_invalid_selection_title_dialog = Невірний вибір у деяких групах
hard_sym_invalid_selection_label_1 = У деяких групах вибрано лише один запис — вони будуть проігноровані.
hard_sym_invalid_selection_label_2 = Щоб жорстко або символьно зв'язати ці файли, необхідно вибрати щонайменше два результати групи.
hard_sym_invalid_selection_label_3 = Перший у групі визнаний як оригінал і не буде змінено, але другий та наступні модифіковані.
hard_sym_link_title_dialog = Підтвердження посилання
hard_sym_link_label = Ви впевнені, що хочете зв'язати ці файли?
move_folder_failed = Не вдалося перемістити папку { $name }. Причина: { $reason }
move_file_failed = Не вдалося перемістити файл { $name }, причина { $reason }
move_files_title_dialog = Виберіть теку, в яку хочете перемістити дубльовані файли
move_files_choose_more_than_1_path = Можна вибрати лише один шлях для копіювання дублікатів файлів, але вибрано { $path_number }.
move_stats = Вдалося перемістити без помилок елементів: { $num_files }/{ $all_files }
save_results_to_file = Збережено результати як з txt, так і з json файлів в теку { $name }.
search_not_choosing_any_music = Помилка: Ви повинні вибрати принаймні один прапорець з типами пошуку музики.
search_not_choosing_any_broken_files = ПОМИЛКА: Ви повинні вибрати принаймні один прапорець з типом пошкоджених файлів.
include_folders_dialog_title = Теки для включення
exclude_folders_dialog_title = Теки для виключення
include_manually_directories_dialog_title = Додати теку вручну
cache_properly_cleared = Кеш успішно очищений
cache_clear_duplicates_title = Очищення кешу дублікатів
cache_clear_similar_images_title = Очищення кешу подібних зображень
cache_clear_similar_videos_title = Очищення кеша схожих відео
cache_clear_message_label_1 = Ви хочете очистити кеш від застарілих записів?
cache_clear_message_label_2 = Ця дія видаляє всі записи кешу, що вказують на недоступні файли.
cache_clear_message_label_3 = Це може трохи прискорити завантаження/збереження кешу.
cache_clear_message_label_4 = УВАГА. Ця дія видаляє всі кешовані дані від вимкнених зовнішніх дисків. Хеші для файлів на цих носіях потрібно буде згенерувати заново.
# Show preview
preview_image_resize_failure = Не вдалося змінити розмір зображення { $name }.
preview_image_opening_failure = Не вдалося відкрити зображення {$name}. Причина: {$reason}
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Група { $current_group }/{ $all_groups } (зображень: { $images_in_group })
compare_move_left_button = L
compare_move_right_button = R
