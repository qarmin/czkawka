# Window titles
window_settings_title = Ρυθμίσεις
window_main_title = Czkawka (Hiccup)
window_progress_title = Σάρωση
window_compare_images = Σύγκριση Εικόνων
# General
general_ok_button = Εντάξει
general_close_button = Κλείσιμο
# Main window
music_title_checkbox = Τίτλος
music_artist_checkbox = Καλλιτέχνης
music_year_checkbox = Έτος
music_bitrate_checkbox = Ρυθμός Bit
music_genre_checkbox = Είδος
music_length_checkbox = Μήκος
music_comparison_checkbox = Κατά Προσέγγιση Σύγκριση
music_checking_by_tags = Ετικέτες
music_checking_by_content = Περιεχόμενο
same_music_seconds_label = Ελάχιστη δεύτερη διάρκεια θραύσματος
same_music_similarity_label = Μέγιστη διαφορά
music_compare_only_in_title_group = Σύγκριση μόνο στον τίτλο
music_compare_only_in_title_group_tooltip =
    Όταν ενεργοποιηθεί, τα αρχεία ομαδοποιούνται κατά τίτλο και στη συνέχεια συγκρίνονται μεταξύ τους.
    
    Με 10000 αρχεία, αντ' αυτού σχεδόν 100 εκατομμύρια συγκρίσεις συνήθως θα υπάρχουν περίπου 20000 συγκρίσεις.
same_music_tooltip =
    Η αναζήτηση παρόμοιων αρχείων μουσικής με βάση το περιεχόμενό του μπορεί να ρυθμιστεί με τη ρύθμιση:
    
    - Ο ελάχιστος χρόνος θραύσματος μετά το οποίο τα αρχεία μουσικής μπορούν να προσδιοριστούν ως παρόμοια
    - Η μέγιστη διαφορά διαφοράς μεταξύ δύο δοκιμαζόμενων θραυσμάτων
    
    Το κλειδί για καλά αποτελέσματα είναι να βρεθούν λογικοί συνδυασμοί αυτών των παραμέτρων, για παρέχονται.
    
    Ο ορισμός του ελάχιστου χρόνου σε 5s και η μέγιστη διαφορά σε 1.0, θα αναζητήσει σχεδόν πανομοιότυπα θραύσματα στα αρχεία.
    Ένας χρόνος 20 δευτερολέπτων και μια μέγιστη διαφορά 6.0, από την άλλη πλευρά, λειτουργεί καλά για την εύρεση remixes/live εκδόσεις κλπ.
    
    Από προεπιλογή, κάθε αρχείο μουσικής συγκρίνεται μεταξύ τους και αυτό μπορεί να πάρει πολύ χρόνο κατά τη δοκιμή πολλών αρχείων, έτσι είναι συνήθως καλύτερο να χρησιμοποιήσετε φακέλους αναφοράς και να προσδιορίσετε ποια αρχεία πρέπει να συγκρίνονται μεταξύ τους (με την ίδια ποσότητα αρχείων, η σύγκριση των δακτυλικών αποτυπωμάτων θα είναι γρηγορότερη τουλάχιστον 4x από ό, τι χωρίς φακέλους αναφοράς).
music_comparison_checkbox_tooltip =
    Ψάχνει για παρόμοια αρχεία μουσικής χρησιμοποιώντας AI, το οποίο χρησιμοποιεί μηχανική μάθηση για να αφαιρέσει παρένθεση από μια φράση. Για παράδειγμα, με αυτήν την επιλογή ενεργοποιημένη, τα εν λόγω αρχεία θα θεωρούνται διπλότυπα:
    
    S’wieald dzizśło’b --- S’wieřdziz’ło’b (Remix Lato 2021)
duplicate_case_sensitive_name = Διάκριση Πεζών/ Κεφαλαίων
duplicate_case_sensitive_name_tooltip =
    When enabled, group only records when they have exactly same name e.g. Żołd <-> Żołd
    
    Disabling such option will group names without checking if each letter is same size e.g. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Μέγεθος και όνομα
duplicate_mode_name_combo_box = Όνομα
duplicate_mode_size_combo_box = Μέγεθος
duplicate_mode_hash_combo_box = Κατακερματισμός
duplicate_hash_type_tooltip =
    Czkawka προσφέρει 3 τύπους hashes:
    
    Blake3 - λειτουργία κρυπτογραφικού hash. Αυτή είναι η προεπιλογή επειδή είναι πολύ γρήγορη.
    
    CRC32 - απλή συνάρτηση hash. Αυτό θα πρέπει να είναι πιο γρήγορα από Blake3, αλλά μπορεί πολύ σπάνια να έχει κάποιες συγκρούσεις.
    
    XXH3 - πολύ παρόμοιο στην απόδοση και την ποιότητα hash με Blake3 (αλλά μη κρυπτογραφικό).
duplicate_check_method_tooltip =
    Προς το παρόν, Czkawka προσφέρει τρεις τύπους μεθόδου για να βρείτε αντίγραφα από:
    
    Όνομα - Εύρεση αρχείων που έχουν το ίδιο όνομα.
    
    Μέγεθος - Εύρεση αρχείων με το ίδιο μέγεθος.
    
    Hash - Εύρεση αρχείων με το ίδιο περιεχόμενο. Αυτή η λειτουργία κατακερματίζει το αρχείο και αργότερα συγκρίνει αυτό το κατακερματισμό για να βρείτε διπλότυπα. Αυτή η λειτουργία είναι ο ασφαλέστερος τρόπος για να βρείτε διπλότυπα. Η εφαρμογή χρησιμοποιεί βαριά κρύπτη, έτσι ώστε η δεύτερη και περαιτέρω σάρωση των ίδιων δεδομένων θα πρέπει να είναι πολύ πιο γρήγορα από την πρώτη.
