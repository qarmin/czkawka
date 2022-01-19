# Window titles
window_settings_title = Opcje
window_main_title = Czkawka
window_progress_title = Skanowanie
window_compare_images = Porównywanie Obrazów
# General
general_ok_button = Ok
general_close_button = Zamknij
# Main window
music_title_checkbox = Tytuł
music_artist_checkbox = Wykonawca
music_album_title_checkbox = Tytuł Albumu
music_album_artist_checkbox = Wykonawca Albumu
music_year_checkbox = Rok
music_comparison_checkbox = Przybliżone Porównywanie
music_comparison_checkbox_tooltip =
    Wyszukuje podobne pliki muzyczne za pomocą AI, która to za pomocą uczenia maszynowego usuwa nawiasy z wyrażenia np. z włączoną opcją, dane pliki zostaną uznane za duplikaty:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Nazwa
duplicate_mode_size_combo_box = Rozmiar
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka oferuje 3 różne algorytmy do tworzenia hashu pliku:
    
    Blake3 - kryptograficzna funkcja haszująca. Z racji połączenia szybkości i niskiej ilości kolizji jest to domyślny tryb.
    
    CRC32 - prosta funkcja haszująca. Powinna być szybsza niż Blake3, lecz bardzo rzadko może mogą wystąpić kolizje hashów.
    
    XXH3 - zarówno pod względem jakości hashu jak i wydajności jest podobny do Blake3, dlatego te algorytmy mogą być używane wymiennie.
duplicate_check_method_tooltip =
    Na chwilę obecną, Czkawka oferuje 3 tryby wyszukiwania duplikatów poprzez:
    
    Nazwę - Znajduje identycznie nazywające się pliki.
    
    Rozmiar - Znajduje pliki o identycznych rozmiarach.
    
    Hash - Wyszukuje pliki o tej samej zawartości. W tym trybie, każdy plik jest hasowany a następnie każdy hash jest porównywany z innymi. Ten tryb używa pamięci podręcznej do przechowywania raz obliczonych hashy, dlatego drugie i kolejne skanowanie, powinno być o wiele szybsze niż pierwsze. Jest to najbezpieczniejszy sposób na znalezienie duplikatów.
image_hash_size_tooltip =
    Czkawka umożliwia zmianę wielkości generowanego hashu dla każdego obrazu. Im większy, tym mniejsze różnice może znaleźć pomiędzy obrazami, lecz również jest nieco wolniejszy w użytkowaniu.
    
    Domyślną wielkością hashu jest 8, które pozwala wyszukiwać zarówno bardzo jak i mało podobne do siebie obrazy. Hashe 16 i 32 powinny być głównie używane dla niemal identycznych plików. Hash 64 bajtowy nie powinien być stosowany, chyba że wymagane jest znalezienie bardzo małych różnic pomiędzy obrazami.
image_resize_filter_tooltip = By obliczyć hash obrazu, biblioteka musi najpierw go zmniejszyć. W zależności od wybranego algorytmu, obraz będzie wyglądał nieco inaczej. Najszybszym, lecz za razem dającym najgorsze efekty jest algorytm Nearest.
image_hash_alg_tooltip = Do wyboru jest kilka algorytmów obliczenia hashu obrazu. Każdy z nich ma swoje słabe i silne punkty i będzie dawał czasem lepsze a czasem gorsze rezultaty w zależności od obrazów, dlatego najlepiej będzie przetestować je na własną rękę.
main_notebook_image_fast_compare = Szybkie porównywanie
main_notebook_image_fast_compare_tooltip =
    Przyśpieszenie wyszukiwania i porównywania hashów.
    
    W przeciwieństwie do zwykłego trybu, w którym każdy skrót jest porównywany ze sobą x razy, gdzie x jest podobieństwem, które wybiera użytkownik, w tym trybie zawsze stosuje się tylko jedno porównanie.
    
    Ta opcja jest zalecana podczas porównywania >10000 obrazów z podobieństwem innym niż 0 (Bardzo Duże).
