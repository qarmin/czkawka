# Window titles
window_settings_title = Options
window_main_title = Czkawka (Hoquet)
window_progress_title = Analyse en cours
window_compare_images = Comparer les images
# General
general_ok_button = Ok
general_close_button = Fermer
# Main window
music_title_checkbox = Titre de la page
music_artist_checkbox = Artiste
music_album_title_checkbox = Titre de l’album
music_album_artist_checkbox = Artiste de l’album
music_year_checkbox = Année
music_comparison_checkbox = Comparaison approximative
music_comparison_checkbox_tooltip =
    Il recherche des fichiers de musique similaires à l'aide de l’intelligence artificielle, qui utilise l'apprentissage automatique pour supprimer des parenthèses d'une phrase, par ex. avec cette option activée, les fichiers en question seront considérés comme des doublons :
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Nom
duplicate_mode_size_combo_box = Taille
duplicate_mode_hash_combo_box = Hachage
duplicate_hash_type_tooltip =
    Czkawka offre 3 types de hachages, qui peuvent être utilisés :
    
    Blake3 - fonction de hachage cryptographique. Il est utilisé comme algorithme de hachage par défaut, car très rapide.
    
    CRC32 - fonction de hachage simple. Cela devrait être plus rapide que Blake3, mais avec probablement de très rares collisions.
    
    XXH3 - très similaire en terme de performances et de qualité de hachage à Blake3, de sorte que de tels modes peuvent être facilement utilisés.
duplicate_check_method_tooltip =
    Pour l'instant, Czkawka offre trois types de méthode pour trouver des doublons par :
    
    Nom - Trouve des fichiers qui ont le même nom.
    
    Taille - Trouve des fichiers qui ont la même taille.
    
    Hachage - Trouve des fichiers qui ont le même contenu. Ce mode permet de hacher les fichiers et de les comparer ensuite pour trouver les doublons. Ce mode est le moyen le plus sûr de trouver les doublons. L'outil utilise lourdement le cache, donc les analyses secondaires et ultérieures des mêmes données devraient être beaucoup plus rapides que la première.
image_hash_size_tooltip =
    Czkawka offre une taille variable de hachage généré pour chaque image. Une taille de hachage plus importante permet de trouver des images avec moins de différences entre les images, mais aussi un peu plus lent à utiliser.
    
    La valeur par défaut pour le hachage est de 8 octets, ce qui permet de trouver des images très similaires et différentes. Les hashs 16 et 32 ne doivent être utilisés que pour des images presque identiques. Le hash de 64 octets ne devrait pas être utilisé, sauf situation où de petites différences sont nécessaires pour trouver
image_resize_filter_tooltip = Pour calculer le hachage de l'image, la bibliothèque doit d'abord la redimensionner. En fonction de l'algorithme choisi, l'image résultée sera peu différente. L'algorithme le plus rapide à utiliser, mais aussi celui qui donne les pires résultats est Nearest.
image_hash_alg_tooltip = Les utilisateurs peuvent choisir un des nombreux algorithmes de calcul du hachage. Chacun a des points forts et des points faibles et donnera parfois de meilleurs résultats pour des images différentes, parfois pires, afin de choisir le meilleur, des tests manuels sont nécessaires
main_notebook_image_fast_compare = Comparaison rapide
main_notebook_image_fast_compare_tooltip =
    Accélère la recherche et la comparaison des hashs.
    
    En opposé au mode normal où chaque hash est comparé les uns aux autres x fois, où x est la similitude de l'utilisateur choisi, dans ce mode, une seule comparaison est utilisée.
    
    Cette option est recommandée lors de la comparaison de >10000 images avec la similitude non 0(Very High) .
