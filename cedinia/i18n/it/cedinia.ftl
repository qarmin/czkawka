# Cedinia - English (fallback)

# App / top bar titles
tool_duplicate_files = Duplicati
tool_empty_folders = Cartelle Vuote
tool_similar_images = Immagini Simili
tool_empty_files = File Vuoti
tool_temporary_files = File Temporanei
tool_big_files = File Più Grandi
tool_broken_files = File Danneggiati
tool_bad_extensions = Estensioni Errate
tool_same_music = Duplicati Musicali
tool_bad_names = Nomi Errati
tool_exif_remover = Dati EXIF
tool_similar_videos = Video Simili (Audio)
tool_directories = Directory
tool_settings = Impostazioni
# Home screen tool card descriptions
home_dup_description = Trova file con lo stesso contenuto
home_empty_folders_description = Directory senza contenuto
home_similar_images_description = Trova foto visivamente simili
home_empty_files_description = File con dimensione zero
home_temp_files_description = File temporanei e memorizzati nella cache
home_big_files_description = File più grandi/più piccoli sul disco
home_broken_files_description = PDF, audio, immagini, archivi
home_bad_extensions_description = File con estensione non valida
home_same_music_description = File audio simili per tag
home_bad_names_description = File con caratteri problematici nel nome
home_exif_description = Immagini con metadati EXIF
home_similar_videos_description = Trova video con audio simile
# Results list
scanning = Scansione in corso...
stopping = Arresto...
no_results = Nessun risultato
press_start = Premere START per scansionare
select_label = Sel.
deselect_label = Desel.
list_label = Elenco
gallery_label = Gal
# Selection popup
selection_popup_title = Seleziona
select_all = Seleziona tutto
select_except_one = Seleziona tutto tranne uno
select_except_largest = Seleziona tutto tranne il più grande
select_except_smallest = Seleziona tutto tranne il più piccolo
select_largest = Seleziona il più grande
select_smallest = Seleziona il più piccolo
select_except_highest_res = Seleziona tutto tranne la risoluzione più alta
select_except_lowest_res = Seleziona tutto tranne la risoluzione più bassa
select_highest_res = Seleziona la risoluzione più alta
select_lowest_res = Seleziona la risoluzione più bassa
invert_selection = Inverti selezione
close = Chiudi
# Deselection popup
deselection_popup_title = Deseleziona
deselect_all = Deseleziona tutto
deselect_except_one = Deseleziona tutto tranne uno
# Confirm popup
cancel = Annulla
delete = Elimina
rename = Rinomina
# Delete errors popup
delete_errors_title = Impossibile eliminare alcuni file:
ok = OK
# Stopping overlay
stopping_overlay_title = Arresto
stopping_overlay_body =
    Completamento della scansione in corso...
    Attendere prego.
