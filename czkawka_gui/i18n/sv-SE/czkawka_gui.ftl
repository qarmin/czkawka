# Window titles
window_settings_title = Inställningar
window_main_title = Czkawka (Hicka)
window_progress_title = Skannar
window_compare_images = Jämför bilder
# General
general_ok_button = Ok
general_close_button = Stäng
# Krokiet info dialog
krokiet_info_title = Avskrivningsmeddelande
krokiet_info_message =
    Czkawka GTK 12.0 är den slutliga utgåvan. Inga ytterligare uppdateringar, funktioner eller buggfixar planeras.
    
    De flesta funktioner från Czkawka GTK finns i Krokiet, oftast i en enklare, snabbare och stabilare form. Krokiet lägger också till nya funktioner och förbättringar som inte var möjliga i GTK-versionen.
    
    Om du fortfarande använder Czkawka GTK, bör det vara lätt att byta till Krokiet, eftersom det har ett liknande gränssnitt, färre beroenden och bättre plattformsoberoende stöd.
    
    PS: Detta meddelande bör bara visas en gång. Om det dyker upp igen, sätt miljövariabeln CZKAWKA_DONT_ANNOY_ME till valfritt icke-tomt värde.
# Main window
music_title_checkbox = Titel
music_artist_checkbox = Artist
music_year_checkbox = År
music_bitrate_checkbox = Bithastighet
music_genre_checkbox = Genre
music_length_checkbox = Längd
music_comparison_checkbox = Ungefärlig jämförelse
music_checking_by_tags = Taggar
music_checking_by_content = Innehåll
same_music_seconds_label = Minsta fragmentlängd i sekunder
same_music_similarity_label = Maximal skillnad
music_compare_only_in_title_group = Jämför inom grupper med liknande titlar
music_compare_only_in_title_group_tooltip =
    När detta är aktiverat grupperas filerna efter titel och jämförs sedan med varandra.
    
    Med 10 000 filer blir det vanligtvis runt 20 000 jämförelser i stället för nästan 100 miljoner.
same_music_tooltip =
    Sökning efter liknande musikfiler genom deras innehåll kan konfigureras genom att ställa in:
    
    - Minsta fragmenttid efter vilken musikfiler kan identifieras som liknande
    - Maximal skillnad mellan två testade fragment
    
    Nyckeln till goda resultat är att hitta förnuftiga kombinationer av dessa parametrar, förutsatt att de är tillgängliga.
    
    Om den minsta tiden sätts till 5 s och den maximala skillnaden till 1,0 söker programmet efter nästan identiska fragment i filerna.
    En tid på 20 s och en maximal skillnad på 6,0 fungerar däremot bra för att hitta remixer, liveversioner osv.
    
    Som standard jämförs varje musikfil med alla andra. Det kan ta lång tid när många filer testas, så det är oftast bättre att använda referensmappar och ange vilka filer som ska jämföras med varandra (med samma antal filer går det minst 4 gånger snabbare att jämföra fingeravtryck med referensmappar än utan).
music_comparison_checkbox_tooltip =
    Den söker efter liknande musikfiler med AI, som använder maskininlärning för att ta bort parenteser ur en fras. När det här alternativet är aktiverat betraktas till exempel följande filer som dubbletter:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Skiftlägeskänslig
duplicate_case_sensitive_name_tooltip =
    När detta är aktiverat grupperas endast poster med exakt samma namn, t.ex. Żołd <-> Żołd
    
    Om detta inaktiveras grupperas namn utan hänsyn till versaler/gemener, t.ex. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Storlek och namn
duplicate_mode_name_combo_box = Namn
duplicate_mode_size_combo_box = Storlek
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka erbjuder 3 typer av hash:
    
    Blake3 - kryptografisk hashfunktion. Detta är standard eftersom det är mycket snabbt.
    
    CRC32 - enkel hashfunktion. Detta bör vara snabbare än Blake3, men kan mycket sällan ha några kollisioner.
    
    XXH3 - mycket lik i prestanda och hashkvalitet till Blake3 (men icke-kryptografisk). Så, sådana lägen kan lätt bytas ut.
