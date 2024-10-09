# Window titles
window_settings_title = Instellingen
window_main_title = Czkawka (Hiccup)
window_progress_title = Scannen
window_compare_images = Vergelijk afbeeldingen
# General
general_ok_button = OK
general_close_button = Afsluiten
# Main window
music_title_checkbox = Aanspreektitel
music_artist_checkbox = Kunstenaar
music_year_checkbox = jaar
music_bitrate_checkbox = Bitsnelheid
music_genre_checkbox = genre
music_length_checkbox = longueur
music_comparison_checkbox = Geschatte vergelijking
music_checking_by_tags = Labels
music_checking_by_content = Inhoud
same_music_seconds_label = Minimale fragment tweede duur
same_music_similarity_label = Maximum verschil
music_compare_only_in_title_group = Vergelijk alleen in de titel
music_compare_only_in_title_group_tooltip =
    Wanneer ingeschakeld, worden bestanden gegroepeerd op titel en vervolgens vergeleken met elkaar.
    
    Met 10000 bestanden, in plaats van bijna 100 miljoen vergelijkingen zullen er meestal ongeveer 20000 vergelijkingen worden gemaakt.
same_music_tooltip =
    Zoeken naar vergelijkbare muziekbestanden door de inhoud ervan kan worden geconfigureerd door instelling:
    
    - De minimale fragmenttijd waarna muziekbestanden kunnen worden geïdentificeerd als vergelijkbaar
    - Het maximale verschil tussen twee geteste fragmenten
    
    De sleutel tot goede resultaten is om verstandige combinaties van deze parameters te vinden, voor opgegeven.
    
    Instelling van de minimale tijd op 5 en het maximale verschil op 1.0, zal zoeken naar bijna identieke fragmenten in de bestanden.
    Een tijd van 20 en een maximaal verschil van 6,0 werkt daarentegen goed voor het vinden van remixes/live versies, enz.
    
    Standaard wordt elk muziekbestand met elkaar vergeleken en dit kan veel tijd in beslag nemen bij het testen van veel bestanden, dus is het meestal beter om referentie-mappen te gebruiken en aan te geven welke bestanden met elkaar moeten worden vergeleken (met dezelfde hoeveelheid bestanden, Het vergelijken van vingerafdrukken is sneller dan zonder referentiemateriaal).
music_comparison_checkbox_tooltip =
    Het zoekt naar vergelijkbare muziekbestanden met behulp van AI, die machine-leren gebruikt om haakjes uit een zin te verwijderen. Bijvoorbeeld met deze optie ingeschakeld de bestanden in kwestie zullen als duplicaten worden beschouwd:
    
    SØ wieľdzizive b --- S000000wie.pldzizľb (Remix Lato 2021)
duplicate_case_sensitive_name = Kist gevoelig
duplicate_case_sensitive_name_tooltip =
    Wanneer ingeschakeld, groep alleen records wanneer ze precies dezelfde naam hebben, b.v. Zit ołd <-> ZØ ołd
    
    Uitschakelen van een dergelijke optie zal namen groeperen zonder te controleren of elke letter hetzelfde formaat heeft, bijv. zghaoŁD <-> Zit ołd
duplicate_mode_size_name_combo_box = Grootte en naam
duplicate_mode_name_combo_box = naam
duplicate_mode_size_combo_box = Grootte
duplicate_mode_hash_combo_box = Toegangssleutel
duplicate_hash_type_tooltip =
    Czkawka biedt 3 soorten hashes:
    
    Blake3 - cryptografische hash-functie. Dit is de standaard omdat het erg snel is.
    
    CRC32 - eenvoudige hashfunctie. Dit zou sneller moeten zijn dan Blake3, maar kan zeer zelden een botsing veroorzaken.
    
    XXH3 - erg vergelijkbaar in prestaties en hashkwaliteit naar Blake3 (maar niet-cryptografie). Dergelijke modi kunnen dus eenvoudig worden verwisseld.
duplicate_check_method_tooltip =
    Op dit moment biedt Czkawka drie soorten methode aan om duplicaten te vinden door:
    
    Naam - Gevonden bestanden met dezelfde naam.
    
    Grootte - Gevonden bestanden die dezelfde grootte hebben.
    
    Hash - Gevonden bestanden die dezelfde inhoud hebben. Deze modus hashet het bestand en vergelijkt deze hash later om duplicaten te vinden. Deze modus is de veiligste manier om duplicaten te vinden. App gebruikt zwaar cache, dus de tweede en verdere scans van dezelfde gegevens zou veel sneller moeten zijn dan de eerste.
