# Window titles
window_settings_title = Innstillinger
window_main_title = Czkawka (Hikke)
window_progress_title = Skanner
window_compare_images = Sammenlign bilder
# General
general_ok_button = OK
general_close_button = Lukk
# Krokiet info dialog
krokiet_info_title = Utfasingsvarsel
krokiet_info_message =
    Czkawka GTK 12.0 er den endelige versjonen. Ingen flere oppdateringer, funksjoner eller feilrettinger er planlagt.
    
    De fleste funksjonene fra Czkawka GTK er tilgjengelige i Krokiet, vanligvis i en enklere, raskere og mer stabil form. Krokiet legger også til nye funksjoner og forbedringer som ikke var mulig i GTK-versjonen.
    
    Hvis du fortsatt bruker Czkawka GTK, bør det være enkelt å bytte til Krokiet, siden det har et lignende grensesnitt, færre avhengigheter og bedre støtte på tvers av plattformer.
    
    PS: Denne meldingen skal bare vises én gang. Hvis den vises igjen, sett miljøvariabelen CZKAWKA_DONT_ANNOY_ME til en ikke-tom verdi.
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
same_music_seconds_label = Minste fragmentvarighet i sekunder
same_music_similarity_label = Maksimal forskjell
music_compare_only_in_title_group = Sammenlign innenfor grupper av lignende titler
music_compare_only_in_title_group_tooltip =
    Når aktivert, blir filer gruppert etter tittel og deretter sammenlignet med hverandre.
    
    Med 10000 filer vil det, i stedet for nesten 100 millioner sammenligninger, vanligvis bli rundt 20000 sammenligninger.
same_music_tooltip =
    Søk etter lignende musikkfiler basert på innhold kan konfigureres ved å angi følgende:
    
    - Minste fragmentvarighet en musikkfil kan identifiseres som lignende etter
    - Maksimal forskjell mellom to testede fragmenter
    
    Nøkkelen til gode resultater er å finne fornuftige kombinasjoner av disse parameterne.
    
    Hvis minimumstiden settes til 5 s og maksimal forskjell til 1,0, vil det se etter nesten identiske fragmenter i filene.
    En tid på 20 s og en maksimal forskjell på 6,0 fungerer derimot bra for å finne remikser/liveversjoner osv.
    
    Som standard sammenlignes hver musikkfil med alle andre, og dette kan ta mye tid når du tester mange filer. Det er derfor vanligvis bedre å bruke referansemapper og spesifisere hvilke filer som skal sammenlignes med hverandre (med samme mengde filer vil sammenligning av fingeravtrykk være minst 4 ganger raskere enn uten referansemapper).
music_comparison_checkbox_tooltip =
    Den søker etter lignende musikkfiler ved hjelp av AI, som bruker maskinlæring til å fjerne parenteser fra en frase. Med dette alternativet aktivert vil for eksempel filene i dette tilfellet bli betraktet som duplikater:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Skill mellom store og små bokstaver
duplicate_case_sensitive_name_tooltip =
    Når aktivert, vil appen bare gruppere når de har nøyaktig samme navn, f.eks. Żołd <-> Żołd
    
    Deaktivering av dette alternativet vil gruppere navn uten å sjekke om hver bokstav er like stor, f.eks. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Størrelse og navn
duplicate_mode_name_combo_box = Navn
duplicate_mode_size_combo_box = Størrelse
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka har 3 typer hashes:
    
    Blake3 - kryptografisk hash-funksjon. Dette er standard fordi den er veldig rask.
    
    CRC32 - enkel hash-funksjon. Dette bør være raskere enn Blake3, men kan svært sjelden ha noen kollisjoner.
    
    XXH3 - meget likt i ytelse og hash-kvalitet til Blake3 (men ikke-kryptografisk). Så, slike moduser kan enkelt byttes om.
