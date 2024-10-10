# Window titles
window_settings_title = Setări
window_main_title = Czkawka (Hiccup)
window_progress_title = Scanare
window_compare_images = Compară imaginile
# General
general_ok_button = Ok
general_close_button = Inchide
# Main window
music_title_checkbox = Titlu
music_artist_checkbox = Artist
music_year_checkbox = An
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Gen
music_length_checkbox = Lungime
music_comparison_checkbox = Comparație aproximativă
music_checking_by_tags = Etichete
music_checking_by_content = Conținut
same_music_seconds_label = Fragment minim a doua durată
same_music_similarity_label = Diferența maximă
music_compare_only_in_title_group = Compară numai în titlu
music_compare_only_in_title_group_tooltip =
    Când este activat, fișierele sunt grupate după titlu și apoi comparate între ele.
    
    Cu 10000 de fişiere, în schimb aproape 100 de milioane de comparaţii vor fi de obicei aproximativ 20000 de comparaţii.
same_music_tooltip =
    Căutarea fişierelor muzicale similare după conţinutul său poate fi configurată prin setarea:
    
    - Timpul minim de fragment după care fişierele muzicale pot fi identificate ca fiind similare
    - Diferenţa maximă între două fragmente testate
    
    Cheia pentru rezultate bune este de a găsi combinaţii rezonabile ale acestor parametri, pentru furnizare.
    
    Setarea timpului minim la 5 s și diferența maximă la 1.0, va căuta fragmente aproape identice în fișiere.
    O perioadă de 20 de ani și o diferență maximă de 6,0, pe de altă parte, funcționează bine pentru a găsi remixuri/versiuni live etc.
    
    În mod implicit, fiecare fișier muzical este comparat unul cu altul și acest lucru poate dura mult timp când testezi mai multe fișiere, astfel încât este de obicei mai bine să se utilizeze dosare de referință și să se precizeze care fișiere trebuie comparate între ele (cu același volum de fișiere; compararea amprentelor digitale va fi mai rapidă de cel puțin 4x decât fără dosare de referință).
music_comparison_checkbox_tooltip =
    Caută fișiere muzicale similare folosind AI, care folosește învățarea mașinăriei pentru a elimina paranteze dintr-o frază. De exemplu, cu această opțiune activată, fișierele în cauză vor fi considerate duplicate:
    
    Remix Lato 2021)
duplicate_case_sensitive_name = Sensibil la caz
duplicate_case_sensitive_name_tooltip =
    Când este activată, grupul înregistrează doar atunci când are exact același nume, de ex. Trunchiul <-> Z<unk> ołd
    
    Dezactivarea acestei opțiuni va grupa numele fără a verifica dacă fiecare literă are aceeași mărime, de ex. z<unk> oŁD <-> Z<unk> ołd
duplicate_mode_size_name_combo_box = Dimensiune și nume
duplicate_mode_name_combo_box = Nume
duplicate_mode_size_combo_box = Dimensiune
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka oferă 3 tipuri de hash-uri:
    
    Blake3 - funcţie criptografică hash. Acesta este implicit pentru că este foarte rapid.
    
    CRC32 - funcţia simplă de hash. Acest lucru ar trebui să fie mai rapid decât Blake3, dar foarte rar poate avea unele coliziuni.
    
    XXH3 - foarte asemănător din punct de vedere al performanței și al calității hash-ului cu Blake3 (dar non-criptografic). Astfel de moduri pot fi ușor interschimbate.
duplicate_check_method_tooltip =
    Deocamdată, Czkawka oferă trei tipuri de metode pentru a găsi duplicate după:
    
    Nume - Găseşte fişiere care au acelaşi nume.
    
    Dimensiune - Găseşte fişiere cu aceeaşi dimensiune.
    
    Hash - Găseşte fişiere care au acelaşi conţinut. Acest mod hashează fişierul şi mai târziu compară acest hash pentru a găsi duplicate. Acest mod este cel mai sigur mod de a găsi duplicate. Aplicaţiile folosesc foarte mult cache, astfel încât scanările de la secundă şi mai departe ale aceloraşi date ar trebui să fie mult mai rapide decât primul.
