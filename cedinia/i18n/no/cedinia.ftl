# Cedinia - English (fallback)

# App / top bar titles
tool_duplicate_files = Duplikater
tool_empty_folders = Tomme mapper
tool_similar_images = Lignende bilder
tool_empty_files = Tomme filer
tool_temporary_files = Midlertidige filer
tool_big_files = Største filer
tool_broken_files = Ødelagte filer
tool_bad_extensions = Feil filendelser
tool_same_music = Musikkduplikater
tool_bad_names = Dårlige navn
tool_exif_remover = EXIF data
tool_similar_videos = Lignende videoer (lyd)
tool_directories = Mapper
tool_settings = Innstillinger
# Home screen tool card descriptions
home_dup_description = Finn filer med samme innhold
home_empty_folders_description = Mapper uten innhold
home_similar_images_description = Finn visuelt like bilder
home_empty_files_description = Filer med null størrelse
home_temp_files_description = Midlertidige og hurtigbufferlagrede filer
home_big_files_description = Største/minste filer på disk
home_broken_files_description = PDF, lyd, bilder, arkiver
home_bad_extensions_description = Filer med ugyldig filendelse
home_same_music_description = Lignende lydfiler etter tagger
home_bad_names_description = Filer med problematiske tegn i navn
home_exif_description = Bilder med EXIF metadata
home_similar_videos_description = Finn videoer med lignende lyd
# Results list
scanning = Skanning pågår...
stopping = Stopper...
no_results = Ingen resultater
press_start = Trykk på START for å skanne
select_label = Sel.
deselect_label = Desel.
list_label = Liste
gallery_label = Galleri
# Selection popup
selection_popup_title = Velg
select_all = Velg alle
select_except_one = Velg alle unntatt en
select_except_largest = Velg alle unntatt største
select_except_smallest = Velg alle unntatt minste
select_largest = Velg største
select_smallest = Velg minste
select_except_highest_res = Velg alle unntatt høyeste oppløsning
select_except_lowest_res = Velg alle unntatt laveste oppløsning
select_highest_res = Velg høyeste oppløsning
select_lowest_res = Velg laveste oppløsning
invert_selection = Inverter merking
close = Lukk
# Deselection popup
deselection_popup_title = Fravelg
deselect_all = Fravelg alle
deselect_except_one = Fravelg alle unntatt én
# Confirm popup
cancel = Avbryt
delete = Slett
rename = Endre navn
# Delete errors popup
delete_errors_title = Kunne ikke slette noen filer:
ok = OK
# Stopping overlay
stopping_overlay_title = Stopper
stopping_overlay_body =
    Avslutter gjeldende søk...
    Vennligst vent.