main_notebook_duplicates = Fichiers en double
main_notebook_empty_directories = Dossiers vides
main_notebook_big_files = Gros fichiers
main_notebook_empty_files = Fichiers vides
main_notebook_temporary = Fichiers temporaires
main_notebook_similar_images = Images similaires
main_notebook_similar_videos = Vidéos similaires
main_notebook_same_music = Doublons de musique
main_notebook_symlinks = Liens symboliques invalides
main_notebook_broken_files = Fichiers cassés
main_tree_view_column_file_name = Nom du fichier
main_tree_view_column_folder_name = Nom du dossier
main_tree_view_column_path = Chemin d'accès
main_tree_view_column_modification = Date de modification
main_tree_view_column_size = Taille
main_tree_view_column_similarity = Similitude
main_tree_view_column_dimensions = Dimensions
main_tree_view_column_title = Titre
main_tree_view_column_artist = Artiste
main_tree_view_column_year = Année
main_tree_view_column_album_title = Titre de l’album
main_tree_view_column_album_artist = Artiste de l’album
main_tree_view_column_symlink_file_name = Nom du lien symbolique
main_tree_view_column_symlink_folder = Dossier de lien symbolique
main_tree_view_column_destination_path = Chemin de destination
main_tree_view_column_type_of_error = Type d'erreur
main_label_check_method = Méthode de vérification
main_label_hash_type = Type de hachage
main_label_hash_size = Taille du hachage
main_label_size_bytes = Taille (octets)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Nombre de fichiers affichés
main_label_resize_algorithm = Algorithme de redimensionnement
main_label_similarity = Similarité{ " " }
check_button_general_same_size = Ignorer la même taille
check_button_general_same_size_tooltip = Ignorer les résultats, les fichiers dont la taille est identique - généralement ce sont des doublons 1:1
main_label_size_bytes_tooltip = Taille des fichiers qui seront utilisés lors de l'analyse
# Upper window
upper_tree_view_included_folder_column_title = Dossiers à rechercher
upper_tree_view_included_reference_column_title = Dossiers de référence
upper_recursive_button = Récursif
upper_recursive_button_tooltip = Si sélectionné, rechercher également les fichiers qui ne sont pas placés directement dans les dossiers choisis.
upper_manual_add_included_button = Ajout manuel
upper_add_included_button = Ajouter
upper_remove_included_button = Retirer
upper_manual_add_excluded_button = Ajout manuel
upper_add_excluded_button = Ajouter
upper_remove_excluded_button = Retirer
upper_manual_add_included_button_tooltip = Permet d'ajouter un nom de répertoire à la recherche manuelle.
upper_add_included_button_tooltip = Ajouter un nouveau répertoire à la recherche.
upper_remove_included_button_tooltip = Supprimer le répertoire de la recherche.
upper_manual_add_excluded_button_tooltip = Permet d'ajouter des noms de répertoires exclus manuellement.
upper_add_excluded_button_tooltip = Ajouter un répertoire à exclure dans la recherche.
upper_remove_excluded_button_tooltip = Supprimer le répertoire de l’exclusion.
upper_notebook_items_configuration = Configuration des éléments
upper_notebook_excluded_directories = Répertoires exclus
upper_notebook_included_directories = Répertoires inclus
upper_allowed_extensions_tooltip =
    Les extensions autorisées doivent être séparées par des virgules (par défaut, toutes sont disponibles).
    
    Macros IMAGE, VIDEO, MUSIC, TEXT qui ajoutent plusieurs extensions à la fois sont également disponibles.
    
    Exemple d'utilisation ".exe, IMAGE, VIDEO, .rar, 7z" - cela signifie que  les fichiers image (par ex. jpg, png), vidéo (par ex., avi, mp4), exe, rar et 7z  seront analysés.
upper_excluded_items_tooltip =
    Les éléments exclus doivent contenir des caractères génériques comme * et doivent être séparés par des virgules.
    Ceci est plus lent que les répertoires exclus, donc utilisez-les attentivement.
