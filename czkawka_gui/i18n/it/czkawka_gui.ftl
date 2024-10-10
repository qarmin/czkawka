# Window titles
window_settings_title = Impostazioni
window_main_title = Czkawka (Singhiozzo)
window_progress_title = Ricerca
window_compare_images = Confronta le immagini
# General
general_ok_button = Ok
general_close_button = Chiudi
# Main window
music_title_checkbox = Titolo
music_artist_checkbox = Artista
music_year_checkbox = Anno
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Genere
music_length_checkbox = Durata
music_comparison_checkbox = Confronto approssimativo
music_checking_by_tags = Etichette
music_checking_by_content = Contenuto
same_music_seconds_label = Durata minima del frammento
same_music_similarity_label = Differenza massima
music_compare_only_in_title_group = Confronta solo nel titolo
music_compare_only_in_title_group_tooltip =
    Se abilitato, i file vengono raggruppati per titolo e poi confrontati tra loro.
    
    Con 10000 file, invece quasi 100 milioni di confronti di solito ci saranno circa 20000 confronti.
same_music_tooltip =
    La ricerca di file musicali simili dal suo contenuto può essere configurata impostando:
    
    - Il tempo minimo di frammento dopo il quale i file musicali possono essere identificati come simili
    - La differenza massima tra due frammenti testati
    
    La chiave per ottenere buoni risultati è trovare combinazioni sensate di questi parametri, per fornito.
    
    Impostando il tempo minimo a 5s e la differenza massima a 1.0, cercherà frammenti quasi identici nei file.
    Un tempo di 20 anni e una differenza massima di 6.0, d'altra parte, funziona bene per trovare remix/versioni live ecc.
    
    Per impostazione predefinita, ogni file musicale viene confrontato tra loro e questo può richiedere molto tempo quando si testano molti file, quindi è di solito meglio usare le cartelle di riferimento e specificare quali file devono essere confrontati tra loro (con la stessa quantità di file, il confronto delle impronte digitali sarà più veloce di almeno 4x che senza cartelle di riferimento).
music_comparison_checkbox_tooltip =
    Cerca file musicali simili usando l'IA, che utilizza l'apprendimento automatico per rimuovere parentesi da una frase. Ad esempio, con questa opzione abilitata, i file in questione saranno considerati duplicati:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Case Sensitive
duplicate_case_sensitive_name_tooltip =
    Se abilitato, raggruppa solo i record quando hanno esattamente lo stesso nome, ad es. Żołd <-> Żołd
    La disattivazione di tale opzione raggrupperà i nomi senza controllare se ogni lettera ha le stesse dimensioni, ad esempio żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Dimensione e nome
duplicate_mode_name_combo_box = Nome
duplicate_mode_size_combo_box = Dimensione
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka offre 3 tipi di hash:
    
    Blake3 - funzione hash crittografica. Questo è il valore predefinito perché è molto veloce.
    
    CRC32 - semplice funzione di hash. Questo dovrebbe essere più veloce di Blake3, ma può molto raramente avere alcune collisioni.
    
    XXH3 - molto simile in termini di prestazioni e qualità di hash a Blake3 (ma non crittografica). Quindi, tali modalità possono essere facilmente intercambiate.
duplicate_check_method_tooltip =
    Per ora, Czkawka offre tre tipi di metodo per trovare i duplicati di:
    
    Nome - Trova i file che hanno lo stesso nome.
    
    Dimensione - Trova i file che hanno la stessa dimensione.
    
    Hash - Trova i file che hanno lo stesso contenuto. Questa modalità fa hash sul file e in seguito confronta questo hash per trovare i duplicati. Questa modalità è il modo più sicuro per trovare i duplicati. App usa pesantemente la cache, quindi la seconda e ulteriori scansioni degli stessi dati dovrebbero essere molto più veloce del primo.
