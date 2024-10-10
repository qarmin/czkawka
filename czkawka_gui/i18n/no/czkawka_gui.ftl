# Window titles
window_settings_title = Innstillinger
window_main_title = Czkawka (Hiccup)
window_progress_title = Skanner
window_compare_images = Sammenlign bilder
# General
general_ok_button = Ok
general_close_button = Lukk
# Main window
music_title_checkbox = Tittel
music_artist_checkbox = Artist
music_year_checkbox = År
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Sjanger
music_length_checkbox = Lengde
music_comparison_checkbox = Omtrentlig sammenligning
music_checking_by_tags = Tagger
music_checking_by_content = Innhold
same_music_seconds_label = Minste fragment andre varighet
same_music_similarity_label = Maksimal differanse
music_compare_only_in_title_group = Sammenlign bare i tittel
music_compare_only_in_title_group_tooltip =
    Når aktivert, blir filer gruppert etter tittel og sammenlignes med hverandre.
    
    Med 10000 filer, vil det i stedet være nesten 100 millioner sammenligninger som regel være rundt 20000 sammenligninger.
same_music_tooltip =
    Søker etter lignende musikkfiler av innholdet kan konfigureres ved å gå inn:
    
    - Minimumsfragmenteringstiden etter hvilken musikkfiler som kan identifiseres som lignende
    - Maksimal forskjell mellom to testede fragmenter
    
    Nøkkelen til gode resultater er å finne fornuftige kombinasjoner av disse parametrene, for utlevert.
    
    Angir minimum tid til 5 s og maksimal forskjell til 1,0, vil se etter nesten identiske fragmenter i filene.
    En tid på 20 s og en maksimal forskjell på 6.0, for den andre siden fungerer bra for å finne remikser/levende versjoner osv.
    
    Som standard kan hver musikkfil sammenlignes med hverandre, og dette kan ta mye tid når du tester mange filer, slik at det vanligvis er bedre å bruke referanselapper og spesifisere hvilke filer som skal sammenlignes med hverandre (med samme mengde filer, å sammenligne fingeravtrykk vil være raskere minst 4 x enn uten referansemapper).
music_comparison_checkbox_tooltip =
    Den søker etter lignende musikkfiler ved hjelp av AI, som bruker maskiner til å fjerne parenteser fra et frase. For eksempel, med dette alternativet er aktivert. filene du vil bli betraktet som duplikater:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Skill mellom små og store bokstaver
duplicate_case_sensitive_name_tooltip =
    Når aktivert, vil du bare gruppere når de har nøyaktig samme navn, f.eks. Żołd <-> Żołd
    
    Deaktivering av en slik opsjon vil gi deg egne navn uten å sjekke om hver bokstav er like stort, f.eks. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Størrelse og navn
duplicate_mode_name_combo_box = Navn
duplicate_mode_size_combo_box = Størrelse
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Tsjekkia har 3 typer hashes:
    
    Blake3 - kryptografisk hash-funksjon. Dette er standard fordi den er veldig rask.
    
    CRC32 - enkel hash-funksjon. Dette bør være raskere enn Blake3, men kan svært sjelden ha noen kollisjoner.
    
    XXH3 - meget likt i ytelse og hash-kvalitet til Blake3 (men ikke-kryptografisk). Såfremt kan slike moduser endres enkelt.
duplicate_check_method_tooltip =
    Tsjekkka tilbyr tre typer metoder for å finne duplikater av:
    
    Navn - Finner filer med samme navn.
    
    Størrelse på funnet finner du filer med samme størrelse.
    
    Hash - Finner filer med samme innhold. Denne modusen ligner filen og senere sammenligner dette hashen for å finne duplikater. Denne modusen er den sikreste måten å finne duplikater. Appen bruker stort, så sekund og ytterligere skanninger av de samme dataene bør være mye raskere enn det første.