upper_excluded_items = Éléments exclus :
upper_allowed_extensions = Extensions autorisées :
# Popovers
popover_select_all = Tout sélectionner
popover_unselect_all = Désélectionner tout
popover_reverse = Inverser la sélection
popover_select_all_except_oldest = Tout sélectionner sauf le plus ancien
popover_select_all_except_newest = Tout sélectionner sauf le plus récent
popover_select_one_oldest = Sélectionner un plus ancien
popover_select_one_newest = Sélectionner la version la plus récente
popover_select_custom = Sélectionner une personnalisation
popover_unselect_custom = Déselection personalisée
popover_select_all_images_except_biggest = Tout sélectionner sauf le plus gros
popover_select_all_images_except_smallest = Tout sélectionner sauf le plus petit
popover_custom_path_check_button_entry_tooltip =
    Permet de sélectionner les enregistrements par son chemin.
    
    Exemple d'utilisation :
    /home/pimpek/rzecz.txt peut être trouvé avec /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Permet de sélectionner les enregistrements par nom de fichier.
    
    Utilisation d'exemple :
    /usr/ping/pong.txt peut être trouvé avec *ong*
popover_custom_regex_check_button_entry_tooltip =
    Permet de sélectionner les enregistrements par Regex.
    
    Avec ce mode, le texte recherché est le chemin avec le nom.
    
    Utilisation d'exemple :
    /usr/bin/ziemniak. xt peut être trouvé avec /ziem[a-z]+
    
    Cela utilise l’implémentation par défaut de la regex Rust, vous pouvez donc en savoir plus à ce sujet dans https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Empêche la sélection de tous les enregistrements du groupe.
    
    Ceci est activé par défaut, car dans la plupart des cas, l'utilisateur ne veut pas supprimer à la fois les fichiers originaux et les doublons mais souhaite conserver au moins un fichier.
    
    Avertissement : Ce paramètre ne fonctionne pas si l'utilisateur a déjà sélectionné tous les résultats dans le groupe manuellement.
popover_custom_regex_path_label = Chemin d'accès
popover_custom_regex_name_label = Nom
popover_custom_regex_regex_label = Chemin de Regex + Nom
popover_custom_all_in_group_label = Ne pas sélectionner tous les enregistrements du groupe
popover_custom_mode_unselect = Désélectionner la personnalisation
popover_custom_mode_select = Sélectionner la personnalisation
popover_invalid_regex = La regex est invalide
popover_valid_regex = La regex est valide
# Bottom buttons
bottom_search_button = Chercher
bottom_select_button = Sélectionner
bottom_delete_button = Supprimer
bottom_save_button = Enregistrer
bottom_symlink_button = Lien symbolique
bottom_hardlink_button = Lien dur
bottom_move_button = Déplacer
bottom_search_button_tooltip = Commencer à rechercher des fichiers/dossiers.
bottom_select_button_tooltip = Sélectionner les enregistrements. Seuls les fichiers/dossiers sélectionnés peuvent être traités plus tard.
bottom_delete_button_tooltip = Supprimer les fichiers/dossiers sélectionnés.
bottom_save_button_tooltip = Enregistrer les données de la recherche sur un fichier
bottom_symlink_button_tooltip =
    Crée des liens symboliques.
    Ne fonctionne que si au moins 2 résultats sont sélectionnés dans le groupe.
    Le premier est inchangé et le second et les suivants sont liés au premier.
bottom_hardlink_button_tooltip =
    Crée des liens durs.
    Ne fonctionne que si au moins 2 résultats sont sélectionnés dans le groupe.
    Le premier est inchangé et le second et les suivants sont liés en dur au premier.
bottom_move_button_tooltip =
    Déplace les fichiers dans le dossier choisi.
    Il copie tous les fichiers dans le dossier sans préserver l'arborescence des répertoires.
    En essayant de déplacer 2 fichiers avec le même nom de dossier, le second échouera et affichera l'erreur.
