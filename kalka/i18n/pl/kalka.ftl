# Kalka – Polish translations (Fluent format)
# Tłumaczenie polskie

# ── Window & app ──────────────────────────────────────────
main-window-title = Kalka - Czyścioch Danych
about-title = O programie Kalka
about-app-name = Kalka
about-subtitle = Edycja PySide6 / Qt 6
about-version = Wersja 11.0.1
about-description =
    Kalka to prosty, szybki i darmowy program do usuwania
    zbędnych plików z komputera.

    Ten interfejs PySide6/Qt korzysta z backendu czkawka_cli
    do wszystkich operacji skanowania i na plikach.

    Funkcje:
      - Znajdowanie duplikatów plików (po hashu, nazwie lub rozmiarze)
      - Znajdowanie pustych plików i folderów
      - Znajdowanie podobnych obrazów, filmów i muzyki
      - Znajdowanie uszkodzonych plików i nieprawidłowych dowiązań
      - Znajdowanie plików z błędnymi rozszerzeniami lub nazwami
      - Usuwanie metadanych EXIF z obrazów
      - Optymalizacja i przycinanie filmów

    Na licencji MIT
    https://github.com/qarmin/czkawka
about-logo-tooltip = O programie Kalka
version-label = Kalka v11.0.1

# ── Status bar ────────────────────────────────────────────
status-ready = Gotowy
status-tab = Karta: { $tab_name }
status-scanning = Skanowanie: { $tab_name }...
status-scan-complete = Skanowanie zakończone: znaleziono { $count } pozycji
status-scan-stopped = Skanowanie zatrzymane przez użytkownika
status-error = Błąd: { $message }
status-deleted = Usunięto { $count } plik(ów)
status-deleted-dry-run = [PRÓBA] Usunięto { $count } plik(ów)
status-moved = Przeniesiono { $count } plik(ów)
status-moved-dry-run = [PRÓBA] Przeniesiono { $count } plik(ów)
status-copied = Skopiowano { $count } plik(ów)
status-copied-dry-run = [PRÓBA] Skopiowano { $count } plik(ów)
status-hardlinks-created = Utworzono { $count } twardych dowiązań
status-symlinks-created = Utworzono { $count } dowiązań symbolicznych
status-exif-cleaned = Usunięto EXIF z { $count } plik(ów)
status-extensions-fixed = Rozszerzenia naprawione
status-names-fixed = Nazwy naprawione
status-results-saved = Wyniki zapisane pomyślnie
status-results-loaded = Wczytano { $count } pozycji z pliku
status-video-optimize = Optymalizacja wideo: użyj CLI bezpośrednio

# ── Buttons ───────────────────────────────────────────────
scan-button = Skanuj
stop-button = Zatrzymaj
select-button = Wybierz
delete-button = Usuń
move-button = Przenieś
save-button = Zapisz
load-button = Wczytaj
load-button-tooltip = Wczytaj wcześniej zapisane wyniki
sort-button = Sortuj
hardlink-button = Twarde dowiązanie
symlink-button = Dowiązanie sym.
rename-button = Zmień nazwę
clean-exif-button = Wyczyść EXIF
optimize-button = Optymalizuj

# ── Tool names ────────────────────────────────────────────
tool-duplicate-files = Zduplikowane pliki
tool-empty-folders = Puste foldery
tool-big-files = Duże pliki
tool-empty-files = Puste pliki
tool-temporary-files = Pliki tymczasowe
tool-similar-images = Podobne obrazy
tool-similar-videos = Podobne wideo
tool-similar-music = Podobna muzyka
tool-invalid-symlinks = Błędne dowiązania
tool-broken-files = Uszkodzone pliki
tool-bad-extensions = Nieprawidłowe rozszerzenia
tool-bad-names = Błędne nazwy
tool-exif-remover = Usuwanie EXIF
tool-video-optimizer = Optymalizator wideo

# ── Column headers ────────────────────────────────────────
column-selection = Wybór
column-size = Rozmiar
column-file-name = Nazwa pliku
column-path = Ścieżka
column-modification-date = Data modyfikacji
column-hash = Hash
column-similarity = Podobieństwo
column-resolution = Rozdzielczość
column-title = Tytuł
column-artist = Artysta
column-year = Rok
column-bitrate = Przepływność
column-genre = Gatunek
column-length = Długość
column-folder-name = Nazwa folderu
column-symlink-name = Nazwa dowiązania
column-symlink-path = Ścieżka dowiązania
column-destination-path = Ścieżka docelowa
column-type-of-error = Typ błędu
column-error-type = Typ błędu
column-current-extension = Obecne rozszerzenie
column-proper-extension = Właściwe rozszerzenie
column-codec = Kodek

# ── Left panel ────────────────────────────────────────────
settings-tooltip = Ustawienia aplikacji
tool-settings-tooltip = Ustawienia narzędzia
fallback-app-name = Kalka

# ── Settings panel ────────────────────────────────────────
settings-title = Ustawienia
settings-close = Zamknij
settings-tab-general = Ogólne
settings-tab-directories = Katalogi
settings-tab-filters = Filtry
settings-tab-preview = Podgląd
settings-window-title = Ustawienia Kalka

