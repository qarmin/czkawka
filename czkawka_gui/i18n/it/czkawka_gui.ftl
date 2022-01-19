# Window titles
window_settings_title = Impostazioni
window_main_title = Singhiozzo
window_progress_title = Ricerca
window_compare_images = Confronta Le Immagini
# General
general_ok_button = Ok
general_close_button = Chiudi
# Main window
music_title_checkbox = Titolo
music_artist_checkbox = Artista
music_album_title_checkbox = Titolo album
music_album_artist_checkbox = Artista album
music_year_checkbox = Anno
music_comparison_checkbox = Confronto approssimativo
music_comparison_checkbox_tooltip =
    Cerca file musicali simili attraverso IA, che usa l'apprendimento automatico per rimuovere le parentesi da una frase. Per esempio, con quest'opzione attiva, questi file saranno considerati duplicati:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Nome
duplicate_mode_size_combo_box = Dimensione
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Singhiozzo offre 3 tipi di hash, che possono essere usati:
    
    Blake3 - funzione hash crittografica. È usato come algoritmo hash predefinito, poiché velocissimo.
    
    CRC32 - funzione hash semplice. Dovrebbe essere più veloce di Blake3, ma potrebbe avere raramente qualche collisione.
    
    XXH3 - molto simile per prestazioni e qualità a Blake3, quindi questi metodi possono essere usati con semplicità.
duplicate_check_method_tooltip =
    Al momento, Singhiozzo offre tre metodi di ricerca dei duplicati:
    
    Nome - Trova i file con lo stesso nome.
    
    Dimensione - Trova i file con la stessa dimensione.
    
    Hash - Trova i file con lo stesso contenuto. Questo metodo fa l'hashing dei file e successivamente li confronta per trovare i duplicati. Questo metodo è il più veloce per cercare i duplicati. Utilizza molto la cache, quindi successivamente la ricerca dati simili dovrebbe essere molto più veloce.
image_hash_size_tooltip =
    Singhiozzo offre la modifica degli hash generati per ogni immagine. Hash più grandi permettono di trovare immagini con un numero minore di differenze, ma rendono la ricerca più lunga.
    
    Il valore predefinito per gli hash è di 8 byte, che permette di trovare immagini molto simili e differenti. Gli hash di 16 and 32 byte dovrebbero essere utilizzate per immagini pressoché identiche. Gli hash di 64 bytes non dovrebbero essere usati, tranne in caso sia necessario cercare differenze molto piccole.
image_resize_filter_tooltip = Per processare l'hash di un'immagine, la libreria deve prima ridimensionarla. In funzione del metodo scelto, l'immagine risultante apparirà leggermente diversa. Il metodo più veloce, ma anche quello che dà i peggiori risultati, è Nearest.
image_hash_alg_tooltip = Gli utenti possono scegliere tra molti metodi di calcolo degli hash. Ognuno ha punti forti e punti deboli, e daranno risultati a volte migliori e a volte peggiori per immagini differenti. Quindi, per scegliere il migliore, sono necessarie prove manuali.
main_notebook_image_fast_compare = Confronto veloce
main_notebook_image_fast_compare_tooltip =
    Accelerare la ricerca e confrontare hash.
    
    In contrasto con la modalità normale in cui ogni hash viene confrontato tra loro x volte, dove x è la somiglianza scelta dall'utente, in questa modalità viene sempre utilizzata una sola comparazione.
    
    Questa opzione è consigliata quando si confrontano >10000 immagini con la similitudine non 0(Molto Alta).