main_notebook_duplicates = Duplikaty
main_notebook_empty_directories = Puste Katalogi
main_notebook_big_files = Duże Pliki
main_notebook_empty_files = Puste Pliki
main_notebook_temporary = Pliki Tymczasowe
main_notebook_similar_images = Podobne Obrazy
main_notebook_similar_videos = Podobne Wideo
main_notebook_same_music = Podobna Muzyka
main_notebook_symlinks = Niepoprawne Symlinki
main_notebook_broken_files = Zepsute Pliki
main_tree_view_column_file_name = Nazwa
main_tree_view_column_folder_name = Nazwa
main_tree_view_column_path = Ścieżka
main_tree_view_column_modification = Data Modyfikacji
main_tree_view_column_size = Rozmiar
main_tree_view_column_similarity = Podobieństwo
main_tree_view_column_dimensions = Wymiary
main_tree_view_column_title = Tytuł
main_tree_view_column_artist = Wykonawca
main_tree_view_column_year = Rok
main_tree_view_column_album_title = Tytuł Albumu
main_tree_view_column_album_artist = Wykonawca Albumu
main_tree_view_column_symlink_file_name = Nazwa Symlinka
main_tree_view_column_symlink_folder = Folder Symlinka
main_tree_view_column_destination_path = Docelowa Ścieżka
main_tree_view_column_type_of_error = Typ Błędu
main_label_check_method = Metoda sprawdzania
main_label_hash_type = Typ hashu
main_label_hash_size = Rozmiar hashu
main_label_size_bytes = Rozmiar(bajty)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Liczba wyświetlanych plików
main_label_resize_algorithm = Algorytm zmiany rozmiaru
main_label_similarity = Podobieństwo{ "   " }
check_button_general_same_size = Ignoruj identyczny rozmiar
check_button_general_same_size_tooltip = Wyrzuca z wyników skanowania pliki, które posiadają identyczny rozmiar, po to by w wynikach zostały tylko niemal identyczne rekordy.
main_label_size_bytes_tooltip = Rozmiar plików które będą zawarte przy przeszukiwaniu
# Upper window
upper_tree_view_included_folder_column_title = Foldery do Przeszukania
upper_tree_view_included_reference_column_title = Foldery Referencyjne
upper_recursive_button = Rekursywnie
upper_recursive_button_tooltip = Jeśli zaznaczony, szuka plików i folderów również w katalogach wewnątrz, nawet jeśli nie znajdują się one bezpośrednio w tym folderze.
upper_manual_add_included_button = Ręcznie Dodaj
upper_add_included_button = Dodaj
upper_remove_included_button = Usuń
upper_manual_add_excluded_button = Ręcznie Dodaj
upper_add_excluded_button = Dodaj
upper_remove_excluded_button = Usuń
upper_manual_add_included_button_tooltip = Pozwala ręcznie dodać foldery do skanowania.
upper_add_included_button_tooltip = Dodaje wybrany folder do przeskanowania.
upper_remove_included_button_tooltip = Usuwa zaznaczony folder z listy do skanowania.
upper_manual_add_excluded_button_tooltip = Pozwala ręcznie dodać ignorowane foldery.
upper_add_excluded_button_tooltip = Dodaje wybrany folder do ignorowanych.
upper_remove_excluded_button_tooltip = Usuwa zaznaczony folder z ignorowanych.
upper_notebook_items_configuration = Konfiguracja Skanowania
upper_notebook_excluded_directories = Ignorowane Foldery
upper_notebook_included_directories = Przeszukiwane Foldery
upper_allowed_extensions_tooltip =
    Dozwolone rozszerzenia muszą być oddzielone za pomocą przecinków - brak rozszerzeń oznacza że wszystkie rozszerzenia są używane.
    
    Makra IMAGE, VIDEO, MUSIC, TEXT które dodają rozszerzenia w paczkach, również są wspierane.
    
    Przykładowe użycie ".exe, IMAGE, VIDEO, .rar, 7z" oznacza że obrazy(np. jpg, png), widea(np. avi, mp4) oraz pliki z rozszerzeniami exe, rar i 7z będą przeskanowane.
upper_excluded_items_tooltip =
    Ignorowane obiekty mogą zawierać *(oznaczający dowolny ciąg znaków) i muszą być oddzielone za pomocą przecinków.
    Działa o wiele wolniej niż Ignorowane Foldery, dlatego należy używać tego ostrożnie.
