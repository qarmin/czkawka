# Window titles
window_settings_title = Ustawienia
window_main_title = Czkawka
window_progress_title = Skanowanie
window_compare_images = Porównywanie Obrazów
# General
general_ok_button = Ok
general_close_button = Zamknij
# Main window
music_title_checkbox = Tytuł
music_artist_checkbox = Wykonawca
music_year_checkbox = Rok
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Gatunek
music_length_checkbox = Długość
music_comparison_checkbox = Przybliżone Porównywanie
music_checking_by_tags = Tagi
music_checking_by_content = Zawartość
same_music_seconds_label = Minimalny fragment drugi czas trwania
same_music_similarity_label = Maksymalna różnica
music_compare_only_in_title_group = Porównaj tylko w tytule
music_compare_only_in_title_group_tooltip =
    Gdy włączone, pliki są pogrupowane według tytułu, a następnie porównywane do siebie.
    
    z 10000 plików, zamiast tego prawie 100 milionów porównań będzie około 20000 porównań.
same_music_tooltip =
    Wyszukiwanie podobnych plików muzycznych przez jego zawartość można skonfigurować przez ustawienie:
    
    - Minimalny czas fragmentu, po którym pliki muzyczne mogą być zidentyfikowane jako podobne
    - Maksymalna różnica między dwoma testowanymi fragmentami
    
    Kluczem do dobrych wyników jest znalezienie rozsądnych kombinacji tych parametrów, do dostarczania.
    
    Ustawianie minimalnego czasu na 5s i maksymalnej różnicy na 1.0, będzie szukać prawie identycznych fragmentów w plikach.
    Czas 20s i maksymalna różnica 6.0, z drugiej strony, dobrze działa w poszukiwaniu remiksów/wersji na żywo itp.
    
    Domyślnie każdy plik muzyczny jest porównywany ze sobą, co może zająć dużo czasu podczas testowania wielu plików, więc zwykle lepiej jest używać folderów referencyjnych i określać, które pliki mają być porównywane ze sobą (z taką samą ilością plików, porównywanie odcisków palców będzie szybsze niż bez folderów referencyjnych).
music_comparison_checkbox_tooltip =
    Wyszukuje podobne pliki muzyczne za pomocą AI, która używa nauki maszynowej, aby usunąć nawiasy z frazy. Na przykład, z tą opcją włączoną, rozpatrywane pliki będą traktowane jako duplikaty:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Uwzględnij Wielkość Liter
duplicate_case_sensitive_name_tooltip =
    Gdy włączone, grupowe rekordy tylko wtedy, gdy mają dokładnie taką samą nazwę, np. Żołd <-> Żołd
    
    Wyłączenie tej opcji spowoduje grupowanie nazw bez sprawdzania, czy każda litera ma ten sam rozmiar, np. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Rozmiar i nazwa
duplicate_mode_name_combo_box = Nazwa
duplicate_mode_size_combo_box = Rozmiar
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka oferuje 3 rodzaje hashów:
    
    Blake3 - kryptograficzna funkcja skrótu. Jest to wartość domyślna, ponieważ jest bardzo szybka.
    
    CRC32 - prosta funkcja haszująca. Powinno to być szybsze od Blake3, ale bardzo rzadko może to prowadzić do kolizji.
    
    XXH3 - bardzo podobna pod względem wydajności i jakości hashu do Blake3 (ale niekryptograficzna). Tak więc takie tryby mogą być łatwo wymienione.
duplicate_check_method_tooltip =
    Na razie Czkawka oferuje trzy typy metod do znalezienia duplikatów przez:
    
    Nazwa - Znajduje pliki o tej samej nazwie.
    
    Rozmiar - Znajduje pliki o tym samym rozmiarze.
    
    Hash - Znajduje pliki, które mają tę samą zawartość. Ten tryb haszuje plik, a następnie porównuje utworzony skrót(hash) aby znaleźć duplikaty. Ten tryb jest najbezpieczniejszym sposobem na znalezienie duplikatów. Aplikacja używa pamięci podręcznej, więc drugie i kolejne skanowanie tych samych danych powinno być dużo szybsze niż za pierwszym razem.