main_notebook_duplicates = File duplicati
main_notebook_empty_directories = Cartelle vuote
main_notebook_big_files = Grandi file
main_notebook_empty_files = File vuoti
main_notebook_temporary = File temporanei
main_notebook_similar_images = Immagini simili
main_notebook_similar_videos = Video simili
main_notebook_same_music = Duplicati musicali
main_notebook_symlinks = Collegamenti invalidi
main_notebook_broken_files = File corrotti
main_tree_view_column_file_name = Nome file
main_tree_view_column_folder_name = Nome cartella
main_tree_view_column_path = Percorso
main_tree_view_column_modification = Data di modifica
main_tree_view_column_size = Dimensione
main_tree_view_column_similarity = Similitudine
main_tree_view_column_dimensions = Dimensioni
main_tree_view_column_title = Titolo
main_tree_view_column_artist = Artista
main_tree_view_column_year = Anno
main_tree_view_column_album_title = Titolo album
main_tree_view_column_album_artist = Artista album
main_tree_view_column_symlink_file_name = Nome collegamento
main_tree_view_column_symlink_folder = Cartella collegamento
main_tree_view_column_destination_path = Percorso di destinazione
main_tree_view_column_type_of_error = Tipo di Errore
main_label_check_method = Metodo di verifica
main_label_hash_type = Tipo hash
main_label_hash_size = Dimensione hash
main_label_size_bytes = Dimensione (byte)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Numero di file visualizzati
main_label_resize_algorithm = Metodo di ridimensionamento
main_label_similarity = Similitudine{ "   " }
check_button_general_same_size = Ignora stesse dimensioni
check_button_general_same_size_tooltip = Nei risultati, ignora i file con le stesse dimensioni - solitamente questo sono duplicati 1:1
main_label_size_bytes_tooltip = Dimensione dei file utilizzati nella ricerca
# Upper window
upper_tree_view_included_folder_column_title = Cartelle di ricerca
upper_tree_view_included_reference_column_title = Cartelle di riferimento
upper_recursive_button = Ricorsivo
upper_recursive_button_tooltip = Se selezionata, cerca anche tra i file non posizionati direttamente nelle catelle selezionate
upper_manual_add_included_button = Aggiungi manualmente
upper_add_included_button = Aggiungi
upper_remove_included_button = Rimuovi
upper_manual_add_excluded_button = Aggiungi manualmente
upper_add_excluded_button = Aggiungi
upper_remove_excluded_button = Rimuovi
upper_manual_add_included_button_tooltip = Permette di aggiungere cartelle per la ricerca manuale
upper_add_included_button_tooltip = Aggiungi nuova cartella per la ricerca
upper_remove_included_button_tooltip = Cancella cartella dalla ricerca
upper_manual_add_excluded_button_tooltip = Permette di aggiungere una cartella da escudere dalla ricerca
upper_add_excluded_button_tooltip = Aggiunge una cartella da escludere dalla ricerca
upper_remove_excluded_button_tooltip = Rimuove una cartella da quelle escluse
upper_notebook_items_configuration = Configurazione degli oggetti
upper_notebook_excluded_directories = Cartelle escluse
upper_notebook_included_directories = Cartelle incluse
upper_allowed_extensions_tooltip =
    Le estensioni permesse devono essere separate da virgola (per impostazione predefinita sono tutte disponibili)
    
    Macro IMAGE, VIDEO, MUSIC, TEXT è anche disponibile, che aggiunge estensioni multiple simultaneamente.
    
    Esempio di utilizzo  ".exe, IMAGE, VIDEO, .rar, 7z" - ciò significa che le immagini (es. jpg, png), i video (es. avi, mp4), i file exe, rar e 7z saranno cercati.
upper_excluded_items_tooltip =
    Le voci escluse devono contenere il carattere jolly * ed essere separate da virgola.
    Questo è più lento che le Cartelle escluse, quindi utilizzalo con cautela.
upper_excluded_items = Voci escluse:
upper_allowed_extensions = Estensioni permesse:
# Popovers
popover_select_all = Seleziona tutto
popover_unselect_all = Deseleziona tutto
popover_reverse = Inverti la selezione
popover_select_all_except_oldest = Seleziona tutti eccetto il più vecchio
popover_select_all_except_newest = Seleziona tutti eccetto il più recente
popover_select_one_oldest = Seleziona il più vecchio
popover_select_one_newest = Seleziona il più recente
popover_select_custom = Selezione personalizzata
popover_unselect_custom = Delezione personalizzata
popover_select_all_images_except_biggest = Seleziona tutti eccetto il più grande
popover_select_all_images_except_smallest = Seleziona tutti eccetto il più piccolo
popover_custom_path_check_button_entry_tooltip =
    Permette la selezione mediante percorso.
    
    Esempio di utilizzo:
    /home/pimpek/rzecz.txt può essere trovato con /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Permette di selezionare le voci attraverso il nome dei file.
    
    Esempio di utilizzo:
    /usr/ping/pong.txt può essere trovato con *ong*
