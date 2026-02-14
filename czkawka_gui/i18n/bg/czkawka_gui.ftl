# Window titles
window_settings_title = Настройки
window_main_title = Czkawka (Хълцук)
window_progress_title = Сканиране
window_compare_images = Сравни изображения
# General
general_ok_button = Ок
general_close_button = Затвори
# Krokiet info dialog
krokiet_info_title = Представяме ви Krokiet - Нова версия на Czkawka
krokiet_info_message = 
        Krokiet е новата, подобрена, по-бърза и по-надеждна версия на Czkawka GTK GUI!

        По-лесно се изпълнява и е по-устойчив на системни промени, тъй като разчита само на основни библиотеки, налични по подразбиране на повечето системи.

        Krokiet също така предлага функции, които Czkawka няма, включително миниатюри в режим на сравнение на видео, EXIF почистване, прогрес при преместване/копиране/изтриване на файлове или разширени опции за сортиране.

        Опитайте го и вижте разликата!

        Czkawka ще продължи да получава поправки на грешки и малки актуализации от мен, но всички нови функции ще бъдат разработени изключително за Krokiet, а всеки е свободен да допринася с нови функции, да добавя липсващи режими или да разширява Czkawka допълнително.

        ПС: Това съобщение трябва да се появи само веднъж. Ако се появи отново, задайте променливата на средата CZKAWKA_DONT_ANNOY_ME на всяка непразна стойност.
# Main window
music_title_checkbox = Заглавие
music_artist_checkbox = Изпълнител
music_year_checkbox = Година
music_bitrate_checkbox = Битрейт
music_genre_checkbox = Жанр
music_length_checkbox = Продължителност
music_comparison_checkbox = Приблизително сравнение
music_checking_by_tags = Етикети
music_checking_by_content = Съдържание
same_music_seconds_label = Минимална продължителност на фрагмента в секунди
same_music_similarity_label = Максимална разлика
music_compare_only_in_title_group = Сравни в групи от подобни заглавия
music_compare_only_in_title_group_tooltip =
    Когато е включено, файловете са групирани по заглавие и след това се сравняват едно с друго.
    
    С 10000 файла, вместо почти 100 милиона сравнения, обикновено ще има около 20000 сравнения.
same_music_tooltip =
    Търсенето на подобни музикални файлове по съдържание може да се конфигурира чрез настройка:
    
    - Минималното време на фрагмента, след което музикалните файлове могат да бъдат идентифицирани като подобни
    - Максимална разлика между два тествани фрагмента
    
    Ключът към добрите резултати е да се намерят разумни комбинации от тези параметри, например.
    
    Ако зададете минималното време на 5 s, а максималната разлика на 1,0, ще търсите почти идентични фрагменти във файловете.
    От друга страна, време от 20 s и максимална разлика от 6,0 работят добре за намиране на ремикси/живи версии и т. н.
    
    По подразбиране всеки музикален файл се сравнява един с друг и това може да отнеме много време при тестване на много файлове, така че обикновено е по-добре да се използват референтни папки и да се укаже кои файлове да се сравняват един с друг(при същото количество файлове сравняването на отпечатъци ще бъде по-бързо поне 4 пъти, отколкото без референтни папки).
music_comparison_checkbox_tooltip =
    Програмата търси подобни музикални файлове с помощта на изкуствен интелект, който използва машинно обучение за премахване на скоби от фраза. Например, при активирана тази опция въпросните файлове ще се считат за дубликати:
    
    Świędziżłób --- Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Чувствително изписване
duplicate_case_sensitive_name_tooltip =
    Когато е разрешено, групата записва само записи с едно и също име, напр. Żołd <-> Żołd
    
    При деактивиране на тази опция имената ще се групират, без да се проверява дали всяка буква е с еднакъв размер, напр. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Размер и име
duplicate_mode_name_combo_box = Име
duplicate_mode_size_combo_box = Размер
duplicate_mode_hash_combo_box = Хеш
duplicate_hash_type_tooltip =
    Czkawka предлага 3 вида хешове:
    
    Blake3 - криптографска хеш функция. Тя е избрана по подразбиране, тъй като е много бърза.
    
    CRC32 - проста хеш функция. Тя би трябвало да е по-бърза от Blake3, но много рядко може да има някои колизии.
    
    XXH3 - много подобна по производителност и качество на хеширане на Blake3 (но некриптографска). Така че тези режими могат лесно да се сменят.
