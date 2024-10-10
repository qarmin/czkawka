# Window titles
window_settings_title = Inställningar
window_main_title = Czkawka (Hiccup)
window_progress_title = Scannar
window_compare_images = Jämför bilder
# General
general_ok_button = Ok
general_close_button = Stäng
# Main window
music_title_checkbox = Titel
music_artist_checkbox = Artist
music_year_checkbox = År
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Genre
music_length_checkbox = Längd
music_comparison_checkbox = Ungefärlig jämförelse
music_checking_by_tags = Taggar
music_checking_by_content = Innehåll
same_music_seconds_label = Minsta fragment sekund varaktighet
same_music_similarity_label = Maximal skillnad
music_compare_only_in_title_group = Jämför endast i titeln
music_compare_only_in_title_group_tooltip =
    När den är aktiverad grupperas filerna efter titel och jämförs sedan med varandra.
    
    Med 10000 filer, i stället nästan 100 miljoner jämförelser brukar det finnas runt 20000 jämförelser.
same_music_tooltip =
    Sökning efter liknande musikfiler genom dess innehåll kan konfigureras genom att ställa in:
    
    - Minsta fragmenttid efter vilken musikfiler kan identifieras som liknande
    - Maximal skillnad mellan två testade fragment
    
    Nyckeln till bra resultat är att hitta förnuftiga kombinationer av dessa parametrar, för tillhandahållen.
    
    Att ställa in den minsta tiden till 5s och den maximala skillnaden till 1.0, kommer att leta efter nästan identiska fragment i filerna.
    En tid på 20-talet och en maximal skillnad på 6,0, å andra sidan, fungerar bra för att hitta remixer/live-versioner etc.
    
    Som standard jämförs varje musikfil med varandra och detta kan ta mycket tid vid testning av många filer, så är det oftast bättre att använda referensmappar och ange vilka filer som ska jämföras med varandra(med samma mängd filer, Att jämföra fingeravtryck kommer att vara snabbare minst 4x än utan referensmappar).
music_comparison_checkbox_tooltip =
    Den söker efter liknande musikfiler med AI, som använder maskininlärning för att ta bort parenteser från en fras. Till exempel, med detta alternativ aktiverat, filerna i fråga kommer att betraktas som dubbletter:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Skiftlägeskänslig
duplicate_case_sensitive_name_tooltip =
    När detta är aktiverat spelar gruppen bara in när de har exakt samma namn t.ex. Żołd <-> Żołd
    
    Inaktivera sådana alternativ kommer gruppnamn utan att kontrollera om varje bokstav är samma storlek t.ex. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Storlek och namn
duplicate_mode_name_combo_box = Namn
duplicate_mode_size_combo_box = Storlek
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka erbjuder 3 typer av hash:
    
    Blake3 - kryptografisk hash-funktion. Detta är standard eftersom det är mycket snabbt.
    
    CRC32 - enkel hash-funktion. Detta bör vara snabbare än Blake3, men kan mycket sällan ha några kollisioner.
    
    XXH3 - mycket lik i prestanda och hashkvalitet till Blake3 (men icke-kryptografisk). Så, sådana lägen kan lätt bytas ut.
duplicate_check_method_tooltip =
    För tillfället erbjuder Czkawka tre typer av metoder för att hitta dubbletter av:
    
    Namn - Hittar filer som har samma namn.
    
    Storlek - Hittar filer som har samma storlek.
    
    Hash - Hittar filer som har samma innehåll. Detta läge hashar filen och senare jämför denna hash för att hitta dubbletter. Detta läge är det säkraste sättet att hitta dubbletter. Appen använder starkt cache, så andra och ytterligare skanningar av samma data bör vara mycket snabbare än den första.