popover_custom_regex_check_button_entry_tooltip =
    Permette di selezionare le voci attraverso il Regex specificato.
    
    Con questo metodo, il testo ricercato è il Percoso con Nome
    
    Esempio di utilizzo:
    /usr/bin/ziemniak.txt può essere trovato con /ziem[a-z]+
    
    Queto utilizza l'implementazione regex Rust predefinita, quindi puoi trovare dettagli su https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Impedisce di selezionare tutte le voci in un gruppo.
    
    Questa è selezionata per impostazione predefinita, poiché nella maggior parte delle situazioni non si vogliono cancellare sia il file originale che i duplicati, ma si vuole mantenere almeno u na copia.
    
    Attenzione: Questa impostazione non funziona se sono già stati selezionati manualmente tutti i risultati.
popover_custom_regex_path_label = Percorso
popover_custom_regex_name_label = Nome
popover_custom_regex_regex_label = Regex Percorso + Nome
popover_custom_all_in_group_label = Non selezionare tutte le voci in un gruppo
popover_custom_mode_unselect = Deselezione personalizzata
popover_custom_mode_select = Selezione personalizzata
popover_invalid_regex = Il Regex non è valido
popover_valid_regex = Il Regex è valido
# Bottom buttons
bottom_search_button = Cerca
bottom_select_button = Seleziona
bottom_delete_button = Cancella
bottom_save_button = Salva
bottom_symlink_button = Collegamenti simbolici
bottom_hardlink_button = Collegamenti fisici
bottom_move_button = Sposta
bottom_search_button_tooltip = Inizia a cercare i file/cartelle
bottom_select_button_tooltip = Seleziona le voci. Solo i file/cartelle selezionati possono essere processati successivamente.
bottom_delete_button_tooltip = Cancella i file/cartelle selezionati
bottom_save_button_tooltip = Salva i risultati della ricerca in un file
bottom_symlink_button_tooltip =
    Crea collegamenti simbolici.
    Funziona solo se almeno due risulati sono selezionati in un gruppo.
    Il primo non viene modificato; il secondo ed i successivi verranno collegati al primo.
bottom_hardlink_button_tooltip =
    Crea collegamenti fisici.
    Funziona solo se almeno due risulati sono selezionati in un gruppo.
    Il primo non viene modificato; il secondo ed i successivi verranno collegati al primo.
bottom_move_button_tooltip =
    Sposta i file nella cartella selezionata.
    Sposta tutti i file nella cartella senza mantenere la struttura della cartella.
    Se si prova a spostare due file con nome identico, il secondo fallirà e verrà mostrato un errore.
bottom_show_errors_tooltip = Mostra/Nasconde il pannello errori inferiore.
bottom_show_upper_notebook_tooltip = Mostra/Nasconde il pannello comandi.
# Progress Window
progress_stop_button = Ferma
# About Window
about_repository_button_tooltip = Collegamento alla pagina del codice sorgente nell'archivio.
about_donation_button_tooltip = Collegamento alla pagina delle donazioni.
about_instruction_button_tooltip = Colegamento alla pagina delle istruzioni.
about_translation_button_tooltip = Link alla pagina Crowdin con le traduzioni delle app. Ufficialmente polacco e inglese sono supportati, ma qualsiasi aiuto con altre lingue sarà apprezzato.
about_repository_button = Archivio
about_donation_button = Donazioni
about_instruction_button = Istruzioni
about_translation_button = Traduzione
# Header
header_setting_button_tooltip = Apre la finestra impostazioni.
header_about_button_tooltip = Apre la finestra delle informazioni sul programma.

# Settings


## General

settings_save_at_exit_button_tooltip = Salva la configurazione su file alla chiusura del programma.
settings_load_at_start_button_tooltip =
    Caricamento della configurazione all'avvio del programma.
    
    Non selezionando questa opzione verranno caricate le impostazioni predefinite.
