# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplikate
tool_empty_folders = Leere Ordner
tool_similar_images = Ähnliche Bilder
tool_empty_files = Leere Dateien
tool_temporary_files = Temporäre Dateien
tool_big_files = Größte Dateien
tool_broken_files = Defekte Dateien
tool_bad_extensions = Falsche Erweiterungen
tool_same_music = Musik-Duplikate
tool_bad_names = Falsche Namen
tool_exif_remover = EXIF-Daten
tool_directories = Verzeichnisse
tool_settings = Einstellungen
# Home screen tool card descriptions
home_dup_description = Suchen Sie Dateien mit dem gleichen Inhalt
home_empty_folders_description = Verzeichnisse ohne Inhalt
home_similar_images_description = Visuell ähnliche Fotos finden
home_empty_files_description = Dateien mit Null-Größe
home_temp_files_description = Temporäre und zwischengespeicherte Dateien
home_big_files_description = Größte/Kleinste Dateien auf der Festplatte
home_broken_files_description = PDF, Audio, Bilder, Archive
home_bad_extensions_description = Dateien mit ungültiger Erweiterung
home_same_music_description = Ähnliche Audiodateien nach Tags
home_bad_names_description = Dateien mit problematischen Zeichen im Namen
home_exif_description = Bilder mit EXIF-Metadaten
# Results list
scanning = Scanne läuft...
stopping = Beenden...
no_results = Keine Ergebnisse
press_start = Zum Scannen START drücken
select_label = Auswahl.
deselect_label = Desel.
list_label = Liste
gallery_label = Gal.
# Selection popup
selection_popup_title = Auswählen
select_all = Alles auswählen
select_except_one = Alle außer einer auswählen
select_except_largest = Alle außer Größten auswählen
select_except_smallest = Alle außer kleinsten auswählen
select_largest = Größte auswählen
select_smallest = Kleinste auswählen
select_except_highest_res = Alle außer der höchsten Auflösung auswählen
select_except_lowest_res = Alle außer niedrigste Auflösung auswählen
select_highest_res = Wählen Sie die höchste Auflösung
select_lowest_res = Niedrigste Auflösung auswählen
invert_selection = Auswahl umkehren
close = Schließen
# Deselection popup
deselection_popup_title = Abwählen
deselect_all = Alle abwählen
deselect_except_one = Alle außer eines abwählen
# Confirm popup
cancel = Abbrechen
delete = Löschen
rename = Umbenennen
# Delete errors popup
delete_errors_title = Löschen einiger Dateien fehlgeschlagen:
ok = Ok
# Stopping overlay
stopping_overlay_title = Stoppen
stopping_overlay_body =
    Aktuellen Scan wird beendet...
    Bitte warten.