duplicate_check_method_tooltip =
    Czkawka tilbyr foreløpig tre typer metoder for å finne duplikater med:
    
    Navn - Finner filer med samme navn.
    
    Størrelse - Finner filer med samme størrelse.
    
    Hash - Finner filer med samme innhold. Denne modusen hasher filen og sammenligner senere denne hashen for å finne duplikater. Denne modusen er den sikreste måten å finne duplikater på. Appen bruker hurtigbuffer mye, så den andre og senere skanningene av de samme dataene bør være mye raskere enn den første.
image_hash_size_tooltip =
    Hvert undersøkte bilde gir en spesiell hash som kan sammenlignes med hverandre, og en liten forskjell mellom dem betyr at disse bildene er like.
    
    8 i hash-størrelse er ganske bra for å finne bilder som bare er litt like originalen. Med et større sett med bilder (>1000) vil dette gi svært mange falske positive. Derfor anbefaler jeg å bruke en større hash-størrelse i dette tilfellet.
    
    16 er standard hash-størrelse, som er et godt kompromiss mellom å finne selv litt like bilder og å ha bare en liten mengde hash-kollisjoner.
    
    32 og 64 i hash-størrelse finner bare svært like bilder, men bør ha nesten ingen falske positiver (unntatt kanskje enkelte bilder med alfakanal).
image_resize_filter_tooltip =
    For å beregne hash av et bilde, må biblioteket først endre størrelsen på det.
    
    Avhengig av valgt algoritme vil bildet som brukes til å beregne hash, se litt forskjellig ut.
    
    Den raskeste algoritmen å bruke, men også den som gir de dårligste resultatene, er Nearest. Den er aktivert som standard, fordi den lavere kvaliteten ikke er særlig synlig med en 16x16 hash-størrelse.
    
    Med en 8x8 hash-størrelse anbefales det å bruke en annen algoritme enn Nearest for å få bedre grupper av bilder.
image_hash_alg_tooltip =
    Brukere kan velge mellom en av mange algoritmer for å beregne hashen.
    
    Hver har både sterke og svake sider, og vil noen ganger gi bedre og noen ganger dårligere resultater for ulike bilder.
    
    Så for å finne den beste for deg, kreves det manuell testing.
image_geometric_invariance_tooltip = Sammenlign også speilvendte/snudde og eventuelt roterte varianter av hvert bilde. Dette forbedrer treffsikkerheten, men øker hash-tiden.
big_files_mode_combobox_tooltip = Lar deg søke etter minste/største filer
big_files_mode_label = Avmerkede filer
big_files_mode_smallest_combo_box = Den minste
big_files_mode_biggest_combo_box = Den største
main_notebook_duplicates = Dupliserte filer
main_notebook_empty_directories = Tomme mapper
main_notebook_big_files = Store filer
main_notebook_empty_files = Tomme filer
main_notebook_temporary = Midlertidige filer
main_notebook_similar_images = Lignende bilder
main_notebook_similar_videos = Lignende videoer
main_notebook_same_music = Musikkduplikater
main_notebook_symlinks = Ugyldige symlinker
main_notebook_broken_files = Ødelagte filer
main_notebook_bad_extensions = Feil filendelser
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
main_tree_view_column_symlink_file_name = Symlink-filnavn
main_tree_view_column_symlink_folder = Symlinkmappe
main_tree_view_column_destination_path = Destinasjonssti
main_tree_view_column_type_of_error = Type feil
main_tree_view_column_current_extension = Gjeldende filendelse
main_tree_view_column_proper_extensions = Riktig filendelse
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Kodek
main_label_check_method = Sjekkmetode
main_label_hash_type = Hashtype
main_label_hash_size = Hashstørrelse
main_label_geometric_invariance = Geometrisk invarians
main_label_size_bytes = Størrelse (bytes)
main_label_min_size = Min
main_label_max_size = Maks
main_label_shown_files = Antall filer som vises
main_label_resize_algorithm = Skaleringsalgoritme
main_label_similarity = Likhet{ "   " }
main_check_box_broken_files_audio = Lyd
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Arkiv
main_check_box_broken_files_image = Bilde
main_check_box_broken_files_video = Video
main_check_box_broken_files_video_tooltip = Bruker ffmpeg/ffprobe for å validere videofiler. Ganske treg og kan detektere pedantiske feil selv om filen spilles fint.
check_button_general_same_size = Ignorer samme størrelse
check_button_general_same_size_tooltip = Ignorer filer med identisk størrelse i resultatene - vanligvis er disse 1:1-duplikater
main_label_size_bytes_tooltip = Størrelse på filer som vil bli brukt i skanning
# Upper window
upper_tree_view_included_folder_column_title = Mapper å søke i
upper_tree_view_included_reference_column_title = Referansemapper
upper_recursive_button = Rekursivt
upper_recursive_button_tooltip = Hvis valgt, søk også etter filer som ikke er plassert direkte under valgte mapper.
upper_manual_add_included_button = Legg til manuelt
upper_add_included_button = Legg til
upper_remove_included_button = Fjern
upper_manual_add_excluded_button = Legg til manuelt
upper_add_excluded_button = Legg til
upper_remove_excluded_button = Fjern
upper_manual_add_included_button_tooltip =
    Legg til et mappenavn å søke i manuelt.
    
    For å legge til flere stier samtidig, separer dem med ;
    
    /home/roman;/home/rozkaz vil legge til to kataloger, /home/roman og /home/rozkaz
