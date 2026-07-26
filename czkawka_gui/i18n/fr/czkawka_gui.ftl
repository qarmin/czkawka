# Window titles
window_settings_title = Paramètres
window_main_title = Czkawka (Hoquet)
window_progress_title = Analyse en cours
window_compare_images = Comparer les images
# General
general_ok_button = Ok
general_close_button = Fermer
# Krokiet info dialog
krokiet_info_title = Avis de dépréciation
krokiet_info_message =
    Czkawka GTK 12.0 est la dernière version. Aucune mise à jour, fonctionnalité ou correction de bugs supplémentaires n'est prévue.
    
    La plupart des fonctionnalités de Czkawka GTK sont disponibles dans Krokiet, généralement sous une forme plus simple, plus rapide et plus stable. Krokiet ajoute également de nouvelles fonctionnalités et améliorations qui n'étaient pas possibles dans la version GTK.
    
    Si vous utilisez encore Czkawka GTK, la transition vers Krokiet devrait être facile, car il possède une interface similaire, moins de dépendances et une meilleure prise en charge multiplateforme.
    
    P.S. : Ce message ne doit apparaître qu'une seule fois. Si vous le voyez réapparaître, définissez la variable d'environnement CZKAWKA_DONT_ANNOY_ME sur une valeur non vide.
# Main window
music_title_checkbox = Titre
music_artist_checkbox = Artiste
music_year_checkbox = Année
music_bitrate_checkbox = Débit binaire
music_genre_checkbox = Genre
music_length_checkbox = Longueur
music_comparison_checkbox = Comparaison approximative
music_checking_by_tags = Étiquettes
music_checking_by_content = Contenu
same_music_seconds_label = Durée minimale de seconde de fragment
same_music_similarity_label = Différence maximale
music_compare_only_in_title_group = Comparer au sein des groupes de titres similaires
music_compare_only_in_title_group_tooltip =
    Lorsque cette option est activée, les fichiers sont regroupés par titre, puis comparés l'un à l'autre.
    
    Pour 10000 fichiers, au lieu de près de 100 millions de comparaisons en général, il y aura environ 20000 comparaisons.
same_music_tooltip = ...cela peut prendre beaucoup de temps lors du test de plusieurs fichiers. Il est donc généralement préférable...
music_comparison_checkbox_tooltip =
    La recherche des fichiers de musique similaires est faite à l’aide d'intelligence artificielle qui utilise l'apprentissage machine pour supprimer les parenthèses d’une phrase. Par exemple, avec cette option activée les fichiers en question seront considérés comme des doublons :
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Sensible à la casse
duplicate_case_sensitive_name_tooltip = Désactiver cette option va regrouper les noms sans se préoccuper de la casse, par exemple żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Taille et nom
duplicate_mode_name_combo_box = Nom
duplicate_mode_size_combo_box = Taille
duplicate_mode_hash_combo_box = Hachage
duplicate_hash_type_tooltip = CRC32 - fonction de hachage simple qui devrait être plus rapide que Blake3. Elle peut, très rarement, provoquer des collisions.
duplicate_check_method_tooltip =
    Pour l'instant, Czkawka offre trois types de méthode pour trouver des doublons par :
    
    Nom - Trouve des fichiers qui ont le même nom.
    
    Taille - Trouve des fichiers qui ont la même taille.
    
    Hachage - Trouve des fichiers qui ont le même contenu. Ce mode permet de hacher le fichier puis de comparer ensuite le hash pour trouver les doublons. Ce mode est le moyen le plus sûr de trouver les doublons. L'application utilisant massivement le cache, les analyses suivantes des mêmes données devraient être beaucoup plus rapides que la première.
image_hash_size_tooltip = Chaque image vérifiée produit un hachage spécial ; ces hachages peuvent être comparés entre eux, et une petite différence entre eux signifie que ces images sont similaires.
image_resize_filter_tooltip =
    L'algorithme le plus rapide à utiliser, mais aussi celui qui donne les pires résultats, est Nearest (le plus proche). Il est activé par défaut, car avec une taille de hachage d'une qualité inférieure à 16x16, cela ne sera que peu visible.
    
    Avec une taille de hachage de 8x8, il est recommandé d'utiliser un algorithme différent de Nearest pour obtenir de meilleurs groupes d'images.
