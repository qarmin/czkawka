# Cedinia – Polski (Polish)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplikaty
tool_empty_folders = Puste foldery
tool_similar_images = Podobne obrazy
tool_empty_files = Puste pliki
tool_temporary_files = Pliki tymczasowe
tool_big_files = Największe pliki
tool_broken_files = Uszkodzone pliki
tool_bad_extensions = Złe rozszerzenia
tool_same_music = Duplikaty muzyki
tool_bad_names = Złe nazwy
tool_exif_remover = Dane EXIF
tool_directories = Katalogi
tool_settings = Ustawienia

# Home screen tool card descriptions
home_dup_description = Znajdź pliki o tej samej zawartości
home_empty_folders_description = Katalogi bez zawartości
home_similar_images_description = Znajdź wizualnie podobne zdjęcia
home_empty_files_description = Pliki o zerowym rozmiarze
home_temp_files_description = Tymczasowe i cache'owane pliki
home_big_files_description = 50 największych plików na dysku
home_broken_files_description = Pliki PDF, audio, obrazy, archiwa
home_bad_extensions_description = Pliki z nieprawidłowym rozszerzeniem
home_same_music_description = Podobne pliki audio wg tagów
home_bad_names_description = Pliki z problematycznymi znakami w nazwie
home_exif_description = Obrazy z metadanymi EXIF

# Results list
scanning = Skanowanie w toku...
stopping = Zatrzymywanie...
no_results = Brak wynikow
press_start = Nacisnij START aby skanowac
select_label = Zaz.
deselect_label = Odzn.
list_label = Lista
gallery_label = Gal.

# Selection popup
selection_popup_title = Zaznaczanie
select_all = Zaznacz wszystko
select_except_one = Zaznacz poza jednym
select_except_largest = Zaznacz poza największym
select_except_smallest = Zaznacz poza najmniejszym
select_largest = Zaznacz największy
select_smallest = Zaznacz najmniejszy
select_except_highest_res = Zaznacz poza największą rozdzielczością
select_except_lowest_res = Zaznacz poza najmniejszą rozdzielczością
select_highest_res = Zaznacz największą rozdzielczość
select_lowest_res = Zaznacz najmniejszą rozdzielczość
invert_selection = Odwroc zaznaczenie
close = Zamknij

# Deselection popup
deselection_popup_title = Odznaczanie
deselect_all = Odznacz wszystko
deselect_except_one = Odznacz poza jednym

# Confirm popup
cancel = Anuluj
delete = Usuń
rename = Zmień nazwę

# Delete errors popup
delete_errors_title = Nie udalo sie usunac niektorych plikow:
ok = OK

# Stopping overlay
stopping_overlay_title = ■ Zatrzymywanie
stopping_overlay_body = Kończenie bieżącego skanu…\nProszę czekać.

# Permission popup
permission_title = 🔒 Dostęp do plików
permission_body = Aby skanować pliki, aplikacja potrzebuje dostępu do pamięci urządzenia. Bez tego uprawnienia skanowanie nie będzie możliwe.
grant = Przyznaj
no_permission_scan_warning = Brak uprawnień do plików – przyznaj dostęp aby skanować

# Settings screen tabs
settings_tab_general = Ogólne
settings_tab_tools = Narzędzia
settings_tab_diagnostics = Info

# Settings — General tab
settings_use_cache = Użyj pamięci podręcznej
settings_use_cache_desc = Przyspiesza kolejne skany (hash/obrazy)
settings_ignore_hidden = Ignoruj ukryte pliki
settings_ignore_hidden_desc = Pliki i foldery zaczynające się od '.'
settings_show_notification = Powiadom po zakończeniu skanu
settings_show_notification_desc = Pokaż systemowe powiadomienie po zakończeniu skanu
settings_notify_only_background = Tylko gdy w tle
settings_notify_only_background_desc = Pomiń powiadomienie jeśli aplikacja jest widoczna
notifications_disabled_banner = 🔔 Powiadomienia wyłączone
notifications_enable_button = Włącz
settings_scan_label = SKANOWANIE
settings_filters_label = FILTRY (niektóre narzędzia)
settings_min_file_size = Min. rozmiar pliku
settings_max_file_size = Maks. rozmiar pliku
settings_language = Język
settings_language_restart = Wymaga restartu aplikacji
settings_common_label = USTAWIENIA OGÓLNE
settings_excluded_items = WYKLUCZONE ELEMENTY (wzorce glob, oddzielone przecinkiem)
settings_excluded_items_placeholder = np. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = DOZWOLONE ROZSZERZENIA (puste = wszystkie)
settings_allowed_extensions_placeholder = np. jpg, png, mp4
settings_excluded_extensions = WYKLUCZONE ROZSZERZENIA
settings_excluded_extensions_placeholder = np. bak, tmp, log

