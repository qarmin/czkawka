# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplicaten
tool_empty_folders = Lege mappen
tool_similar_images = Vergelijkbare afbeeldingen
tool_empty_files = Lege bestanden
tool_temporary_files = Tijdelijke bestanden
tool_big_files = Grootste bestanden
tool_broken_files = Kapotte Bestanden
tool_bad_extensions = Slechte extensies
tool_same_music = Muziek duplicaten
tool_bad_names = Slechte namen
tool_exif_remover = EXIF gegevens
tool_directories = Mappen
tool_settings = Instellingen
# Home screen tool card descriptions
home_dup_description = Bestanden met dezelfde inhoud zoeken
home_empty_folders_description = Mappen zonder inhoud
home_similar_images_description = Vind visueel vergelijkbare foto's
home_empty_files_description = Bestanden met nul grootte
home_temp_files_description = Tijdelijke en opgeslagen bestanden
home_big_files_description = Grootste/Kleinste bestanden op schijf
home_broken_files_description = PDF, audio, afbeeldingen, archieven
home_bad_extensions_description = Bestanden met ongeldige extensie
home_same_music_description = Vergelijkbare audiobestanden via tags
home_bad_names_description = Bestanden met problematische tekens in naam
home_exif_description = Afbeeldingen met EXIF metadata
# Results list
scanning = Scannen in uitvoering...
stopping = Stoppen...
no_results = Geen resultaten gevonden
press_start = Druk op START om te scannen
select_label = Zel.
deselect_label = Desel.
list_label = Klantenlijst
gallery_label = Gal.
# Selection popup
selection_popup_title = Selecteren
select_all = Alles selecteren
select_except_one = Alles behalve één selecteren
select_except_largest = Selecteer alles behalve de grootste
select_except_smallest = Selecteer alles behalve de kleinste
select_largest = Selecteer grootste
select_smallest = Selecteer kleinste
select_except_highest_res = Alles selecteren behalve de hoogste resolutie
select_except_lowest_res = Alles behalve de laagste resolutie selecteren
select_highest_res = Hoogste resolutie selecteren
select_lowest_res = Selecteer de laagste resolutie
invert_selection = Selectie omkeren
close = Afsluiten
# Deselection popup
deselection_popup_title = Deselecteer
deselect_all = Deselecteer alles
deselect_except_one = Deselecteer alles behalve één
# Confirm popup
cancel = annuleren
delete = Verwijderen
rename = Hernoem
# Delete errors popup
delete_errors_title = Kan sommige bestanden niet verwijderen:
ok = Ok
# Stopping overlay
stopping_overlay_title = Stoppen
stopping_overlay_body =
    Beëindig huidige scan...
    Een ogenblik geduld.