image_hash_size_tooltip =
    Każdy zaznaczony obraz tworzy specjalny skrót, który można porównać ze sobą, a niewielka różnica między nimi oznacza, że obrazy te są podobne.
    
    8 hashh rozmiar jest dość dobry do wyszukiwania obrazów, które są tylko trochę podobne do oryginału. Dzięki większemu zestawowi zdjęć (>1000), spowoduje to uzyskanie dużej ilości fałszywych dodatnich, więc zalecam użycie większego rozmiaru skrótu w tym przypadku.
    
    16 to domyślny rozmiar hasha, który jest dość dobrym kompromisem między znalezieniem nawet nieco podobnych obrazów a zaledwie niewielką ilością kolizji haszujących.
    
    32 i 64 hashy znajdują tylko bardzo podobne obrazy, ale nie powinny mieć prawie żadnych fałszywych pozytywnych (może z wyjątkiem niektórych obrazów z kanałem alfa).
image_resize_filter_tooltip =
    Aby obliczyć skrót obrazu, biblioteka musi najpierw zmienić jego rozmiar.
    
    Dołącz do wybranego algorytmu, wynikowy obraz użyty do obliczenia skrótu będzie wyglądał nieco inaczej.
    
    Najszybszy algorytm do użycia, ale także ten, który daje najgorsze wyniki, jest najbardziej potrzebny. Domyślnie jest włączona, ponieważ przy rozmiarze skrótu 16x16 jego jakość nie jest naprawdę widoczna.
    
    Przy rozmiarze skrótu 8x8 zaleca się użycie innego algorytmu niż najbliższy, aby mieć lepsze grupy obrazów.
image_hash_alg_tooltip =
    Użytkownicy mogą wybrać jeden z wielu algorytmów obliczania hashu.
    
    Każdy ma zarówno mocniejsze jak i słabsze punkty i czasami daje lepsze a czasami gorsze wyniki dla różnych obrazów.
    
    Najlepiej jest samemu potestować jaki algorytm ma najlepsze wyniki(może to nie być zawsze dobrze widoczne).
big_files_mode_combobox_tooltip = Pozwala na wyszukiwanie najmniejszych lub największych plików
big_files_mode_label = Sprawdzane pliki
big_files_mode_smallest_combo_box = Najmniejsze
big_files_mode_biggest_combo_box = Największe
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
main_notebook_bad_extensions = Błędne rozszerzenia
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
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Długość
main_tree_view_column_genre = Gatunek
main_tree_view_column_symlink_file_name = Nazwa Symlinka
main_tree_view_column_symlink_folder = Folder Symlinka
main_tree_view_column_destination_path = Docelowa Ścieżka
main_tree_view_column_type_of_error = Typ Błędu
main_tree_view_column_current_extension = Aktualne rozszerzenie
main_tree_view_column_proper_extensions = Poprawne rozszerzenia
main_label_check_method = Metoda sprawdzania
main_label_hash_type = Typ hashu
main_label_hash_size = Rozmiar hashu
main_label_size_bytes = Rozmiar (bajty)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Liczba wyświetlanych plików
main_label_resize_algorithm = Algorytm zmiany rozmiaru
main_label_similarity = Podobieństwo{ "   " }
main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archiwa
main_check_box_broken_files_image = Obraz
check_button_general_same_size = Ignoruj identyczny rozmiar
check_button_general_same_size_tooltip = Ignoruj pliki o identycznym rozmiarze w wynikach - zazwyczaj są to duplikaty 1:1
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
upper_manual_add_included_button_tooltip =
    Dodaj nazwę katalogu do ręcznego wyszukiwania.
    
    Aby dodać wiele ścieżek na raz, należy je oddzielić za pomocą średnika ;
    
    /home/roman;/home/rozkaz doda dwa katalogi /home/roman i /home/rozkaz
upper_add_included_button_tooltip = Dodaje wybrany folder do przeskanowania.
upper_remove_included_button_tooltip = Usuwa zaznaczony folder z listy do skanowania.
upper_manual_add_excluded_button_tooltip =
    Dodaj ręcznie katalog do listy wykluczonych.
    
    Aby dodać wiele ścieżek na raz, oddziel je średnikiem ;
    
    /home/roman;/home/krokiet doda dwa katalogi /home/roman i /home/keokiet