image_hash_size_tooltip =
    Ogni immagine selezionata produce un hash speciale che può essere confrontato l'uno con l'altro, e una piccola differenza tra loro significa che queste immagini sono simili.
    
    8 hash size è abbastanza buono per trovare immagini che sono solo un po 'simili all'originale. Con un insieme più grande di immagini (>1000), questo produrrà una grande quantità di falsi positivi, quindi vi consiglio di utilizzare una dimensione di hash più grande in questo caso.
    
    16 è la dimensione predefinita dell'hash che è abbastanza un buon compromesso tra trovare anche un po 'di immagini simili e avere solo una piccola quantità di collisioni di hash.
    
    32 e 64 hash trovano solo immagini molto simili, ma non dovrebbero avere quasi falsi positivi (forse tranne alcune immagini con canale alfa).
image_resize_filter_tooltip =
    Per calcolare l'hash dell'immagine, la libreria deve prima ridimensionarla.
    
    Dipende dall'algoritmo scelto, l'immagine risultante utilizzata per calcolare l'hash apparirà un po' diversa.
    
    L'algoritmo più veloce da usare, ma anche quello che dà i peggiori risultati, è più vicino. È abilitato per impostazione predefinita, perché con 16x16 dimensioni hash di qualità inferiore non è davvero visibile.
    
    Con 8x8 dimensioni di hash si consiglia di utilizzare un algoritmo diverso da quello più vicino, per avere migliori gruppi di immagini.
image_hash_alg_tooltip =
    Gli utenti possono scegliere tra uno dei molti algoritmi di calcolo dell'hash.
    
    Ognuno ha punti forti e più deboli e a volte darà risultati migliori e a volte peggiori per immagini diverse.
    
    Quindi, per determinare quello migliore per te, è necessario un test manuale.
big_files_mode_combobox_tooltip = Consente di cercare file più piccoli/più grandi
big_files_mode_label = File controllati
big_files_mode_smallest_combo_box = Il Più Piccolo
big_files_mode_biggest_combo_box = Il Più Grande
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
main_notebook_bad_extensions = Estensioni Errate
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
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Durata
main_tree_view_column_genre = Genere
main_tree_view_column_symlink_file_name = Nome collegamento
main_tree_view_column_symlink_folder = Cartella Collegamenti Simbolici
main_tree_view_column_destination_path = Percorso di destinazione
main_tree_view_column_type_of_error = Tipo di Errore
main_tree_view_column_current_extension = Estensione Corrente
main_tree_view_column_proper_extensions = Estensione Corretta
main_label_check_method = Metodo di verifica
main_label_hash_type = Tipo hash
main_label_hash_size = Dimensione hash
main_label_size_bytes = Dimensione (byte)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Numero di file visualizzati
main_label_resize_algorithm = Metodo di ridimensionamento
main_label_similarity = Similitudine{ "   " }
main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Compresso
main_check_box_broken_files_image = Immagine
check_button_general_same_size = Ignora stesse dimensioni
check_button_general_same_size_tooltip = Ignora i file con dimensioni identiche nei risultati - di solito sono duplicati 1:1
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
upper_manual_add_included_button_tooltip =
    Aggiungi nome directory da cercare a mano.
    
    Per aggiungere più percorsi contemporaneamente, separarli da ;
    
    /home/roman;/home/rozkaz aggiungerà due directory /home/roman e /home/rozkaz
upper_add_included_button_tooltip = Aggiungi nuova cartella per la ricerca
upper_remove_included_button_tooltip = Cancella cartella dalla ricerca
upper_manual_add_excluded_button_tooltip =
    Aggiungi nome directory escluso a mano.
    
    Per aggiungere più percorsi contemporaneamente, separarli da ;
    
    /home/roman;/home/krokiet aggiungerà due directory /home/roman e /home/keokiet
