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
tool_bad_extensions = Felaktiga filändelser
tool_same_music = Musikdubbletter
tool_bad_names = Felaktiga namn
tool_exif_remover = EXIF-data
tool_similar_videos = Liknande videor (ljud)
tool_directories = Mappar
tool_settings = Inställningar
# Home screen tool card descriptions
home_dup_description = Hitta filer med samma innehåll
home_empty_folders_description = Mappar utan innehåll
home_similar_images_description = Hitta visuellt liknande bilder
home_empty_files_description = Filer med nollstorlek
home_temp_files_description = Tillfälliga och cachade filer
home_big_files_description = Största/minsta filer på disk
home_broken_files_description = PDF, ljud, bilder, arkiv
home_bad_extensions_description = Filer med ogiltig filändelse
home_same_music_description = Liknande ljudfiler med taggar
home_bad_names_description = Filer med problematiska tecken i namnet
home_exif_description = Bilder med EXIF-metadata
home_similar_videos_description = Hitta videor med liknande ljud
# Results list
scanning = Skanning pågår….
stopping = Stoppar...
no_results = Inga resultat
press_start = Tryck på START för att skanna
select_label = Mark.
deselect_label = Avmark.
list_label = Lista
gallery_label = Gall.
# Selection popup
selection_popup_title = Markera
select_all = Markera alla
select_except_one = Markera alla utom en
select_except_largest = Markera alla utom den största
select_except_smallest = Markera alla utom den minsta
select_largest = Markera den största
select_smallest = Markera den minsta
select_except_highest_res = Markera alla utom den högsta upplösningen
select_except_lowest_res = Markera alla utom den lägsta upplösningen
select_highest_res = Markera den högsta upplösningen
select_lowest_res = Markera den lägsta upplösningen
invert_selection = Invertera markering
close = Stäng
# Deselection popup
deselection_popup_title = Avmarkera
deselect_all = Avmarkera alla
deselect_except_one = Avmarkera alla utom en
# Confirm popup
cancel = Avbryt
delete = Ta bort
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
settings_use_cache_desc = Snabbar upp efterföljande skanningar (hashvärden/bilder)
settings_ignore_hidden = Ignorera dolda filer
settings_ignore_hidden_desc = Filer och mappar vars namn börjar med "."
settings_show_notification = Meddela när skanningen är klar
settings_show_notification_desc = Visa en systemavisering vid skanning slutförd
settings_notify_only_background = Endast när i bakgrunden
settings_notify_only_background_desc = Hoppa över avisering om appen är synlig
notifications_disabled_banner = Aviseringar inaktiverade
notifications_enable_button = Aktivera
settings_scan_label = SKANNA
settings_filters_label = FILTER (vissa verktyg)
settings_min_file_size = Min. filstorlek
settings_max_file_size = Max. filstorlek
settings_language = Språk
settings_language_restart = Kräver omstart av appen
settings_common_label = VANLIGA INSTÄLLNINGAR
settings_excluded_items = UNDANTAGNA OBJEKT (globmönster, kommaseparerade)
settings_excluded_items_placeholder = t.ex. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = TILLGÄNGLIGA FILÄNDELSER (tom = alla)
settings_allowed_extensions_placeholder = t.ex. jpg, png, mp4
settings_excluded_extensions = EXKLUDERADE FILÄNDELSER
settings_excluded_extensions_placeholder = t.ex. bak, tmp, logg
# Settings - Tools section labels
settings_duplicates_header = DUBBLETTER
settings_check_method_label = JÄMFÖRELSEMETOD
settings_check_method = Metod
settings_hash_type_label = HASHTYP
settings_hash_type = Hashtyp
settings_hash_type_desc = Blake3 - är det rekommenderade alternativet. CRC32 innebär en liten risk för falska positiva resultat
settings_similar_images_header = LIKNANDE BILDER
settings_similarity_preset = Likhetströskel
settings_similarity_desc = Mycket hög = endast nästan identiska
settings_hash_size = Hashstorlek
settings_hash_size_desc = Större storlekar ger färre falska positiva resultat, men hittar också färre liknande bilder
settings_hash_alg = Hash-algoritm
settings_image_filter = Storleksändringsfilter
settings_geometric_invariance = Geometrisk invarians
settings_ignore_same_size = Ignorera bilder med samma mått
settings_gallery_image_fit_cover = Galleri: beskär till kvadrat
settings_gallery_image_fit_cover_desc = Fyll hela rutan; inaktivera för att behålla originalets bildförhållande
settings_big_files_header = STÖRSTA FILER
settings_search_mode = Sökläge
settings_file_count = Antal filer
settings_same_music_header = MUSIKDUBBLETTER
settings_music_check_method = Jämförelseläge
settings_music_compare_tags_label = JÄMFÖRDA TAGGAR
settings_music_title = Titel
settings_music_artist = Konstnär
settings_music_year = År
settings_music_length = Längd
settings_music_genre = Genre
settings_music_bitrate = Bithastighet
settings_music_approx = Ungefärlig taggjämförelse
settings_temporary_files_header = TEMPORARI FILER
settings_temporary_files_extensions_label = EXTENSIONER
settings_temporary_files_extensions_placeholder = t.ex. .tmp, .bak, ~
settings_temporary_files_reset = Återställ till standardvärden
settings_broken_files_header = TRASIGA FILER
settings_broken_files_note = Resursintensiv skanning. För bästa prestanda bör du använda Krokiet på skrivbordet.
settings_broken_files_types_label = FILTYPER SOM KONTROLLERAS
settings_broken_audio = Ljud
settings_broken_pdf = PDF
settings_broken_archive = Arkiv
settings_broken_image = Bild
settings_broken_font = Typsnitt
settings_broken_markup = Markup (JSON/XML/TOML)
settings_similar_videos_header = LIKNANDE VIDEOR (LJUD)
settings_similar_videos_audio_preset = Förinställning för ljudets likhet
settings_similar_videos_audio_preset_desc = Kontrollerar hur strikt ljud måste matcha
settings_bad_names_header = FELAKTIGA NAMN
settings_bad_names_checks_label = KONTROLLER
settings_bad_names_uppercase_ext = Filändelse med versaler
settings_bad_names_emoji = Emoji i namn
settings_bad_names_space = Mellanslag vid start/slut
settings_bad_names_non_ascii = Tecken som inte är ASCII
settings_bad_names_duplicated = Upprepade tecken
settings_ignore_same_resolution = Ignorera bilder med samma upplösning
# Settings - Appearance section
settings_appearance_label = UTSEENDE
settings_dark_theme = Mörkt tema
settings_dark_theme_desc = Använd mörk färgschema
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIK
diagnostics_thumbnails = Miniatyrbildscache
diagnostics_app_cache = Applikationscache
diagnostics_refresh = Uppdatera
diagnostics_clear_thumbnails = Rensa miniatyrer
diagnostics_open_thumbnails_folder = Öppna mapp
diagnostics_clear_cache = Rensa cache
diagnostics_open_cache_folder = Öppna mapp
diagnostics_export_logs = Exportera loggar
logs_label = LOGGGA
logs_export_title = Exportera loggar
logs_export_saved = Loggar kopierade till:
logs_export_failed = Kunde inte exportera loggar
diagnostics_collect_test = Testa åtkomst till filer
diagnostics_collect_test_desc = Kontrollera hur många filer som är tillgängliga
diagnostics_collect_test_run = Kör
diagnostics_collect_test_stop = Stoppa
collect_test_cancelled = Stoppad av användaren
diag_confirm_clear_thumbnails = Rensa hela miniatyrcachen?
diag_confirm_clear_cache = Rensa hela appcachen?
about_repo = Källkodsarkiv
about_translate = Översättningar
about_donate = Stöd
# Collect-test result popup
collect_test_title = Testresultat
collect_test_volumes = Volymer:
collect_test_folders = Mappar:
collect_test_files = Filer:
collect_test_time = Tid:
# Licenses
licenses_label = LICENS
third_party_licenses = Tredjepartslicenser
licenses_popup_title = Licenser för tredje part
# Directories screen
directories_include_header = Inkludera
directories_included = Inkluderade
directories_exclude_header = Exkludera
directories_excluded_header = Exkluderade
directories_add = Inkludera
no_paths = Inga sökvägar - lägg till nedan
directories_volume_header = Volymer
directories_volume_refresh = Uppdatera
directories_volume_add = Lägg till
# Bottom navigation
nav_home = Start
nav_dirs = Mappar
nav_settings = Inställningar
# Status messages set from Rust
status_ready = Redo
status_stopped = Stoppad
status_no_results = Inga resultat
status_deleted_selected = Tog bort markerade
status_deleted_with_errors = Tog bort med fel
scan_not_started = Skanningen har inte startat
found_items_prefix = Hittade
found_items_suffix = objekt
deleted_items_prefix = Tog bort
deleted_items_suffix = objekt
deleted_errors_suffix = fel
renamed_prefix = Bytte namn på
renamed_files_suffix = filer
renamed_errors_suffix = fel
cleaned_exif_prefix = Rensat EXIF från
cleaned_exif_suffix = filer
cleaned_exif_errors_suffix = fel
rename_error_read_file_name = Kan inte läsa filnamnet
rename_error_read_directory = Kan inte läsa katalog
and_more_prefix = ...och
and_more_suffix = mer
# Gallery / delete popups
gallery_delete_button = Ta bort
gallery_back = Tillbaka
gallery_confirm_delete = Ja, ta bort
deleting_files = Tar bort filer...
stop = Stoppa
scanning_fallback = Skannar...
app_subtitle = För att hedra slaget vid Cedynia (972 CE)
app_license = Frontend för Czkawka Core - GPL-3.0
about_app_label = OM
cache_label = CACHE
# Notification
scan_completed_notification = Skanning klar - { $file_count } objekt hittade
# Confirm popups (set from Rust)
confirm_clean_exif = Vill du verkligen rensa EXIF-taggar från { $n } markerade filer?
confirm_delete_items = Är du säker på att du vill ta bort { $n } markerade objekt?
gallery_confirm_delete_msg = Du håller på att ta bort { $total_images } bilder i { $total_groups } grupper.
gallery_confirm_delete_warning = Alla objekt är markerade i { $unsafe_groups } grupper!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Att beräkna och jämföra ljudfingeravtryck är mycket resurskrävande och kan ta lång tid. Det rekommenderas att använda Krokiet på ett skrivbordssystem för denna uppgift.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } filer x { $per_file } / fil = { $total } totalt
similar_images_group_header = { $count } liknande bilder
same_music_group_header = { $count } liknande spår
similar_videos_group_header = { $count } liknande videor
# Rename confirmation
confirm_rename_items = Är du säker på att du vill byta namn på { $n } markerade filer?
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
option_max_size_unlimited = Obegränsad
option_audio_preset_identical = Identisk
option_audio_preset_clip = Klipp in längre
option_audio_preset_similar = Liknande
# Volume labels (shown in the directories screen)
volume_internal_storage = Intern lagring
volume_sd_card = Minneskort (SD-kort)
volume_storage = Lagringsvolym
# Directories screen
directories_referenced_tooltip = Refererad (inte borttagen)
directories_include_section_header = INKLUDERADE
directories_exclude_section_header = EXKLUDERADE
directories_custom_paths = Anpassade sökvägar
directories_check_button = Analysera
directories_check_popup_title = Mappstatistik
directories_check_label_included = Inkluderade sökvägar:
directories_check_label_excluded = Exkluderade sökvägar:
directories_check_label_referenced = Referenssökvägar:
directories_check_label_would_scan = Filer att skanna:
directories_check_label_processable = Filer som kan bearbetas:
directories_check_scanning = Skannar...
directories_check_warning_no_processable = Inga filer som kan bearbetas hittades. Kontrollera vilka mappar som har inkluderats eller exkluderats
path_edit_title_include = Lägg till bland inkluderade
path_edit_title_exclude = Lägg till bland exkluderade
path_edit_placeholder = Ange sökväg...
path_edit_not_exists = Sökvägen finns inte
path_edit_is_dir = Mapp
path_edit_is_file = Fil
path_edit_no_newlines = Sökvägar får inte innehålla radbrytningar. Enter-tangenten får inte användas
ctx_menu_title = Öppna
ctx_open_file = Öppna objekt
ctx_open_folder = Öppna överordnad mapp
dir_open_folder = Öppna mapp
# Compare view
compare_label = Jämför
compare_loading = Laddar bilder…
compare_cancelling = Avbryter…
compare_computing = Beräknar diff…
compare_mode_normal = Sida
compare_mode_split = Dela
compare_mode_overlay = Överlagring
compare_mode_diff = Skillnad
compare_res_mismatch = Olika resolutioner - diff kan vara felaktig