image_hash_size_tooltip =
    Hvert avmerkede bilde gir en spesiell hash som kan sammenlignes med hverandre, og en liten forskjell mellom dem betyr at disse bildene er like.
    
    8 hash-størrelse er ganske bra for å finne bilder som er litt likt original. Med et større sett med bilder (>1000) vil dette gi svært mange falske positive. Derfor anbefaler jeg å bruke en større hash-størrelse i denne saken.
    
    16 er standard hash-størrelse som er et godt kompromiss mellom å finne selv små lignende bilder og å ha bare en liten mengde hash-kollisjoner.
    
    32 og 64 hashes finner bare lignende bilder, men bør ha nesten ingen falske positiver (kanskje unntatt bilder med alfa-kanal).
image_resize_filter_tooltip =
    To compute hash of image, the library must first resize it.
    
    Depend on chosen algorithm, the resulting image used to calculate hash will looks a little different.
    
    The fastest algorithm to use, but also the one which gives the worst results, is Nearest. It is enabled by default, because with 16x16 hash size lower quality it is not really visible.
    
    With 8x8 hash size it is recommended to use a different algorithm than Nearest, to have better groups of images.
image_hash_alg_tooltip =
    Brukere kan velge mellom en av mange algoritmer i beregningen av hashen.
    
    Hver har både sterke og svakere poeng, og vil noen ganger gi bedre og noen ganger verre resultater for ulike bilder.
    
    Så for å bestemme det beste for deg, kreves manuell testing.
big_files_mode_combobox_tooltip = Lar deg søke etter minste/største filer
big_files_mode_label = Avmerkede filer
big_files_mode_smallest_combo_box = Den minste
big_files_mode_biggest_combo_box = Den største
main_notebook_duplicates = Dupliser filer
main_notebook_empty_directories = Tomme mapper
main_notebook_big_files = Store filer
main_notebook_empty_files = Tomme filer
main_notebook_temporary = Midlertidige filer
main_notebook_similar_images = Lignende bilder
main_notebook_similar_videos = Lignende videoer
main_notebook_same_music = Musikk dupliserer
main_notebook_symlinks = Ugyldige Symlinks
main_notebook_broken_files = Ødelagte filer
main_notebook_bad_extensions = Feil utvidelser
main_tree_view_column_file_name = Filnavn
main_tree_view_column_folder_name = Mappenavn
main_tree_view_column_path = Sti
main_tree_view_column_modification = Endret dato
main_tree_view_column_size = Størrelse
main_tree_view_column_similarity = Likhet
main_tree_view_column_dimensions = Dimensjoner
main_tree_view_column_title = Tittel
main_tree_view_column_artist = Artist
main_tree_view_column_year = År
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Lengde
main_tree_view_column_genre = Sjanger
main_tree_view_column_symlink_file_name = Symlink filnavn
main_tree_view_column_symlink_folder = Mappe for Symlink
main_tree_view_column_destination_path = Destinasjonssti
main_tree_view_column_type_of_error = Type feil
main_tree_view_column_current_extension = Gjeldende utvidelse
main_tree_view_column_proper_extensions = Riktig utvidelse
main_label_check_method = Sjekkmetode
main_label_hash_type = Type hash
main_label_hash_size = Størrelse på hash
main_label_size_bytes = Størrelse (bytes)
main_label_min_size = Min
main_label_max_size = Maks
main_label_shown_files = Antall filer som vises
main_label_resize_algorithm = Endre algoritmen
main_label_similarity = Similarity{ " " }
main_check_box_broken_files_audio = Lyd
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Arkiv
main_check_box_broken_files_image = Bilde
check_button_general_same_size = Ignorer samme størrelse
check_button_general_same_size_tooltip = Ignorer filer med identisk størrelse i resultater - vanligvis disse er 1:1 duplikater
main_label_size_bytes_tooltip = Størrelse på filer som vil bli brukt i skanning
# Upper window
upper_tree_view_included_folder_column_title = Mapper å søke etter
upper_tree_view_included_reference_column_title = Referanse mapper
upper_recursive_button = Rekursivt
upper_recursive_button_tooltip = Hvis valgt, søk også etter filer som ikke er plassert direkte under valgte mapper.
upper_manual_add_included_button = Manuelt legg til
upper_add_included_button = Legg til
upper_remove_included_button = Fjern
upper_manual_add_excluded_button = Manuelt legg til
upper_add_excluded_button = Legg til
upper_remove_excluded_button = Fjern
upper_manual_add_included_button_tooltip =
    Legg til mappenavn for å søke etter hånd.
    
    For å legge til flere baner samtidig, separer dem med ;
    
    /home/roman;/home/rozkaz vil legge til to kataloger /home/roman og /home/rozkaz