bottom_show_errors_tooltip = Afficher/Masquer le panneau d'erreur du bas.
bottom_show_upper_notebook_tooltip = Afficher/Masquer le panneau supérieur du bloc-notes.
# Progress Window
progress_stop_button = Arrêter
# About Window
about_repository_button_tooltip = Lien vers la page du dépôt avec le code source.
about_donation_button_tooltip = Lien vers la page des dons.
about_instruction_button_tooltip = Lien vers la page d'instruction.
about_translation_button_tooltip = Lien vers la page Crowdin avec les traductions d'applications. Officiellement le polonais et l'anglais sont pris en charge, mais toute aide avec une autre langue sera appréciée.
about_repository_button = Dépôt
about_donation_button = Faire un don
about_instruction_button = Instructions
about_translation_button = Traduction
# Header
header_setting_button_tooltip = Ouvre la fenêtre des paramètres.
header_about_button_tooltip = Ouvre la boîte de dialogue avec les informations sur l'application.

# Settings


## General

settings_save_at_exit_button_tooltip = Enregistre la configuration dans un fichier lors de la fermeture de l'application.
settings_load_at_start_button_tooltip =
    Chargement de la configuration à partir d’un fichier.
    
    Ne pas sélectionner cette option chargera les paramètres par défaut.
settings_confirm_deletion_button_tooltip = Affiche la boîte de dialogue de confirmation lorsque vous cliquez sur le bouton Supprimer.
settings_confirm_link_button_tooltip = Affiche la boîte de dialogue de confirmation lorsque vous cliquez sur le bouton lien symbolique/lien en dur.
settings_confirm_group_deletion_button_tooltip = Affiche la boîte de dialogue lorsque vous essayez de supprimer tous les enregistrements du groupe.
settings_show_text_view_button_tooltip = Affiche le panneau d'erreur en bas de page.
settings_use_cache_button_tooltip = Option permettant de ne pas utiliser la fonctionnalité de cache.
settings_save_also_as_json_button_tooltip = Enregistrer le cache en lecture au format JSON humain. Il est possible de modifier son contenu. Le cache de ce fichier sera automatiquement lu par l'application si le cache de format binaire (avec l'extension de la corbeille) est manquant.
settings_use_trash_button_tooltip = Lorsqu’elle est activé, elle déplace les fichiers vers la corbeille au lieu de les supprimer définitivement.
settings_language_label_tooltip = Permet de choisir la langue de l'interface à partir des langues disponibles.
settings_save_at_exit_button = Enregistrer la configuration à la sortie
settings_load_at_start_button = Charger la configuration au démarrage
settings_confirm_deletion_button = Afficher la boîte de dialogue de confirmation lors de la suppression des fichiers
settings_confirm_link_button = Afficher la boîte de dialogue de confirmation lorsque des liens en dur/liens symboliques sont des fichiers
settings_confirm_group_deletion_button = Afficher la boîte de dialogue de confirmation lors de la suppression de tous les fichiers du groupe
settings_show_text_view_button = Afficher le panneau de texte du bas
settings_use_cache_button = Utiliser le cache
settings_save_also_as_json_button = Enregistrer le cache aussi dans le fichier JSON
settings_use_trash_button = Déplacer les fichiers supprimés vers la corbeille
settings_language_label = Langue
settings_multiple_delete_outdated_cache_checkbutton = Supprimer automatiquement les entrées de cache obsolètes
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Permet de supprimer les résultats de cache obsolètes qui pointent vers des fichiers inexistants.
    
    Lorsque cette option est activée, l'application s'assure lors du chargement des enregistrements, que tous les points vers des fichiers valides et ignorent les fichiers cassés.
    
    La désactivation de cette option aidera à scanner des fichiers sur des disques externes, de sorte que les entrées de cache à leur sujet ne seront pas purgées lors de la prochaine analyse.
    
    Si vous avez des centaines de milliers d'enregistrements en cache, il est conseillé d'activer cette option pour accélérer le chargement et l'enregistrement du cache au démarrage et à la fin de la numérisation.
settings_notebook_general = Généraux
settings_notebook_duplicates = Doublons
settings_notebook_images = Images similaires
settings_notebook_videos = Vidéo similaire

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Afficher l'aperçu à droite lors de la sélection du fichier image.
settings_multiple_image_preview_checkbutton = Afficher l'aperçu de l'image
settings_multiple_clear_cache_button_tooltip =
    Effacer manuellement le cache des entrées obsolètes.
    Ne devrait être utilisé que si le nettoyage automatique a été désactivé.