duplicate_check_method_tooltip =
    Czkawka erbjuder för närvarande tre metoder för att hitta dubbletter:
    
    Namn - Hittar filer som har samma namn.
    
    Storlek - Hittar filer som har samma storlek.
    
    Hash - Hittar filer som har samma innehåll. Detta läge hashar filen och jämför sedan hashvärdet för att hitta dubbletter. Detta läge är det säkraste sättet att hitta dubbletter. Appen använder cache i hög grad, så andra och ytterligare skanningar av samma data bör vara mycket snabbare än den första.
image_hash_size_tooltip =
    Varje kontrollerad bild får en särskild hash som kan jämföras med andra hashar. En liten skillnad mellan dem innebär att bilderna är lika.
    
    Hashstorleken 8 är ganska bra för att hitta bilder som bara är lite lika originalet. Med en större bilduppsättning (>1000) ger detta många falska positiva resultat, så i sådana fall rekommenderas en större hashstorlek.
    
    16 är standardstorleken för hashar och är en ganska bra kompromiss mellan att hitta även lite lika bilder och att bara få ett litet antal hashkollisioner.
    
    Hashstorlekarna 32 och 64 hittar bara mycket lika bilder, men bör nästan inte ge några falska positiva resultat (möjligen förutom vissa bilder med alfakanal).
image_resize_filter_tooltip =
    För att beräkna bildens hash måste biblioteket först ändra storlek på bilden.
    
    Beroende på vald algoritm kommer den bild som används för att beräkna hashen att se lite olika ut.
    
    Den snabbaste algoritmen, men också den som ger sämst resultat, är Närmaste. Den är aktiverad som standard eftersom den lägre kvaliteten inte märks särskilt mycket med hashstorleken 16 × 16.
    
    Med hashstorleken 8 × 8 rekommenderas en annan algoritm än Närmaste för att få bättre grupper av bilder.
image_hash_alg_tooltip =
    Du kan välja mellan flera algoritmer för att beräkna hashvärdet.
    
    Alla har både styrkor och svagheter och kan ge bättre eller sämre resultat för olika bilder.
    
    För att avgöra vilken som passar dig bäst krävs därför manuell testning.
