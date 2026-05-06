# Cedinia - Polish

app_name = Cedinia
tool_duplicate_files = Duplikaty
tool_empty_folders = Puste foldery
tool_similar_images = Podobne obrazy
tool_empty_files = Puste pliki
tool_temporary_files = Pliki tymczasowe
tool_big_files = Największe pliki
tool_broken_files = Uszkodzone pliki
tool_bad_extensions = Nieprawidłowe rozszerzenia
tool_same_music = Duplikaty muzyki
tool_bad_names = Niepoprawne nazwy
tool_exif_remover = Dane EXIF
tool_directories = Katalogi
tool_settings = Ustawienia

home_dup_description = Znajdź pliki o tej samej zawartości
home_empty_folders_description = Katalogi bez zawartości
home_similar_images_description = Znajdź wizualnie podobne zdjęcia
home_empty_files_description = Pliki o zerowym rozmiarze
home_temp_files_description = Pliki tymczasowe i cache
home_big_files_description = Największe i najmniejsze pliki na dysku
home_broken_files_description = PDF, audio, obrazy, archiwa
home_bad_extensions_description = Pliki z nieprawidłowym rozszerzeniem
home_same_music_description = Podobne pliki audio według tagów
home_bad_names_description = Pliki z problematycznymi znakami w nazwie
home_exif_description = Obrazy z metadanymi EXIF

scanning = Trwa skanowanie...
stopping = Zatrzymywanie...
no_results = Brak wyników
press_start = Naciśnij START, aby rozpocząć skanowanie
select_label = Zazn.
deselect_label = Odzn.
list_label = Lista
gallery_label = Galeria

selection_popup_title = Wybór
select_all = Zaznacz wszystko
select_except_one = Zaznacz wszystko oprócz jednego
select_except_largest = Zaznacz wszystko oprócz największego
select_except_smallest = Zaznacz wszystko oprócz najmniejszego
select_largest = Zaznacz największy
select_smallest = Zaznacz najmniejszy
select_except_highest_res = Zaznacz wszystko oprócz najwyższej rozdzielczości
select_except_lowest_res = Zaznacz wszystko oprócz najniższej rozdzielczości
select_highest_res = Zaznacz najwyższą rozdzielczość
select_lowest_res = Zaznacz najniższą rozdzielczość
invert_selection = Odwróć zaznaczenie
close = Zamknij

deselection_popup_title = Odznaczanie
deselect_all = Odznacz wszystko
deselect_except_one = Odznacz wszystko oprócz jednego

cancel = Anuluj
delete = Usuń
rename = Zmień nazwę

delete_errors_title = Nie udało się usunąć niektórych plików:
ok = OK

stopping_overlay_title = Zatrzymywanie
stopping_overlay_body =
    Kończenie bieżącego skanowania...
        Proszę czekać.

permission_title = Dostęp do plików
permission_body = Aby skanować pliki, aplikacja potrzebuje dostępu do pamięci urządzenia. Bez tego skanowanie nie będzie możliwe.
grant = Przyznaj
no_permission_scan_warning = Brak dostępu do plików - przyznaj uprawnienia

settings_tab_general = Ogólne
settings_tab_tools = Narzędzia
settings_tab_diagnostics = Informacje

settings_use_cache = Użyj cache
settings_use_cache_desc = Przyspiesza kolejne skanowania (hashy i obrazów)
settings_ignore_hidden = Ignoruj ukryte pliki
settings_ignore_hidden_desc = Pliki i foldery zaczynające się od '.'
settings_show_notification = Powiadom po zakończeniu skanowania
settings_show_notification_desc = Wyświetl powiadomienie systemowe po zakończeniu
settings_notify_only_background = Tylko w tle
settings_notify_only_background_desc = Pomiń powiadomienie, jeśli aplikacja jest widoczna
notifications_disabled_banner = Powiadomienia wyłączone
notifications_enable_button = Włącz

settings_scan_label = SKANOWANIE
settings_filters_label = FILTRY (wybrane narzędzia)
settings_min_file_size = Minimalny rozmiar pliku
settings_max_file_size = Maksymalny rozmiar pliku
settings_language = Język
settings_language_restart = Wymaga ponownego uruchomienia
settings_common_label = USTAWIENIA OGÓLNE

settings_excluded_items = WYKLUCZONE POZYCJE (wzorce glob, oddzielone przecinkami)
settings_excluded_items_placeholder = np. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = DOZWOLONE ROZSZERZENIA (puste = wszystkie)
settings_allowed_extensions_placeholder = np. jpg, png, mp4
settings_excluded_extensions = WYKLUCZONE ROZSZERZENIA
settings_excluded_extensions_placeholder = np. bak, tmp, log

settings_duplicates_header = DUPLIKATY
settings_check_method_label = METODA PORÓWNANIA
settings_check_method = Metoda
settings_hash_type_label = TYP HASHU
settings_hash_type = Typ hashu
settings_hash_type_desc = Blake3 - zalecany, CRC32 ma niewielkie ryzyko błędów