duplicate_check_method_tooltip =
    Засега Czkawka предлага три вида методи за намиране на дубликати чрез:
    
    Име - Намира файлове с еднакво име.
    
    Размер - Намира файлове с еднакъв размер.
    
    Hash - Намира файлове с еднакво съдържание. Този режим хешира файла и по-късно сравнява този хеш, за да намери дубликати. Този режим е най-сигурният начин за намиране на дубликати. Приложението използва силно кеша, така че второто и следващите сканирания на едни и същи данни би трябвало да са много по-бързи от първото.
image_hash_size_tooltip =
    Всяко сравнено изображение дава специален хеш, който може да бъде сравнен с другите и малка разлика между тях означава че изображенията са близки.
    
    Размер 8 хеш е сравнително добър за намиране на изображения, които са близки до оригинала. С по-голям набор изображения (>1000), това ще доведе до голяма бройка фалшиви позитивни, така че препоръчвам да се ползва по-голям размер на хеша в този случай.
    
    16 е размер по подразбиране, който е сравнително добър компромис между намиране на малки разлики в изображенията и имайки малко хеш колизии.
    
    32 и 64 хешове намират само много сходни изображения, но ще имат почти никакви фалшиви позитивни (с изключение на някой изображения с алфа канал).
image_resize_filter_tooltip =
    За да изчисли хеша на изображението, библиотеката трябва първо да го оразмери.
    
    В зависимост от избрания алгоритъм, крайното изображение използвано за изчисляване на хеша може да изглежда леко различно.
    
    Най-бързият алгоритъм, но и даващ най-лоши резултати е Най-Близък. Използва се по-подразбиране, защото хеш с размер 16х16 с ниско качество не е толкова видимо.
    
    С 8х8 хеш, се препоръчва да се използва различен алгоритъм от Най-Близък за да има по-добри групи изображения.
image_hash_alg_tooltip =
    Потребителите могат да изберат един от многото алгоритми за изчисляване на хеша.
    
    Всеки от тях има както силни, така и слаби страни и понякога дава по-добри, а понякога по-лоши резултати за различни изображения.
    
    Затова, за да определите най-добрия за вас, е необходимо ръчно тестване.
