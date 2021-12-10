# Różne
general_ok_button = Ok
general_close_button = Zamknij


general_bytes = bajtów
general_lost = zaprzepaszczono

# Główne okno(środek)
music_title_checkbox = Tytuł
music_artist_checkbox = Wykonawca
music_album_title_checkbox = Tytuł Albumu
music_album_artist_checkbox = Wykonawca Albumu
music_year_checkbox = Rok
music_comparison_checkbox = Przybliżone Porównywanie

duplicate_mode_name_checkbox = Nazwa
duplicate_mode_size_checkbox = Rozmiar
duplicate_mode_hash_checkbox = Hash


duplicate_mode_name_checkbox_tooltip = 
        Służy do znajdowania plików o identycznych nazwach.
        
        Ten tryb nie sprawdza zawartości pliku, dlatego należy uważać przy jego stosowaniu.
        
duplicate_mode_size_checkbox_tooltip = 
        Służy do znajdowania plików o identycznych rozmiarach.

        Ten tryb nie sprawdza zawartości pliku, dlatego należy uważać przy jego stosowaniu.
        
duplicate_mode_hash_checkbox_tooltip = 
        Znajduje pliki z identyczną zawartością, bez względu na nazwę i rozszerzenie pliku.

        W tym trybie, każdy plik jest hasowany a następnie każdy hash jest porównywany z innymi.
  
        Ten tryb używa pamięci podręcznej do przechowywania raz obliczonych hashy, dlatego drugie i kolejne skanowanie, powinno być o wiele szybsze niż pierwsze.

duplicate_hash_checkbox_blake3 = Blake3 jest kryptograficzną funkcją haszującą. Z racji połączenia szybkości i niskiej ilości kolizji jest to domyślny tryb.
duplicate_hash_checkbox_crc32 = CRC32 to prosta funkcja haszująca. Powinna być szybsza niż Blake3, lecz bardzo rzadko może mogą wystąpić kolizje hashy.
duplicate_hash_checkbox_xxh3 = XXH3 zarówno pod względem jakości hashu jak i wydajności jest podobny do Blake3, dlatego te algorytmy mogą być używane wymiennie.

image_hash_checkbox_8 = Domyślna wielkość hasha, pozwala na wyszukanie obrazów od bardzo do minimalnie podobnych. Przy niższych poziomach podobieństwa, może grupować wiele niepodobnych obrazków.
image_hash_checkbox_16 = Bardziej precyzyjny od 8. Przydatny gdy ze względu na dużą ilość zdjęć i kolizji między hashami niższy hash wyświetla nieprawidłowe dane.
image_hash_checkbox_32 = Umożliwia wyszukiwanie niemal identycznych obrazów, różniących się niewielkimi szczegółami.
image_hash_checkbox_64 = Tryb predatora, tworzy ogromne hashe które porównywane, wczytywane i zapisywane znacznie dłużej od poprzedników. Z racji wad nie powinien być raczej stosowany.

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
# Górne okno
upper_recursive_button = Rekursywnie
upper_recursive_button_tooltip = Jeśli zaznaczony, szuka plików i folderów również w katalogach wewnątrz, nawet jeśli nie znajdują się one bezpośrednio w tym folderze.

upper_manual_add_included_button = Ręcznie Dodaj
upper_add_included_button = Dodaj
upper_remove_included_button = Usuń
upper_manual_add_excluded_button = Ręcznie Dodaj
upper_add_excluded_button = Dodaj
upper_remove_excluded_button =  Usuń

upper_manual_add_included_button_tooltip = Pozwala ręcznie dodać foldery do skanowania
upper_add_included_button_tooltip = Dodaje wybrany folder do przeskanowania
upper_remove_included_button_tooltip =  Usuwa zaznaczony folder z listy do przeskanowania
upper_manual_add_excluded_button_tooltip = Pozwala ręcznie dodać foldery do ignorowania
upper_add_excluded_button_tooltip = Dodaje wybrany folder do ignorowanych
upper_remove_excluded_button_tooltip =  Usuwa zaznaczony folder z ignorowanych

upper_notebook_allowed_extension = Dozwolone Rozszerzenia
upper_notebook_excluded_items = Ignorowane Elementy
upper_notebook_excluded_directories = Ignorowane Foldery
upper_notebook_included_directories = Przeszukiwane Foldery

# Zaznaczanie elementów
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

        W tym trybie przeszukiwanym tekstem jest pełna ścieżka(nazwa + ścieżka)

        Example usage:
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
# Przyciski na dole
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

about_repository_button = Repozytorium
about_donation_button = Dotacje
about_instruction_button = Instrukcja(ENG)

# Header
header_setting_button_tooltip = Otwórz okno z ustawieniami programu.
header_about_button_tooltip = Otwórz okno z informacjami o programie.
header_language_button_tooltip = Używaj języka Polskiego lub Angielskiego.

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
settings_use_trash_button_tooltip = Przenosi pliki do kosza zamiast usuwać je permanentnie.

settings_save_at_exit_button = Zapisuj konfigurację przy wyłączaniu
settings_load_at_start_button = Ładuj ustawienia na starcie z pliku
settings_confirm_deletion_button = Pokazuj okno potwierdzające usuwanie plików
settings_confirm_link_button = Pokazuj potwierdzenie usuwania hard/symlinków
settings_confirm_group_deletion_button = Pokazuj okno potwierdzające usuwanie wszystkich obiektów w grupie
settings_show_text_view_button = Pokazuj panel tekstowy na dole
settings_use_cache_button = Używaj pamięci podręcznej
settings_use_trash_button = Przenoś pliki do kosza