image_hash_size_tooltip =
    Κάθε επιλεγμένη εικόνα παράγει ένα ειδικό hash το οποίο μπορεί να συγκριθεί μεταξύ τους, και μια μικρή διαφορά μεταξύ τους σημαίνει ότι αυτές οι εικόνες είναι παρόμοια.
    
    8 μέγεθος hash είναι αρκετά καλό να βρείτε εικόνες που είναι μόνο λίγο παρόμοια με το πρωτότυπο. Με ένα μεγαλύτερο σύνολο εικόνων (>1000), αυτό θα παράγει ένα μεγάλο ποσό ψευδών θετικών, γι 'αυτό συνιστώ να χρησιμοποιήσετε ένα μεγαλύτερο μέγεθος hash σε αυτή την περίπτωση.
    
    16 είναι το προεπιλεγμένο μέγεθος hash το οποίο είναι ένας αρκετά καλός συμβιβασμός ανάμεσα στην εύρεση ακόμη και λίγο παρόμοιες εικόνες και έχει μόνο μια μικρή ποσότητα συγκρούσεων hash.
    
    32 και 64 hashes βρείτε μόνο παρόμοιες εικόνες, αλλά θα πρέπει να έχουν σχεδόν καμία ψευδή θετικά (ίσως εκτός από μερικές εικόνες με άλφα κανάλι).
image_resize_filter_tooltip =
    Για να υπολογιστεί το hash της εικόνας, η βιβλιοθήκη πρέπει πρώτα να το αλλάξει μέγεθος.
    
    Εξαρτάται από τον επιλεγμένο αλγόριθμο, η προκύπτουσα εικόνα που χρησιμοποιείται για τον υπολογισμό του hash θα φαίνεται λίγο διαφορετική.
    
    Ο γρηγορότερος αλγόριθμος που χρησιμοποιείται, αλλά και εκείνος που δίνει τα χειρότερα αποτελέσματα, είναι ο Nearest. Είναι ενεργοποιημένη από προεπιλογή, επειδή με μέγεθος 16x16 hash χαμηλότερη ποιότητα δεν είναι πραγματικά ορατή.
    
    Με μέγεθος κατακερματισμού 8x8 συνιστάται να χρησιμοποιήσετε διαφορετικό αλγόριθμο από το Nearest, για να έχετε καλύτερες ομάδες εικόνων.
image_hash_alg_tooltip =
    Users can choose from one of many algorithms of calculating the hash.
    
    Each has both strong and weaker points and will sometimes give better and sometimes worse results for different images.
    
    So, to determine the best one for you, manual testing is required.
big_files_mode_combobox_tooltip = Επιτρέπει την αναζήτηση για μικρότερα/μεγαλύτερα αρχεία
big_files_mode_label = Ελεγχμένα αρχεία
big_files_mode_smallest_combo_box = Το Μικρότερο
big_files_mode_biggest_combo_box = Το Μεγαλύτερο
main_notebook_duplicates = Αντίγραφο Αρχείων
main_notebook_empty_directories = Άδειοι Κατάλογοι
main_notebook_big_files = Μεγάλα Αρχεία
main_notebook_empty_files = Κενά Αρχεία
main_notebook_temporary = Προσωρινά Αρχεία
main_notebook_similar_images = Παρόμοιες Εικόνες
main_notebook_similar_videos = Παρόμοια Βίντεο
main_notebook_same_music = Αντίγραφο Μουσικής
main_notebook_symlinks = Μη Έγκυρα Symlinks
main_notebook_broken_files = Κατεστραμμένα Αρχεία
main_notebook_bad_extensions = Εσφαλμένες Επεκτάσεις
main_tree_view_column_file_name = Όνομα Αρχείου
main_tree_view_column_folder_name = Όνομα Φακέλου
main_tree_view_column_path = Διαδρομή
main_tree_view_column_modification = Ημερομηνία Τροποποίησης
main_tree_view_column_size = Μέγεθος
main_tree_view_column_similarity = Ομοιότητα
main_tree_view_column_dimensions = Διαστάσεις
main_tree_view_column_title = Τίτλος
main_tree_view_column_artist = Καλλιτέχνης
main_tree_view_column_year = Έτος
main_tree_view_column_bitrate = Ρυθμός Bit
main_tree_view_column_length = Μήκος
main_tree_view_column_genre = Είδος
main_tree_view_column_symlink_file_name = Όνομα Αρχείου Συντόμευσης
main_tree_view_column_symlink_folder = Φάκελος Συντόμευσης
main_tree_view_column_destination_path = Διαδρομή Προορισμού
main_tree_view_column_type_of_error = Τύπος Σφάλματος
main_tree_view_column_current_extension = Τρέχουσα Επέκταση
main_tree_view_column_proper_extensions = Κατάλληλη Επέκταση
main_label_check_method = Έλεγχος μεθόδου
main_label_hash_type = Τύπος κατακερματισμού
main_label_hash_size = Μέγεθος κατακερματισμού
main_label_size_bytes = Μέγεθος (bytes)
main_label_min_size = Ελάχιστο
main_label_max_size = Μέγιστο
main_label_shown_files = Αριθμός εμφανιζόμενων αρχείων
main_label_resize_algorithm = Αλλαγή μεγέθους αλγορίθμου
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = Ήχος
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Αρχειοθέτηση
main_check_box_broken_files_image = Εικόνα
check_button_general_same_size = Αγνόηση ίδιου μεγέθους
check_button_general_same_size_tooltip = Αγνοήστε τα αρχεία με το ίδιο μέγεθος στα αποτελέσματα - συνήθως αυτά είναι 1: 1 διπλότυπα
main_label_size_bytes_tooltip = Μέγεθος αρχείων που θα χρησιμοποιηθούν κατά τη σάρωση
# Upper window
upper_tree_view_included_folder_column_title = Φάκελοι προς αναζήτηση
upper_tree_view_included_reference_column_title = Φάκελοι Αναφοράς
upper_recursive_button = Αναδρομικά
upper_recursive_button_tooltip = Αν επιλεχθεί, αναζητήστε επίσης αρχεία που δεν τοποθετούνται απευθείας σε επιλεγμένους φακέλους.
upper_manual_add_included_button = Χειροκίνητη Προσθήκη
upper_add_included_button = Προσθήκη
upper_remove_included_button = Αφαίρεση
upper_manual_add_excluded_button = Χειροκίνητη Προσθήκη
upper_add_excluded_button = Προσθήκη
upper_remove_excluded_button = Αφαίρεση
upper_manual_add_included_button_tooltip =
    Προσθήκη ονόματος καταλόγου στην αναζήτηση με το χέρι.
    
    Για να προσθέσετε πολλαπλές διαδρομές ταυτόχρονα, διαχωρίστε τις με το ;
    
    /home/roman;/home/rozkaz θα προσθέσετε δύο καταλόγους /home/roman και /home/rozkaz