# Permission popup
permission_title = Dateizugriff
permission_body = Um Dateien zu scannen, benötigt die App Zugriff auf den Gerätespeicher. Ohne diese Berechtigung ist das Scannen nicht möglich.
grant = Zuschuss
no_permission_scan_warning = Kein Dateizugriff - Berechtigung zum Scannen gewähren
# Settings screen tabs
settings_tab_general = Allgemein
settings_tab_tools = Werkzeuge
settings_tab_diagnostics = Information
# Settings - General tab
settings_use_cache = Cache verwenden
settings_use_cache_desc = Beschleunigt folgende Scans (Hash/Bilder)
settings_ignore_hidden = Versteckte Dateien ignorieren
settings_ignore_hidden_desc = Dateien und Ordner beginnend mit '.'
settings_show_notification = Benachrichtigen, wenn Scan beendet ist
settings_show_notification_desc = Systembenachrichtigung beim Scanabschluss anzeigen
settings_notify_only_background = Nur im Hintergrund
settings_notify_only_background_desc = Benachrichtigung überspringen, wenn die App sichtbar ist
notifications_disabled_banner = Benachrichtigungen deaktiviert
notifications_enable_button = Aktivieren
settings_scan_label = SCAN
settings_filters_label = FILTERS (einige Tools)
settings_min_file_size = Min. Dateigröße
settings_max_file_size = Max. Dateigröße
settings_language = Sprache
settings_language_restart = App-Neustart erforderlich
settings_common_label = COMMON-EINSTELLUNGEN
settings_excluded_items = EXKLUDETE ITEMS (Glob-Muster, durch Komma getrennt)
settings_excluded_items_placeholder = z.B. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EXTENSIONEN ERLAUBEN (leer = alle)
settings_allowed_extensions_placeholder = z.B. jpg, png, mp4
settings_excluded_extensions = AUSSCHLIESSLICHE EXTENSIONEN
settings_excluded_extensions_placeholder = z.B. bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLIKATEN
settings_check_method_label = COMPARISON-METHOD
settings_check_method = Methode
settings_hash_type_label = HASH-Typ
settings_hash_type = Hash Typ
settings_hash_type_desc = Blake3 - wird empfohlen, CRC32 haben eine geringe Chance auf Fehlalarme
settings_similar_images_header = ÄHNLICHE BILDER
settings_similarity_preset = Ähnlichkeitsschwelle
settings_similarity_desc = Sehr hoch = nur fast identisch
settings_hash_size = Hash-Größe
settings_hash_size_desc = Größere Größen, weniger falsche Positive, aber weniger ähnliche Bilder finden
settings_hash_alg = Hash-Algorithmus
settings_image_filter = Filter skalieren
settings_ignore_same_size = Bilder mit den gleichen Dimensionen ignorieren
settings_gallery_image_fit_cover = Galerie: auf Quadrat zuschneiden
settings_gallery_image_fit_cover_desc = Füllen Sie die Kachel; deaktivieren um das ursprüngliche Seitenverhältnis zu behalten
settings_big_files_header = BIGGESTE DATEIEN
settings_search_mode = Suchmodus
settings_file_count = Dateigröße
settings_same_music_header = MUSISCHE DUPLIKATEN
settings_music_check_method = Vergleichsmodus
settings_music_compare_tags_label = GESCHLOSSENE TAGS
settings_music_title = Titel
settings_music_artist = Künstler
settings_music_year = Jahr
settings_music_length = Länge
settings_music_genre = Genre
settings_music_bitrate = Bitrate
settings_music_approx = Ungefährer Tag-Vergleich
settings_broken_files_header = BROKEN DATEIEN
settings_broken_files_note = Ressourcenintensiver Scan. Für beste Performance verwenden Sie Krokiet auf dem Desktop.
settings_broken_files_types_label = GESCHÄFTSTYPEN
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Archivieren
settings_broken_image = Bild
settings_bad_names_header = SCHLECHTE BEZEICHNUNGEN
settings_bad_names_checks_label = PRÜFUNGEN
settings_bad_names_uppercase_ext = Großbuchstaben Erweiterung
settings_bad_names_emoji = Emoji im Namen
settings_bad_names_space = Leerzeichen am Start/Ende
settings_bad_names_non_ascii = Nicht-ASCII-Zeichen
settings_bad_names_duplicated = Wiederholte Zeichen
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIK
diagnostics_thumbnails = Miniaturansicht-Cache
diagnostics_app_cache = App-Cache
diagnostics_refresh = Aktualisieren
diagnostics_clear_thumbnails = Thumbnails löschen
diagnostics_open_thumbnails_folder = Ordner öffnen
diagnostics_clear_cache = Cache leeren
diagnostics_open_cache_folder = Ordner öffnen
diagnostics_collect_test = Datei-Zugriffstest
diagnostics_collect_test_desc = Überprüfen Sie, wie viele Dateien zugänglich sind
diagnostics_collect_test_run = Ausführen
diagnostics_collect_test_stop = Stoppen
collect_test_cancelled = Stoppt von Benutzer
diag_confirm_clear_thumbnails = Thumbnail-Cache löschen?
diag_confirm_clear_cache = Alle App-Cache löschen?
about_repo = Repository
about_translate = Übersetzungen
about_donate = Unterstützung
# Collect-test result popup
collect_test_title = Testergebnisse
collect_test_volumes = Lautstärke:
collect_test_folders = Ordner:
collect_test_files = Dateien:
collect_test_time = Zeit:
# Licenses
licenses_label = LIZENZ
third_party_licenses = Drittlizenzen
licenses_popup_title = Drittanbieter-Lizenzen
# Directories screen
directories_include_header = Einschließen
directories_included = Beinhaltet
directories_exclude_header = Ausschließen
directories_excluded_header = Ausgeschlossen
directories_add = Einschließen
no_paths = Keine Pfade - unten hinzufügen
directories_volume_header = Lautstärke
directories_volume_refresh = Aktualisieren
directories_volume_add = Neu
# Bottom navigation
nav_home = Start
nav_dirs = Verzeichnisse
nav_settings = Einstellungen
# Status messages set from Rust
status_ready = Bereit
status_stopped = Stoppt
status_no_results = Keine Ergebnisse
status_deleted_selected = Ausgewählte gelöscht
status_deleted_with_errors = Mit Fehlern gelöscht
scan_not_started = Scan nicht gestartet
found_items_prefix = Gefunden
found_items_suffix = gegenstände
deleted_items_prefix = Gelöscht
deleted_items_suffix = gegenstände
deleted_errors_suffix = Fehler
renamed_prefix = Umbenannt
renamed_files_suffix = Dateien
renamed_errors_suffix = Fehler
cleaned_exif_prefix = EXIF bereinigt von
cleaned_exif_suffix = Dateien
cleaned_exif_errors_suffix = Fehler
and_more_prefix = ...und
and_more_suffix = mehr
# Gallery / delete popups
gallery_delete_button = Löschen
gallery_back = Zurück
gallery_confirm_delete = Ja, löschen
deleting_files = Lösche Dateien...
stop = Stoppen
files_suffix = Dateien
scanning_fallback = Scanne...
app_subtitle = Zu Ehren der Schlacht von Cedynia (972 n. Chr.)
app_license = Frontend für Czkawka Core - GPL-3.0
about_app_label = ÜBER
cache_label = CACHE
# Notification
scan_completed_notification = Scannen abgeschlossen - { $file_count } Elemente gefunden
# Confirm popups (set from Rust)
confirm_clean_exif = Sind Sie sicher, dass Sie EXIF-Tags von { $n } ausgewählten Dateien löschen möchten?
confirm_delete_items = Sind Sie sicher, dass Sie die ausgewählten Elemente { $n } löschen möchten?
gallery_confirm_delete_msg = Sie sind dabei { $total_images } Bilder in { $total_groups } Gruppen zu löschen.
gallery_confirm_delete_warning = Alle Elemente sind in { $unsafe_groups } Gruppen ausgewählt!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Die Berechnung und der Vergleich von Audio-Fingerabdrücken ist sehr ressourcenintensiv und kann sehr lange dauern. Es wird empfohlen, Krokiet auf einem Desktop-System für diese Aufgabe zu verwenden.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Sammeln von Dateien
stage_scanning_name = Suche nach Name
stage_scanning_size_name = Suche nach Name und Größe
stage_scanning_size = Suche nach Größe
stage_pre_hash = Vorhashing
stage_full_hash = Hashing
stage_loading_cache = Cache wird geladen
stage_saving_cache = Cache speichern
stage_calculating_image_hashes = Berechne Bildhashes
stage_comparing_images = Bilder vergleichen
stage_calculating_video_hashes = Berechne Video-Hashes
stage_checking_files = Überprüfe Dateien
stage_checking_extensions = Überprüfe Erweiterungen
stage_checking_names = Überprüfe Namen
stage_reading_music_tags = Liest Musik-Tags
stage_comparing_tags = Tags vergleichen
stage_calculating_music_fingerprints = Musik-Fingerabdrücke berechnen
stage_comparing_fingerprints = Fingerabdrücke vergleichen
stage_extracting_exif = Lese EXIF-Tags
stage_creating_video_thumbnails = Erstelle Video-Thumbnails
stage_processing_videos = Videos werden verarbeitet
stage_deleting = Lösche Dateien
stage_renaming = Dateien umbenennen
stage_moving = Dateien verschieben
stage_hardlinking = Erstellen von harten Links
stage_symlinking = Erstelle Symlinks
stage_optimizing_videos = Videos optimieren
stage_cleaning_exif = Reinige EXIF
# Group headers in scan results
duplicates_group_header = { $count } Dateien x { $per_file } / file = { $total } gesamt
similar_images_group_header = { $count } ähnliche Bilder
same_music_group_header = { $count } ähnliche Tracks
# Rename confirmation
confirm_rename_items = Sind Sie sicher, dass Sie die ausgewählten Dateien umbenennen möchten { $n}?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Größte
option_search_mode_smallest = Kleinste
option_similarity_very_high = V.Hoch
option_similarity_high = Hoch
option_similarity_medium = Mittel
option_similarity_low = Niedrig
option_similarity_very_low = V.Niedrig
option_similarity_minimal = Min.
option_check_method_hash = Hash
option_check_method_name = Name
option_check_method_size_and_name = Größe+Name
option_check_method_size = Größe
option_music_method_tags = Tags
option_music_method_audio = Audio
option_min_size_none = Keine
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Unbegrenzt
# Volume labels (shown in the directories screen)
volume_internal_storage = Interner Speicher
volume_sd_card = Speicherkarte (SD-Karte)
volume_storage = Speicherlautstärke
# Directories screen
directories_referenced_tooltip = Verwiesen (nicht gelöscht)
directories_include_section_header = EINSCHLIESSLICH
directories_exclude_section_header = AUSSCHLIESSLICH
directories_custom_paths = Eigene Pfade
directories_check_button = Analysieren
directories_check_popup_title = Verzeichnisstatistik
directories_check_label_included = Inklusive Pfade:
directories_check_label_excluded = Ausgeschlossene Pfade:
directories_check_label_referenced = Referenzpfad:
directories_check_label_would_scan = Zu scannende Dateien:
directories_check_label_processable = Verarbeitbare Dateien:
directories_check_scanning = Scanne...
directories_check_warning_no_processable = Keine verarbeitbaren Dateien gefunden - überprüfen Sie die enthaltenen oder ausgeschlossenen Ordner
path_edit_title_include = Hinzufügen
path_edit_title_exclude = Zu Ausschluss hinzufügen
path_edit_placeholder = Pfad eingeben...
path_edit_not_exists = Pfad existiert nicht
path_edit_is_dir = Verzeichnis
path_edit_is_file = Datei
path_edit_no_newlines = Pfade dürfen keine Zeilenumbrüche enthalten — Eingabe ist nicht erlaubt
ctx_menu_title = Öffnen
ctx_open_file = Element öffnen
ctx_open_folder = Eltern-Ordner öffnen
