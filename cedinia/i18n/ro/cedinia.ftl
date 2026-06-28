# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplicate
tool_empty_folders = Golire dosare
tool_similar_images = Imagini similare
tool_empty_files = Fișiere goale
tool_temporary_files = Fișiere temporare
tool_big_files = Fișiere mai mari
tool_broken_files = Fișiere defecte
tool_bad_extensions = Extensii rele
tool_same_music = Duplicate Muzică
tool_bad_names = Nume greșit
tool_exif_remover = Date EXIF
tool_similar_videos = Videoclipuri similare (Audio)
tool_directories = Directoare
tool_settings = Setări
# Home screen tool card descriptions
home_dup_description = Găsește fișiere cu același conținut
home_empty_folders_description = Dosare fără conținut
home_similar_images_description = Găsiți fotografii vizuale similare
home_empty_files_description = Fișiere cu dimensiune zero
home_temp_files_description = Fișiere temporare și în cache
home_big_files_description = Fişiere mai mici pe disc
home_broken_files_description = PDF, audio, imagini, arhive
home_bad_extensions_description = Fișiere cu extensie invalidă
home_same_music_description = Fişiere audio similare după etichete
home_bad_names_description = Fișiere cu caractere problematice în nume
home_exif_description = Imagini cu metadate EXIF
home_similar_videos_description = Găsiți videoclipuri cu audio similar
# Results list
scanning = Scanare în curs...
stopping = Oprire...
no_results = Niciun rezultat
press_start = Apăsați START pentru a scana
select_label = Sel.
deselect_label = Desel.
list_label = Listă
gallery_label = Gal
# Selection popup
selection_popup_title = Selectare
select_all = Selectează tot
select_except_one = Selectează toate cu excepția uneia
select_except_largest = Selectează toate cu excepția celui mai mare
select_except_smallest = Selectează toate cu excepția celor mici
select_largest = Selectează cel mai mare
select_smallest = Selectează cel mai mic
select_except_highest_res = Selectează toate cu excepția rezoluției maxime
select_except_lowest_res = Selectează toate cu excepția celei mai mici rezoluții
select_highest_res = Selectează cea mai mare rezoluție
select_lowest_res = Selectează cea mai mică rezoluție
invert_selection = Inversează selecția
close = Inchide
# Deselection popup
deselection_popup_title = Deselectează
deselect_all = Deselectează tot
deselect_except_one = Deselectează toate cu o singură excepție
# Confirm popup
cancel = Anulează
delete = Ștergere
rename = Redenumire
# Delete errors popup
delete_errors_title = Nu s-a putut șterge unele fișiere:
ok = Ok
# Stopping overlay
stopping_overlay_title = Oprire
stopping_overlay_body =
    Se termină scanarea curentă...
    Vă rugăm să așteptați.