# Permission popup
permission_title = Accesso ai File
permission_body = Per eseguire la scansione dei file, l'app ha bisogno di accedere all'archiviazione del dispositivo. Senza questo permesso, la scansione non sarà possibile.
grant = Concedi
no_permission_scan_warning = Nessun accesso ai file - concedi l'autorizzazione per la scansione
# Settings screen tabs
settings_tab_general = Generale
settings_tab_tools = Strumenti
settings_tab_diagnostics = Informazioni
# Settings - General tab
settings_use_cache = Usa cache
settings_use_cache_desc = Accelera le scansioni successive (hash/immagini)
settings_ignore_hidden = Ignora i file nascosti
settings_ignore_hidden_desc = File e cartelle che iniziano con '.'
settings_show_notification = Notifica quando la scansione termina
settings_show_notification_desc = Mostra una notifica di sistema al completamento della scansione
settings_notify_only_background = Solo quando in background
settings_notify_only_background_desc = Salta la notifica se l'applicazione è visibile
notifications_disabled_banner = Notifiche disabilitate
notifications_enable_button = Abilita
settings_scan_label = SCANSIONE
settings_filters_label = FILTRI (alcuni strumenti)
settings_min_file_size = Dimensione minima del file
settings_max_file_size = Dimensione massima del file
settings_language = Lingua
settings_language_restart = Richiede il riavvio dell'app
settings_common_label = IMPOSTAZIONI COMUNI
settings_excluded_items = OGGETTI ESCLUSI (pattern glob, separati da virgola)
settings_excluded_items_placeholder = es. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = ESTENSIONI CONSENTITE (vuote = tutte)
settings_allowed_extensions_placeholder = es. jpg, png, mp4
settings_excluded_extensions = ESTENSIONI ESCLUSE
settings_excluded_extensions_placeholder = es. bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLICATI
settings_check_method_label = METODO DI CONFRONTO
settings_check_method = Metodo
settings_hash_type_label = TIPO DI HASH
settings_hash_type = Tipo di hash
settings_hash_type_desc = Blake3 è l'opzione consigliata; CRC32 ha una piccola probabilità di falsi positivi
settings_similar_images_header = IMMAGINI SIMILI
settings_similarity_preset = Soglia di somiglianza
settings_similarity_desc = Molto Alta = solo quasi identiche
settings_hash_size = Dimensione hash
settings_hash_size_desc = Dimensioni più grandi hanno meno falsi positivi, ma trovano anche meno immagini simili
settings_hash_alg = Algoritmo di hash
settings_image_filter = Filtro di ridimensionamento
settings_geometric_invariance = Invarianza geometrica
settings_ignore_same_size = Ignora immagini con le stesse dimensioni
settings_gallery_image_fit_cover = Galleria: ritaglia a quadrato
settings_gallery_image_fit_cover_desc = Riempi la casella; disabilita per mantenere le proporzioni originali
settings_big_files_header = FILE PIÙ GRANDI
settings_search_mode = Modalità di ricerca
settings_file_count = Conteggio file
settings_same_music_header = DUPLICATI MUSICALI
settings_music_check_method = Modalità di confronto
settings_music_compare_tags_label = TAG CONFRONTATI
settings_music_title = Titolo
settings_music_artist = Artista
settings_music_year = Anno
settings_music_length = Lunghezza
settings_music_genre = Genere
settings_music_bitrate = Bitrate
settings_music_approx = Confronto approssimativo dei tag
settings_temporary_files_header = FILE TEMPORANEI
settings_temporary_files_extensions_label = ESTENSIONI
settings_temporary_files_extensions_placeholder = ad esempio: .tmp, .bak, ~
settings_temporary_files_reset = Reimposta impostazioni predefinite
settings_broken_files_header = FILE DANNEGGIATI
settings_broken_files_note = Scansione intensiva di risorse. Per ottenere le migliori prestazioni utilizzare Krokiet sul desktop.
settings_broken_files_types_label = TIPI CONTROLLATI
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Archivio
settings_broken_image = Immagine
settings_broken_font = Carattere
settings_broken_markup = Markup (JSON/XML/TOML)
settings_similar_videos_header = VIDEO SIMILARI (AUDIO)
settings_similar_videos_audio_preset = Preimpostazione somiglianza audio
settings_similar_videos_audio_preset_desc = Controlla quanto rigorosamente l'audio deve corrispondere
settings_bad_names_header = NOMI ERRATI
settings_bad_names_checks_label = CONTROLLI
settings_bad_names_uppercase_ext = Estensione maiuscola
settings_bad_names_emoji = Emoji nel nome
settings_bad_names_space = Spazi all'inizio/fine
settings_bad_names_non_ascii = Caratteri non-ASCII
settings_bad_names_duplicated = Caratteri ripetuti
settings_ignore_same_resolution = Ignora le immagini con la stessa risoluzione
# Settings - Appearance section
settings_appearance_label = ASPETTO
settings_dark_theme = Tema scuro
settings_dark_theme_desc = Usa lo schema di colori scuri
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTICA
diagnostics_thumbnails = Cache delle miniature
diagnostics_app_cache = Cache app
diagnostics_refresh = Aggiorna
diagnostics_clear_thumbnails = Cancella miniature
diagnostics_open_thumbnails_folder = Apri cartella
diagnostics_clear_cache = Svuota cache
diagnostics_open_cache_folder = Apri cartella
diagnostics_export_logs = Esporta log
logs_label = LOG
logs_export_title = Esporta log
logs_export_saved = Log copiati a:
logs_export_failed = Impossibile esportare i log
diagnostics_collect_test = Test di accesso al file
diagnostics_collect_test_desc = Controlla quanti file sono accessibili
diagnostics_collect_test_run = Esegui
diagnostics_collect_test_stop = Interrompi
collect_test_cancelled = Fermato dall'utente
diag_confirm_clear_thumbnails = Cancellare tutta la cache delle miniature?
diag_confirm_clear_cache = Cancellare tutta la cache dell'app?
about_repo = Repository
about_translate = Traduzioni
about_donate = Dona
# Collect-test result popup
collect_test_title = Risultati della prova
collect_test_volumes = Volumi:
collect_test_folders = Cartelle:
collect_test_files = File:
collect_test_time = Tempo:
# Licenses
licenses_label = LICENZA
third_party_licenses = Licenze di terze parti
licenses_popup_title = Licenze di terze parti
# Directories screen
directories_include_header = Includi
directories_included = Incluso
directories_exclude_header = Escludi
directories_excluded_header = Escluso
directories_add = Includi
no_paths = Nessun percorso - aggiungi sotto
directories_volume_header = Volumi
directories_volume_refresh = Aggiorna
directories_volume_add = Aggiungi
# Bottom navigation
nav_home = Inizio
nav_dirs = Directory
nav_settings = Impostazioni
# Status messages set from Rust
status_ready = Pronto
status_stopped = Fermato
status_no_results = Nessun risultato
status_deleted_selected = Eliminato selezionato
status_deleted_with_errors = Eliminato con errori
scan_not_started = Scansione non avviata
found_items_prefix = Trovati
found_items_suffix = elementi
deleted_items_prefix = Eliminati
deleted_items_suffix = elementi
deleted_errors_suffix = errori
renamed_prefix = Rinominati
renamed_files_suffix = file
renamed_errors_suffix = errori
cleaned_exif_prefix = EXIF pulito da
cleaned_exif_suffix = file
cleaned_exif_errors_suffix = errori
rename_error_read_file_name = Impossibile leggere il nome del file
rename_error_read_directory = Impossibile leggere la directory
and_more_prefix = ...e
and_more_suffix = altri
# Gallery / delete popups
gallery_delete_button = Elimina
gallery_back = Indietro
gallery_confirm_delete = Sì, elimina
deleting_files = Eliminazione file...
stop = Interrompi
scanning_fallback = Scansione...
app_subtitle = In onore della battaglia di Cedynia (972 CE)
app_license = Frontend per Czkawka Core - GPL-3.0
about_app_label = INFO
cache_label = CACHE
# Notification
scan_completed_notification = Scansione completata - { $file_count } elementi trovati
# Confirm popups (set from Rust)
confirm_clean_exif = Sei sicuro di voler pulire i tag EXIF da { $n } file selezionati?
confirm_delete_items = Sei sicuro di voler eliminare { $n } elementi selezionati?
gallery_confirm_delete_msg = Stai per eliminare { $total_images } immagini in { $total_groups } gruppi.
gallery_confirm_delete_warning = Tutti gli elementi sono selezionati in { $unsafe_groups } gruppi!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Il calcolo e il confronto delle impronte digitali audio richiedono molte risorse e possono richiedere molto tempo. Si consiglia di usare Krokiet su un sistema desktop per questa attività.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } file x { $per_file } / file = { $total } totale
similar_images_group_header = { $count } immagini simili
same_music_group_header = { $count } tracce simili
similar_videos_group_header = { $count } video simili
# Rename confirmation
confirm_rename_items = Sei sicuro di voler rinominare { $n } file selezionati?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Più Grande
option_search_mode_smallest = Più Piccolo
option_similarity_very_high = V.Alto
option_similarity_high = Alto
option_similarity_medium = Medio
option_similarity_low = Basso
option_similarity_very_low = V.Basso
option_similarity_minimal = Min
option_check_method_hash = Hash
option_check_method_name = Nome
option_check_method_size_and_name = Dimensione+Nome
option_check_method_size = Dimensione
option_music_method_tags = Tag
option_music_method_audio = Audio
option_min_size_none = Nessuno
option_max_size_unlimited = Illimitato
option_audio_preset_identical = Identico
option_audio_preset_clip = Clip nel più lungo
option_audio_preset_similar = Simile
# Volume labels (shown in the directories screen)
volume_internal_storage = Memoria Interna
volume_sd_card = Scheda di Memoria (Scheda SD)
volume_storage = Volume di Archiviazione
# Directories screen
directories_referenced_tooltip = Referenziato (non eliminato)
directories_include_section_header = INCLUSO
directories_exclude_section_header = ESCLUSO
directories_custom_paths = Percorsi Personalizzati
directories_check_button = Analizza
directories_check_popup_title = Statistiche Directory
directories_check_label_included = Percorsi inclusi:
directories_check_label_excluded = Percorsi esclusi:
directories_check_label_referenced = Percorsi di riferimento:
directories_check_label_would_scan = File da scansionare:
directories_check_label_processable = File processabili:
directories_check_scanning = Scansione...
directories_check_warning_no_processable = Nessun file processabile trovato: verifica le cartelle incluse/escluse
path_edit_title_include = Aggiungi a Includi
path_edit_title_exclude = Aggiungi a Escludi
path_edit_placeholder = Inserisci percorso...
path_edit_not_exists = Il percorso non esiste
path_edit_is_dir = Directory
path_edit_is_file = File
path_edit_no_newlines = I percorsi non possono contenere interruzioni di riga - il tasto Invio non è consentito
ctx_menu_title = Apri
ctx_open_file = Apri elemento
ctx_open_folder = Apri cartella superiore
dir_open_folder = Apri cartella
# Compare view
compare_label = Confronta
compare_loading = Caricamento immagini...
compare_cancelling = Annullamento...
compare_computing = Calcolo diff...
compare_mode_normal = Affiancato
compare_mode_split = Divisione
compare_mode_overlay = Sovrapposizione
compare_mode_diff = Diff
compare_res_mismatch = Risoluzioni diverse - diff può essere impreciso
