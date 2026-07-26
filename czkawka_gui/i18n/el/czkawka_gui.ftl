# Window titles
window_settings_title = Ρυθμίσεις
window_main_title = Czkawka (λόξιγκας)
window_progress_title = Σάρωση
window_compare_images = Σύγκριση Εικόνων
# General
general_ok_button = Εντάξει
general_close_button = Κλείσιμο
# Krokiet info dialog
krokiet_info_title = Ανακοίνωση Κατάργησης
krokiet_info_message =
    Czkawka GTK 12.0 είναι η τελική έκδοση. Δεν έχουν προγραμματιστεί περαιτέρω ενημερώσεις, χαρακτηριστικά ή διορθώσεις σφαλμάτων.
    
    Τα περισσότερα χαρακτηριστικά από Czkawka GTK είναι διαθέσιμα στο Krokiet, συνήθως σε απλούστερη, ταχύτερη και πιο σταθερή μορφή. Το Krokiet προσθέτει επίσης νέα χαρακτηριστικά και βελτιώσεις που δεν ήταν δυνατές στην έκδοση GTK.
    
    Αν εξακολουθείτε να χρησιμοποιείτε Czkawka GTK, η μετάβαση στο Krokiet θα πρέπει να είναι εύκολη, αφού έχει μια παρόμοια διεπαφή, λιγότερες εξαρτήσεις, και καλύτερη υποστήριξη μεταξύ πλατφορμών.
    
    PS: Αυτό το μήνυμα θα πρέπει να εμφανίζεται μόνο μία φορά. Αν εμφανιστεί ξανά, ορίστε τη μεταβλητή περιβάλλοντος CZKAWKA_DONT_ANNOY_ME σε οποιαδήποτε μη κενή τιμή.
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
same_music_seconds_label = Ελάχιστη διάρκεια θραύσματος σε δευτερόλεπτα
same_music_similarity_label = Μέγιστη διαφορά
music_compare_only_in_title_group = Σύγκριση εντός ομάδων παρόμοιων τίτλων
music_compare_only_in_title_group_tooltip =
    Όταν ενεργοποιηθεί, τα αρχεία ομαδοποιούνται κατά τίτλο και στη συνέχεια συγκρίνονται μεταξύ τους.
    
    Με 10000 αρχεία, αντί για σχεδόν 100 εκατομμύρια συγκρίσεις συνήθως θα υπάρχουν περίπου 20000 συγκρίσεις.
same_music_tooltip =
    Η αναζήτηση παρόμοιων αρχείων μουσικής με βάση το περιεχόμενό τους μπορεί να ρυθμιστεί με τη ρύθμιση:
    
    - Ο ελάχιστος χρόνος θραύσματος μετά το οποίο τα αρχεία μουσικής μπορούν να προσδιοριστούν ως παρόμοια
    - Η μέγιστη διαφορά μεταξύ δύο δοκιμαζόμενων θραυσμάτων
    
    Το κλειδί για καλά αποτελέσματα είναι να βρεθούν λογικοί συνδυασμοί αυτών των παραμέτρων που παρέχονται.
    
    Ο ορισμός του ελάχιστου χρόνου σε 5s και η μέγιστη διαφορά σε 1.0, θα αναζητήσει σχεδόν πανομοιότυπα θραύσματα στα αρχεία.
    Ένας χρόνος 20 δευτερολέπτων και μια μέγιστη διαφορά 6.0, από την άλλη πλευρά, λειτουργεί καλά για την εύρεση remixes/live εκδόσεις κλπ.
    
    Από προεπιλογή, κάθε αρχείο μουσικής συγκρίνεται με όλα τα υπόλοιπα και αυτό μπορεί να πάρει πολύ χρόνο κατά τη δοκιμή πολλών αρχείων, έτσι είναι συνήθως καλύτερο να χρησιμοποιήσετε φακέλους αναφοράς και να προσδιορίσετε ποια αρχεία πρέπει να συγκρίνονται μεταξύ τους (με την ίδια ποσότητα αρχείων, η σύγκριση των δακτυλικών αποτυπωμάτων θα είναι γρηγορότερη τουλάχιστον 4x από ό,τι χωρίς φακέλους αναφοράς).
music_comparison_checkbox_tooltip =
    Ψάχνει για παρόμοια αρχεία μουσικής χρησιμοποιώντας AI, το οποίο χρησιμοποιεί μηχανική μάθηση για να αφαιρέσει παρενθέσεις από μια φράση. Για παράδειγμα, με αυτήν την επιλογή ενεργοποιημένη, τα εν λόγω αρχεία θα θεωρούνται διπλότυπα:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Διάκριση Πεζών/Κεφαλαίων
duplicate_case_sensitive_name_tooltip =
    Όταν είναι ενεργοποιημένη, η συγχώνευση πραγματοποιείται μόνο για εγγραφές που έχουν ακριβώς το ίδιο όνομα, π.χ. Żołd <-> Żołd
    
    Η απενεργοποίηση αυτής της επιλογής θα συγχωνεύσει ονόματα χωρίς να ελέγχει αν κάθε γράμμα έχει το ίδιο μέγεθος, π.χ. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Μέγεθος και όνομα
duplicate_mode_name_combo_box = Όνομα
duplicate_mode_size_combo_box = Μέγεθος
duplicate_mode_hash_combo_box = Κατακερματισμός
duplicate_hash_type_tooltip =
    Czkawka προσφέρει 3 τύπους κατακερματισμών:
    
    Blake3 - συνάρτηση κρυπτογραφικού κατακερματισμού. Αυτή είναι η προεπιλογή επειδή είναι πολύ γρήγορη.
    
    CRC32 - απλή συνάρτηση κατακερματισμού. Αυτή θα πρέπει να είναι πιο γρήγορη από το Blake3, αλλά μπορεί πολύ σπάνια να έχει κάποιες συγκρούσεις.
    
    XXH3 - πολύ παρόμοιο στην απόδοση και την ποιότητα κατακερματισμού με το Blake3 (αλλά μη κρυπτογραφικό). Έτσι, αυτές οι λειτουργίες μπορούν να εναλλάσσονται εύκολα μεταξύ τους.
