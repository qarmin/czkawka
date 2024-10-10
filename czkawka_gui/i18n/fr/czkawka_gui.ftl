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
music_checking_by_tags = Tags
music_checking_by_content = Contenu
same_music_seconds_label = Durée minimale de seconde de fragment
same_music_similarity_label = Différence maximale
music_compare_only_in_title_group = Comparer seulement dans le titre
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    La recherche de fichiers musicaux aux contenus similaires peut être configurée en définissant :
    
    - La durée minimale d'un fragment pour que des fichiers musicaux soient identifiés comme similaires
    - La différence maximale entre deux fragments testés
    
    La clé pour arriver à de bons résultats est de trouver des combinaisons raisonnables de ces paramètres.
    
    Fixer le temps minimum à 5 secondes et la différence maximale à 1.0, cherchera des fragments presque identiques dans les fichiers.
    Un temps de 20 secondes et une différence maximale de 6.0 fonctionne bien pour trouver des remixes/versions live, etc.
    
    Par défaut, chaque fichier musical est comparé à tous les autres et cela peut prendre beaucoup de temps lors du test de plusieurs fichier. Il est donc généralement préférable d'utiliser des dossiers de référence et de spécifier quels fichiers doivent être comparés les uns avec les autres (avec la même quantité de fichiers, la comparaison des empreintes sera au moins 4x plus rapide que sans dossier de référence).
music_comparison_checkbox_tooltip =
    La recherche des fichiers de musique similaires est faite à l’aide d'intelligence artificielle qui utilise l'apprentissage machine pour supprimer les parenthèses d’une phrase. Par exemple, avec cette option activée les fichiers en question seront considérés comme des doublons :
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Sensible à la casse
duplicate_case_sensitive_name_tooltip =
    Quand activé, groupe les enregistrements uniquement quand ils ont exactement le même nom, par exemple Żołd <-> Żołd
    
    Désactiver cette option va regrouper les noms sans se préocupper de la casse, par exemple żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Taille et nom
duplicate_mode_name_combo_box = Nom
duplicate_mode_size_combo_box = Taille
duplicate_mode_hash_combo_box = Hachage
duplicate_hash_type_tooltip =
    Czkawka offre 3 types de hachages :
    
    Blake3 - fonction de hachage cryptographique. Il est utilisé comme algorithme de hachage par défaut car très rapide.
    
    CRC32 - fonction de hachage simple qui devrait être plus rapide que Blake3. Peut, très rarement, provoquer des collisions.
    
    XXH3 - très similaire en terme de performances et de qualité de hachage à Blake3 mais non cryptographique. De ce fait ils peuvent facilement être changés l'un pour l'autre.
duplicate_check_method_tooltip =
    Pour l'instant, Czkawka offre trois types de méthode pour trouver des doublons par :
    
    Nom - Trouve des fichiers qui ont le même nom.
    
    Taille - Trouve des fichiers qui ont la même taille.
    
    Hachage - Trouve des fichiers qui ont le même contenu. Ce mode permet de hacher le fichier puis de comparer ensuite le hash pour trouver les doublons. Ce mode est le moyen le plus sûr de trouver les doublons. L'application utilisant massivement le cache, les analyses suivantes des mêmes données devraient être beaucoup plus rapides que la première.
image_hash_size_tooltip =
    Chaque image vérifiée produit un hachage spécial qui peut être comparé les uns aux autres, et une petite différence entre elles signifie que ces images sont similaires.
    
    La taille du hachage 8 est assez bonne pour trouver des images qui ne sont qu'un peu similaires à l'original. Avec un plus grand ensemble d'images (>1000), cela produira une grande quantité de faux positifs, donc je recommande d'utiliser une plus grande taille de hachage dans ce cas.
    
    16 est la taille par défaut du hachage, ce qui est un bon compromis entre trouver même un peu des images similaires et n'avoir qu'une petite quantité de collisions de hachage.
    
    32 et 64 hachages ne trouvent que des images très similaires, mais devraient avoir presque pas de faux positifs (peut-être sauf certaines images avec canal alpha).