upper_add_excluded_button_tooltip = Aggiunge una cartella da escludere dalla ricerca
upper_remove_excluded_button_tooltip = Rimuove una cartella da quelle escluse
upper_notebook_items_configuration = Configurazione degli oggetti
upper_notebook_excluded_directories = Cartelle escluse
upper_notebook_included_directories = Cartelle incluse
upper_allowed_extensions_tooltip =
    Le estensioni consentite devono essere separate da virgole (di default tutte sono disponibili).
    
    Sono disponibili anche i seguenti Macro, che aggiungono più estensioni contemporaneamente: IMAGE, VIDEO, MUSIC, TESTO.
    
    Esempio di utilizzo ".exe, IMAGE, VIDEO, .rar, 7z" - questo significa che le immagini (e. . jpg, png), video (ad esempio avi, mp4), exe, rar e file 7z verranno analizzati.
upper_excluded_extensions_tooltip =
    Elenco dei file disabilitati che verranno ignorati nella scansione.
    
    Quando si usano entrambe le estensioni consentite e disabilitate, questo ha la priorità maggiore, quindi il file non verrà selezionato.
upper_excluded_items_tooltip =
    Gli elementi esclusi devono contenere * caratteri jolly e devono essere separati da virgole.
    Questo è più lento delle directory escluse, quindi usalo attentamente.
upper_excluded_items = Voci escluse:
upper_allowed_extensions = Estensioni permesse:
upper_excluded_extensions = Estensioni Disabilitate:
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
    Seleziona i record per percorso.
    
    Esempio di utilizzo:
    /home/pimpek/rzecz.txt può essere trovato con /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Seleziona i record per nomi file.
    
    Esempio di utilizzo:
    /usr/ping/pong.txt può essere trovato con *ong*