upper_add_included_button_tooltip = Legg til ny mappe i søk.
upper_remove_included_button_tooltip = Fjern mappen fra søk.
upper_manual_add_excluded_button_tooltip =
    Legg til ekskludert mappenavn for hånd.
    
    For å legge til flere stier på en gang, separer dem med ;
    
    /home/roman;/home/krokiet vil legge til to kataloger /home/roman og /home/keokiet
upper_add_excluded_button_tooltip = Legg til mappe som skal utelukkes i søk.
upper_remove_excluded_button_tooltip = Fjern mappen fra ekskludert.
upper_notebook_items_configuration = Konfigurasjon av elementer
upper_notebook_excluded_directories = Ekskluderte stier
upper_notebook_included_directories = Inkluderte stier
upper_allowed_extensions_tooltip =
    Tillatte filendelser må være atskilt med komma (som standard er alle tilgjengelige).
    
    Følgende makroer, som legger til flere filendelser samtidig, er også tilgjengelige: IMAGE, VIDEO, MUSIC, TEXT.
    
    Brukseksempel ".exe, IMAGE, VIDEO, .rar, 7z" - dette betyr at bilder (f.eks. jpg, png), videoer (f.eks. avi, mp4), exe, rar og 7z filer vil bli skannet.
upper_excluded_extensions_tooltip =
    Liste over deaktiverte filer som vil bli ignorert under skanning.
    
    Ved bruk av både tillatte og deaktiverte filendelser har denne høyere prioritet, så filen vil ikke bli sjekket.
upper_excluded_items_tooltip =
    Ekskluderte elementer må inneholde * wildcard og skal være separert med komma.
    Dette er tregere enn Ekskluderte stier, så bruk det forsiktig.
upper_excluded_items = Ekskluderte elementer:
upper_allowed_extensions = Tillatte filendelser:
upper_excluded_extensions = Deaktiverte filendelser:
# Popovers
popover_select_all = Velg alle
popover_unselect_all = Fjern alle valg
popover_reverse = Omvendt utvalg
popover_select_all_except_shortest_path = Velg alle unntatt korteste sti
popover_select_all_except_longest_path = Velg alle unntatt lengste sti
popover_select_all_except_oldest = Velg alle unntatt eldste
popover_select_all_except_newest = Velg alle unntatt nyeste
popover_select_one_oldest = Velg en eldste
popover_select_one_newest = Velg en nyeste
popover_select_custom = Velg egendefinert
popover_unselect_custom = Avvelg egendefinert
popover_select_all_images_except_biggest = Velg alle unntatt største
popover_select_all_images_except_smallest = Velg alle unntatt minste
popover_custom_path_check_button_entry_tooltip =
    Velg poster etter sti.
    
    Eksempelbruk:
    /home/pimpek/rzecz.txt kan finnes med /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Velg poster etter filnavn.
    
    Eksempelbruk:
    /usr/ping/pong.txt kan finnes med *ong*