image_hash_size_tooltip =
    Fiecare imagine verificată produce un hash special, care poate fi comparat între ele, si o diferenta mica intre ele inseamna ca aceste imagini sunt similare.
    
    dimensiunea de 8 hash este destul de bună pentru a găsi imagini care sunt doar puţin similare cu originalul. Cu un set mai mare de imagini (>1000), acesta va produce o cantitate mare de fals pozitiv, Aşa că vă recomand să utilizaţi o mărime mai mare de hash în acest caz.
    
    16 este dimensiunea implicită a hash-ului care este un compromis destul de bun între a găsi chiar și imagini similare și a avea doar o mică coliziune a hash-ului.
    
    32 și 64 de hash-uri găsesc doar imagini foarte similare, dar nu ar trebui să aibă aproape nicio poziție falsă (poate cu excepția unor imagini cu un canal alfa).
image_resize_filter_tooltip =
    Pentru a calcula hash of imagine, biblioteca trebuie mai întâi să o redimensioneze.
    
    În funcție de algoritmul ales, imaginea rezultată folosită pentru a calcula hash va arăta puțin diferit.
    
    Cel mai rapid algoritm de utilizat, dar şi cel care dă cele mai slabe rezultate, este Nearest. Acesta este activat în mod implicit, deoarece cu dimensiunea de 16x16 a hash-ului este de calitate mai mică decât cea vizibilă.
    
    Cu dimensiunea hash de 8x8 este recomandat să se folosească un algoritm diferit de Nearest, pentru a avea grupuri mai bune de imagini.
image_hash_alg_tooltip =
    Utilizatorii pot alege unul dintre multele algoritmi de calculare a hash-ului.
    
    Fiecare are atât puncte puternice, cât şi puncte mai slabe şi va da uneori rezultate mai bune şi uneori mai proaste pentru imagini diferite.
    
    Deci, pentru a determina cel mai bun dintre voi, este necesară testarea manuală.
big_files_mode_combobox_tooltip = Permite căutarea celor mai mici/mai mari fişiere
big_files_mode_label = Fișiere verificate
big_files_mode_smallest_combo_box = Cel mai mic
big_files_mode_biggest_combo_box = Miggest
main_notebook_duplicates = Fișiere duplicate
main_notebook_empty_directories = Dosare goale
main_notebook_big_files = Fișiere mari
main_notebook_empty_files = Fișiere goale
main_notebook_temporary = Fișiere temporare
main_notebook_similar_images = Imagini similare
main_notebook_similar_videos = Video similare
main_notebook_same_music = Duplicate Muzică
main_notebook_symlinks = Simboluri invalide
main_notebook_broken_files = Fișiere defecte
main_notebook_bad_extensions = Extensii rele
main_tree_view_column_file_name = Numele fișierului
main_tree_view_column_folder_name = Nume folder
main_tree_view_column_path = Cale
main_tree_view_column_modification = Data modificării
main_tree_view_column_size = Dimensiune
main_tree_view_column_similarity = Similaritate
main_tree_view_column_dimensions = Dimensiuni
main_tree_view_column_title = Titlu
main_tree_view_column_artist = Artist
main_tree_view_column_year = An
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Lungime
main_tree_view_column_genre = Gen
main_tree_view_column_symlink_file_name = Numele fișierului Symlink
main_tree_view_column_symlink_folder = Dosar Symlink
main_tree_view_column_destination_path = Calea destinației
main_tree_view_column_type_of_error = Tip de eroare
main_tree_view_column_current_extension = Extensia curentă
main_tree_view_column_proper_extensions = Extensie corectă
main_label_check_method = Metoda de verificare
main_label_hash_type = Tip hash
main_label_hash_size = Dimensiune hash
main_label_size_bytes = Dimensiune (octeți)
main_label_min_size = Minim
main_label_max_size = Maxim
main_label_shown_files = Numărul de fișiere afișate
main_label_resize_algorithm = Redimensionare algoritm
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Arhivează
main_check_box_broken_files_image = Imagine
check_button_general_same_size = Ignoră aceeași dimensiune
check_button_general_same_size_tooltip = Ignoră fișierele cu rezultate de dimensiune identică - de obicei, acestea sunt de 1:1 duplicate
main_label_size_bytes_tooltip = Dimensiunea fişierelor care vor fi utilizate în scanare
# Upper window
upper_tree_view_included_folder_column_title = Dosare de căutat
upper_tree_view_included_reference_column_title = Dosare de referință
upper_recursive_button = Recursiv
upper_recursive_button_tooltip = Dacă este selectat, caută și fișiere care nu sunt plasate direct în dosarele alese.
upper_manual_add_included_button = Adăugare manuală
upper_add_included_button = Adăugare
upper_remove_included_button = Elimină
upper_manual_add_excluded_button = Adăugare manuală
upper_add_excluded_button = Adăugare
upper_remove_excluded_button = Elimină
upper_manual_add_included_button_tooltip =
    Adăugați numele directorului pentru a căuta manual.
    
    Pentru a adăuga căi multiple simultan, separați-le de ;
    
    /home/roman;/home/rozkaz va adăuga două directoare /home/roman și /home/rozkaz