popover_custom_regex_check_button_entry_tooltip =
    Seleziona i record per Regex specificato.
    
    Con questa modalità, il testo cercato è Percorso con Nome.
    
    Esempio di utilizzo:
    /usr/bin/ziemniak. xt può essere trovato con /ziem[a-z]+
    
    Questo utilizza l'implementazione regex Rust predefinita. Puoi leggere di più qui: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Abilita rilevamento maiuscolo/minuscolo.
    
    Quando disabilitato /home/* trova sia /HoMe/roman che /home/roman.
popover_custom_not_all_check_button_tooltip =
    Impedisce di selezionare tutti i record nel gruppo.
    
    Questo è abilitato per impostazione predefinita, perché nella maggior parte delle situazioni, non si desidera eliminare entrambi i file originali e duplicati, ma si desidera lasciare almeno un file.
    
    ATTENZIONE: Questa impostazione non funziona se hai già selezionato manualmente tutti i risultati in un gruppo.
popover_custom_regex_path_label = Percorso
popover_custom_regex_name_label = Nome
popover_custom_regex_regex_label = Regex Percorso + Nome
popover_custom_case_sensitive_check_button = Differenzia maiuscole/minuscole
popover_custom_all_in_group_label = Non selezionare tutte le voci in un gruppo
popover_custom_mode_unselect = Deselezione personalizzata
popover_custom_mode_select = Selezione personalizzata
popover_sort_file_name = Nome del file
popover_sort_folder_name = Nome cartella
popover_sort_full_name = Nome e cognome
popover_sort_size = Dimensione
popover_sort_selection = Selezione
popover_invalid_regex = Regex non valida
popover_valid_regex = Regex valida
# Bottom buttons
bottom_search_button = Cerca
bottom_select_button = Seleziona
bottom_delete_button = Cancella
bottom_save_button = Salva
bottom_symlink_button = Collegamenti simbolici
bottom_hardlink_button = Collegamenti fisici
bottom_move_button = Sposta
bottom_sort_button = Ordina
bottom_search_button_tooltip = Avvia ricerca
bottom_select_button_tooltip = Seleziona record. Solo i file/cartelle selezionati possono essere elaborati in seguito.
bottom_delete_button_tooltip = Cancella i file/cartelle selezionati
bottom_save_button_tooltip = Salva i risultati della ricerca in un file
bottom_symlink_button_tooltip =
    Crea collegamenti simbolici.
    Funziona solo quando sono selezionati almeno due risultati in un gruppo.
    Il primo è invariato e il secondo e poi sono symlinked al primo.
bottom_hardlink_button_tooltip =
    Crea collegamenti fisici.
    Funziona solo quando sono selezionati almeno due risultati in un gruppo.
    Il primo è invariato e il secondo e poi sono hardlinked al primo.
bottom_hardlink_button_not_available_tooltip =
    Crea collegamenti fisici.
    Il pulsante è disabilitato, perché non è possibile creare collegamenti fisici.
    I collegamenti fisici funzionano solo con i privilegi di amministratore su Windows, quindi assicurati di eseguire l'app come amministratore.
    Se l'app funziona già con tali privilegi, controlla problemi simili su Github.
bottom_move_button_tooltip =
    Sposta i file nella directory scelta.
    Copia tutti i file nella directory senza conservare l'albero delle directory.
    Quando si tenta di spostare due file con il nome identico nella cartella, il secondo fallirà e mostrerà errore.
bottom_sort_button_tooltip = Ordina file/cartelle in base al metodo selezionato.
bottom_show_errors_tooltip = Mostra/Nasconde il pannello di testo inferiore.
bottom_show_upper_notebook_tooltip = Mostra/Nasconde il pannello comandi.
# Progress Window
progress_stop_button = Ferma
progress_stop_additional_message = Interrompi richiesta
# About Window
about_repository_button_tooltip = Link alla pagina del repository con il codice sorgente.
about_donation_button_tooltip = Link alla pagina della donazione.
about_instruction_button_tooltip = Link alla pagina delle istruzioni.
about_translation_button_tooltip = Link alla pagina Crowdin con le traduzioni delle app. Ufficialmente polacco e inglese sono supportati.
about_repository_button = Archivio
about_donation_button = Donazioni
about_instruction_button = Istruzioni
about_translation_button = Traduzione
# Header
header_setting_button_tooltip = Apre la finestra delle impostazioni.
header_about_button_tooltip = Apre la finestra delle informazioni sul programma.

# Settings


## General

settings_number_of_threads = Numero di thread usati
settings_number_of_threads_tooltip = Numero di thread usati, 0 significa che tutti i thread disponibili saranno utilizzati.
settings_use_rust_preview = Usa librerie esterne invece di gtk per caricare anteprime
settings_use_rust_preview_tooltip =
    Utilizzando le anteprime gtk a volte sarà più veloce e supporterà più formati, ma a volte questo potrebbe essere esattamente il contrario.
    
    Se hai problemi con il caricamento delle anteprime, puoi provare a modificare questa impostazione.
    
    Nei sistemi non-linux, si consiglia di utilizzare questa opzione, perché gtk-pixbuf non è sempre disponibile e quindi disabilitando questa opzione non caricherà le anteprime di alcune immagini.
settings_label_restart = È necessario riavviare l'app per applicare le impostazioni!
settings_ignore_other_filesystems = Ignora altri filesystem (solo Linux)
settings_ignore_other_filesystems_tooltip =
    ignora i file che non sono nello stesso file system delle directory cercate.
    
    Funziona come l'opzione -xdev nel comando find su Linux
settings_save_at_exit_button_tooltip = Salva la configurazione su file quando chiudi l'app.
settings_load_at_start_button_tooltip =
    Carica la configurazione dal file all'apertura dell'applicazione.
    
    Se non è abilitata, verranno utilizzate le impostazioni predefinite.
settings_confirm_deletion_button_tooltip = Mostra la finestra di conferma quando si fa clic sul pulsante Elimina.
settings_confirm_link_button_tooltip = Mostra la finestra di conferma quando si fa clic sul pulsante hard/symlink.
settings_confirm_group_deletion_button_tooltip = Mostra la finestra di avviso quando si tenta di eliminare tutti i record dal gruppo.
settings_show_text_view_button_tooltip = Mostra il pannello di testo in fondo all'interfaccia utente.
settings_use_cache_button_tooltip = Usa cache file.
settings_save_also_as_json_button_tooltip = Salva la cache in formato JSON (leggibile). È possibile modificarne il contenuto. La cache da questo file verrà letta automaticamente dall'app se manca la cache in formato binario (con estensione bid).
settings_use_trash_button_tooltip = Sposta i file nel cestino invece eliminandoli in modo permanente.
settings_language_label_tooltip = Lingua per l'interfaccia utente.
settings_save_at_exit_button = Salva la configurazione alla chiusura dell'app
settings_load_at_start_button = Carica la configurazione quando apri l'app
settings_confirm_deletion_button = Mostra finestra di conferma alla cancellazione di qualsiasi file
settings_confirm_link_button = Mostra finestra di conferma alla creazione di collegamenti per qualsiasi file
settings_confirm_group_deletion_button = Mostra finestra di conferma alla cancellazione di tutti gli elementi in un gruppo
settings_show_text_view_button = Mostra il pannello testuale inferiore
settings_use_cache_button = Utilizza cache
settings_save_also_as_json_button = Salva anche la cache come file JSON
settings_use_trash_button = Sposta i file rimossi nel cestino
settings_language_label = Lingua
settings_multiple_delete_outdated_cache_checkbutton = Cancella automaticamente la cache obsoleta
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Elimina i risultati della cache obsoleti che puntano a file inesistenti.
    
    Quando abilitata, l'app si assicura durante il caricamento dei record, che tutti i record puntino a file validi (quelli interrotti vengono ignorati).
    
    Disabilitare questo aiuterà durante la scansione dei file su unità esterne, quindi le voci della cache su di loro non verranno eliminate nella prossima scansione.
    
    Nel caso di avere centinaia di migliaia di record nella cache, si consiglia di abilitare questa funzione, che velocizzerà il caricamento della cache/salvataggio all'inizio/fine della scansione.
settings_notebook_general = Generale
settings_notebook_duplicates = Duplicati
settings_notebook_images = Immagini simili
settings_notebook_videos = Video simili

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Mostra l'anteprima sul lato destro (quando si seleziona un file immagine).
settings_multiple_image_preview_checkbutton = Mostra anteprima immagini
settings_multiple_clear_cache_button_tooltip =
    Pulisci manualmente la cache delle voci obsolete.
    Questo dovrebbe essere usato solo se la compensazione automatica è stata disabilitata.
settings_multiple_clear_cache_button = Rimuovi i risultati obsoleti dalla cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Nasconde tutti i file tranne uno, se tutti puntano agli stessi dati (sono hardlinked).
    
    Esempio: Nel caso in cui ci siano (su disco) sette file che sono collegati a dati specifici e un file diverso con gli stessi dati ma un inode diverso, poi nel mirino duplicato, verrà mostrato solo un file univoco e un file da quelli hardlink.
settings_duplicates_minimal_size_entry_tooltip =
    Imposta la dimensione minima del file che verrà memorizzata nella cache.
    
    Scegliendo un valore più piccolo si genereranno più record. Questo accelererà la ricerca, ma rallenterà il caricamento / il salvataggio della cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Abilita la cache di prehash (un hash calcolato da una piccola parte del file) che consente il ritiro anticipato di risultati non duplicati.
    
    È disabilitato per impostazione predefinita perché può causare rallentamenti in alcune situazioni.
    
    Si consiglia vivamente di usarlo durante la scansione di centinaia di migliaia o milioni di file, perché può accelerare la ricerca di più volte.
settings_duplicates_prehash_minimal_entry_tooltip = Dimensione minima delle voci della cache
settings_duplicates_hide_hard_link_button = Nascondi collegamenti rigidi (solo Linux e macOS)
settings_duplicates_prehash_checkbutton = Utilizza la cash prehash
settings_duplicates_minimal_size_cache_label = Dimensione minima dei file (in byte) salvati nella cache
settings_duplicates_minimal_size_cache_prehash_label = Dimensione minima dei file (in byte) salvati nella cache prehash

## Saving/Loading settings

settings_saving_button_tooltip = Salva la configurazione attuale delle impostazioni su file.
settings_loading_button_tooltip = Carica le impostazioni dal file e sostituisci con esse la configurazione corrente.
settings_reset_button_tooltip = Reimposta la configurazione corrente a quella predefinita.
settings_saving_button = Salva configurazione
settings_loading_button = Carica configurazione
settings_reset_button = Reimposta configurazione

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Apre la cartella in cui sono memorizzati i file cache txt.
    
    La modifica dei file cache può causare la visualizzazione di risultati non validi. Tuttavia, la modifica del percorso può risparmiare tempo quando si sposta una grande quantità di file in una posizione diversa.
    
    È possibile copiare questi file tra computer per risparmiare tempo sulla scansione di nuovo per i file (ovviamente se hanno una struttura di directory simile).
    
    In caso di problemi con la cache, questi file possono essere rimossi. L'app li rigenererà automaticamente.
settings_folder_settings_open_tooltip =
    Apre la cartella in cui viene memorizzata la configurazione Czkawka.
    
    ATTENZIONE: Modificare manualmente la configurazione potrebbe rompere il flusso di lavoro.
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
compute_found_bad_extensions = Trovati { $number_files } file con estensioni non valide
# Progress window
progress_scanning_general_file = Processando { $file_number } file
progress_scanning_extension_of_files = Verifica estensione del file { $file_checked }/{ $all_files }
progress_scanning_broken_files = Verificando { $file_checked }/{ $all_files } file
progress_scanning_video = Hashing di { $file_checked }/{ $all_files } video
progress_scanning_image = Hashing di { $file_checked }/{ $all_files } image
progress_comparing_image_hashes = Confrontando { $file_checked }/{ $all_files } hash delle immagini
progress_scanning_music_tags_end = Confrontando le etichette di { $file_checked }/{ $all_files } file musicali
progress_scanning_music_tags = Leggendo le etichette di { $file_checked }/{ $all_files } file musicali
progress_scanning_music_content_end = Confronto delle impronte digitali di { $file_checked }/{ $all_files } file musicale
progress_scanning_music_content = Calcolo dell'impronta digitale di { $file_checked }/{ $all_files } file musicale
progress_scanning_empty_folders = Verificando { $folder_number } cartelle
progress_scanning_size = Leggendo le dimensioni di { $file_number } file
progress_scanning_size_name = Scansione nome e dimensione del file { $file_number }
progress_scanning_name = Leggendo il nome di { $file_number } file
progress_analyzed_partial_hash = Analizzato gli hash parziali di { $file_checked }/{ $all_files } file
progress_analyzed_full_hash = Analizzato gli hash completi di { $file_checked }/{ $all_files } file
progress_prehash_cache_loading = Caricamento della cache prehash
progress_prehash_cache_saving = Salvataggio della cache prehash
progress_hash_cache_loading = Caricamento della cache hash
progress_hash_cache_saving = Salvataggio della cache hash
progress_cache_loading = Caricamento cache
progress_cache_saving = Salvataggio cache
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
saving_loading_failed_to_read_config_file = Impossibile caricare la configurazione da "{ $path }" perché non esiste o non è un file.
saving_loading_failed_to_read_data_from_file = Impossibile leggere il file "{ $path }", motivo "{ $reason }".
saving_loading_orphan_data = Trovati dati orfani "{ $data }" nella riga "{ $line }".
saving_loading_not_valid = L'impostazione "{ $data }" non esiste nella versione corrente dell'app.
# Invalid symlinks
invalid_symlink_infinite_recursion = Ricorsione infinita
invalid_symlink_non_existent_destination = File di destinazione inesistente
# Other
selected_all_reference_folders = Impossibile avviare la ricerca, quando tutte le directory sono impostate come cartelle di riferimento
searching_for_data = Ricerca dei dati, può durare a lungo, attendere prego...
text_view_messages = MESSAGGI
text_view_warnings = ATTENZIONE
text_view_errors = ERRORI
about_window_motto = Questo programma può essere usato liberamente e lo sarà sempre.
# Various dialog
dialogs_ask_next_time = Chiedi la prossima volta
delete_file_failed = Impossibile eliminare il file { $name }, motivo { $reason }
delete_title_dialog = Conferma di cancellazione
delete_question_label = Sei sicuro di cancellare i file?
delete_all_files_in_group_title = Conferma di cancellazione di tutti i file nel gruppo
delete_all_files_in_group_label1 = In alcuni gruppi tutti i record sono selezionati.
delete_all_files_in_group_label2 = Sei sicuro di cancellarli tutti?
delete_folder_failed = Impossibile eliminare la cartella { $dir } perché la cartella non esiste, non hai i permessi o la cartella non è vuota.
delete_items_label = { $items } i file verranno eliminati.
delete_items_groups_label = { $items } i file da { $groups } gruppi verranno eliminati.
hardlink_failed = Collegamento fisico fallito
hard_sym_invalid_selection_title_dialog = Selezione invalida in alcuni gruppi
hard_sym_invalid_selection_label_1 = In alcuni gruppi c'è solo un record selezionato e verrà ignorato.
hard_sym_invalid_selection_label_2 = Per essere in grado di collegare hard/sym questi file, è necessario selezionare almeno due risultati nel gruppo.
hard_sym_invalid_selection_label_3 = Il primo nel gruppo sarà considerato l'originale ed inalteratom, ma il secondo ed i successivi verranno modificati.
hard_sym_link_title_dialog = Conferma collegamento
hard_sym_link_label = Sei sicuro di voler collegare questi file?
move_folder_failed = Spostamento cartella { $name } fallito, ragione { $reason }
move_file_failed = Spostamento file { $name } fallito, ragione { $reason }
move_files_title_dialog = Seleziona la cartella dove vuoi spostare i file duplicati
move_files_choose_more_than_1_path = Solo un percorso può essere selezionato per essere in grado di copiare i file duplicati, selezionato { $path_number }.
move_stats = { $num_files }/{ $all_files } elementi spostati con successo
save_results_to_file = Risultati salvati sia in file txt che json nella cartella { $name }.
search_not_choosing_any_music = ERRORE: Devi selezionare almeno una casella dei metodi di ricerca musicali.
search_not_choosing_any_broken_files = ERRORE: è necessario selezionare almeno una casella di controllo selezionando il tipo di file danneggiati.
include_folders_dialog_title = Cartelle incluse
exclude_folders_dialog_title = Cartelle escluse
include_manually_directories_dialog_title = Aggiungi cartella manualmente
cache_properly_cleared = Cache cancellata con successo
cache_clear_duplicates_title = Cancellazione cache dei duplicati
cache_clear_similar_images_title = Cancellazione cache delle immagini simili
cache_clear_similar_videos_title = Cancellazione cache dei video simili
cache_clear_message_label_1 = Vuoi cancellare la cache delle voci obsolete?
cache_clear_message_label_2 = Questa operazione rimuoverà tutte le voci della cache che puntano a file non validi.
cache_clear_message_label_3 = Questo può velocizzare il carico/salvataggio nella cache.
cache_clear_message_label_4 = ATTENZIONE: L'operazione rimuoverà tutti i dati memorizzati nella cache da unità esterne scollegate. Quindi ogni hash dovrà essere rigenerato.
# Show preview
preview_image_resize_failure = Ridimensionamento dell'immagine { $name } non riuscito.
preview_image_opening_failure = Impossibile aprire l'immagine { $name }, motivo { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Gruppo { $current_group }/{ $all_groups } ({ $images_in_group } immagini)
compare_move_left_button = L
compare_move_right_button = R
