# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Doublons
tool_empty_folders = Dossiers vides
tool_similar_images = Images similaires
tool_empty_files = Fichiers vides
tool_temporary_files = Fichiers temporaires
tool_big_files = Plus gros fichiers
tool_broken_files = Fichiers cassés
tool_bad_extensions = Mauvaises extensions
tool_same_music = Doublons de musique
tool_bad_names = Mauvais noms
tool_exif_remover = Données EXIF
tool_directories = Répertoires
tool_settings = Réglages
# Home screen tool card descriptions
home_dup_description = Trouver des fichiers avec le même contenu
home_empty_folders_description = Répertoires sans contenu
home_similar_images_description = Trouver des photos similaires visuellement
home_empty_files_description = Fichiers avec zéro taille
home_temp_files_description = Fichiers temporaires et mis en cache
home_big_files_description = Fichiers les plus gros / les plus petits sur le disque
home_broken_files_description = PDF, audio, images, archives
home_bad_extensions_description = Fichiers avec extension invalide
home_same_music_description = Fichiers audio similaires par tags
home_bad_names_description = Fichiers avec des caractères problématiques dans le nom
home_exif_description = Images avec métadonnées EXIF
# Results list
scanning = Analyse en cours...
stopping = Arrêt...
no_results = Aucun résultat
press_start = Appuyez sur START pour analyser
select_label = Sel.
deselect_label = Desel.
list_label = Liste
gallery_label = Gal.
# Selection popup
selection_popup_title = Sélectionner
select_all = Tout sélectionner
select_except_one = Tout sélectionner sauf un
select_except_largest = Tout sélectionner sauf le plus grand
select_except_smallest = Tout sélectionner sauf le plus petit
select_largest = Sélectionner le plus grand
select_smallest = Sélectionner le plus petit
select_except_highest_res = Tout sélectionner sauf la résolution la plus élevée
select_except_lowest_res = Tout sélectionner sauf la résolution la plus basse
select_highest_res = Sélectionnez la résolution la plus élevée
select_lowest_res = Sélectionnez la résolution la plus basse
invert_selection = Inverser la sélection
close = Fermer
# Deselection popup
deselection_popup_title = Désélectionner
deselect_all = Désélectionner tout
deselect_except_one = Tout désélectionner sauf un
# Confirm popup
cancel = Abandonner
delete = Supprimez
rename = Renommer
# Delete errors popup
delete_errors_title = Impossible de supprimer certains fichiers:
ok = Ok
# Stopping overlay
stopping_overlay_title = Arrêt en cours
stopping_overlay_body =
    Fin de l'analyse en cours...
    Veuillez patienter.