image_geometric_invariance_tooltip = Jämför också spegelvända/vända och valfritt roterade varianter av varje bild. Detta förbättrar matchningen men ökar hashtiden.
big_files_mode_combobox_tooltip = Gör det möjligt att söka efter minsta/största filer
big_files_mode_label = Markerade filer
big_files_mode_smallest_combo_box = Den minsta
big_files_mode_biggest_combo_box = Den största
main_notebook_duplicates = Dubblettfiler
main_notebook_empty_directories = Tomma mappar
main_notebook_big_files = Stora filer
main_notebook_empty_files = Tomma filer
main_notebook_temporary = Tillfälliga filer
main_notebook_similar_images = Liknande bilder
main_notebook_similar_videos = Liknande videor
main_notebook_same_music = Musikdubbletter
main_notebook_symlinks = Ogiltiga symboliska länkar
main_notebook_broken_files = Trasiga filer
main_notebook_bad_extensions = Felaktiga filändelser
main_tree_view_column_file_name = Filnamn
main_tree_view_column_folder_name = Mappens namn
main_tree_view_column_path = Sökväg
main_tree_view_column_modification = Ändringsdatum
main_tree_view_column_size = Storlek
main_tree_view_column_similarity = Likhet
main_tree_view_column_dimensions = Dimensioner
main_tree_view_column_title = Titel
main_tree_view_column_artist = Artist
main_tree_view_column_year = År
main_tree_view_column_bitrate = Bithastighet
main_tree_view_column_length = Längd
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = Namn på symbolisk länk
main_tree_view_column_symlink_folder = Mapp för symboliska länkar
main_tree_view_column_destination_path = Målsökvägen
main_tree_view_column_type_of_error = Typ av fel
main_tree_view_column_current_extension = Nuvarande filändelse
main_tree_view_column_proper_extensions = Korrekt filändelse
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Kodek
main_label_check_method = Kontrollmetod
main_label_hash_type = Hashtyp
main_label_hash_size = Hashstorlek
main_label_geometric_invariance = Geometrisk invarians
main_label_size_bytes = Storlek (bytes)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Antal visade filer
main_label_resize_algorithm = Ändra storlek på algoritm
main_label_similarity = Likhet{ "   " }
main_check_box_broken_files_audio = Ljud
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Arkiv
main_check_box_broken_files_image = Bild
main_check_box_broken_files_video = Video
main_check_box_broken_files_video_tooltip = Använder ffmpeg/ffprobe för att validera videofiler. Ganska långsam och kan detektera pedantiska fel även om filen spelas upp korrekt.
check_button_general_same_size = Ignorera samma storlek
check_button_general_same_size_tooltip = Ignorera filer med samma storlek i resultat - vanligtvis är dessa 1:1 dubbletter
main_label_size_bytes_tooltip = Storlek på filer som kommer att användas vid skanning
# Upper window
upper_tree_view_included_folder_column_title = Mappar att söka
upper_tree_view_included_reference_column_title = Referensmappar
upper_recursive_button = Rekursiv
upper_recursive_button_tooltip = Om detta är markerat söker programmet även efter filer som inte ligger direkt under de valda mapparna.
upper_manual_add_included_button = Lägg till manuellt
upper_add_included_button = Lägg till
upper_remove_included_button = Ta bort
upper_manual_add_excluded_button = Lägg till manuellt
upper_add_excluded_button = Lägg till
upper_remove_excluded_button = Ta bort
upper_manual_add_included_button_tooltip =
    Lägg till en mapp att söka i manuellt.
    
    Om du vill lägga till flera sökvägar samtidigt, separera dem med ;
    
    /home/roman;/home/rozkaz lägger till två mappar: /home/roman och /home/rozkaz
upper_add_included_button_tooltip = Lägg till en ny mapp att söka i.
upper_remove_included_button_tooltip = Ta bort mapp från sökningen.
upper_manual_add_excluded_button_tooltip =
    Lägg till en exkluderad mapp manuellt.
    
    Om du vill lägga till flera sökvägar samtidigt, separera dem med ;
    
    /home/roman;/home/krokiet lägger till två mappar: /home/roman och /home/krokiet
upper_add_excluded_button_tooltip = Lägg till en mapp som ska exkluderas från sökningen.
upper_remove_excluded_button_tooltip = Ta bort mapp från exkluderade.
upper_notebook_items_configuration = Objektkonfiguration
upper_notebook_excluded_directories = Exkluderade sökvägar
upper_notebook_included_directories = Inkluderade sökvägar
upper_allowed_extensions_tooltip =
    Tillåtna filändelser måste separeras med kommatecken (som standard är alla tillgängliga).
    
    Följande makron, som lägger till flera filändelser samtidigt, finns också: IMAGE, VIDEO, MUSIC, TEXT.
    
    Exempel: ”.exe, IMAGE, VIDEO, .rar, 7z” innebär att bilder (t.ex. jpg, png), videor (t.ex. avi, mp4) samt exe-, rar- och 7z-filer genomsöks.
upper_excluded_extensions_tooltip =
    Lista över inaktiverade filändelser som ignoreras vid genomsökning.
    
    När både tillåtna och inaktiverade filändelser används har denna lista högre prioritet, så filen kontrolleras inte.
upper_excluded_items_tooltip =
    Exkluderade objekt måste innehålla jokertecknet * och separeras med kommatecken.
    Detta är långsammare än exkluderade sökvägar, så använd det med försiktighet.
