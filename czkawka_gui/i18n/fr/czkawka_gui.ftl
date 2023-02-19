# Window titles
window_settings_title = Paramètres
window_main_title = Czkawka (Hoquet)
window_progress_title = Analyse en cours
window_compare_images = Comparer les images
# General
general_ok_button = Ok
general_close_button = Fermer
# Main window
music_title_checkbox = Titre de la page
music_artist_checkbox = Artiste
music_year_checkbox = Année
music_bitrate_checkbox = Débit binaire
music_genre_checkbox = Genre
music_length_checkbox = Longueur
music_comparison_checkbox = Comparaison approximative
music_comparison_checkbox_tooltip =
    Il recherche des fichiers de musique similaires à l’aide d’une intelligence artificielle, qui utilise le machine learning pour supprimer les parenthèses d’une phrase, par exemple avec cette option activée, les fichiers en question seront considérés comme des doublons :
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Sensible à la casse
duplicate_case_sensitive_name_tooltip =
    Lorsqu'il est activé, ne grouper que les enregistrements quand ils ont exactement le même nom (p. ex. Żołd <-> Żołd
    
    Désactiver cette option va regrouper les noms sans vérifier si chaque lettre a la même taille, par exemple żoŁD <-> Żołd
duplicate_mode_name_combo_box = Nom
duplicate_mode_size_combo_box = Taille
duplicate_mode_hash_combo_box = Hachage
duplicate_hash_type_tooltip =
    Czkawka offre 3 types de hachages :
    
    Blake3 - fonction de hachage cryptographique. Il est utilisé comme algorithme de hachage par défaut, car très rapide.
    
    CRC32 - fonction de hachage simple. Cela devrait être plus rapide que Blake3, mais avec probablement de très rares collisions.
    
    XXH3 - très similaire en terme de performances et de qualité de hachage à Blake3 (mais non cryptographique). Donc ils peuvent facilement être interchangés.
duplicate_check_method_tooltip =
    Pour l'instant, Czkawka offre trois types de méthode pour trouver des doublons par :
    
    Nom - Trouve des fichiers qui ont le même nom.
    
    Taille - Trouve des fichiers qui ont la même taille.
    
    Hachage - Trouve des fichiers qui ont le même contenu. Ce mode permet de hacher le fichier et de le comparer ensuite pour trouver les doublons. Ce mode est le moyen le plus sûr de trouver les doublons. L'application utilise lourdement le cache, donc les analyses secondaires et ultérieures des mêmes données devraient être beaucoup plus rapides que la première.
image_hash_size_tooltip =
    Chaque image vérifiée produit un hachage spécial qui peut être comparé entre les autres, et une petite différence entre elles signifie que ces images sont similaires.
    
    La taille de 8 hachages est assez bonne pour trouver des images qui ne sont que peu similaires à l'original. Avec un plus grand ensemble d'images(>1000) produira une grande quantité de faux positifs, je recommande donc d'utiliser pour une telle quantité plus grande taille de hachage.
    
    16 est la taille de hachage par défaut, ce qui est assez bon compromis entre trouver même un peu d'images similaires et avoir peu de collisions de hachage.
    
    32 et 64 hachages ne trouvent que des images très similaires, mais presque ne devraient pas avoir de faux positifs (peut-être sauf certaines images avec canal alpha).
image_resize_filter_tooltip =
    Pour calculer le hachage de l'image, la bibliothèque doit d'abord le redimensionner.
    
    Dépend de l'algorithme choisi, image résultée utilisée pour calculer le hachage peut sembler peu différente.
    
    L'algorithme le plus rapide à utiliser, mais aussi celui qui donne les pires résultats est Nearest, elle est activée par défaut, car avec une taille de hachage 16x16, une qualité inférieure n'est pas vraiment visible.
    
    Avec une taille de hachage de 8x8 il est recommandé d'utiliser un algorithme différent de Nearest, pour avoir de meilleurs groupes d'images.
image_hash_alg_tooltip =
    Les utilisateurs peuvent choisir parmi un des nombreux algorithmes de calcul du hachage.
    
    Chacune a des points forts et des points faibles et donnera parfois des résultats meilleurs et parfois pires pour des images différentes.
    
    Donc, pour déterminer le meilleur pour vous, des tests manuels sont requis.