upper_excluded_items = Ignorowane Obiekty:
upper_allowed_extensions = Dozwolone Rozszerzenia:
# Popovers
popover_select_all = Zaznacz wszystko
popover_unselect_all = Odznacz wszystko
popover_reverse = Odwróć zaznaczenie
popover_select_all_except_oldest = Zaznacz wszystkie oprócz najstarszego
popover_select_all_except_newest = Zaznacz wszystkie oprócz najnowszego
popover_select_one_oldest = Zaznacz jedno najstarsze
popover_select_one_newest = Zaznacz jedno najnowsze
popover_select_custom = Własne zaznaczanie
popover_unselect_custom = Własne odznaczanie
popover_select_all_images_except_biggest = Zaznacz wszystkie oprócz największego
popover_select_all_images_except_smallest = Zaznacz wszystkie oprócz najmniejszego
popover_custom_path_check_button_entry_tooltip =
    Pozwala zaznaczać rekordy według ścieżki.
    
    Przykładowe użycie:
    /home/pimpek/rzecz.txt może zostać znalezione poprzez /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Pozwala zaznaczać rekordy według ścieżki.
    
    Przykładowe użycie:
    /usr/ping/pong.txt może zostać znalezione poprzez *ong*
popover_custom_regex_check_button_entry_tooltip =
    Pozwala wyszukiwać rekordy za pomocą regexów.
    
    W tym trybie przeszukiwanym tekstem jest pełna ścieżka(nazwa pliku + ścieżka do niego).
    
    Przykładowe użycie:
    /usr/bin/ziemniak.txt może zostać znalezione with /ziem[a-z]+
    
    Używana jest domyśla implementacja Rust regex o której użyciu można więcej przeczytać tutaj - https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Zapobiega przed zaznaczeniem wszystkich rekordów w danej grupie.
    
    Tryb jest domyślnie aktywny, ponieważ w większości przypadków zaznaczenie wszystkich rekordów w grupie nie jest czymś czego oczekuje użytkownik.
    
    Uwaga: To ustawienie nie powoduje pomija grupy w których to użytkownik wcześniej zaznaczył wszystkie rekordy ręcznie.
popover_custom_regex_path_label = Ścieżka
popover_custom_regex_name_label = Nazwa
popover_custom_regex_regex_label = Regex - Pełna ścieżka
popover_custom_all_in_group_label = Nie zaznaczaj wszystkich rekordów w grupie
popover_custom_mode_unselect = Własne odznaczanie
popover_custom_mode_select = Własne zaznaczanie
popover_invalid_regex = Regex jest niepoprawny
popover_valid_regex = Regex jest poprawny
# Bottom buttons
bottom_search_button = Szukaj
bottom_select_button = Zaznacz
bottom_delete_button = Usuń
bottom_save_button = Zapisz
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Przenieś
bottom_search_button_tooltip = Rozpocznij przeszukiwanie.
bottom_select_button_tooltip = Zaznacz elementy.
bottom_delete_button_tooltip = Usuń zaznaczone elementy.
bottom_save_button_tooltip = Zapisz informacje o skanowaniu.
bottom_symlink_button_tooltip =
    Tworzy linki symboliczne.
    Działa tylko jeśli przynajmniej 2 wyniki są zaznaczone w danej grupie.
    Pierwszy zaznaczony nie jest modyfikowany, a drugi i kolejne są usuwane i tworzone linki symboliczne w ich miejscach.
bottom_hardlink_button_tooltip =
    Tworzy twarde linki.
    Działa tylko jeśli przynajmniej 2 wyniki są zaznaczone w danej grupie.
    Pierwszy zaznaczony nie jest modyfikowany, a drugi i kolejne są usuwane i tworzone twarde linki w ich miejscach.
bottom_move_button_tooltip =
    Przenosi pliki do podanej lokalizacji.
    Kopiuje pliki bez zachowywania struktury katalogów.
    Podczas próby skopiowania 2 plików z identycznymi nazwami, drugi nie zostanie przeniesiony i błąd zostanie wyświetlony.