# Permission popup
permission_title = Accès au fichier
permission_body = Pour scanner des fichiers, l'application a besoin d'accéder au stockage de l'appareil. Sans cette autorisation, le balayage ne sera pas possible.
grant = Accorder
no_permission_scan_warning = Pas d'accès au fichier - accorder la permission de scanner
# Settings screen tabs
settings_tab_general = Généraux
settings_tab_tools = Outils
settings_tab_diagnostics = Infos
# Settings - General tab
settings_use_cache = Utiliser le cache
settings_use_cache_desc = Accélère les scans suivants (hash/images)
settings_ignore_hidden = Ignorer les fichiers cachés
settings_ignore_hidden_desc = Fichiers et dossiers commençant par '.'
settings_show_notification = Notifier lorsque l'analyse est terminée
settings_show_notification_desc = Afficher une notification système à la fin de l'analyse
settings_notify_only_background = Seulement en arrière-plan
settings_notify_only_background_desc = Ignorer la notification si l'application est visible
notifications_disabled_banner = Notifications désactivées
notifications_enable_button = Activer
settings_scan_label = SCAN
settings_filters_label = FILTRES (quelques outils)
settings_min_file_size = Taille minimale du fichier
settings_max_file_size = Taille maximale du fichier
settings_language = Langue
settings_language_restart = Nécessite un redémarrage de l'application
settings_common_label = PARAMÈTRES COMMUNS
settings_excluded_items = OBJETS EXCLUÉS (modèles de globe, séparés par des virgules)
settings_excluded_items_placeholder = par exemple *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EXTENSIONS AUTORISÉES (vide = tous)
settings_allowed_extensions_placeholder = par exemple jpg, png, mp4
settings_excluded_extensions = EXCLUSIONS EXCLUES
settings_excluded_extensions_placeholder = par exemple bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLICATIONS
settings_check_method_label = MÉTHODE DE COMPARISON
settings_check_method = Méthode
settings_hash_type_label = Type d'HASH
settings_hash_type = Type de hachage
settings_hash_type_desc = Blake3 - est l'option recommandée, CRC32 a de petites chances de faux positifs
settings_similar_images_header = IMAGES SIMILAIRES
settings_similarity_preset = Seuil de similitude
settings_similarity_desc = Très haut = seulement presque identique
settings_hash_size = Taille du hachage
settings_hash_size_desc = Les tailles plus grandes, ont moins de faux positifs, mais trouvent aussi moins d'images similaires
settings_hash_alg = Algorithme de hachage
settings_image_filter = Redimensionner le filtre
settings_ignore_same_size = Ignorer les images avec les mêmes dimensions
settings_gallery_image_fit_cover = Galerie: Recadrer en carré
settings_gallery_image_fit_cover_desc = Remplir la tuile ; désactiver pour conserver les proportions d'origine
settings_big_files_header = MEILLEURS FICHIERS
settings_search_mode = Mode de recherche
settings_file_count = Nombre de fichiers
settings_same_music_header = DUPLICATIONS MUSIQUES
settings_music_check_method = Mode de comparaison
settings_music_compare_tags_label = TAGS COMPAURÉS
settings_music_title = Titre de la page
settings_music_artist = Artiste
settings_music_year = Année
settings_music_length = Longueur
settings_music_genre = Genre
settings_music_bitrate = Débit binaire
settings_music_approx = Comparaison approximative des tags
settings_broken_files_header = FICHIERS DE BROKEN
settings_broken_files_note = Analyse intensive en ressources. Pour de meilleures performances, utilisez Krokiet sur votre bureau.
settings_broken_files_types_label = TYPES CONNECTÉS
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Archive
settings_broken_image = Image
settings_bad_names_header = NOMBRE DE BAD
settings_bad_names_checks_label = CHÈQUES
settings_bad_names_uppercase_ext = Extension en majuscule
settings_bad_names_emoji = Emoji en nom
settings_bad_names_space = Espaces au début/fin
settings_bad_names_non_ascii = Caractères non-ASCII
settings_bad_names_duplicated = Caractères répétés
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIC
diagnostics_thumbnails = Cache des miniatures
diagnostics_app_cache = Cache des applications
diagnostics_refresh = Rafraîchir
diagnostics_clear_thumbnails = Effacer les vignettes
diagnostics_open_thumbnails_folder = Ouvrir le dossier
diagnostics_clear_cache = Vider le cache
diagnostics_open_cache_folder = Ouvrir le dossier
diagnostics_collect_test = Test d'accès au fichier
diagnostics_collect_test_desc = Vérifier combien de fichiers sont accessibles
diagnostics_collect_test_run = Exécuter
diagnostics_collect_test_stop = Arrêter
collect_test_cancelled = Arrêté par l'utilisateur
diag_confirm_clear_thumbnails = Effacer tout le cache des miniatures?
diag_confirm_clear_cache = Effacer le cache de l'application?
about_repo = Dépôt
about_translate = Traductions
about_donate = Soutien
# Collect-test result popup
collect_test_title = Résultats du test
collect_test_volumes = Volumes :
collect_test_folders = Dossiers :
collect_test_files = Fichiers :
collect_test_time = Heure:
# Licenses
licenses_label = LICENCE
third_party_licenses = Licences tierces
licenses_popup_title = Licences tierces
# Directories screen
directories_include_header = Inclure
directories_included = Inclus
directories_exclude_header = Exclure
directories_excluded_header = Exclus
directories_add = Inclure
no_paths = Aucun chemin - ajouter ci-dessous
directories_volume_header = Volumes
directories_volume_refresh = Rafraîchir
directories_volume_add = Ajouter
# Bottom navigation
nav_home = Début
nav_dirs = Répertoires
nav_settings = Réglages
# Status messages set from Rust
status_ready = Prêt
status_stopped = Arrêté
status_no_results = Aucun résultat
status_deleted_selected = Suppression de la sélection
status_deleted_with_errors = Supprimé avec des erreurs
scan_not_started = Scan non démarré
found_items_prefix = Trouvé
found_items_suffix = Eléments
deleted_items_prefix = Supprimé
deleted_items_suffix = Eléments
deleted_errors_suffix = Erreurs
renamed_prefix = Renommé
renamed_files_suffix = fichiers
renamed_errors_suffix = Erreurs
cleaned_exif_prefix = Nettoyé EXIF de
cleaned_exif_suffix = fichiers
cleaned_exif_errors_suffix = Erreurs
and_more_prefix = ... et
and_more_suffix = plus
# Gallery / delete popups
gallery_delete_button = Supprimez
gallery_back = Précédent
gallery_confirm_delete = Oui, supprimer
deleting_files = Suppression des fichiers...
stop = Arrêter
files_suffix = fichiers
scanning_fallback = Analyse en cours...
app_subtitle = En l'honneur de la bataille de Cédynie (972 CE)
app_license = Frontend pour Noyau Czkawka - GPL-3.0
about_app_label = À propos
cache_label = CACHE
# Notification
scan_completed_notification = Scan terminé - { $file_count } éléments trouvés
# Confirm popups (set from Rust)
confirm_clean_exif = Êtes-vous sûr de vouloir nettoyer les balises EXIF des fichiers sélectionnés { $n } ?
confirm_delete_items = Êtes-vous sûr de vouloir supprimer { $n } éléments sélectionnés ?
gallery_confirm_delete_msg = Vous êtes sur le point de supprimer les images { $total_images } dans les groupes { $total_groups}.
gallery_confirm_delete_warning = Tous les articles sont sélectionnés dans les groupes { $unsafe_groups } !
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Le calcul et la comparaison des empreintes sonores sont très gourmands en ressources et peuvent prendre beaucoup de temps. Il est recommandé d'utiliser Krokiet sur un système de bureau pour cette tâche.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Collecte des fichiers
stage_scanning_name = Analyse par nom
stage_scanning_size_name = Analyse par nom et taille
stage_scanning_size = Analyse par taille
stage_pre_hash = Pré-hachage
stage_full_hash = Hachage
stage_loading_cache = Chargement de la cache
stage_saving_cache = Sauvegarde du cache
stage_calculating_image_hashes = Calcul des hachages des images
stage_comparing_images = Comparaison des images
stage_calculating_video_hashes = Calcul des hachages vidéo
stage_checking_files = Vérification des fichiers
stage_checking_extensions = Vérification des extensions
stage_checking_names = Vérification des noms
stage_reading_music_tags = Lecture des tags de musique
stage_comparing_tags = Comparaison des tags
stage_calculating_music_fingerprints = Calcul des empreintes musicales
stage_comparing_fingerprints = Comparaison des empreintes digitales
stage_extracting_exif = Lecture des balises EXIF
stage_creating_video_thumbnails = Création de miniatures vidéo
stage_processing_videos = Traitement des vidéos
stage_deleting = Suppression des fichiers
stage_renaming = Renommer les fichiers
stage_moving = Déplacement des fichiers
stage_hardlinking = Création de liens durs
stage_symlinking = Création de liens symboliques
stage_optimizing_videos = Optimisation des vidéos
stage_cleaning_exif = Nettoyage EXIF
# Group headers in scan results
duplicates_group_header = { $count } fichiers x { $per_file } / fichier = { $total } total
similar_images_group_header = { $count } images similaires
same_music_group_header = { $count } pistes similaires
# Rename confirmation
confirm_rename_items = Êtes-vous sûr de vouloir renommer { $n } fichiers sélectionnés ?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Plus gros
option_search_mode_smallest = Le plus petit
option_similarity_very_high = V.Haut
option_similarity_high = Élevé
option_similarity_medium = Moyenne
option_similarity_low = Bas
option_similarity_very_low = V.Très faible
option_similarity_minimal = Min.
option_check_method_hash = Hachage
option_check_method_name = Nom
option_check_method_size_and_name = Taille+Nom
option_check_method_size = Taille
option_music_method_tags = Mots-clés
option_music_method_audio = Audio
option_min_size_none = Aucun
option_min_size_1kb = 1 Ko
option_min_size_8kb = 8 Ko
option_min_size_64kb = 64 Ko
option_min_size_1mb = 1 Mo
option_max_size_16kb = 16 Ko
option_max_size_1mb = 1 Mo
option_max_size_10mb = 10 Mo
option_max_size_100mb = 100 Mo
option_max_size_unlimited = Illimité
# Volume labels (shown in the directories screen)
volume_internal_storage = Stockage interne
volume_sd_card = Carte mémoire (carte SD)
volume_storage = Volume de stockage
# Directories screen
directories_referenced_tooltip = Référencé (non supprimé)
directories_include_section_header = INCLUS
directories_exclude_section_header = EXCLUSÉ
directories_custom_paths = Chemins personnalisés
directories_check_button = Analyser
directories_check_popup_title = Statistiques du répertoire
directories_check_label_included = Chemins inclus :
directories_check_label_excluded = Chemins exclus :
directories_check_label_referenced = Chemins de référence:
directories_check_label_would_scan = Fichiers à analyser:
directories_check_label_processable = Fichiers à traiter :
directories_check_scanning = Analyse en cours...
directories_check_warning_no_processable = Aucun fichier pouvant être traité - vérifiez que vos dossiers inclus/exclus
path_edit_title_include = Ajouter à Inclure
path_edit_title_exclude = Ajouter à Exclure
path_edit_placeholder = Entrez le chemin...
path_edit_not_exists = Le chemin n'existe pas
path_edit_is_dir = Répertoire
path_edit_is_file = Fichier
path_edit_no_newlines = Les chemins ne peuvent pas contenir de nouvelles lignes — La clé Entrée n'est pas autorisée
ctx_menu_title = Ouvert
ctx_open_file = Ouvrir l'objet
ctx_open_folder = Ouvrir le dossier parent