settings_confirm_deletion_button_tooltip = Mostra richiesta di confema alla selezione del pulsante Cancella.
settings_confirm_link_button_tooltip = Mostra richiesta di conferma alla selezione di Crea collegamento.
settings_confirm_group_deletion_button_tooltip = Mostra richiesta di conferma al tentativo di cancellare tutti gli elementi in un gruppo.
settings_show_text_view_button_tooltip = Mostra il pannello errori in basso.
settings_use_cache_button_tooltip = Opzione per non utilizzare la funzione di caching.
settings_save_also_as_json_button_tooltip = Salva la cache in formato JSON umano. È possibile modificarne il contenuto. La cache da questo file verrà letta automaticamente dall'app se la cache in formato binario (con estensione bid) sarà mancante.
settings_use_trash_button_tooltip = Se selezionata, sposterà i file el cestino anziché cancellarli definitivamente.
settings_language_label_tooltip = Permette di selezionare la lingua dell'interfaccia tra quelle disponibili.
settings_save_at_exit_button = Salva la configurazione all'uscita
settings_load_at_start_button = Carica la configurazione all'avvio
settings_confirm_deletion_button = Mostra finestra di conferma alla cancellazione di qualsiasi file
settings_confirm_link_button = Mostra finestra di conferma alla creazione di collegamenti per qualsiasi file
settings_confirm_group_deletion_button = Mostra finestra di conferma alla cancellazione di tutti gli elementi in un gruppo
settings_show_text_view_button = Mostra il pannello testuale inferiore
settings_use_cache_button = Utilizza cache
settings_save_also_as_json_button = Salva la cache anche nel file JSON
settings_use_trash_button = Sposta i file rimossi nel cestino
settings_language_label = Lingua
settings_multiple_delete_outdated_cache_checkbutton = Cancella automaticamente la cache obsoleta
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Permette di cancellare i risultati cache obsoleti che puntano a file inesistenti.
    
    Se attiva, il programma verifica, al caricamento, che tutti gli elementi puntino a file validi, ignorando quelli interrotti.
    
    Disattivare questa opzione aiuterà nella ricerca su dischi esterni, in modo che le voci di cache relativi non vengano cancellate alla ricerca successiva.
    
    Nel caso si abbiano centinaia di migliaia di elementi nella cache, è suggerito selezionare questa opzione per accelerare il caricamento ed il salvataggio della cache all'avvio ed alla fine della ricerca.
settings_notebook_general = Generale
settings_notebook_duplicates = Duplicati
settings_notebook_images = Immagini simili
settings_notebook_videos = Video simili

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Mostra l'anteprima sulla destra alla selezione delle immagini.
settings_multiple_image_preview_checkbutton = Mostra anteprima immagini
settings_multiple_clear_cache_button_tooltip =
    Cancella manualmente gli elementi obsoleti dalla cache.
    Dovrebbe essere utilizzata solo se le cancellazione automatica della cache è disattivata.
settings_multiple_clear_cache_button = Rimuove risultati obsoleti dalla cache delle immagini

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Nascondi tutti i file tranne uno, se puntano agli stessi dati (sono collegati fisicamente).
    
    Ad esempio, se su un disco ci sono 7 file fisicamente colleati a dati specifici ed un altro file con gli stessi dati ma inode differente, verranno mostrati solo un unico file ed un collegamento fisico tra quelli esistenti.
settings_duplicates_minimal_size_entry_tooltip =
    Permette di impostare le dimensioni minime dei file che verranno processati tramite cache.
    
    La selezione di valori minori genererà più elementi, che velocizzeranno la ricerca, ma rallenteranno il caricamento ed il salvataggio della cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Abilita il caching dei prehash (hash generati da una piccola porzione dei file) che permette di scartare prima gli elementi non duplicati.
    
    È disattivata per impostazione predefinita perché può causare rallentamenti in alcune condizioni.
    
    È fortemente consigliato utilizzare questa funzione quando si cerca tra centinaia di migliaia o milioni di file, perché può velocizzare di molti fattori la ricerca.
settings_duplicates_prehash_minimal_entry_tooltip = Dimensione minima delle voci della cache
settings_duplicates_hide_hard_link_button = Nascondi collegamenti fisici (solo in Linux e MacOS)
settings_duplicates_prehash_checkbutton = Utilizza la cash prehash
settings_duplicates_minimal_size_cache_label = Dimensione minima in byte dei dei file salvati nella cache
settings_duplicates_minimal_size_cache_prehash_label = Dimensione minima in byte dei file salvati nella cache prehash

## Saving/Loading settings

settings_saving_button_tooltip = Salva le impostazioni selezionate su file
settings_loading_button_tooltip = Carica le impostazioni di configurazione da file.
settings_reset_button_tooltip = Reimposta le impostazioni predefinite.
settings_saving_button = Salva configurazione
settings_loading_button = Carica configurazione
settings_reset_button = Reimposta configurazione

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Apre la cartellaa dove sono memorizzati i file della cache.
    
    La loro modifica causerà la visualizzazione di risultati invalidi ma ad esempio modificando il percorso può risparmiare tempo se si spostano grosse quantità di file in percorsi differenti big amount of files to different place.
    
    Puoi copiare questi file tra computer per risparmiare tempo di ricerca (in caso di struttura cartele simile).
    
    In caso di problemi con la cache, questi file possono essere rimossi, affinché il programmi li rigeneri.