upper_add_included_button_tooltip = Προσθήκη νέου φακέλου για αναζήτηση.
upper_remove_included_button_tooltip = Διαγραφή καταλόγου από την αναζήτηση.
upper_manual_add_excluded_button_tooltip =
    Προσθήκη εξαιρούμενου ονόματος καταλόγου με το χέρι.
    
    Για να προσθέσετε πολλαπλές διαδρομές ταυτόχρονα, διαχωρίστε τις με το ;
    
    /home/roman;/home/krokiet θα προσθέσει δύο καταλόγους /home/roman και /home/keokiet
upper_add_excluded_button_tooltip = Προσθήκη καταλόγου για να αποκλειστεί στην αναζήτηση.
upper_remove_excluded_button_tooltip = Διαγραφή καταλόγου από αποκλεισμένους.
upper_notebook_items_configuration = Ρύθμιση Στοιχείων
upper_notebook_excluded_directories = Εξαιρούμενοι Κατάλογοι
upper_notebook_included_directories = Συμπεριλαμβανόμενοι Κατάλογοι
upper_allowed_extensions_tooltip =
    Οι επιτρεπόμενες επεκτάσεις πρέπει να διαχωρίζονται με κόμματα (εξ ορισμού είναι διαθέσιμες).
    
    Τα ακόλουθα Macros, τα οποία προσθέτουν πολλαπλές επεκτάσεις ταυτόχρονα, είναι επίσης διαθέσιμα: IMAGE, VIDEO, MUSIC, TEXT.
    
    Χρήση παράδειγμα ".exe, IMAGE, VIDEO, .rar, 7z" - αυτό σημαίνει ότι οι εικόνες (π. χ. . jpg, png), βίντεο (π.χ. avi, mp4), exe, rar και 7z αρχεία θα σαρωθούν.
upper_excluded_extensions_tooltip =
    Λίστα απενεργοποιημένων αρχείων που θα αγνοηθούν κατά τη σάρωση.
    
    Όταν χρησιμοποιείτε και τις δύο επιτρεπόμενες και απενεργοποιημένες επεκτάσεις, αυτή έχει υψηλότερη προτεραιότητα, οπότε το αρχείο δεν θα ελεγχθεί.
upper_excluded_items_tooltip =
    Τα εξαιρούμενα αντικείμενα πρέπει να περιέχουν * μπαλαντέρ και πρέπει να διαχωρίζονται με κόμματα.
    Αυτό είναι πιο αργό από τους Αποκλεισμένους Κατάλογους, οπότε χρησιμοποιήστε το προσεκτικά.
upper_excluded_items = Εξαιρούμενα Αντικείμενα:
upper_allowed_extensions = Επιτρεπόμενες Επεκτάσεις:
upper_excluded_extensions = Απενεργοποιημένες Επεκτάσεις:
# Popovers
popover_select_all = Επιλογή όλων
popover_unselect_all = Αποεπιλογή όλων
popover_reverse = Αντίστροφη Επιλογή
popover_select_all_except_oldest = Επιλογή όλων εκτός από το παλαιότερο
popover_select_all_except_newest = Επιλογή όλων εκτός από το νεότερο
popover_select_one_oldest = Επιλέξτε ένα παλαιότερο
popover_select_one_newest = Επιλέξτε ένα νεότερο
popover_select_custom = Επιλέξτε προσαρμοσμένο
popover_unselect_custom = Αποεπιλογή προσαρμοσμένου
popover_select_all_images_except_biggest = Επιλογή όλων εκτός από το μεγαλύτερο
popover_select_all_images_except_smallest = Επιλογή όλων εκτός των μικρότερων
popover_custom_path_check_button_entry_tooltip =
    Επιλέξτε εγγραφές με διαδρομή.
    
    Παράδειγμα χρήσης:
    /home/pimpek/rzecz.txt μπορεί να βρεθεί με /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Επιλέξτε εγγραφές με ονόματα αρχείων.
    
    Παράδειγμα χρήσης:
    /usr/ping/pong.txt μπορεί να βρεθεί με *ong*