image_hash_size_tooltip =
    Varje kontrollerad bild ger en speciell hash som kan jämföras med varandra, och en liten skillnad mellan dem innebär att dessa bilder är liknande.
    
    8 hash storlek är ganska bra att hitta bilder som bara är lite liknande till originalet. Med en större uppsättning bilder (>1000), kommer detta att producera en stor mängd falska positiva, så jag rekommenderar att använda en större hash storlek i detta fall.
    
    16 är standard hashstorlek vilket är en ganska bra kompromiss mellan att hitta även lite liknande bilder och att bara ha en liten mängd hashkollisioner.
    
    32 och 64 hashen finner endast mycket liknande bilder, men bör ha nästan inga falska positiva (kanske förutom vissa bilder med alfa-kanal).
image_resize_filter_tooltip =
    För att beräkna hash av bilden, måste biblioteket först ändra storlek på den.
    
    Beroende på vald algoritm kommer den resulterande bilden som används för att beräkna hash att se lite annorlunda ut.
    
    Den snabbaste algoritmen att använda, men också den som ger de sämsta resultaten, är nära! Det är aktiverat som standard, eftersom med 16x16 hash storlek lägre kvalitet är det inte riktigt synligt.
    
    Med 8x8 hashstorlek rekommenderas att använda en annan algoritm än Närmaste för att få bättre grupper av bilder.
image_hash_alg_tooltip =
    Användare kan välja mellan en av många algoritmer för att beräkna hash.
    
    Var och en har både starka och svagare punkter och ger ibland bättre och ibland sämre resultat för olika bilder.
    
    Så, för att bestämma den bästa för dig krävs manuell testning.
big_files_mode_combobox_tooltip = Gör det möjligt att söka efter minsta/största filer
big_files_mode_label = Markerade filer
big_files_mode_smallest_combo_box = Den minsta
big_files_mode_biggest_combo_box = Den största
main_notebook_duplicates = Duplicera filer
main_notebook_empty_directories = Tomma kataloger
main_notebook_big_files = Stora filer
main_notebook_empty_files = Tomma filer
main_notebook_temporary = Tillfälliga filer
main_notebook_similar_images = Liknande bilder
main_notebook_similar_videos = Liknande videor
main_notebook_same_music = Musik Duplicerar
main_notebook_symlinks = Ogiltiga Symlinks
main_notebook_broken_files = Trasiga filer
main_notebook_bad_extensions = Dåliga tillägg
main_tree_view_column_file_name = Filnamn
main_tree_view_column_folder_name = Mappens namn
main_tree_view_column_path = Sökväg
main_tree_view_column_modification = Senast ändrad
main_tree_view_column_size = Storlek
main_tree_view_column_similarity = Likhet
main_tree_view_column_dimensions = Dimensioner
main_tree_view_column_title = Titel
main_tree_view_column_artist = Artist
main_tree_view_column_year = År
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Längd
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = Symlink filnamn
main_tree_view_column_symlink_folder = Symlink mapp
main_tree_view_column_destination_path = Målsökvägen
main_tree_view_column_type_of_error = Typ av fel
main_tree_view_column_current_extension = Nuvarande tillägg
main_tree_view_column_proper_extensions = Rätt tillägg
main_label_check_method = Kontrollera metod
main_label_hash_type = Hash typ
main_label_hash_size = Hashstorlek
main_label_size_bytes = Storlek (bytes)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Antal visade filer
main_label_resize_algorithm = Ändra storlek på algoritm
main_label_similarity = Similarity{ " " }
main_check_box_broken_files_audio = Ljud
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Arkiv
main_check_box_broken_files_image = Bild
check_button_general_same_size = Ignorera samma storlek
check_button_general_same_size_tooltip = Ignorera filer med samma storlek i resultat - vanligtvis är dessa 1:1 dubbletter
main_label_size_bytes_tooltip = Storlek på filer som kommer att användas vid skanning
# Upper window
upper_tree_view_included_folder_column_title = Mappar att söka
upper_tree_view_included_reference_column_title = Referens mappar
upper_recursive_button = Rekursiv
upper_recursive_button_tooltip = Om vald, sök även efter filer som inte placeras direkt under valda mappar.
upper_manual_add_included_button = Manuell Lägg till
upper_add_included_button = Lägg till
upper_remove_included_button = Ta bort
upper_manual_add_excluded_button = Manuell Lägg till
upper_add_excluded_button = Lägg till
upper_remove_excluded_button = Ta bort
upper_manual_add_included_button_tooltip =
    Lägg till katalognamn för att söka för hand.
    
    För att lägga till flera sökvägar samtidigt, separera dem med ;
    
    /home/roman;/home/rozkaz lägger till två kataloger /home/roman och /home/rozkaz