settings_multiple_clear_cache_button = Supprimer les résultats obsolètes du cache des images

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Masque tous les fichiers sauf un, si sont des points vers les mêmes données (sont reliés en dur).
    
    Par exemple. dans le cas où sur le disque il y a 7 fichiers qui sont reliés à des données spécifiques et un fichier différent avec les mêmes données mais un inode différent, alors dans le moteur de recherche en double ne sera visible qu'un seul fichier et un seul fichier provenant de fichiers reliés par des liens durs.
settings_duplicates_minimal_size_entry_tooltip =
    Permet de définir la taille minimale du fichier, qui sera mis en cache.
    
    Choisir une valeur plus petite, générera plus d'enregistrements qui accéléreront la recherche, mais ralentiront le chargement/sauvegarde de la cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Active la mise en cache du prehash(hash calculé à partir d'une petite partie du fichier) qui permet de jeter plus tôt les résultats non dupliqués.
    
    Il est désactivé par défaut car il peut causer dans certaines situations des ralentissements.
    
    Il est fortement recommandé de l'utiliser lors de la numérisation de centaines de milliers ou de millions de fichiers, car il peut accélérer la recherche plusieurs fois.
settings_duplicates_prehash_minimal_entry_tooltip = Taille minimale des entrées en cache.
settings_duplicates_hide_hard_link_button = Cacher les liens durs (uniquement Linux et MacOS)
settings_duplicates_prehash_checkbutton = Utiliser le cache de prehash
settings_duplicates_minimal_size_cache_label = Taille minimale des fichiers en octets enregistrés dans le cache
settings_duplicates_minimal_size_cache_prehash_label = Taille minimale des fichiers en octets enregistrés dans le cache de préhachage

## Saving/Loading settings

settings_saving_button_tooltip = Enregistrer la configuration des paramètres actuels dans un fichier.
settings_loading_button_tooltip = Charger les paramètres à partir du fichier et remplacer la configuration actuelle avec eux.
settings_reset_button_tooltip = Réinitialiser la configuration actuelle à la configuration par défaut.
settings_saving_button = Enregistrer la configuration
settings_loading_button = Charger la configuration
settings_reset_button = Réinitialiser la configuration

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Ouvre le dossier où sont stockés les fichiers txt avec le cache.
    
    Les modifier peut causer l'affichage de résultats invalides mais les modifier (par exemple le chemin peut faire gagner du temps lorsque vous déplacez une grande quantité de fichiers à un endroit différent.
    
    Vous pouvez copier ces fichiers entre ordinateurs pour gagner du temps sur l'analyse à nouveau pour les fichiers (bien sûr s'ils ont une structure de répertoire similaire).
    
    En cas de problème avec le cache, ces fichiers peuvent être supprimés, l'application les régénèrera automatiquement.
settings_folder_settings_open_tooltip =
    Ouvre le dossier où la configuration de Czkawka est stockée.
    
    Les modifier à la main peut endommager votre flux de travail.