popover_custom_regex_check_button_entry_tooltip =
    Επιλέξτε εγγραφές με καθορισμένο Regex.
    
    Με αυτή τη λειτουργία, το κείμενο αναζήτησης είναι η διαδρομή με το όνομα.
    
    Παράδειγμα χρήσης:
    /usr/bin/ziemniak. xt μπορεί να βρεθεί με /ziem[a-z]+
    
    Αυτό χρησιμοποιεί την προεπιλεγμένη εφαρμογή Rust regex. Μπορείτε να διαβάσετε περισσότερα για αυτό εδώ: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    
    
    Όταν απενεργοποιηθεί το /home/* βρίσκει και το /HoMe/roman και το /home/roman.
popover_custom_not_all_check_button_tooltip =
    Prevents selecting all records in group.
    
    This is enabled by default, because in most situations, you don't want to delete both original and duplicates files, but want to leave at least one file.
    
    WARNING: This setting doesn't work if you have already manually selected all results in a group.
popover_custom_regex_path_label = Διαδρομή
popover_custom_regex_name_label = Όνομα
popover_custom_regex_regex_label = Regex Διαδρομή + Όνομα
popover_custom_case_sensitive_check_button = Διάκριση πεζών/ κεφαλαίων
popover_custom_all_in_group_label = Να μην επιλέγονται όλες οι εγγραφές στην ομάδα
popover_custom_mode_unselect = Αποεπιλογή Προσαρμοσμένου
popover_custom_mode_select = Επιλογή Προσαρμοσμένου
popover_sort_file_name = Όνομα αρχείου
popover_sort_folder_name = Όνομα φακέλου
popover_sort_full_name = Πλήρες όνομα
popover_sort_size = Μέγεθος
popover_sort_selection = Επιλογή
popover_invalid_regex = Regex δεν είναι έγκυρο
popover_valid_regex = Regex είναι έγκυρο
# Bottom buttons
bottom_search_button = Αναζήτηση
bottom_select_button = Επιλογή
bottom_delete_button = Διαγραφή
bottom_save_button = Αποθήκευση
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Μετακίνηση
bottom_sort_button = Ταξινόμηση
bottom_search_button_tooltip = Έναρξη αναζήτησης
bottom_select_button_tooltip = Επιλέξτε εγγραφές. Μόνο επιλεγμένα αρχεία/φάκελοι μπορούν να υποβληθούν σε μεταγενέστερη επεξεργασία.
bottom_delete_button_tooltip = Διαγραφή επιλεγμένων αρχείων/φακέλων.
bottom_save_button_tooltip = Αποθήκευση δεδομένων σχετικά με την αναζήτηση σε αρχείο
bottom_symlink_button_tooltip =
    Δημιουργία συμβολικών συνδέσμων.
    Λειτουργεί μόνο όταν επιλεγούν τουλάχιστον δύο αποτελέσματα σε μια ομάδα.
    Πρώτα παραμένει αμετάβλητη και δεύτερον και αργότερα συνδέεται με την πρώτη.
bottom_hardlink_button_tooltip =
    Δημιουργία hardlinks.
    λειτουργεί μόνο όταν επιλεγούν τουλάχιστον δύο αποτελέσματα σε μια ομάδα.
    Πρώτα παραμένει αμετάβλητη και η δεύτερη και αργότερα συνδέονται σκληρά με την πρώτη.
bottom_hardlink_button_not_available_tooltip =
    Δημιουργία hardlinks. Το κουμπί
    είναι απενεργοποιημένο, επειδή οι hardlinks δεν μπορούν να δημιουργηθούν.
    Hardlinks λειτουργεί μόνο με δικαιώματα διαχειριστή στα Windows, οπότε φροντίστε να εκτελέσετε την εφαρμογή ως διαχειριστής.
    Εάν η εφαρμογή λειτουργεί ήδη με τέτοια δικαιώματα ελέγξτε για παρόμοια ζητήματα στο Github.
bottom_move_button_tooltip =
    Moves files to chosen directory.
    It copies all files to the directory without preserving the directory tree.
    When trying to move two files with identical name to folder, second will fail and show error.
bottom_sort_button_tooltip = Ταξινόμηση αρχείων/φακέλων σύμφωνα με την επιλεγμένη μέθοδο.
bottom_show_errors_tooltip = Εμφάνιση/Απόκρυψη πίνακα κάτω κειμένου.
bottom_show_upper_notebook_tooltip = Εμφάνιση/Απόκρυψη ανώτερου πίνακα σημειωμάτων.
# Progress Window
progress_stop_button = Διακοπή
progress_stop_additional_message = Η διακοπή ζητήθηκε
# About Window
about_repository_button_tooltip = Σύνδεσμος προς σελίδα αποθετηρίου με πηγαίο κώδικα.
about_donation_button_tooltip = Σύνδεση με τη σελίδα δωρεών.
about_instruction_button_tooltip = Σύνδεσμος στη σελίδα οδηγιών.
about_translation_button_tooltip = Σύνδεσμος προς τη σελίδα του Crowdin με μεταφράσεις εφαρμογών. Υιοθετούνται επίσημα πολωνικά και αγγλικά.
about_repository_button = Αποθετήριο
about_donation_button = Δωρεά
about_instruction_button = Οδηγίες
about_translation_button = Μετάφραση
# Header
header_setting_button_tooltip = Άνοιγμα διαλόγου ρυθμίσεων.
header_about_button_tooltip = Άνοιγμα διαλόγου με πληροφορίες σχετικά με την εφαρμογή.

# Settings


## General

settings_number_of_threads = Αριθμός χρησιμοποιημένων νημάτων
settings_number_of_threads_tooltip = Αριθμός χρησιμοποιημένων νημάτων, 0 σημαίνει ότι θα χρησιμοποιηθούν όλα τα διαθέσιμα νήματα.
settings_use_rust_preview = Χρήση εξωτερικών βιβλιοθηκών αντ' αυτού gtk για φόρτωση προεπισκοπήσεων
settings_use_rust_preview_tooltip =
    Χρησιμοποιώντας gtk προεπισκοπήσεις θα είναι μερικές φορές πιο γρήγορα και να υποστηρίξει περισσότερες μορφές, αλλά μερικές φορές αυτό θα μπορούσε να είναι ακριβώς το αντίθετο.
    
    Αν έχετε προβλήματα με τη φόρτωση προεπισκόπησης, μπορείτε να προσπαθήσετε να αλλάξετε αυτή τη ρύθμιση.
    
    Σε συστήματα χωρίς linux, συνιστάται να χρησιμοποιήσετε αυτήν την επιλογή, επειδή το gtk-pixbuf δεν είναι πάντα διαθέσιμο εκεί, οπότε η απενεργοποίηση αυτής της επιλογής δεν θα φορτώσει προεπισκοπήσεις ορισμένων εικόνων.
settings_label_restart = Πρέπει να επανεκκινήσετε την εφαρμογή για να εφαρμόσετε τις ρυθμίσεις!
settings_ignore_other_filesystems = Αγνόηση άλλων συστημάτων αρχείων (μόνο Linux)
settings_ignore_other_filesystems_tooltip =
    αγνοεί αρχεία που δεν είναι στο ίδιο σύστημα αρχείων με αναζήτηση καταλόγων.
    
    Λειτουργεί όπως η επιλογή -xdev στην εντολή εύρεσης στο Linux
settings_save_at_exit_button_tooltip = Αποθήκευση ρυθμίσεων σε αρχείο κατά το κλείσιμο της εφαρμογής.
settings_load_at_start_button_tooltip =
    Φόρτωση παραμέτρων από το αρχείο κατά το άνοιγμα της εφαρμογής.
    
    Αν δεν είναι ενεργοποιημένη, θα χρησιμοποιηθούν οι προεπιλεγμένες ρυθμίσεις.
settings_confirm_deletion_button_tooltip = Εμφάνιση διαλόγου επιβεβαίωσης όταν κάνετε κλικ στο κουμπί διαγραφής.
settings_confirm_link_button_tooltip = Εμφάνιση διαλόγου επιβεβαίωσης όταν κάνετε κλικ στο κουμπί hard/symlink
settings_confirm_group_deletion_button_tooltip = Εμφάνιση διαλόγου προειδοποίησης όταν προσπαθείτε να διαγράψετε όλες τις εγγραφές από την ομάδα.
settings_show_text_view_button_tooltip = Εμφάνιση πίνακα κειμένου στο κάτω μέρος της διεπαφής χρήστη.
settings_use_cache_button_tooltip = Χρήση προσωρινής μνήμης αρχείων.
settings_save_also_as_json_button_tooltip = Αποθήκευση προσωρινής μνήμης σε (αναγνώσιμη από άνθρωπο) μορφή JSON. Είναι δυνατή η τροποποίηση του περιεχομένου του. Η προσωρινή μνήμη από αυτό το αρχείο θα διαβάζεται αυτόματα από την εφαρμογή αν λείπει η κρύπτη δυαδικής μορφής (με επέκταση κάδου).
settings_use_trash_button_tooltip = Μετακινεί τα αρχεία στον κάδο απορριμμάτων αντί να τα διαγράφει μόνιμα.
settings_language_label_tooltip = Γλώσσα διεπαφής χρήστη.
settings_save_at_exit_button = Αποθήκευση ρυθμίσεων κατά το κλείσιμο της εφαρμογής
settings_load_at_start_button = Φόρτωση ρυθμίσεων κατά το άνοιγμα της εφαρμογής
settings_confirm_deletion_button = Εμφάνιση διαλόγου επιβεβαίωσης κατά τη διαγραφή αρχείων
settings_confirm_link_button = Εμφάνιση διαλόγου επιβεβαίωσης όταν σκληρά/συντόμευση αρχείων
settings_confirm_group_deletion_button = Εμφάνιση διαλόγου επιβεβαίωσης κατά τη διαγραφή όλων των αρχείων της ομάδας
settings_show_text_view_button = Εμφάνιση κάτω πίνακα κειμένου
settings_use_cache_button = Χρήση προσωρινής μνήμης
settings_save_also_as_json_button = Επίσης αποθήκευση προσωρινής μνήμης ως αρχείο JSON
settings_use_trash_button = Μετακίνηση διαγραμμένων αρχείων στον κάδο απορριμμάτων
settings_language_label = Γλώσσα
settings_multiple_delete_outdated_cache_checkbutton = Αυτόματη διαγραφή ξεπερασμένων καταχωρήσεων cache
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Delete outdated cache results which point to non-existent files.
    
    When enabled, app makes sure when loading records, that all records point to valid files (broken ones are ignored).
    
    Disabling this will help when scanning files on external drives, so cache entries about them will not be purged in the next scan.
    
    In the case of having hundred of thousands records in cache, it is suggested to enable this, which will speedup cache loading/saving at start/end of the scan.
settings_notebook_general = Γενικά
settings_notebook_duplicates = Διπλότυπα
settings_notebook_images = Παρόμοιες Εικόνες
settings_notebook_videos = Παρόμοια Βίντεο

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Εμφανίζει την προεπισκόπηση στη δεξιά πλευρά (όταν επιλέγετε ένα αρχείο εικόνας).
settings_multiple_image_preview_checkbutton = Εμφάνιση προεπισκόπησης εικόνας
settings_multiple_clear_cache_button_tooltip =
    Χειροκίνητη εκκαθάριση της λανθάνουσας μνήμης των ξεπερασμένων καταχωρήσεων.
    Αυτό θα πρέπει να χρησιμοποιηθεί μόνο αν η αυτόματη εκκαθάριση έχει απενεργοποιηθεί.
settings_multiple_clear_cache_button = Κατάργηση παρωχημένων αποτελεσμάτων από τη μνήμη cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Hides all files except one, if all point to the same data (are hardlinked).
    
    Example: In the case where there are (on disk) seven files which are hardlinked to specific data and one different file with same data but a different inode, then in duplicate finder, only one unique file and one file from hardlinked ones will be shown.
settings_duplicates_minimal_size_entry_tooltip =
    Ορίστε το ελάχιστο μέγεθος αρχείου που θα αποθηκευτεί.
    
    Επιλέγοντας μια μικρότερη τιμή θα δημιουργήσει περισσότερες εγγραφές. Αυτό θα επιταχύνει την αναζήτηση, αλλά επιβραδύνει τη φόρτωση της λανθάνουσας μνήμης.
settings_duplicates_prehash_checkbutton_tooltip =
    Ενεργοποιεί την προσωρινή αποθήκευση του prehash (ένα κατακερματισμό υπολογισμένο από ένα μικρό μέρος του αρχείου) το οποίο επιτρέπει την προηγούμενη απόρριψη μη διπλών αποτελεσμάτων.
    
    Είναι απενεργοποιημένο από προεπιλογή επειδή μπορεί να προκαλέσει επιβραδύνσεις σε ορισμένες περιπτώσεις.
    
    Συνιστάται ιδιαίτερα να το χρησιμοποιήσετε κατά τη σάρωση εκατοντάδων χιλιάδων ή εκατομμυρίων αρχείων, επειδή μπορεί να επιταχύνει την αναζήτηση κατά πολλές φορές.
settings_duplicates_prehash_minimal_entry_tooltip = Ελάχιστο μέγεθος της προσωρινά αποθηκευμένης καταχώρησης.
settings_duplicates_hide_hard_link_button = Απόκρυψη σκληρών συνδέσμων (μόνο Linux και macOS)
settings_duplicates_prehash_checkbutton = Χρήση προσωρινής μνήμης prehash
settings_duplicates_minimal_size_cache_label = Ελάχιστο μέγεθος των αρχείων (σε byte) αποθηκεύονται στη μνήμη cache
settings_duplicates_minimal_size_cache_prehash_label = Ελάχιστο μέγεθος των αρχείων (σε byte) αποθηκεύονται στην προσωρινή μνήμη prehash

## Saving/Loading settings

settings_saving_button_tooltip = Αποθήκευση των τρεχουσών ρυθμίσεων ρυθμίσεων στο αρχείο.
settings_loading_button_tooltip = Φόρτωση ρυθμίσεων από το αρχείο και αντικατάσταση των τρεχουσών ρυθμίσεων με αυτές.
settings_reset_button_tooltip = Επαναφορά των τρεχουσών ρυθμίσεων στην προκαθορισμένη.
settings_saving_button = Αποθήκευση διαμόρφωσης
settings_loading_button = Φόρτωση διαμόρφωσης
settings_reset_button = Επαναφορά ρύθμισης παραμέτρων

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Opens the folder where the cache txt files are stored.
    
    Modifying the cache files may cause invalid results to be shown. However, modifying path may save time when moving a big amount of files to a different location.
    
    You can copy these files between computers to save time on scanning again for files (of course if they have similar directory structure).
    
    In the case of problems with the cache, these files can be removed. The app will automatically regenerate them.
settings_folder_settings_open_tooltip =
    Ανοίγει το φάκελο όπου αποθηκεύεται η ρύθμιση Czkawka.
    
    ΠΡΟΕΙΔΟΠΟΙΗΣΗ: Η χειροκίνητη τροποποίηση της ρύθμισης μπορεί να σπάσει τη ροή εργασίας σας.
settings_folder_cache_open = Άνοιγμα φακέλου cache
settings_folder_settings_open = Άνοιγμα φακέλου ρυθμίσεων
# Compute results
compute_stopped_by_user = Η αναζήτηση σταμάτησε από το χρήστη
compute_found_duplicates_hash_size = Found { $number_files } duplicates in { $number_groups } groups which took { $size }
compute_found_duplicates_name = Βρέθηκαν διπλότυπα { $number_files } σε ομάδες { $number_groups }
compute_found_empty_folders = Βρέθηκαν { $number_files } άδειοι φάκελοι
compute_found_empty_files = Βρέθηκαν { $number_files } άδεια αρχεία
compute_found_big_files = Βρέθηκαν { $number_files } μεγάλα αρχεία
compute_found_temporary_files = Βρέθηκαν { $number_files } προσωρινά αρχεία
compute_found_images = Βρέθηκαν { $number_files } παρόμοιες εικόνες σε ομάδες { $number_groups }
compute_found_videos = Βρέθηκαν { $number_files } παρόμοια βίντεο σε ομάδες { $number_groups }
compute_found_music = Βρέθηκαν { $number_files } παρόμοια αρχεία μουσικής σε ομάδες { $number_groups }
compute_found_invalid_symlinks = Βρέθηκαν { $number_files } μη έγκυρες συμβολικές συνδέσεις
compute_found_broken_files = Βρέθηκαν { $number_files } κατεστραμμένα αρχεία
compute_found_bad_extensions = Βρέθηκαν { $number_files } αρχεία με μη έγκυρες επεκτάσεις
# Progress window
progress_scanning_general_file = Σάρωση { $file_number } αρχείου
progress_scanning_extension_of_files = Έλεγχος επέκτασης αρχείου { $file_checked }/{ $all_files }
progress_scanning_broken_files = Έλεγχος αρχείου { $file_checked }/{ $all_files }
progress_scanning_video = Hashing of { $file_checked }/{ $all_files } βίντεο
progress_scanning_image = Hashing of { $file_checked }/{ $all_files } image
progress_comparing_image_hashes = Σύγκριση { $file_checked }/{ $all_files } κατακερματισμού εικόνας
progress_scanning_music_tags_end = Συγκρίνοντας ετικέτες του αρχείου μουσικής { $file_checked }/{ $all_files }
progress_scanning_music_tags = Ανάγνωση ετικετών του αρχείου μουσικής { $file_checked }/{ $all_files }
progress_scanning_music_content_end = Σύγκριση δακτυλικού αποτυπώματος του αρχείου μουσικής { $file_checked }/{ $all_files }
progress_scanning_music_content = Υπολογισμός δακτυλικού αποτυπώματος του αρχείου μουσικής { $file_checked }/{ $all_files }
progress_scanning_empty_folders = Σάρωση φακέλου { $folder_number }
progress_scanning_size = Μέγεθος σάρωσης του αρχείου { $file_number }
progress_scanning_size_name = Σάρωση ονόματος και μεγέθους αρχείου { $file_number }
progress_scanning_name = Σάρωση ονόματος αρχείου { $file_number }
progress_analyzed_partial_hash = Αναλυμένο μερικό κατακερματισμό των αρχείων { $file_checked }/{ $all_files }
progress_analyzed_full_hash = Ανάλυση πλήρους hash των { $file_checked }/{ $all_files } αρχείων
progress_prehash_cache_loading = Φόρτωση προσωρινής μνήμης
progress_prehash_cache_saving = Αποθήκευση προσωρινής μνήμης prehash
progress_hash_cache_loading = Φόρτωση προσωρινής μνήμης hash
progress_hash_cache_saving = Αποθήκευση λανθάνουσας μνήμης
progress_cache_loading = Φόρτωση προσωρινής μνήμης
progress_cache_saving = Αποθήκευση προσωρινής μνήμης
progress_current_stage = Current Stage:{ "  " }
progress_all_stages = Όλα Τα Στάδια:{" " }
# Saving loading 
saving_loading_saving_success = Αποθηκευμένες ρυθμίσεις για το αρχείο { $name }.
saving_loading_saving_failure = Αποτυχία αποθήκευσης δεδομένων ρύθμισης παραμέτρων στο αρχείο { $name }.
saving_loading_reset_configuration = Οι τρέχουσες ρυθμίσεις εκκαθαρίστηκαν.
saving_loading_loading_success = Τοποθετημένες ρυθμίσεις παραμέτρων εφαρμογής.
saving_loading_invalid_string = Για το κλειδί "{ $key }" βρήκε μη έγκυρο αποτέλεσμα - "{ $result }" το οποίο δεν είναι συμβολοσειρά.
saving_loading_invalid_int = Για το κλειδί "{ $key }" βρήκε μη έγκυρο αποτέλεσμα - "{ $result }" που δεν είναι ακέραιος.
saving_loading_invalid_bool = Για το κλειδί "{ $key }" βρήκε μη έγκυρο αποτέλεσμα - "{ $result }" που δεν είναι bool.
saving_loading_decode_problem_bool = Αποτυχία αποκωδικοποίησης bool από το κλειδί "{ $key }" βρέθηκε "{ $result }" αλλά οι επιτρεπόμενες τιμές είναι 0, 1, αληθείς ή ψευδείς.
saving_loading_saving_same_keys = Προσπάθεια εξοικονόμησης ρυθμίσεων με διπλό κλειδί "{ $key }".
saving_loading_failed_to_get_home_directory = Αποτυχία λήψης του αρχικού φακέλου για να ανοίξετε/αποθηκεύσετε το αρχείο ρυθμίσεων.
saving_loading_folder_config_instead_file = Αδυναμία δημιουργίας ή ανοίγματος αρχείου αποθήκευσης στη διαδρομή "{ $path }" επειδή ήδη υπάρχει φάκελος.
saving_loading_failed_to_create_configuration_folder = Αποτυχία ρύθμισης παραμέτρων για τη δημιουργία φακέλου ρυθμίσεων "{ $path }", λόγος "{ $reason }".
saving_loading_failed_to_create_config_file = Αποτυχία δημιουργίας αρχείου ρυθμίσεων "{ $path }", λόγος "{ $reason }".
saving_loading_failed_to_read_config_file = Αδυναμία φόρτωσης ρύθμισης παραμέτρων από το "{ $path }" επειδή δεν υπάρχει ή δεν είναι αρχείο.
saving_loading_failed_to_read_data_from_file = Αδυναμία ανάγνωσης δεδομένων από το αρχείο "{ $path }", λόγος "{ $reason }".
saving_loading_orphan_data = Βρέθηκαν ορφανά δεδομένα "{ $data }" στη γραμμή "{ $line }".
saving_loading_not_valid = Η ρύθμιση "{ $data }" δεν υπάρχει στην τρέχουσα έκδοση εφαρμογών.
# Invalid symlinks
invalid_symlink_infinite_recursion = Άπειρη αναδρομή
invalid_symlink_non_existent_destination = Αρχείο ανύπαρκτου προορισμού
# Other
selected_all_reference_folders = Αδυναμία έναρξης αναζήτησης, όταν όλοι οι κατάλογοι ορίζονται ως φάκελοι αναφοράς
searching_for_data = Αναζήτηση δεδομένων, μπορεί να πάρει λίγο, παρακαλώ περιμένετε...
text_view_messages = ΜΗΝΥΜΑΤΑ
text_view_warnings = ΠΡΟΕΙΔΟΠΟΙΗΣΕΙΣ
text_view_errors = ΣΦΑΛΜΑ
about_window_motto = Αυτό το πρόγραμμα είναι ελεύθερο να χρησιμοποιηθεί και πάντα θα είναι.
# Various dialog
dialogs_ask_next_time = Ερώτηση την επόμενη φορά
delete_file_failed = Αποτυχία διαγραφής αρχείου { $name }, λόγος { $reason }
delete_title_dialog = Διαγραφή επιβεβαίωσης
delete_question_label = Είστε βέβαιοι ότι θέλετε να διαγράψετε αρχεία?
delete_all_files_in_group_title = Επιβεβαίωση διαγραφής όλων των αρχείων της ομάδας
delete_all_files_in_group_label1 = Σε ορισμένες ομάδες έχουν επιλεγεί όλες οι εγγραφές.
delete_all_files_in_group_label2 = Είστε βέβαιοι ότι θέλετε να τα διαγράψετε?
delete_folder_failed = Αποτυχία διαγραφής του φακέλου { $dir } επειδή ο φάκελος δεν υπάρχει, δεν έχετε άδεια ή ο φάκελος δεν είναι κενός.
delete_items_label = { $items } τα αρχεία θα διαγραφούν.
delete_items_groups_label = { $items } τα αρχεία από τις ομάδες { $groups } θα διαγραφούν.
hardlink_failed = Αποτυχία hardlink
hard_sym_invalid_selection_title_dialog = Μη έγκυρη επιλογή με κάποιες ομάδες
hard_sym_invalid_selection_label_1 = Σε ορισμένες ομάδες έχει επιλεγεί μόνο μία εγγραφή και θα αγνοηθεί.
hard_sym_invalid_selection_label_2 = Για να είναι δυνατή η σκληρή/συσχέτιση αυτών των αρχείων, πρέπει να επιλεγούν τουλάχιστον δύο αποτελέσματα στην ομάδα.
hard_sym_invalid_selection_label_3 = Η πρώτη στην ομάδα αναγνωρίζεται ως πρωτότυπο και δεν αλλάζεται, αλλά η δεύτερη και αργότερα τροποποιείται.
hard_sym_link_title_dialog = Επιβεβαίωση συνδέσμου
hard_sym_link_label = Είστε βέβαιοι ότι θέλετε να συνδέσετε αυτά τα αρχεία?
move_folder_failed = Αποτυχία μετακίνησης του φακέλου { $name }, λόγος { $reason }
move_file_failed = Αποτυχία μετακίνησης αρχείου { $name }, λόγος { $reason }
move_files_title_dialog = Επιλέξτε φάκελο στον οποίο θέλετε να μετακινήσετε διπλότυπα αρχεία
move_files_choose_more_than_1_path = Μόνο μία διαδρομή μπορεί να επιλεγεί για να είναι σε θέση να αντιγράψει τα διπλά αρχεία τους, επιλεγμένα { $path_number }.
move_stats = Σωστά μετακινήθηκαν { $num_files }/{ $all_files } στοιχεία
save_results_to_file = Αποθηκεύτηκε αποτελέσματα τόσο σε txt και αρχεία json στο φάκελο { $name }.
search_not_choosing_any_music = ΣΦΑΛΜΑ: Πρέπει να επιλέξετε τουλάχιστον ένα πλαίσιο ελέγχου με τύπους αναζήτησης μουσικής.
search_not_choosing_any_broken_files = ΣΦΑΛΜΑ: Πρέπει να επιλέξετε τουλάχιστον ένα πλαίσιο ελέγχου με τον τύπο των επιλεγμένων κατεστραμμένων αρχείων.
include_folders_dialog_title = Φάκελοι που θα συμπεριληφθούν
exclude_folders_dialog_title = Φάκελοι προς εξαίρεση
include_manually_directories_dialog_title = Προσθήκη καταλόγου χειροκίνητα
cache_properly_cleared = Σωστό εκκαθάριση προσωρινής μνήμης
cache_clear_duplicates_title = Εκκαθάριση διπλότυπων cache
cache_clear_similar_images_title = Εκκαθάριση παρόμοιων εικόνων cache
cache_clear_similar_videos_title = Εκκαθάριση παρόμοιων βίντεο cache
cache_clear_message_label_1 = Θέλετε να καθαρίσετε την προσωρινή μνήμη των ξεπερασμένων καταχωρήσεων?
cache_clear_message_label_2 = Αυτή η λειτουργία θα καταργήσει όλες τις καταχωρήσεις προσωρινής αποθήκευσης που δείχνουν σε μη έγκυρα αρχεία.
cache_clear_message_label_3 = Αυτό μπορεί να επιταχύνει ελαφρώς τη φόρτωση/αποθήκευση στη μνήμη cache.
cache_clear_message_label_4 = ΠΡΟΕΙΔΟΠΟΙΗΣΗ: Η λειτουργία θα αφαιρέσει όλα τα προσωρινά αποθηκευμένα δεδομένα από τις αποσυνδεδεμένες εξωτερικές μονάδες. Έτσι, κάθε hash θα πρέπει να αναγεννηθεί.
# Show preview
preview_image_resize_failure = Αποτυχία αλλαγής μεγέθους εικόνας { $name }.
preview_image_opening_failure = Αποτυχία ανοίγματος εικόνας { $name }, λόγος { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Ομάδα { $current_group }/{ $all_groups } ({ $images_in_group } εικόνες)
compare_move_left_button = L
compare_move_right_button = R