upper_add_included_button_tooltip = Adăugați un nou director pentru căutare.
upper_remove_included_button_tooltip = Ștergeți directorul de căutare.
upper_manual_add_excluded_button_tooltip =
    Adaugă numele folderului exclus manual.
    
    Pentru a adăuga căi multiple simultan, separați-le de ;
    
    /home/roman;/home/krokiet va adăuga două directoare /home/roman și /home/keokiet
upper_add_excluded_button_tooltip = Adauga directorul pentru a fi exclus in cautare.
upper_remove_excluded_button_tooltip = Ştergeţi directorul din excludere.
upper_notebook_items_configuration = Configurare articole
upper_notebook_excluded_directories = Dosare excluse
upper_notebook_included_directories = Dosare incluse
upper_allowed_extensions_tooltip =
    Extensiile permise trebuie separate prin virgulă (implicit toate sunt disponibile).
    
    Următoarele macro care adaugă simultan extensii multiple sunt de asemenea disponibile: IMAGE, VIDEO, MUSIC, TEXT.
    
    Foloseste exemplul ".exe, IMAGE, VIDEO, .rar, 7z" - asta inseamna ca imaginile (e. . fișiere jpg, png), videoclipuri (de ex. avi, mp4), exe, rar și 7z vor fi scanate.
upper_excluded_extensions_tooltip =
    Lista fişierelor dezactivate care vor fi ignorate în scanare.
    
    La utilizarea extensiilor permise și dezactivate, aceasta are prioritate mai mare, deci fișierul nu va fi verificat.
upper_excluded_items_tooltip =
    Elementele excluse trebuie sa contina * wildcard si sa fie separate prin virgule.
    Acest lucru este mai lent decat directoarele excluse, asa ca il folositi cu atentie.
upper_excluded_items = Elemente excluse:
upper_allowed_extensions = Extensii permise:
upper_excluded_extensions = Extensii dezactivate:
# Popovers
popover_select_all = Selectează tot
popover_unselect_all = Deselectează tot
popover_reverse = Selectare inversă
popover_select_all_except_oldest = Selectează toate cu excepția celor mai vechi
popover_select_all_except_newest = Selectează toate cu excepția celor noi
popover_select_one_oldest = Selectează unul mai vechi
popover_select_one_newest = Selectaţi unul dintre cele mai noi
popover_select_custom = Selectare particularizată
popover_unselect_custom = Deselectare particularizată
popover_select_all_images_except_biggest = Selectează toate cu excepția celui mai mare
popover_select_all_images_except_smallest = Selectează toate cu excepția celor mici
popover_custom_path_check_button_entry_tooltip =
    Selectaţi înregistrările după cale.
    
    Exemplu de utilizare:
    /home/pimpek/rzecz.txt poate fi găsit cu /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Selectaţi înregistrările cu numele fişierelor.
    
    Exemplu de utilizare:
    /usr/ping/pong.txt poate fi găsit cu *ong*