upper_add_included_button_tooltip = Lägg till ny katalog att söka.
upper_remove_included_button_tooltip = Ta bort katalog från sökning.
upper_manual_add_excluded_button_tooltip =
    Lägg till exkluderat katalognamn för hand.
    
    För att lägga till flera sökvägar samtidigt, separera dem med ;
    
    /home/roman;/home/krokiet kommer att lägga till två kataloger /home/roman och /home/keokiet
upper_add_excluded_button_tooltip = Lägg till katalog som ska exkluderas i sökningen.
upper_remove_excluded_button_tooltip = Ta bort katalog från utesluten.
upper_notebook_items_configuration = Objekt konfiguration
upper_notebook_excluded_directories = Uteslutna kataloger
upper_notebook_included_directories = Inkluderade kataloger
upper_allowed_extensions_tooltip =
    Tillåtna tillägg måste separeras med kommatecken (som standard alla är tillgängliga).
    
    Följande makron som lägger till flera tillägg samtidigt, finns också: IMAGE, VIDEO, MUSIC, TEXT.
    
    Användningsexempel ".exe, IMAGE, VIDEO, .rar, 7z" - det betyder att bilder (e. . jpg, png), videor (t.ex. avi, mp4), exe, rar, och 7z filer kommer att skannas.
upper_excluded_extensions_tooltip =
    Lista över inaktiverade filer som kommer att ignoreras i skanning.
    
    Vid användning av både tillåtna och inaktiverade tillägg har denna högre prioritet, så filen kommer inte att kontrolleras.
upper_excluded_items_tooltip =
    Exkluderade artiklar måste innehålla * jokertecken och bör separeras med kommatecken.
    Detta är långsammare än uteslutna kataloger, så använd det noggrant.
upper_excluded_items = Exkluderade objekt:
upper_allowed_extensions = Tillåtna tillägg:
upper_excluded_extensions = Inaktiverade tillägg:
# Popovers
popover_select_all = Radera
popover_unselect_all = Avmarkera alla
popover_reverse = Omvänd markering
popover_select_all_except_oldest = Välj alla utom äldsta
popover_select_all_except_newest = Välj alla utom nyaste
popover_select_one_oldest = Välj en äldsta
popover_select_one_newest = Välj en nyaste
popover_select_custom = Välj anpassad
popover_unselect_custom = Avmarkera anpassade
popover_select_all_images_except_biggest = Välj alla utom största
popover_select_all_images_except_smallest = Välj alla utom minsta
popover_custom_path_check_button_entry_tooltip =
    Välj poster efter sökväg.
    
    Exempel användning:
    /home/pimpek/rzecz.txt hittas med /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Välj poster efter filnamn.
    
    Exempel användning:
    /usr/ping/pong.txt finns med *ong*