# General settings
settings-cli-path = Ścieżka czkawka_cli:
settings-browse = Przeglądaj
settings-thread-count = Liczba wątków:
settings-thread-auto = Automatycznie (wszystkie rdzenie)
settings-recursive = Wyszukiwanie rekurencyjne
settings-use-cache = Użyj pamięci podręcznej
settings-move-to-trash = Przenieś do kosza zamiast trwałego usunięcia
settings-hide-hard-links = Ukryj twarde dowiązania
settings-save-as-json = Zapisz wyniki jako JSON (zamiast tekstu)
settings-select-cli-binary = Wybierz plik binarny czkawka_cli

# Directories settings
settings-included-dirs = Wybrane katalogi
settings-excluded-dirs = Wykluczone katalogi
settings-add = Dodaj
settings-remove = Usuń
settings-select-dir-include = Wybierz katalog do uwzględnienia
settings-select-dir-exclude = Wybierz katalog do wykluczenia

# Filters settings
settings-excluded-items = Wykluczone elementy:
settings-excluded-items-hint = Wzorce wildcard, rozdzielone przecinkami (np. *.tmp,cache_*)
settings-allowed-extensions = Dozwolone rozszerzenia:
settings-allowed-extensions-hint = np. jpg,png,gif
settings-excluded-extensions = Wykluczone rozszerzenia:
settings-excluded-extensions-hint = np. log,tmp
settings-min-file-size = Minimalny rozmiar pliku:
settings-min-file-size-hint = W bajtach (np. 1024)
settings-max-file-size = Maksymalny rozmiar pliku:
settings-max-file-size-hint = W bajtach (puste = bez limitu)

# Preview settings
settings-show-image-preview = Pokaż podgląd obrazu

# ── Tool settings panel ──────────────────────────────────
tool-settings-title = Ustawienia narzędzia

# Duplicate files
subsettings-check-method = Metoda sprawdzania:
subsettings-hash-type = Typ hasha:
subsettings-case-sensitive = Uwzględniaj wielkość liter

# Similar images
subsettings-hash-size = Rozmiar hasha:
subsettings-resize-algorithm = Algorytm zmiany rozmiaru:
subsettings-image-hash-type = Typ hasha:
subsettings-ignore-same-size = Ignoruj ten sam rozmiar
subsettings-max-difference = Maksymalna różnica:

# Similar videos
subsettings-crop-detect = Wykrywanie przycinania:
subsettings-skip-forward = Pomiń początek (s):
subsettings-hash-duration = Czas haszowania (s):

# Similar music
subsettings-audio-check-type = Typ sprawdzania audio:
subsettings-tag-matching = Dopasowanie tagów
subsettings-approximate-comparison = Przybliżone porównywanie
subsettings-fingerprint-matching = Dopasowanie odcisków
subsettings-compare-similar-titles = Porównuj z podobnymi tytułami

# Big files
subsettings-method = Metoda:
subsettings-the-biggest = Największe
subsettings-the-smallest = Najmniejsze
subsettings-number-of-files = Liczba plików:

# Broken files
subsettings-file-types = Typy plików do sprawdzenia:

# Bad names
subsettings-check-for = Sprawdź:
subsettings-uppercase-ext = Wielkie litery w rozszerzeniu
subsettings-uppercase-ext-hint = Pliki z .JPG, .PNG itp.
subsettings-emoji = Emoji w nazwie
subsettings-emoji-hint = Pliki zawierające znaki emoji
subsettings-space = Spacja na początku/końcu
subsettings-space-hint = Spacje na początku lub końcu nazwy
subsettings-non-ascii = Znaki spoza ASCII
subsettings-non-ascii-hint = Znaki spoza zakresu ASCII
subsettings-remove-duplicated = Powtórzone znaki niealfanumeryczne
subsettings-remove-duplicated-hint = np. plik--nazwa..txt
subsettings-restricted-charset = Ograniczony zestaw znaków:
subsettings-restricted-charset-hint = Dozwolone znaki specjalne, oddzielone przecinkami

# EXIF
subsettings-ignored-exif-tags = Ignorowane tagi EXIF:
subsettings-ignored-exif-tags-hint = Tagi do pominięcia, oddzielone przecinkami

# Video optimizer
subsettings-mode = Tryb:
subsettings-crop-settings = Ustawienia przycinania
subsettings-crop-type = Typ przycinania:
subsettings-black-pixel-threshold = Próg czarnego piksela:
subsettings-black-bar-min-pct = Minimalny % czarnego paska:
subsettings-max-samples = Maksymalna liczba próbek:
subsettings-min-crop-size = Minimalny rozmiar przycięcia:
subsettings-transcode-settings = Ustawienia transkodowania
subsettings-excluded-codecs = Wykluczone kodeki:
subsettings-target-codec = Docelowy kodek:
subsettings-quality = Jakość:
subsettings-fail-if-bigger = Niepowodzenie jeśli większy