settings_folder_cache_open = Ouvrir le dossier de cache
settings_folder_settings_open = Ouvrir le dossier des paramètres
# Compute results
compute_stopped_by_user = La recherche a été arrêtée par l'utilisateur
compute_found_duplicates_hash_size = { $number_files } doublons trouvés dans { $number_groups } groupes qui ont pris { $size }
compute_found_duplicates_name = { $number_files } doublons trouvés dans les groupes { $number_groups }
compute_found_empty_folders = { $number_files } dossiers vides trouvés
compute_found_empty_files = { $number_files } fichiers vides trouvés
compute_found_big_files = { $number_files } grands fichiers trouvés
compute_found_temporary_files = { $number_files } fichiers temporaires trouvés
compute_found_images = { $number_files } images similaires trouvées dans les groupes { $number_groups }
compute_found_videos = { $number_files } vidéos similaires trouvées dans les groupes { $number_groups }
compute_found_music = { $number_files } fichiers de musique similaires trouvés dans les groupes { $number_groups }
compute_found_invalid_symlinks = { $number_files } liens symboliques invalides trouvés
compute_found_broken_files = { $number_files } fichiers cassés trouvés
# Progress window
progress_scanning_general_file = Analyse du fichier { $file_number }
progress_scanning_broken_files = Vérification du fichier { $file_checked }/{ $all_files }
progress_scanning_video = Hachage de la vidéo { $file_checked }/{ $all_files }
progress_scanning_image = Hachage de l'image { $file_checked }/{ $all_files }
progress_comparing_image_hashes = Comparaison du hachage de l'image { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Comparaison des tags du fichier de musique { $file_checked }/{ $all_files }
progress_scanning_music_tags = Lecture des balises du fichier de musique { $file_checked }/{ $all_files }
progress_scanning_empty_folders = Analyse du dossier { $folder_number }
progress_scanning_size = Analyse de la taille du fichier { $file_number }
progress_scanning_name = Analyse du nom du fichier { $file_number }
progress_analyzed_partial_hash = Hash partiel analysé des fichiers { $file_checked }/{ $all_files }
progress_analyzed_full_hash = Hash complet analysé des fichiers { $file_checked }/{ $all_files }
progress_current_stage = Étape actuelle:{ "  " }
progress_all_stages = Toutes les étapes:{ " " }
# Saving loading 
saving_loading_saving_success = Configuration enregistrée dans le fichier { $name }.
saving_loading_saving_failure = Impossible d'enregistrer les données de configuration dans le fichier { $name }.
saving_loading_reset_configuration = La configuration actuelle a été effacée.
saving_loading_loading_success = Configuration de l'application correctement chargée.
saving_loading_invalid_string = Pour la clé "{ $key }" trouvé résultat invalide - "{ $result }" qui n'est pas une chaîne.
saving_loading_invalid_int = Pour la clé "{ $key }" trouvé résultat invalide - "{ $result }" qui n'est pas un entier.
saving_loading_invalid_bool = Pour la clé "{ $key }" trouvé résultat invalide - "{ $result }" qui n'est pas un bool.
saving_loading_decode_problem_bool = Impossible de décoder le bool de la clé "{ $key }" trouvé "{ $result }" mais les valeurs autorisées sont 0, 1, true ou false.
saving_loading_saving_same_keys = Tentative de sauvegarde du paramètre avec la clé «{ $key }».
saving_loading_failed_to_get_home_directory = Impossible d'obtenir le répertoire racine pour ouvrir/enregistrer le fichier de configuration.
saving_loading_folder_config_instead_file = Impossible de créer ou d'ouvrir le fichier de configuration de sauvegarde dans le chemin "{ $path }" car il existe déjà un dossier.
saving_loading_failed_to_create_configuration_folder = La configuration pour créer le dossier de configuration "{ $path }", raison "{ $reason }".
saving_loading_failed_to_create_config_file = Impossible de créer le fichier de configuration "{ $path }", raison "{ $reason }".
saving_loading_failed_to_read_config_file = Impossible de charger la configuration depuis "{ $path }" car elle n'existe pas ou n'est pas un fichier.
saving_loading_failed_to_read_data_from_file = Impossible de lire les données du fichier "{ $path }", raison "{ $reason }".
saving_loading_orphan_data = Données orphelines «{ $data }» trouvées à la ligne «{ $line }».
saving_loading_not_valid = Le paramètre "{ $data }" n'existe pas dans la version actuelle de l'application.
# Invalid symlinks
invalid_symlink_infinite_recursion = Récursion infinie
invalid_symlink_non_existent_destination = Fichier de destination inexistant
# Other
searching_for_data = Recherche de données, cela peut prendre un certain temps, veuillez patienter...
text_view_messages = MESSAGES
text_view_warnings = ATTENTIONS
text_view_errors = ERREUR
about_window_motto = Ce programme est gratuit et le sera toujours.
# Various dialog
dialogs_ask_next_time = Demander la prochaine fois
delete_file_failed = Impossible de supprimer le fichier { $name }, raison { $reason }
delete_title_dialog = Confirmation de la suppression
delete_question_label = Êtes-vous sûr de vouloir supprimer les fichiers ?
delete_all_files_in_group_title = Confirmation de la suppression de tous les fichiers du groupe
delete_all_files_in_group_label1 = Dans certains groupes, il y a sélectionné tous les enregistrements.
delete_all_files_in_group_label2 = Êtes-vous sûr de vouloir les supprimer ?
delete_folder_failed = Impossible de supprimer le dossier { $dir } car le dossier n'existe pas, vous n'avez pas les permissions ou n'est pas vide.
delete_items_label = { $items } fichiers seront supprimés.
delete_items_groups_label = { $items } fichiers de { $groups } groupes seront supprimés.
hardlink_failed = Impossible de relier le matériel
hard_sym_invalid_selection_title_dialog = Sélection non valide avec certains groupes
hard_sym_invalid_selection_label_1 = Dans certains groupes, il n'y a qu'un seul enregistrement sélectionné et il sera ignoré.
hard_sym_invalid_selection_label_2 = Pour être en mesure de relier ces fichiers, au moins 2 résultats dans le groupe doivent être sélectionnés.
hard_sym_invalid_selection_label_3 = Le premier dans le groupe est reconnu comme original et n'est pas modifié, mais le second et plus tard sont modifiés.
hard_sym_link_title_dialog = Confirmation du lien
hard_sym_link_label = Êtes-vous sûr de vouloir lier ce fichier ?
move_folder_failed = Impossible de déplacer le dossier { $name }, raison { $reason }
move_file_failed = Impossible de déplacer le fichier { $name }, raison { $reason }
move_files_title_dialog = Choisissez le dossier dans lequel vous voulez déplacer les fichiers dupliqués
move_files_choose_more_than_1_path = Un seul chemin doit être sélectionné pour pouvoir copier des fichiers dupliqués, sélectionné { $path_number }.
move_stats = Éléments { $num_files }/{ $all_files } correctement déplacés
save_results_to_file = Résultats enregistrés dans le fichier { $name }
search_not_choosing_any_music = ERREUR : Vous devez sélectionner au moins une case à cocher avec les types de recherche de musique.
include_folders_dialog_title = Dossiers à inclure
exclude_folders_dialog_title = Dossiers à exclure
include_manually_directories_dialog_title = Ajouter un répertoire manuellement
cache_properly_cleared = Cache correctement vidé
cache_clear_duplicates_title = Effacement du cache des doublons
cache_clear_similar_images_title = Nettoyage du cache des images similaires
cache_clear_similar_videos_title = Nettoyage du cache des vidéos similaires
cache_clear_message_label_1 = Voulez-vous vider le cache des entrées obsolètes?
cache_clear_message_label_2 = Cette opération supprimera toutes les entrées du cache qui pointent vers des fichiers non valides.
cache_clear_message_label_3 = Cela peut accélérer un peu le chargement/sauvegarde dans le cache.
cache_clear_message_label_4 = AVERTISSEMENT : L'opération supprimera toutes les données mises en cache des disques externes débranchés, donc le hachage devra être généré à nouveau.
# Show preview
preview_temporary_file = Impossible d'ouvrir le fichier d'image temporaire { $name }, raison { $reason }.
preview_0_size = Impossible de créer l'aperçu de l'image { $name }, avec 0 largeur ou hauteur.
preview_temporary_image_save = Impossible d'enregistrer le fichier d'image temporaire dans { $name }, raison { $reason }.
preview_temporary_image_remove = Impossible de supprimer le fichier d'image temporaire { $name }, raison { $reason }.
preview_failed_to_create_cache_dir = Impossible de créer le répertoire { $name } nécessaire à l'aperçu de l'image, raison { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Groupe { $current_group }/{ $all_groups } ({ $images_in_group } images)
compare_move_left_button = L
compare_move_right_button = R