upper_excluded_items = Exkluderade objekt:
upper_allowed_extensions = Tillåtna filändelser:
upper_excluded_extensions = Inaktiverade filändelser:
# Popovers
popover_select_all = Markera alla
popover_unselect_all = Avmarkera alla
popover_reverse = Omvänd markering
popover_select_all_except_shortest_path = Markera alla utom den kortaste sökvägen
popover_select_all_except_longest_path = Markera alla utom den längsta sökvägen
popover_select_all_except_oldest = Markera alla utom den äldsta
popover_select_all_except_newest = Markera alla utom den nyaste
popover_select_one_oldest = Markera den äldsta
popover_select_one_newest = Markera den nyaste
popover_select_custom = Anpassad markering
popover_unselect_custom = Anpassad avmarkering
popover_select_all_images_except_biggest = Markera alla utom den största
popover_select_all_images_except_smallest = Markera alla utom den minsta
popover_custom_path_check_button_entry_tooltip =
    Markera poster efter sökväg.
    
    Exempel på användning:
    /home/pimpek/rzecz.txt hittas med /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Markera poster efter filnamn.
    
    Exempel på användning:
    /usr/ping/pong.txt hittas med *ong*
popover_custom_regex_check_button_entry_tooltip =
    Markera poster med angivet regex.
    
    I det här läget är söktexten sökväg plus namn.
    
    Exempel på användning:
    /usr/bin/ziemniak.txt hittas med /ziem[a-z]+
    
    Det här använder Rusts standardimplementation för regex. Du kan läsa mer här: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Aktiverar skiftlägeskänslig detektion.
    
    När du inaktiverat /home/* hittar du både /HoMe/roman och /home/roman.
popover_custom_not_all_check_button_tooltip =
    Förhindrar att alla poster i gruppen markeras.
    
    Detta är aktiverat som standard eftersom du i de flesta situationer inte vill ta bort både originalfilen och dubblettfilerna, utan lämna kvar minst en fil.
    
    VARNING: Den här inställningen fungerar inte om du redan har markerat alla resultat i en grupp manuellt.
popover_custom_regex_path_label = Sökväg
popover_custom_regex_name_label = Namn
popover_custom_regex_regex_label = Regex sökväg + namn
popover_custom_case_sensitive_check_button = Skiftlägeskänslighet
popover_custom_all_in_group_label = Markera inte alla poster i gruppen
popover_custom_mode_unselect = Anpassad avmarkering
popover_custom_mode_select = Anpassad markering
popover_sort_file_name = Filnamn
popover_sort_folder_name = Mappnamn
popover_sort_full_name = Fullständigt namn
popover_sort_size = Storlek
popover_sort_selection = Markerat
popover_invalid_regex = Regex är ogiltigt
popover_valid_regex = Regex är giltigt
# Bottom buttons
bottom_search_button = Sök
bottom_select_button = Markera
bottom_delete_button = Ta bort
bottom_save_button = Spara
bottom_symlink_button = Symbolisk länk
bottom_hardlink_button = Hårdlänk
bottom_move_button = Flytta
bottom_sort_button = Sortera
bottom_compare_button = Jämför
bottom_search_button_tooltip = Starta sökning
bottom_select_button_tooltip = Markera poster. Endast markerade filer/mappar kan bearbetas senare.
bottom_delete_button_tooltip = Ta bort markerade filer/mappar.
bottom_save_button_tooltip = Spara data om sökning till fil
bottom_symlink_button_tooltip =
    Skapa symboliska länkar.
    Fungerar endast när minst två resultat i en grupp är markerade.
    Den första filen lämnas oförändrad och övriga markerade filer ersätts med symboliska länkar till den första.
bottom_hardlink_button_tooltip =
    Skapa hårdlänkar.
    Fungerar endast när minst två resultat i en grupp är markerade.
    Den första filen lämnas oförändrad och övriga markerade filer ersätts med hårdlänkar till den första.
bottom_hardlink_button_not_available_tooltip =
    Skapa hårdlänkar.
    Knappen är inaktiverad eftersom hårdlänkar inte kan skapas.
    I Windows krävs administratörsbehörighet för att skapa hårdlänkar. Se därför till att köra appen som administratör.
    Om appen redan körs med sådan behörighet kan du kontrollera om det finns liknande problem på GitHub.
bottom_move_button_tooltip =
    Flyttar filer till en vald mapp.
    Alla filer kopieras till mappen utan att mappstrukturen bevaras.
    Om två filer med samma namn flyttas till samma mapp misslyckas den andra och ett fel visas.
bottom_sort_button_tooltip = Sorterar filer/mappar enligt vald metod.
bottom_compare_button_tooltip = Jämför bilder i gruppen.
bottom_show_errors_tooltip = Visa/dölj undertextpanelen.
bottom_show_upper_notebook_tooltip = Visa/dölj övre anteckningsbokspanelen.
# Progress Window
progress_stop_button = Stoppa
progress_stop_additional_message = Stoppa begärd
# About Window
about_repository_button_tooltip = Länk till källkodsarkivet med källkod.
about_donation_button_tooltip = Länk till donationssidan.
about_instruction_button_tooltip = Länk till instruktionssidan.
about_translation_button_tooltip = Länk till Crowdin sida med appöversättningar. Officiellt stöds polska och engelska.
about_repository_button = Källkodsarkiv
about_donation_button = Donation
about_instruction_button = Instruktion
about_translation_button = Översättning
# Header
header_setting_button_tooltip = Öppnar dialogrutan för inställningar.
header_about_button_tooltip = Öppnar dialog med info om app.
header_krokiet_button_tooltip = Prova Krokiet - den nya och förbättrade versionen!
# Krokiet promo dialog
krokiet_promo_title = Möt Krokiet!
krokiet_promo_message =
    Hallå där, modiga Czkawka användare!
    
    Kraften är helt klart med dig, men Krokiet är inte - en nyare, snabbare, lättare, och betydligt mer stilig (förutsatt att appar faktiskt kan vara snygga) dubbla renare.
    
    Krokiet innehåller allt folk gillade om Czkawka. Det är helt gratis, öppen källkod, har en unik och enkel UI (både prisas och hatas av många), introducerar en hel del nya funktioner, använder färre beroenden och fungerar mycket mer tillförlitligt över olika plattformar.
    
    Och om du på något sätt missade det, det finns redan en ännu nyare app än Krokiet - Cedinia, utformad främst för Android-enheter och pekskärmsanvändning.
    
    Czkawka GTK betjänade oss väl, men dess klocka har upphört.
krokiet_promo_link_download = Ladda ner Krokiet/Cedinia
krokiet_promo_link_project = Projekt sida

# Settings


## General

settings_number_of_threads = Antal använda trådar
settings_number_of_threads_tooltip = Antal trådar som används. 0 innebär att alla tillgängliga trådar används.
settings_use_rust_preview = Använd externa bibliotek i stället för gtk för att läsa in förhandsgranskningar
settings_use_rust_preview_tooltip =
    Att använda gtk-förhandsvisningar kommer ibland att vara snabbare och stödja fler format, men ibland kan det vara precis tvärtom.
    
    Om du har problem med att ladda förhandsvisningar, kan du försöka ändra den här inställningen.
    
    På icke-Linux-system rekommenderas att använda detta alternativ, eftersom gtk-pixbuf inte alltid är tillgänglig där så inaktivera detta alternativ kommer inte att ladda förhandsvisningar av vissa bilder.
settings_label_restart = Du måste starta om appen för att tillämpa inställningar!
settings_ignore_other_filesystems = Ignorera andra filsystem (endast Linux)
settings_ignore_other_filesystems_tooltip =
    ignorerar filer som inte finns på samma filsystem som de genomsökta mapparna.
    
    Fungerar på samma sätt som alternativet -xdev i kommandot find på Linux
settings_save_at_exit_button_tooltip = Spara konfigurationen till fil när appen stängs.
settings_load_at_start_button_tooltip =
    Ladda konfigurationen från fil när appen öppnas.
    
    Om funktionen inte är aktiverad används standardinställningarna.
settings_confirm_deletion_button_tooltip = Visa bekräftelsedialog när du klickar på knappen ta bort.
settings_confirm_link_button_tooltip = Visa en bekräftelsedialog när du klickar på knappen för hårdlänkar/symboliska länkar.
settings_confirm_group_deletion_button_tooltip = Visa varningsdialog när du försöker ta bort alla poster från gruppen.
settings_show_text_view_button_tooltip = Visa textpanelen längst ner i användargränssnittet.
settings_use_cache_button_tooltip = Använd filcache.
settings_save_also_as_json_button_tooltip = Spara cachen i JSON-format (läsbart för människor). Det går att ändra innehållet. Cachen från den här filen läses automatiskt in av appen om cachen i binärt format (med bin som filändelse) saknas.
settings_use_trash_button_tooltip = Flyttar filer till papperskorgen i stället för att ta bort dem permanent.
settings_language_label_tooltip = Språk för användargränssnitt.
settings_save_at_exit_button = Spara konfiguration när appen stängs
settings_load_at_start_button = Ladda konfiguration när appen öppnas
settings_confirm_deletion_button = Visa bekräftelsedialog vid borttagning av filer
settings_confirm_link_button = Visa bekräftelsedialog när hårdlänkar/symboliska länkar skapas för filer
settings_confirm_group_deletion_button = Visa bekräftelsedialog när alla filer tas bort i grupp
settings_show_text_view_button = Visa längst ned textpanel
settings_use_cache_button = Använd cache
settings_save_also_as_json_button = Spara även cache som JSON-fil
settings_use_trash_button = Flytta borttagna filer till papperskorgen
settings_language_label = Språk
settings_multiple_delete_outdated_cache_checkbutton = Ta bort föråldrade cacheposter automatiskt
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Ta bort föråldrade cacheresultat som pekar på filer som inte finns.
    
    När detta är aktiverat kontrollerar appen vid inläsning att alla poster pekar på giltiga filer (trasiga poster tas bort från cachen).
    
    Om du genomsöker filer på externa enheter kan du inaktivera detta för att undvika att deras cacheposter rensas vid nästa genomsökning.
    
    Om cachen innehåller hundratusentals poster rekommenderas detta, eftersom det gör inläsning och sparande av cache snabbare vid start och slut av genomsökningen.
settings_notebook_general = Allmänt
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
    Döljer alla filer utom en om de pekar på samma data (är hårdlänkade).
    
    Exempel: Om det finns sju filer (på disken) som är hårdlänkade till samma data och en annan fil med samma data men en annan inode, visar dubblettsökaren bara två filer: den unika filen och en av de hårdlänkade filerna.
settings_duplicates_minimal_size_entry_tooltip =
    Ange minimal filstorlek som kommer att cachelagras.
    
    Att välja ett mindre värde kommer att generera fler poster. Detta kommer att snabba upp sökningen, men göra cacheinläsning och cachesparande långsammare.
settings_duplicates_prehash_checkbutton_tooltip =
    Aktiverar cachelagring av prehash (en hash som beräknas från en liten del av filen), så att resultat som inte är dubbletter kan uteslutas tidigare.
    
    Det är inaktiverat som standard eftersom det kan orsaka prestandaförsämringar i vissa situationer.
    
    Det rekommenderas starkt när du genomsöker hundratusentals eller miljontals filer, eftersom det kan göra sökningen flera gånger snabbare.
settings_duplicates_prehash_minimal_entry_tooltip = Minimal storlek på cachad post.
settings_duplicates_hide_hard_link_button = Dölj hårdlänkar
settings_duplicates_prehash_checkbutton = Använd prehash-cache
settings_duplicates_minimal_size_cache_label = Minsta storlek på filer (i byte) som sparas i cache
settings_duplicates_minimal_size_cache_prehash_label = Minsta storlek på filer (i byte) som sparas i prehash-cache

## Saving/Loading settings

settings_saving_button_tooltip = Spara den aktuella inställningskonfigurationen till en fil.
settings_loading_button_tooltip = Ladda inställningar från fil och ersätt den aktuella konfigurationen med dem.
settings_reset_button_tooltip = Återställ den aktuella konfigurationen till standardkonfigurationen.
settings_saving_button = Spara konfiguration
settings_loading_button = Ladda konfiguration
settings_reset_button = Återställ konfiguration

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Öppnar mappen där cachefilerna lagras som txt-filer.
    
    Om du ändrar cachefilerna kan ogiltiga resultat visas. Att ändra sökvägarna kan dock spara tid när ett stort antal filer flyttas till en annan plats.
    
    Du kan kopiera dessa filer mellan datorer för att slippa söka igenom filerna på nytt (förutsatt att de har en liknande mappstruktur).
    
    Vid problem med cachen kan dessa filer tas bort. Appen återskapar dem automatiskt.
settings_folder_settings_open_tooltip =
    Öppnar mappen där Czkawka-konfigurationen lagras.
    
    VARNING: Om du ändrar konfigurationen manuellt kan det störa ditt arbetsflöde.
settings_folder_cache_open = Öppna cachemapp
settings_folder_settings_open = Öppna inställningsmapp
# Compute results
compute_stopped_by_user = Sökningen stoppades av användaren
compute_found_duplicates_hash_size = Hittade { $number_files } dubbletter i { $number_groups } grupper som tar upp { $size } på { $time }
compute_found_duplicates_name = Hittade { $number_files } dubbletter i { $number_groups } grupper på { $time }
compute_found_empty_folders = Hittade { $number_files } tomma mappar på { $time }
compute_found_empty_files = Hittade { $number_files } tomma filer på { $time }
compute_found_big_files = Hittade { $number_files } stora filer på { $time }
compute_found_temporary_files = Hittade { $number_files } temporära filer på { $time }
compute_found_images = Hittade { $number_files } liknande bilder i { $number_groups } grupper på { $time }
compute_found_videos = Hittade { $number_files } liknande videor i { $number_groups } grupper på { $time }
compute_found_music = Hittade { $number_files } liknande musikfiler i { $number_groups } grupper på { $time }
compute_found_invalid_symlinks = Hittade { $number_files } ogiltiga symboliska länkar på { $time }
compute_found_broken_files = Hittade { $number_files } trasiga filer på { $time }
compute_found_bad_extensions = Hittade { $number_files } filer med ogiltiga filändelser på { $time }
# Progress window
progress_current_stage = Aktuellt steg:{ "  " }
progress_all_stages = Alla steg:{ "  " }
# Saving loading 
saving_loading_saving_success = Konfigurationen sparades till filen { $name }.
saving_loading_saving_failure = Det gick inte att spara konfigurationsdata till filen { $name }. Orsak: { $reason }.
saving_loading_reset_configuration = Aktuell konfiguration har rensats.
saving_loading_loading_success = Programkonfigurationen lästes in.
saving_loading_no_config_file = Ingen konfigurationsfil hittad, med standardinställningar.
saving_loading_failed_to_create_config_file = Det gick inte att skapa konfigurationsfilen "{ $path }". Orsak: "{ $reason }".
saving_loading_failed_to_read_config_file = Kan inte ladda konfiguration från "{ $path }" eftersom den inte finns eller inte är en fil.
saving_loading_failed_to_read_data_from_file = Det går inte att läsa data från filen "{ $path }". Orsak: "{ $reason }".
# Other
selected_all_reference_folders = Det går inte att starta sökningen när alla mappar har angetts som referensmappar
searching_for_data = Söker data, det kan ta en stund, vänta...
text_view_messages = MEDDELANDEN
text_view_warnings = VARNINGAR
text_view_errors = FEL
about_window_motto = Detta program är gratis att använda och kommer alltid att vara.
krokiet_new_app = Czkawka är i underhållsläge, vilket innebär att endast kritiska fel kommer att rättas och inga nya funktioner kommer att läggas till. För nya funktioner, titta gärna på den nya Krokiet-appen, som är stabilare, snabbare och fortfarande utvecklas aktivt.
# Various dialog
dialogs_ask_next_time = Fråga nästa gång
symlink_failed = Det gick inte att skapa en symbolisk länk från { $name } till { $target }. Orsak: { $reason }
delete_title_dialog = Ta bort bekräftelse
delete_question_label = Är du säker på att du vill ta bort filer?
delete_all_files_in_group_title = Bekräftelse av att ta bort alla filer i grupp
delete_all_files_in_group_label1 = I vissa grupper är alla poster markerade.
delete_all_files_in_group_label2 = Är du säker på att du vill ta bort dem?
delete_items_label = { $items } filer kommer att tas bort.
delete_items_groups_label = { $items } filer från { $groups } grupper kommer att tas bort.
hardlink_failed = Det gick inte att skapa en hårdlänk från { $name } till { $target }. Orsak: { $reason }
hard_sym_invalid_selection_title_dialog = Ogiltigt val med vissa grupper
hard_sym_invalid_selection_label_1 = I vissa grupper är bara en post markerad och den kommer att ignoreras.
hard_sym_invalid_selection_label_2 = För att kunna skapa hårdlänkar/symboliska länkar för dessa filer måste minst två resultat i gruppen markeras.
hard_sym_invalid_selection_label_3 = Först i grupp känns igen som original och ändras inte, men andra och senare ändras.
hard_sym_link_title_dialog = Länkbekräftelse
hard_sym_link_label = Är du säker på att du vill länka dessa filer?
move_folder_failed = Det gick inte att flytta mappen { $name }. Orsak: { $reason }
move_file_failed = Det gick inte att flytta filen { $name }. Orsak: { $reason }
move_files_title_dialog = Välj mapp dit du vill flytta dubblettfiler
move_files_choose_more_than_1_path = Endast en sökväg får vara markerad för att dubblettfilerna ska kunna kopieras. Markerade: { $path_number }.
move_stats = { $num_files } av { $all_files } objekt flyttades
save_results_to_file = Resultaten sparades som både txt- och JSON-filer i mappen "{ $name }".
search_not_choosing_any_music = FEL: Du måste markera minst en kryssruta med söktyper för musik.
search_not_choosing_any_broken_files = FEL: Du måste markera minst en kryssruta med typ av trasiga filer som ska kontrolleras.
include_folders_dialog_title = Mappar att inkludera
exclude_folders_dialog_title = Mappar att exkludera
include_manually_directories_dialog_title = Lägg till mapp manuellt
cache_properly_cleared = Cachen rensades korrekt
cache_clear_duplicates_title = Rensar dubblettcache
cache_clear_similar_images_title = Rensar liknande bildcache
cache_clear_similar_videos_title = Rensar cache för liknande videor
cache_clear_message_label_1 = Vill du rensa cachen från föråldrade poster?
cache_clear_message_label_2 = Denna åtgärd tar bort alla cacheposter som pekar på ogiltiga filer.
cache_clear_message_label_3 = Detta kan göra inläsning/sparande i cache något snabbare.
cache_clear_message_label_4 = VARNING: Åtgärden kommer att ta bort alla cachade data från frånkopplade externa enheter. Därför måste varje hashvärde genereras om.
# Show preview
preview_image_resize_failure = Det gick inte att ändra storlek på bilden { $name }.
preview_image_opening_failure = Det gick inte att öppna bilden { $name }. Orsak: { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupp { $current_group }/{ $all_groups } ({ $images_in_group } bilder)
compare_move_left_button = L
compare_move_right_button = R