# ── Progress widget ──────────────────────────────────────
progress-initializing = Inicjalizacja...
progress-starting = Rozpoczynanie skanowania...
progress-current = Bieżący
progress-overall = Ogólny
progress-scan-complete = Skanowanie zakończone
progress-completed-in = Ukończono w { $time }
progress-done = gotowe

# ── Results view ─────────────────────────────────────────
results-no-results = Brak wyników
results-found-grouped = Znaleziono { $total } plików ({ $size }) w { $groups } grupach
results-found-flat = Znaleziono { $total } pozycji ({ $size })
results-selected = Wybrano: { $selected }/{ $total } ({ $selected_size }/{ $total_size })

# ── Preview panel ────────────────────────────────────────
preview-title = Podgląd
preview-no-preview = Brak podglądu
preview-file-not-found = Nie znaleziono pliku
preview-not-available = Podgląd niedostępny
    dla tego typu pliku
preview-cannot-load = Nie można załadować obrazu

# ── Context menu ─────────────────────────────────────────
context-open-file = Otwórz plik
context-open-folder = Otwórz folder
context-select = Zaznacz
context-deselect = Odznacz

# ── Bottom panel ─────────────────────────────────────────
bottom-included-dirs = Wybrane katalogi:
bottom-excluded-dirs = Wykluczone katalogi:

# ── Delete dialog ────────────────────────────────────────
delete-dialog-title = Usuń pliki
delete-dialog-message = Czy na pewno chcesz usunąć { $count } wybranych plików?
delete-dialog-trash = Przenieś do kosza zamiast trwałego usunięcia
delete-dialog-dry-run = Próba (tylko podgląd, pliki nie zostaną usunięte)
delete-dialog-confirm = Usuń

# ── Move dialog ──────────────────────────────────────────
move-dialog-title = Przenieś/Kopiuj pliki
move-dialog-message = Przenieś lub skopiuj { $count } wybranych plików do:
move-dialog-placeholder = Wybierz folder docelowy...
move-dialog-preserve = Zachowaj strukturę folderów
move-dialog-copy-mode = Kopiuj zamiast przenosić
move-dialog-dry-run = Próba (tylko podgląd, pliki nie zostaną przeniesione)
move-dialog-confirm = Przenieś
move-dialog-select-dest = Wybierz katalog docelowy

# ── Rename dialog ────────────────────────────────────────
rename-dialog-confirm = Zmień nazwę
rename-dialog-ext-message = Naprawić rozszerzenia { $count } wybranych plików?

    Pliki zostaną przemianowane na właściwe rozszerzenia.
rename-dialog-names-message = Naprawić nazwy { $count } wybranych plików?

    Pliki z problematycznymi nazwami zostaną przemianowane.

# ── Select dialog ────────────────────────────────────────
select-dialog-title = Wybór wyników
select-dialog-prompt = Wybierz tryb zaznaczania:
select-all = Zaznacz wszystko
unselect-all = Odznacz wszystko
invert-selection = Odwróć zaznaczenie
select-biggest-size = Wybierz największe (wg rozmiaru)
select-smallest-size = Wybierz najmniejsze (wg rozmiaru)
select-newest = Wybierz najnowsze
select-oldest = Wybierz najstarsze
select-shortest-path = Wybierz najkrótszą ścieżkę
select-longest-path = Wybierz najdłuższą ścieżkę
cancel = Anuluj

# ── Sort dialog ──────────────────────────────────────────
sort-dialog-title = Sortuj wyniki
sort-by = Sortuj wg:
sort-ascending = Rosnąco

# ── Hardlink / symlink dialogs ───────────────────────────
hardlink-dialog-title = Utwórz twarde dowiązania
hardlink-dialog-message = Zastąpić { $count } wybranych plików twardymi dowiązaniami do:
    { $reference }?
symlink-dialog-title = Utwórz dowiązania symboliczne
symlink-dialog-message = Zastąpić { $count } wybranych plików dowiązaniami symbolicznymi do:
    { $reference }?
no-reference-title = Brak referencji
no-reference-message = Nie można określić pliku referencyjnego. Pozostaw co najmniej jeden plik odznaczony w grupie.

# ── EXIF dialog ──────────────────────────────────────────
exif-dialog-title = Wyczyść EXIF
exif-dialog-message = Usunąć metadane EXIF z { $count } wybranych plików?

# ── Video optimize dialog ────────────────────────────────
video-optimize-title = Optymalizacja wideo
video-optimize-message = Optymalizacja wideo dla { $count } plików zostanie wykonana za pomocą czkawka_cli. Sprawdź pasek stanu.

# ── Warnings / errors ────────────────────────────────────
no-directories-title = Brak katalogów
no-directories-message = Dodaj co najmniej jeden katalog do skanowania w dolnym panelu.
no-selection-title = Brak wyboru
no-selection-delete = Nie wybrano plików do usunięcia.
no-selection-move = Nie wybrano plików.
no-results-title = Brak wyników
no-results-save = Brak wyników do zapisania.
no-destination-title = Brak celu
no-destination-message = Wybierz folder docelowy.
scan-error-title = Błąd skanowania

# ── Save/load dialogs ────────────────────────────────────
save-dialog-title = Zapisz wyniki
load-dialog-title = Wczytaj wyniki