big_files_mode_combobox_tooltip = Позволява търсене на най-малките/най-големите файлове
big_files_mode_label = Проверени файлове
big_files_mode_smallest_combo_box = Най-малкия
big_files_mode_biggest_combo_box = Най-големия
main_notebook_duplicates = Повтарящи се файлове
main_notebook_empty_directories = Празни директории
main_notebook_big_files = Големи файлове
main_notebook_empty_files = Празни файлове
main_notebook_temporary = Временни файлове
main_notebook_similar_images = Подобни изображения
main_notebook_similar_videos = Подобни видеа
main_notebook_same_music = Музикални дубликати
main_notebook_symlinks = Невалидни симлинкове
main_notebook_broken_files = Повредени файлове
main_notebook_bad_extensions = Повредени разширения
main_tree_view_column_file_name = Име на файла
main_tree_view_column_folder_name = Име на папката
main_tree_view_column_path = Път
main_tree_view_column_modification = Дата на промяна
main_tree_view_column_size = Размер
main_tree_view_column_similarity = Прилика
main_tree_view_column_dimensions = Размери
main_tree_view_column_title = Заглавие
main_tree_view_column_artist = Изпълнител
main_tree_view_column_year = Година
main_tree_view_column_bitrate = Битрейт
main_tree_view_column_length = Дължина
main_tree_view_column_genre = Жанр
main_tree_view_column_symlink_file_name = Име на файла на Symlink
main_tree_view_column_symlink_folder = Symlink папка
main_tree_view_column_destination_path = Път за местоположение
main_tree_view_column_type_of_error = Тип на грешка
main_tree_view_column_current_extension = Избрано разширение
main_tree_view_column_proper_extensions = Правилно разширение
main_tree_view_column_fps = БПС
main_tree_view_column_codec = Кодек
main_label_check_method = Провери метод
main_label_hash_type = Хеш тип
main_label_hash_size = Хеш размер
main_label_size_bytes = Размер (байтове)
main_label_min_size = Мин
main_label_max_size = Макс
main_label_shown_files = Брой на показани файлове
main_label_resize_algorithm = Преоразмери алгоритъма
main_label_similarity = Сходство{ " " }
main_check_box_broken_files_audio = Аудио
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Архив
main_check_box_broken_files_image = Изображение
main_check_box_broken_files_video = Видео
main_check_box_broken_files_video_tooltip = Използва ffmpeg/ffprobe за валидиране на видео файлове. Доста бавно и може да открие педантични грешки дори ако файлът се възпроизвежда добре.
check_button_general_same_size = Игнорирай еднакъв размер
check_button_general_same_size_tooltip = Игнорирай файлове с идентичен размер в резултата - обикновено това са 1:1 дубликати
main_label_size_bytes_tooltip = Размер на файловете, които ще се използват при сканиране
# Upper window
upper_tree_view_included_folder_column_title = Папки за търсене
upper_tree_view_included_reference_column_title = Папки за справка
upper_recursive_button = Рекурсивен
upper_recursive_button_tooltip = Ако е избрано, се търсят и файлове, които не са поставени директно в избраните папки.
upper_manual_add_included_button = Ръчно добавяне
upper_add_included_button = Добави
upper_remove_included_button = Премахни
upper_manual_add_excluded_button = Ръчно добавяне
upper_add_excluded_button = Добави
upper_remove_excluded_button = Премахни
upper_manual_add_included_button_tooltip =
    Добавяне на име на директория за ръчно търсене.
    
    За да добавите няколко пътища наведнъж, разделете ги с ;
    
    /home/roman;/home/rozkaz ще добави две директории /home/roman и /home/rozkaz
upper_add_included_button_tooltip = Добавяне на нова директория за търсене.
upper_remove_included_button_tooltip = Изтриване на директорията от търсенето.
upper_manual_add_excluded_button_tooltip =
    Добавете името на изключената директория на ръка.
    
    За да добавите няколко пътя наведнъж, разделете ги с ;
    
    /home/roman;/home/krokiet ще добави две директории /home/roman и /home/keokiet
upper_add_excluded_button_tooltip = Добавяне на директория, която да бъде изключена при търсене.
upper_remove_excluded_button_tooltip = Изтриване на директория от изключените.
upper_notebook_items_configuration = Конфигурация на елементите
upper_notebook_excluded_directories = Изключени пътища
upper_notebook_included_directories = Включени пътища
upper_allowed_extensions_tooltip =
    Разрешените разширения трябва да бъдат разделени със запетаи (по подразбиране са налични всички).
    
    Налични са и следните макроси, които добавят няколко разширения наведнъж: ИЗОБРАЖЕНИЕ, ВИДЕО, МУЗИКА, ТЕКСТ.
    
    Пример за използване ".exe, IMAGE, VIDEO, .rar, 7z" - това означава, че ще бъдат сканирани изображения (напр. jpg, png), видеоклипове (напр. avi, mp4), файлове exe, rar и 7z.
upper_excluded_extensions_tooltip =
    Списък с изключени от търсенето файлове.
    
    Когато се ползват едновременно включени и изключени разширения, този тук има по-голям приоритет и файла няма да бъде проверен.
upper_excluded_items_tooltip = 
        Изключените елементи трябва да съдържат * wildcard и да бъдат разделени с ками.
        Това е по-бавно от Excluded Paths, така че използвайте внимателно.