settings_folder_settings_open_tooltip =
    Apri la cartella dove è salvata la configurazione di Czkawka.
    
    La modifica manuale può comprometterne il funzionamento.
settings_folder_cache_open = Apri la cartella della cache
settings_folder_settings_open = Apri la cartella delle impostazioni
# Compute results
compute_stopped_by_user = Ricerca interrotta dall'utente
compute_found_duplicates_hash_size = Trovati { $number_files } duplicati in { $number_groups } gruppi che occupano { $size }
compute_found_duplicates_name = Trovati { $number_files } duplicati in { $number_groups } gruppi
compute_found_empty_folders = Trovate { $number_files } cartelle vuote
compute_found_empty_files = Trovati { $number_files } file vuoti
compute_found_big_files = Trovati { $number_files } grandi file
compute_found_temporary_files = Trovati { $number_files } file temporanei
compute_found_images = Trovate { $number_files } immagini simili in { $number_groups } gruppi
compute_found_videos = Trovati { $number_files } video simili in { $number_groups } gruppi
compute_found_music = Trovati { $number_files } file musicali simili in { $number_groups } gruppi
compute_found_invalid_symlinks = Trovati { $number_files } colegamenti simbolici invalidi
compute_found_broken_files = Trovati { $number_files } file corrotti
# Progress window
progress_scanning_general_file = Processando { $file_number } file
progress_scanning_broken_files = Verificando { $file_checked }/{ $all_files } file
progress_scanning_video = Hashing di { $file_checked }/{ $all_files } video
progress_scanning_image = Hashing di { $file_checked }/{ $all_files } image
progress_comparing_image_hashes = Confrontando { $file_checked }/{ $all_files } hash delle immagini
progress_scanning_music_tags_end = Confrontando le etichette di { $file_checked }/{ $all_files } file musicali
progress_scanning_music_tags = Leggendo le etichette di { $file_checked }/{ $all_files } file musicali
progress_scanning_empty_folders = Verificando { $folder_number } cartelle
progress_scanning_size = Leggendo le dimensioni di { $file_number } file
progress_scanning_name = Leggendo il nome di { $file_number } file
progress_analyzed_partial_hash = Analizzato gli hash parziali di { $file_checked }/{ $all_files } file
progress_analyzed_full_hash = Analizzato gli hash completi di { $file_checked }/{ $all_files } file
progress_current_stage = Fase attuale:{ "  " }
progress_all_stages = Tutte le fasi:{ "  " }
# Saving loading 
saving_loading_saving_success = Configurazione salvata su file
saving_loading_saving_failure = Salvataggio della configurazione nel file { $name } fallito.
saving_loading_reset_configuration = La configurazione corrente è stata cancellata.
saving_loading_loading_success = Caricamento configurazione da file avvenuto con successo
saving_loading_invalid_string = Trovato risultato non valido nella chiave "{ $key }" - "{ $result }" non è una stringa.
saving_loading_invalid_int = Trovato risultato non valido nella chiave "{ $key }" - "{ $result }" non è un numero intero.
saving_loading_invalid_bool = Trovato risultato non valido nella chiave "{ $key }" - "{ $result }" non è un valore booleano.
saving_loading_decode_problem_bool = Decodifica booleana fallita per la chiave "{ $key }"; trovato "{ $result }", ma i valori possibili sono 0, 1, 'true' o 'false'.
saving_loading_saving_same_keys = Tentativo di salvataggio dell'impostazione con chiave duplicata "{ $key }".
saving_loading_failed_to_get_home_directory = Recupero cartella home fallito, per caricamento/salvataggio del file di configurazione.
saving_loading_folder_config_instead_file = Impossibile create o caricare/salvare il file di configurazione al percorso "{ $path }" poiché la cartella esiste già.
saving_loading_failed_to_create_configuration_folder = Creazione cartella di configurazione "{ $path }" fallita, motivo "{ $reason }".
saving_loading_failed_to_create_config_file = Impossibile creare il file di configurazione "{ $path }", motivo "{ $reason }".
saving_loading_failed_to_read_config_file = Impossibile caricare configurazione da "{ $path }" poiché il file non esiste o non è un file.
saving_loading_failed_to_read_data_from_file = Impossibile leggere il file "{ $path }", motivo "{ $reason }".
saving_loading_orphan_data = Trovati dati orfani "{ $data }" nella riga "{ $line }".
saving_loading_not_valid = L'impostazione "{ $data }" non esiste per l'attuale versiione del programma.
# Invalid symlinks
invalid_symlink_infinite_recursion = Ricorsione infinita
invalid_symlink_non_existent_destination = File di destinazione non esistente
# Other
searching_for_data = Ricerca dei dati, può durare a lungo, attendere prego...
text_view_messages = MESSAGGI
text_view_warnings = ATTENZIONE
text_view_errors = ERRORI
about_window_motto = Questo programma può essere usato liberamente e lo sarà sempre.
# Various dialog
dialogs_ask_next_time = Chiedi la prossima volta
delete_file_failed = Rimozione file { $name } fallita, ragione { $reason }
delete_title_dialog = Conferma di cancellazione
delete_question_label = Sei sicuro di cancellare i file?
delete_all_files_in_group_title = Conferma di cancellazione di tutti i file nel gruppo
delete_all_files_in_group_label1 = In alcuni gruppi tutti gli elementi sono selezionati.
delete_all_files_in_group_label2 = Sei sicuro di cancellarli tutti?
delete_folder_failed = Cancellazione cartella { $dir } fallita perché la cartella non esiste, non si hanno permessi sufficienti o perché non vuota.
delete_items_label = { $items } file verranno rimossi.
delete_items_groups_label = { $items } file da { $groups } gruppi verranno rimossi.
hardlink_failed = Collegamento fisico fallito
hard_sym_invalid_selection_title_dialog = Selezione invalida in alcuni gruppi
hard_sym_invalid_selection_label_1 = In alcuni gruppi c'è solo una voce selezionata e verrà ignorata.
hard_sym_invalid_selection_label_2 = Per poter collegare simbolicamente/fisicamente questi files, devono essere selezionati almeno due risultati.
hard_sym_invalid_selection_label_3 = Il primo nel gruppo sarà considerato l'originale ed inalteratom, ma il secondo ed i successivi verranno modificati.
hard_sym_link_title_dialog = Conferma collegamento
hard_sym_link_label = Sei sicuro di collegare questi file?
move_folder_failed = Spostamento cartella { $name } fallito, ragione { $reason }
move_file_failed = Spostamento file { $name } fallito, ragione { $reason }
move_files_title_dialog = Seleziona la cartella dove vuoi spostare i file duplicati
move_files_choose_more_than_1_path = Solo un percorso deve essere selezionato per copiarvi i file, selezionati { $path_number }
move_stats = { $num_files }/{ $all_files } elementi spostati con successo
save_results_to_file = Risultati salvati su file
search_not_choosing_any_music = ERRORE: Devi selezionare almeno una casella dei metodi di ricerca musicali.
include_folders_dialog_title = Cartelle incluse
exclude_folders_dialog_title = Cartelle escluse
include_manually_directories_dialog_title = Aggiungi cartella manualmente
cache_properly_cleared = Cache cancellata con successo
cache_clear_duplicates_title = Cancellazione cache dei duplicati
cache_clear_similar_images_title = Cancellazione cache delle immagini simili
cache_clear_similar_videos_title = Cancellazione cache dei video simili
cache_clear_message_label_1 = Vuoi cancellare le voci obsolete dalla cache?
cache_clear_message_label_2 = Quest'operazione cancellera tutti gli elementi che puntano a file invalidi.
cache_clear_message_label_3 = Questo velocizzerà un po' il caricamento/savataggio della cache.
cache_clear_message_label_4 = WARNING: L'operazione cancellerà tutti i dati della cache dai dischi esterni non collegati, quindi gli hash dpvranno essere generati nuovamente.
# Show preview
preview_temporary_file = Impossibile aprire immagine temporanea { $name }, ragione { $reason }
preview_0_size = Impossibile creare anteprima dell'immagine { $name }, con larghezza o altezza 0
preview_temporary_image_save = Impossibile salvare immagine temporanea in { $name }, ragione { $reason }
preview_temporary_image_remove = Impossibile cancellare immagine temporanea { $name }, ragione { $reason }
preview_failed_to_create_cache_dir = Impossibile creare cartella { $name } necessaria per l'anteprima immagine, ragione { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Gruppo { $current_group }/{ $all_groups } ({ $images_in_group } immagini)
compare_move_left_button = L
compare_move_right_button = R