# Permission popup
permission_title = Acces fișier
permission_body = Pentru a scana fișierele, aplicația are nevoie de acces la memoria dispozitivului. Fără această permisiune, scanarea nu va fi posibilă.
grant = Acordă
no_permission_scan_warning = Fără acces la fișier - acordă permisiunea de a scana
# Settings screen tabs
settings_tab_general = Generalități
settings_tab_tools = Unelte
settings_tab_diagnostics = Informații
# Settings - General tab
settings_use_cache = Utilizare geocutie
settings_use_cache_desc = Accelerează scanările ulterioare (hash/imagini)
settings_ignore_hidden = Ignoră fișierele ascunse
settings_ignore_hidden_desc = Fișiere și dosare care încep cu ''
settings_show_notification = Notifică când scanarea se termină
settings_show_notification_desc = Arată o notificare de sistem la finalizarea scanării
settings_notify_only_background = Numai când în fundal
settings_notify_only_background_desc = Sari peste notificare dacă aplicația este vizibilă
notifications_disabled_banner = Notificări dezactivate
notifications_enable_button = Activare
settings_scan_label = SCANARE
settings_filters_label = FILTRELE (unele unelte)
settings_min_file_size = Dimensiune minimă fișier
settings_max_file_size = Mărimea maximă a fișierului
settings_language = Limba
settings_language_restart = Necesită repornirea aplicației
settings_common_label = SETĂRI COMUNE
settings_excluded_items = OBIECTE EXCLUZATE (modele de virgulă, separate prin virgulă)
settings_excluded_items_placeholder = ex. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EXTENSIUNI ADMISE (goale = toate)
settings_allowed_extensions_placeholder = de ex. jpg, png, mp4
settings_excluded_extensions = EXTENSIUNI EXCLUSE
settings_excluded_extensions_placeholder = ex. coacă, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLICAȚII
settings_check_method_label = METODĂ DE COMPARARE
settings_check_method = Metodă
settings_hash_type_label = TIPUL HASH
settings_hash_type = Tip hash
settings_hash_type_desc = Blake3 - este recomandată opțiunea, CRC32 are șanse mici de a avea efecte pozitive false
settings_similar_images_header = IMAGINI SIMILARE
settings_similarity_preset = Prag similar
settings_similarity_desc = Foarte mare = numai aproape identic
settings_hash_size = Dimensiune hash
settings_hash_size_desc = Mărimile mai mari au mai puține rezultate pozitive false, dar găsesc și imagini mai puțin similare
settings_hash_alg = Algoritm hash
settings_image_filter = Redimensionare filtru
settings_geometric_invariance = Invarianță geometrică
settings_ignore_same_size = Ignoră imaginile cu aceleași dimensiuni
settings_gallery_image_fit_cover = Galerie: recoltă în pătrat
settings_gallery_image_fit_cover_desc = Umpleți dala; dezactivați pentru a păstra raportul original de aspect
settings_big_files_header = FILURI BIGGEST
settings_search_mode = Mod căutare
settings_file_count = Număr de fișiere
settings_same_music_header = DUPLICAȚII MUSIC
settings_music_check_method = Mod de comparaţie
settings_music_compare_tags_label = TAGURI COMPAATE
settings_music_title = Titlu
settings_music_artist = Artist
settings_music_year = An
settings_music_length = Lungime
settings_music_genre = Gen
settings_music_bitrate = Debit de biți
settings_music_approx = Comparare aproximativă a etichetelor
settings_temporary_files_header = FILURI TEMPORARE
settings_temporary_files_extensions_label = EXTENSIUNI
settings_temporary_files_extensions_placeholder = e.g. .tmp, .bak, ~
settings_temporary_files_reset = Resetare la valorile implicite
settings_broken_files_header = FILURI BROKEN
settings_broken_files_note = Scanare intensivă de resurse. Pentru cea mai bună performanță folosește Krokiet pe desktop.
settings_broken_files_types_label = TIPURI VERIFICATE
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Arhivează
settings_broken_image = Imagine
settings_broken_font = Font
settings_broken_markup = Marcaj (JSON/XML/TOML)
settings_similar_videos_header = VIDEO SIMILAR (AUDIO)
settings_similar_videos_audio_preset = Presetare asemănare audio
settings_similar_videos_audio_preset_desc = Controlează cât de strict audio trebuie să se potrivească
settings_bad_names_header = NAMELE BAD
settings_bad_names_checks_label = CONTROALE
settings_bad_names_uppercase_ext = Extensie cu majuscule
settings_bad_names_emoji = Emoji în nume
settings_bad_names_space = Spaţii la start/end
settings_bad_names_non_ascii = Caractere non-ASCII
settings_bad_names_duplicated = Caractere repetate
settings_ignore_same_resolution = Ignoră imaginile cu aceeași rezoluție
# Settings - Appearance section
settings_appearance_label = APARENȚĂ
settings_dark_theme = Temă întunecată
settings_dark_theme_desc = Utilizaţi o schemă de culori întunecată
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTICE
diagnostics_thumbnails = Cache pentru miniaturi
diagnostics_app_cache = Geocutie aplicaţie
diagnostics_refresh = Împrospătează
diagnostics_clear_thumbnails = Șterge miniaturile
diagnostics_open_thumbnails_folder = Deschide dosarul
diagnostics_clear_cache = Golește memoria cache
diagnostics_open_cache_folder = Deschide dosarul
diagnostics_export_logs = Exportă jurnalele
logs_label = Loguri
logs_export_title = Exportă jurnalele
logs_export_saved = Jurnale copiate în:
logs_export_failed = Jurnalele nu au putut fi exportate
diagnostics_collect_test = Test acces fișier
diagnostics_collect_test_desc = Verificați câte fișiere sunt accesibile
diagnostics_collect_test_run = Rulează
diagnostics_collect_test_stop = Oprește
collect_test_cancelled = Oprită de utilizator
diag_confirm_clear_thumbnails = Ştergeţi toate cutiile miniaturale?
diag_confirm_clear_cache = Șterge tot cache-ul aplicației?
about_repo = Depozit
about_translate = Traduceri
about_donate = Suport
# Collect-test result popup
collect_test_title = Rezultatele testului
collect_test_volumes = Volume:
collect_test_folders = Dosare:
collect_test_files = Fișiere:
collect_test_time = Ora:
# Licenses
licenses_label = LICENŢĂ
third_party_licenses = Licențe pentru terți
licenses_popup_title = Licențe terțe
# Directories screen
directories_include_header = Includeți
directories_included = Inclus
directories_exclude_header = Excludere
directories_excluded_header = Exclus
directories_add = Includeți
no_paths = Nici o cale - adaugă mai jos
directories_volume_header = Volume
directories_volume_refresh = Împrospătează
directories_volume_add = Adăugare
# Bottom navigation
nav_home = Pornire
nav_dirs = Directoare
nav_settings = Setări
# Status messages set from Rust
status_ready = Gata
status_stopped = Oprit
status_no_results = Niciun rezultat
status_deleted_selected = Sters selectat
status_deleted_with_errors = Șters cu erori
scan_not_started = Scanarea nu a început
found_items_prefix = Găsit
found_items_suffix = articole
deleted_items_prefix = Șters
deleted_items_suffix = articole
deleted_errors_suffix = erori
renamed_prefix = Redenumit
renamed_files_suffix = fişiere
renamed_errors_suffix = erori
cleaned_exif_prefix = Curățare EXIF din
cleaned_exif_suffix = fişiere
cleaned_exif_errors_suffix = erori
rename_error_read_file_name = Nu se poate citi numele fișierului
rename_error_read_directory = Directorul nu poate fi citit
and_more_prefix = ...și
and_more_suffix = mai mult
# Gallery / delete popups
gallery_delete_button = Ștergere
gallery_back = Înapoi
gallery_confirm_delete = Da, șterge
deleting_files = Ștergere fișiere...
stop = Oprește
scanning_fallback = Scanare...
app_subtitle = În onoarea Bătăliei din Cedynia (972 CE)
app_license = Frontend pentru Czkawka Core - GPL-3.0
about_app_label = DESPRE
cache_label = CACHE
# Notification
scan_completed_notification = Scanare completă - { $file_count } elemente găsite
# Confirm popups (set from Rust)
confirm_clean_exif = Sunteţi sigur că doriţi să ştergeţi etichetele EXIF din { $n } selectate?
confirm_delete_items = Sunteţi sigur că doriţi să ştergeţi { $n } elementele selectate?
gallery_confirm_delete_msg = Sunteţi pe cale să ştergeţi imaginile { $total_images } în grupurile { $total_groups }.
gallery_confirm_delete_warning = Toate elementele sunt selectate în grupele { $unsafe_groups }!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Calcularea și compararea amprentelor audio necesită foarte multe resurse și poate dura mult timp. Este recomandat să utilizaţi Krokiet pe un calculator pentru această sarcină.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } fişiere x { $per_file } / fişier = { $total } total
similar_images_group_header = { $count } imagini similare
same_music_group_header = { $count } piese similare
similar_videos_group_header = { $count } videoclipuri similare
# Rename confirmation
confirm_rename_items = Sunteţi sigur că doriţi să redenumiţi { $n } fişierele selectate?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Miggest
option_search_mode_smallest = Mici
option_similarity_very_high = V.Înaltă
option_similarity_high = Ridicat
option_similarity_medium = Medie
option_similarity_low = scazut
option_similarity_very_low = V. Foarte scăzut
option_similarity_minimal = Minim
option_check_method_hash = Hash
option_check_method_name = Nume
option_check_method_size_and_name = Dimensiune+Nume
option_check_method_size = Dimensiune
option_music_method_tags = Etichete
option_music_method_audio = Audio
option_min_size_none = Niciunul
option_max_size_unlimited = Nelimitat
option_audio_preset_identical = Identic
option_audio_preset_clip = Clip mai lung
option_audio_preset_similar = Similare
# Volume labels (shown in the directories screen)
volume_internal_storage = Stocare internă
volume_sd_card = Card de memorie (SD Card)
volume_storage = Volum stocare
# Directories screen
directories_referenced_tooltip = Referință (nu ștearsă)
directories_include_section_header = INCLUSE
directories_exclude_section_header = EXCLUZĂ
directories_custom_paths = Căi personalizate
directories_check_button = Analizează
directories_check_popup_title = Statistici Director
directories_check_label_included = Căi incluse:
directories_check_label_excluded = Căi excluse:
directories_check_label_referenced = Căi de referință:
directories_check_label_would_scan = Fișiere de scanat:
directories_check_label_processable = Fișiere procesabile:
directories_check_scanning = Scanare...
directories_check_warning_no_processable = Nu au fost găsite fișiere procesabile - verifică dosarele incluse/excluse
path_edit_title_include = Adaugă la Includere
path_edit_title_exclude = Adaugă la Excludere
path_edit_placeholder = Introduceți calea...
path_edit_not_exists = Calea nu există
path_edit_is_dir = Director
path_edit_is_file = Fişier
path_edit_no_newlines = Căile nu pot conține noutăți - cheia nu este permisă
ctx_menu_title = Deschideți
ctx_open_file = Deschideți articolul
ctx_open_folder = Deschide dosarul părinte
dir_open_folder = Deschide dosarul
# Compare view
compare_label = Compară
compare_loading = Se încarcă imaginile…
compare_cancelling = Se anulează…
compare_computing = Calculare diff…
compare_mode_normal = Lateral
compare_mode_split = Împarte
compare_mode_overlay = Suprapunere
compare_mode_diff = Dif
compare_res_mismatch = Diferite rezoluții - diff poate fi inexact