duplicate_check_method_tooltip =
    Προς το παρόν, το Czkawka προσφέρει τρεις τύπους μεθόδου για να βρείτε διπλότυπα:
    
    Όνομα - Εύρεση αρχείων που έχουν το ίδιο όνομα.
    
    Μέγεθος - Εύρεση αρχείων με το ίδιο μέγεθος.
    
    Κατακερματισμός - Εύρεση αρχείων με το ίδιο περιεχόμενο. Αυτή η λειτουργία κατακερματίζει το αρχείο και αργότερα συγκρίνει αυτόν τον κατακερματισμό για να βρείτε διπλότυπα. Αυτή η λειτουργία είναι ο ασφαλέστερος τρόπος για να βρείτε διπλότυπα. Η εφαρμογή χρησιμοποιεί εκτενώς την προσωρινή μνήμη, έτσι ώστε η δεύτερη και οι επόμενες σαρώσεις των ίδιων δεδομένων να είναι πολύ πιο γρήγορες από την πρώτη.
image_hash_size_tooltip =
    Κάθε ελεγμένη εικόνα παράγει έναν ειδικό κατακερματισμό ο οποίος μπορεί να συγκριθεί με τους άλλους, και μια μικρή διαφορά μεταξύ τους σημαίνει ότι αυτές οι εικόνες είναι παρόμοιες.
    
    Το μέγεθος κατακερματισμού 8 είναι αρκετά καλό για να βρείτε εικόνες που είναι μόνο λίγο παρόμοιες με το πρωτότυπο. Με ένα μεγαλύτερο σύνολο εικόνων (>1000), αυτό θα παράγει μεγάλο αριθμό ψευδών θετικών, γι' αυτό συνιστώ να χρησιμοποιήσετε μεγαλύτερο μέγεθος κατακερματισμού σε αυτή την περίπτωση.
    
    Το 16 είναι το προεπιλεγμένο μέγεθος κατακερματισμού, το οποίο είναι αρκετά καλός συμβιβασμός ανάμεσα στην εύρεση έστω και λίγο παρόμοιων εικόνων και στο να έχει μόνο μικρό αριθμό συγκρούσεων κατακερματισμού.
    
    Τα μεγέθη κατακερματισμού 32 και 64 βρίσκουν μόνο πολύ παρόμοιες εικόνες, αλλά δεν θα πρέπει να έχουν σχεδόν καθόλου ψευδή θετικά (ίσως εκτός από μερικές εικόνες με κανάλι άλφα).
image_resize_filter_tooltip =
    Για να υπολογιστεί ο κατακερματισμός μιας εικόνας, η βιβλιοθήκη πρέπει πρώτα να αλλάξει το μέγεθός της.
    
    Ανάλογα με τον επιλεγμένο αλγόριθμο, η εικόνα που προκύπτει και χρησιμοποιείται για τον υπολογισμό του κατακερματισμού θα φαίνεται λίγο διαφορετική.
    
    Ο ταχύτερος αλγόριθμος, αλλά και αυτός που δίνει τα χειρότερα αποτελέσματα, είναι ο Nearest. Είναι ενεργοποιημένος από προεπιλογή, επειδή με μέγεθος κατακερματισμού 16x16 η χαμηλότερη ποιότητα δεν είναι πραγματικά ορατή.
    
    Με μέγεθος κατακερματισμού 8x8 συνιστάται να χρησιμοποιήσετε διαφορετικό αλγόριθμο από τον Nearest, ώστε να έχετε καλύτερες ομάδες εικόνων.
image_hash_alg_tooltip =
    Οι χρήστες μπορούν να επιλέξουν έναν από τους πολλούς αλγορίθμους υπολογισμού του κατακερματισμού.
    
    Καθένας έχει δυνατά και αδύναμα σημεία και θα δίνει άλλοτε καλύτερα και άλλοτε χειρότερα αποτελέσματα για διαφορετικές εικόνες.
    
    Επομένως, για να προσδιορίσετε τον καλύτερο για εσάς, απαιτείται χειροκίνητη δοκιμή.