upper_excluded_items = Изключени елементи:
upper_allowed_extensions = Разрешени разширения:
upper_excluded_extensions = Изключени разширения:
# Popovers
popover_select_all = Избери всички
popover_unselect_all = Размаркирайте всички
popover_reverse = Избери обратното
popover_select_all_except_shortest_path = Изберете всички, освен най-краткия път
popover_select_all_except_longest_path = Изберете всички, освен най-дълъг път
popover_select_all_except_oldest = Избери всички освен най-старото
popover_select_all_except_newest = Избери всички освен най-новото
popover_select_one_oldest = Избери най-старото
popover_select_one_newest = Избери най-новото
popover_select_custom = Избери по избор
popover_unselect_custom = Размаркирай по избор
popover_select_all_images_except_biggest = Избери всички освен най-големия
popover_select_all_images_except_smallest = Избери всички освен най-малкия
popover_custom_path_check_button_entry_tooltip =
    Изберете записи по път.
    
    Пример за използване:
    /home/pimpek/rzecz.txt може да бъде намерен с /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Изберете записи по имена на файлове.
    
    Пример за използване:
    /usr/ping/pong.txt може да бъде намерен с *ong*
popover_custom_regex_check_button_entry_tooltip =
    Избиране на записи по зададен Regex.
    
    В този режим търсеният текст е Path with Name.
    
    Пример за използване:
    /usr/bin/ziemniak.txt може да бъде намерен с /ziem[a-z]+
    
    В този случай се използва имплементацията на regex по подразбиране на Rust. Можете да прочетете повече за нея тук: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Активира откриването с отчитане на големи и малки букви.
    
    Когато е изключено, /home/* намира както /HoMe/roman, така и /home/roman.
popover_custom_not_all_check_button_tooltip =
    Предотвратява избирането на всички записи в групата.
    
    Това е разрешено по подразбиране, тъй като в повечето ситуации не искате да изтривате и оригиналните, и дублираните файлове, а искате да оставите поне един файл.
    
    ПРЕДУПРЕЖДЕНИЕ: Тази настройка не работи, ако вече сте избрали ръчно всички резултати в групата.
popover_custom_regex_path_label = Път
popover_custom_regex_name_label = Име
popover_custom_regex_regex_label = Regex Път + Име
popover_custom_case_sensitive_check_button = Чувствителност на буквите
popover_custom_all_in_group_label = Да не се избират всички записи в групата
popover_custom_mode_unselect = Премахване на избора по избор
popover_custom_mode_select = Избери по избор
popover_sort_file_name = Име на файла
popover_sort_folder_name = Име на папката
popover_sort_full_name = Пълно име
popover_sort_size = Размер
popover_sort_selection = Избор
popover_invalid_regex = Regex е невалиден
popover_valid_regex = Regex е валиден
# Bottom buttons
bottom_search_button = Търсене
bottom_select_button = Избери
bottom_delete_button = Изтрий
bottom_save_button = Запази
bottom_symlink_button = Симлинк
bottom_hardlink_button = Хардлинк
bottom_move_button = Премести
bottom_sort_button = Сортирай
bottom_compare_button = Сравни
bottom_search_button_tooltip = Започни търсене
bottom_select_button_tooltip = Изберете записи. Само избраните файлове/папки могат да бъдат обработени по-късно.
bottom_delete_button_tooltip = Изтрий избрани файлове/папки.
bottom_save_button_tooltip = Записване на данни за търсенето във файл
bottom_symlink_button_tooltip =
    Създаване на символни връзки.
    Работи само когато са избрани поне два резултата в група.
    Първият е непроменен, а вторият и по-късните са символни връзки към първия.
bottom_hardlink_button_tooltip =
    Създаване на твърди връзки.
    Работи само когато са избрани поне два резултата в група.
    Първият е непроменен, а вторият и по-късните са свързани с първия.
bottom_hardlink_button_not_available_tooltip =
    Създаване на твърди връзки.
    Бутонът е деактивиран, тъй като не могат да се създават твърди връзки.
    Хардлинковете работят само с администраторски права в Windows, затова не забравяйте да стартирате приложението като администратор.
    Ако приложението вече работи с такива привилегии, проверете за подобни проблеми в Github.
bottom_move_button_tooltip =
    Премества файлове в избрана директория.
    Той копира всички файлове в директорията, без да запазва дървото на директориите.
    При опит за преместване на два файла с еднакво име в папка, вторият ще се провали и ще покаже грешка.
bottom_sort_button_tooltip = Сортира файловете/папките според избрания метод.
bottom_compare_button_tooltip = Сравни изображенията в групата.
bottom_show_errors_tooltip = Показване/скриване на долния текстов панел.
bottom_show_upper_notebook_tooltip = Показване/скриване на горния панел на бележника.
# Progress Window
progress_stop_button = Спри
progress_stop_additional_message = Спри избраните
# About Window
about_repository_button_tooltip = Връзка към страницата на хранилището с изходния код.
about_donation_button_tooltip = Връзка към страницата за дарения.
about_instruction_button_tooltip = Връзка към страницата с инструкции.
about_translation_button_tooltip = Връзка към страницата на Crowdin с преводи на приложения. Официално се поддържат полски и английски език.
about_repository_button = Хранилище
about_donation_button = Дарение
about_instruction_button = Инструкции
about_translation_button = Преводи
# Header
header_setting_button_tooltip = Отваря диалогов прозорец за настройки.
header_about_button_tooltip = Отваря диалогов прозорец с информация за приложението.

# Settings


## General

settings_number_of_threads = Брой използвани нишки
settings_number_of_threads_tooltip = Брой използвани нишки, 0 означава, че ще бъдат използвани всички налични нишки.
settings_use_rust_preview = Използвай външна библиотека вместо GTK да зареди визуализацията
settings_use_rust_preview_tooltip =
    Използвайки GTK визуализации, понякога ще е по-бързо и ще поддържа повече формати, но понякога това може да е точно обратното.
    
    Ако имате проблеми със зареждането на визуализации, можете да пробвате да промените тази настройка.
    
    На не Linux-ови системи е препоръчително да ползвате тази опция, защото gtk-pixbuf не винаги е налично, затова изключването на тази опция може да спре зареждането на визуализациите на някои изображения.
settings_label_restart = Трябва да рестартирате приложението, за да приложите настройките!
settings_ignore_other_filesystems = Игнориране на други файлови системи (само за Linux)
settings_ignore_other_filesystems_tooltip =
    игнорира файлове, които не са в същата файлова система като търсените директории.
    
    Работи по същия начин като опцията -xdev в командата find в Linux
settings_save_at_exit_button_tooltip = Записване на конфигурацията във файл при затваряне на приложението.
settings_load_at_start_button_tooltip =
    Зареждане на конфигурацията от файл при отваряне на приложението.
    
    Ако не е разрешено, ще се използват настройките по подразбиране.
settings_confirm_deletion_button_tooltip = Показване на диалогов прозорец за потвърждение при натискане на бутона за изтриване.
settings_confirm_link_button_tooltip = Показване на диалогов прозорец за потвърждение, когато щракнете върху бутона за твърда/симултанна връзка.
settings_confirm_group_deletion_button_tooltip = Показване на диалогов прозорец с предупреждение при опит за изтриване на всички записи от групата.
settings_show_text_view_button_tooltip = Показване на текстовия панел в долната част на потребителския интерфейс.
settings_use_cache_button_tooltip = Използвайте кеш за файлове.
settings_save_also_as_json_button_tooltip = Записване на кеша в (разбираем за човека) формат JSON. Възможно е да променяте съдържанието му. Кешът от този файл ще бъде прочетен автоматично от приложението, ако липсва кеш в двоичен формат (с разширение bin).
settings_use_trash_button_tooltip = Премества файловете в кошчето, вместо да ги изтрие окончателно.
settings_language_label_tooltip = Език за потребителски интерфейс.
settings_save_at_exit_button = Запазване на конфигурацията при затваряне на приложението
settings_load_at_start_button = Зареждане на конфигурацията при отваряне на приложението
settings_confirm_deletion_button = Показване на диалогов прозорец за потвърждение при изтриване на файлове
settings_confirm_link_button = Показване на диалогов прозорец за потвърждение при твърди/симетрични връзки на файлове
settings_confirm_group_deletion_button = Показване на диалогов прозорец за потвърждение при изтриване на всички файлове в групата
settings_show_text_view_button = Показване на долния текстов панел
settings_use_cache_button = Използвай кеш
settings_save_also_as_json_button = Също запази леша като JSON файл
settings_use_trash_button = Премести изтритите файлове в кошчето
settings_language_label = Език
settings_multiple_delete_outdated_cache_checkbutton = Автоматично изтриване на остарелите записи в кеша
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Изтриване на остарелите резултати от кеша, които сочат към несъществуващи файлове.
    
    Когато е разрешено, приложението се уверява, че при зареждане на записи всички записи сочат към валидни файлове (повредените се игнорират).
    
    Деактивирането на тази функция ще помогне при сканиране на файлове на външни дискове, тъй като записите от кеша за тях няма да бъдат изчистени при следващото сканиране.
    
    В случай че имате стотици хиляди записи в кеша, предлагаме да включите тази опция, което ще ускори зареждането/спасяването на кеша в началото/края на сканирането.
settings_notebook_general = Общи
settings_notebook_duplicates = Дубликати
settings_notebook_images = Сходни изображения
settings_notebook_videos = Сходни видеа

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Показва предварителен преглед от дясната страна (при избиране на файл с изображение).
settings_multiple_image_preview_checkbutton = Показване на предварителен преглед на изображението
settings_multiple_clear_cache_button_tooltip =
    Изчистете ръчно кеша от остарели записи.
    Това трябва да се използва само ако автоматичното изчистване е деактивирано.
settings_multiple_clear_cache_button = Премахни остарели резултати от кеша.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Скрива всички файлове с изключение на един, ако всички сочат към едни и същи данни (са твърдо свързани).
    
    Пример: В случай, че на диска има седем файла, които са свързани с определени данни, и един различен файл със същите данни, но с различен inode, тогава в търсачката за дубликати ще бъдат показани само един уникален файл и един файл от свързаните.
settings_duplicates_minimal_size_entry_tooltip =
    Задаване на минималния размер на файла, който ще се кешира.
    
    Ако изберете по-малка стойност, ще се генерират повече записи. Това ще ускори търсенето, но ще забави зареждането/запазването на кеша.
settings_duplicates_prehash_checkbutton_tooltip =
    Позволява кеширане на prehash (хеш, изчислен от малка част от файла), което позволява по-ранно отхвърляне на недублирани резултати.
    
    По подразбиране е забранено, тъй като в някои ситуации може да доведе до забавяне на работата.
    
    Силно се препоръчва да се използва при сканиране на стотици хиляди или милиони файлове, защото може да ускори търсенето многократно.
settings_duplicates_prehash_minimal_entry_tooltip = Минимален размер на записа в кеша.
settings_duplicates_hide_hard_link_button = Скрий твърди връзки
settings_duplicates_prehash_checkbutton = Използване на предварителен кеш
settings_duplicates_minimal_size_cache_label = Минимален размер на файловете (в байтове), записани в кеша
settings_duplicates_minimal_size_cache_prehash_label = Минимален размер на файловете (в байтове), които се записват в предварителния кеш

## Saving/Loading settings

settings_saving_button_tooltip = Записване на текущата конфигурация на настройките във файл.
settings_loading_button_tooltip = Зареждане на настройките от файл и заместване на текущата конфигурация с тях.
settings_reset_button_tooltip = Възстановяване на текущата конфигурация до тази по подразбиране.
settings_saving_button = Запазване на конфигурацията
settings_loading_button = Конфигурация за зареждане
settings_reset_button = Нулиране на конфигурацията

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Отваря папката, в която се съхраняват кеш txt файловете.
    
    Промяната на кеш файловете може да доведе до показване на невалидни резултати. Промяната на пътя обаче може да спести време при преместване на голямо количество файлове на друго място.
    
    Можете да копирате тези файлове между компютрите, за да спестите време за повторно сканиране на файловете (разбира се, ако те имат сходна структура на директориите).
    
    В случай на проблеми с кеша тези файлове могат да бъдат премахнати. Приложението автоматично ще ги възстанови.
settings_folder_settings_open_tooltip =
    Отваря папката, в която се съхранява конфигурацията на Czkawka.
    
    ПРЕДУПРЕЖДЕНИЕ: Ръчното модифициране на конфигурацията може да наруши работния ви процес.
settings_folder_cache_open = Отворете папката с кеш
settings_folder_settings_open = Отваряне на папката с настройки
# Compute results
compute_stopped_by_user = Търсенето е спряно от потребител
compute_found_duplicates_hash_size = Намерени са { $number_files } дубликати в { $number_groups } групи, които заемат { $size } за { $time }
compute_found_duplicates_name = Намерих { $number_files } дублики в { $number_groups } групи за { $time }
compute_found_empty_folders = Найдени са { $number_files } празни папки във { $time }
compute_found_empty_files = Найдени са { $number_files } празни файлова обекта в { $time }
compute_found_big_files = Намерих { $number_files } големи файла в { $time }
compute_found_temporary_files = Найдени са { $number_files } временни файлъв в { $time }
compute_found_images = Найшли се { $number_files } подобни изображения в { $number_groups } групи за { $time }
compute_found_videos = Намерил { $number_files } подобни видео файла в { $number_groups } групи за { $time }
compute_found_music = Найдено { $number_files } подобни музикални файлове в { $number_groups } групи за { $time }
compute_found_invalid_symlinks = Намерени { $number_files } невалидни символни връзки в { $time }
compute_found_broken_files = Намерих { $number_files } повредени файла в { $time }
compute_found_bad_extensions = Намерени са { $number_files } файла с невалидни разширения за { $time }
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] Сканиран { $file_number } файл
       *[other] Сканирани { $file_number } файлове
    }
progress_scanning_extension_of_files = Проверено разширение на { $file_checked }/{ $all_files } файла
progress_scanning_broken_files = Проверени { $file_checked }/{ $all_files } файла от ({ $data_checked }/{ $all_data })
progress_scanning_video = Хеширани { $file_checked }/{ $all_files } видеа
progress_creating_video_thumbnails = Created thumbnails of { $file_checked }/{ $all_files } video
progress_scanning_image = Хеширани { $file_checked }/{ $all_files } изображения ({ $data_checked }/{ $all_data })
progress_comparing_image_hashes = Сравнени { $file_checked }/{ $all_files } хешове на изображения
progress_scanning_music_tags_end = Сравнени тагове на { $file_checked }/{ $all_files } музикални файла
progress_scanning_music_tags = Прочетени { $file_checked }/{ $all_files } тага на музикални файла
progress_scanning_music_content_end = Сравнени { $file_checked }/{ $all_files } отпечатъка на музикални файла
progress_scanning_music_content = Изчислени { $file_checked }/{ $all_files } отпечатъка на музикални файла ({ $data_checked }/{ $all_data })
progress_scanning_empty_folders =
    { $folder_number ->
        [one] Сканирана { $folder_number } папка
       *[other] Сканирани { $folder_number } папки
    }
progress_scanning_size = Сканиран размер на { $file_number } файла
progress_scanning_size_name = Сканиран име и размер на { $file_number } файла
progress_scanning_name = Сканиран име на { $file_number } файла
progress_analyzed_partial_hash = Анализиран частичен хеш на { $file_checked }/{ $all_files } файла ({ $data_checked }/{ $all_data })
progress_analyzed_full_hash = Анализиран пълен хеш на { $file_checked }/{ $all_files } файла ({ $data_checked }/{ $all_data })
progress_prehash_cache_loading = Зареждане на prehash кеш
progress_prehash_cache_saving = Запис на prehash кеш
progress_hash_cache_loading = Зареждане на hash кеш
progress_hash_cache_saving = Запис на hash кеш
progress_cache_loading = Зарежда кеш
progress_cache_saving = Запазва кеш
progress_current_stage = Текущ етап:{ " " }
progress_all_stages = Всички етапи:{ " " }
# Saving loading 
saving_loading_saving_success = Запазване на конфигурацията във файл { $name }.
saving_loading_saving_failure = Неуспешно спъжаване на конфигурационните данни в файл { $name }, причина { $reason }.
saving_loading_reset_configuration = Текущата конфигурация е изтрита.
saving_loading_loading_success = Правилно заредена конфигурация на приложението.
saving_loading_failed_to_create_config_file = Неуспешно създаване на конфигурационен файл "{ $path }", причина "{ $reason }".
saving_loading_failed_to_read_config_file = Не може да се зареди конфигурация от "{ $path }", защото тя не съществува или не е файл.
saving_loading_failed_to_read_data_from_file = Не може да се прочетат данни от файл "{ $path }", причина "{ $reason }".
# Other
selected_all_reference_folders = Не може да се стартира търсене, когато всички директории са зададени като референтни папки
searching_for_data = Търсене на данни, може да отнеме известно време, моля, изчакайте...
text_view_messages = СЪОБЩЕНИЯ
text_view_warnings = ПРЕДУПРЕЖДЕНИЯ
text_view_errors = ГРЕШКИ
about_window_motto = Тази програма е безплатна за използване и винаги ще бъде такава.
krokiet_new_app = Цквака е в режим на поддръжка, че се приемат само критични грешки и нито еднаnova функционалност ще бъде добавена. За nova функционалност моля проверете новата апликация Крочиец, която е по-стабилна и изтеглянечна и се развива все още активно.
# Various dialog
dialogs_ask_next_time = Попитайте следващия път
symlink_failed = Failed to symlink { $name } to { $target }, reason { $reason }
delete_title_dialog = Изтрий потвърждението
delete_question_label = Сигурни ли сте, че искате да изтриете файловете?
delete_all_files_in_group_title = Потвърждаване на изтриването на всички файлове в групата
delete_all_files_in_group_label1 = В някои групи се избират всички записи.
delete_all_files_in_group_label2 = Сигурни ли сте, че искате да ги изтриете?
delete_items_label = { $items } файловете ще бъдат изтрити.
delete_items_groups_label = { $items } Файловете от { $groups } групите ще бъдат изтрити.
hardlink_failed = Неуспех при създаване на твърд линк за { $name } в { $target }, причина { $reason }
hard_sym_invalid_selection_title_dialog = Невалидна селекция при някои групи
hard_sym_invalid_selection_label_1 = В някои групи е избран само един запис и той ще бъде пренебрегнат.
hard_sym_invalid_selection_label_2 = За да можете да свържете тези файлове с твърда/симетрична връзка, трябва да изберете поне два резултата в групата.
hard_sym_invalid_selection_label_3 = Първият в групата се признава за оригинален и не се променя, но вторият и следващите се променят.
hard_sym_link_title_dialog = Потвърждаване на връзката
hard_sym_link_label = Потвърждаване на връзкатаСигурни ли сте, че искате да свържете тези файлове?
move_folder_failed = Неуспешно преместване на папка { $name }, причина { $reason }
move_file_failed = Неуспешно преместване на файл { $name }, причина { $reason }
move_files_title_dialog = Изберете папката, в която искате да преместите дублираните файлове
move_files_choose_more_than_1_path = Може да се избере само един път, за да може да се копират дублираните им файлове, selected { $path_number }.
move_stats = Правилно преместени { $num_files }/{ $all_files } елементи
save_results_to_file = Запазени резултати едновременно към txt и json файлове в папка "{ $name }".
search_not_choosing_any_music = ГРЕШКА: Трябва да изберете поне едно квадратче за отметка с типове търсене на музика.
search_not_choosing_any_broken_files = ГРЕШКА: Трябва да изберете поне едно квадратче за отметка с тип на проверените счупени файлове.
include_folders_dialog_title = Папки, които да се включват
exclude_folders_dialog_title = Папки, които да се изключат
include_manually_directories_dialog_title = Добаеви ръчно директория
cache_properly_cleared = Правилно изчистен кеш
cache_clear_duplicates_title = Изчистване на кеша за дубликати
cache_clear_similar_images_title = Изчистване на кеша на подобни изображения
cache_clear_similar_videos_title = Изчистване на кеша на подобни видеоклипове
cache_clear_message_label_1 = Искате ли да изчистите кеша от остарели записи?
cache_clear_message_label_2 = Тази операция ще премахне всички записи в кеша, които сочат към невалидни файлове.
cache_clear_message_label_3 = Това може леко да ускори зареждането/записването в кеша.
cache_clear_message_label_4 = ПРЕДУПРЕЖДЕНИЕ: Операцията ще премахне всички кеширани данни от изключените външни дискове. Така че всеки хеш ще трябва да бъде възстановен.
# Show preview
preview_image_resize_failure = Неуспешно променяне на размера на изображението { $name }.
preview_image_opening_failure = Неуспешно отваряне на изображение { $name }, причина { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Група { $current_group }/{ $all_groups } ({ $images_in_group } изображения)
compare_move_left_button = Л
compare_move_right_button = Д