# Permission popup
permission_title = Filtilgang
permission_body = For å skanne filer må appen ha tilgang til enhetens lagring. Uten denne tillatelsen vil ikke skanning være mulig.
grant = Tillat
no_permission_scan_warning = Ingen filtilgang - gi tillatelse til å skanne
# Settings screen tabs
settings_tab_general = Generelt
settings_tab_tools = Verktøy
settings_tab_diagnostics = Informasjon
# Settings - General tab
settings_use_cache = Bruk hurtigbuffer
settings_use_cache_desc = Akselererer påfølgende skanninger (hash/images)
settings_ignore_hidden = Ignorer skjulte filer
settings_ignore_hidden_desc = Filer og mapper som begynner med '.'
settings_show_notification = Varsle når skanning er ferdig
settings_show_notification_desc = Vis en systemnotifikasjon når skanningen er fullført
settings_notify_only_background = Bare når i bakgrunnen
settings_notify_only_background_desc = Hopp over varsel hvis appen er synlig
notifications_disabled_banner = Varslinger deaktivert
notifications_enable_button = Aktiver
settings_scan_label = SKANN
settings_filters_label = FILTRE (noen verktøy)
settings_min_file_size = Min. filstørrelse
settings_max_file_size = Maks. filstørrelse
settings_language = Språk
settings_language_restart = Krever omstart av appen
settings_common_label = FELLES INNSTILLINGER
settings_excluded_items = EKSKLUDERTE ELEMENTER (glob-mønstre, kommaseparert)
settings_excluded_items_placeholder = f.eks *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = TILLATTE FILENDELSER (tomt = alle)
settings_allowed_extensions_placeholder = f.eks jpg, png, mp4
settings_excluded_extensions = EKSKLUDERTE FILENDELSER
settings_excluded_extensions_placeholder = f.eks bak, tmp, logg
# Settings - Tools section labels
settings_duplicates_header = DUPLIKATER
settings_check_method_label = SAMMENLIGNINGSMETODE
settings_check_method = Metode
settings_hash_type_label = HASH-TYPE
settings_hash_type = Hash-type
settings_hash_type_desc = Blake3 - anbefalt alternativ, CRC32 har liten sjanse for falske positiver
settings_similar_images_header = LIGNENDE BILDER
settings_similarity_preset = Likhetsterskel
settings_similarity_desc = Svært høy = bare nær-identisk
settings_hash_size = Størrelse på hash
settings_hash_size_desc = Større størrelse gir færre falske positiver, men finner også færre resultater
settings_hash_alg = Hash-algoritme
settings_image_filter = Skaleringsfilter
settings_geometric_invariance = Geometrisk invarians
settings_ignore_same_size = Ignorer bilder med samme dimensjoner
settings_gallery_image_fit_cover = Galleri: beskjær til firkant
settings_gallery_image_fit_cover_desc = Fyll ruten; deaktiver for å beholde opprinnelige forhold
settings_big_files_header = STØRSTE FILER
settings_search_mode = Søkemodus
settings_file_count = Antall filer
settings_same_music_header = MUSIKKDUPLIKATER
settings_music_check_method = Sammenligningsmodus
settings_music_compare_tags_label = SAMMENLIGNEDE TAGGER
settings_music_title = Tittel
settings_music_artist = Kunstner
settings_music_year = År
settings_music_length = Lengde
settings_music_genre = Sjanger
settings_music_bitrate = Bithastighet
settings_music_approx = Omtrentlig sammenligning av tagger
settings_temporary_files_header = TEMPORÆRE FILER
settings_temporary_files_extensions_label = FILENDELSER
settings_temporary_files_extensions_placeholder = f.eks. .tmp, .bak, ~
settings_temporary_files_reset = Tilbakestill til standard
settings_broken_files_header = ØDELAGTE FILER
settings_broken_files_note = Ressurskrevende skanning. For best ytelse, bruk Krokiet på skrivebord.
settings_broken_files_types_label = SJEKKEDE TYPER
settings_broken_audio = Lyd
settings_broken_pdf = PDF
settings_broken_archive = Arkiv
settings_broken_image = Bilde
settings_broken_font = Skrifttype
settings_broken_markup = Markup (JSON/XML/TOML)
settings_similar_videos_header = LIGNENDE VIDEOER (LYD)
settings_similar_videos_audio_preset = Forhåndsinnstilling for lydlikhet
settings_similar_videos_audio_preset_desc = Kontrollerer hvor strengt lyden må samsvare
settings_bad_names_header = DÅRLIGE NAVN
settings_bad_names_checks_label = SJEKKER
settings_bad_names_uppercase_ext = Filendelse med store bokstaver
settings_bad_names_emoji = Emoji i navn
settings_bad_names_space = Mellomrom ved start/slutt
settings_bad_names_non_ascii = Ikke-ASCII-tegn
settings_bad_names_duplicated = Gjentatte tegn
settings_ignore_same_resolution = Ignorer bilder med samme oppløsning
# Settings - Appearance section
settings_appearance_label = UTSEENDE
settings_dark_theme = Mørkt tema
settings_dark_theme_desc = Bruk mørkt fargevalg
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIKK
diagnostics_thumbnails = Hurtigbuffer for miniatyrbilder
diagnostics_app_cache = App-hurtigbuffer
diagnostics_refresh = Oppdater
diagnostics_clear_thumbnails = Tøm miniatyrbilder
diagnostics_open_thumbnails_folder = Åpne mappe
diagnostics_clear_cache = Tøm hurtigbufferen
diagnostics_open_cache_folder = Åpne mappe
diagnostics_export_logs = Eksporter logger
logs_label = LOGGER
logs_export_title = Eksporter logger
logs_export_saved = Logger kopiert til:
logs_export_failed = Kunne ikke eksportere logger
diagnostics_collect_test = Filtilgangstest
diagnostics_collect_test_desc = Sjekk hvor mange filer som er tilgjengelige
diagnostics_collect_test_run = Kjør
diagnostics_collect_test_stop = Stopp
collect_test_cancelled = Stoppet av bruker
diag_confirm_clear_thumbnails = Slett all hurtigbuffer for miniatyrbilder?
diag_confirm_clear_cache = Tømme all app-hurtigbuffer?
about_repo = Kodelager
about_translate = Oversettelser
about_donate = Doner
# Collect-test result popup
collect_test_title = Testresultater
collect_test_volumes = Volumer:
collect_test_folders = Mapper:
collect_test_files = Filer:
collect_test_time = Tid:
# Licenses
licenses_label = LISENS
third_party_licenses = Tredjepartslisenser
licenses_popup_title = Tredjepartslisenser
# Directories screen
directories_include_header = Inkluder
directories_included = Inkludert
directories_exclude_header = Ekskluder
directories_excluded_header = Ekskludert
directories_add = Inkluder
no_paths = Ingen stier - legg til under
directories_volume_header = Volumer
directories_volume_refresh = Oppdater
directories_volume_add = Legg til
# Bottom navigation
nav_home = Start
nav_dirs = Mapper
nav_settings = Innstillinger
# Status messages set from Rust
status_ready = Klar
status_stopped = Stoppet
status_no_results = Ingen resultater
status_deleted_selected = Slettet valgte
status_deleted_with_errors = Slettet med feil
scan_not_started = Skann ikke startet
found_items_prefix = Funnet
found_items_suffix = elementer
deleted_items_prefix = Slettet
deleted_items_suffix = elementer
deleted_errors_suffix = feil
renamed_prefix = Endret navn
renamed_files_suffix = filer
renamed_errors_suffix = feil
cleaned_exif_prefix = Renset EXIF fra
cleaned_exif_suffix = filer
cleaned_exif_errors_suffix = feil
rename_error_read_file_name = Kan ikke lese filnavn
rename_error_read_directory = Kan ikke lese mappen
and_more_prefix = ...og
and_more_suffix = mer
# Gallery / delete popups
gallery_delete_button = Slett
gallery_back = Tilbake
gallery_confirm_delete = Ja, slett
deleting_files = Sletter filer...
stop = Stopp
scanning_fallback = Skanner...
app_subtitle = Til ære for slaget ved Cedynia (972 CE)
app_license = Frontend for Czkawka Core - GPL-3.0
about_app_label = OM
cache_label = HURTIGBUFFER
# Notification
scan_completed_notification = Skanning fullført - { $file_count } elementer funnet
# Confirm popups (set from Rust)
confirm_clean_exif = Er du sikker på at du vil rense EXIF tagger fra { $n } valgte filer?
confirm_delete_items = Er du sikker på at du vil slette { $n } valgte elementer?
gallery_confirm_delete_msg = Du er i ferd med å slette { $total_images } bilder i { $total_groups } grupper.
gallery_confirm_delete_warning = Alle elementer er valgt i { $unsafe_groups } grupper!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Beregning og sammenligning av lydfingeravtrykk er svært ressurskrevende og kan ta lang tid. Det anbefales å bruke Krokiet på et skrivebordssystem for denne oppgaven.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } filer x { $per_file } / fil = { $total } totalt
similar_images_group_header = { $count } lignende bilder
same_music_group_header = { $count } lignende spor
similar_videos_group_header = { $count } lignende videoer
# Rename confirmation
confirm_rename_items = Er du sikker på at du vil endre navnet på { $n } valgte filer?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Størst
option_search_mode_smallest = Minste
option_similarity_very_high = V.Høy
option_similarity_high = Høy
option_similarity_medium = Middels
option_similarity_low = Lav
option_similarity_very_low = V.Lav
option_similarity_minimal = Minimum
option_check_method_hash = Hash
option_check_method_name = Navn
option_check_method_size_and_name = Størrelse+Navn
option_check_method_size = Størrelse
option_music_method_tags = Tagger
option_music_method_audio = Lyd
option_min_size_none = Ingen
option_max_size_unlimited = Ubegrenset
option_audio_preset_identical = Identisk
option_audio_preset_clip = Klipp i lengre
option_audio_preset_similar = Lignende
# Volume labels (shown in the directories screen)
volume_internal_storage = Intern lagring
volume_sd_card = Minnekort (SD-kort)
volume_storage = Lagringsvolum
# Directories screen
directories_referenced_tooltip = Referert (ikke slettet)
directories_include_section_header = INKLUDERT
directories_exclude_section_header = EKSKLUDERT
directories_custom_paths = Tilpassede stier
directories_check_button = Analyser
directories_check_popup_title = Katalogstatistikk
directories_check_label_included = Inkluderte stier:
directories_check_label_excluded = Ekskluderte stier:
directories_check_label_referenced = Referansestier:
directories_check_label_would_scan = Filer å skanne:
directories_check_label_processable = Behandlingsbare filer:
directories_check_scanning = Skanner...
directories_check_warning_no_processable = Ingen behandlingsbare filer funnet - kontroller dine inkluderte/ekskluderte mapper
path_edit_title_include = Legg til i Inkluder
path_edit_title_exclude = Legg til i Ekskluder
path_edit_placeholder = Angi sti...
path_edit_not_exists = Stien finnes ikke
path_edit_is_dir = Katalog
path_edit_is_file = Fil
path_edit_no_newlines = Stier kan ikke inneholde linjeskift - Enter-tasten er ikke tillatt
ctx_menu_title = Åpne
ctx_open_file = Åpne element
ctx_open_folder = Åpne overordnet mappe
dir_open_folder = Åpne mappe
# Compare view
compare_label = Sammenlign
compare_loading = Laster bilder...
compare_cancelling = Avbryter...
compare_computing = Beregner diff...
compare_mode_normal = Side
compare_mode_split = Splitt
compare_mode_overlay = Overlegg
compare_mode_diff = Diff
compare_res_mismatch = Forskjellige oppløsninger - diff kan være unøyaktig
