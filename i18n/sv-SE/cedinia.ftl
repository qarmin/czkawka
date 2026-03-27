# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Dubbletter
tool_empty_folders = Tomma mappar
tool_similar_images = Liknande bilder
tool_empty_files = Tomma filer
tool_temporary_files = Tillfälliga filer
tool_big_files = Största filer
tool_broken_files = Trasiga filer
tool_bad_extensions = Dåliga tillägg
tool_same_music = Musik Duplicerar
tool_bad_names = Dåliga namn
tool_exif_remover = EXIF data
tool_directories = Kataloger
tool_settings = Inställningar
# Home screen tool card descriptions
home_dup_description = Hitta filer med samma innehåll
home_empty_folders_description = Kataloger utan innehåll
home_similar_images_description = Hitta visuellt liknande bilder
home_empty_files_description = Filer med nollstorlek
home_temp_files_description = Tillfälliga och cachade filer
home_big_files_description = Största/minsta filer på disk
home_broken_files_description = PDF, ljud, bilder, arkiv
home_bad_extensions_description = Filer med ogiltigt tillägg
home_same_music_description = Liknande ljudfiler med taggar
home_bad_names_description = Filer med problematiska tecken i namnet
home_exif_description = Bilder med EXIF-metadata
# Results list
scanning = Skannar pågår...
stopping = Stoppar...
no_results = Inga resultat
press_start = Tryck på START för att skanna
select_label = Sälj.
deselect_label = Desel.
list_label = Lista
gallery_label = Gal.
# Selection popup
selection_popup_title = Välj
select_all = Markera alla
select_except_one = Markera alla utom en
select_except_largest = Välj alla utom största
select_except_smallest = Välj alla utom minsta
select_largest = Välj största
select_smallest = Välj minsta
select_except_highest_res = Välj alla utom högsta upplösning
select_except_lowest_res = Välj alla utom lägsta upplösning
select_highest_res = Välj högsta upplösning
select_lowest_res = Välj lägsta upplösning
invert_selection = Invertera markering
close = Stäng
# Deselection popup
deselection_popup_title = Avmarkera
deselect_all = Avmarkera alla
deselect_except_one = Avmarkera alla utom en
# Confirm popup
cancel = Avbryt
delete = Radera
rename = Döp om
# Delete errors popup
delete_errors_title = Det gick inte att ta bort några filer:
ok = Ok
# Stopping overlay
stopping_overlay_title = Stoppar
stopping_overlay_body =
    Slutför aktuell skanning...
    Vänligen vänta.