upper_add_included_button_tooltip = Legg til ny mappe i søk.
upper_remove_included_button_tooltip = Slett mappen fra søk.
upper_manual_add_excluded_button_tooltip =
    Legg til ekskludert mappenavn for hånd.
    
    For å legge til flere baner på en gang, separer dem med ;
    
    /home/roman;/home/krokiet vil legge til to kataloger /home/roman and /home/keokiet
upper_add_excluded_button_tooltip = Legg til mappe som skal utelukkes i søk.
upper_remove_excluded_button_tooltip = Slett mappen fra ekskludert.
upper_notebook_items_configuration = Konfigurasjon av elementer
upper_notebook_excluded_directories = Ekskluderte kataloger
upper_notebook_included_directories = Inkluderte mapper
upper_allowed_extensions_tooltip =
    Tillatte utvidelser må være atskilt med komma (ved at alle er tilgjengelige).
    
    Følgende makroer, som legger til flere utvidelser samtidig, er også tilgjengelige: IMAGE, VIDEO, MUSIC, TEXT.
    
    Bruk eksempel ".exe, IMAGE, VIDEO, .rar, 7z" - dette betyr at bilder (e. . jpg, png), videoer (f.eks. avi, mp4), exe, rar og 7z filer vil bli skannet.
upper_excluded_extensions_tooltip =
    Liste over deaktiverte filer som vil bli ignorert i skanning.
    
    Ved bruk av både tillatte og deaktiverte utvidelser, har denne prioriteten høyere enn prioritet, så filen vil ikke bli sjekket.
upper_excluded_items_tooltip =
    Ekskluderte elementer må inneholde * jokertegn og bør separeres med komma.
    Dette er tregere enn ekskluderte kataloger, så bruk den med forsiktighet.
upper_excluded_items = Ekskluderte elementer:
upper_allowed_extensions = Tillatte utvidelser:
upper_excluded_extensions = Deaktiverte utvidelser:
# Popovers
popover_select_all = Velg alle
popover_unselect_all = Fjern alle valg
popover_reverse = Omvendt utvalg
popover_select_all_except_oldest = Velg alt unntatt det eldste
popover_select_all_except_newest = Velg alle unntatt nyeste
popover_select_one_oldest = Velg en eldste
popover_select_one_newest = Velg en nyeste
popover_select_custom = Velg egendefinert
popover_unselect_custom = Avvelg egendefinert
popover_select_all_images_except_biggest = Velg alle unntatt største
popover_select_all_images_except_smallest = Velg alle unntatt minste
popover_custom_path_check_button_entry_tooltip =
    Velg poster som sti.
    
    Eksempelbruk:
    /home/pimpek/rzecz.txt kan du finne med /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Velg poster etter filnavn.
    
    Eksempelbruk:
    /usr/ping/pong.txt kan finnes med *ong*