settings_similar_images_header = PODOBNE OBRAZY
settings_similarity_preset = Próg podobieństwa
settings_similarity_desc = Bardzo wysoki = tylko niemal identyczne
settings_hash_size = Rozmiar hashu
settings_hash_size_desc = Większy = mniej błędów, ale mniej wyników
settings_hash_alg = Algorytm haszowania
settings_image_filter = Filtr skalowania
settings_ignore_same_size = Ignoruj obrazy o tych samych wymiarach

settings_gallery_image_fit_cover = Galeria: przycinanie do kwadratu
settings_gallery_image_fit_cover_desc = Wypełnia kafelek; wyłącz, aby zachować proporcje

settings_big_files_header = NAJWIĘKSZE PLIKI
settings_search_mode = Tryb wyszukiwania
settings_file_count = Liczba plików

settings_same_music_header = DUPLIKATY MUZYKI
settings_music_check_method = Tryb porównania
settings_music_compare_tags_label = PORÓWNYWANE TAGI
settings_music_title = Tytuł
settings_music_artist = Artysta
settings_music_year = Rok
settings_music_length = Długość
settings_music_genre = Gatunek
settings_music_bitrate = Bitrate
settings_music_approx = Przybliżone porównanie tagów

settings_broken_files_header = USZKODZONE PLIKI
settings_broken_files_note = Wymaga dużo zasobów. Zalecane użycie na komputerze.
settings_broken_files_types_label = SPRAWDZANE TYPY
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Archiwum
settings_broken_image = Obraz

settings_bad_names_header = NIEPOPRAWNE NAZWY
settings_bad_names_checks_label = SPRAWDZANE REGUŁY
settings_bad_names_uppercase_ext = Wielkie litery w rozszerzeniu
settings_bad_names_emoji = Emoji w nazwie
settings_bad_names_space = Spacje na początku/końcu
settings_bad_names_non_ascii = Znaki spoza ASCII
settings_bad_names_duplicated = Powtarzające się znaki

diagnostics_header = DIAGNOSTYKA
diagnostics_thumbnails = Cache miniatur
diagnostics_app_cache = Cache aplikacji
diagnostics_refresh = Odśwież
diagnostics_clear_thumbnails = Wyczyść miniatury
diagnostics_open_thumbnails_folder = Otwórz folder
diagnostics_clear_cache = Wyczyść cache
diagnostics_open_cache_folder = Otwórz folder
diagnostics_collect_test = Test dostępu do plików
diagnostics_collect_test_desc = Sprawdź liczbę dostępnych plików
diagnostics_collect_test_run = Uruchom
diagnostics_collect_test_stop = Zatrzymaj

collect_test_cancelled = Zatrzymano przez użytkownika
diag_confirm_clear_thumbnails = Wyczyścić cache miniatur?
diag_confirm_clear_cache = Wyczyścić cache aplikacji?

about_repo = Repozytorium
about_translate = Tłumaczenia
about_donate = Wsparcie

collect_test_title = Wyniki testu
collect_test_volumes = Wolumeny:
collect_test_folders = Foldery:
collect_test_files = Pliki:
collect_test_time = Czas:

licenses_label = LICENCJA
third_party_licenses = Licencje zewnętrzne
licenses_popup_title = Licencje zewnętrzne

directories_include_header = Dołączone
directories_included = Dołączone
directories_exclude_header = Wykluczone
directories_excluded_header = Wykluczone
directories_add = Dodaj
no_paths = Brak ścieżek - dodaj poniżej

directories_volume_header = Wolumeny
directories_volume_refresh = Odśwież
directories_volume_add = Dodaj

nav_home = Start
nav_dirs = Katalogi
nav_settings = Ustawienia

status_ready = Gotowy
status_stopped = Zatrzymano
status_no_results = Brak wyników
status_deleted_selected = Usunięto wybrane
status_deleted_with_errors = Usunięto z błędami

scan_not_started = Skanowanie nie rozpoczęte
found_items_prefix = Znaleziono
found_items_suffix = elementów
deleted_items_prefix = Usunięto
deleted_items_suffix = elementów
deleted_errors_suffix = błędów

renamed_prefix = Zmieniono nazwę
renamed_files_suffix = plików
renamed_errors_suffix = błędów

cleaned_exif_prefix = Usunięto EXIF z
cleaned_exif_suffix = plików
cleaned_exif_errors_suffix = błędów

and_more_prefix = ...oraz
and_more_suffix = więcej

# Gallery / delete popups
gallery_delete_button = Usuń
gallery_back = Powrót
gallery_confirm_delete = Tak, usuń
deleting_files = Usuwanie plików...
stop = Zatrzymaj
files_suffix = plików
scanning_fallback = Skanowanie...
app_subtitle = Na cześć Bitwy pod Cedynią (972)
app_license = Frontend dla Czkawka Core - GPL-3.0
about_app_label = O aplikacji
cache_label = PAMIĘĆ PODRĘCZNA