popover_custom_regex_check_button_entry_tooltip =
    Selectaţi înregistrările specificate de Regex.
    
    Cu acest mod, textul căutat este calea cu numele.
    
    Exemplu de utilizare:
    /usr/bin/ziemniak. xt poate fi găsit cu /ziem[a-z]+
    
    Acest lucru folosește implementările implicite Rust regex. Puteți citi mai multe despre ele aici: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Activează detectarea cazurilor sensibile.
    
    Când este dezactivat /home/* găsește atât /HoMe/roman cât și /home/roman.
popover_custom_not_all_check_button_tooltip =
    Previne selectarea tuturor înregistrărilor din grup.
    
    Aceasta este activată în mod implicit, deoarece în majoritatea situațiilor, nu doriţi să ştergeţi atât fişierele originale, cât şi duplicate, dar doriţi să lăsaţi cel puţin un fişier.
    
    ATENŢIE: Această setare nu funcţionează dacă aţi selectat deja manual toate rezultatele într-un grup.
popover_custom_regex_path_label = Cale
popover_custom_regex_name_label = Nume
popover_custom_regex_regex_label = Cale Regex + Nume
popover_custom_case_sensitive_check_button = Sensibil la caz
popover_custom_all_in_group_label = Nu selectaţi toate înregistrările în grup
popover_custom_mode_unselect = Deselectare particularizată
popover_custom_mode_select = Selectare particularizată
popover_sort_file_name = Nume fișier
popover_sort_folder_name = Nume dosar
popover_sort_full_name = Numele complet
popover_sort_size = Dimensiune
popover_sort_selection = Selecţie
popover_invalid_regex = Regex nu este valid
popover_valid_regex = Regex este valid
# Bottom buttons
bottom_search_button = Caută
bottom_select_button = Selectare
bottom_delete_button = Ștergere
bottom_save_button = Salvează
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Mutare
bottom_sort_button = Sortează
bottom_search_button_tooltip = Începe căutarea
bottom_select_button_tooltip = Selectaţi înregistrările. Numai fişierele/dosarele selectate pot fi procesate ulterior.
bottom_delete_button_tooltip = Ştergeţi fişierele/dosarele selectate.
bottom_save_button_tooltip = Salvează datele despre căutare în fișier
bottom_symlink_button_tooltip =
    Creaţi link-uri simbolice.
    Funcţionează numai atunci când cel puţin două rezultate într-un grup sunt selectate.
    Prima este neschimbată, iar a doua și mai târziu simpatizează cu primul.
bottom_hardlink_button_tooltip =
    Creează link-uri hardware.
    Funcţionează numai atunci când cel puţin două rezultate sunt selectate într-un grup.
    Prima este neschimbată, iar a doua și mai târziu sunt greu legate mai întâi.
bottom_hardlink_button_not_available_tooltip =
    Creează link-uri hardware.
    Butonul este dezactivat, deoarece hardlink-urile nu pot fi create.
    Legăturile fizice funcționează doar cu privilegii de administrator pe Windows, așa că asigură-te că rulezi aplicația ca administrator.
    Dacă aplicația funcționează deja cu astfel de privilegii verificați pentru probleme similare pe Giwhere,
bottom_move_button_tooltip =
    Mută fișierele în directorul ales.
    Copiază toate fișierele în director fără a păstra directorul arborescent.
    Când se încearcă mutarea a două fișiere cu nume identic în folder, al doilea va eșua și va afișa eroarea.
bottom_sort_button_tooltip = Sortează fișierele/dosarele în funcție de metoda selectată.
bottom_show_errors_tooltip = Arată/ascunde panoul de text de jos.
bottom_show_upper_notebook_tooltip = Arată/Ascunde panoul de notebook-uri de sus.
# Progress Window
progress_stop_button = Oprește
progress_stop_additional_message = Oprire solicitată
# About Window
about_repository_button_tooltip = Link către pagina de depozit cu codul sursă.
about_donation_button_tooltip = Link la pagina de donare.
about_instruction_button_tooltip = Link către pagina de instrucțiuni.
about_translation_button_tooltip = Link catre pagina Crowdin cu traducerea aplicatiilor. Oficial, limba poloneza si engleza sunt suportate.
about_repository_button = Depozit
about_donation_button = Donație
about_instruction_button = Instrucțiuni
about_translation_button = Traducere
# Header
header_setting_button_tooltip = Deschide dialogul de setări.
header_about_button_tooltip = Deschide dialogul cu informații despre aplicație.

# Settings


## General

settings_number_of_threads = Numar discutii folosite
settings_number_of_threads_tooltip = Numărul de teme folosite, 0 înseamnă că vor fi folosite toate temele disponibile.
settings_use_rust_preview = Folosește în schimb gtk librării externe pentru a încărca previzualizările
settings_use_rust_preview_tooltip =
    Utilizarea de previzualizări gtk va fi uneori mai rapidă și va suporta mai multe formate, dar uneori aceasta ar putea fi exact opusul.
    
    Dacă aveţi probleme cu încărcarea previzualizărilor, puteţi încerca să schimbaţi această setare.
    
    Pe sistemele non-linux, se recomandă folosirea acestei optiuni, pentru că gtk-pixbuf nu sunt întotdeauna disponibile, astfel încât dezactivarea acestei opțiuni nu va încărca previzualizarea unor imagini.
settings_label_restart = Trebuie să reporniți aplicația pentru a aplica setările!
settings_ignore_other_filesystems = Ignorați alte sisteme de fișiere (doar Linux)
settings_ignore_other_filesystems_tooltip =
    ignoră fişierele care nu se află în acelaşi sistem de fişiere ca şi directoarele căutate.
    
    Funcţionează la fel ca opţiunea -xdev în găsirea comenzii în Linux
settings_save_at_exit_button_tooltip = Salvați configurația în fișier la închiderea aplicației.
settings_load_at_start_button_tooltip =
    Încarcă configurația din fișier la deschiderea aplicației.
    
    Dacă nu este activată, se vor folosi setările implicite.
settings_confirm_deletion_button_tooltip = Afișați caseta de confirmare când faceți clic pe butonul de ștergere.
settings_confirm_link_button_tooltip = Afișați caseta de confirmare când faceți clic pe butonul hard/symlink.
settings_confirm_group_deletion_button_tooltip = Arată dialogul de avertizare când se încearcă ștergerea tuturor înregistrărilor din grup.
settings_show_text_view_button_tooltip = Arată panoul de text în partea de jos a interfeței utilizatorului.
settings_use_cache_button_tooltip = Foloseşte cache-ul fişierelor.
settings_save_also_as_json_button_tooltip = Salvează cache-ul în formatul JSON (citibil uman). Este posibil să îi modifici conținutul. Geocutia din acest fişier va fi citită automat de aplicaţie dacă nu există geocutie în format binar (cu extensie bin)
settings_use_trash_button_tooltip = Mută fișierele la gunoi în loc să le ștergi definitiv.
settings_language_label_tooltip = Limba interfeței utilizatorului.
settings_save_at_exit_button = Salvați configurația la închiderea aplicației
settings_load_at_start_button = Încarcă configurația la deschiderea aplicației
settings_confirm_deletion_button = Arată dialog de confirmare la ștergerea oricăror fișiere
settings_confirm_link_button = Arată dialog de confirmare atunci când fişierele hard/symlink
settings_confirm_group_deletion_button = Arată dialog de confirmare la ștergerea tuturor fișierelor din grup
settings_show_text_view_button = Arată panoul de text jos
settings_use_cache_button = Utilizare geocutie
settings_save_also_as_json_button = De asemenea, salvează cache-ul ca fișier JSON
settings_use_trash_button = Mută fișierele șterse în gunoi
settings_language_label = Limba
settings_multiple_delete_outdated_cache_checkbutton = Şterge automat intrările învechite
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Ştergeţi rezultatele învechite ale geocutiei care indică fişierele inexistente.
    
    Atunci când este activată, aplicația se asigură la încărcarea înregistrărilor, că toate înregistrările indică către fișiere valide (cele decongelate sunt ignorate).
    
    Dezactivarea acestui lucru va ajuta la scanarea fişierelor pe unităţi externe, astfel încât intrările de cache despre acestea nu vor fi şterse în următoarea scanare.
    
    În cazul în care există sute de mii de înregistrări în cache; se sugerează să activezi acest lucru, care va încărca/salva cache-ul la start/end al scanării.
settings_notebook_general = Generalități
settings_notebook_duplicates = Duplicate
settings_notebook_images = Imagini similare
settings_notebook_videos = Video similar

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Afișează previzualizarea în partea dreaptă (când se selectează un fișier imagine).
settings_multiple_image_preview_checkbutton = Arată previzualizarea imaginii
settings_multiple_clear_cache_button_tooltip =
    Curăță manual cache-ul intrărilor învechite.
    Acest lucru ar trebui folosit doar dacă curățarea automată a fost dezactivată.
settings_multiple_clear_cache_button = Elimină rezultatele învechite din geocutie.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Ascunde toate fişierele, cu excepţia unuia, dacă toate arată spre aceleaşi date (sunt conectate).
    
    Exemplu: În cazul în care sunt (pe disc) şapte fişiere care sunt greu legate de date specifice şi un fişier diferit cu aceleaşi date, dar un alt inventar, apoi în duplicat va fi afișat un singur fișier unic și un fișier de la cele hardlink-ului.
settings_duplicates_minimal_size_entry_tooltip =
    Setaţi dimensiunea minimă a fişierului care va fi memorată în cache.
    
    Alegerea unei valori mai mici va genera mai multe înregistrări. (Automatic Translation) Aceasta va accelera căutarea, dar va încetini încărcarea/salvarea cache-ului.
settings_duplicates_prehash_checkbutton_tooltip =
    Permite stocarea în cache a prehash (un hash calculat dintr-o mică parte a fișierului) care permite concedierea mai timpurie a rezultatelor nereplicate.
    
    este dezactivat implicit deoarece poate cauza încetiniri în unele situații.
    
    Este foarte recomandat sa il utilizezi cand scanezi sute de mii sau milioane de fisiere, pentru ca poate accelera cautarea de mai multe ori.
settings_duplicates_prehash_minimal_entry_tooltip = Dimensiunea minimă a intrării în cache
settings_duplicates_hide_hard_link_button = Ascunde link-urile hard (doar Linux și macOS)
settings_duplicates_prehash_checkbutton = Foloseste cache-ul prehash
settings_duplicates_minimal_size_cache_label = Dimensiunea minimă a fişierelor (în octeţi) salvate în cache
settings_duplicates_minimal_size_cache_prehash_label = Dimensiunea minimă a fişierelor (în octeţi) salvate în cache de prehash

## Saving/Loading settings

settings_saving_button_tooltip = Salvați setările curente în fișier.
settings_loading_button_tooltip = Încarcă setările din fișier și înlocuiește configurația curentă cu ele.
settings_reset_button_tooltip = Resetați configurația curentă la cea implicită.
settings_saving_button = Salvează configurația
settings_loading_button = Încarcă configurația
settings_reset_button = Resetare configurație

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Deschide folderul unde sunt stocate fișierele txt cache-ul.
    
    Modificarea fișierelor de cache poate duce la afișarea unor rezultate invalide. Cu toate acestea, modificarea traiectoriei poate salva timpul atunci când mutați un număr mare de fișiere într-o locație diferită.
    
    Puteţi copia aceste fişiere între computere pentru a salva timp la scanarea din nou pentru fişiere (desigur, dacă au o structură de directoare similară).
    
    În caz de probleme cu geocutia, aceste fişiere pot fi şterse. Aplicaţia le va regenera automat.
settings_folder_settings_open_tooltip =
    Deschide folderul unde este stocată configurația Czkawka.
    
    AVERTISMENT: Modificarea manuală a configurației poate rupe fluxul de lucru.
settings_folder_cache_open = Deschide dosarul cache
settings_folder_settings_open = Deschide folderul de setări
# Compute results
compute_stopped_by_user = Căutarea a fost oprită de utilizator
compute_found_duplicates_hash_size = Am găsit { $number_files } duplicate în { $number_groups } grupuri care au luat { $size }
compute_found_duplicates_name = Am găsit { $number_files } duplicate în grupurile { $number_groups }
compute_found_empty_folders = Foldere goale { $number_files }
compute_found_empty_files = Fisiere goale gasite { $number_files }
compute_found_big_files = Fisiere mari gasite { $number_files }
compute_found_temporary_files = Fișiere temporare găsite { $number_files }
compute_found_images = S-au găsit imagini similare { $number_files } în grupurile { $number_groups }
compute_found_videos = S-au găsit videoclipuri similare { $number_files } în grupurile { $number_groups }
compute_found_music = Am găsit { $number_files } fişiere muzicale similare în grupurile { $number_groups }
compute_found_invalid_symlinks = { $number_files } link-uri simbolice invalide găsite
compute_found_broken_files = Fișiere defecte { $number_files } găsite
compute_found_bad_extensions = Fișiere { $number_files } cu extensii invalide
# Progress window
progress_scanning_general_file = Se scanează fişierul { $file_number }
progress_scanning_extension_of_files = Se verifică extensia fișierului { $file_checked }/{ $all_files }
progress_scanning_broken_files = Se verifică fişierul { $file_checked }/{ $all_files }
progress_scanning_video = Hashing of { $file_checked }/{ $all_files } video
progress_scanning_image = Hashing of { $file_checked }/{ $all_files } image
progress_comparing_image_hashes = Comparare hash imagine { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Compararea etichetelor fișierului de muzică { $file_checked }/{ $all_files }
progress_scanning_music_tags = Citirea etichetelor din fişierul de muzică { $file_checked }/{ $all_files }
progress_scanning_music_content_end = Compararea amprentelor fișierului de muzică { $file_checked }/{ $all_files }
progress_scanning_music_content = Calcularea amprentei fişierului de muzică { $file_checked }/{ $all_files }
progress_scanning_empty_folders = Se scanează directorul { $folder_number }
progress_scanning_size = Dimensiune scanare fișier { $file_number }
progress_scanning_size_name = Se scanează numele și dimensiunea fișierului { $file_number }
progress_scanning_name = Se scanează numele fișierului { $file_number }
progress_analyzed_partial_hash = S-a analizat hash-ul parțial al fișierelor { $file_checked }/{ $all_files }
progress_analyzed_full_hash = A fost analizat hash complet al fişierelor { $file_checked }/{ $all_files }
progress_prehash_cache_loading = Se încarcă cache-ul prehash
progress_prehash_cache_saving = Salvare cache prehash
progress_hash_cache_loading = Încărcare cache hash
progress_hash_cache_saving = Salvare cache hash
progress_cache_loading = Se încarcă geocutia
progress_cache_saving = Salvare geocutie
progress_current_stage = Current Stage:{ "  " }
progress_all_stages = All Stages:{ "  " }
# Saving loading 
saving_loading_saving_success = Configurație salvată în fișierul { $name }.
saving_loading_saving_failure = Salvarea datelor de configurare în fișierul { $name } a eșuat.
saving_loading_reset_configuration = Configurația curentă a fost ștearsă.
saving_loading_loading_success = Configurare aplicație încărcată corespunzător.
saving_loading_invalid_string = Pentru cheia "{ $key }" a găsit un rezultat nevalid - "{ $result }" care nu este un șir.
saving_loading_invalid_int = Pentru cheia "{ $key }" a găsit un rezultat invalid - "{ $result }" care nu este un număr întreg.
saving_loading_invalid_bool = Pentru cheia "{ $key }" a găsit rezultat nevalid - "{ $result }" care nu este un bool.
saving_loading_decode_problem_bool = Nu s-a reușit decodificarea boolului de la tasta "{ $key }" găsit "{ $result }" dar valorile permise sunt 0, 1, adevărat sau false.
saving_loading_saving_same_keys = Se încearcă salvarea setării cu cheie duplicată "{ $key }".
saving_loading_failed_to_get_home_directory = Nu s-a putut obține directorul acasă pentru a deschide/salva fișierul de configurare.
saving_loading_folder_config_instead_file = Nu se poate crea sau deschide fișierul de configurare în calea "{ $path }" deoarece există deja un dosar.
saving_loading_failed_to_create_configuration_folder = Nu s-a reuşit configurarea pentru a crea folderul de configurare "{ $path }", motivul "{ $reason }".
saving_loading_failed_to_create_config_file = Nu s-a putut crea fișierul de configurare "{ $path }", motivul "{ $reason }".
saving_loading_failed_to_read_config_file = Nu se poate încărca configurația din "{ $path }" deoarece nu există sau nu este un fișier.
saving_loading_failed_to_read_data_from_file = Datele din fişierul "{ $path }", motivul "{ $reason }".
saving_loading_orphan_data = Am găsit date orfane "{ $data }" în rândul "{ $line }".
saving_loading_not_valid = Setarea "{ $data }" nu există în versiunea curentă a aplicației.
# Invalid symlinks
invalid_symlink_infinite_recursion = Recepţie infinită
invalid_symlink_non_existent_destination = Fișier destinație inexistent
# Other
selected_all_reference_folders = Nu se poate începe căutarea, atunci când toate directoarele sunt setate ca dosare de referință
searching_for_data = Se caută date, poate dura ceva timp, vă rugăm așteptați...
text_view_messages = MESAJE
text_view_warnings = ATENȚIONĂRI
text_view_errors = EROARE
about_window_motto = Acest program este liber de utilizat și va fi întotdeauna.
# Various dialog
dialogs_ask_next_time = Întreabă data viitoare
delete_file_failed = Nu s-a reușit ștergerea fișierului { $name }, motivul { $reason }
delete_title_dialog = Ștergeți confirmarea
delete_question_label = Sunteţi sigur că doriţi să ştergeţi fişierele?
delete_all_files_in_group_title = Confirmarea ștergerii tuturor fișierelor din grup
delete_all_files_in_group_label1 = In unele grupuri, toate inregistrarile sunt selectate.
delete_all_files_in_group_label2 = Sunteţi sigur că doriţi să le ştergeţi?
delete_folder_failed = Nu s-a reușit ștergerea dosarului { $dir } deoarece directorul nu există, nu aveți permisiunea sau folderul nu este gol.
delete_items_label = { $items } fișiere vor fi șterse.
delete_items_groups_label = { $items } fișiere din grupurile { $groups } vor fi șterse.
hardlink_failed = Eșuare la hardlink
hard_sym_invalid_selection_title_dialog = Selecţie invalidă cu unele grupuri
hard_sym_invalid_selection_label_1 = În unele grupuri există doar o înregistrare selectată și va fi ignorată.
hard_sym_invalid_selection_label_2 = Pentru a putea lega hard/sym aceste fișiere, cel puțin două rezultate în grup trebuie să fie selectate.
hard_sym_invalid_selection_label_3 = Prima în grup este recunoscută ca fiind originală şi nu se modifică, dar se modifică a doua şi mai târziu.
hard_sym_link_title_dialog = Confirmare link
hard_sym_link_label = Sunteţi sigur că doriţi să conectaţi aceste fişiere?
move_folder_failed = Nu s-a reușit mutarea dosarului { $name }, motivul { $reason }
move_file_failed = Nu s-a reușit mutarea fișierului { $name }, motivul { $reason }
move_files_title_dialog = Alegeți directorul în care doriți să mutați fișierele duplicate
move_files_choose_more_than_1_path = Poate fi selectată doar o singură cale pentru a putea copia fişierele duplicate, selectate { $path_number }.
move_stats = Elemente corect mutate { $num_files }/{ $all_files }
save_results_to_file = Rezultate salvate atât pentru fişierele txt cât şi pentru fişierele json în directorul { $name }.
search_not_choosing_any_music = EROARE: Trebuie să selectaţi cel puţin o casetă cu tipuri de căutare de muzică.
search_not_choosing_any_broken_files = EROARE: Trebuie să selectaţi cel puţin o casetă de selectare cu tipul de fişiere bifate.
include_folders_dialog_title = Dosare de inclus
exclude_folders_dialog_title = Dosare de exclus
include_manually_directories_dialog_title = Adaugă director manual
cache_properly_cleared = Geocutie golită corect
cache_clear_duplicates_title = Golire duplicate cache
cache_clear_similar_images_title = Curăță cache imagini similare
cache_clear_similar_videos_title = Curățare cache video similar
cache_clear_message_label_1 = Vrei să ştergi memoria cache a intrărilor învechite?
cache_clear_message_label_2 = Această operaţie va elimina toate intrările din cache-ul care indică fişiere invalide.
cache_clear_message_label_3 = Aceasta poate încărca/salva uşor accelerat în cache.
cache_clear_message_label_4 = AVERTISMENT: Operația va elimina toate datele stocate în cache din unplugged external drive. Deci fiecare hash va trebui să fie regenerat.
# Show preview
preview_image_resize_failure = Redimensionarea imaginii { $name } a eșuat.
preview_image_opening_failure = Nu s-a reușit deschiderea imaginii { $name }, motivul { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grup { $current_group }/{ $all_groups } ({ $images_in_group } imagini)
compare_move_left_button = l
compare_move_right_button = R