bottom_show_errors_tooltip = Pokazuje/ukrywa dolny panel z informacjami.
bottom_show_upper_notebook_tooltip = Pokazuje/ukrywa górny panel.
# Progress Window
progress_stop_button = Stop
# About Window
about_repository_button_tooltip = Link do repozytorium z kodem źródłowym
about_donation_button_tooltip = Link do strony z dotacjami.
about_instruction_button_tooltip = Link do strony z instrukcją.
about_translation_button_tooltip = Link do strony Crowdin z tłumaczeniami aplikacji. Oficjalnie polski i angielski są wspierane ale za każdą pomoc w tłumaczeniu innych języków będę wdzięczny.
about_repository_button = Repozytorium
about_donation_button = Dotacje
about_instruction_button = Instrukcja(ENG)
about_translation_button = Tłumaczenie
# Header
header_setting_button_tooltip = Otwórz okno z ustawieniami programu.
header_about_button_tooltip = Otwórz okno z informacjami o programie.

# Settings


## General

settings_save_at_exit_button_tooltip = Zapisuje konfigurację programu podczas wychodzenia z niego.
settings_load_at_start_button_tooltip =
    Ładowanie plików na starcie z plików.
    
    Nie zaznaczenie tej opcji, spowoduje załadowanie domyślnych ustawień.
settings_confirm_deletion_button_tooltip = Wyświetla okno potwierdzające usuwanie plików.
settings_confirm_link_button_tooltip = Wyświetla okno potwierdzające usuwanie, gdy tworzone są hard/symlinki.
settings_confirm_group_deletion_button_tooltip = Wyświetla okno potwierdzające usuwanie, gdy wszystkie rekordy w danej grupie są zaznaczone.
settings_show_text_view_button_tooltip = Pokazuje na dole ekranu panel tekstowy.
settings_use_cache_button_tooltip = Umożliwia zapisywanie rekordów do pamięci podręcznej.
settings_save_also_as_json_button_tooltip = Zapisz pamięć podręczną do pliku w formacie JSON umożliwiającym zmiany zawartości. Pamięć podręczna z tego pliku zostanie odczytana automatycznie przez aplikację, jeśli nie istnieje plik z pamięcią podręczną w formacie binarnym (z rozszerzeniem bin).
settings_use_trash_button_tooltip = Przenosi pliki do kosza zamiast usuwać je permanentnie.
settings_language_label_tooltip = Pozwala wybrać język interfejsu.
settings_save_at_exit_button = Zapisuj konfigurację przy wyłączaniu
settings_load_at_start_button = Ładuj ustawienia na starcie z pliku
settings_confirm_deletion_button = Pokazuj okno potwierdzające usuwanie plików
settings_confirm_link_button = Pokazuj potwierdzenie usuwania hard/symlinków
settings_confirm_group_deletion_button = Pokazuj okno potwierdzające usuwanie wszystkich obiektów w grupie
settings_show_text_view_button = Pokazuj panel tekstowy na dole
settings_use_cache_button = Używaj pamięci podręcznej
settings_save_also_as_json_button = Zapisz pamięć podręczną również do pliku JSON
settings_use_trash_button = Przenoś pliki do kosza
settings_language_label = Język
settings_multiple_delete_outdated_cache_checkbutton = Usuwaj automatycznie nieaktualne rekordy z pamięci podręcznej
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Pozwala na automatyczne usuwanie rekordów, które wskazują na nieaktualne pliki.
    
    W przypadku gdy pole jest zaznaczone, upewnij się, że wszystkie dyski zewnętrzne są podpięte by nie stracić zapisanych hashy z pamięci podręcznej.
    
    Wyłączenie tej opcji spowoduje, że nawet przy skanowaniu odpiętych dysków, rekordy w pamięci podręcznej nie będą usuwane.
    
    W przypadku posiadania dziesiątek czy setek tysięcy rekordów w pamięci podręcznej, zalecane jest zaznaczenie tej opcji, ponieważ przyspiesza to ładowanie i zapisywanie pamięci podręcznej.
settings_notebook_general = Ogólne
settings_notebook_duplicates = Duplikaty
settings_notebook_images = Podobne Obrazy
settings_notebook_videos = Podobne Wideo

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Pokazuje podgląd obrazów po ich zaznaczeniu po prawej stronie aplikacji.
settings_multiple_image_preview_checkbutton = Pokazuj podgląd obrazów
settings_multiple_clear_cache_button_tooltip = Ręcznie czyści pamięć podręczną z nieaktualnych danych.
settings_multiple_clear_cache_button = Usuń nieaktualne dane z pamięci podręcznej

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Ukrywa wszystkie pliki oprócz jednego, jeśli wskazują na dokładnie ten sam plik(to samo inode).
    
    Przykładowo - gdy program znalazł 7 plików na dokładnie ten sam plik(z tym samym inode) a dodatkowo jeden na inny, to w wynikach wyszukiwania wyświetlą się jedynie 2 wyniki - jeden z pierwszej grupy i drugi z drugiej.