# Permission popup
permission_title = Åtkomst till fil
permission_body = För att skanna filer behöver appen åtkomst till enhetslagring. Utan denna behörighet kommer skanning inte att vara möjlig.
grant = Bevilja
no_permission_scan_warning = Ingen filåtkomst - bevilja behörighet att skanna
# Settings screen tabs
settings_tab_general = Allmänt
settings_tab_tools = Verktyg
settings_tab_diagnostics = Information
# Settings - General tab
settings_use_cache = Använd cache
settings_use_cache_desc = Snabbar upp efterföljande skanningar (hash/images)
settings_ignore_hidden = Ignorera dolda filer
settings_ignore_hidden_desc = Filer och mappar som börjar med ''
settings_show_notification = Meddela när skanningen är klar
settings_show_notification_desc = Visa en systemavisering vid skanning slutförd
settings_notify_only_background = Endast när i bakgrunden
settings_notify_only_background_desc = Hoppa över avisering om appen är synlig
notifications_disabled_banner = Aviseringar inaktiverade
notifications_enable_button = Aktivera
settings_scan_label = SKANNA
settings_filters_label = FILTERAR (vissa verktyg)
settings_min_file_size = Min. filstorlek
settings_max_file_size = Max. filstorlek
settings_language = Språk
settings_language_restart = Kräver omstart av appen
settings_common_label = KOMMON INSTÄLLNINGAR
settings_excluded_items = EXCLUDED ITEMS (glob mönster, kommaseparerade)
settings_excluded_items_placeholder = t.ex. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = TILLGÄNGLIGA EXTENSIONER (tom = alla)
settings_allowed_extensions_placeholder = t.ex. jpg, png, mp4
settings_excluded_extensions = EXKLICKA EXTENSIONER
settings_excluded_extensions_placeholder = t.ex. bak, tmp, logg
# Settings - Tools section labels
settings_duplicates_header = DUPLIKATER
settings_check_method_label = KOMPARISON METOD
settings_check_method = Metod
settings_hash_type_label = HASHTYP
settings_hash_type = Hash typ
settings_hash_type_desc = Blake3 - rekommenderas alternativ, CRC32 har liten chans att falska positiva
settings_similar_images_header = LIKNANDE BILDER
settings_similarity_preset = Likhet tröskel
settings_similarity_desc = Mycket hög = endast nästan identisk
settings_hash_size = Hashstorlek
settings_hash_size_desc = Större storlekar, har mindre falska positiva, men hittar också mindre liknande bilder
settings_hash_alg = Hash-algoritm
settings_image_filter = Ändra storlek
settings_ignore_same_size = Ignorera bilder med samma mått
settings_gallery_image_fit_cover = Galleri: beskära till kvadrat
settings_gallery_image_fit_cover_desc = Fyll i kakel; inaktivera för att behålla det ursprungliga bildförhållandet
settings_big_files_header = BIGGEST FILER
settings_search_mode = Sök läge
settings_file_count = Antal filer
settings_same_music_header = MUSIKA DUPLIKATER
settings_music_check_method = Jämförelse-läge
settings_music_compare_tags_label = KOMPARED TAGS
settings_music_title = Titel
settings_music_artist = Konstnären
settings_music_year = År
settings_music_length = Längd
settings_music_genre = Genre
settings_music_bitrate = Bithastighet
settings_music_approx = Ungefärlig tagg jämförelse
settings_broken_files_header = BLÄDDRA FILER
settings_broken_files_note = Resursintensiv skanning. För bästa prestanda använd Krokiet på skrivbordet.
settings_broken_files_types_label = CHECKADE TYPER
settings_broken_audio = Ljud
settings_broken_pdf = PDF
settings_broken_archive = Arkivera
settings_broken_image = Bild
settings_bad_names_header = BAD NAMN
settings_bad_names_checks_label = CHECKAR
settings_bad_names_uppercase_ext = Versaler förlängning
settings_bad_names_emoji = Emoji i namn
settings_bad_names_space = Mellanslag vid start/slut
settings_bad_names_non_ascii = Tecken som inte är ASCII
settings_bad_names_duplicated = Upprepade tecken
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIKER
diagnostics_thumbnails = Miniatyrbildscache
diagnostics_app_cache = Applikationscache
diagnostics_refresh = Uppdatera
diagnostics_clear_thumbnails = Rensa miniatyrer
diagnostics_open_thumbnails_folder = Öppna mapp
diagnostics_clear_cache = Rensa cache
diagnostics_open_cache_folder = Öppna mapp
diagnostics_collect_test = Testa åtkomst till filer
diagnostics_collect_test_desc = Kontrollera hur många filer som är tillgängliga
diagnostics_collect_test_run = Kör
diagnostics_collect_test_stop = Stoppa
collect_test_cancelled = Stoppad av användare
diag_confirm_clear_thumbnails = Rensa alla miniatyrcache?
diag_confirm_clear_cache = Rensa alla appcache?
about_repo = Utveckling
about_translate = Översättningar
about_donate = Stöd
# Collect-test result popup
collect_test_title = Testa resultat
collect_test_volumes = Volymer:
collect_test_folders = Mappar:
collect_test_files = Filer:
collect_test_time = Tid:
# Licenses
licenses_label = LICENS
third_party_licenses = Tredjeparts licenser
licenses_popup_title = Licenser för tredje part
# Directories screen
directories_include_header = Inkludera
directories_included = Ingår
directories_exclude_header = Exkludera
directories_excluded_header = Utesluten
directories_add = Inkludera
no_paths = Inga sökvägar - lägg till nedan
directories_volume_header = Volymer
directories_volume_refresh = Uppdatera
directories_volume_add = Lägg till
# Bottom navigation
nav_home = Starta
nav_dirs = Kataloger
nav_settings = Inställningar
# Status messages set from Rust
status_ready = Redo
status_stopped = Stoppad
status_no_results = Inga resultat
status_deleted_selected = Borttagna valda
status_deleted_with_errors = Tog bort med fel
scan_not_started = Skanna inte startad
found_items_prefix = Hittad
found_items_suffix = objekt
deleted_items_prefix = Borttagen
deleted_items_suffix = objekt
deleted_errors_suffix = fel
renamed_prefix = Namnbyte
renamed_files_suffix = filer
renamed_errors_suffix = fel
cleaned_exif_prefix = Rensat EXIF från
cleaned_exif_suffix = filer
cleaned_exif_errors_suffix = fel
and_more_prefix = ...och
and_more_suffix = mer
# Gallery / delete popups
gallery_delete_button = Radera
gallery_back = Tillbaka
gallery_confirm_delete = Ja, ta bort
deleting_files = Tar bort filer...
stop = Stoppa
files_suffix = filer
scanning_fallback = Skannar...
app_subtitle = För att hedra slaget vid Cedynia (972 CE)
app_license = Frontend för Czkawka Core - GPL-3.0
about_app_label = OM
cache_label = CACHE
# Notification
scan_completed_notification = Skanna klar - { $file_count } objekt hittade
# Confirm popups (set from Rust)
confirm_clean_exif = Vill du verkligen rensa EXIF-taggar från { $n } valda filer?
confirm_delete_items = Är du säker på att du vill ta bort { $n } valda objekt?
gallery_confirm_delete_msg = Du håller på att ta bort { $total_images } bilder i { $total_groups } grupper.
gallery_confirm_delete_warning = Alla objekt är markerade i { $unsafe_groups } grupper!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Att beräkna och jämföra ljudfingeravtryck är mycket resurskrävande och kan ta lång tid. Det rekommenderas att använda Krokiet på ett skrivbordssystem för denna uppgift.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Samlar in filer
stage_scanning_name = Skannar efter namn
stage_scanning_size_name = Skannar efter namn och storlek
stage_scanning_size = Skannar efter storlek
stage_pre_hash = För-hashning
stage_full_hash = Hashning
stage_loading_cache = Laddar cache
stage_saving_cache = Sparar cache
stage_calculating_image_hashes = Beräknar bildhashen
stage_comparing_images = Jämföra bilder
stage_calculating_video_hashes = Beräknar videohashar
stage_checking_files = Kontrollerar filer
stage_checking_extensions = Kontrollerar tillägg
stage_checking_names = Kontrollerar namn
stage_reading_music_tags = Läser musik taggar
stage_comparing_tags = Jämföra taggar
stage_calculating_music_fingerprints = Beräknar musikfingeravtryck
stage_comparing_fingerprints = Jämföra fingeravtryck
stage_extracting_exif = Läser EXIF-taggar
stage_creating_video_thumbnails = Skapa miniatyrer för video
stage_processing_videos = Bearbetar videor
stage_deleting = Tar bort filer
stage_renaming = Byter namn på filer
stage_moving = Flyttar filer
stage_hardlinking = Skapar hårda länkar
stage_symlinking = Skapar symlänkar
stage_optimizing_videos = Optimerar videor
stage_cleaning_exif = Rengöring av EXIF
# Group headers in scan results
duplicates_group_header = { $count } filer x { $per_file } / fil = { $total } totalt
similar_images_group_header = { $count } liknande bilder
same_music_group_header = { $count } liknande spår
# Rename confirmation
confirm_rename_items = Är du säker på att du vill byta namn på { $n } valda filer?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Största
option_search_mode_smallest = Minsta
option_similarity_very_high = V.Hög
option_similarity_high = Hög
option_similarity_medium = Medel
option_similarity_low = Låg
option_similarity_very_low = V.Låg
option_similarity_minimal = Min.
option_check_method_hash = Hash
option_check_method_name = Namn
option_check_method_size_and_name = Storlek+namn
option_check_method_size = Storlek
option_music_method_tags = Taggar
option_music_method_audio = Ljud
option_min_size_none = Ingen
option_min_size_1kb = 1 kB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Obegränsad
# Volume labels (shown in the directories screen)
volume_internal_storage = Intern lagring
volume_sd_card = Minneskort (SD-kort)
volume_storage = Lagring Volym
# Directories screen
directories_referenced_tooltip = Refererad (ej borttagen)
directories_include_section_header = ÖKLICKA
directories_exclude_section_header = EXKLICKA
directories_custom_paths = Anpassade sökvägar
directories_check_button = Analysera
directories_check_popup_title = Katalogstatistik
directories_check_label_included = Inkluderade sökvägar:
directories_check_label_excluded = Uteslutna sökvägar:
directories_check_label_referenced = Referens sökvägar:
directories_check_label_would_scan = Filer att skanna:
directories_check_label_processable = Processbara filer:
directories_check_scanning = Skannar...
directories_check_warning_no_processable = Inga bearbetningsbara filer hittades - verifiera dina inkluderade/uteslutna mappar
path_edit_title_include = Lägg till i Inkludera
path_edit_title_exclude = Lägg till för att utesluta
path_edit_placeholder = Ange sökväg...
path_edit_not_exists = Sökvägen finns inte
path_edit_is_dir = Katalog
path_edit_is_file = Fil
path_edit_no_newlines = Sökvägar kan inte innehålla newlines — Ange nyckel är inte tillåten
ctx_menu_title = Öppna
ctx_open_file = Öppna objekt
ctx_open_folder = Öppna överordnad mapp