big_files_mode_combobox_tooltip = Permet de rechercher les fichiers les plus petits ou les plus grands
big_files_mode_label = Fichiers cochés
big_files_mode_smallest_combo_box = Le plus petit
big_files_mode_biggest_combo_box = Le plus grand
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
main_notebook_bad_extensions = Mauvaises extensions
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
main_tree_view_column_bitrate = Débit binaire
main_tree_view_column_length = Longueur
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = Nom du lien symbolique
main_tree_view_column_symlink_folder = Dossier Symlink
main_tree_view_column_destination_path = Chemin de destination
main_tree_view_column_type_of_error = Type d'erreur
main_tree_view_column_current_extension = Extension actuelle
main_tree_view_column_proper_extensions = Propre extension
main_label_check_method = Méthode de vérification
main_label_hash_type = Type de hachage
main_label_hash_size = Taille du hachage
main_label_size_bytes = Taille (octets)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Nombre de fichiers affichés
main_label_resize_algorithm = Algorithme de redimensionnement
main_label_similarity = Similarité{ " " }
main_check_box_broken_files_audio = L'audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archiver
main_check_box_broken_files_image = Image
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
upper_manual_add_included_button_tooltip =
    Ajouter le nom du répertoire à rechercher à la main.
    
    Pour ajouter plusieurs chemins à la fois, séparez-les par ;
    
    /home/roman;/home/rozkaz ajoutera deux répertoires /home/roman et /home/rozkaz
upper_add_included_button_tooltip = Ajouter un nouveau répertoire à la recherche.
upper_remove_included_button_tooltip = Supprimer le répertoire de la recherche.
upper_manual_add_excluded_button_tooltip =
    Ajouter un nom de répertoire exclu à la main.
    
    Pour ajouter plusieurs chemins à la fois, séparez-les par ;
    
    /home/roman;/home/krokiet ajoutera deux répertoires /home/roman et /home/keokiet
upper_add_excluded_button_tooltip = Ajouter un répertoire à exclure dans la recherche.
upper_remove_excluded_button_tooltip = Supprimer le répertoire de l’exclusion.
upper_notebook_items_configuration = Configuration des éléments
upper_notebook_excluded_directories = Répertoires exclus
upper_notebook_included_directories = Répertoires inclus
upper_allowed_extensions_tooltip =
    Les extensions autorisées doivent être séparées par des virgules (par défaut, toutes sont disponibles).
    
    Les Macros suivantes, qui ajoutent plusieurs extensions à la fois, sont également disponibles : IMAGE, VIDEO, MUSIC, TEXT.
    
    Exemple d'utilisation ".exe, IMAGE, VIDEO, .rar, 7z" - cela signifie que les images (e. jpg, png), des vidéos (par exemple, avi, mp4), exe, rar et 7z des fichiers seront scannés.
upper_excluded_items_tooltip =
    Les éléments exclus doivent contenir des caractères génériques * et doivent être séparés par des virgules.
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
    Sélectionnez les enregistrements par chemin.
    
    Exemple d'utilisation :
    /home/pimpek/rzecz.txt peut être trouvé avec /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Sélectionnez les enregistrements par nom de fichier.
    
    Exemple d'utilisation :
    /usr/ping/pong.txt peut être trouvé avec *ong*