settings_duplicates_minimal_size_entry_tooltip =
    Opcja umożliwia ustawienie minimalnej wielkości pliku, której hash będzie zapisywany do pamięci podręcznej.
    
    Im mniejsza wartość, tym hash większej ilości plików będzie przechowywany w pamięci podręcznej, co przyspieszy wyszukiwanie plików lecz jednocześnie spowolni wczytywanie/zapisywanie hashu do pliku.
settings_duplicates_prehash_checkbutton_tooltip =
    Umożliwia zapisywanie cząstkowego hashu do pamięci podręcznej, który umożliwia na wcześniejsze wyrzucenie plików z unikalnymi rozmiarami.
    
    Domyślnie jest zablokowane, ponieważ może powodować spowolnione skanowanie w niektórych sytuacjach.
    
    Jest mocno polecane osobom które skanują tylko i wyłącznie katalogi zawierające dziesiątki lub setki tysięcy lub nawet miliony plików, ponieważ może to wielokrotnie przyspieszyć proces skanowania.
settings_duplicates_prehash_minimal_entry_tooltip = Minimalny rozmiar pliku, którego cząstkowy hash będzie zapisywany do pamięci podręcznej.
settings_duplicates_hide_hard_link_button = Ukrywaj twarde dowiązania(nie działa na Windowsie)
settings_duplicates_prehash_checkbutton = Używaj pamięci podręcznej dla hashy cząstkowych
settings_duplicates_minimal_size_cache_label = Wielkość pliku, od którego hash będzie zapisywany w pamięci podręcznej
settings_duplicates_minimal_size_cache_prehash_label = Wielkość pliku, od którego cząstkowy hash będzie zapisywany w pamięci podręcznej

## Saving/Loading settings

settings_saving_button_tooltip = Zapisuje aktualne ustawienia do pliku.
settings_loading_button_tooltip = Ładuje ustawienia z pliku.
settings_reset_button_tooltip = Resetuje aktualne ustawienia do domyślnie używanych przez aplikację.
settings_saving_button = Zapisanie ustawień
settings_loading_button = Załadowanie ustawień
settings_reset_button = Reset ustawień

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Otwiera folder gdzie przechowywana jest pamięć podręczna aplikacji.
    
    Jej ręczne modyfikowanie może powodować wyświetlanie niepoprawnych wyników lub jej uszkodzenie spowoduje konieczność ponownej generacji, lecz umożliwia też oszczędzenie czasu przy przesuwaniu większej ilości plików.
    
    Można pliki kopiować pomiędzy komputerami by zaoszczędzić czas na hashowaniu plików(oczywiście tylko gdy dane są przechowywane w identycznej strukturze katalogów na komputerach).
settings_folder_settings_open_tooltip =
    Otwiera folder gdzie Czkawka przechowuje ustawienia.
    
    Ich ręczna zmiana, może spowodować różne błędy i kataklizmy, o których fizjologom się nie śniło.