## Multiple - ustawienia wskazywane przez większość zakładek
settings_multiple_delete_outdated_cache_checkbutton = Usuwaj automatycznie nieaktualne rekordy z pamięci podręcznej
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Pozwala na automatyczne usuwanie rekordów, które wskazują na nieaktualne pliki.

        W przypadku gdy pole jest zaznaczone, upewnij się, że wszystkie dyski zewnętrzne są podpięte by nie stracić zapisanych hashów z pamięci podręcznej.

        Wyłączenie tej opcji spowoduje, że nawet przy skanowaniu odpiętych dysków, rekordy w pamięci podręcznej nie będą usuwane.

        W przypadku posiadania dziesiątek czy setek tysięcy rekordów w pamięci podręcznej, zalecane jest zaznaczenie tej opcji, ponieważ przyspiesza to ładowanie i zapisywanie pamięci podręcznej.

settings_multiple_image_preview_checkbutton_tooltip = Pokazuje podgląd obrazów po ich zaznaczeniu po prawej stronie aplikacji.
settings_multiple_image_preview_checkbutton = Pokazuj podgląd obrazów 

settings_multiple_clear_cache_button_tooltip = 
        Ręcznie czyści pamięć podręczną z nieaktualnych danych.
	Opcja powinna być używana tylko gdy automatyczne czyszczenie pamięci podręcznej jest wyłączone.

settings_multiple_clear_cache_button = Usuń nieaktualne dane z pamięci podręcznej

## Duplicates
settings_duplicates_hide_hard_link_button_tooltip = 
        Ukrywa wszystkie pliki oprócz jednego, jeśli wskazują na dokładnie ten sam plik(to samo inode).

        Przykładowo - gdy program znalazł 7 plików na dokładnie ten sam plik(to samo inode) a dodatkowo jeden na inny, to w wynikach wyszukiwania wyświetlą się jedynie 2 wyniki - jeden z pierwszej grupy i drugi z drugiej.

settings_duplicates_minimal_size_entry_tooltip = 
        Opcja umożliwia ustawienie minimalnej wielkości pliku, której hash będzie zapisywany do pamięci podręcznej.

        Im mniejsza wartość, tym hash większej ilości plików będzie przechowywany w pamięci podręcznej, co przyspieszy wyszukiwanie plików lecz jednocześnie spowolni wczytywanie/zapisywanie hashu do pliku.

settings_duplicates_prehash_checkbutton_tooltip = 
        Umożliwia zapisywanie cząstkowego hashu do pamięci podręcznej, który umożliwia na wcześniejsze wyrzucenie plików z unikalnymi rozmiarami.

        Domyślnie jest zablokowane, ponieważ może powodować spowolnione skanowanie w niektórych sytuacjach.

        Jest mocno polecane osobom które skanują tylko i wyłącznie katalogi zawierające dziesiątki lub setki tysięcy lub nawet miliony plików, ponieważ może to wielokrotnie przyspieszyć proces skanowania.

settings_duplicates_prehash_minimal_entry_tooltip = Minimalny rozmiar pliku, którego cząstowy hash będzie zapisywany do pamięci podręcznej.

settings_duplicates_hide_hard_link_button = Ukrywaj twarde dowiązania(nie działa na Windowsie)
settings_duplicates_prehash_checkbutton = Używaj pamięci podręcznej dla hashy cząstkowych

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

        Jej ręczne modyfikowanie może powodować wyświetlanie niepoprawnych wyników lub jej uszkodzenie spowoduje koniecznośc ponownej generacji, lecz umożliwia też oszczędzenie czasu przy przesuwaniu większej ilości plików.

        Można pliki kopiować pomiędzy komputerami by zaoszczędzić czas na hashowaniu plików(oczywiście tylko gdy dane są przechowywane w identycznej strukturze katalogów na komputerach).

settings_folder_settings_open_tooltip = 
        Otwiera folder gdzie Czkawka przechowuje ustawienia.

        Ich ręczna zmiana, może spowodować różne błędy i kataklizmy, którym fiziologom się nie śniły.

settings_folder_cache_open = Otwórz folder pamięci podręcznej
settings_folder_settings_open = Otwórz folder ustawień

# Compute results
compute_stopped_by_user = Przeszukiwanie zostało zatrzymane przez użytkownika

compute_found = Znaleziono
compute_duplicated_files_in = duplikatów w
compute_groups_which_took = grupach, które zabrały 
compute_groups = grup
compute_duplicates_for = duplikatów dla

compute_empty_folders = pustych folderów
compute_empty_files = pustych plików
compute_biggest_files = największych plików
compute_temporary_files = plików tymczasowych
compute_similar_image = obrazów
compute_similar_videos = plików wideo
compute_music_files = plików muzycznych
compute_symlinks = niepoprawnych linków symbolicznych
compute_broken_files = zepsutych plików

# Progress window
progress_scanned = Przeskanowano
progress_files = pliku
progress_folders = folderu
progress_tags = Zczytywanie tagów z 
progress_hashing = Hashowanie
progress_checking = Sprawdzanie
progress_size = rozmiar
progress_name = nazwa
progress_analyzed_full_hash = Przeanalizowano pełny hash
progress_analyzed_partial_hash = Przeanalizowano częściowy hash 


# Other
searching_for_data = Przeszukiwanie dysku, może to potrwać chwilę, proszę czekać...