upper_add_excluded_button_tooltip = Dodaje wybrany folder do ignorowanych.
upper_remove_excluded_button_tooltip = Usuwa zaznaczony folder z ignorowanych.
upper_notebook_items_configuration = Konfiguracja Skanowania
upper_notebook_excluded_directories = Ignorowane Foldery
upper_notebook_included_directories = Przeszukiwane Foldery
upper_allowed_extensions_tooltip =
    Dozwolone rozszerzenia muszą być oddzielone przecinkami (domyślnie wszystkie są dostępne).
    
    Istnieją makra, które umożliwiają dołączenie za jednym razem określonych typów plików IMAGE, VIDEO, MUSIC, TEXT.
    
    Przykład użycia ".exe, IMAGE, VIDEO, .rar, 7z" - oznacza że obrazy (np. jpg, png), filmy (np. avi, mp4), exe, rar i 7z zostaną sprawdzone.
upper_excluded_extensions_tooltip =
    Lista wyłączonych plików, które zostaną zignorowane w skanowaniu.
    
    Gdy używasz zarówno dozwolonych, jak i wyłączonych rozszerzeń, ten ma wyższy priorytet, więc plik nie zostanie sprawdzony.
upper_excluded_items_tooltip =
    Wykluczone elementy muszą zawierać znak * (który odpowiada za dowolny ciąg znaków) i powinny być oddzielone przecinkami.
    Jest to wolniejszy sposób od zwykłego wykluczania katalogów, więc należy używać go ostrożnie.
upper_excluded_items = Ignorowane Obiekty:
upper_allowed_extensions = Dozwolone Rozszerzenia:
upper_excluded_extensions = Wyłączone rozszerzenia:
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
    Zaznacza rekordy według ścieżki.
    
    Przykładowe użycie:
    /home/pimpek/rzecz.txt można znaleźć używając /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Zaznacza rekordy według nazw plików.
    
    Przykładowe użycie:
    /usr/ping/pong.txt można znaleźć za pomocą *ong*