image_geometric_invariance_tooltip = Συγκρίνετε επίσης καθρεφτισμένες/αναστραμμένες και προαιρετικά περιστραμμένες παραλλαγές κάθε εικόνας. Αυτό βελτιώνει το ταίριασμα, αλλά αυξάνει τον χρόνο κατακερματισμού.
big_files_mode_combobox_tooltip = Επιτρέπει την αναζήτηση για τα μικρότερα/μεγαλύτερα αρχεία
big_files_mode_label = Ελεγχμένα αρχεία
big_files_mode_smallest_combo_box = Τα Μικρότερα
big_files_mode_biggest_combo_box = Τα Μεγαλύτερα
main_notebook_duplicates = Διπλότυπα Αρχεία
main_notebook_empty_directories = Άδειοι Κατάλογοι
main_notebook_big_files = Μεγάλα Αρχεία
main_notebook_empty_files = Κενά Αρχεία
main_notebook_temporary = Προσωρινά Αρχεία
main_notebook_similar_images = Παρόμοιες Εικόνες
main_notebook_similar_videos = Παρόμοια Βίντεο
main_notebook_same_music = Διπλότυπα Μουσικής
main_notebook_symlinks = Μη Έγκυροι Συμβολικοί Σύνδεσμοι
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
main_tree_view_column_symlink_file_name = Όνομα Αρχείου Συμβολικού Συνδέσμου
main_tree_view_column_symlink_folder = Φάκελος Συμβολικού Συνδέσμου
main_tree_view_column_destination_path = Διαδρομή Προορισμού
main_tree_view_column_type_of_error = Τύπος Σφάλματος
main_tree_view_column_current_extension = Τρέχουσα Επέκταση
main_tree_view_column_proper_extensions = Κατάλληλη Επέκταση
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Codec
main_label_check_method = Μέθοδος ελέγχου
main_label_hash_type = Τύπος κατακερματισμού
main_label_hash_size = Μέγεθος κατακερματισμού
main_label_geometric_invariance = Γεωμετρική αναλλοιότητα
main_label_size_bytes = Μέγεθος (bytes)
main_label_min_size = Ελάχιστο
main_label_max_size = Μέγιστο
main_label_shown_files = Αριθμός εμφανιζόμενων αρχείων
main_label_resize_algorithm = Αλγόριθμος αλλαγής μεγέθους
main_label_similarity = Ομοιότητα{ "   " }
main_check_box_broken_files_audio = Ήχος
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Αρχείο συμπίεσης
main_check_box_broken_files_image = Εικόνα
main_check_box_broken_files_video = Βίντεο
main_check_box_broken_files_video_tooltip = Χρησιμοποιεί το ffmpeg/ffprobe για την επικύρωση αρχείων βίντεο. Πολύ αργό και μπορεί να ανιχνεύσει αυστηρές ατέλειες ακόμη και αν το αρχείο παίζει κανονικά.
check_button_general_same_size = Αγνόηση ίδιου μεγέθους
check_button_general_same_size_tooltip = Αγνοήστε τα αρχεία με το ίδιο μέγεθος στα αποτελέσματα - συνήθως αυτά είναι 1:1 διπλότυπα
main_label_size_bytes_tooltip = Μέγεθος αρχείων που θα χρησιμοποιηθούν κατά τη σάρωση
# Upper window
upper_tree_view_included_folder_column_title = Φάκελοι προς αναζήτηση
upper_tree_view_included_reference_column_title = Φάκελοι αναφοράς
upper_recursive_button = Αναδρομικά
upper_recursive_button_tooltip = Αν επιλεχθεί, θα αναζητούνται επίσης αρχεία που δεν τοποθετούνται απευθείας σε επιλεγμένους φακέλους.
upper_manual_add_included_button = Χειροκίνητη Προσθήκη
upper_add_included_button = Προσθήκη
upper_remove_included_button = Αφαίρεση
upper_manual_add_excluded_button = Χειροκίνητη Προσθήκη
upper_add_excluded_button = Προσθήκη
upper_remove_excluded_button = Αφαίρεση
upper_manual_add_included_button_tooltip =
    Προσθήκη ονόματος καταλόγου στην αναζήτηση με το χέρι.
    
    Για να προσθέσετε πολλαπλές διαδρομές ταυτόχρονα, διαχωρίστε τις με το ;
    
    /home/roman;/home/rozkaz θα προσθέσει δύο καταλόγους /home/roman και /home/rozkaz
upper_add_included_button_tooltip = Προσθήκη νέου καταλόγου για αναζήτηση.
upper_remove_included_button_tooltip = Διαγραφή καταλόγου από την αναζήτηση.
upper_manual_add_excluded_button_tooltip =
    Προσθήκη εξαιρούμενου ονόματος καταλόγου με το χέρι.
    
    Για να προσθέσετε πολλαπλές διαδρομές ταυτόχρονα, διαχωρίστε τις με το ;
    
    /home/roman;/home/krokiet θα προσθέσει δύο καταλόγους /home/roman και /home/keokiet
upper_add_excluded_button_tooltip = Προσθήκη καταλόγου για να αποκλειστεί στην αναζήτηση.
upper_remove_excluded_button_tooltip = Διαγραφή καταλόγου από αποκλεισμένους.
upper_notebook_items_configuration = Διαμόρφωση Στοιχείων
upper_notebook_excluded_directories = Αποκλεισμένες Διαδρομές
upper_notebook_included_directories = Συμπεριλημμένες Διαδρομές
upper_allowed_extensions_tooltip =
    Οι επιτρεπόμενες επεκτάσεις πρέπει να διαχωρίζονται με κόμματα (εξ ορισμού είναι όλες διαθέσιμες).
    
    Τα ακόλουθα Macros, τα οποία προσθέτουν πολλαπλές επεκτάσεις ταυτόχρονα, είναι επίσης διαθέσιμα: IMAGE, VIDEO, MUSIC, TEXT.
    
    Παράδειγμα χρήσης: ".exe, IMAGE, VIDEO, .rar, 7z" - αυτό σημαίνει ότι οι εικόνες (π.χ. jpg, png), τα βίντεο (π.χ. avi, mp4), καθώς και τα αρχεία exe, rar και 7z θα σαρωθούν.
upper_excluded_extensions_tooltip =
    Λίστα απενεργοποιημένων αρχείων που θα αγνοηθούν κατά τη σάρωση.
    
    Όταν χρησιμοποιείτε και τις δύο επιτρεπόμενες και απενεργοποιημένες επεκτάσεις, αυτή έχει υψηλότερη προτεραιότητα, οπότε το αρχείο δεν θα ελεγχθεί.
upper_excluded_items_tooltip =
    Τα στοιχεία προς εξαίρεση πρέπει να περιέχουν το σύμβολο μπαλαντέρ * και να διαχωρίζονται με κόμματα.
    Αυτό είναι πιο αργό από τις Αποκλεισμένες Διαδρομές, οπότε χρησιμοποιήστε το προσεκτικά.