image_resize_filter_tooltip =
    Pour calculer le hachage de l'image, la bibliothèque doit d'abord la redimensionner.
    
    En fonction de l'algorithme choisi, l'image résultante utilisée pour calculer le hachage pourra sembler un peu différente.
    
    L'algorithme le plus rapide à utiliser, mais aussi celui qui donne les pires résultats, est PlusProche. Il est activé par défaut, car avec une taille de hachage d'une qualité inférieure à 16x16, cela ne sera que peu visible.
    
    Avec une taille de hachage de 8x8, il est recommandé d'utiliser un algorithme différent de PlusProche pour obtenir de meilleurs groupes d'images.
image_hash_alg_tooltip =
    Les utilisateurs peuvent choisir parmi de nombreux algorithmes pour calculer le hash.
    
    Chacun a des points forts et des points faibles et donnera parfois des résultats meilleurs et parfois pires pour des images différentes.
    
    Par conséquent, des tests manuels sont requis pour déterminer celui qui donnera le meileur résultat pour vous.
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
main_tree_view_column_symlink_folder = Dossier du lien symbolique
main_tree_view_column_destination_path = Chemin de destination
main_tree_view_column_type_of_error = Type d'erreur
main_tree_view_column_current_extension = Extension actuelle
main_tree_view_column_proper_extensions = Extension correcte
main_label_check_method = Méthode de vérification
main_label_hash_type = Type de hachage
main_label_hash_size = Taille du hachage
main_label_size_bytes = Taille (octets)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Nombre de fichiers affichés
main_label_resize_algorithm = Algorithme de redimensionnement
main_label_similarity = Similarité{ " " }
main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archiver
main_check_box_broken_files_image = Image
check_button_general_same_size = Ignorer la même taille
check_button_general_same_size_tooltip = Ignorer les fichiers avec la même taille dans les résultats - généralement ce sont des doublons 1:1
main_label_size_bytes_tooltip = Taille des fichiers qui seront utilisés lors de l'analyse
# Upper window
upper_tree_view_included_folder_column_title = Dossiers dans lesquels chercher
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
    Ajouter manuellement le nom du répertoire à rechercher.
    
    Pour ajouter plusieurs chemins à la fois, séparez-les avec « ; »
    
    « /home/roman;/home/rozkaz » ajoutera deux répertoires « /home/roman » et « /home/rozkaz »
upper_add_included_button_tooltip = Ajouter un nouveau répertoire à la recherche.
upper_remove_included_button_tooltip = Supprimer le répertoire de la recherche.
upper_manual_add_excluded_button_tooltip =
    Ajouter manuellement un nom de répertoire exclu.
    
    Pour ajouter plusieurs chemins à la fois, séparez-les ave « ; »
    
    « /home/roman;/home/krokiet » ajoutera deux répertoires « /home/roman » et « /home/keokiet »
upper_add_excluded_button_tooltip = Ajouter un répertoire à exclure de la recherche.
upper_remove_excluded_button_tooltip = Retirer le répertoire de la liste de ceux exclus.
upper_notebook_items_configuration = Configuration des éléments
upper_notebook_excluded_directories = Répertoires exclus
upper_notebook_included_directories = Répertoires inclus
upper_allowed_extensions_tooltip =
    Les extensions autorisées doivent être séparées par des virgules (toutes sont disponibles par défaut).
    
    Les Macros suivantes, qui ajoutent plusieurs extensions à la fois, sont également disponibles : IMAGE, VIDEO, MUSIC, TEXT.
    
    Exemple d'utilisation : « .exe, IMAGE, VIDEO, .rar, 7z » - signifie que les fichiers images (par exemple jpg, png), des vidéos (par exemple avi, mp4), exe, rar et 7z seront scannés.
upper_excluded_extensions_tooltip =
    Liste des fichiers désactivés qui seront ignorés lors de l'analyse.
    
    Lorsque vous utilisez des extensions autorisées et désactivées, celle-ci a une priorité plus élevée, donc le fichier ne sera pas vérifié.
upper_excluded_items_tooltip =
    Les éléments exclus doivent contenir le caractère joker « * » et être séparés par des virgules.
    Ceci est plus lent que les répertoires exclus, donc à utiliser avec prudence.