image_hash_size_tooltip =
    Elke gecontroleerde afbeelding produceert een speciale hash die met elkaar kan worden vergeleken en een klein verschil tussen hen betekent dat deze afbeeldingen vergelijkbaar zijn.
    
    8 hash size is vrij goed om afbeeldingen te vinden die maar een beetje lijken op origineel. Met een grotere set afbeeldingen (>1000) levert dit een grote hoeveelheid valse positieven op. Dus ik raad in dit geval aan een grotere hashgrootte te gebruiken.
    
    16 is de standaard hash-afmeting, wat een heel goed compromis is tussen het vinden van zelfs een beetje gelijksoortige afbeeldingen en het hebben van slechts een klein aantal hash-botsingen.
    
    32 en 64 hashes vinden slechts zeer gelijksoortige afbeeldingen, maar zouden bijna geen valse positieve motieven moeten hebben (behalve sommige afbeeldingen met alpha kanaal).
image_resize_filter_tooltip =
    Om hash van de afbeelding te berekenen, moet de bibliotheek deze eerst grootschalen.
    
    Afhankelijk van het gekozen algoritme, zal de uiteindelijke afbeelding die gebruikt wordt om hash te berekenen er een beetje anders uitzien.
    
    Het snelste algoritme te gebruiken, maar ook het algoritme dat de slechtste resultaten geeft, is het dichtstbijst. Het is standaard ingeschakeld, want met 16x16 hash grootte is het niet echt zichtbaar.
    
    met 8x8 hash grootte is het raadzaam om een ander algoritme te gebruiken dan Nearest, om betere groepen afbeeldingen te hebben.
image_hash_alg_tooltip =
    Gebruikers kunnen kiezen uit een van de vele algoritmes om de hash te berekenen.
    
    Elk van deze punten heeft sterke en zwakke punten en zal soms betere en soms slechtere resultaten opleveren voor verschillende afbeeldingen.
    
    Dus om het beste voor u te bepalen, is handmatige test vereist.
big_files_mode_combobox_tooltip = Maakt het mogelijk om naar kleinste/grootste bestanden te zoeken
big_files_mode_label = Gecontroleerde bestanden
big_files_mode_smallest_combo_box = De Kleinste
big_files_mode_biggest_combo_box = De Grootste
main_notebook_duplicates = Dupliceer Bestanden
main_notebook_empty_directories = Lege mappen
main_notebook_big_files = Grote bestanden
main_notebook_empty_files = Lege bestanden
main_notebook_temporary = Tijdelijke bestanden
main_notebook_similar_images = Vergelijkbare afbeeldingen
main_notebook_similar_videos = Soortgelijke video's
main_notebook_same_music = Muziek duplicaten
main_notebook_symlinks = Ongeldige Symlinks
main_notebook_broken_files = Kapotte Bestanden
main_notebook_bad_extensions = Slechte extensies
main_tree_view_column_file_name = File Name
main_tree_view_column_folder_name = Map Naam
main_tree_view_column_path = Pad
main_tree_view_column_modification = Wijziging datum
main_tree_view_column_size = Grootte
main_tree_view_column_similarity = Vergelijkbaarheid
main_tree_view_column_dimensions = Mål
main_tree_view_column_title = Aanspreektitel
main_tree_view_column_artist = Kunstenaar
main_tree_view_column_year = jaar
main_tree_view_column_bitrate = Bitsnelheid
main_tree_view_column_length = longueur
main_tree_view_column_genre = genre
main_tree_view_column_symlink_file_name = Symlink bestandsnaam
main_tree_view_column_symlink_folder = Symlink map
main_tree_view_column_destination_path = Bestemming pad
main_tree_view_column_type_of_error = Type fout
main_tree_view_column_current_extension = Huidige extensie
main_tree_view_column_proper_extensions = Proper Extensie
main_label_check_method = Controleer methode
main_label_hash_type = Soort hash
main_label_hash_size = Hash grootte
main_label_size_bytes = Grootte (bytes)
main_label_min_size = Min.
main_label_max_size = Max.
main_label_shown_files = Aantal getoonde bestanden
main_label_resize_algorithm = Algoritme aanpassen
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = Geluid
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Archief
main_check_box_broken_files_image = Afbeelding
check_button_general_same_size = Negeer dezelfde grootte
check_button_general_same_size_tooltip = Bestanden met identieke grootte in resultaten negeren - meestal zijn deze 1:1 duplicaten
main_label_size_bytes_tooltip = Grootte van bestanden die zullen worden gebruikt in scan
# Upper window
upper_tree_view_included_folder_column_title = Mappen om te zoeken
upper_tree_view_included_reference_column_title = Referentie Mappen
upper_recursive_button = Recursief
upper_recursive_button_tooltip = Indien geselecteerd, zoek ook naar bestanden die niet direct onder de gekozen mappen worden geplaatst.
upper_manual_add_included_button = Handmatig toevoegen
upper_add_included_button = Toevoegen
upper_remove_included_button = Verwijderen
upper_manual_add_excluded_button = Handmatig toevoegen
upper_add_excluded_button = Toevoegen
upper_remove_excluded_button = Verwijderen
upper_manual_add_included_button_tooltip =
    Voeg mapnaam toe om met de hand te zoeken.
    
    Om meerdere paden tegelijk toe te voegen, scheiden ze met ;
    
    /home/roman;/home/rozkaz zal twee mappen / home/roman en /home/rozkaz toevoegen