popover_custom_regex_check_button_entry_tooltip =
    Velg elementer ved angitt Regex.
    
    Med denne modus, vil søppelteksten være Sti med navn.
    
    Eksempel på bruk:
    /usr/bin/ziemniak. xt finner du med /ziem[a-z]+
    
    Dette bruker standard Rust regex implementasjon. Du kan lese mer om den her: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Aktiverer case-sensitiv deteksjon.
    
    Når deaktivert/hjem/* funn både /HoMe/roman og /home/roman.
popover_custom_not_all_check_button_tooltip =
    Hindrer å velge alle poster i gruppen.
    
    Dette er aktivert som standard, fordi i de fleste situasjoner du ikke vil slette både originale og duplikatfiler, men vil forlate minst en fil.
    
    ADVARSEL: Denne innstillingen fungerer ikke hvis du allerede har valgt alle resultater i en gruppe.
popover_custom_regex_path_label = Sti
popover_custom_regex_name_label = Navn
popover_custom_regex_regex_label = Regex sti + navn
popover_custom_case_sensitive_check_button = Skill store og små bokstaver
popover_custom_all_in_group_label = Ikke velg alle poster i gruppen
popover_custom_mode_unselect = Avvelg egendefinert
popover_custom_mode_select = Velg egendefinert
popover_sort_file_name = Filnavn
popover_sort_folder_name = Mappenavn
popover_sort_full_name = Fullt navn
popover_sort_size = Størrelse
popover_sort_selection = Utvalg
popover_invalid_regex = Regex er ugyldig
popover_valid_regex = Regex er gyldig
# Bottom buttons
bottom_search_button = Søk
bottom_select_button = Velg
bottom_delete_button = Slett
bottom_save_button = Save
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Flytt
bottom_sort_button = Sorter
bottom_search_button_tooltip = Starte søk
bottom_select_button_tooltip = Velg oppføringer. Bare valgte filer/mapper kan bli behandlet senere.
bottom_delete_button_tooltip = Slett valgte filer/mapper.
bottom_save_button_tooltip = Lagre data om søk i fil
bottom_symlink_button_tooltip =
    Opprett symbolske lenker.
    Virker bare når minst to resultater i en gruppe er valgt.
    Først er uendret og sekund og senere er symlinket til først.
bottom_hardlink_button_tooltip =
    Opprette fastkoblinger.
    Virker bare når minst to resultater i en gruppe er valgt.
    Først er uendret og annet og senere er vanskelig knyttet til først.
bottom_hardlink_button_not_available_tooltip =
    Opprett faste koblinger.
    Knappen er deaktivert, fordi faste koblinger ikke kan opprettes.
    Faste koblinger fungerer bare med administratorrettigheter i Windows, så pass på at du kjører programmet som administrator.
    Hvis programmet allerede fungerer med slike privilegier, sjekk om lignende problemer er observert på GitHub.
bottom_move_button_tooltip =
    Flytter filer til valgt mappe.
    Den kopierer alle filer til mappen uten å lagre mappetreet.
    Når du prøver å flytte to filer med identisk navn til mappe, vil det andre feile og vise feil.
bottom_sort_button_tooltip = Sorter filer/mapper etter valgt metode.
bottom_show_errors_tooltip = Vis/Skjul bunntekstpanelet.
bottom_show_upper_notebook_tooltip = Vis/Skjul øvre notebook panel.
# Progress Window
progress_stop_button = Stopp
progress_stop_additional_message = Stopp forespurt
# About Window
about_repository_button_tooltip = Link til pakkesiden med kildekoden.
about_donation_button_tooltip = Lenke til donasjonssiden.
about_instruction_button_tooltip = Lenke til instruksjonssiden.
about_translation_button_tooltip = Link til Crowdin-siden med app oversettelser. Officialt Polsk og engelsk støttes.
about_repository_button = Pakkebrønn
about_donation_button = Donasjon
about_instruction_button = Instruksjon
about_translation_button = Oversettelse
# Header
header_setting_button_tooltip = Åpner dialogboksen for innstillinger.
header_about_button_tooltip = Åpner dialog med info om app.

# Settings


## General

settings_number_of_threads = Antall brukte tråder
settings_number_of_threads_tooltip = Antall brukte tråder. 0 betyr at alle tilgjengelige tråder vil bli brukt.
settings_use_rust_preview = Bruk eksterne biblioteker i stedet gtk for å laste forhåndsvisninger
settings_use_rust_preview_tooltip =
    Med gtk innledning vil noen ganger være raskere og støtte flere formater, men noen ganger kan dette være akkurat det motsatte.
    
    Hvis du har problemer med å laste forhåndsvisninger, kan du prøve å endre denne innstillingen.
    
    på ikke-linux-systemer anbefales det å bruke dette alternativet, fordi gtk-pixbuf ikke alltid finnes, slik at deaktivering av dette alternativet ikke vil laste forhåndsvisninger på noen bilder.
settings_label_restart = Start programmet på nytt for å bruke innstillingene!
settings_ignore_other_filesystems = Ignorer andre filsystemer (bare Linux)
settings_ignore_other_filesystems_tooltip =
    ignorerer filer som ikke er i samme filsystem som søk-kataloger.
    
    Fungerer samme som -xdev alternativet i å finne kommandoen på Linux
settings_save_at_exit_button_tooltip = Lagre konfigurasjon som fil når appen lukkes.
settings_load_at_start_button_tooltip =
    Last inn konfigurasjon fra filen når du åpner appen.
    
    Hvis ikke er aktivert brukes standard innstillinger.
settings_confirm_deletion_button_tooltip = Vis bekreftelsesdialog når du klikker på Slette-knappen.
settings_confirm_link_button_tooltip = Vis bekreftelsesdialog når du klikker på knappen hard/symlink.
settings_confirm_group_deletion_button_tooltip = Vis advarselsdialog når du prøver å slette alle poster fra gruppen.
settings_show_text_view_button_tooltip = Vis tekstpanelet nederst av brukergrensesnittet.
settings_use_cache_button_tooltip = Bruk filmellomlager.
settings_save_also_as_json_button_tooltip = Lagre cache til (human lesbar) JSON-format. Det er mulig å endre innholdet. Cachen fra denne filen vil automatisk bli lest av app dersom binært format mellomlager (med «bøi-utvidelsen») mangler.
settings_use_trash_button_tooltip = Flytter filer til papirkurv istedenfor å slette dem permanent.
settings_language_label_tooltip = Språk til brukergrensesnitt.
settings_save_at_exit_button = Lagre konfigurasjon når appen lukkes
settings_load_at_start_button = Last inn konfigurasjon når du åpner appen
settings_confirm_deletion_button = Vis bekreftelsesdialog ved sletting av filer
settings_confirm_link_button = Vis bekreftelsesdialog når noen filer er fast/symlinker
settings_confirm_group_deletion_button = Vis "Bekreft"-dialog når du sletter alle filer i gruppen
settings_show_text_view_button = Vis nederste tekstpanel
settings_use_cache_button = Bruk buffer
settings_save_also_as_json_button = Lagre også mellomlager som JSON-fil
settings_use_trash_button = Flytt slettede filer til papirkurv
settings_language_label = Language
settings_multiple_delete_outdated_cache_checkbutton = Slett utdaterte cache-oppføringer automatisk
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Slett utdaterte cache-resultater som peker til ikke-eksisterende filer.
    
    Når aktivert sørger appen for å laste inn poster, at alle oppføringer peker til gyldige filer (ødelagte blir ignorert).
    
    Deaktivering av dette vil hjelpe når du skanner filer på eksterne stasjoner, så cacheoppføringer om dem vil ikke bli tømt i neste skanning.
    
    Når det gjelder å ha hundre og tusenvis av registreringer i cache, det er foreslått å aktivere dette, som vil øke hurtigbufferen innlasting/lagring ved start/slutten av skanningen.
settings_notebook_general = Generelt
settings_notebook_duplicates = Duplikater
settings_notebook_images = Lignende bilder
settings_notebook_videos = Lignende video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Viser forhåndsvisning på høyre side (når du velger en bildefil).
settings_multiple_image_preview_checkbutton = Vis forhåndsvisning av bilde
settings_multiple_clear_cache_button_tooltip =
    Manuelt tømmer hurtigbufferen av utdaterte oppføringer.
    Dette bør bare brukes hvis automatisk tømming er deaktivert.
settings_multiple_clear_cache_button = Fjern utdaterte resultater fra mellomlager.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Skjuler alle filer unntatt en, hvis alle peker til samme data (knyttes sammen).
    
    Eksempel: I det tilfellet hvor det (på disk) er syv filer som er vanskelige å koble til bestemte data og én annen fil med samme data men en annen innhold, deretter i duplisert finer, vises bare én unik fil og én fil fra hardkoblede vil vises.
settings_duplicates_minimal_size_entry_tooltip =
    Angi minste filstørrelse som blir bufret i cachen.
    
    Hvis du velger en mindre verdi, genererer du flere poster. Dette vil fremskynde søk, men tregere hurtigbufferet lasting/lagring.
settings_duplicates_prehash_checkbutton_tooltip =
    Aktiverer hurtigbufring av prehash (en hash beregnet fra en liten del av filen) som tillater tidligere fjerning av ikke-dupliserte resultater.
    
    Den er deaktivert som standard fordi den kan forårsake langsommere i noen situasjoner.
    
    Det anbefales å bruke det når det skannes hundre eller millioner filer, fordi det kan fremskynde søk med flere ganger.
settings_duplicates_prehash_minimal_entry_tooltip = Minimal størrelse på mellomlagret oppføring.
settings_duplicates_hide_hard_link_button = Skjul hardlinker (kun Linux og macOS)
settings_duplicates_prehash_checkbutton = Bruk prehash cache
settings_duplicates_minimal_size_cache_label = Minimal størrelse på filer (i byte) lagret i mellomlager
settings_duplicates_minimal_size_cache_prehash_label = Minimal størrelse på filer (i byte) lagret i hurtigbuffer for prehash

## Saving/Loading settings

settings_saving_button_tooltip = Lagre gjeldende innstillingskonfigurasjon til filen.
settings_loading_button_tooltip = Last innstillinger fra fil og erstatt gjeldende konfigurasjon med dem.
settings_reset_button_tooltip = Tilbakestill den gjeldende konfigurasjonen til standard.
settings_saving_button = Lagre konfigurasjonen
settings_loading_button = Last inn konfigurasjon
settings_reset_button = Tilbakestill konfigurasjon

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Åpner mappen der cache txt filene er lagret.
    
    Modifisere cachefilene kan forårsake ugyldige resultater. Imidlertid kan modifisere stien spare tid når du flytter store mengder filer til en annen posisjon.
    
    Du kan kopiere disse filene mellom datamaskiner for å spare tid ved skanning igjen for filer (hvis de har en lignende katalogstruktur).
    
    Hvis det oppstår problemer med cachen, kan disse filene fjernes. Appen vil automatisk regenerere dem.
settings_folder_settings_open_tooltip =
    Åpner mappen der Czkawka konfigurasjonen er lagret.
    
    ADVARSEL: Manuelt endre konfigurasjonen kan ødelegge arbeidsflyten din.
settings_folder_cache_open = Åpne mappe for hurtigbuffer
settings_folder_settings_open = Åpne innstillingsmappen
# Compute results
compute_stopped_by_user = Søket ble stoppet av bruker
compute_found_duplicates_hash_size = Fant { $number_files } duplikater i { $number_groups } grupper som tok { $size }
compute_found_duplicates_name = Fant { $number_files } duplikater i { $number_groups } grupper
compute_found_empty_folders = Fant { $number_files } tomme mapper
compute_found_empty_files = Fant { $number_files } tomme filer
compute_found_big_files = Fant { $number_files } store filer
compute_found_temporary_files = Fant { $number_files } midlertidige filer
compute_found_images = Fant { $number_files } lignende bilder i { $number_groups } grupper
compute_found_videos = Fant { $number_files } lignende videoer i { $number_groups } grupper
compute_found_music = Fant { $number_files } lignende musikkfiler i { $number_groups } grupper
compute_found_invalid_symlinks = Fant { $number_files } ugyldige symlinker
compute_found_broken_files = Fant { $number_files } ødelagte filer
compute_found_bad_extensions = Fant { $number_files } filer med ugyldige utvidelser
# Progress window
progress_scanning_general_file = Skanner { $file_number } fil
progress_scanning_extension_of_files = Sjekker utvidelse av { $file_checked }/{ $all_files } fil
progress_scanning_broken_files = Kontrollerer { $file_checked }/{ $all_files } fil
progress_scanning_video = Hashing av { $file_checked }/{ $all_files } video
progress_scanning_image = Hashing av { $file_checked }/{ $all_files } bilde
progress_comparing_image_hashes = Sammenligner { $file_checked }/{ $all_files } bilde-hash
progress_scanning_music_tags_end = Sammenligner tagger med { $file_checked }/{ $all_files } musikkfil
progress_scanning_music_tags = Leser tagger på { $file_checked }/{ $all_files } musikkfil
progress_scanning_music_content_end = Sammenligner fingeravtrykk på { $file_checked }/{ $all_files } musikkfil
progress_scanning_music_content = Beregner fingeravtrykk på { $file_checked }/{ $all_files } musikkfil
progress_scanning_empty_folders = Skanner { $folder_number } mappe
progress_scanning_size = Skanner størrelse på { $file_number } fil
progress_scanning_size_name = Skanning av navn og størrelse på { $file_number } fil
progress_scanning_name = Skanning av navn på { $file_number } fil
progress_analyzed_partial_hash = Analyserte delvis hash med { $file_checked }/{ $all_files } filer
progress_analyzed_full_hash = Analyserte full hash med { $file_checked }/{ $all_files } filer
progress_prehash_cache_loading = Laster prehash cache
progress_prehash_cache_saving = Lagrer prehash-cache
progress_hash_cache_loading = Laster hash-cache
progress_hash_cache_saving = Lagrer hurtigbufferen for hash
progress_cache_loading = Laster cache
progress_cache_saving = Lagrer cachen
progress_current_stage = Gjeldende trinn: { " " }
progress_all_stages = Alle stadier:{ " " }
# Saving loading 
saving_loading_saving_success = Lagret konfigurasjon til filen { $name }.
saving_loading_saving_failure = Klarte ikke å lagre konfigurasjonsdataene til filen { $name }.
saving_loading_reset_configuration = Gjeldende konfigurasjon ble fjernet.
saving_loading_loading_success = Godt lastet applikasjonskonfigurasjon.
saving_loading_invalid_string = For nøkkelen "{ $key }" fant du ugyldig resultat - "{ $result }" som ikke er en streng.
saving_loading_invalid_int = I nøkkelen "{ $key }" fant du ugyldig resultat - "{ $result }" som ikke er et heltall.
saving_loading_invalid_bool = For nøkkelen "{ $key }" fant ugyldig resultat - "{ $result }" som ikke er en bool.
saving_loading_decode_problem_bool = Kan ikke dekode bool fra nøkkelen "{ $key }" funnet "{ $result }", men tillatte verdier er 0, 1, sanne eller usann.
saving_loading_saving_same_keys = Prøver å lagre innstillinger med duplisert nøkkel "{ $key }".
saving_loading_failed_to_get_home_directory = Klarte ikke å hente hjemmappen for å åpne og lagre konfigurasjonsfilen.
saving_loading_folder_config_instead_file = Kan ikke opprette eller åpne lagringsfilbanen{ $path }" fordi allerede finnes en mappe.
saving_loading_failed_to_create_configuration_folder = Kunne ikke konfigurere opprette konfigurasjonsmappen "{ $path }", grunn "{ $reason }".
saving_loading_failed_to_create_config_file = Kunne ikke opprette konfigurasjonsfilen{ $path }", grunn "{ $reason }".
saving_loading_failed_to_read_config_file = Kan ikke laste konfigurasjonen fra "{ $path }" fordi den ikke eksisterer eller ikke er en fil.
saving_loading_failed_to_read_data_from_file = Kan ikke lese data fra filen{ $path }", grunn "{ $reason }".
saving_loading_orphan_data = Funnet orphan data "{ $data }" i linje "{ $line }".
saving_loading_not_valid = Setter "{ $data }" finnes ikke i gjeldende appversjon.
# Invalid symlinks
invalid_symlink_infinite_recursion = Uendelig rekursjon
invalid_symlink_non_existent_destination = Ikke-eksisterende målfil
# Other
selected_all_reference_folders = Kan ikke starte søk, når alle kataloger er angitt som referanselapper
searching_for_data = Søker data, det kan ta en stund, vennligst vent...
text_view_messages = MELDINGER
text_view_warnings = ADVARSELSER
text_view_errors = FEILSER
about_window_motto = Dette programmet er gratis å bruke og vil alltid være.
# Various dialog
dialogs_ask_next_time = Spør neste gang
delete_file_failed = Kunne ikke slette filen { $name }, årsak { $reason }
delete_title_dialog = Bekreft sletting
delete_question_label = Er du sikker på at du vil slette filer?
delete_all_files_in_group_title = Bekreftelse på sletting av alle filer i gruppen
delete_all_files_in_group_label1 = For noen grupper er alle poster valgt.
delete_all_files_in_group_label2 = Er du sikker på at du vil slette dem?
delete_folder_failed = Kunne ikke slette mappen { $dir } fordi mappen ikke eksisterer, du har ikke tillatelse eller mappen er ikke tom.
delete_items_label = { $items } filer vil bli slettet.
delete_items_groups_label = { $items } filer fra { $groups } grupper vil bli slettet.
hardlink_failed = Mislyktes i å koble sammen
hard_sym_invalid_selection_title_dialog = Ugyldig valg med noen grupper
hard_sym_invalid_selection_label_1 = I noen grupper er det bare én post valgt og det vil bli ignorert.
hard_sym_invalid_selection_label_2 = For å kunne feste koblingen til disse filene, må minst to resultater i gruppen velges.
hard_sym_invalid_selection_label_3 = Først i gruppen gjenkjennes som originalen og endres ikke, men sekund og senere endres.
hard_sym_link_title_dialog = Lenke bekreftelse
hard_sym_link_label = Er du sikker på at du vil koble disse filene?
move_folder_failed = Kunne ikke flytte mappen { $name }, årsak { $reason }
move_file_failed = Kunne ikke flytte filen { $name }, årsak { $reason }
move_files_title_dialog = Velg mappen du vil flytte dupliserte filer til
move_files_choose_more_than_1_path = Bare én sti kan velges for å kunne kopiere sine dupliserte filer, valgt { $path_number }.
move_stats = Flott flyttet { $num_files }/{ $all_files } elementer
save_results_to_file = Lagrede resultater både i txt og json-filer i { $name } mappe.
search_not_choosing_any_music = FEIL: Du må velge minst en avkrysningsboks med musikk som søker.
search_not_choosing_any_broken_files = FEIL: Du må velge minst en avkrysningsboks med sjekket ødelagte filer.
include_folders_dialog_title = Mapper å inkludere
exclude_folders_dialog_title = Mapper som skal ekskluderes
include_manually_directories_dialog_title = Legg til mappe manuelt
cache_properly_cleared = Riktig tømt cache
cache_clear_duplicates_title = Tømmer duplikatene
cache_clear_similar_images_title = Fjerner lignende bilde-mellomlager
cache_clear_similar_videos_title = Tømmer hurtigbufferen for videoer
cache_clear_message_label_1 = Vil du slette cachen med utdaterte oppføringer?
cache_clear_message_label_2 = Denne operasjonen vil fjerne alle cacheoppføringer som peker til ugyldige filer.
cache_clear_message_label_3 = Dette kan øke lasting og lagring på cache.
cache_clear_message_label_4 = ADVARSEL: Operasjonen vil fjerne alle bufrede data fra eksterne stasjoner som ikke er koblet til. Så hver hash må regenereres.
# Show preview
preview_image_resize_failure = Kunne ikke endre størrelse på bildet { $name }.
preview_image_opening_failure = Klarte ikke å åpne bilde { $name }, årsak { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Gruppe { $current_group }/{ $all_groups } ({ $images_in_group } bilder)
compare_move_left_button = L
compare_move_right_button = R
