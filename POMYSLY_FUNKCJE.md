# Czkawka — pomysły na nowe funkcje i elementy GUI

Przegląd funkcjonalny wszystkich frontendów (krokiet, cedinia, czkawka_gui, czkawka_cli)
oraz nieużytych możliwości silnika `czkawka_core`. Skupione **wyłącznie na funkcjonalności
i interfejsie** — nie na błędach/kodzie.

Legenda rozmiaru: 🟢 drobne · 🟡 średnie · 🔴 duże

---

## 0. Wspólne dla wszystkich frontendów (najwyższy priorytet)

Te pomysły powtarzały się w kilku frontendach jednocześnie — warto je rozważyć centralnie:

- 🟡 **Profile / presety skanowania** — zapisywanie i wczytywanie całych konfiguracji narzędzia
  (np. „Deep Clean", „Biblioteka muzyki", „Zdjęcia", presety czułości low/medium/high).
- 🟡 **Pasek wyszukiwania w wynikach** — filtrowanie już załadowanych wyników po nazwie/ścieżce/rozmiarze
  bez ponownego skanu.
- 🔴 **Undo/Redo akcji** — cofanie ostatnich operacji (delete/move/rename), kosz tymczasowy z możliwością restore.
- 🔴 **Przenieś / kopiuj zamiast usuwać** — pełnoprawna akcja „przenieś zaznaczone do katalogu X"
  (krokiet ma move folders, cedinia tego nie ma w pełni).
- 🟡 **Tryb dry-run / podgląd akcji** — pokaż dokładnie co zostanie zrobione zanim cokolwiek się stanie.
- 🟡 **Historia skanów** — ekran z poprzednimi skanami (data, narzędzie, liczba wyników, odzyskane miejsce).
- 🟡 **Eksport wyników** — CSV / JSON / TXT / shell-script (`rm`/`mv`/`ln`) do wykonania poza aplikacją.
- 🟡 **Inteligentne auto-zaznaczanie** — reguły „zachowaj najnowszy / najdłuższą ścieżkę / najwyższą rozdzielczość".
- 🔴 **Planowanie skanów** — okresowe skany w tle (cron/systemd na desktopie, WorkManager na Androidzie).
- 🟡 **Statystyki po skanie** — czas trwania, liczba przeskanowanych/znalezionych plików, ile miejsca do odzyskania.

---

## 1. Krokiet (desktop, Slint)

### Akcje na wynikach
- 🟡 **Zamiana kopii na hardlink** — dla duplikatów zamień zaznaczone kopie na hardlinki do oryginału.
- 🟡 **Batch rename wg szablonu** — `{date}_{name}_{index}`, zamiana spacji, renumeracja.
- 🟡 **Zarchiwizuj zaznaczone** — spakuj wybrane pliki do .zip/.tar.gz.
- 🟢 **Otwórz folder nadrzędny** — szybkie otwarcie katalogu rodzica w menedżerze plików.
- 🟡 **Dodaj do playlisty (M3U/PLS)** — dla podobnej muzyki.

### Filtry / zaznaczanie
- 🟡 **Regex w custom-select** — obok wildcards.
- 🔴 **Warunki AND/OR w custom-select** — łączenie reguł logicznych.
- 🟡 **Filtr po tagu EXIF** — aparat, data, ISO, rozdzielczość.
- 🟢 **Filtr „zmodyfikowane ostatnio"** — dzień/tydzień/miesiąc.
- 🟢 **Zaznacz po rozszerzeniu** w bieżącym widoku; **odwróć zaznaczenie w grupie**.

### Podgląd / preview
- 🔴 **Split view side-by-side** dla duplikatów/obrazów w widoku głównym.
- 🟡 **Metadane w panelu preview** — EXIF, ID3, właściwości pliku.
- 🟡 **Slideshow** dla podobnych obrazów (z regulacją prędkości).
- 🔴 **Video scrubber** — klikalna oś czasu zamiast tylko keyframe'ów.
- 🟡 **Hex viewer** dla broken/binarnych plików.

### Workflow / UX
- 🟡 **Drag & drop folderów** do list included/excluded.
- 🔴 **Multi-tab skanów** — kilka skanów równolegle do porównania.
- 🟡 **Status bar** — liczba zaznaczonych, łączny rozmiar, czas ostatniego skanu.
- 🟡 **Tree view duplikatów** — zwijane/rozwijane grupy zamiast płaskiej listy.
- 🟡 **Konfigurowalne skróty klawiszowe** (Delete/Trash/Move/Select All).
- 🟢 **Tooltipy na przyciskach**; 🟡 **resizable + zapamiętywane szerokości kolumn**.
- 🟡 **Kontrola liczby wątków / limit CPU** podczas skanu.

---

## 2. Cedinia (Android / dotyk, Slint)

### Gesty dotykowe
- 🟡 **Swipe w lewo = usuń** (z undo); 🟡 **swipe w prawo = ustaw jako referencję**.
- 🟡 **Double-tap na nagłówku grupy** = zaznacz/odznacz całą grupę.
- 🟢 **Long-press na tle** = odznacz wszystko.
- 🟡 **Pull-to-refresh** = ponowny skan.

### Galeria / podgląd
- 🟡 **Fullscreen lightbox** — tap → pełny ekran, swipe między obrazami, pinch-to-zoom.
- 🟡 **Suwak side-by-side** w trybie porównania (X% lewy / Y% prawy obraz).
- 🟡 **Histogram + metadane** obrazu w trybie compare.
- 🔴 **Zapamiętywanie decyzji compare** („zawsze keep A") by pomijać powtarzalne grupy.

### Android-natywne
- 🔴 **Integracja SAF** — dostęp do karty SD i chmury (Drive/OneDrive).
- 🟡 **Folder picker per narzędzie** — wybór jednego folderu zamiast całego `/storage`.
- 🔴 **Widget na ekran główny** — ostatnie wyniki + szybki start.
- 🟡 **App shortcuts** (long-press ikony) — „Skanuj duplikaty", „Ostatnie wyniki".
- 🔴 **Background scan + battery optimization** — skan w tle, wznowienie po reboocie.
- 🟢 **Material You** — kolory z tapety (Android 12+); auto dark mode.

### Workflow / wygoda
- 🔴 **Batch operations menu** — przenieś/kopiuj/hardlink/symlink/otwórz wszystkie.
- 🟢 **Toast po akcji** — „Usunięto 5 plików".
- 🟡 **Storage health dashboard** w Home — zajętość dysku, % duplikatów, największe foldery.
- 🟢 **Ładne ilustracje empty-state** zamiast emoji.

---

## 3. czkawka_gui (GTK4, starszy)

### Brakujące narzędzia (są w CLI/krokiet, brak w GTK)
- 🟡 **Bad Names** — wyszukiwanie plików o niepoprawnych nazwach.
- 🔴 **Video Optimizer** — transkodowanie / obcinanie czarnych pasów.
- 🟡 **Exif Remover** — usuwanie metadanych z obrazów.

### Interfejs
- 🟡 **Grid view miniatur** dla Similar Images + suwak rozmiaru miniatur.
- 🟡 **Podgląd metadanych** (ID3/EXIF) przed usunięciem.
- 🟡 **Pasek filtrowania wyników** w czasie rzeczywistym.
- 🟢 **Ciemny tryb** w ustawieniach; 🟡 **rozszerzone tooltipy** opcji.
- 🟡 **Szczegółowe statystyki** — miejsce do odzyskania, rozkład duplikatów po katalogach.

---

## 4. czkawka_cli

### Wyjście / integracja
- 🟡 **Formaty CSV / YAML / XML** wyników; 🟡 **streaming JSON** (`--stream-json`).
- 🟡 **`--progress-json`** — postęp w JSON dla integracji z innymi narzędziami.
- 🟡 **Webhooks** — HTTP POST z wynikami po skanie.
- 🟢 **Zmienne ENV** (`CZKAWKA_THREADS`, `CZKAWKA_CACHE_PATH`) + globalny `~/.czkawka/config.toml`.

### Bezpieczeństwo / interaktywność
- 🟡 **`--interactive`** — krok po kroku z potwierdzeniami.
- 🟢 **`--confirm-each`** — potwierdzenie każdego usunięcia.
- 🟡 **`--max-files-delete` / `--max-space-delete`** — limity bezpieczeństwa.
- 🟡 **`--export-fix-script`** — generuj skrypt bash/powershell zamiast usuwać od razu.
- 🟢 **`--timing`** — czas każdej fazy (skan/hash/porównanie/usuwanie).

### Funkcje
- 🟡 **`-r/--regex-filter`** + **`--filter-condition`** (np. `size > 100MB AND path contains temp`).
- 🟡 **Komenda `batch`** — wiele skanów z pliku konfiguracyjnego.
- 🟡 **Komenda `report`** — delta między dwoma skanami.
- 🟢 **`--export-playlist m3u`** dla duplikatów muzyki.

---

## 5. Ukryte możliwości silnika (są w core, słabo/wcale wystawione w GUI)

> Te opcje już istnieją w `czkawka_core` — wystarczy dodać im element UI.

- 🟢 **`case_sensitive_name_comparison`** (Duplicates) → checkbox obok metody „NAME".
- 🟢 **`exclude_images_with_same_size` / `..._resolution`** (Similar Images) → toggle w głównym widoku.
- 🟡 **`compare_fingerprints_only_with_similar_titles`** (Same Music) → redukuje false-positive, brak w GUI.
- 🟢 **`search_zero_byte_content_files` / `search_non_printable_content_files`** (Empty Files) → checkboxy.
- 🟡 **`exclude_other_filesystems`** (Linux) → checkbox „pomiń zamontowane/sieciowe dyski".
- 🟡 **`hide_hard_links`** (Common) → toggle ukrywania hardlinków w wynikach.
- 🟡 **`save_also_as_json`** (Common) → brak w CLI i GUI; globalna flaga zapisu JSON.
- 🔴 **`hardware_encoder`** (Video Optimizer) → **całkowicie brak w CLI** — flaga `--hardware-encoder [nvenc|vaapi|qsv|videotoolbox|amf]`.
- 🟡 **`custom_ffmpeg_command`** (Video Optimizer) → pole istnieje, nieużywane; flaga dla zaawansowanych.
- 🟡 **`noise_reduction` / `_strength`** (Video Optimizer) → brak w CLI mimo wsparcia w core.
- 🟡 **Metadane wideo** (`fps`, `codec`, `bitrate`, `width`, `height`) → kolumna „Info wideo" w wynikach.
- 🟡 **Presety Broken Files** (PDF/AUDIO/IMAGE/ARCHIVE/VIDEO/FONT/MARKUP) → „Tylko multimedia"/„Tylko dokumenty".
- 🟡 **Presety Similar Videos** zamiast wielu suwaków na raz — „Quick / Balanced / Detailed".
- 🔴 **Reference Folder Mode + HardLink** — potężna kombinacja, ale niewyjaśniona; potrzebny czytelny tryb w UI.

---

## 6. Pomysły na zupełnie nowe narzędzia

- 🔴 **Storage Space Analyzer** — treemap/sunburst zajętości po kategoriach (obrazy/wideo/dokumenty/temp/duplikaty), drill-down.
- 🔴 **Media Library Cleanup Assistant** — master łączący Duplicates + Similar Images + Same Music + Broken Files z rekomendacjami „zachowaj najwyższą rozdzielczość".
- 🔴 **File Integrity Verifier** — weryfikacja sum kontrolnych z dołączonych `.md5/.sha256` lub manifestów (rozszerzenie Broken Files).
- 🔴 **Duplicate Content Finder (bez hasha)** — fuzzy matching zawartości (Levenshtein/perceptual) — pomost między Duplicates a Similar Images.
- 🔴 **Batch File Converter** — konwersja formatów (jpg→png, mp4→mkv, docx→pdf) z profilami; uogólnienie Video Optimizer.
- 🟡 **Orphaned References Finder** — wykrywa martwe ścieżki w configach/skrótach/symlinkach (Invalid Symlinks + Broken Files).
- 🟡 **Permission & Ownership Audit** (Linux/macOS) — pliki z niebezpiecznymi uprawnieniami (777), nieoczekiwanym właścicielem.
- 🟡 **Language & Encoding Detector** — pliki tekstowe ze złym/mieszanym kodowaniem + konwersja UTF-8/16/Latin1.
- 🟡 **Duplicate Merger for Documents** — fuzzy matching dokumentów (txt/md/docx) — „Same Music dla tekstu".
- 🟡 **Metadata Normalizer** — normalizacja EXIF/ID3 wg wzorców (rozszerzenie Exif Remover + Same Music).

---

*Wygenerowano przez 4 równoległych agentów przeglądowych (krokiet / cedinia / czkawka_gui+cli / core gap analysis).*