image_hash_alg_tooltip = ...des tests manuels sont requis pour déterminer celui qui donnera le meilleur résultat pour vous.
image_geometric_invariance_tooltip = Comparez également les variantes en miroir / retourné et éventuellement en rotation de chaque image. Cela améliore la correspondance, mais augmente le temps de hachage.
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
main_tree_view_column_similarity = Similarité
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
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Codec
main_label_check_method = Méthode de vérification
main_label_hash_type = Type de hachage
main_label_hash_size = Taille du hachage
main_label_geometric_invariance = Invariance géométrique
main_label_size_bytes = Taille (octets)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Nombre de fichiers affichés
main_label_resize_algorithm = Algorithme de redimensionnement
main_label_similarity = Similarité{ " " }
main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archive
main_check_box_broken_files_image = Image
main_check_box_broken_files_video = Vidéo
main_check_box_broken_files_video_tooltip = Utilise ffmpeg/ffprobe pour valider les fichiers vidéo. Très lent et peut détecter des erreurs insignifiantes même si le fichier est bien lu.
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
upper_notebook_excluded_directories = Chemins exclus
upper_notebook_included_directories = Chemins inclus
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
popover_select_all_except_shortest_path = Tout sélectionner sauf le chemin le plus court
popover_select_all_except_longest_path = Tout sélectionner sauf le chemin le plus long
popover_select_all_except_oldest = Tout sélectionner sauf le plus ancien
popover_select_all_except_newest = Tout sélectionner sauf le plus récent
popover_select_one_oldest = Sélectionner l'élément le plus ancien
popover_select_one_newest = Sélectionner l'élément le plus récent
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
popover_custom_mode_unselect = Désactiver la sélection personnalisée
popover_custom_mode_select = Activer la sélection personnalisée
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
bottom_compare_button = Comparer
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
bottom_hardlink_button_not_available_tooltip = ...assurez-vous d'exécuter l'application en tant qu'administrateur.
bottom_move_button_tooltip =
    Déplace les fichiers vers le répertoire choisi.
    Ceci copie tous les fichiers dans le répertoire cible sans préserver l'arborescence du répertoire source.
    Si on tente de déplacer deux fichiers avec le même nom vers le dossier, le second échouera et un message d'erreur s'affichera.
bottom_sort_button_tooltip = Trie les fichiers/dossiers selon la méthode sélectionnée.
bottom_compare_button_tooltip = Comparer les images dans le groupe.
bottom_show_errors_tooltip = Afficher/Masquer le panneau de texte du bas.
bottom_show_upper_notebook_tooltip = Afficher/Masquer le panneau supérieur du bloc-notes.
# Progress Window
progress_stop_button = Arrêter
progress_stop_additional_message = Arrêt demandé
# About Window
about_repository_button_tooltip = Lien vers la page du dépôt avec le code source.
about_donation_button_tooltip = Lien vers la page des dons.
about_instruction_button_tooltip = Lien vers la page d'instruction.
about_translation_button_tooltip = Lien vers la page Crowdin avec les traductions de l'application. Le polonais et l'anglais sont officiellement pris en charge.
about_repository_button = Dépôt
about_donation_button = Faire un don
about_instruction_button = Instructions
about_translation_button = Traduction
about_other_apps_button = Autres applications
about_other_apps_dialog_title = Autres applications par qarmin
about_other_apps_open_source_note = Toutes les applications listées sont gratuites et open source.
about_other_apps_open_button = Ouvrir
about_other_apps_szyszka_desc = Renommeur de fichiers rapide et puissant.
about_other_apps_mykrut_desc = Gestionnaire de fichiers Linux simple, rapide et avec des partis pris affirmés.
about_other_apps_dcmki_viewer_desc = Visionneuse DICOM simple.
about_other_apps_video_thumbnailer_desc = Enveloppe autour du générateur de miniatures vidéo utilisé dans Czkawka.
about_other_apps_space_finder_desc = Recherche simple des plus gros fichiers de votre système.
about_other_apps_system_info_collector_desc = Récupère l'utilisation de la RAM/CPU à partir de l'OS et l'affiche sous forme de graphiques.
# Header
header_setting_button_tooltip = Ouvre la fenêtre des paramètres.
header_about_button_tooltip = Ouvre la boîte de dialogue contenant les informations sur l'application.
header_krokiet_button_tooltip = Essayez Krokiet - la nouvelle version améliorée !
# Krokiet promo dialog
krokiet_promo_title = Rencontrez Krokiet !
krokiet_promo_message =
    Bonjour, utilisateur courageux de Czkawka !
    
    La Force est manifestement avec vous, mais Krokiet ne l'est pas - un outil de nettoyage de doublons plus récent, plus rapide, plus léger et, disons-le, significativement plus attrayant (si l'on peut parler d'attrait chez les applications).
    
    Krokiet inclut tout ce que les gens appréciaient chez Czkawka. Il est totalement gratuit, open source, possède une interface utilisateur unique et simple (à la fois encensée et critiquée par beaucoup), introduit de nombreuses nouvelles fonctionnalités, utilise moins de dépendances et fonctionne de manière beaucoup plus fiable sur différentes plateformes.
    
    Et si vous l'avez manqué, il existe déjà une application encore plus récente que Krokiet : Cedinia, conçue principalement pour les appareils Android et l'utilisation tactile.
    
    Czkawka GTK nous a bien servis, mais son temps est révolu.