settings_folder_cache_open = Otwórz folder pamięci podręcznej
settings_folder_settings_open = Otwórz folder ustawień
# Compute results
compute_stopped_by_user = Przeszukiwanie zostało zatrzymane przez użytkownika
compute_found_duplicates_hash_size = Znaleziono { $number_files } duplikatów w { $number_groups } grupach, które zajmują { $size }
compute_found_duplicates_name = Znaleziono { $number_files } duplikatów w { $number_groups } grupach
compute_found_empty_folders = Znaleziono { $number_files } pustych folderów
compute_found_empty_files = Znaleziono { $number_files } pustych plików
compute_found_big_files = Znaleziono { $number_files } największych plików
compute_found_temporary_files = Znaleziono { $number_files } tymczasowych plików
compute_found_images = Znaleziono { $number_files } podobnych obrazów w { $number_groups } grupach
compute_found_videos = Znaleziono { $number_files } podobnych plików wideo w { $number_groups } grupach
compute_found_music = Znaleziono { $number_files } podobnych plików muzycznych w { $number_groups } grupach
compute_found_invalid_symlinks = Znaleziono { $number_files } niepoprawnych dowiązań symbolicznych
compute_found_broken_files = Znaleziono { $number_files } uszkodzonych plików
# Progress window
progress_scanning_general_file = Skanowanie { $file_number } pliku
progress_scanning_broken_files = Sprawdzanie { $file_checked }/{ $all_files } pliku
progress_scanning_video = Hashowanie { $file_checked }/{ $all_files } pliku wideo
progress_scanning_image = Hashowanie { $file_checked }/{ $all_files } obrazu
progress_comparing_image_hashes = Porównywanie { $file_checked }/{ $all_files } hashu obrazu
progress_scanning_music_tags_end = Porównywanie tagów { $file_checked }/{ $all_files } pliku audio
progress_scanning_music_tags = Sczytywanie tagów { $file_checked }/{ $all_files } pliku audio
progress_scanning_empty_folders = Przeszukiwanie { $folder_number } folderu
progress_scanning_size = Sprawdzanie rozmiaru { $file_number } pliku
progress_scanning_name = Sprawdzanie nazwy { $file_number } pliku
progress_analyzed_partial_hash = Obliczanie częściowego hashu { $file_checked }/{ $all_files } pliku
progress_analyzed_full_hash = Obliczanie pełnego hashu { $file_checked }/{ $all_files } pliku
progress_current_stage = Aktualny Etap:{ "  " }
progress_all_stages = Wszystkie Etapy:{ "  " }
# Saving loading 
saving_loading_saving_success = Zapisano konfigurację do pliku { $name }.
saving_loading_saving_failure = Nie udało zapisać się konfiguracji do pliku { $name }.
saving_loading_reset_configuration = Przywrócono domyślą konfigurację.
saving_loading_loading_success = Poprawnie załadowano ustawienia aplikacji.
saving_loading_invalid_string = Dla klucza "{ $key }" znaleziono niepoprawną wartość - "{ $result }" która nie jest jedno-liniowym ciągiem znaków.
saving_loading_invalid_int = Dla klucza "{ $key }" znaleziono niepoprawną wartość - "{ $result }" która nie jest liczbą całkowitą.
saving_loading_invalid_bool = Dla klucza "{ $key }" znaleziono niepoprawną wartość - "{ $result }" która nie jest typem logicznym.
saving_loading_decode_problem_bool = Nie udało się zdekodować wartości z klucza "{ $key }", znaleziono "{ $result }" ale dozwolone wartości to 0, 1, true lub false.
saving_loading_saving_same_keys = Próba zapisania ustawień z zduplikowanym kluczem "{ $key }".
saving_loading_failed_to_get_home_directory = Nie udało się pobrać informacji o katalogu domowym by otworzyć i zapisać plik konfiguracyjny.
saving_loading_folder_config_instead_file = Nie można utworzyć lub otworzyć pliku konfiguracyjnego w ścieżce "{ $path }", ponieważ w danej ścieżce istnieje już folder.
saving_loading_failed_to_create_configuration_folder = Nie udało się utworzyć folderu konfiguracyjnego "{ $path }", powód "{ $reason }".
saving_loading_failed_to_create_config_file = Nie udało się utworzyć pliku konfiguracyjnego "{ $path }", powód "{ $reason }".
saving_loading_failed_to_read_config_file = Nie można załadować konfiguracji z "{ $path }" ponieważ nie istnieje lub nie jest plikiem.
saving_loading_failed_to_read_data_from_file = Nie można odczytać danych z pliku "{ $path }", powód "{ $reason }".
saving_loading_orphan_data = Znaleziono osierocone dane "{ $data }" w wierszu "{ $line }".
saving_loading_not_valid = Ustawienie "{ $data }" nie istnieje w bieżącej wersji aplikacji.
# Invalid symlinks
invalid_symlink_infinite_recursion = Nieskończona rekurencja
invalid_symlink_non_existent_destination = Nie istniejący docelowy plik
# Other
searching_for_data = Przeszukiwanie dysku, może to potrwać chwilę, proszę czekać...
text_view_messages = WIADOMOŚCI
text_view_warnings = OSTRZEŻENIA
text_view_errors = BŁĘDY
about_window_motto =
    Program jest i będzie zawsze darmowy do użytku.
    
    Może interfejs programu nie jest ergonomiczny,
    ale za to przynajmniej kod jest nieczytelny.