popover_custom_regex_check_button_entry_tooltip =
    Velg poster ved angitt regex.
    
    Med denne modusen vil den søkte teksten være Sti med navn.
    
    Eksempel på bruk:
    /usr/bin/ziemniak.txt finner du med /ziem[a-z]+
    
    Dette bruker standard Rust-regex-implementasjon. Du kan lese mer om den her: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Aktiverer case-sensitiv deteksjon.
    
    Når deaktivert, finner /home/* både /HoMe/roman og /home/roman.
popover_custom_not_all_check_button_tooltip =
    Hindrer å velge alle poster i gruppen.
    
    Dette er aktivert som standard, fordi i de fleste situasjoner vil du ikke slette både originale og dupliserte filer, men vil beholde minst én fil.
    
    ADVARSEL: Denne innstillingen fungerer ikke hvis du allerede har valgt alle resultater i en gruppe manuelt.
popover_custom_regex_path_label = Sti
popover_custom_regex_name_label = Navn
popover_custom_regex_regex_label = Regex sti + navn
popover_custom_case_sensitive_check_button = Skill mellom store og små bokstaver
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
bottom_save_button = Lagre
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Flytt
bottom_sort_button = Sorter
bottom_compare_button = Sammenlign
bottom_search_button_tooltip = Start søk
bottom_select_button_tooltip = Velg poster. Bare valgte filer/mapper kan bli behandlet senere.
bottom_delete_button_tooltip = Slett valgte filer/mapper.
bottom_save_button_tooltip = Lagre data om søk i fil
bottom_symlink_button_tooltip =
    Opprett symlinker.
    Virker bare når minst to resultater i en gruppe er valgt.
    Den første forblir uendret, og den andre og senere blir symlinket til den første.
bottom_hardlink_button_tooltip =
    Opprett hardlinker.
    Virker bare når minst to resultater i en gruppe er valgt.
    Den første forblir uendret, og den andre og senere blir hardlinket til den første.
bottom_hardlink_button_not_available_tooltip =
    Opprett hardlinker.
    Knappen er deaktivert, fordi hardlinker ikke kan opprettes.
    Hardlinker fungerer bare med administratorrettigheter i Windows, så pass på at du kjører programmet som administrator.
    Hvis programmet allerede fungerer med slike privilegier, sjekk om lignende problemer er observert på GitHub.
bottom_move_button_tooltip =
    Flytter filer til valgt mappe.
    Den kopierer alle filer til mappen uten å bevare mappetreet.
    Når du prøver å flytte to filer med identisk navn til en mappe, vil den andre feile og vise en feil.
bottom_sort_button_tooltip = Sorterer filer/mapper etter valgt metode.
bottom_compare_button_tooltip = Sammenlign bilder i gruppen.
bottom_show_errors_tooltip = Vis/skjul tekstpanelet nederst.
bottom_show_upper_notebook_tooltip = Vis/Skjul øvre notebook panel.
# Progress Window
progress_stop_button = Stopp
progress_stop_additional_message = Stopp forespurt
# About Window
about_repository_button_tooltip = Lenke til repository-siden med kildekoden.
about_donation_button_tooltip = Lenke til donasjonssiden.
about_instruction_button_tooltip = Lenke til instruksjonssiden.
about_translation_button_tooltip = Lenke til Crowdin-siden med appoversettelser. Offisielt støttes polsk og engelsk.
about_repository_button = Repository
about_donation_button = Donasjon
about_instruction_button = Instruksjon
about_translation_button = Oversettelse
about_other_apps_button = Andre apper
about_other_apps_dialog_title = Andre programmer av qarmin
about_other_apps_open_source_note = Alle oppførte applikasjoner er gratis og åpen kildekode.
about_other_apps_open_button = Åpne
about_other_apps_szyszka_desc = Rask og kraftig verktøy for å endre navn på filer.
about_other_apps_mykrut_desc = Enkel, rask og opinionert filbehandler for Linux.
about_other_apps_dcmki_viewer_desc = Enkel DICOM-fremviser.
about_other_apps_video_thumbnailer_desc = Wrapper rundt videominiatyrgeneratoren som brukes i Czkawka.
about_other_apps_space_finder_desc = Et enkelt verktøy for å finne de største filene på systemet ditt.
about_other_apps_system_info_collector_desc = Samler RAM/CPU-bruken fra operativsystemet og viser den som grafer.
# Header
header_setting_button_tooltip = Åpner dialogboksen for innstillinger.
header_about_button_tooltip = Åpner dialog med info om app.
header_krokiet_button_tooltip = Prøv Krokiet - den nye og forbedrede versjonen!
# Krokiet promo dialog
krokiet_promo_title = Møt Krokiet!
krokiet_promo_message =
    Hei der, modig Czkawka-bruker!
    
    Kraften er åpenbart med deg, men Krokiet er ikke - en nyere, raskere, lettere og betydelig kjekkere (forutsatt at apper faktisk kan være kjekke) duplikatrenser.
    
    Krokiet inneholder alt folk likte ved Czkawka. Den er helt gratis, åpen kildekode, har et unikt og enkelt brukergrensesnitt (både rost og hatet av mange), introduserer mange nye funksjoner, bruker færre avhengigheter og fungerer langt mer pålitelig på tvers av ulike plattformer.
    
    Og hvis du har gått glipp av det, finnes det allerede en enda nyere app enn Krokiet - Cedinia, designet først og fremst for Android-enheter og berøringsskjerm.
    
    Czkawka GTK har tjent oss godt, men vakten er nå over.
krokiet_promo_link_download = Last ned Krokiet/Cedinia
krokiet_promo_link_project = Prosjektside

# Settings


## General

settings_number_of_threads = Antall brukte tråder
settings_number_of_threads_tooltip = Antall brukte tråder. 0 betyr at alle tilgjengelige tråder vil bli brukt.
settings_use_rust_preview = Bruk eksterne biblioteker i stedet for gtk for å laste forhåndsvisninger
settings_use_rust_preview_tooltip =
    Bruk av gtk-forhåndsvisninger vil noen ganger være raskere og støtte flere formater, men noen ganger kan det være akkurat det motsatte.
    
    Hvis du har problemer med å laste forhåndsvisninger, kan du prøve å endre denne innstillingen.
    
    På ikke-Linux-systemer anbefales det å bruke dette alternativet, fordi gtk-pixbuf ikke alltid er tilgjengelig der, slik at deaktivering av dette alternativet vil hindre forhåndsvisning av enkelte bilder.
settings_label_restart = Start programmet på nytt for å bruke innstillingene!
settings_ignore_other_filesystems = Ignorer andre filsystemer (bare Linux)
settings_ignore_other_filesystems_tooltip =
    ignorerer filer som ikke er i samme filsystem som de gjennomsøkte katalogene.
    
    Fungerer på samme måte som -xdev-alternativet i find-kommandoen på Linux
settings_save_at_exit_button_tooltip = Lagre konfigurasjon til fil når appen lukkes.
settings_load_at_start_button_tooltip =
    Last inn konfigurasjon fra filen når du åpner appen.
    
    Hvis dette ikke er aktivert, brukes standardinnstillingene.
settings_confirm_deletion_button_tooltip = Vis bekreftelsesdialog når du klikker på slett-knappen.
settings_confirm_link_button_tooltip = Vis bekreftelsesdialog når du klikker på hard-/symlink-knappen.
settings_confirm_group_deletion_button_tooltip = Vis advarselsdialog når du prøver å slette alle poster fra gruppen.
settings_show_text_view_button_tooltip = Vis tekstpanelet nederst i brukergrensesnittet.
settings_use_cache_button_tooltip = Bruk filmellomlager.
settings_save_also_as_json_button_tooltip = Lagre hurtigbufferen til (menneskelesbart) JSON-format. Det er mulig å endre innholdet. Hurtigbufferen fra denne filen vil bli lest automatisk av appen dersom hurtigbuffer i binærformat (med bin-filendelse) mangler.
settings_use_trash_button_tooltip = Flytter filer til papirkurv istedenfor å slette dem permanent.
settings_language_label_tooltip = Språk for brukergrensesnittet.
settings_save_at_exit_button = Lagre konfigurasjon når appen lukkes
settings_load_at_start_button = Last inn konfigurasjon når du åpner appen
settings_confirm_deletion_button = Vis bekreftelsesdialog ved sletting av filer
settings_confirm_link_button = Vis bekreftelsesdialog når du oppretter hardlinker/symlinker for filer
settings_confirm_group_deletion_button = Vis bekreftelsesdialog når du sletter alle filer i gruppen
settings_show_text_view_button = Vis nederste tekstpanel
settings_use_cache_button = Bruk hurtigbuffer
settings_save_also_as_json_button = Lagre også hurtigbufferen som JSON-fil
settings_use_trash_button = Flytt slettede filer til papirkurv
settings_language_label = Språk
settings_multiple_delete_outdated_cache_checkbutton = Slett utdaterte hurtigbuffer-oppføringer automatisk
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Slett utdaterte hurtigbufferresultater som peker til filer som ikke finnes.
    
    Når dette er aktivert, sørger appen for at alle poster peker til gyldige filer når den laster inn poster (ødelagte blir ignorert).
    
    Å deaktivere dette vil hjelpe når du skanner filer på eksterne stasjoner, slik at hurtigbufferoppføringer om dem ikke blir slettet i neste skanning.
    
    Hvis du har flere hundre tusen poster i hurtigbufferen, anbefales det å aktivere dette, som vil gjøre innlasting/lagring av hurtigbufferen raskere ved start/slutt av skanningen.
settings_notebook_general = Generelt
settings_notebook_duplicates = Duplikater
settings_notebook_images = Lignende bilder
settings_notebook_videos = Lignende video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Viser forhåndsvisning på høyre side (når du velger en bildefil).
settings_multiple_image_preview_checkbutton = Vis forhåndsvisning av bilde
settings_multiple_clear_cache_button_tooltip =
    Tøm hurtigbufferen for utdaterte oppføringer manuelt.
    Dette bør bare brukes hvis automatisk tømming er deaktivert.
settings_multiple_clear_cache_button = Fjern utdaterte resultater fra hurtigbufferen.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Skjuler alle filer unntatt én, hvis alle peker til samme data (er fastkoblet).
    
    Eksempel: Hvis det (på disk) finnes syv filer som er fastkoblet til bestemte data og én annen fil med samme data, men med en annen inode, vil dupliseringsverktøyet bare vise én unik fil og én fil fra de fastkoblede filene.
settings_duplicates_minimal_size_entry_tooltip =
    Angi minste filstørrelse som lagres i hurtigbufferen.
    
    Hvis du velger en mindre verdi, genererer du flere poster. Dette vil gjøre søket raskere, men gjøre innlasting/lagring av hurtigbufferen tregere.
settings_duplicates_prehash_checkbutton_tooltip =
    Aktiverer hurtigbufring av prehash (en hash beregnet fra en liten del av filen), som gjør det mulig å tidligere forkaste ikke-dupliserte resultater.
    
    Den er deaktivert som standard fordi den i noen situasjoner kan gjøre ting tregere.
    
    Det anbefales sterkt å bruke den når du skanner hundretusenvis eller millioner av filer, fordi den kan gjøre søket flere ganger raskere.
settings_duplicates_prehash_minimal_entry_tooltip = Minimal størrelse på oppføring i hurtigbufferen.
settings_duplicates_hide_hard_link_button = Skjul hardlinker
settings_duplicates_prehash_checkbutton = Bruk prehash-hurtigbuffer
settings_duplicates_minimal_size_cache_label = Minimal størrelse på filer (i byte) lagret i hurtigbufferen
settings_duplicates_minimal_size_cache_prehash_label = Minimal størrelse på filer (i byte) lagret i hurtigbufferen for prehash

## Saving/Loading settings

settings_saving_button_tooltip = Lagre gjeldende innstillingskonfigurasjon til filen.
settings_loading_button_tooltip = Last innstillinger fra fil og erstatt gjeldende konfigurasjon med dem.
settings_reset_button_tooltip = Tilbakestill den gjeldende konfigurasjonen til standard.
settings_saving_button = Lagre konfigurasjon
settings_loading_button = Last inn konfigurasjon
settings_reset_button = Tilbakestill konfigurasjon

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Åpner mappen der hurtigbuffertekstfilene er lagret.
    
    Å modifisere hurtigbufferfilene kan forårsake at ugyldige resultater vises. Å modifisere stien kan imidlertid spare tid når du flytter en stor mengde filer til et annet sted.
    
    Du kan kopiere disse filene mellom datamaskiner for å slippe å skanne filene på nytt (så lenge de har en lignende katalogstruktur).
    
    Hvis det oppstår problemer med hurtigbufferen, kan disse filene fjernes. Appen vil automatisk regenerere dem.
settings_folder_settings_open_tooltip =
    Åpner mappen der Czkawka-konfigurasjonen er lagret.
    
    ADVARSEL: Manuell endring av konfigurasjonen kan ødelegge arbeidsflyten din.
settings_folder_cache_open = Åpne hurtigbuffermappen
settings_folder_settings_open = Åpne innstillingsmappen
# Compute results
compute_stopped_by_user = Søket ble stoppet av bruker
compute_found_duplicates_hash_size = Fant { $number_files } duplikater i { $number_groups } grupper som tok { $size } i { $time }
compute_found_duplicates_name = Fant { $number_files } duplikater i { $number_groups } grupper i { $time }
compute_found_empty_folders = Fant { $number_files } tomme mapper i { $time }
compute_found_empty_files = Fant { $number_files } tomme filer i { $time }
compute_found_big_files = Fant { $number_files } store filer i { $time }
compute_found_temporary_files = Fant { $number_files } midlertidige filer i { $time }
compute_found_images = Fant { $number_files } lignende bilder i { $number_groups } grupper i { $time }
compute_found_videos = Fant { $number_files } lignende videoer i { $number_groups } grupper i { $time }
compute_found_music = Fant { $number_files } lignende musikkfiler i { $number_groups } grupper i { $time }
compute_found_invalid_symlinks = Fant { $number_files } ugyldige symlinker i { $time }
compute_found_broken_files = Fant { $number_files } ødelagte filer i { $time }
compute_found_bad_extensions = Fant { $number_files } filer med ugyldige filendelser i { $time }
# Progress window
progress_current_stage = Gjeldende trinn: { " " }
progress_all_stages = Alle stadier:{ "  " }
# Saving loading 
saving_loading_saving_success = Lagret konfigurasjon til filen { $name }.
saving_loading_saving_failure = Kunne ikke lagre konfigurasjonsdata til filen { $name }, årsak { $reason }.
saving_loading_reset_configuration = Gjeldende konfigurasjon ble fjernet.
saving_loading_loading_success = Riktig lastet applikasjonskonfigurasjon.
saving_loading_no_config_file = Fant ingen konfigurasjonsfil, bruker standardinnstillinger.
saving_loading_failed_to_create_config_file = Kunne ikke opprette konfigurasjonsfilen "{ $path }", grunn "{ $reason }".
saving_loading_failed_to_read_config_file = Kan ikke laste konfigurasjonen fra "{ $path }" fordi den ikke eksisterer eller ikke er en fil.
saving_loading_failed_to_read_data_from_file = Kan ikke lese data fra filen "{ $path }", grunn "{ $reason }".
# Other
selected_all_reference_folders = Kan ikke starte søk, når alle kataloger er angitt som referansemapper
searching_for_data = Søker data, det kan ta en stund, vennligst vent...
text_view_messages = MELDINGER
text_view_warnings = ADVARSLER
text_view_errors = FEIL
about_window_motto = Dette programmet er gratis å bruke og vil alltid være det.
krokiet_new_app = Denne GTK-versjonen av Czkawka utvikles ikke lenger fra og med versjon 12. For nye funksjoner og aktiv utvikling, bruk Krokiet, som er mer stabil og mer ytelsessterk.
# Various dialog
dialogs_ask_next_time = Spør neste gang
symlink_failed = Kunne ikke opprette symlink fra { $name } til { $target }, årsak { $reason }
delete_title_dialog = Bekreft sletting
delete_question_label = Er du sikker på at du vil slette filer?
delete_all_files_in_group_title = Bekreftelse på sletting av alle filer i gruppen
delete_all_files_in_group_label1 = For noen grupper er alle poster valgt.
delete_all_files_in_group_label2 = Er du sikker på at du vil slette dem?
delete_items_label = { $items } filer vil bli slettet.
delete_items_groups_label = { $items } filer fra { $groups } grupper vil bli slettet.
hardlink_failed = Kunne ikke opprette hardlink fra { $name } til { $target }, årsak { $reason }
hard_sym_invalid_selection_title_dialog = Ugyldig valg med noen grupper
hard_sym_invalid_selection_label_1 = I noen grupper er det bare én post valgt og det vil bli ignorert.
hard_sym_invalid_selection_label_2 = For å kunne opprette hard-/symkoblinger for disse filene, må minst to resultater i gruppen velges.
hard_sym_invalid_selection_label_3 = Først i gruppen gjenkjennes som originalen og endres ikke, men den andre og senere blir endret.
hard_sym_link_title_dialog = Lenkebekreftelse
hard_sym_link_label = Er du sikker på at du vil koble disse filene?
move_folder_failed = Kunne ikke flytte mappen { $name }, årsak { $reason }
move_file_failed = Kunne ikke flytte filen { $name }, årsak { $reason }
move_files_title_dialog = Velg mappen du vil flytte dupliserte filer til
move_files_choose_more_than_1_path = Bare én sti kan velges for å kunne kopiere sine dupliserte filer, valgt { $path_number }.
move_stats = Riktig flyttet { $num_files }/{ $all_files } elementer
save_results_to_file = Lagret resultater både til txt- og json-filer i "{ $name }"-mappen.
search_not_choosing_any_music = FEIL: Du må velge minst én avkrysningsboks for musikksøketyper.
search_not_choosing_any_broken_files = FEIL: Du må velge minst en avkrysningsboks med sjekket ødelagte filer.
include_folders_dialog_title = Mapper å inkludere
exclude_folders_dialog_title = Mapper å ekskludere
include_manually_directories_dialog_title = Legg til mappe manuelt
cache_properly_cleared = Riktig tømt hurtigbuffer
cache_clear_duplicates_title = Tømmer hurtigbufferen for duplikater
cache_clear_similar_images_title = Tømmer hurtigbufferen for lignende bilder
cache_clear_similar_videos_title = Tømmer hurtigbufferen for videoer
cache_clear_message_label_1 = Vil du slette de utdaterte oppføringene fra hurtigbufferen?
cache_clear_message_label_2 = Denne operasjonen vil fjerne alle hurtigbufferoppføringer som peker til ugyldige filer.
cache_clear_message_label_3 = Dette kan gjøre innlasting og lagring av hurtigbufferen noe raskere.
cache_clear_message_label_4 = ADVARSEL: Operasjonen vil fjerne alle data i hurtigbufferen fra eksterne stasjoner som ikke er koblet til. Så hver hash må regenereres.
# Show preview
preview_image_resize_failure = Kunne ikke endre størrelse på bildet { $name }.
preview_image_opening_failure = Klarte ikke å åpne bildet { $name }, årsak { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Gruppe { $current_group }/{ $all_groups } ({ $images_in_group } bilder)
compare_move_left_button = L
compare_move_right_button = R