popover_custom_regex_check_button_entry_tooltip =
    Välj poster efter specificerad Regex.
    
    Med detta läge är sökord sökväg med namn.
    
    Exempel användning:
    /usr/bin/ziemniak. xt kan hittas med /ziem[a-z]+
    
    Detta använder Rust regex-implementationen. Du kan läsa mer om det här: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Aktiverar skiftlägeskänslig detektion.
    
    När du inaktiverat /home/* hittar du både /HoMe/roman och /home/roman.
popover_custom_not_all_check_button_tooltip =
    Förhindrar att alla poster väljs i grupp.
    
    Detta är aktiverat som standard, eftersom i de flesta situationer, du inte vill ta bort både original och dubbletter filer, men vill lämna minst en fil.
    
    VARNING: Den här inställningen fungerar inte om du redan manuellt har valt alla resultat i en grupp.
popover_custom_regex_path_label = Sökväg
popover_custom_regex_name_label = Namn
popover_custom_regex_regex_label = Regex sökväg + namn
popover_custom_case_sensitive_check_button = Skiftlägeskänslighet
popover_custom_all_in_group_label = Välj inte alla poster i gruppen
popover_custom_mode_unselect = Avmarkera anpassad
popover_custom_mode_select = Välj anpassad
popover_sort_file_name = Filnamn
popover_sort_folder_name = Mapp namn
popover_sort_full_name = Fullständigt namn
popover_sort_size = Storlek
popover_sort_selection = Markerat
popover_invalid_regex = Regex är ogiltigt
popover_valid_regex = Regex är giltigt
# Bottom buttons
bottom_search_button = Sökning
bottom_select_button = Välj
bottom_delete_button = Radera
bottom_save_button = Save
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Flytta
bottom_sort_button = Sortera
bottom_search_button_tooltip = Starta sökning
bottom_select_button_tooltip = Välj poster. Endast valda filer/mappar kan senare bearbetas.
bottom_delete_button_tooltip = Ta bort markerade filer/mappar.
bottom_save_button_tooltip = Spara data om sökning till fil
bottom_symlink_button_tooltip =
    Skapa symboliska länkar.
    Fungerar endast när minst två resultat i en grupp väljs.
    Först är oförändrad och andra och senare är symanknutna till först.
bottom_hardlink_button_tooltip =
    Skapa hardlinks.
    Fungerar endast när minst två resultat i en grupp är valda.
    Först är oförändrad och andra och senare är hårt länkade till först.
bottom_hardlink_button_not_available_tooltip =
    Skapa hardlinks.
    Knappen är inaktiverad, eftersom hardlinks inte kan skapas.
    Hårdlänkar fungerar bara med administratörsrättigheter i Windows, så se till att köra appen som administratör.
    Om appen redan fungerar med sådana rättigheter kontrollera liknande problem på Github.
bottom_move_button_tooltip =
    Flyttar filer till vald katalog.
    Det kopierar alla filer till katalogen utan att bevara katalogträdet.
    När du försöker flytta två filer med identiskt namn till mappen kommer det andra att misslyckas och visa fel.
bottom_sort_button_tooltip = Sortera filer/mappar enligt vald metod.
bottom_show_errors_tooltip = Visa/Dölj undertextpanelen.
bottom_show_upper_notebook_tooltip = Visa/Dölj övre anteckningsbokspanelen.
# Progress Window
progress_stop_button = Stoppa
progress_stop_additional_message = Stoppa begärd
# About Window
about_repository_button_tooltip = Länk till utvecklingskatalogen med källkod.
about_donation_button_tooltip = Länk till donationssidan.
about_instruction_button_tooltip = Länk till instruktionssidan.
about_translation_button_tooltip = Länk till Crowdin sida med appöversättningar. Officiellt stöds polska och engelska.
about_repository_button = Filförråd
about_donation_button = Donationer
about_instruction_button = Instruktion
about_translation_button = Översättning
# Header
header_setting_button_tooltip = Öppnar dialogrutan för inställningar.
header_about_button_tooltip = Öppnar dialog med info om app.

# Settings


## General

settings_number_of_threads = Antal använda trådar
settings_number_of_threads_tooltip = Antal gängor, 0 betyder att alla gängor kommer att användas.
settings_use_rust_preview = Använd externa bibliotek istället gtk för att ladda förhandsvisningar
settings_use_rust_preview_tooltip =
    Att använda gtk-förhandsvisningar kommer ibland att vara snabbare och stödja fler format, men ibland kan det vara precis tvärtom.
    
    Om du har problem med att ladda förhandsvisningar, kan du försöka ändra den här inställningen.
    
    På icke-Linux-system rekommenderas att använda detta alternativ, eftersom gtk-pixbuf inte alltid är tillgänglig där så inaktivera detta alternativ kommer inte att ladda förhandsvisningar av vissa bilder.
settings_label_restart = Du måste starta om appen för att tillämpa inställningar!
settings_ignore_other_filesystems = Ignorera andra filsystem (endast Linux)
settings_ignore_other_filesystems_tooltip =
    ignorerar filer som inte finns i samma filsystem som sökta kataloger.
    
    Fungerar samma som -xdev alternativ för att hitta kommandot på Linux
settings_save_at_exit_button_tooltip = Spara konfigurationen till fil när appen stängs.
settings_load_at_start_button_tooltip =
    Ladda konfigurationen från filen när appen öppnas.
    
    Om den inte är aktiverad kommer standardinställningarna att användas.
settings_confirm_deletion_button_tooltip = Visa bekräftelsedialog när du klickar på knappen ta bort.
settings_confirm_link_button_tooltip = Visa bekräftelsedialog när du klickar på den hårda/symboliska länkknappen.
settings_confirm_group_deletion_button_tooltip = Visa varningsdialog när du försöker ta bort alla poster från gruppen.
settings_show_text_view_button_tooltip = Visa textpanelen längst ner i användargränssnittet.
settings_use_cache_button_tooltip = Använd filcache.
settings_save_also_as_json_button_tooltip = Spara cache till (läsbar) JSON-format. Det är möjligt att ändra dess innehåll. Cache från denna fil kommer att läsas automatiskt av appen om binärt format cache (med bin extension) saknas.
settings_use_trash_button_tooltip = Flyttar filer till papperskorgen istället ta bort dem permanent.
settings_language_label_tooltip = Språk för användargränssnitt.
settings_save_at_exit_button = Spara konfiguration när appen stängs
settings_load_at_start_button = Ladda konfiguration när appen öppnas
settings_confirm_deletion_button = Visa bekräftelsedialog vid borttagning av filer
settings_confirm_link_button = Visa bekräftelsedialog när hårda/symboliska länkar filer
settings_confirm_group_deletion_button = Visa bekräftelsedialog när alla filer tas bort i grupp
settings_show_text_view_button = Visa längst ned textpanel
settings_use_cache_button = Använd cache
settings_save_also_as_json_button = Spara även cache som JSON-fil
settings_use_trash_button = Flytta raderade filer till papperskorgen
settings_language_label = Language
settings_multiple_delete_outdated_cache_checkbutton = Ta bort föråldrade cache-poster automatiskt
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Ta bort föråldrade cacheresultat som pekar på obefintliga filer.
    
    När den är aktiverad, se till att appen när du laddar poster, att alla poster pekar på giltiga filer (trasiga dem ignoreras).
    
    Att inaktivera detta kommer att hjälpa när du skannar filer på externa enheter, så cacheposter om dem kommer inte att rensas i nästa skanning.
    
    När det gäller att ha hundratusentals poster i cache, det föreslås för att aktivera detta, vilket kommer att påskynda cache-inläsning/spara vid start/slut av sökningen.
settings_notebook_general = Info
settings_notebook_duplicates = Dubbletter
settings_notebook_images = Liknande bilder
settings_notebook_videos = Liknande video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Visar förhandsgranskning på höger sida (vid val av bildfil).
settings_multiple_image_preview_checkbutton = Visa förhandsgranskning av bild
settings_multiple_clear_cache_button_tooltip =
    Rensa cache manuellt för föråldrade poster.
    Detta bör endast användas om automatisk rensning har inaktiverats.
settings_multiple_clear_cache_button = Ta bort föråldrade resultat från cachen.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Döljer alla filer utom en, om alla pekar på samma data (är hardlinked).
    
    Exempel: I det fall där det finns (på disk) sju filer som är hårdkopplade till specifika data och en annan fil med samma data men ett annat inode, i dubblettsökare, kommer endast en unik fil och en fil från hårdlänkade att visas.
settings_duplicates_minimal_size_entry_tooltip =
    Ange minimal filstorlek som kommer att cachelagras.
    
    Att välja ett mindre värde kommer att generera fler poster. Detta kommer att snabba upp sökningen, men bromsa cache-laddning/spara.
settings_duplicates_prehash_checkbutton_tooltip =
    Aktiverar cachelagring av prehash (en hash beräknad från en liten del av filen) vilket tillåter tidigare avfärdande av icke-duplicerade resultat.
    
    Det är inaktiverat som standard eftersom det kan orsaka nedgångar i vissa situationer.
    
    Det rekommenderas starkt att använda det när du skannar hundratusentals eller miljoner filer, eftersom det kan påskynda sökningen flera gånger.
settings_duplicates_prehash_minimal_entry_tooltip = Minimal storlek på cachad post.
settings_duplicates_hide_hard_link_button = Dölj hårda länkar (endast Linux och macOS)
settings_duplicates_prehash_checkbutton = Använd prehash cache
settings_duplicates_minimal_size_cache_label = Minimal storlek på filer (i bytes) sparade i cache
settings_duplicates_minimal_size_cache_prehash_label = Minimal storlek på filer (i bytes) sparade för att kunna använda cache

## Saving/Loading settings

settings_saving_button_tooltip = Spara konfigurationen för nuvarande inställningar till filen.
settings_loading_button_tooltip = Ladda inställningar från fil och ersätta den aktuella konfigurationen med dem.
settings_reset_button_tooltip = Återställ den aktuella konfigurationen till standardkonfigurationen.
settings_saving_button = Spara konfiguration
settings_loading_button = Ladda konfiguration
settings_reset_button = Återställ konfiguration

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Öppnar mappen där cache-txt-filer lagras.
    
    Ändring av cache-filer kan leda till att ogiltiga resultat visas. Dock kan ändra sökvägen spara tid när du flyttar en stor mängd filer till en annan plats.
    
    Du kan kopiera dessa filer mellan datorer för att spara tid på skanning igen för filer (naturligtvis om de har liknande katalogstruktur).
    
    Vid problem med cachen kan dessa filer tas bort. Appen kommer automatiskt att regenerera dem.
settings_folder_settings_open_tooltip =
    Öppnar mappen där Czkawka-konfigurationen lagras.
    
    VARNING: Manuellt modifierande av konfigurationen kan bryta ditt arbetsflöde.
settings_folder_cache_open = Öppna cachemapp
settings_folder_settings_open = Öppna inställningsmapp
# Compute results
compute_stopped_by_user = Sökandet stoppades av användaren
compute_found_duplicates_hash_size = Hittade { $number_files } dubbletter i { $number_groups } grupper som tog { $size }
compute_found_duplicates_name = Hittade { $number_files } dubbletter i { $number_groups } grupper
compute_found_empty_folders = Hittade { $number_files } tomma mappar
compute_found_empty_files = Hittades { $number_files } tomma filer
compute_found_big_files = Hittade { $number_files } stora filer
compute_found_temporary_files = Hittade { $number_files } tillfälliga filer
compute_found_images = Hittade { $number_files } liknande bilder i { $number_groups } grupper
compute_found_videos = Hittade { $number_files } liknande videoklipp i { $number_groups } grupper
compute_found_music = Hittade { $number_files } liknande musik filer i { $number_groups } grupper
compute_found_invalid_symlinks = Hittade { $number_files } ogiltiga symboliska länkar
compute_found_broken_files = Hittades { $number_files } trasiga filer
compute_found_bad_extensions = Hittades { $number_files } filer med ogiltiga tillägg
# Progress window
progress_scanning_general_file = Scanning { $file_number } fil
progress_scanning_extension_of_files = Kontrollerar tillägg till { $file_checked }/{ $all_files } fil
progress_scanning_broken_files = Kontrollerar { $file_checked }/{ $all_files } fil
progress_scanning_video = Hashning av { $file_checked }/{ $all_files } video
progress_scanning_image = Hashning av { $file_checked }/{ $all_files } bild
progress_comparing_image_hashes = Jämföra { $file_checked }/{ $all_files } bildhash
progress_scanning_music_tags_end = Jämföra taggar för { $file_checked }/{ $all_files } musikfil
progress_scanning_music_tags = Lästaggar för { $file_checked }/{ $all_files } musikfil
progress_scanning_music_content_end = Jämföra fingeravtryck av { $file_checked }/{ $all_files } musikfil
progress_scanning_music_content = Beräknar fingeravtryck av { $file_checked }/{ $all_files } musikfil
progress_scanning_empty_folders = Skannar { $folder_number } mappen
progress_scanning_size = Skannar storleken på { $file_number } fil
progress_scanning_size_name = Skannar namn och storlek på { $file_number } fil
progress_scanning_name = Skannar namn på { $file_number } fil
progress_analyzed_partial_hash = Analyserade partiella hash av { $file_checked }/{ $all_files } filer
progress_analyzed_full_hash = Analyserad full hash av { $file_checked }/{ $all_files } filer
progress_prehash_cache_loading = Laddar prehash cache
progress_prehash_cache_saving = Sparar Omfattande cache
progress_hash_cache_loading = Laddar hash-cache
progress_hash_cache_saving = Sparar hash-cache
progress_cache_loading = Laddar cache
progress_cache_saving = Sparar cache
progress_current_stage = Nuvarande steg:{ " " }
progress_all_stages = Alla etapper:{ " "  }
# Saving loading 
saving_loading_saving_success = Sparad konfiguration till filen { $name }.
saving_loading_saving_failure = Det gick inte att spara konfigurationsdata till filen { $name }.
saving_loading_reset_configuration = Aktuell konfiguration har rensats.
saving_loading_loading_success = Korrekt laddad app-konfiguration.
saving_loading_invalid_string = För nyckel "{ $key }" hittade ogiltigt resultat - "{ $result }" vilket inte är en sträng.
saving_loading_invalid_int = För nyckeln "{ $key }" hittade ogiltigt resultat - "{ $result }" vilket inte är ett heltal.
saving_loading_invalid_bool = För nyckel "{ $key }" hittade ogiltigt resultat - "{ $result }" vilket inte är en bool.
saving_loading_decode_problem_bool = Det gick inte att avkoda Bollen från nyckel "{ $key }" hittade "{ $result }" men tillåtna värden är 0, 1, sanna eller falska.
saving_loading_saving_same_keys = Försöker spara inställningen med duplicerad nyckel "{ $key }".
saving_loading_failed_to_get_home_directory = Det gick inte att hämta hemkatalogen till att öppna/spara konfigurationsfil.
saving_loading_folder_config_instead_file = Kan inte skapa eller öppna spara konfigurationsfilen i sökvägen "{ $path }" eftersom det redan finns en mapp.
saving_loading_failed_to_create_configuration_folder = Det gick inte att skapa konfigurationsmappen "{ $path }", orsak "{ $reason }".
saving_loading_failed_to_create_config_file = Det gick inte att skapa konfigurationsfil "{ $path }", orsak "{ $reason }".
saving_loading_failed_to_read_config_file = Kan inte ladda konfiguration från "{ $path }" eftersom den inte finns eller inte är en fil.
saving_loading_failed_to_read_data_from_file = Kan inte läsa data från fil "{ $path }", anledning "{ $reason }".
saving_loading_orphan_data = Hittade föräldralösa data "{ $data }" i rad "{ $line }".
saving_loading_not_valid = Inställningen "{ $data }" finns inte i nuvarande appversion.
# Invalid symlinks
invalid_symlink_infinite_recursion = Oändlig recursion
invalid_symlink_non_existent_destination = Icke-existerande målfil
# Other
selected_all_reference_folders = Kan inte börja söka, när alla kataloger är inställda som referensmappar
searching_for_data = Söker data, det kan ta en stund, vänta...
text_view_messages = MEDDELANDEN
text_view_warnings = VARNINGAR
text_view_errors = FEL
about_window_motto = Detta program är gratis att använda och kommer alltid att vara.
# Various dialog
dialogs_ask_next_time = Fråga nästa gång
delete_file_failed = Det gick inte att ta bort filen { $name } varför { $reason }
delete_title_dialog = Ta bort bekräftelse
delete_question_label = Är du säker på att du vill ta bort filer?
delete_all_files_in_group_title = Bekräftelse av att ta bort alla filer i grupp
delete_all_files_in_group_label1 = I vissa grupper är alla poster valda.
delete_all_files_in_group_label2 = Är du säker på att du vill radera dem?
delete_folder_failed = Det gick inte att ta bort mappen { $dir } eftersom mappen inte existerar, du har inte behörighet eller mappen är inte tom.
delete_items_label = { $items } filer kommer att tas bort.
delete_items_groups_label = { $items } filer från { $groups } grupper kommer att raderas.
hardlink_failed = Det gick inte att hardlink
hard_sym_invalid_selection_title_dialog = Ogiltigt val med vissa grupper
hard_sym_invalid_selection_label_1 = I vissa grupper finns det bara en post vald och den kommer att ignoreras.
hard_sym_invalid_selection_label_2 = För att kunna länka dessa filer måste minst två resultat i gruppen väljas.
hard_sym_invalid_selection_label_3 = Först i grupp känns igen som original och ändras inte, men andra och senare ändras.
hard_sym_link_title_dialog = Länkbekräftelse
hard_sym_link_label = Är du säker på att du vill länka dessa filer?
move_folder_failed = Det gick inte att flytta mappen { $name } anledning { $reason }
move_file_failed = Det gick inte att flytta filen { $name } anledning { $reason }
move_files_title_dialog = Välj mapp som du vill flytta duplicerade filer till
move_files_choose_more_than_1_path = Endast en sökväg kan väljas för att kunna kopiera sina duplicerade filer, valda { $path_number }.
move_stats = Korrekt flyttad { $num_files }/{ $all_files } objekt
save_results_to_file = Sparade resultat både till txt och json filer i { $name } mapp.
search_not_choosing_any_music = FEL: Du måste välja minst en kryssruta med söktyper för musik.
search_not_choosing_any_broken_files = FEL: Du måste välja minst en kryssruta med typ av markerade trasiga filer.
include_folders_dialog_title = Mappar att inkludera
exclude_folders_dialog_title = Mappar att exkludera
include_manually_directories_dialog_title = Lägg till katalog manuellt
cache_properly_cleared = Rensad cache
cache_clear_duplicates_title = Rensar dubbletter cache
cache_clear_similar_images_title = Rensar liknande bildcache
cache_clear_similar_videos_title = Rensar liknande videoklipp cache
cache_clear_message_label_1 = Vill du rensa cachen för föråldrade inlägg?
cache_clear_message_label_2 = Denna åtgärd kommer att ta bort alla cache-poster som pekar på ogiltiga filer.
cache_clear_message_label_3 = Detta kan något speedup ladda/spara till cache.
cache_clear_message_label_4 = VARNING: Åtgärden kommer att ta bort alla cachade data från frånkopplade externa enheter. Så varje hash kommer att behöva regenereras.
# Show preview
preview_image_resize_failure = Kunde inte ändra storlek på bild { $name }.
preview_image_opening_failure = Det gick inte att öppna bilden { $name } skäl { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupp { $current_group }/{ $all_groups } ({ $images_in_group } bilder)
compare_move_left_button = L
compare_move_right_button = R