# Various dialog
dialogs_ask_next_time = Pytaj następnym razem
delete_file_failed = Nie udało się usunąć pliku { $name }, powód { $reason }
delete_title_dialog = Potwierdzenie usunięcia
delete_question_label = Czy na pewno usunąć te pliki?
delete_all_files_in_group_title = Potwierdzenie usunięcia wszystkich plików w grupie
delete_all_files_in_group_label1 = W niektórych grupach zaznaczono wszystkie rekordy.
delete_all_files_in_group_label2 = Czy na pewno je usunąć?
delete_folder_failed = Nie udało się usunąć folderu { $dir } ponieważ nie istnieje, uprawnienia nie są wystarczające lub nie jest pusty.
delete_items_label = { $items } plików będzie usuniętych.
delete_items_groups_label = { $items } plików z { $groups } grup będzie usuniętych.
hardlink_failed = Nie udało się utworzyć twardego dowiązania
hard_sym_invalid_selection_title_dialog = Niepoprawne zaznaczenie w niektórych grupach
hard_sym_invalid_selection_label_1 = W niektórych grupach zaznaczono tylko 1 rekord, który zostanie zignorowany.
hard_sym_invalid_selection_label_2 = Aby móc używać dowiązań, należy zaznaczyć przynajmniej 2 obiekty w danej grupie.
hard_sym_invalid_selection_label_3 = Pierwszy pozostaje nienaruszony a drugi i kolejne są dowiązywane do tego pierwszego.
hard_sym_link_title_dialog = Potwierdzenie dowiązania
hard_sym_link_label = Czy na pewno dowiązać te pliki?
move_folder_failed = Nie można przenieść folderu { $name }, powód { $reason }
move_file_failed = Nie można przenieść pliku { $name }, powód { $reason }
move_files_title_dialog = Wybierz folder, do którego zostaną przeniesione pliki
move_files_choose_more_than_1_path = Można przenieść elementy tylko do 1 folderu, zaznaczono { $path_number }.
move_stats = Poprawnie przeniesiono { $num_files }/{ $all_files } elementów
save_results_to_file = Zapisano wyniki do pliku { $name }
search_not_choosing_any_music = BŁĄD: Musisz zaznaczyć przynajmniej jeden pole, według którego będą wyszukiwane podobne pliki muzyczne.
include_folders_dialog_title = Foldery do przeszukiwania
exclude_folders_dialog_title = Foldery do ignorowania
include_manually_directories_dialog_title = Dodaj katalogi ręcznie
cache_properly_cleared = Poprawnie wyczyszczono pamięć podręczną
cache_clear_duplicates_title = Czyszczenie pamięci podręcznej duplikatów
cache_clear_similar_images_title = Czyszczenie pamięci podręcznej podobnych obrazów
cache_clear_similar_videos_title = Czyszczenie pamięci podręcznej podobnych plików wideo
cache_clear_message_label_1 = Czy na pewno chcesz oczyścić pamięć podręczną z przestarzałych wpisów?
cache_clear_message_label_2 = Ta operacja usunie wszystkie rekordy, które wskazują na nieistniejące pliki.
cache_clear_message_label_3 = Może spowodować to przyspieszenie ładowania i zapisywania danych do pamięci w trakcie skanowania.
cache_clear_message_label_4 = OSTRZEŻENIE: Usunięte zostaną wszystkie rekordy z odpiętych dyskach zewnętrznych i konieczne będzie ich ponowne sprawdzenie po podpięciu.
# Show preview
preview_temporary_file = Nie udało się otworzyć tymczasowego obrazu { $name }, powód { $reason }.
preview_0_size = Nie można stworzyć podglądu obrazu { $name }, z wysokością lub szerokością 0 pikseli.
preview_temporary_image_save = Nie udało się zapisać tymczasowego obrazu do { $name }, powód { $reason }.
preview_temporary_image_remove = Nie udało się usunąć tymczasowego obrazu { $name }, powód { $reason }.
preview_failed_to_create_cache_dir = Nie udało stworzyć się katalogu { $name } wymaganego do stworzenia podglądu obrazu, powód { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupa { $current_group }/{ $all_groups } ({ $images_in_group } obrazów)
compare_move_left_button = L
compare_move_right_button = P