upper_add_included_button_tooltip = Voeg nieuwe map toe om te zoeken.
upper_remove_included_button_tooltip = Map verwijderen uit zoekopdracht.
upper_manual_add_excluded_button_tooltip =
    Voeg uitgesloten mapnaam met de hand toe.
    
    Om meerdere paden tegelijk toe te voegen, scheid ze met
    
    /home/roman;/home/krokiet zal twee mappen / home/roman en /home/keokiet toevoegen
upper_add_excluded_button_tooltip = Voeg map toe om uitgesloten te worden in zoekopdracht.
upper_remove_excluded_button_tooltip = Verwijder map van uitgesloten.
upper_notebook_items_configuration = Artikelen configuratie
upper_notebook_excluded_directories = Uitgesloten Mappen
upper_notebook_included_directories = Inbegrepen Mappen
upper_allowed_extensions_tooltip =
    Toegestane extensies moeten door komma's gescheiden worden (standaard zijn alle beschikbaar).
    
    De volgende macro's die meerdere extensies in één keer toevoegen, zijn ook beschikbaar: IMAGE, VIDEO, MUSIC, TEXT.
    
    Gebruiksgebruik voorbeeld ".exe, IMAGE, VIDEO, .rar, 7z" - dit betekent dat afbeeldingen (e. . jpg, png), video's (bijv. avi, mp4), exe, rr en 7z bestanden worden gescand.
upper_excluded_extensions_tooltip =
    Lijst van uitgeschakelde bestanden die genegeerd zullen worden in scan.
    
    Wanneer gebruik wordt gemaakt van toegestane en uitgeschakelde extensies, heeft deze hogere prioriteit, dus het bestand zal niet worden gecontroleerd.
upper_excluded_items_tooltip =
    Uitgesloten items moeten * jokertekens bevatten en moeten gescheiden worden door komma's.
    Dit is langzamer dan uitgesloten mappen, dus gebruik het zorgvuldig.
upper_excluded_items = Uitgesloten artikelen:
upper_allowed_extensions = Toegestane extensies:
upper_excluded_extensions = Uitgeschakelde extensies:
# Popovers
popover_select_all = Alles selecteren
popover_unselect_all = Selectie ongedaan maken
popover_reverse = Omgekeerde selectie
popover_select_all_except_oldest = Alles selecteren behalve oudste
popover_select_all_except_newest = Selecteer alles behalve nieuwste
popover_select_one_oldest = Selecteer één oudste
popover_select_one_newest = Selecteer een nieuwste
popover_select_custom = Selecteer aangepaste
popover_unselect_custom = Aangepaste deselecteer ongedaan maken
popover_select_all_images_except_biggest = Alles selecteren behalve de grootste
popover_select_all_images_except_smallest = Selecteer alles behalve de kleinste
popover_custom_path_check_button_entry_tooltip =
    Records per pad selecteren.
    
    Voorbeeld gebruik:
    /home/pimpek/rzecz.txt kan worden gevonden met /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Records selecteren op bestandnamen.
    
    Voorbeeld gebruik:
    /usr/ping/pong.txt kan worden gevonden met *ong*