# Settings — Tools section labels
settings_duplicates_header = DUPLIKATY
settings_check_method_label = METODA PORÓWNANIA
settings_check_method = Metoda
settings_hash_type_label = TYP HASHA
settings_hash_type = Typ hasha
settings_hash_type_desc = Blake3 – najszybszy; CRC32/xxH3 – alternatywy
settings_similar_images_header = PODOBNE OBRAZY
settings_similarity_preset = Próg podobieństwa
settings_similarity_desc = Bardzo wysoka = tylko prawie identyczne
settings_hash_size = Rozmiar hasha
settings_hash_size_desc = Większy = dokładniejszy, wolniejszy
settings_hash_alg = Algorytm hasha
settings_image_filter = Filtr zmiany rozmiaru
settings_ignore_same_size = Ignoruj obrazy o tych samych wymiarach
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
settings_music_bitrate = Przepływność
settings_music_approx = Przybliżone porównywanie tagów
settings_broken_files_header = USZKODZONE PLIKI
settings_broken_files_types_label = SPRAWDZANE TYPY
settings_broken_audio = Dźwięk
settings_broken_pdf = PDF
settings_broken_archive = Archiwum
settings_broken_image = Obraz
settings_bad_names_header = ZŁE NAZWY
settings_bad_names_checks_label = SPRAWDZENIA
settings_bad_names_uppercase_ext = Wielkie litery w rozszerzeniu
settings_bad_names_emoji = Emoji w nazwie
settings_bad_names_space = Spacje na początku/końcu
settings_bad_names_non_ascii = Znaki spoza ASCII
settings_bad_names_duplicated = Powtarzające się znaki

# Settings — Diagnostics tab
diagnostics_header = DIAGNOSTYKA
diagnostics_thumbnails = Pamięć miniatur
diagnostics_app_cache = Pamięć aplikacji
diagnostics_refresh = Odśwież
diagnostics_clear_thumbnails = Wyczyść miniatury
diagnostics_clear_cache = Wyczyść cache
diagnostics_collect_test = Test skanowania
diagnostics_collect_test_desc = Skanuje każdy wolumin rekurencyjnie
diagnostics_collect_test_run = Uruchom
diagnostics_collect_test_stop = Stop
collect_test_cancelled = ⛔ Zatrzymano przez użytkownika
diag_confirm_clear_thumbnails = Wyczyścić cache miniatur?
diag_confirm_clear_cache = Wyczyścić cache aplikacji?
about_repo = Repozytorium
about_translate = Tłumaczenia
about_donate = Wesprzyj

# Collect-test result popup
collect_test_title = 📊 Wyniki testu
collect_test_volumes = 💾 Woluminy:
collect_test_folders = 📁 Foldery:
collect_test_files = 📄 Pliki:
collect_test_time = ⏱ Czas:
collect_test_ms = " ms"

# Directories screen
directories_include_header = Katalogi do skanowania
directories_exclude_header = Katalogi wykluczone
directories_add = + Dodaj
no_paths = Brak ścieżek – dodaj poniżej
directories_volume_header = Woluminy
directories_volume_refresh = Odśwież
directories_volume_add = Dodaj

# Bottom navigation
nav_home = Start
nav_dirs = Katalogi
nav_settings = Ustawienia

# Status messages set from Rust
status_ready = Gotowy / Ready
status_stopped = Zatrzymano
status_no_results = Brak wyników
status_deleted_selected = Usunięto zaznaczone
status_deleted_with_errors = Usunięto z błędami
scan_not_started = Nie uruchomiono skanu
found_items_prefix = Znaleziono
found_items_suffix = elementów
deleted_items_prefix = Usunięto
deleted_items_suffix = elementów
deleted_errors_suffix = błędów
renamed_prefix = Zmieniono nazwy
renamed_files_suffix = plików
renamed_errors_suffix = błędów
cleaned_exif_prefix = Wyczyszczono EXIF z
cleaned_exif_suffix = plików
cleaned_exif_errors_suffix = błędów
and_more_prefix = …i
and_more_suffix = więcej

# Gallery / delete popups
gallery_delete_button = Usuń
gallery_back = Wstecz
gallery_confirm_delete = Tak, usuń
deleting_files = Usuwanie plików…
stop = Stop
files_suffix = plików
scanning_fallback = Skanowanie…
app_subtitle = Ku czci Bitwy pod Cedynią (972 r.)
app_license = Frontend dla Czkawka Core  •  GPL-3.0
about_app_label = O APLIKACJI
cache_label = PAMIĘĆ PODRĘCZNA

# Scan stage labels (shown during scan progress)
stage_collecting_files = Zbieranie plików
stage_scanning_name = Skanowanie po nazwie
stage_scanning_size_name = Skanowanie po nazwie i rozmiarze
stage_scanning_size = Skanowanie po rozmiarze
stage_pre_hash = Pre-hash
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
stage_calculating_music_fingerprints = Obliczanie odcisków muzycznych
stage_comparing_fingerprints = Porównywanie odcisków muzycznych
stage_extracting_exif = Odczyt tagów EXIF
stage_creating_video_thumbnails = Tworzenie miniatur wideo
stage_processing_videos = Przetwarzanie wideo
stage_deleting = Usuwanie plików
stage_renaming = Zmiana nazw plików
stage_moving = Przenoszenie plików
stage_hardlinking = Tworzenie hardlinków
stage_symlinking = Tworzenie dowiązań
stage_optimizing_videos = Optymalizacja wideo
stage_cleaning_exif = Czyszczenie EXIF