# Permission popup
permission_title = Toegang tot bestand
permission_body = Om bestanden te scannen, moet de app toegang hebben tot de opslag van het apparaat. Zonder deze toestemming is scannen niet mogelijk.
grant = Toestaan
no_permission_scan_warning = Geen bestandstoegang - geef toestemming om te scannen
# Settings screen tabs
settings_tab_general = Algemeen
settings_tab_tools = Hulpmiddelen
settings_tab_diagnostics = Informatie
# Settings - General tab
settings_use_cache = Gebruik cache
settings_use_cache_desc = Versnelt volgende scans (hash/images)
settings_ignore_hidden = Verborgen bestanden negeren
settings_ignore_hidden_desc = Bestanden en mappen beginnend met '.'
settings_show_notification = Melden wanneer scan is voltooid
settings_show_notification_desc = Toon een systeemmelding bij voltooiing van de scan
settings_notify_only_background = Alleen als op de achtergrond
settings_notify_only_background_desc = Melding overslaan als de app zichtbaar is
notifications_disabled_banner = Meldingen uitgeschakeld
notifications_enable_button = Inschakelen
settings_scan_label = SCANNEREN
settings_filters_label = FILTERS (sommige tools)
settings_min_file_size = Minimale bestandsgrootte
settings_max_file_size = Max. bestandsgrootte
settings_language = Taal
settings_language_restart = Vereist herstart van app
settings_common_label = GEWOON INSTELLINGEN
settings_excluded_items = EXCLUDED ITEMS (glob patronen, komma gescheiden)
settings_excluded_items_placeholder = bijv. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EXTENSIES TOESTAAN (leeg = alles)
settings_allowed_extensions_placeholder = bijv. jpg, png, mp4
settings_excluded_extensions = EXTENSIES UITGEVOERD
settings_excluded_extensions_placeholder = bijv. bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLICATEN
settings_check_method_label = METHOD KOPEN
settings_check_method = Methode
settings_hash_type_label = HASH TYPE
settings_hash_type = Soort hash
settings_hash_type_desc = Blake3 - wordt aangeraden optie, CRC32 hebben een kleine kans op valse positieven
settings_similar_images_header = SIMILAR AFBEELDINGEN
settings_similarity_preset = Drempelwaarde gelijkwaardigheid
settings_similarity_desc = Zeer Hoog = alleen bijna identiek
settings_hash_size = Hash grootte
settings_hash_size_desc = Grotere formaten, hebben minder valse positieven, maar vinden ook minder gelijksoortige afbeeldingen
settings_hash_alg = Hash algoritme
settings_image_filter = Filter aanpassen
settings_ignore_same_size = Negeer afbeeldingen met dezelfde afmetingen
settings_gallery_image_fit_cover = Galerij: bijsnijden tot vierkant
settings_gallery_image_fit_cover_desc = Vul de tegel; schakel uit om de originele hoogte-breedteverhouding te behouden
settings_big_files_header = BIGGESTE BESTANDEN
settings_search_mode = Zoek modus
settings_file_count = Aantal bestanden
settings_same_music_header = MUSICIES DUPLICATEN
settings_music_check_method = Vergelijkingsmodus
settings_music_compare_tags_label = GECOMPAREERDE TAGS
settings_music_title = Aanspreektitel
settings_music_artist = Kunstenaar
settings_music_year = jaar
settings_music_length = longueur
settings_music_genre = genre
settings_music_bitrate = Bitsnelheid
settings_music_approx = Geschatte tag vergelijking
settings_broken_files_header = BROKEN BESTANDEN
settings_broken_files_note = Resource-intensieve scan. Gebruik de desktopversie van Krokiet.
settings_broken_files_types_label = GECHECKEERde TYPEN
settings_broken_audio = Geluid
settings_broken_pdf = PDF-bestand
settings_broken_archive = Archief
settings_broken_image = Afbeelding
settings_bad_names_header = BAD NAAMS
settings_bad_names_checks_label = KIES
settings_bad_names_uppercase_ext = Hoofdletters extensie
settings_bad_names_emoji = Emoji in de naam
settings_bad_names_space = Spaties bij het starten/einde
settings_bad_names_non_ascii = Niet-ASCII-tekens
settings_bad_names_duplicated = Herhaalde tekens
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIËREN
diagnostics_thumbnails = Miniatuurweergavecache
diagnostics_app_cache = App-cache
diagnostics_refresh = Vernieuwen
diagnostics_clear_thumbnails = Miniaturen wissen
diagnostics_open_thumbnails_folder = Map openen
diagnostics_clear_cache = Cache legen
diagnostics_open_cache_folder = Map openen
diagnostics_collect_test = Test bestandstoegang
diagnostics_collect_test_desc = Controleer hoeveel bestanden toegankelijk zijn
diagnostics_collect_test_run = Uitvoeren
diagnostics_collect_test_stop = Stoppen
collect_test_cancelled = Gestopt door gebruiker
diag_confirm_clear_thumbnails = Alle miniatuurcache wissen?
diag_confirm_clear_cache = Alle app-cache wissen?
about_repo = Bewaarplaats
about_translate = Vertalen
about_donate = Ondersteuning
# Collect-test result popup
collect_test_title = Test resultaten
collect_test_volumes = Volumes:
collect_test_folders = Mappen:
collect_test_files = Bestanden:
collect_test_time = Tijd:
# Licenses
licenses_label = LICENTIE
third_party_licenses = Licenties van derden
licenses_popup_title = Licenties van derden
# Directories screen
directories_include_header = Inclusief
directories_included = Inbegrepen
directories_exclude_header = Uitsluiten
directories_excluded_header = Uitgesloten
directories_add = Inclusief
no_paths = Geen paden - voeg hieronder toe
directories_volume_header = Volumes
directories_volume_refresh = Vernieuwen
directories_volume_add = Toevoegen
# Bottom navigation
nav_home = Beginnen
nav_dirs = Mappen
nav_settings = Instellingen
# Status messages set from Rust
status_ready = Klaar
status_stopped = Gestopt
status_no_results = Geen resultaten gevonden
status_deleted_selected = Geselecteerde items verwijderd
status_deleted_with_errors = Verwijderd met fouten
scan_not_started = Scan niet gestart
found_items_prefix = Gevonden
found_items_suffix = artikelen
deleted_items_prefix = Verwijderd
deleted_items_suffix = artikelen
deleted_errors_suffix = fouten
renamed_prefix = Hernoemd
renamed_files_suffix = bestanden
renamed_errors_suffix = fouten
cleaned_exif_prefix = Schoongemaakte EXIF van
cleaned_exif_suffix = bestanden
cleaned_exif_errors_suffix = fouten
and_more_prefix = ...en
and_more_suffix = meer
# Gallery / delete popups
gallery_delete_button = Verwijderen
gallery_back = Achterzijde
gallery_confirm_delete = Ja, verwijderen
deleting_files = Bestanden verwijderen...
stop = Stoppen
files_suffix = bestanden
scanning_fallback = Scannen...
app_subtitle = Ter ere van de slag van Cedynia (972 CE)
app_license = Frontend voor Czkawka Core - GPL-3.0
about_app_label = Over
cache_label = ACHTERZIJDE
# Notification
scan_completed_notification = Scan voltooid - { $file_count } items gevonden
# Confirm popups (set from Rust)
confirm_clean_exif = Weet je zeker dat je EXIF tags van { $n } geselecteerde bestanden wilt wissen?
confirm_delete_items = Weet je zeker dat je { $n } geselecteerde items wilt verwijderen?
gallery_confirm_delete_msg = Je staat op het punt { $total_images } afbeeldingen in { $total_groups } groepen te verwijderen.
gallery_confirm_delete_warning = Alle items zijn geselecteerd in { $unsafe_groups } groepen!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Audio-vingerafdrukken berekenen en vergelijken is erg bronintensief en kan een lange tijd duren. Het wordt aangeraden om Krokiet te gebruiken op een bureaublad voor deze taak.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Bestanden verzamelen
stage_scanning_name = Scannen op naam
stage_scanning_size_name = Scannen op naam en grootte
stage_scanning_size = Scannen op grootte
stage_pre_hash = Voorafgaande hashing
stage_full_hash = Hashing
stage_loading_cache = Cache laden
stage_saving_cache = Cache opslaan
stage_calculating_image_hashes = Berekenen afbeeldingshashes
stage_comparing_images = Afbeeldingen vergelijken
stage_calculating_video_hashes = Video-hashes berekenen
stage_checking_files = Bestanden controleren
stage_checking_extensions = Extensies controleren
stage_checking_names = Namen controleren
stage_reading_music_tags = Muziektags lezen
stage_comparing_tags = Tags vergelijken
stage_calculating_music_fingerprints = Vingerafdrukken van muziek berekenen
stage_comparing_fingerprints = Vingerafdrukken vergelijken
stage_extracting_exif = Lees EXIF tags
stage_creating_video_thumbnails = Video-miniaturen worden aangemaakt
stage_processing_videos = Video's verwerken
stage_deleting = Bestanden verwijderen
stage_renaming = Bestanden hernoemen
stage_moving = Bestanden verplaatsen
stage_hardlinking = harde links maken
stage_symlinking = Symlinks maken
stage_optimizing_videos = Video's optimaliseren
stage_cleaning_exif = ExIFF opruimen
# Group headers in scan results
duplicates_group_header = { $count } bestanden x { $per_file } / bestand = { $total } in totaal
similar_images_group_header = { $count } soortgelijke afbeeldingen
same_music_group_header = { $count } soortgelijke nummers
# Rename confirmation
confirm_rename_items = Weet je zeker dat je { $n } geselecteerde bestanden wilt hernoemen?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Grootste
option_search_mode_smallest = Kleinste
option_similarity_very_high = Hoog
option_similarity_high = hoog
option_similarity_medium = Middelgroot
option_similarity_low = laag
option_similarity_very_low = V.Laag
option_similarity_minimal = min.
option_check_method_hash = Toegangssleutel
option_check_method_name = naam
option_check_method_size_and_name = Grootte+Naam
option_check_method_size = Grootte
option_music_method_tags = Labels
option_music_method_audio = Geluid
option_min_size_none = geen
option_min_size_1kb = 1 kB
option_min_size_8kb = 8 kB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 kB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Onbeperkt
# Volume labels (shown in the directories screen)
volume_internal_storage = Interne opslag
volume_sd_card = Geheugenkaart (SD-kaart)
volume_storage = Volume voor opslag
# Directories screen
directories_referenced_tooltip = Gerefereerd (niet verwijderd)
directories_include_section_header = INCLUDEERD
directories_exclude_section_header = UITGESLOTEN
directories_custom_paths = Aangepaste paden
directories_check_button = Analyseren
directories_check_popup_title = Map statistieken
directories_check_label_included = Inbegrepen paden:
directories_check_label_excluded = Uitgesloten paden:
directories_check_label_referenced = Referentie paden:
directories_check_label_would_scan = Te scannen bestanden:
directories_check_label_processable = Verbruikbare bestanden:
directories_check_scanning = Scannen...
directories_check_warning_no_processable = Geen verwerkbare bestanden gevonden - controleer uw include/uitgesloten mappen
path_edit_title_include = Toevoegen aan opnemen
path_edit_title_exclude = Toevoegen aan Uitsluiten
path_edit_placeholder = Pad invoeren...
path_edit_not_exists = Pad bestaat niet
path_edit_is_dir = Catalogus
path_edit_is_file = Bestand
path_edit_no_newlines = Paden mogen geen nieuwe regels bevatten — Enter key is niet toegestaan
ctx_menu_title = Open
ctx_open_file = Artikel openen
ctx_open_folder = Open bovenliggende map