popover_custom_regex_check_button_entry_tooltip =
    Select records by specified Regex.
    
    In this mode is searched text Path with Name.
    
    Voorbeeld use usage:
    /usr/bin/ziemniak. xt kan gevonden worden met /ziem[a-z]+
    
    Dit gebruikt de standaard Rust regex implementatie. Je kunt hier meer over lezen: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Maakt hoofdlettergevoelige detectie mogelijk.
    
    Wanneer uitgeschakeld /home/* vindt zowel /HoMe/roman en /home/roman.
popover_custom_not_all_check_button_tooltip =
    Voorkomt dat alle records in de groep worden geselecteerd.
    
    Dit is standaard ingeschakeld, omdat in de meeste situaties u wilt niet zowel origineel als duplicaten verwijderen, maar ten minste één bestand achterlaten.
    
    WAARSCHUWING: Deze instelling werkt niet als je al handmatig alle resultaten hebt geselecteerd in een groep.
popover_custom_regex_path_label = Pad
popover_custom_regex_name_label = naam
popover_custom_regex_regex_label = Regex pad + naam
popover_custom_case_sensitive_check_button = Hoofdletter gevoelig
popover_custom_all_in_group_label = Niet alle records in groep selecteren
popover_custom_mode_unselect = Aangepaste deselecteren
popover_custom_mode_select = Selecteer aangepast
popover_sort_file_name = Bestandsnaam is vereist
popover_sort_folder_name = Folder Name
popover_sort_full_name = Volledige naam
popover_sort_size = Grootte
popover_sort_selection = Selectie
popover_invalid_regex = Regex is ongeldig
popover_valid_regex = Regex is geldig
# Bottom buttons
bottom_search_button = Zoeken
bottom_select_button = Selecteren
bottom_delete_button = Verwijderen
bottom_save_button = Opslaan
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Verplaatsen
bottom_sort_button = Sorteren
bottom_search_button_tooltip = Zoeken starten
bottom_select_button_tooltip = Selecteer records. Alleen geselecteerde bestanden/mappen kunnen later worden verwerkt.
bottom_delete_button_tooltip = Verwijder geselecteerde bestanden/mappen.
bottom_save_button_tooltip = Gegevens opslaan van zoekopdracht naar bestand
bottom_symlink_button_tooltip =
    Maak symbolische links.
    Werkt alleen wanneer ten minste twee resultaten in een groep zijn geselecteerd.
    De eerste is ongewijzigd en de tweede is symgekoppeld naar eerst.
bottom_hardlink_button_tooltip =
    Maak hardlinks.
    Werkt alleen wanneer ten minste twee resultaten in een groep worden geselecteerd.
    De eerste is ongewijzigd en de tweede keer en later zijn vastgekoppeld aan eerst.
bottom_hardlink_button_not_available_tooltip =
    Maak hardlinks.
    Knop is uitgeschakeld, omdat hardlinks niet kunnen worden gemaakt.
    Hardlinks werkt alleen met beheerdersrechten op Windows, dus zorg ervoor dat je de app als administrator gebruikt.
    Als de app al werkt met dergelijke privileges controle op gelijksoortige issues op Github.
bottom_move_button_tooltip =
    Verplaatst bestanden naar de gekozen map.
    Het kopieert alle bestanden naar de map zonder de mapstructuur te bewaren.
    Wanneer twee bestanden met dezelfde naam naar de map worden verplaatst, zal de tweede mislukt en de fout worden weergegeven.
bottom_sort_button_tooltip = Sorteert bestanden/mappen op de geselecteerde methode.
bottom_show_errors_tooltip = Onderste tekstvenster tonen/verbergen
bottom_show_upper_notebook_tooltip = Toon/Verberg bovenste notitieboekpaneel.
# Progress Window
progress_stop_button = Stoppen
progress_stop_additional_message = Stop aangevraagd
# About Window
about_repository_button_tooltip = Link naar de repository pagina met broncode.
about_donation_button_tooltip = Link naar donatie pagina.
about_instruction_button_tooltip = Link naar instructiepagina.
about_translation_button_tooltip = Link naar de Crowdin pagina met appvertalingen. Officieel worden Pools en Engels ondersteund.
about_repository_button = Bewaarplaats
about_donation_button = Donatie
about_instruction_button = Instructie
about_translation_button = Vertaling
# Header
header_setting_button_tooltip = Opent instellingen dialoogvenster.
header_about_button_tooltip = Opent dialoogvenster met info over app.

# Settings


## General

settings_number_of_threads = Aantal gebruikte threads
settings_number_of_threads_tooltip = Aantal gebruikte threads, 0 betekent dat alle beschikbare threads zullen worden gebruikt.
settings_use_rust_preview = Gebruik externe bibliotheken in plaats daarvan om previews te laden
settings_use_rust_preview_tooltip =
    Het gebruik van gtk previews zal soms sneller zijn en ondersteuning bieden voor meer formaten, maar soms kan het precies het tegenovergestelde zijn.
    
    Als je problemen hebt met het laden van previews, kan je proberen deze instelling te veranderen.
    
    Op niet-linux systemen is het aangeraden om deze optie te gebruiken, omdat gtk-pixbuf niet altijd beschikbaar is, dus het uitschakelen van deze optie zal geen voorvertoningen van sommige afbeeldingen laden.
settings_label_restart = U moet de app herstarten om de instellingen toe te passen!
settings_ignore_other_filesystems = Negeer andere bestandssystemen (alleen Linux)
settings_ignore_other_filesystems_tooltip =
    negeert bestanden die niet in hetzelfde bestandssysteem zitten als gezochte mappen.
    
    Werkt dezelfde als -xdev optie in het zoekcommando voor Linux
settings_save_at_exit_button_tooltip = Configuratie opslaan in bestand bij het sluiten van de app.
settings_load_at_start_button_tooltip =
    Laad de configuratie van het bestand bij het openen van de app.
    
    Indien niet ingeschakeld, worden standaard instellingen gebruikt.
settings_confirm_deletion_button_tooltip = Bevestigingsvenster tonen bij het klikken op de knop verwijderen.
settings_confirm_link_button_tooltip = Toon bevestigingsvenster bij het klikken op de hard/symlink knop.
settings_confirm_group_deletion_button_tooltip = Waarschuwingsvenster weergeven wanneer geprobeerd wordt om alle records uit de groep te verwijderen.
settings_show_text_view_button_tooltip = Tekstpaneel aan de onderkant van de gebruikersinterface weergeven.
settings_use_cache_button_tooltip = Gebruik bestandscache.
settings_save_also_as_json_button_tooltip = Cache opslaan in (menselijk leesbaar) JSON formaat. Het is mogelijk om de inhoud te wijzigen. Cache van dit bestand wordt automatisch gelezen door de app als er een binaire cache (met bin extensie) ontbreekt.
settings_use_trash_button_tooltip = Verplaatst bestanden naar prullenbak in plaats daarvan ze permanent te verwijderen.
settings_language_label_tooltip = Taal voor de gebruikersinterface.
settings_save_at_exit_button = Configuratie opslaan bij het sluiten van app
settings_load_at_start_button = Laad configuratie bij het openen van app
settings_confirm_deletion_button = Toon bevestigingsdialoog bij het verwijderen van bestanden
settings_confirm_link_button = Melding bevestigen bij hard/symlinks van bestanden
settings_confirm_group_deletion_button = Toon het bevestigingsvenster bij het verwijderen van alle bestanden in de groep
settings_show_text_view_button = Toon onderaan tekstpaneel
settings_use_cache_button = Gebruik cache
settings_save_also_as_json_button = Sla ook de cache op als JSON-bestand
settings_use_trash_button = Verwijderde bestanden verplaatsen naar prullenbak
settings_language_label = Taal
settings_multiple_delete_outdated_cache_checkbutton = Verouderde cache vermeldingen automatisch verwijderen
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Verwijder verouderde cacheresultaten die verwijzen naar niet-bestaande bestanden.
    
    Indien ingeschakeld, zorgt de app ervoor dat bij het laden van records, dat alle records naar geldige bestanden verwijzen (gebroken bestanden worden genegeerd).
    
    Dit uitschakelen zal helpen bij het scannen van bestanden op externe schijven, dus cache-items over deze zullen niet worden gewist in de volgende scan.
    
    In het geval van honderdduizenden records in de cache, het wordt aangeraden om dit in te schakelen, dit zal de cache laden/opslaan aan het starten/einde van de scan versnellen.
settings_notebook_general = Algemeen
settings_notebook_duplicates = Duplicaten
settings_notebook_images = Vergelijkbare afbeeldingen
settings_notebook_videos = Gelijkaardige Video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Toont voorbeeld aan rechterkant (bij het selecteren van een afbeeldingsbestand).
settings_multiple_image_preview_checkbutton = Toon voorvertoning afbeelding
settings_multiple_clear_cache_button_tooltip =
    Handmatig de cache van verouderde items wissen.
    Dit mag alleen worden gebruikt als automatisch wissen is uitgeschakeld.
settings_multiple_clear_cache_button = Verwijder verouderde resultaten uit de cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Verbergt alle bestanden behalve één, als alle naar dezelfde gegevens verwijst (zijn hardlinked).
    
    Voorbeeld: In het geval waar er (op schijf) zeven bestanden zijn die zijn gekoppeld aan specifieke data en één ander bestand met dezelfde gegevens, maar een andere inode, dan in dubbele zoeker, slechts één uniek bestand en één bestand van de hardgelinkte bestanden zullen worden weergegeven.
settings_duplicates_minimal_size_entry_tooltip =
    Stel de minimale bestandsgrootte in die gecached zal worden.
    
    kiezen van een kleinere waarde zal meer records genereren. Dit zal het zoeken versnellen, maar de cache aan het laden/opslaan.
settings_duplicates_prehash_checkbutton_tooltip =
    Hiermee kan het cachen van prehash (een hash berekend van een klein deel van het bestand) eerder verwijderen van niet-gedupliceerde resultaten.
    
    Het is standaard uitgeschakeld omdat het in sommige situaties vertraging kan veroorzaken.
    
    Het is sterk aanbevolen om het te gebruiken bij het scannen van honderdduizenden of miljoen bestanden, omdat het zoeken meerdere keren kan versnellen.
settings_duplicates_prehash_minimal_entry_tooltip = Minimale grootte van gecachete invoer.
settings_duplicates_hide_hard_link_button = Verberg harde links (alleen Linux en macOS)
settings_duplicates_prehash_checkbutton = Gebruik prehash cache
settings_duplicates_minimal_size_cache_label = Minimale bestandsgrootte (in bytes) opgeslagen in de cache
settings_duplicates_minimal_size_cache_prehash_label = Minimale grootte van bestanden (in bytes) opgeslagen naar prehash cache

## Saving/Loading settings

settings_saving_button_tooltip = De huidige instellingen configuratie opslaan in bestand.
settings_loading_button_tooltip = Laad de instellingen uit het bestand en vervang de huidige configuratie.
settings_reset_button_tooltip = De huidige configuratie terugzetten naar de standaard.
settings_saving_button = Configuratie opslaan
settings_loading_button = Laad configuratie
settings_reset_button = Reset configuratie

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Opent de map waar de cache txt bestanden zijn opgeslagen.
    
    Het wijzigen van de cachebestanden kan ervoor zorgen dat ongeldige resultaten worden getoond. Het wijzigen van een pad kan echter tijd besparen bij het verplaatsen van een grote hoeveelheid bestanden naar een andere locatie.
    
    U kunt deze bestanden tussen computers kopiëren om tijd te besparen bij het scannen van bestanden (van natuurlijk als ze een vergelijkbare directory structuur hebben).
    
    In geval van problemen met de cache kunnen deze bestanden worden verwijderd. De app zal ze automatisch opnieuw genereren.
settings_folder_settings_open_tooltip =
    Opent de map waar de Czkawka config is opgeslagen.
    
    WAARSCHUWING: Handmatig wijzigen van de configuratie kan uw workflow verbreken.
settings_folder_cache_open = Open cachemap
settings_folder_settings_open = Instellingenmap openen
# Compute results
compute_stopped_by_user = Zoeken is gestopt door gebruiker
compute_found_duplicates_hash_size = Gevonden { $number_files } duplicaten in { $number_groups } groepen die { $size } namen
compute_found_duplicates_name = Gevonden { $number_files } duplicaten in { $number_groups } groepen
compute_found_empty_folders = Gevonden { $number_files } lege mappen
compute_found_empty_files = Gevonden { $number_files } lege bestanden
compute_found_big_files = Gevonden { $number_files } grote bestanden
compute_found_temporary_files = Gevonden { $number_files } tijdelijke bestanden
compute_found_images = Gevonden { $number_files } soortgelijke afbeeldingen in { $number_groups } groepen
compute_found_videos = Gevonden { $number_files } vergelijkbare video's in { $number_groups } groepen
compute_found_music = Gevonden { $number_files } vergelijkbare muziekbestanden in { $number_groups } groepen
compute_found_invalid_symlinks = Gevonden { $number_files } ongeldige symlinks
compute_found_broken_files = Gevonden { $number_files } gebroken bestanden
compute_found_bad_extensions = { $number_files } bestanden met ongeldige extensies gevonden
# Progress window
progress_scanning_general_file = Scannen van { $file_number } bestand
progress_scanning_extension_of_files = Extensie van { $file_checked }/{ $all_files } bestand controleren
progress_scanning_broken_files = Controleren { $file_checked }/{ $all_files } bestand
progress_scanning_video = Hashing van { $file_checked }/{ $all_files } video
progress_scanning_image = Hashing van { $file_checked }/{ $all_files } afbeelding
progress_comparing_image_hashes = Vergelijk { $file_checked }/{ $all_files } afbeelding hash
progress_scanning_music_tags_end = Tags vergelijken van { $file_checked }/{ $all_files } muziekbestand
progress_scanning_music_tags = Leestags van { $file_checked }/{ $all_files } muziekbestand
progress_scanning_music_content_end = Vingerafdruk van { $file_checked }/{ $all_files } muziekbestand vergelijken
progress_scanning_music_content = Vingerafdruk berekenen van { $file_checked }/{ $all_files } muziekbestand
progress_scanning_empty_folders = Scannen { $folder_number } map
progress_scanning_size = Scannen grootte van { $file_number } bestand
progress_scanning_size_name = Scannen van naam en grootte van { $file_number } bestand
progress_scanning_name = Scannen van naam { $file_number } bestand
progress_analyzed_partial_hash = Geanalyseerde gedeeltelijke hash van { $file_checked }/{ $all_files } bestanden
progress_analyzed_full_hash = Volledige hash van { $file_checked }/{ $all_files } bestanden geanalyseerd
progress_prehash_cache_loading = Prehash cache laden
progress_prehash_cache_saving = Opslaan van prehash cache
progress_hash_cache_loading = hash-cache laden
progress_hash_cache_saving = hash cache opslaan
progress_cache_loading = Cache laden
progress_cache_saving = Cache opslaan
progress_current_stage = Current Stage:{ "  " }
progress_all_stages = All Stages:{ "  " }
# Saving loading 
saving_loading_saving_success = Configuratie opgeslagen in bestand { $name }.
saving_loading_saving_failure = Kan configuratiegegevens niet opslaan in bestand { $name }.
saving_loading_reset_configuration = Huidige configuratie is gewist.
saving_loading_loading_success = Goed geladen app configuratie.
saving_loading_invalid_string = Voor sleutel "{ $key }" vond een ongeldig resultaat - "{ $result }" wat geen string is.
saving_loading_invalid_int = Voor sleutel "{ $key }" vond een ongeldig resultaat - "{ $result }" wat geen geheel getal is.
saving_loading_invalid_bool = Voor sleutel "{ $key }" vond een ongeldig resultaat - "{ $result }" wat geen baas is.
saving_loading_decode_problem_bool = Kan bool van sleutel niet decoderen "{ $key }" gevonden "{ $result }" maar toegestane waarden zijn 0, 1, waar of onwaar.
saving_loading_saving_same_keys = Proberen de instelling op te slaan met de gedupliceerde sleutel "{ $key }".
saving_loading_failed_to_get_home_directory = Fout bij het ophalen van de home directory naar configuratiebestand openen/opslaan.
saving_loading_folder_config_instead_file = Kan configuratiebestand niet aanmaken of openen in pad "{ $path }" omdat er al een map is.
saving_loading_failed_to_create_configuration_folder = Configuratie mislukt om configuratiemap "{ $path }", reden "{ $reason } " te maken.
saving_loading_failed_to_create_config_file = Fout bij het aanmaken van het configuratiebestand "{ $path }", reden "{ $reason }".
saving_loading_failed_to_read_config_file = Kan configuratie niet laden van "{ $path }" omdat deze niet bestaat of geen bestand is.
saving_loading_failed_to_read_data_from_file = Kan gegevens niet lezen van bestand "{ $path }", reden "{ $reason }".
saving_loading_orphan_data = orphan data gevonden "{ $data }" in regel "{ $line }".
saving_loading_not_valid = Instelling "{ $data }" bestaat niet in de huidige versie van de app.
# Invalid symlinks
invalid_symlink_infinite_recursion = Oneindige recursie
invalid_symlink_non_existent_destination = Niet-bestaand doelbestand
# Other
selected_all_reference_folders = Kan zoeken niet starten, als alle mappen als referentie mappen zijn ingesteld
searching_for_data = Gegevens zoeken, het kan een tijdje duren, even wachten...
text_view_messages = BERICHTEN
text_view_warnings = LET OP
text_view_errors = FOUTEN
about_window_motto = Dit programma is gratis te gebruiken en zal dat altijd zijn.
# Various dialog
dialogs_ask_next_time = Volgende keer vragen
delete_file_failed = Kan het bestand { $name }niet verwijderen, reden { $reason }
delete_title_dialog = Bevestiging verwijderen
delete_question_label = Weet u zeker dat u bestanden wilt verwijderen?
delete_all_files_in_group_title = Bevestiging van het verwijderen van alle bestanden in groep
delete_all_files_in_group_label1 = In sommige groepen worden alle records geselecteerd.
delete_all_files_in_group_label2 = Weet u zeker dat u deze wilt verwijderen?
delete_folder_failed = Kan de map { $dir } niet verwijderen omdat de map niet bestaat, u hebt geen toestemming of de map is niet leeg.
delete_items_label = { $items } bestanden worden verwijderd.
delete_items_groups_label = { $items } bestanden van { $groups } groepen worden verwijderd.
hardlink_failed = Hardlink mislukt
hard_sym_invalid_selection_title_dialog = Ongeldige selectie met sommige groepen
hard_sym_invalid_selection_label_1 = In sommige groepen is er slechts één record geselecteerd en zal worden genegeerd.
hard_sym_invalid_selection_label_2 = Om deze bestanden hard/sym te kunnen koppelen, moeten ten minste twee resultaten in de groep worden geselecteerd.
hard_sym_invalid_selection_label_3 = Eerst wordt de groep als origineel erkend en niet veranderd, en vervolgens worden de tweede wijzigingen aangebracht.
hard_sym_link_title_dialog = Bevestiging link
hard_sym_link_label = Weet u zeker dat u deze bestanden wilt koppelen?
move_folder_failed = Map { $name }kon niet verplaatst worden, reden { $reason }
move_file_failed = Kon bestand niet verplaatsen { $name }, reden { $reason }
move_files_title_dialog = Kies de map waarnaar u gedupliceerde bestanden wilt verplaatsen
move_files_choose_more_than_1_path = Er kan slechts één pad geselecteerd zijn om hun gedupliceerde bestanden te kopiëren, geselecteerde { $path_number }.
move_stats = Naar behoren verplaatst { $num_files }/{ $all_files } items
save_results_to_file = Opgeslagen resultaten zowel in txt als in json bestanden in { $name } map.
search_not_choosing_any_music = FOUT: U moet ten minste één selectievakje met muziekinstypes selecteren.
search_not_choosing_any_broken_files = FOUT: U moet ten minste één selectievakje selecteren met type van aangevinkte bestanden.
include_folders_dialog_title = Mappen om op te nemen
exclude_folders_dialog_title = Mappen om uit te sluiten
include_manually_directories_dialog_title = Voeg map handmatig toe
cache_properly_cleared = Cache op juiste wijze gewist
cache_clear_duplicates_title = Duplicaten cache wissen
cache_clear_similar_images_title = Leeg soortgelijke afbeeldingen-cache
cache_clear_similar_videos_title = Leeg soortgelijke video cache
cache_clear_message_label_1 = Wilt u de cache van verouderde items wissen?
cache_clear_message_label_2 = Deze actie zal alle cache-items verwijderen die naar ongeldige bestanden wijzen.
cache_clear_message_label_3 = Dit kan de laden/opslaan enigszins versnellen.
cache_clear_message_label_4 = WAARSCHUWING: De bewerking zal alle opgeslagen data van externe schijven verwijderen. Daarom zal elke hash opnieuw moeten worden gegenereerd.
# Show preview
preview_image_resize_failure = Formaat wijzigen van afbeelding { $name } is mislukt.
preview_image_opening_failure = Kan afbeelding { $name }niet openen, reden { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Groep { $current_group }/{ $all_groups } ({ $images_in_group } afbeeldingen)
compare_move_left_button = L
compare_move_right_button = R