upper_excluded_items = Εξαιρούμενα Στοιχεία:
upper_allowed_extensions = Επιτρεπόμενες Επεκτάσεις:
upper_excluded_extensions = Απενεργοποιημένες Επεκτάσεις:
# Popovers
popover_select_all = Επιλογή όλων
popover_unselect_all = Αποεπιλογή όλων
popover_reverse = Αντίστροφη Επιλογή
popover_select_all_except_shortest_path = Επιλογή όλων εκτός από τη συντομότερη διαδρομή
popover_select_all_except_longest_path = Επιλογή όλων εκτός από τη μακρύτερη διαδρομή
popover_select_all_except_oldest = Επιλογή όλων εκτός από το παλαιότερο
popover_select_all_except_newest = Επιλογή όλων εκτός από το νεότερο
popover_select_one_oldest = Επιλογή ενός παλαιότερου
popover_select_one_newest = Επιλογή ενός νεότερου
popover_select_custom = Επιλογή προσαρμοσμένου
popover_unselect_custom = Αποεπιλογή προσαρμοσμένου
popover_select_all_images_except_biggest = Επιλογή όλων εκτός από τη μεγαλύτερη
popover_select_all_images_except_smallest = Επιλογή όλων εκτός από τη μικρότερη
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
    /usr/bin/ziemniak.txt μπορεί να βρεθεί με /ziem[a-z]+
    
    Αυτό χρησιμοποιεί την προεπιλεγμένη υλοποίηση Rust regex. Μπορείτε να διαβάσετε περισσότερα για αυτό εδώ: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Ενεργοποιεί τον εντοπισμό με διάκριση πεζών/κεφαλαίων.
    
    Όταν απενεργοποιηθεί, το /home/* βρίσκει και το /HoMe/roman και το /home/roman.
popover_custom_not_all_check_button_tooltip =
    Αποτρέπει την επιλογή όλων των εγγραφών σε μια ομάδα.
    
    Αυτό είναι ενεργοποιημένο από προεπιλογή, καθώς στις περισσότερες περιπτώσεις δεν θέλετε να διαγράψετε τόσο τα πρωτότυπα όσο και τα διπλότυπα αρχεία, αλλά θέλετε να αφήσετε τουλάχιστον ένα αρχείο.
    
    ΠΡΟΕΙΔΟΠΟΙΗΣΗ: Αυτή η ρύθμιση δεν λειτουργεί αν έχετε ήδη επιλέξει χειροκίνητα όλα τα αποτελέσματα σε μια ομάδα.
popover_custom_regex_path_label = Διαδρομή
popover_custom_regex_name_label = Όνομα
popover_custom_regex_regex_label = Regex Διαδρομή + Όνομα
popover_custom_case_sensitive_check_button = Διάκριση πεζών/κεφαλαίων
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
bottom_symlink_button = Συμβολικός Σύνδεσμος
bottom_hardlink_button = Σκληρός Σύνδεσμος
bottom_move_button = Μετακίνηση
bottom_sort_button = Ταξινόμηση
bottom_compare_button = Σύγκριση
bottom_search_button_tooltip = Έναρξη αναζήτησης
bottom_select_button_tooltip = Επιλέξτε εγγραφές. Μόνο επιλεγμένα αρχεία/φάκελοι μπορούν να υποβληθούν σε μεταγενέστερη επεξεργασία.
bottom_delete_button_tooltip = Διαγραφή επιλεγμένων αρχείων/φακέλων.
bottom_save_button_tooltip = Αποθήκευση δεδομένων σχετικά με την αναζήτηση σε αρχείο
bottom_symlink_button_tooltip =
    Δημιουργία συμβολικών συνδέσμων.
    Λειτουργεί μόνο όταν επιλεγούν τουλάχιστον δύο αποτελέσματα σε μια ομάδα.
    Το πρώτο παραμένει αμετάβλητο και το δεύτερο και τα επόμενα συνδέονται συμβολικά με το πρώτο.
bottom_hardlink_button_tooltip =
    Δημιουργία σκληρών συνδέσμων.
    Λειτουργεί μόνο όταν επιλεγούν τουλάχιστον δύο αποτελέσματα σε μια ομάδα.
    Το πρώτο παραμένει αμετάβλητο και το δεύτερο και τα επόμενα συνδέονται σκληρά με το πρώτο.
bottom_hardlink_button_not_available_tooltip =
    Δημιουργία σκληρών συνδέσμων.
    Το κουμπί είναι απενεργοποιημένο, επειδή δεν είναι δυνατή η δημιουργία σκληρών συνδέσμων.
    Οι σκληροί σύνδεσμοι λειτουργούν μόνο με δικαιώματα διαχειριστή στα Windows, οπότε φροντίστε να εκτελέσετε την εφαρμογή ως διαχειριστής.
    Εάν η εφαρμογή λειτουργεί ήδη με τέτοια δικαιώματα, ελέγξτε για παρόμοια ζητήματα στο Github.
bottom_move_button_tooltip =
    Μετακινεί τα αρχεία στον επιλεγμένο κατάλογο.
    Αντιγράφει όλα τα αρχεία στον κατάλογο χωρίς να διατηρεί τη δομή του δέντρου καταλόγων.
    Όταν προσπαθείτε να μετακινήσετε δύο αρχεία με το ίδιο όνομα στον φάκελο, το δεύτερο θα αποτύχει και θα εμφανιστεί σφάλμα.
bottom_sort_button_tooltip = Ταξινόμηση αρχείων/φακέλων σύμφωνα με την επιλεγμένη μέθοδο.
bottom_compare_button_tooltip = Σύγκριση εικόνων στην ομάδα.
bottom_show_errors_tooltip = Εμφάνιση/Απόκρυψη του κάτω πίνακα κειμένου.
bottom_show_upper_notebook_tooltip = Εμφάνιση/Απόκρυψη άνω πίνακα καρτελών.
# Progress Window
progress_stop_button = Διακοπή
progress_stop_additional_message = Η διακοπή ζητήθηκε
# About Window
about_repository_button_tooltip = Σύνδεσμος προς σελίδα αποθετηρίου με πηγαίο κώδικα.
about_donation_button_tooltip = Σύνδεσμος προς τη σελίδα δωρεών.
about_instruction_button_tooltip = Σύνδεσμος στη σελίδα οδηγιών.
about_translation_button_tooltip = Σύνδεσμος προς τη σελίδα του Crowdin με μεταφράσεις εφαρμογών. Υποστηρίζονται επίσημα τα πολωνικά και τα αγγλικά.
about_repository_button = Αποθετήριο
about_donation_button = Δωρεά
about_instruction_button = Οδηγίες
about_translation_button = Μετάφραση
about_other_apps_button = Άλλες Εφαρμογές
about_other_apps_dialog_title = Άλλες εφαρμογές από τον qarmin
about_other_apps_open_source_note = Όλες οι εφαρμογές που παρατίθενται είναι δωρεάν και ανοιχτού κώδικα.
about_other_apps_open_button = Άνοιγμα
about_other_apps_szyszka_desc = Γρήγορο και ισχυρό εργαλείο μετονομασίας αρχείων.
about_other_apps_mykrut_desc = Απλός, γρήγορος διαχειριστής αρχείων Linux με άποψη.
about_other_apps_dcmki_viewer_desc = Απλό πρόγραμμα προβολής DICOM.
about_other_apps_video_thumbnailer_desc = Περιτύλιγμα γύρω από τη γεννήτρια μικρογραφιών βίντεο που χρησιμοποιείται στην Czkawka.
about_other_apps_space_finder_desc = Απλό εργαλείο εντοπισμού των μεγαλύτερων αρχείων στο σύστημά σας.
about_other_apps_system_info_collector_desc = Συλλέγει τη χρήση RAM/CPU από το λειτουργικό σύστημα και την εμφανίζει ως γραφήματα.
# Header
header_setting_button_tooltip = Άνοιγμα διαλόγου ρυθμίσεων.
header_about_button_tooltip = Άνοιγμα διαλόγου με πληροφορίες σχετικά με την εφαρμογή.
header_krokiet_button_tooltip = Δοκιμάστε το Krokiet - η νέα και βελτιωμένη έκδοση!
# Krokiet promo dialog
krokiet_promo_title = Γνωρίστε το Krokiet!
krokiet_promo_message =
    Γεια σας, γενναίος χρήστης Czkawka!
    
    Η Δύναμη είναι σαφώς μαζί σας, αλλά το Krokiet δεν είναι - ένα νεότερο, γρηγορότερο, ελαφρύτερο, και σημαντικά πιο όμορφο (υποθέτοντας ότι οι εφαρμογές μπορούν πραγματικά να είναι όμορφες) εργαλείο καθαρισμού διπλότυπων.
    
    Το Krokiet περιλαμβάνει όλα όσα άρεσαν στον κόσμο σχετικά με το Czkawka. Είναι εντελώς δωρεάν, ανοικτού κώδικα, έχει ένα μοναδικό και απλό UI (που έχει επαινεθεί αλλά και επικριθεί από πολλούς), εισάγει πολλά νέα χαρακτηριστικά, χρησιμοποιεί λιγότερες εξαρτήσεις και λειτουργεί πολύ πιο αξιόπιστα σε διαφορετικές πλατφόρμες.
    
    Και αν σας διέφυγε, υπάρχει ήδη μια ακόμα νεότερη εφαρμογή από το Krokiet - Cedinia, σχεδιασμένη κυρίως για συσκευές Android και χρήση οθόνης αφής.
    
    Czkawka GTK μας υπηρέτησε καλά, αλλά η σκοπιά του τελείωσε.
krokiet_promo_link_download = Λήψη Krokiet/Cedinia
krokiet_promo_link_project = Σελίδα έργου

# Settings


## General

settings_number_of_threads = Αριθμός χρησιμοποιημένων νημάτων
settings_number_of_threads_tooltip = Αριθμός χρησιμοποιημένων νημάτων, 0 σημαίνει ότι θα χρησιμοποιηθούν όλα τα διαθέσιμα νήματα.
settings_use_rust_preview = Χρήση εξωτερικών βιβλιοθηκών αντί του gtk για φόρτωση προεπισκοπήσεων
settings_use_rust_preview_tooltip =
    Η χρήση προεπισκοπήσεων gtk θα είναι μερικές φορές πιο γρήγορη και θα υποστηρίζει περισσότερες μορφές, αλλά μερικές φορές αυτό θα μπορούσε να είναι ακριβώς το αντίθετο.
    
    Αν έχετε προβλήματα με τη φόρτωση προεπισκόπησης, μπορείτε να προσπαθήσετε να αλλάξετε αυτή τη ρύθμιση.
    
    Σε συστήματα εκτός Linux, συνιστάται να χρησιμοποιήσετε αυτήν την επιλογή, επειδή το gtk-pixbuf δεν είναι πάντα διαθέσιμο εκεί, οπότε η απενεργοποίηση αυτής της επιλογής δεν θα φορτώσει προεπισκοπήσεις ορισμένων εικόνων.
settings_label_restart = Πρέπει να επανεκκινήσετε την εφαρμογή για να εφαρμόσετε τις ρυθμίσεις!
settings_ignore_other_filesystems = Αγνόηση άλλων συστημάτων αρχείων (μόνο Linux)
settings_ignore_other_filesystems_tooltip =
    αγνοεί αρχεία που δεν βρίσκονται στο ίδιο σύστημα αρχείων με τους καταλόγους αναζήτησης.
    
    Λειτουργεί όπως η επιλογή -xdev στην εντολή find στο Linux
settings_save_at_exit_button_tooltip = Αποθήκευση ρυθμίσεων σε αρχείο κατά το κλείσιμο της εφαρμογής.
settings_load_at_start_button_tooltip =
    Φόρτωση ρυθμίσεων από το αρχείο κατά το άνοιγμα της εφαρμογής.
    
    Αν δεν είναι ενεργοποιημένη, θα χρησιμοποιηθούν οι προεπιλεγμένες ρυθμίσεις.
settings_confirm_deletion_button_tooltip = Εμφάνιση διαλόγου επιβεβαίωσης όταν κάνετε κλικ στο κουμπί διαγραφής.
settings_confirm_link_button_tooltip = Εμφάνιση διαλόγου επιβεβαίωσης όταν κάνετε κλικ στο κουμπί σκληρού/συμβολικού συνδέσμου.
settings_confirm_group_deletion_button_tooltip = Εμφάνιση διαλόγου προειδοποίησης όταν προσπαθείτε να διαγράψετε όλες τις εγγραφές από την ομάδα.
settings_show_text_view_button_tooltip = Εμφάνιση πίνακα κειμένου στο κάτω μέρος της διεπαφής χρήστη.
settings_use_cache_button_tooltip = Χρήση προσωρινής μνήμης αρχείων.
settings_save_also_as_json_button_tooltip = Αποθήκευση προσωρινής μνήμης σε μορφή JSON (αναγνώσιμη από άνθρωπο). Είναι δυνατή η τροποποίηση του περιεχομένου της. Η προσωρινή μνήμη από αυτό το αρχείο θα διαβάζεται αυτόματα από την εφαρμογή αν λείπει η προσωρινή μνήμη δυαδικής μορφής (με επέκταση bin).
settings_use_trash_button_tooltip = Μετακινεί τα αρχεία στον κάδο απορριμμάτων αντί να τα διαγράφει μόνιμα.
settings_language_label_tooltip = Γλώσσα διεπαφής χρήστη.
settings_save_at_exit_button = Αποθήκευση ρυθμίσεων κατά το κλείσιμο της εφαρμογής
settings_load_at_start_button = Φόρτωση ρυθμίσεων κατά το άνοιγμα της εφαρμογής
settings_confirm_deletion_button = Εμφάνιση διαλόγου επιβεβαίωσης κατά τη διαγραφή αρχείων
settings_confirm_link_button = Εμφάνιση διαλόγου επιβεβαίωσης κατά τη δημιουργία σκληρών/συμβολικών συνδέσμων για αρχεία
settings_confirm_group_deletion_button = Εμφάνιση διαλόγου επιβεβαίωσης κατά τη διαγραφή όλων των αρχείων της ομάδας
settings_show_text_view_button = Εμφάνιση κάτω πίνακα κειμένου
settings_use_cache_button = Χρήση προσωρινής μνήμης
settings_save_also_as_json_button = Επίσης αποθήκευση προσωρινής μνήμης ως αρχείο JSON
settings_use_trash_button = Μετακίνηση διαγραμμένων αρχείων στον κάδο απορριμμάτων
settings_language_label = Γλώσσα
settings_multiple_delete_outdated_cache_checkbutton = Αυτόματη διαγραφή ξεπερασμένων καταχωρήσεων προσωρινής μνήμης
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Αφαιρεί τις παλιές καταχωρήσεις προσωρινής μνήμης που δείχνουν σε αρχεία που δεν υπάρχουν πλέον.
    
    Όταν είναι ενεργοποιημένο, η εφαρμογή διασφαλίζει ότι, κατά τη φόρτωση των εγγραφών, όλες οι εγγραφές δείχνουν σε έγκυρα αρχεία (τα κατεστραμμένα αγνοούνται).
    
    Η απενεργοποίηση αυτού θα βοηθήσει κατά τη σάρωση αρχείων σε εξωτερικούς δίσκους, ώστε οι καταχωρήσεις προσωρινής μνήμης γι' αυτά να μην διαγράφονται στην επόμενη σάρωση.
    
    Στην περίπτωση που υπάρχουν εκατοντάδες χιλιάδες εγγραφές στην προσωρινή μνήμη, προτείνεται η ενεργοποίηση αυτής της επιλογής, η οποία θα επιταχύνει τη φόρτωση/αποθήκευση της προσωρινής μνήμης στην αρχή/τέλος της σάρωσης.
settings_notebook_general = Γενικά
settings_notebook_duplicates = Διπλότυπα
settings_notebook_images = Παρόμοιες Εικόνες
settings_notebook_videos = Παρόμοια Βίντεο

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Εμφανίζει την προεπισκόπηση στη δεξιά πλευρά (όταν επιλέγετε ένα αρχείο εικόνας).
settings_multiple_image_preview_checkbutton = Εμφάνιση προεπισκόπησης εικόνας
settings_multiple_clear_cache_button_tooltip =
    Χειροκίνητη εκκαθάριση της προσωρινής μνήμης από ξεπερασμένες καταχωρήσεις.
    Αυτό θα πρέπει να χρησιμοποιείται μόνο αν η αυτόματη εκκαθάριση έχει απενεργοποιηθεί.
settings_multiple_clear_cache_button = Κατάργηση παρωχημένων αποτελεσμάτων από την προσωρινή μνήμη.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Αποκρύπτει όλα τα αρχεία εκτός από ένα, αν όλα δείχνουν στα ίδια δεδομένα (είναι συνδεδεμένα με σκληρό σύνδεσμο).
    
    Παράδειγμα: Στην περίπτωση όπου υπάρχουν (στον δίσκο) επτά αρχεία που είναι συνδεδεμένα με σκληρό σύνδεσμο σε συγκεκριμένα δεδομένα και ένα διαφορετικό αρχείο με τα ίδια δεδομένα αλλά διαφορετικό inode, τότε στον εντοπιστή διπλότυπων θα εμφανιστούν μόνο ένα μοναδικό αρχείο και ένα αρχείο από αυτά με σκληρό σύνδεσμο.
settings_duplicates_minimal_size_entry_tooltip =
    Ορίστε το ελάχιστο μέγεθος αρχείου που θα αποθηκεύεται στην προσωρινή μνήμη.
    
    Επιλέγοντας μια μικρότερη τιμή θα δημιουργηθούν περισσότερες εγγραφές. Αυτό θα επιταχύνει την αναζήτηση, αλλά θα επιβραδύνει τη φόρτωση/αποθήκευση της προσωρινής μνήμης.
settings_duplicates_prehash_checkbutton_tooltip =
    Ενεργοποιεί την προσωρινή αποθήκευση του προκατακερματισμού (ενός κατακερματισμού υπολογισμένου από ένα μικρό μέρος του αρχείου) το οποίο επιτρέπει την προηγούμενη απόρριψη μη διπλών αποτελεσμάτων.
    
    Είναι απενεργοποιημένο από προεπιλογή επειδή μπορεί να προκαλέσει επιβραδύνσεις σε ορισμένες περιπτώσεις.
    
    Συνιστάται ιδιαίτερα να το χρησιμοποιήσετε κατά τη σάρωση εκατοντάδων χιλιάδων ή εκατομμυρίων αρχείων, επειδή μπορεί να επιταχύνει την αναζήτηση κατά πολλές φορές.
settings_duplicates_prehash_minimal_entry_tooltip = Ελάχιστο μέγεθος της προσωρινά αποθηκευμένης καταχώρησης.
settings_duplicates_hide_hard_link_button = Απόκρυψη σκληρών συνδέσμων
settings_duplicates_prehash_checkbutton = Χρήση προσωρινής μνήμης προκατακερματισμού
settings_duplicates_minimal_size_cache_label = Ελάχιστο μέγεθος των αρχείων (σε byte) που αποθηκεύεται στην προσωρινή μνήμη
settings_duplicates_minimal_size_cache_prehash_label = Ελάχιστο μέγεθος των αρχείων (σε byte) που αποθηκεύεται στην προσωρινή μνήμη προκατακερματισμού

## Saving/Loading settings

settings_saving_button_tooltip = Αποθήκευση των τρεχουσών ρυθμίσεων στο αρχείο.
settings_loading_button_tooltip = Φόρτωση ρυθμίσεων από το αρχείο και αντικατάσταση των τρεχουσών ρυθμίσεων με αυτές.
settings_reset_button_tooltip = Επαναφορά των τρεχουσών ρυθμίσεων στην προκαθορισμένη.
settings_saving_button = Αποθήκευση διαμόρφωσης
settings_loading_button = Φόρτωση διαμόρφωσης
settings_reset_button = Επαναφορά διαμόρφωσης

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Ανοίγει τον φάκελο όπου αποθηκεύονται τα αρχεία txt της προσωρινής μνήμης.
    
    Η τροποποίηση των αρχείων προσωρινής μνήμης μπορεί να οδηγήσει σε εσφαλμένα αποτελέσματα. Ωστόσο, η τροποποίηση της διαδρομής μπορεί να εξοικονομήσει χρόνο όταν μετακινείτε μεγάλο αριθμό αρχείων σε διαφορετική τοποθεσία.
    
    Μπορείτε να αντιγράψετε αυτά τα αρχεία μεταξύ υπολογιστών για να εξοικονομήσετε χρόνο στην εκ νέου σάρωση αρχείων (φυσικά, αν έχουν παρόμοια δομή καταλόγου).
    
    Σε περίπτωση προβλημάτων με την προσωρινή μνήμη, αυτά τα αρχεία μπορούν να αφαιρεθούν. Η εφαρμογή θα τα αναδημιουργήσει αυτόματα.
settings_folder_settings_open_tooltip =
    Ανοίγει το φάκελο όπου αποθηκεύονται οι ρυθμίσεις του Czkawka.
    
    ΠΡΟΕΙΔΟΠΟΙΗΣΗ: Η χειροκίνητη τροποποίηση των ρυθμίσεων μπορεί να σπάσει τη ροή εργασίας σας.
settings_folder_cache_open = Άνοιγμα φακέλου προσωρινής μνήμης
settings_folder_settings_open = Άνοιγμα φακέλου ρυθμίσεων
# Compute results
compute_stopped_by_user = Η αναζήτηση διακόπηκε από τον χρήστη
compute_found_duplicates_hash_size = Βρέθηκαν { $number_files } διπλότυπα σε { $number_groups } ομάδες που πήραν { $size } σε { $time }
compute_found_duplicates_name = Βρέθηκαν { $number_files } διπλότυπα σε { $number_groups } ομάδες σε { $time }
compute_found_empty_folders = Βρέθηκαν { $number_files } άδειοι φάκελοι σε { $time }
compute_found_empty_files = Βρέθηκαν { $number_files } κενά αρχεία σε { $time }
compute_found_big_files = Βρέθηκαν { $number_files } μεγάλα αρχεία σε { $time }
compute_found_temporary_files = Βρέθηκαν { $number_files } προσωρινά αρχεία σε { $time }
compute_found_images = Βρέθηκαν { $number_files } παρόμοιες εικόνες σε { $number_groups } ομάδες σε { $time }
compute_found_videos = Βρέθηκαν { $number_files } παρόμοια βίντεο σε { $number_groups } ομάδες σε { $time }
compute_found_music = Βρέθηκαν { $number_files } παρόμοια αρχεία μουσικής σε { $number_groups } ομάδες σε { $time }
compute_found_invalid_symlinks = Βρέθηκαν { $number_files } μη έγκυροι συμβολικοί σύνδεσμοι σε { $time }
compute_found_broken_files = Βρέθηκαν { $number_files } κατεστραμμένα αρχεία σε { $time }
compute_found_bad_extensions = Βρέθηκαν { $number_files } αρχεία με μη έγκυρες επεκτάσεις σε { $time }
# Progress window
progress_current_stage = Τρέχον στάδιο:{ "  " }
progress_all_stages = Όλα τα στάδια:{ "  " }
# Saving loading 
saving_loading_saving_success = Η διαμόρφωση αποθηκεύτηκε στο αρχείο { $name }.
saving_loading_saving_failure = Αποτυχία αποθήκευσης δεδομένων ρύθμισης παραμέτρων στο αρχείο { $name }, λόγος { $reason }.
saving_loading_reset_configuration = Η τρέχουσα διαμόρφωση εκκαθαρίστηκε.
saving_loading_loading_success = Η διαμόρφωση της εφαρμογής φορτώθηκε επιτυχώς.
saving_loading_no_config_file = Δεν βρέθηκε αρχείο ρύθμισης παραμέτρων, χρησιμοποιώντας τις προεπιλεγμένες ρυθμίσεις.
saving_loading_failed_to_create_config_file = Αποτυχία δημιουργίας αρχείου ρυθμίσεων "{ $path }", λόγος "{ $reason }".
saving_loading_failed_to_read_config_file = Αδυναμία φόρτωσης ρυθμίσεων από το "{ $path }" επειδή δεν υπάρχει ή δεν είναι αρχείο.
saving_loading_failed_to_read_data_from_file = Αδυναμία ανάγνωσης δεδομένων από το αρχείο "{ $path }", λόγος "{ $reason }".
# Other
selected_all_reference_folders = Αδυναμία έναρξης αναζήτησης, όταν όλοι οι κατάλογοι ορίζονται ως φάκελοι αναφοράς
searching_for_data = Αναζήτηση δεδομένων, μπορεί να πάρει λίγο, παρακαλώ περιμένετε...
text_view_messages = ΜΗΝΥΜΑΤΑ
text_view_warnings = ΠΡΟΕΙΔΟΠΟΙΗΣΕΙΣ
text_view_errors = ΣΦΑΛΜΑΤΑ
about_window_motto = Αυτό το πρόγραμμα είναι και θα είναι πάντα ελεύθερο προς χρήση.
krokiet_new_app = Αυτή η έκδοση GTK του Czkawka δεν αναπτύσσεται πλέον από την έκδοση 12 και έπειτα. Για νέα χαρακτηριστικά και ενεργή ανάπτυξη, παρακαλώ χρησιμοποιήστε το Krokiet, το οποίο είναι πιο σταθερό και αποδοτικό.
# Various dialog
dialogs_ask_next_time = Ερώτηση την επόμενη φορά
symlink_failed = Αποτυχία δημιουργίας συμβολικού συνδέσμου { $name } προς { $target }, λόγος { $reason }
delete_title_dialog = Επιβεβαίωση διαγραφής
delete_question_label = Είστε βέβαιοι ότι θέλετε να διαγράψετε αρχεία;
delete_all_files_in_group_title = Επιβεβαίωση διαγραφής όλων των αρχείων της ομάδας
delete_all_files_in_group_label1 = Σε ορισμένες ομάδες έχουν επιλεγεί όλες οι εγγραφές.
delete_all_files_in_group_label2 = Είστε βέβαιοι ότι θέλετε να τα διαγράψετε;
delete_items_label = { $items } αρχεία θα διαγραφούν.
delete_items_groups_label = { $items } αρχεία από { $groups } ομάδες θα διαγραφούν.
hardlink_failed = Αποτυχία δημιουργίας σκληρού συνδέσμου { $name } προς { $target }, λόγος { $reason }
hard_sym_invalid_selection_title_dialog = Μη έγκυρη επιλογή με κάποιες ομάδες
hard_sym_invalid_selection_label_1 = Σε ορισμένες ομάδες έχει επιλεγεί μόνο μία εγγραφή και θα αγνοηθεί.
hard_sym_invalid_selection_label_2 = Για να είναι δυνατή η σκληρή/συμβολική σύνδεση αυτών των αρχείων, πρέπει να επιλεγούν τουλάχιστον δύο αποτελέσματα στην ομάδα.
hard_sym_invalid_selection_label_3 = Η πρώτη στην ομάδα αναγνωρίζεται ως πρωτότυπο και δεν αλλάζεται, αλλά η δεύτερη και οι επόμενες τροποποιούνται.
hard_sym_link_title_dialog = Επιβεβαίωση συνδέσμου
hard_sym_link_label = Είστε βέβαιοι ότι θέλετε να συνδέσετε αυτά τα αρχεία;
move_folder_failed = Αποτυχία μετακίνησης του φακέλου { $name }, λόγος { $reason }
move_file_failed = Αποτυχία μετακίνησης αρχείου { $name }, λόγος { $reason }
move_files_title_dialog = Επιλέξτε φάκελο στον οποίο θέλετε να μετακινήσετε διπλότυπα αρχεία
move_files_choose_more_than_1_path = Μόνο μία διαδρομή μπορεί να επιλεγεί για να είναι δυνατή η αντιγραφή των διπλότυπων αρχείων τους, επιλεγμένες { $path_number }.
move_stats = Σωστά μετακινήθηκαν { $num_files }/{ $all_files } στοιχεία
save_results_to_file = Αποθηκεύτηκαν τα αποτελέσματα τόσο σε αρχεία txt όσο και json στον φάκελο "{ $name }".
search_not_choosing_any_music = ΣΦΑΛΜΑ: Πρέπει να επιλέξετε τουλάχιστον ένα πλαίσιο ελέγχου με τύπους αναζήτησης μουσικής.
search_not_choosing_any_broken_files = ΣΦΑΛΜΑ: Πρέπει να επιλέξετε τουλάχιστον ένα πλαίσιο ελέγχου με τον τύπο των επιλεγμένων κατεστραμμένων αρχείων.
include_folders_dialog_title = Φάκελοι προς συμπερίληψη
exclude_folders_dialog_title = Φάκελοι προς εξαίρεση
include_manually_directories_dialog_title = Χειροκίνητη προσθήκη καταλόγου
cache_properly_cleared = Σωστή εκκαθάριση προσωρινής μνήμης
cache_clear_duplicates_title = Εκκαθάριση προσωρινής μνήμης διπλότυπων
cache_clear_similar_images_title = Εκκαθάριση προσωρινής μνήμης παρόμοιων εικόνων
cache_clear_similar_videos_title = Εκκαθάριση προσωρινής μνήμης παρόμοιων βίντεο
cache_clear_message_label_1 = Θέλετε να καθαρίσετε την προσωρινή μνήμη των ξεπερασμένων καταχωρήσεων;
cache_clear_message_label_2 = Αυτή η λειτουργία θα καταργήσει όλες τις καταχωρήσεις προσωρινής μνήμης που δείχνουν σε μη έγκυρα αρχεία.
cache_clear_message_label_3 = Αυτό μπορεί να επιταχύνει ελαφρώς τη φόρτωση/αποθήκευση στην προσωρινή μνήμη.
cache_clear_message_label_4 = ΠΡΟΕΙΔΟΠΟΙΗΣΗ: Η λειτουργία θα αφαιρέσει όλα τα προσωρινά αποθηκευμένα δεδομένα από τις αποσυνδεδεμένες εξωτερικές μονάδες. Έτσι, κάθε κατακερματισμός θα πρέπει να αναγεννηθεί.
# Show preview
preview_image_resize_failure = Αποτυχία αλλαγής μεγέθους εικόνας { $name }.
preview_image_opening_failure = Αποτυχία ανοίγματος εικόνας { $name }, λόγος { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Ομάδα { $current_group }/{ $all_groups } ({ $images_in_group } εικόνες)
compare_move_left_button = L
compare_move_right_button = R