popover_custom_regex_check_button_entry_tooltip =
    Wybierz rekordy według określonego Regexa.
    
    W tym trybie wyszukiwanym tekstem jest pełna ścieżka(wraz z nazwą).
    
    Przykładowe użycie:
    /usr/bin/ziemniak. xt można znaleźć za pomocą /ziem[a-z]+
    
    Używana jest tutaj domyślnej implementacja Regexa w Rust. Więcej informacji na ten temat można znaleźć tutaj: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Umożliwia wykrywanie wielkości liter.
    
    Wykluczenie /home/* znajdzie zarówno /HoMe/roman, jak i /home/roman.
popover_custom_not_all_check_button_tooltip =
    Zapobiega wybraniu wszystkich rekordów w grupie.
    
    Ta opcja jest domyślnie włączona, ponieważ w większości sytuacji prawdopodobnie nie chcesz usuwać zarówno oryginałów jak i duplikatów, lecz chcesz pozostawić co najmniej jeden plik.
    
    OSTRZEŻENIE: To ustawienie nie działa jeśli wcześniej ręcznie zostały wybrane wszystkie rekordy w grupie.
popover_custom_regex_path_label = Ścieżka
popover_custom_regex_name_label = Nazwa
popover_custom_regex_regex_label = Regex - Pełna ścieżka
popover_custom_case_sensitive_check_button = Rozróżniaj wielkość liter
popover_custom_all_in_group_label = Nie zaznaczaj wszystkich rekordów w grupie
popover_custom_mode_unselect = Własne odznaczanie
popover_custom_mode_select = Własne zaznaczanie
popover_sort_file_name = Nazwa pliku
popover_sort_folder_name = Nazwa katalogu
popover_sort_full_name = Pełna nazwa
popover_sort_size = Rozmiar
popover_sort_selection = Zaznaczanie
popover_invalid_regex = Regex jest niepoprawny
popover_valid_regex = Regex jest poprawny
# Bottom buttons
bottom_search_button = Szukaj
bottom_select_button = Zaznacz
bottom_delete_button = Usuń
bottom_save_button = Zapisz
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_compare_button = Porównaj
bottom_move_button = Przenieś
bottom_sort_button = Sortuj
bottom_search_button_tooltip = Rozpocznij wyszukiwanie
bottom_select_button_tooltip = Wybierz rekordy. Tylko wybrane pliki/foldery mogą być później przetwarzane.
bottom_delete_button_tooltip = Usuń zaznaczone elementy.
bottom_save_button_tooltip = Zapisz informacje o skanowaniu.
bottom_symlink_button_tooltip =
    Utwórz linki symboliczne.
    Działa tylko wtedy, gdy co najmniej dwa wyniki w grupie są zaznaczone.
    Pierwszy jest niezmieniony, drugi i następny jest powiązywany z pierwszym.
bottom_hardlink_button_tooltip =
    Tworzenie twardych linków.
    Działa tylko wtedy, gdy wybrano co najmniej dwa rekordy w grupie.
    Pierwszy jest niezmieniony, drugi i następny jest dowiązywany z pierwszym.
bottom_hardlink_button_not_available_tooltip =
    Tworzenie twardych dowiązań.
    Przycisk jest zablokowany, gdyż stworzenie twardego dowiązania nie jest możliwe.
    Dowiązanie tego rodzaju może tworzyć administrator w systemie Windows, więc należy upewnić się że aplikacja jest uruchomiona przez z tymi uprawnieniami.
    Jeśli aplikacja działa z nimi, należy przeszukać issues w Githubie celem znalezienia możliwych rozwiązań danego problemu.
bottom_move_button_tooltip =
    Przenosi pliki do wybranego katalogu.
    Kopiuje wszystkie pliki do katalogu bez zachowania struktury plików.
    Podczas próby przeniesienia dwóch plików o identycznej nazwie do folderu, drugi plik nie zostanie przeniesiony i pojawi się błąd.
bottom_compare_button_tooltip = Porównuje pliki/foldery w grupach.
bottom_sort_button_tooltip = Sortuje pliki/foldery zgodnie z wybraną metodą.
bottom_show_errors_tooltip = Pokaż/Ukryj dolny panel tekstowy.
bottom_show_upper_notebook_tooltip = Pokazuje/ukrywa górny panel.
# Progress Window
progress_stop_button = Stop
progress_stop_additional_message = Przerywanie skanowania
# About Window
about_repository_button_tooltip = Link do repozytorium z kodem źródłowym
about_donation_button_tooltip = Link do strony z dotacjami.
about_instruction_button_tooltip = Link do strony z instrukcją.
about_translation_button_tooltip = Link do strony Crowdin z tłumaczeniami aplikacji. Oficialnie wspierany jest język polski i angielski.
about_repository_button = Repozytorium
about_donation_button = Dotacje
about_instruction_button = Instrukcja(ENG)
about_translation_button = Tłumaczenie
# Header
header_setting_button_tooltip = Otwórz okno z ustawieniami programu.
header_about_button_tooltip = Otwórz okno z informacjami o programie.

# Settings


## General

settings_number_of_threads = Liczba używanych wątków
settings_number_of_threads_tooltip = Liczba używanych wątków, 0 oznacza, że zostaną użyte wszystkie dostępne wątki.
settings_use_rust_preview = Użyj zewnętrznych bibliotek zamiast gtk, aby załadować podgląd
settings_use_rust_preview_tooltip =
    Korzystanie z podglądu gtk będzie czasem szybsze i obsługuje więcej formatów, ale czasami może to być dokładnie odwrotne.
    
    Jeśli masz problemy z ładowaniem podglądów, możesz spróbować zmienić to ustawienie.
    
    W systemach innych niż linux zaleca się użycie tej opcji, ponieważ gtk-pixbuf nie zawsze jest tam dostępny, więc wyłączenie tej opcji nie załaduje podglądu niektórych obrazów.
settings_label_restart = Musisz ponownie uruchomić aplikację, aby aplikacja zaciągnęła nowe ustawienia!
settings_ignore_other_filesystems = Ignoruj inne systemy plików (tylko Linux)
settings_ignore_other_filesystems_tooltip =
    ignoruje pliki, które nie są w tym samym systemie plików co przeszukiwane katalogi.
    
    Działa tak samo jak opcja -xdev w komendzie find na Linux
settings_save_at_exit_button_tooltip = Zapisz konfigurację do pliku podczas zamykania aplikacji.
settings_load_at_start_button_tooltip =
    Wczytaj konfigurację z pliku podczas otwierania aplikacji.
    
    Jeśli nieaktywny, zostaną użyte domyślne ustawienia.
settings_confirm_deletion_button_tooltip = Pokaż okno dialogowe potwierdzające usuwanie przy próbie usunięcia rekordu.
settings_confirm_link_button_tooltip = Pokaż dodatkowe okno dialogowe przy próbie utworzenia hard/symlinków.
settings_confirm_group_deletion_button_tooltip = Pokaż okno dialogowe ostrzegające przy próbie usunięcia wszystkich rekordów z grupy.
settings_show_text_view_button_tooltip = Pokaż dolny panel tekstowy.
settings_use_cache_button_tooltip = Użyj pamięci podręcznej plików.
settings_save_also_as_json_button_tooltip = Zapisz pamięć podręczną do formatu JSON (czytelnego dla człowieka). Można modyfikować jego zawartość. Pamięć podręczna z tego pliku zostanie odczytana automatycznie przez aplikację, jeśli brakuje pamięci podręcznej formatu binarnego (z rozszerzeniem bin).
settings_use_trash_button_tooltip = Przenosi pliki do kosza zamiast usuwać je na stałe.
settings_language_label_tooltip = Język interfejsu użytkownika.
settings_save_at_exit_button = Zapisz konfigurację podczas zamykania aplikacji
settings_load_at_start_button = Załaduj konfigurację z pliku podczas otwierania aplikacji
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
    Usuń nieaktualne rekordy z pamięci podręcznej, które wskazują na nieistniejące pliki.
    
    Po włączeniu aplikacja upewnia się, że podczas ładowania rekordów wszystkie wskazują na prawidłowe pliki (uszkodzone czy zmienione pliki są ignorowane).
    
    Wyłączenie tej opcji, pomoże podczas skanowania plików na zewnętrznych dyskach, więc wpisy dotyczące ich nie zostaną usunięte w następnym skanowaniu.
    
    W przypadku posiadania stu tysięcy rekordów w pamięci podręcznej, sugeruje się, aby włączyć tę opcję, ponieważ przyspieszy ładowanie/zapisywanie pamięci podręcznej na początku/końcu skanowania.
settings_notebook_general = Ogólne
settings_notebook_duplicates = Duplikaty
settings_notebook_images = Podobne Obrazy
settings_notebook_videos = Podobne Wideo

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Pokazuje podgląd po prawej stronie (podczas zaznaczania obrazu).
settings_multiple_image_preview_checkbutton = Pokazuj podgląd obrazów
settings_multiple_clear_cache_button_tooltip =
    Ręcznie wyczyść pamięć podręczną przestarzałych wpisów.
    To powinno być używane tylko wtedy, gdy automatyczne czyszczenie zostało wyłączone.
settings_multiple_clear_cache_button = Usuń nieaktualne wyniki z pamięci podręcznej.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Ukrywa wszystkie pliki z wyjątkiem jednego, jeśli wszystkie wskazują na te same dane (są połączone twardym dowiązaniem).
    
    Przykład: W przypadku gdy istnieje (na dysku) siedem plików, które są twardo dowiązane ze sobą i jeden inny plik z tymi samymi danymi, ale innym inode, wtedy w oknie wyników, wyświetlony zostanie tylko jeden unikalny plik i jeden plik z siedmiu dowiązanych ze sobą plików.
settings_duplicates_minimal_size_entry_tooltip =
    Ustaw minimalny rozmiar pliku, który zapisywany będzie do pliku z pamięcią podręcznej.
    
    Wybór mniejszej wartości spowoduje wygenerowanie większej ilości rekordów. To przyspieszy wyszukiwanie, ale spowolni ładowanie/zapisywanie danych do pamięci podręcznej.
settings_duplicates_prehash_checkbutton_tooltip =
    Włącza zapisywanie częściowych haszów do pamięci podręcznej (hash obliczany jest tylko z małej części pliku), które pozwala na wcześniejsze odrzucenie unikalnych plików.
    
    Jest domyślnie wyłączona opcja, ponieważ może spowodować spowolnienie w niektórych sytuacjach.
    
    Zaleca się używanie tej opcji podczas skanowania setek tysięcy lub milionów plików, ponieważ może przyspieszyć wielokrotnie przeszukiwanie i wyłaczać gdy skanuje się niewielką ilość danych.
settings_duplicates_prehash_minimal_entry_tooltip = Minimalny rozmiar pliku, którego cząstkowy hash będzie zapisywany do pamięci podręcznej.
settings_duplicates_hide_hard_link_button = Ukryj twarde dowiązania (tylko Linux i macOS)
settings_duplicates_prehash_checkbutton = Używaj pamięci podręcznej dla hashy cząstkowych
settings_duplicates_minimal_size_cache_label = Minimalny rozmiar plików (w bajtach) zapisywanych do pamięci podręcznej
settings_duplicates_minimal_size_cache_prehash_label = Minimalny rozmiar plików (w bajtach) przy zapisywaniu ich częściowego haszu do pamięci podręcznej

## Saving/Loading settings

settings_saving_button_tooltip = Zapisz aktualną konfigurację ustawień do pliku.
settings_loading_button_tooltip = Załaduj ustawienia z pliku i nadpisz bieżącą konfigurację.
settings_reset_button_tooltip = Przywróć domyślną konfigurację.
settings_saving_button = Zapisanie ustawień
settings_loading_button = Załadowanie ustawień
settings_reset_button = Reset ustawień

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Otwiera folder gdzie przechowywana jest pamięć podręczna aplikacji.
    
    Ręczne modyfikowanie może powodować wyświetlanie niepoprawnych wyników lub jej uszkodzenie spowoduje konieczność ponownej generacji, lecz umożliwia też oszczędzenie czasu przy przesuwaniu większej ilości plików.
    
    Pliki można kopiować pomiędzy komputerami by zaoszczędzić czas na hashowaniu plików (oczywiście tylko gdy dane są przechowywane w identycznej strukturze katalogów na komputerach).
    
    W razie problemów z pamięcią podręczną, pliki mogą zostać usunięte. Aplikacja automatycznie je zregeneuje.
settings_folder_settings_open_tooltip =
    Otwiera folder, w którym konfiguracja Czkawki jest przechowywana.
    
    OSTRZEŻENIE: ręczna modyfikacja konfiguracji może zakłócić przepływ twojej pracy.
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
compute_found_bad_extensions = Znaleziono { $number_files } plików z nieprawidłowymi rozszerzeniami
# Progress window
progress_scanning_general_file = Skanowanie { $file_number } pliku
progress_scanning_extension_of_files = Sprawdzanie rozszerzenia { $file_checked }/{ $all_files } pliku
progress_scanning_broken_files = Sprawdzanie { $file_checked }/{ $all_files } plików
progress_scanning_video = Hashowanie { $file_checked }/{ $all_files } pliku wideo
progress_scanning_image = Hashowanie { $file_checked }/{ $all_files } obrazu
progress_comparing_image_hashes = Porównywanie { $file_checked }/{ $all_files } hashu obrazu
progress_scanning_music_tags_end = Porównywanie tagów { $file_checked }/{ $all_files } pliku audio
progress_scanning_music_tags = Sczytywanie tagów { $file_checked }/{ $all_files } pliku audio
progress_scanning_music_content_end = Porównywanie odcisku palca { $file_checked }/{ $all_files } pliku muzycznego
progress_scanning_music_content = Obliczanie odcisku palca { $file_checked }/{ $all_files } pliku muzycznego
progress_scanning_empty_folders = Przeszukiwanie { $folder_number } folderu
progress_scanning_size = Sprawdzanie rozmiaru { $file_number } pliku
progress_scanning_size_name = Skanowanie nazwy i rozmiaru pliku { $file_number }
progress_scanning_name = Sprawdzanie nazwy { $file_number } pliku
progress_analyzed_partial_hash = Obliczanie częściowego hashu { $file_checked }/{ $all_files } pliku
progress_analyzed_full_hash = Obliczanie pełnego hashu { $file_checked }/{ $all_files } pliku
progress_prehash_cache_loading = Ładowanie pamięci podręcznej prehashu
progress_prehash_cache_saving = Zapisywanie pamięci podręcznej prehashu
progress_hash_cache_loading = Ładowanie pamięci podręcznej hash
progress_hash_cache_saving = Zapisywanie pamięci podręcznej skrótu
progress_cache_loading = Ładowanie pamięci podręcznej
progress_cache_saving = Zapisywanie pamięci podręcznej
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
invalid_symlink_non_existent_destination = Nieistniejący docelowy plik
# Other
selected_all_reference_folders = Nie można rozpocząć wyszukiwania, gdy wszystkie katalogi są ustawione jako foldery źródłowe (referencyjne)
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
delete_all_files_in_group_label1 = W niektórych grupach wszystkie rekordy są zaznaczone.
delete_all_files_in_group_label2 = Czy na pewno je usunąć?
delete_folder_failed = Nie udało się usunąć folderu { $dir } ponieważ nie istnieje, uprawnienia nie są wystarczające lub nie jest pusty.
delete_items_label = { $items } plików będzie usuniętych.
delete_items_groups_label = { $items } plików z { $groups } grup zostanie usuniętych.
hardlink_failed = Nie udało się utworzyć twardego dowiązania
hard_sym_invalid_selection_title_dialog = Niepoprawne zaznaczenie w niektórych grupach
hard_sym_invalid_selection_label_1 = W niektórych grupach jest zaznaczony tylko jeden rekord i zostanie zignorowany.
hard_sym_invalid_selection_label_2 = Aby móc mocno połączyć te pliki, należy wybrać co najmniej dwa rekordy w grupie.
hard_sym_invalid_selection_label_3 = Pierwszy pozostaje nienaruszony a drugi i kolejne są dowiązywane do tego pierwszego.
hard_sym_link_title_dialog = Potwierdzenie dowiązania
hard_sym_link_label = Czy na pewno chcesz dowiązać te pliki?
move_folder_failed = Nie można przenieść folderu { $name }, powód { $reason }
move_file_failed = Nie można przenieść pliku { $name }, powód { $reason }
move_files_title_dialog = Wybierz folder, do którego zostaną przeniesione pliki
move_files_choose_more_than_1_path = Tylko jedna ścieżka może być wybrana, aby móc skopiować zduplikowane pliki, wybrano { $path_number }.
move_stats = Poprawnie przeniesiono { $num_files }/{ $all_files } elementów
save_results_to_file = Zapisano wyniki zarówno do plików txt, jak i json w folderze { $name }.
search_not_choosing_any_music = BŁĄD: Musisz zaznaczyć przynajmniej jeden pole, według którego będą wyszukiwane podobne pliki muzyczne.
search_not_choosing_any_broken_files = BŁĄD: Musisz wybrać co najmniej jedno pole wyboru z rodzajem uszkodzonych plików.
include_folders_dialog_title = Foldery do przeszukiwania
exclude_folders_dialog_title = Foldery do ignorowania
include_manually_directories_dialog_title = Dodaj katalogi ręcznie
cache_properly_cleared = Poprawnie wyczyszczono pamięć podręczną
cache_clear_duplicates_title = Czyszczenie pamięci podręcznej duplikatów
cache_clear_similar_images_title = Czyszczenie pamięci podręcznej podobnych obrazów
cache_clear_similar_videos_title = Czyszczenie pamięci podręcznej podobnych plików wideo
cache_clear_message_label_1 = Czy na pewno chcesz oczyścić pamięć podręczną z przestarzałych wpisów?
cache_clear_message_label_2 = Ta operacja usunie wszystkie rekordy, które wskazują na nieistniejące pliki.
cache_clear_message_label_3 = Może to nieznacznie przyspieszyć ładowanie/oszczędzanie pamięci podręcznej.
cache_clear_message_label_4 = OSTRZEŻENIE: Operacja usunie wszystkie dane w pamięci podręcznej z wyłączonych dysków zewnętrznych. Zatem każdy hash będzie musiał zostać zregenerowany.
# Show preview
preview_image_resize_failure = Nie udało się zmienić rozmiaru obrazu { $name }.
preview_image_opening_failure = Nie udało się otworzyć obrazu { $name }, powód { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupa { $current_group }/{ $all_groups } ({ $images_in_group } obrazów)
compare_move_left_button = L
compare_move_right_button = P