krokiet_promo_link_download = Télécharger Krokiet/Cedinia
krokiet_promo_link_project = Page du projet

# Settings


## General

settings_number_of_threads = Nombre de threads utilisés
settings_number_of_threads_tooltip = Nombre de threads utilisés. « 0 » signifie que tous les threads disponibles seront utilisés.
settings_use_rust_preview = Utiliser des bibliothèques externes à la place de gtk pour charger les aperçus
settings_use_rust_preview_tooltip = ...sera parfois plus rapide et gérera plus de formats...
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
settings_confirm_link_button_tooltip = Afficher une boîte de dialogue de confirmation lorsque vous cliquez sur le bouton de lien dur/symbolique.
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
settings_multiple_delete_outdated_cache_checkbutton_tooltip = Il est conseillé d'activer cette option quand des centaines de milliers d'enregistrements sont dans le cache. Ceci permettra d'accélérer le chargement et la sauvegarde du cache au démarrage et à la fin de l'analyse.
settings_notebook_general = Général
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
    Active la mise en cache du préhachage (un hachage calculé à partir d'une petite partie du fichier) qui permet un rejet plus rapide des résultats non dupliqués.
    
    Il est désactivé par défaut car il peut causer des ralentissements dans certaines situations.
    
    Il est fortement recommandé de l'utiliser lors de la numérisation de centaines de milliers ou de millions de fichiers, car il peut accélérer la recherche plusieurs fois.
settings_duplicates_prehash_minimal_entry_tooltip = Taille minimale de l'entrée en cache.
settings_duplicates_hide_hard_link_button = Masquer les liens durs
settings_duplicates_prehash_checkbutton = Utiliser le cache de préhachage
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

settings_folder_cache_open_tooltip = ...la modification du chemin peut faire gagner du temps lorsqu'une grande quantité de fichiers est déplacée vers un autre emplacement.
settings_folder_settings_open_tooltip =
    Ouvre le dossier où la configuration de Czkawka est stockée.
    
    AVERTISSEMENT : modifier manuellement la configuration peut endommager votre workflow.
settings_folder_cache_open = Ouvrir le dossier de cache
settings_folder_settings_open = Ouvrir le dossier des paramètres
# Compute results
compute_stopped_by_user = La recherche a été interrompue par l'utilisateur
compute_found_duplicates_hash_size = { $number_files } doublons trouvés dans { $number_groups } groupes, ce qui a pris { $size } en { $time }
compute_found_duplicates_name = { $number_files } doublons trouvés dans { $number_groups } groupes en { $time }
compute_found_empty_folders = { $number_files } dossiers vides trouvés en { $time }
compute_found_empty_files = { $number_files } fichiers vides trouvés en { $time }
compute_found_big_files = { $number_files } fichiers volumineux trouvés en { $time }
compute_found_temporary_files = { $number_files } fichiers temporaires trouvés en { $time }
compute_found_images = { $number_files } images similaires trouvées dans { $number_groups } groupes en { $time }
compute_found_videos = { $number_files } vidéos similaires trouvées dans { $number_groups } groupes en { $time }
compute_found_music = { $number_files } fichiers de musique similaires trouvés dans { $number_groups } groupes en { $time }
compute_found_invalid_symlinks = { $number_files } liens symboliques non valides trouvés en { $time }
compute_found_broken_files = Trouvé { $number_files } fichiers cassés en { $time }
compute_found_bad_extensions = { $number_files } fichiers trouvés avec des extensions invalides en { $time }
# Progress window
progress_current_stage = Étape actuelle :{ "  " }
progress_all_stages = Toutes les étapes :{ " " }
# Saving loading 
saving_loading_saving_success = Configuration enregistrée dans le fichier { $name }.
saving_loading_saving_failure = Impossible d'enregistrer les données de configuration dans le fichier { $name }, raison { $reason }.
saving_loading_reset_configuration = La configuration actuelle a été effacée.
saving_loading_loading_success = Configuration de l'application correctement chargée.
saving_loading_no_config_file = Aucun fichier de configuration trouvé, en utilisant les paramètres par défaut.
saving_loading_failed_to_create_config_file = Impossible de créer le fichier de configuration "{ $path }". Raison : "{ $reason }".
saving_loading_failed_to_read_config_file = Impossible de charger la configuration depuis "{ $path }" car elle n'existe pas ou n'est pas un fichier.
saving_loading_failed_to_read_data_from_file = Impossible de lire les données du fichier "{ $path }". Raison : "{ $reason }".
# Other
selected_all_reference_folders = Impossible de lancer la recherche quand tous les répertoires sont définis comme des répertoires de référence
searching_for_data = Recherche de données. Cela peut prendre un certain temps, veuillez patienter...
text_view_messages = MESSAGES
text_view_warnings = AVERTISSEMENTS
text_view_errors = ERREURS
about_window_motto = Ce programme peut être utilisé gratuitement et le sera toujours.
krokiet_new_app = Cette version GTK de Czkawka n'est plus développée depuis la version 12. Pour bénéficier de nouvelles fonctionnalités et d'un développement actif, veuillez utiliser Krokiet, qui est plus stable et plus performant.
# Various dialog
dialogs_ask_next_time = Demander la prochaine fois
symlink_failed = Échec de la liaison symbolique { $name } à { $target }, raison { $reason }
delete_title_dialog = Confirmation de la suppression
delete_question_label = Êtes-vous sûr de vouloir supprimer les fichiers ?
delete_all_files_in_group_title = Confirmation de la suppression de tous les fichiers du groupe
delete_all_files_in_group_label1 = L'ensemble des enregistrements est sélectionné dans certains groupes.
delete_all_files_in_group_label2 = Êtes-vous sûr de vouloir les supprimer ?
delete_items_label = { $items } fichiers seront supprimés.
delete_items_groups_label = { $items } fichiers de { $groups } groupes seront supprimés.
hardlink_failed = Échec de la liaison en dur de { $name } vers { $target }, raison { $reason }
hard_sym_invalid_selection_title_dialog = Sélection invalide avec certains groupes
hard_sym_invalid_selection_label_1 = Un seul enregistrement est sélectionné dans certains groupes et il sera ignoré.
hard_sym_invalid_selection_label_2 = Au moins deux résultats au sein du groupe doivent être sélectionnés pour pouvoir les relier par un lien en dur ou symbolique.
hard_sym_invalid_selection_label_3 = Le premier dans le groupe est reconnu comme original et n'est pas modifié mais les suivants le seront.
hard_sym_link_title_dialog = Confirmation du lien
hard_sym_link_label = Êtes-vous sûr de vouloir relier ces fichiers ?
move_folder_failed = Impossible de déplacer le dossier { $name }. Raison : { $reason }
move_file_failed = Impossible de déplacer le fichier { $name }. Raison : { $reason }
move_files_title_dialog = Choisissez le dossier dans lequel vous voulez déplacer les fichiers dupliqués
move_files_choose_more_than_1_path = Un seul chemin peut être sélectionné pour pouvoir copier leurs fichiers dupliqués. { $path_number } est sélectionné.
move_stats = Éléments { $num_files }/{ $all_files } correctement déplacés
save_results_to_file = Résultats enregistrés dans les fichiers txt et json dans le dossier "{ $name }".
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