# Notification
scan_completed_notification = Skanowanie zakończone - znaleziono { $file_count } elementów

# Confirm popups (set from Rust)
confirm_clean_exif = Czy na pewno chcesz usunąć dane EXIF z { $n } wybranych plików?
confirm_delete_items = Czy na pewno chcesz usunąć { $n } wybranych elementów?
gallery_confirm_delete_msg = Za chwilę usuniesz { $total_images } obrazów w { $total_groups } grupach.
gallery_confirm_delete_warning = Wszystkie elementy są zaznaczone w { $unsafe_groups } grupach!

# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Obliczanie i porównywanie odcisków audio jest bardzo zasobożerne i może trwać długo. Zalecane jest użycie wersji desktopowej.

# Scan stage labels
stage_collecting_files = Zbieranie plików
stage_scanning_name = Skanowanie według nazwy
stage_scanning_size_name = Skanowanie według nazwy i rozmiaru
stage_scanning_size = Skanowanie według rozmiaru
stage_pre_hash = Wstępne haszowanie
stage_full_hash = Haszowanie
stage_loading_cache = Ładowanie cache
stage_saving_cache = Zapisywanie cache
stage_calculating_image_hashes = Obliczanie hashy obrazów
stage_comparing_images = Porównywanie obrazów
stage_calculating_video_hashes = Obliczanie hashy wideo
stage_checking_files = Sprawdzanie plików
stage_checking_extensions = Sprawdzanie rozszerzeń
stage_checking_names = Sprawdzanie nazw
stage_reading_music_tags = Odczyt tagów muzycznych
stage_comparing_tags = Porównywanie tagów
stage_calculating_music_fingerprints = Obliczanie odcisków audio
stage_comparing_fingerprints = Porównywanie odcisków audio
stage_extracting_exif = Odczyt danych EXIF
stage_creating_video_thumbnails = Tworzenie miniatur wideo
stage_processing_videos = Przetwarzanie wideo
stage_deleting = Usuwanie plików
stage_renaming = Zmiana nazw plików
stage_moving = Przenoszenie plików
stage_hardlinking = Tworzenie twardych linków
stage_symlinking = Tworzenie dowiązań symbolicznych
stage_optimizing_videos = Optymalizacja wideo
stage_cleaning_exif = Usuwanie danych EXIF

# Group headers
duplicates_group_header = { $count } plików x { $per_file } / plik = { $total } łącznie
similar_images_group_header = { $count } podobnych obrazów
same_music_group_header = { $count } podobnych utworów

# Rename confirmation
confirm_rename_items = Czy na pewno chcesz zmienić nazwę { $n } wybranych plików?

# Combo-box options
option_search_mode_biggest = Największe
option_search_mode_smallest = Najmniejsze
option_similarity_very_high = Bardzo wysoki
option_similarity_high = Wysoki
option_similarity_medium = Średni
option_similarity_low = Niski
option_similarity_very_low = Bardzo niski
option_similarity_minimal = Minimalny
option_check_method_hash = Hash
option_check_method_name = Nazwa
option_check_method_size_and_name = Rozmiar + nazwa
option_check_method_size = Rozmiar
option_music_method_tags = Tagi
option_music_method_audio = Audio
option_min_size_none = Brak
option_max_size_unlimited = Bez limitu

# Volumes
volume_internal_storage = Pamięć wewnętrzna
volume_sd_card = Karta SD
volume_storage = Wolumen pamięci

# Directories
directories_referenced_tooltip = Referencyjne (nieusuwane)
directories_include_section_header = DOŁĄCZONE
directories_exclude_section_header = WYKLUCZONE
directories_custom_paths = Niestandardowe
directories_check_button = Analizuj
directories_check_popup_title = Statystyki katalogów
directories_check_label_included = Dołączone ścieżki:
directories_check_label_excluded = Wykluczone ścieżki:
directories_check_label_referenced = Ścieżki referencyjne:
directories_check_label_would_scan = Pliki do przeskanowania:
directories_check_label_processable = Pliki możliwe do przetworzenia:
directories_check_scanning = Skanowanie...
directories_check_warning_no_processable = Brak plików do przetworzenia - sprawdź ustawienia

path_edit_title_include = Dodaj do dołączonych
path_edit_title_exclude = Dodaj do wykluczonych
path_edit_placeholder = Wprowadź ścieżkę...
path_edit_not_exists = Ścieżka nie istnieje
path_edit_is_dir = Katalog
path_edit_is_file = Plik
path_edit_no_newlines = Ścieżki nie mogą zawierać nowych linii - Enter jest niedozwolony

ctx_menu_title = Otwórz
ctx_open_file = Otwórz element
ctx_open_folder = Otwórz folder nadrzędny