upper_excluded_items = Éléments exclus :
upper_allowed_extensions = Extensions autorisées :
upper_excluded_extensions = Extensions désactivées :
# Popovers
popover_select_all = Tout sélectionner
popover_unselect_all = Tout désélectionner
popover_reverse = Inverser la sélection
popover_select_all_except_oldest = Tout sélectionner sauf le plus ancien
popover_select_all_except_newest = Tout sélectionner sauf le plus récent
popover_select_one_oldest = Sélectionner un élément plus ancien
popover_select_one_newest = Sélectionner un élément récent
popover_select_custom = Sélection personnalisée
popover_unselect_custom = Annuler la sélection personnalisée
popover_select_all_images_except_biggest = Tout sélectionner sauf le plus gros
popover_select_all_images_except_smallest = Tout sélectionner sauf le plus petit
popover_custom_path_check_button_entry_tooltip =
    Sélectionner les enregistrements par chemin.
    
    Exemple d'utilisation :
    « /home/pimpek/rzecz.txt » peut être trouvé avec « /home/pim* »
popover_custom_name_check_button_entry_tooltip =
    Sélectionner les enregistrements par nom de fichier.
    
    Exemple d'utilisation :
    « /usr/ping/pong.txt » peut être trouvé avec « *ong* »
popover_custom_regex_check_button_entry_tooltip =
    Sélectionner les enregistrements par Regex spécifié.
    
    Dans ce mode, le texte recherché est le Chemin avec le Nom.
    
    Exemple d'utilisation:
    « /usr/bin/ziemniak.txt » peut être trouvé avec « /ziem[a-z]+ »
    
    Cela utilise l'implémentation par défaut de Rust regex : https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Active la détection sensible à la casse.
    
    Si désactivé, « /home/* » trouve « /HoMe/roman » et « /home/roman ».
popover_custom_not_all_check_button_tooltip =
    Empêche la sélection de tous les enregistrements dans le groupe.
    
    Ceci est activé par défaut car, dans la plupart des cas, vous ne voulez pas supprimer à la fois les fichiers originaux et les doublons mais souhaitez laisser au moins un fichier.
    
    AVERTISSEMENT : ce réglage ne fonctionne pas si vous avez déjà sélectionné manuellement tous les résultats dans un groupe.
popover_custom_regex_path_label = Chemin d'accès
popover_custom_regex_name_label = Nom
popover_custom_regex_regex_label = Chemin d'accès Regex + Nom
popover_custom_case_sensitive_check_button = Sensible à la casse
popover_custom_all_in_group_label = Ne pas sélectionner tous les enregistrements du groupe
popover_custom_mode_unselect = Désélectionner la personnalisation
popover_custom_mode_select = Sélectionner la personnalisation
popover_sort_file_name = Nom du fichier
popover_sort_folder_name = Nom du dossier
popover_sort_full_name = Nom complet
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
bottom_select_button_tooltip = Sélectionnez les enregistrements. Seuls les fichiers/dossiers sélectionnés pourront être traités plus tard.
bottom_delete_button_tooltip = Supprimer les fichiers/dossiers sélectionnés.
bottom_save_button_tooltip = Enregistrer les données de la recherche dans un fichier
bottom_symlink_button_tooltip =
    Créer des liens symboliques.
    Ne fonctionne que si au moins deux résultats dans un groupe sont sélectionnés.
    Le premier reste inchangé, tous les suivants sont transformés en lien symbolique vers ce premier résultat.
bottom_hardlink_button_tooltip =
    Créer des liens durs.
    Ne fonctionne que si au moins deux résultats dans un groupe sont sélectionnés.
    Le premier reste inchangé, tous les suivants sont transformés en lien dur vers ce premier résultat.
bottom_hardlink_button_not_available_tooltip =
    Créer des liens durs.
    Le bouton est désactivé car des liens durs ne peuvent être créés.
    Les liens durs ne fonctionnent qu’avec les privilèges administrateur sous Windows, assurez-vous d'éxécuter l’application en tant qu’administrateur.
    Si l’application fonctionne déjà avec ces privilèges, vérifiez les signalements de bogues similaires sur GitHub.
bottom_move_button_tooltip =
    Déplace les fichiers vers le répertoire choisi.
    Ceci copie tous les fichiers dans le répertoire cible sans préserver l'arborescence du répertoire source.
    Si on tente de déplacer deux fichiers avec le même nom vers le dossier, le second échouera et un message d'erreur s'affichera.
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
about_translation_button_tooltip = Lien vers la page Crowdin avec les traductions de lapplication. Le polonais et l'anglais sont officiellement pris en charge.
about_repository_button = Dépôt
about_donation_button = Faire un don
about_instruction_button = Instructions
about_translation_button = Traduction
# Header
header_setting_button_tooltip = Ouvre la fenêtre des paramètres.
header_about_button_tooltip = Ouvre la boîte de dialogue contenant les informations sur l'application.

# Settings


## General

settings_number_of_threads = Nombre de threads utilisés
settings_number_of_threads_tooltip = Nombre de threads utilisés. « 0 » signifie que tous les threads disponibles seront utilisés.
settings_use_rust_preview = Utiliser des bibliothèques externes à la place gtk pour charger les aperçus
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = Vous devez redémarrer l’application pour appliquer les réglages !
settings_ignore_other_filesystems = Ignorer les autres systèmes de fichiers (Linux uniquement)
settings_ignore_other_filesystems_tooltip =
    ignore les fichiers qui ne sont pas dans le même système de fichiers que les répertoires recherchés.
    
    Fonctionne de la même manière que l'option « -xdev » de la commande « find » sous Linux
settings_save_at_exit_button_tooltip = Enregistrer la configuration dans un fichier à la fermeture de l'application.
settings_load_at_start_button_tooltip =
    Charger la configuration à partir du fichier à l'ouverture de l'application.
    
    Si désactivé, les paramètres par défaut seront utilisés.
settings_confirm_deletion_button_tooltip = Afficher une boîte de dialogue de confirmation lorsque vous cliquez sur le bouton Supprimer.
settings_confirm_link_button_tooltip = Afficher une boîte de dialogue de confirmation lorsque vous cliquez sur le bouton « hard/symlink ».
settings_confirm_group_deletion_button_tooltip = Afficher une boîte de dialogue d'avertissement lorsque vous essayez de supprimer tous les enregistrements du groupe.
settings_show_text_view_button_tooltip = Afficher le panneau de texte en bas de l'interface utilisateur.
settings_use_cache_button_tooltip = Utiliser le cache de fichiers.
settings_save_also_as_json_button_tooltip = Enregistrer le cache au format JSON (lisible par un humain). Il est possible de modifier son contenu. Le contenu de ce fichier sera lu automatiquement par l'application si le cache au format binaire (extension .bin) est manquant.
settings_use_trash_button_tooltip = Déplace les fichiers vers la corbeille au lieu de les supprimer définitivement.
settings_language_label_tooltip = Langue de l'interface utilisateur.
settings_save_at_exit_button = Enregistrer la configuration à la fermeture de l'application
settings_load_at_start_button = Charger la configuration à l'ouverture de l'application
settings_confirm_deletion_button = Afficher une boîte de dialogue de confirmation lors de la suppression de fichiers
settings_confirm_link_button = Afficher une boîte de dialogue de confirmation lorsque des liens en dur ou symboliques vers des fichiers sont créés
settings_confirm_group_deletion_button = Afficher une boîte de dialogue de confirmation lors de la suppression de tous les fichiers d'un groupe
settings_show_text_view_button = Afficher le panneau de texte du bas
settings_use_cache_button = Utiliser le cache
settings_save_also_as_json_button = Également enregistrer le cache en tant que fichier JSON
settings_use_trash_button = Déplacer les fichiers supprimés vers la corbeille
settings_language_label = Langue
settings_multiple_delete_outdated_cache_checkbutton = Supprimer automatiquement les entrées de cache obsolètes
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Supprimer du cache les résultats obsolètes pointant vers des fichiers inexistants.
    
    Lorsque cette option est activée, l'application s'assure lors du chargement des enregistrements que tous pointent vers des fichiers valides (les fichiers cassés sont ignorés).
    
    Désactiver cette option facilitera l'analyse de fichiers sur des disques externes: les entrées de cache les concernant ne seront pas purgées lors de la prochaine analyse.
    
    Il est conseillé de d'activer cette option quand des centaines de milliers d'enregistrements sont dans le cache. Ceci permettra d'accélérer le chargement et la sauvegarde du cache au démarrage et à la fin de l'analyse.
settings_notebook_general = Généraux
settings_notebook_duplicates = Doublons
settings_notebook_images = Images similaires
settings_notebook_videos = Vidéo similaire

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Affiche l'aperçu à droite (lors de la sélection d'un fichier image).
settings_multiple_image_preview_checkbutton = Afficher l'aperçu de l'image
settings_multiple_clear_cache_button_tooltip =
    Vider manuellement le cache des entrées obsolètes.
    À utiliser uniquement si le nettoyage automatique a été désactivé.
settings_multiple_clear_cache_button = Supprimer les résultats périmés du cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Masque tous les fichiers, sauf un, si tous pointent vers les mêmes données (avec lien en dur).
    
    Exemple : soient sur le disque sept fichiers reliés à des données spécifiques et un fichier différent avec les mêmes données mais un inode différent ; dans le module de recherche des doublons seuls un fichier unique et un fichier provenant des liens en dur seront affichés.
settings_duplicates_minimal_size_entry_tooltip =
    Définit la taille minimale du fichier qui sera mis en cache.
    
    Choisir une valeur plus petite générera plus d'enregistrements. Cela accélérera la recherche, mais ralentira le chargement/l'enregistrement du cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Active la mise en cache du prehash (un hachage calculé à partir d'une petite partie du fichier) qui permet un rejet plus rapide des résultats non dupliqués.
    
    Il est désactivé par défaut car il peut causer des ralentissements dans certaines situations.
    
    Il est fortement recommandé de l'utiliser lors de la numérisation de centaines de milliers ou de millions de fichiers, car il peut accélérer la recherche de manière géométrique.
settings_duplicates_prehash_minimal_entry_tooltip = Taille minimale de l'entrée en cache.
settings_duplicates_hide_hard_link_button = Masquer les liens en dur (Linux et macOS uniquement)
settings_duplicates_prehash_checkbutton = Utiliser le cache de prehash
settings_duplicates_minimal_size_cache_label = Taille minimale des fichiers (en octets) enregistrés dans le cache
settings_duplicates_minimal_size_cache_prehash_label = Taille minimale des fichiers (en octets) enregistrés dans le cache de préhachage

## Saving/Loading settings

settings_saving_button_tooltip = Enregistrez les paramètres de configuration actuels dans un fichier.
settings_loading_button_tooltip = Charger les paramètres à partir d'un fichier pour remplacer la configuration actuelle.
settings_reset_button_tooltip = Réinitialiser la configuration actuelle pour revenir à celle par défaut.
settings_saving_button = Enregistrer la configuration
settings_loading_button = Charger la configuration
settings_reset_button = Réinitialiser la configuration

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Ouvre le dossier où sont stockés les fichiers « .txt » de cache.
    
    La modification des fichiers de cache peut provoquer l'affichage de résultats invalides. Cependant, la modification du chemin peut faire gagner du temps lorsque une grande quantité de fichiers est déplacée vers un autre emplacement.
    
    Vous pouvez copier ces fichiers entre ordinateurs pour gagner du temps sur une nouvelle analyse de fichiers (à condition, bien sûr, qu'ils aient une structure de répertoire similaire).
    
    En cas de problèmes avec le cache, ces fichiers peuvent être supprimés. L'application les régénèrera automatiquement.
settings_folder_settings_open_tooltip =
    Ouvre le dossier où la configuration de Czkawka est stockée.
    
    AVERTISSEMENT : modifier manuellement la configuration peut endommager votre workflow.
settings_folder_cache_open = Ouvrir le dossier de cache
settings_folder_settings_open = Ouvrir le dossier des paramètres
# Compute results
compute_stopped_by_user = La recherche a été interrompue par l'utilisateur
compute_found_duplicates_hash_size = { $number_files } doublons trouvés dans { $number_groups } groupes qui ont pris { $size }
compute_found_duplicates_name = { $number_files } doublons trouvés dans { $number_groups } groupes
compute_found_empty_folders = { $number_files } dossiers vides trouvés
compute_found_empty_files = { $number_files } fichiers vides trouvés
compute_found_big_files = { $number_files } gros fichiers trouvés
compute_found_temporary_files = { $number_files } fichiers temporaires trouvés
compute_found_images = { $number_files } images similaires trouvées dans { $number_groups } groupes
compute_found_videos = { $number_files } vidéos similaires trouvées dans { $number_groups } groupes
compute_found_music = { $number_files } fichiers de musique similaires trouvés dans { $number_groups } groupes
compute_found_invalid_symlinks = { $number_files } liens symboliques invalides trouvés
compute_found_broken_files = { $number_files } fichiers cassés trouvés
compute_found_bad_extensions = { $number_files } fichiers avec des extensions invalides trouvés
# Progress window
progress_scanning_general_file = Analyse du fichier { $file_number }
progress_scanning_extension_of_files = Vérification de l'extension du fichier { $file_checked }/{ $all_files }
progress_scanning_broken_files = Vérification du fichier { $file_checked }/{ $all_files }
progress_scanning_video = Hachage de la vidéo { $file_checked }/{ $all_files }
progress_scanning_image = Hachage de l'image { $file_checked }/{ $all_files }
progress_comparing_image_hashes = Comparaison du hachage de l'image { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Comparaison des tags du fichier audio { $file_checked }/{ $all_files }
progress_scanning_music_tags = Lecture des balises du fichier audio { $file_checked }/{ $all_files }
progress_scanning_music_content_end = Comparaison de l'empreinte numérique du fichier audio { $file_checked }/{ $all_files }
progress_scanning_music_content = Calcul de l'empreinte numérique du fichier audio { $file_checked }/{ $all_files }
progress_scanning_empty_folders = Analyse du dossier { $folder_number }
progress_scanning_size = Analyse de la taille du fichier { $file_number }
progress_scanning_size_name = Analyse du nom et de la taille du fichier { $file_number }
progress_scanning_name = Analyse du nom du fichier { $file_number }
progress_analyzed_partial_hash = Analyse partielle du hash de { $file_checked }/{ $all_files } fichiers
progress_analyzed_full_hash = Analyse complète du hash de { $file_checked }/{ $all_files } fichiers
progress_prehash_cache_loading = Chargement du cache du prehash
progress_prehash_cache_saving = Sauvegarde du cache du prehash
progress_hash_cache_loading = Chargement du cache de hachage
progress_hash_cache_saving = Sauvegarde du cache de hachage
progress_cache_loading = Chargement de la cache
progress_cache_saving = Sauvegarde du cache
progress_current_stage = Étape actuelle :{ "  " }
progress_all_stages = Toutes les étapes :{ " " }
# Saving loading 
saving_loading_saving_success = Configuration enregistrée dans le fichier { $name }.
saving_loading_saving_failure = Impossible d'enregistrer les données de configuration dans le fichier { $name }.
saving_loading_reset_configuration = La configuration actuelle a été effacée.
saving_loading_loading_success = Configuration de l'application correctement chargée.
saving_loading_invalid_string = Résultat invalide trouvé pour la clé "{ $key }" - "{ $result }" n'est pas une chaîne.
saving_loading_invalid_int = Résultat invalide trouvé pour la clé "{ $key }" - "{ $result }" n'est pas un entier.
saving_loading_invalid_bool = Résultat invalide trouvé pour la clé "{ $key }" - "{ $result }" n'est pas un booléen.
saving_loading_decode_problem_bool = Impossible de décoder le booléen de la clé "{ $key }". Trouvé "{ $result }" mais les valeurs autorisées sont 0, 1, true ou false.
saving_loading_saving_same_keys = Tentative de sauvegarde du paramètre avec la clé dupliquée «{ $key }».
saving_loading_failed_to_get_home_directory = Impossible d'obtenir le répertoire racine pour ouvrir/enregistrer le fichier de configuration.
saving_loading_folder_config_instead_file = Impossible de créer ou d'ouvrir le fichier de configuration sauvegardé à l'emplacement "{ $path }" car il existe déjà un dossier.
saving_loading_failed_to_create_configuration_folder = Échec de la configuration pour créer le dossier de configuration "{ $path }". Raison : "{ $reason }".
saving_loading_failed_to_create_config_file = Impossible de créer le fichier de configuration "{ $path }". Raison : "{ $reason }".
saving_loading_failed_to_read_config_file = Impossible de charger la configuration depuis "{ $path }" car elle n'existe pas ou n'est pas un fichier.
saving_loading_failed_to_read_data_from_file = Impossible de lire les données du fichier "{ $path }". Raison : "{ $reason }".
saving_loading_orphan_data = Données orphelines « { $data } » trouvées à la ligne « { $line } ».
saving_loading_not_valid = Le paramètre « { $data } » n'existe pas dans la version actuelle de l'application.
# Invalid symlinks
invalid_symlink_infinite_recursion = Récursion infinie
invalid_symlink_non_existent_destination = Fichier de destination inexistant
# Other
selected_all_reference_folders = Impossible de lancer la recherche quand tous les répertoires sont définis comme des répertoires de référence
searching_for_data = Recherche de données. Cela peut prendre un certain temps, veuillez patienter…
text_view_messages = MESSAGES
text_view_warnings = AVERTISSEMENTS
text_view_errors = ERREURS
about_window_motto = Ce programme peut être utilisée gratuitement et le sera toujours.
# Various dialog
dialogs_ask_next_time = Demander la prochaine fois
delete_file_failed = Impossible de supprimer le fichier { $name }. Raison : { $reason }
delete_title_dialog = Confirmation de la suppression
delete_question_label = Êtes-vous sûr de vouloir supprimer les fichiers ?
delete_all_files_in_group_title = Confirmation de la suppression de tous les fichiers du groupe
delete_all_files_in_group_label1 = L'ensemble des enregistrements est sélectionné dans certains groupes.
delete_all_files_in_group_label2 = Êtes-vous sûr de vouloir les supprimer ?
delete_folder_failed = Impossible de supprimer le dossier { $dir } car soit le dossier n'existe pas, vous n'avez pas la permission ou le dossier n'est pas vide.
delete_items_label = { $items } fichiers seront supprimés.
delete_items_groups_label = { $items } fichiers de { $groups } groupes seront supprimés.
hardlink_failed = Impossible de créer un lien en dur
hard_sym_invalid_selection_title_dialog = Sélection invalide avec certains groupes
hard_sym_invalid_selection_label_1 = Un seul enregistrement est sélectionné dans certains groupes et il sera ignoré.
hard_sym_invalid_selection_label_2 = Au moins deux résultats au sein du groupe doivent être sélectionnés pour les relier ces par un lien en dur ou symbolique.
hard_sym_invalid_selection_label_3 = Le premier dans le groupe est reconnu comme original et n'est pas modifié mais les suivants le seront.
hard_sym_link_title_dialog = Confirmation du lien
hard_sym_link_label = Êtes-vous sûr de vouloir relier ces fichiers ?
move_folder_failed = Impossible de déplacer le dossier { $name }. Raison : { $reason }
move_file_failed = Impossible de déplacer le fichier { $name }. Raison : { $reason }
move_files_title_dialog = Choisissez le dossier dans lequel vous voulez déplacer les fichiers dupliqués
move_files_choose_more_than_1_path = Un seul chemin peut être sélectionné pour pouvoir copier leurs fichiers dupliqués. { $path_number } est sélectionné.
move_stats = Éléments { $num_files }/{ $all_files } correctement déplacés
save_results_to_file = Résultats enregistrés dans les fichiers txt et json dans le dossier { $name }.
search_not_choosing_any_music = ERREUR : vous devez sélectionner au moins une case à cocher parmi les types de recherche de musique.
search_not_choosing_any_broken_files = ERREUR : vous devez sélectionner au moins une case à cocher parmi les types de fichiers cassés.
include_folders_dialog_title = Dossiers à inclure
exclude_folders_dialog_title = Dossiers à exclure
include_manually_directories_dialog_title = Ajouter un répertoire manuellement
cache_properly_cleared = Cache correctement vidé
cache_clear_duplicates_title = Purge du cache des doublons
cache_clear_similar_images_title = Purge du cache des images similaires
cache_clear_similar_videos_title = Purge du cache des vidéos similaires
cache_clear_message_label_1 = Voulez-vous vider le cache des entrées obsolètes ?
cache_clear_message_label_2 = Cette opération supprimera toutes les entrées du cache qui pointent vers des fichiers invalides.
cache_clear_message_label_3 = Cela peut légèrement accélérer le chargement et la sauvegarde dans le cache.
cache_clear_message_label_4 = AVERTISSEMENT : cette opération supprimera toutes les données mises en cache des disques externes débranchés. Chaque hachage devra donc être régénéré.
# Show preview
preview_image_resize_failure = Impossible de redimensionner l'image { $name }.
preview_image_opening_failure = Impossible d'ouvrir l'image { $name }. Raison : { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Groupe { $current_group }/{ $all_groups } ({ $images_in_group } images)
compare_move_left_button = L
compare_move_right_button = R