popover_custom_regex_check_button_entry_tooltip =
    Sélectionnez les enregistrements par Regex spécifié.
    
    Avec ce mode, le texte recherché est le chemin avec le nom.
    
    Utilisation d'exemple :
    /usr/bin/ziemniak. xt peut être trouvé avec /ziem[a-z]+
    
    Cela utilise l'implémentation par défaut de la regex Rust : https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Active la détection sensible à la casse.
    
    Si désactivé, /home/* trouve /HoMe/roman et /home/roman.
popover_custom_not_all_check_button_tooltip =
    Empêche la sélection de tous les enregistrements dans le groupe.
    
    Ceci est activé par défaut, car dans la plupart des situations, vous ne voulez pas supprimer à la fois les fichiers originaux et les doublons, mais vous voulez laisser au moins un fichier.
    
    AVERTISSEMENT : Ce paramètre ne fonctionne pas si vous avez déjà sélectionné manuellement tous les résultats dans un groupe.
popover_custom_regex_path_label = Chemin d'accès
popover_custom_regex_name_label = Nom
popover_custom_regex_regex_label = Chemin de Regex + Nom
popover_custom_case_sensitive_check_button = Sensible à la casse
popover_custom_all_in_group_label = Ne pas sélectionner tous les enregistrements du groupe
popover_custom_mode_unselect = Désélectionner la personnalisation
popover_custom_mode_select = Sélectionner la personnalisation
popover_sort_file_name = Nom du fichier
popover_sort_folder_name = Nom dossier
popover_sort_full_name = Nom
popover_sort_size = Taille
popover_sort_selection = Sélection
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
bottom_sort_button = Trier
bottom_search_button_tooltip = Lancer la recherche
bottom_select_button_tooltip = Sélectionnez les enregistrements. Seuls les fichiers/dossiers sélectionnés peuvent être traités plus tard.
bottom_delete_button_tooltip = Supprimer les fichiers/dossiers sélectionnés.
bottom_save_button_tooltip = Enregistrer les données de la recherche sur un fichier
bottom_symlink_button_tooltip =
    Créer des liens symboliques.
    Ne fonctionne que si au moins deux résultats dans un groupe sont sélectionnés.
    Le premier est inchangé et le second et plus tard sont liés au premier.
bottom_hardlink_button_tooltip =
    Créer des liens durs.
    Ne fonctionne que si au moins deux résultats dans un groupe sont sélectionnés.
    Le premier est inchangé et le second et plus tard sont hardliés au premier.
bottom_hardlink_button_not_available_tooltip =
    Créer des liens durs.
    Le bouton est désactivé, car des liens durs ne peuvent être créés.
    Les liens durs ne fonctionnent qu’avec les privilèges administrateur sous Windows pour être sûr d’exécuter l’application en tant qu’administrateur.
    Si l’application fonctionne déjà avec ces privilèges, vérifiez les signalements de bogues similaires sur GitHub.
bottom_move_button_tooltip =
    Déplace les fichiers vers le répertoire choisi.
    Il copie tous les fichiers dans le répertoire sans préserver l'arborescence des répertoires.
    En essayant de déplacer deux fichiers avec le même nom vers le dossier, le second échouera et affichera l'erreur.
bottom_sort_button_tooltip = Trie les fichiers/dossiers selon la méthode sélectionnée.
bottom_show_errors_tooltip = Afficher/Masquer le panneau de texte du bas.
bottom_show_upper_notebook_tooltip = Afficher/Masquer le panneau supérieur du bloc-notes.
# Progress Window
progress_stop_button = Arrêter
progress_stop_additional_message = Arrêt demandé
# About Window
about_repository_button_tooltip = Lien vers la page du dépôt avec le code source.
about_donation_button_tooltip = Lien vers la page des dons.
about_instruction_button_tooltip = Lien vers la page d'instruction.
about_translation_button_tooltip = Lien vers la page Crowdin avec les traductions d'applications. Officiellement le polonais et l'anglais sont pris en charge.
about_repository_button = Dépôt
about_donation_button = Faire un don
about_instruction_button = Instructions
about_translation_button = Traduction
# Header
header_setting_button_tooltip = Ouvre la fenêtre des paramètres.
header_about_button_tooltip = Ouvre la boîte de dialogue avec les informations sur l'application.

# Settings


## General

settings_number_of_threads = Nombre de threads utilisés
settings_number_of_threads_tooltip = Nombre de threads utilisés, 0 signifie que tous les threads disponibles seront utilisés.
settings_label_restart = Vous devez redémarrer l’application pour appliquer les paramètres !
settings_ignore_other_filesystems = Ignorer les autres systèmes de fichiers (uniquement Linux)
settings_ignore_other_filesystems_tooltip =
    ignore les fichiers qui ne sont pas dans le même système de fichiers que les répertoires recherchés.
    
    Fonctionne de la même manière que l'option -xdev dans la commande find sur Linux
settings_save_at_exit_button_tooltip = Enregistrer la configuration dans un fichier lors de la fermeture de l'application.
settings_load_at_start_button_tooltip =
    Charger la configuration à partir du fichier lors de l'ouverture de l'application.
    
    Si non activé, les paramètres par défaut seront utilisés.
settings_confirm_deletion_button_tooltip = Afficher la boîte de dialogue de confirmation lorsque vous cliquez sur le bouton Supprimer.
settings_confirm_link_button_tooltip = Afficher la boîte de dialogue de confirmation lorsque vous cliquez sur le bouton hard/symlink.
settings_confirm_group_deletion_button_tooltip = Afficher la boîte de dialogue d'avertissement lorsque vous essayez de supprimer tous les enregistrements du groupe.
settings_show_text_view_button_tooltip = Afficher le panneau de texte en bas de l'interface utilisateur.
settings_use_cache_button_tooltip = Utiliser le cache de fichiers.
settings_save_also_as_json_button_tooltip = Enregistrer le cache au format JSON (lisible par l'homme). Il est possible de modifier son contenu. Le cache de ce fichier sera lu automatiquement par l'application si le cache du format binaire (avec l'extension de la corbeille) est manquant.
settings_use_trash_button_tooltip = Déplace les fichiers vers la corbeille au lieu de les supprimer définitivement.
settings_language_label_tooltip = Langue pour l'interface utilisateur.
settings_save_at_exit_button = Enregistrer la configuration lors de la fermeture de l'application
settings_load_at_start_button = Charger la configuration lors de l'ouverture de l'application
settings_confirm_deletion_button = Afficher la boîte de dialogue de confirmation lors de la suppression des fichiers
settings_confirm_link_button = Afficher la boîte de dialogue de confirmation lorsque des liens en dur/liens symboliques sont des fichiers
settings_confirm_group_deletion_button = Afficher la boîte de dialogue de confirmation lors de la suppression de tous les fichiers du groupe
settings_show_text_view_button = Afficher le panneau de texte du bas
settings_use_cache_button = Utiliser le cache
settings_save_also_as_json_button = Enregistrer également le cache en tant que fichier JSON
settings_use_trash_button = Déplacer les fichiers supprimés vers la corbeille
settings_language_label = Langue
settings_multiple_delete_outdated_cache_checkbutton = Supprimer automatiquement les entrées de cache obsolètes
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Supprimer les résultats du cache obsolètes qui pointent vers des fichiers inexistants.
    
    Lorsque cette option est activée, l'application s'assure lors du chargement des enregistrements, que tous les enregistrements pointent vers des fichiers valides (les fichiers cassés sont ignorés).
    
    Désactiver ceci aidera lors de l'analyse de fichiers sur des disques externes, de sorte que les entrées de cache à leur sujet ne seront pas purgées lors de la prochaine analyse.
    
    Dans le cas d'avoir des centaines de milliers de dossiers en cache, il est conseillé de l'activer, ce qui accélérera le chargement/la sauvegarde du cache au démarrage/fin de l'analyse.
settings_notebook_general = Généraux
settings_notebook_duplicates = Doublons
settings_notebook_images = Images similaires
settings_notebook_videos = Vidéo similaire

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Affiche l'aperçu à droite (lors de la sélection d'un fichier image).
settings_multiple_image_preview_checkbutton = Afficher l'aperçu de l'image
settings_multiple_clear_cache_button_tooltip =
    Vider manuellement le cache des entrées obsolètes.
    Cela ne doit être utilisé que si le nettoyage automatique a été désactivé.
settings_multiple_clear_cache_button = Supprimer les résultats obsolètes du cache des images

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Masque tous les fichiers, sauf un, si tous pointent vers les mêmes données (sont reliés en dur).
    
    Exemple : Dans le cas où il y a (sur le disque) sept fichiers qui sont reliés à des données spécifiques et un fichier différent avec les mêmes données mais un inode différent, puis dans le moteur de recherche en double, un seul fichier unique et un seul fichier provenant de liens durs seront affichés.
settings_duplicates_minimal_size_entry_tooltip =
    Définit la taille minimale du fichier qui sera mis en cache.
    
    Choisir une valeur plus petite générera plus d'enregistrements. Cela accélérera la recherche, mais ralentira le chargement/l'enregistrement du cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Active la mise en cache du prehash (un hachage calculé à partir d'une petite partie du fichier) qui permet le rejet antérieur des résultats non dupliqués.
    
    Il est désactivé par défaut car il peut causer des ralentissements dans certaines situations.
    
    Il est fortement recommandé de l'utiliser lors de la numérisation de centaines de milliers ou de millions de fichiers, car il peut accélérer la recherche de plusieurs fois.
settings_duplicates_prehash_minimal_entry_tooltip = Taille minimale des entrées en cache.
settings_duplicates_hide_hard_link_button = Masquer les liens durs (uniquement Linux et macOS)
settings_duplicates_prehash_checkbutton = Utiliser le cache de prehash
settings_duplicates_minimal_size_cache_label = Taille minimale des fichiers (en octets) enregistrés dans le cache
settings_duplicates_minimal_size_cache_prehash_label = Taille minimale des fichiers (en octets) enregistrés dans le cache de préhachage

## Saving/Loading settings

settings_saving_button_tooltip = Enregistrez la configuration des paramètres actuels dans un fichier.
settings_loading_button_tooltip = Charger les paramètres à partir du fichier et remplacer la configuration actuelle avec eux.
settings_reset_button_tooltip = Réinitialiser la configuration actuelle à la configuration par défaut.
settings_saving_button = Enregistrer la configuration
settings_loading_button = Charger la configuration
settings_reset_button = Réinitialiser la configuration

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Ouvre le dossier où sont stockés les fichiers de cache txt.
    
    La modification des fichiers de cache peut faire apparaître des résultats non valides. Cependant, la modification du chemin peut gagner du temps lorsque vous déplacez une grande quantité de fichiers vers un autre emplacement.
    
    Vous pouvez copier ces fichiers entre ordinateurs pour gagner du temps sur l'analyse de nouveau pour les fichiers (bien sûr s'ils ont une structure de répertoire similaire).
    
    En cas de problèmes avec le cache, ces fichiers peuvent être supprimés. L'application les régénèrera automatiquement.
settings_folder_settings_open_tooltip =
    Ouvre le dossier où la configuration de Czkawka est stockée.
    
    AVERTISSEMENT : modifier manuellement la configuration peut endommager votre workflow.
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
compute_found_bad_extensions = { $number_files } fichiers avec des extensions non valides trouvés
# Progress window
progress_scanning_general_file = Analyse du fichier { $file_number }
progress_scanning_extension_of_files = Vérification de l'extension du fichier { $file_checked }/{ $all_files }
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
saving_loading_not_valid = Le paramètre «{ $data }» n'existe pas dans la version actuelle de l'application.
# Invalid symlinks
invalid_symlink_infinite_recursion = Récursion infinie
invalid_symlink_non_existent_destination = Fichier de destination inexistant
# Other
selected_all_reference_folders = Impossible de lancer la recherche, quand tous les répertoires sont définis comme des répertoires de référence
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
delete_all_files_in_group_label1 = Dans certains groupes, tous les enregistrements sont sélectionnés.
delete_all_files_in_group_label2 = Êtes-vous sûr de vouloir les supprimer ?
delete_folder_failed = Impossible de supprimer le dossier { $dir } car le dossier n'existe pas, vous n'avez pas la permission ou le dossier n'est pas vide.
delete_items_label = { $items } fichiers seront supprimés.
delete_items_groups_label = { $items } fichiers de { $groups } groupes seront supprimés.
hardlink_failed = Impossible de relier le matériel
hard_sym_invalid_selection_title_dialog = Sélection non valide avec certains groupes
hard_sym_invalid_selection_label_1 = Dans certains groupes, il n'y a qu'un seul enregistrement sélectionné et il sera ignoré.
hard_sym_invalid_selection_label_2 = Pour être en mesure de relier ces fichiers, au moins deux résultats dans le groupe doivent être sélectionnés.
hard_sym_invalid_selection_label_3 = Le premier dans le groupe est reconnu comme original et n'est pas modifié, mais le second et plus tard sont modifiés.
hard_sym_link_title_dialog = Confirmation du lien
hard_sym_link_label = Êtes-vous sûr de vouloir lier ces fichiers ?
move_folder_failed = Impossible de déplacer le dossier { $name }, raison { $reason }
move_file_failed = Impossible de déplacer le fichier { $name }, raison { $reason }
move_files_title_dialog = Choisissez le dossier dans lequel vous voulez déplacer les fichiers dupliqués
move_files_choose_more_than_1_path = Un seul chemin peut être sélectionné pour pouvoir copier leurs fichiers dupliqués, sélectionné { $path_number }.
move_stats = Éléments { $num_files }/{ $all_files } correctement déplacés
save_results_to_file = Résultats enregistrés dans le fichier { $name }
search_not_choosing_any_music = ERREUR : Vous devez sélectionner au moins une case à cocher avec les types de recherche de musique.
search_not_choosing_any_broken_files = ERREUR : Vous devez sélectionner au moins une case à cocher avec le type de fichiers cassés cochés.
include_folders_dialog_title = Dossiers à inclure
exclude_folders_dialog_title = Dossiers à exclure
include_manually_directories_dialog_title = Ajouter un répertoire manuellement
cache_properly_cleared = Cache correctement vidé
cache_clear_duplicates_title = Effacement du cache des doublons
cache_clear_similar_images_title = Nettoyage du cache des images similaires
cache_clear_similar_videos_title = Nettoyage du cache des vidéos similaires
cache_clear_message_label_1 = Voulez-vous vider le cache des entrées obsolètes?
cache_clear_message_label_2 = Cette opération supprimera toutes les entrées du cache qui pointent vers des fichiers non valides.
cache_clear_message_label_3 = Cela peut légèrement accélérer le chargement/sauvegarde dans le cache.
cache_clear_message_label_4 = AVERTISSEMENT : L'opération supprimera toutes les données mises en cache des disques externes débranchés. Chaque hachage devra donc être régénéré.
# Show preview
preview_image_resize_failure = Impossible de redimensionner l'image { $name }.
preview_image_opening_failure = Impossible d'ouvrir l'image { $name }, raison { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Groupe { $current_group }/{ $all_groups } ({ $images_in_group } images)
compare_move_left_button = L
compare_move_right_button = R
