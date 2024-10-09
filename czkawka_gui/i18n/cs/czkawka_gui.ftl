# Window titles
window_settings_title = Nastavení
window_main_title = Czkawka (Škytavka)
window_progress_title = Skenování
window_compare_images = Porovnat obrázky
# General
general_ok_button = Ok
general_close_button = Zavřít
# Main window
music_title_checkbox = Hlava 1 – Celkem
music_artist_checkbox = Umělec
music_year_checkbox = Rok
music_bitrate_checkbox = Přenosová rychlost
music_genre_checkbox = Žánr
music_length_checkbox = Délka
music_comparison_checkbox = Přibližné srovnání
music_checking_by_tags = Štítky
music_checking_by_content = Obsah
same_music_seconds_label = Minimální délka trvání druhého fragmentu
same_music_similarity_label = Maximální rozdíl
music_compare_only_in_title_group = Porovnat pouze v názvu
music_compare_only_in_title_group_tooltip =
    Pokud je povoleno, soubory jsou seskupeny podle názvu a poté vzájemně porovnávány.
    
    S 10000 soubory, místo toho se obvykle uskuteční téměř 100 milionů srovnání kolem 20000 srovnání.
same_music_tooltip =
    Vyhledávání podobných hudebních souborů podle jejich obsahu může být nakonfigurováno nastavením:
    
    - Minimální doba fragmentu, po které mohou být hudební soubory identifikovány jako podobné
    - Maximální rozdíl mezi dvěma testovanými fragmenty
    
    Klíč k dobrým výsledkům je najít rozumné kombinace těchto parametrů, pro stanovení.
    
    Nastavení minimální doby na 5 s a maximální rozdíl na 1,0 bude hledat téměř stejné fragmenty v souborech.
    Čas 20 s a maximální rozdíl 6,0 na druhé straně funguje dobře pro nalezení remixů/živých verzí atd.
    
    Ve výchozím nastavení je každý hudební soubor porovnáván mezi sebou a to může trvat dlouho při testování mnoha souborů, takže je obvykle lepší používat referenční složky a specifikovat, které soubory mají být vzájemně porovnány (se stejným množstvím souborů, porovnávání otisků prstů bude rychlejší alespoň 4x než bez referenčních složek).
music_comparison_checkbox_tooltip =
    Vyhledá podobné hudební soubory pomocí AI, která používá strojové učení k odstranění závorek z fráze. Například, pokud je tato možnost povolena, příslušné soubory budou považovány za duplicitní soubory:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Rozlišuje malá a velká písmena
duplicate_case_sensitive_name_tooltip =
    Pokud je povoleno, skupiny pouze záznamy, pokud mají přesně stejný název, např.Żołd <-> Żołd
    
    Zakázání takové volby bude názvy skupin bez kontroly, zda je každé písmeno stejné velikosti, např. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Velikost a název
duplicate_mode_name_combo_box = Název
duplicate_mode_size_combo_box = Velikost
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka nabízí 3 typy hash:
    
    Blake3 - kryptografická hash funkce. Toto je výchozí, protože je velmi rychlý.
    
    CRC32 - jednoduchá hash funkce. To by mělo být rychlejší než Blake3, ale může docházet jen zřídka k nějakým střetům.
    
    XXH3 - velmi podobné ve výkonu a kvalitě hash jako Blake3 (ale nekryptografické). Takovéto režimy mohou být snadno zaměnitelné.
duplicate_check_method_tooltip =
    Pro tuto chvíli nabízí Czkawka tři typy metod, které vyhledávají duplicitní soubory:
    
    Název - Nalezení souborů, které mají stejný název.
    
    Velikost - Nalezí soubory, které mají stejnou velikost.
    
    Hash - Najde soubory, které mají stejný obsah. Tento režim hashuje soubor a později porovnává tento hash s nalezením duplikátů. Tento režim je nejbezpečnějším způsobem, jak nalézt duplikáty. Aplikace používá mezipaměť, takže druhé a další skenování stejných dat by mělo být mnohem rychlejší než první.
image_hash_size_tooltip =
    Každý zkontrolovaný obrázek vytváří speciální hash který lze porovnávat, a malý rozdíl mezi nimi znamená, že tyto obrázky jsou podobné.
    
    8 hash velikost je docela dobrá k nalezení obrázků, které jsou jen trochu podobné originálům. S větší sadou obrázků (>1000) to vytvoří velké množství falešných pozitivních prvků, takže doporučuji v tomto případě použít větší hash velikost.
    
    16 je výchozí velikost hashu, což je docela dobrý kompromis mezi nalezením i trochu podobných obrázků a malým množstvím hashových kolizí.
    
    32 a 64 hash nalezly jen velmi podobné obrázky, ale neměly by mít téměř žádné falešné pohledy (možná kromě některých obrázků s alfa kanálem).
image_resize_filter_tooltip =
    Pro výpočet hash obrázku musí knihovna nejprve změnit velikost.
    
    V závislosti na zvoleném algoritmu bude výsledný obrázek použitý k výpočtu hash vypadat trochu jinak.
    
    Nejrychlejší algoritmus k používání, ale také ten, který dává nejhorší výsledky, je blízko. Ve výchozím nastavení je povoleno, protože s menší kvalitou 16x16 hash není ve skutečnosti viditelná.
    
    S velikostí hash 8x8 je doporučeno použít jiný algoritmus než nejbližší pro lepší skupiny obrázků.
image_hash_alg_tooltip =
    Uživatelé si mohou vybrat z jednoho z mnoha algoritmů pro výpočet hashu.
    
    Každý má silné a slabší body a někdy přinese lepší a někdy horší výsledky pro různé obrázky.
    
    Takže k určení nejlepšího pro vás je vyžadováno ruční testování.
big_files_mode_combobox_tooltip = Umožňuje vyhledávat malé / největší soubory
big_files_mode_label = Zaškrtnuté soubory
big_files_mode_smallest_combo_box = Nejmenší
big_files_mode_biggest_combo_box = Největší
main_notebook_duplicates = Duplicitní soubory
main_notebook_empty_directories = Prázdné adresáře
main_notebook_big_files = Velké soubory
main_notebook_empty_files = Prázdné soubory
main_notebook_temporary = Dočasné soubory
main_notebook_similar_images = Podobné obrázky
main_notebook_similar_videos = Podobná videa
main_notebook_same_music = Hudební duplikáty
main_notebook_symlinks = Neplatné symbolické odkazy
main_notebook_broken_files = Rozbité soubory
main_notebook_bad_extensions = Špatná rozšíření
main_tree_view_column_file_name = Název souboru
main_tree_view_column_folder_name = Název složky
main_tree_view_column_path = Cesta
main_tree_view_column_modification = Datum změny
main_tree_view_column_size = Velikost
main_tree_view_column_similarity = Podobnost
main_tree_view_column_dimensions = Rozměry
main_tree_view_column_title = Hlava
main_tree_view_column_artist = Umělec
main_tree_view_column_year = Rok
main_tree_view_column_bitrate = Přenosová rychlost
main_tree_view_column_length = Délka
main_tree_view_column_genre = Žánr
main_tree_view_column_symlink_file_name = Název souboru symbolického odkazu
main_tree_view_column_symlink_folder = Složka symbolického odkazu
main_tree_view_column_destination_path = Cílová cesta
main_tree_view_column_type_of_error = Typ chyby
main_tree_view_column_current_extension = Aktuální rozšíření
main_tree_view_column_proper_extensions = Řádné rozšíření
main_label_check_method = Metoda kontroly
main_label_hash_type = Typ Hash
main_label_hash_size = Velikost hash
main_label_size_bytes = Velikost (bajty)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Počet zobrazených souborů
main_label_resize_algorithm = Změna velikosti algoritmu
main_label_similarity = Podobnost { " " }
main_check_box_broken_files_audio = Zvuk
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archivovat
main_check_box_broken_files_image = Obrázek
check_button_general_same_size = Ignorovat stejnou velikost
check_button_general_same_size_tooltip = Ignorovat soubory se stejnou velikostí ve výsledcích - obvykle se jedná o 1:1 duplicitní
main_label_size_bytes_tooltip = Velikost souborů, které budou použity při skenování
# Upper window
upper_tree_view_included_folder_column_title = Vyhledávané složky
upper_tree_view_included_reference_column_title = Referenční složky
upper_recursive_button = Rekurentní
upper_recursive_button_tooltip = Pokud je vybráno, hledejte také soubory, které nejsou umístěny přímo pod vybranými složkami.
upper_manual_add_included_button = Ruční přidání
upper_add_included_button = Přidat
upper_remove_included_button = Odebrat
upper_manual_add_excluded_button = Ruční přidání
upper_add_excluded_button = Přidat
upper_remove_excluded_button = Odebrat
upper_manual_add_included_button_tooltip =
    Přidat název adresáře k hledání ručně.
    
    Chcete-li přidat více cest najednou, oddělte je od ;
    
    /home/roman;/home/rozkaz přidá dva adresáře /home/roman a /home/rozkaz
upper_add_included_button_tooltip = Přidat nový adresář k vyhledávání.
upper_remove_included_button_tooltip = Odstranit adresář z hledání.
upper_manual_add_excluded_button_tooltip =
    Přidejte ručně název vyloučené adresáře.
    
    Chcete-li přidat více cest najednou, oddělte je od ;
    
    /home/roman;/home/krokiet přidá dva adresáře /home/roman a /home/keokiet
upper_add_excluded_button_tooltip = Přidat adresář, který bude při hledání vyloučen.
upper_remove_excluded_button_tooltip = Odstranit adresář z vyloučení.
upper_notebook_items_configuration = Konfigurace položek
upper_notebook_excluded_directories = Vyloučené adresáře
upper_notebook_included_directories = Zahrnuté adresáře
upper_allowed_extensions_tooltip =
    Povolené přípony musí být odděleny čárkami (ve výchozím nastavení jsou všechny k dispozici).
    
    Následující makra, která přidávají více rozšíření najednou, jsou také k dispozici: IMAGE, VIDEO, MUSIC, TEXT.
    
    Příklad použití ".exe, IMAGE, VIDEO, .rar, 7z" - to znamená, že obrázky (např. . jpg, png), videa (např. avi, mp4), exe, rar a 7z soubory budou naskenovány.
upper_excluded_extensions_tooltip =
    Seznam zakázaných souborů, které budou při skenování ignorovány.
    
    Při používání povolených i zakázaných přípon, má tato vyšší prioritu, takže soubor nebude zaškrtnut.
upper_excluded_items_tooltip =
    Vyloučené položky musí obsahovat * zástupné znaky a měly by být odděleny čárkami.
    To je pomalejší než vyloučené adresáře, proto je používáme opatrně.
upper_excluded_items = Vyloučené položky:
upper_allowed_extensions = Povolená rozšíření:
upper_excluded_extensions = Zakázané rozšíření:
# Popovers
popover_select_all = Vybrat vše
popover_unselect_all = Odznačit vše
popover_reverse = Reverzní výběr
popover_select_all_except_oldest = Vybrat vše kromě nejstarších
popover_select_all_except_newest = Vybrat vše kromě nejnovějších
popover_select_one_oldest = Vyberte jeden nejstarší
popover_select_one_newest = Vyberte jeden nejnovější
popover_select_custom = Vybrat vlastní
popover_unselect_custom = Zrušit výběr vlastních
popover_select_all_images_except_biggest = Vybrat vše kromě největších
popover_select_all_images_except_smallest = Vybrat všechny kromě nejmenších
popover_custom_path_check_button_entry_tooltip =
    Vyberte záznamy podle cesty.
    
    Příklad použití:
    /home/pimpek/rzecz.txt lze nalézt pomocí /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Vyberte záznamy podle názvů souborů.
    
    Příklad použití:
    /usr/ping/pong.txt lze nalézt s *ong*
popover_custom_regex_check_button_entry_tooltip =
    Vyberte záznamy podle zadaného Regexu.
    
    S tímto režimem je vyhledávaná cesta se jménem.
    
    Příklad použití:
    /usr/bin/ziemniak. xt lze nalézt pomocí /ziem[a-z]+
    
    Toto používá výchozí implementaci Rust regex. Více o tom si můžete přečíst zde: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Umožňuje detekci citlivosti na malá a velká písmena.
    
    Pokud je vypnuta /doma/* nálezů jak /HoMe/roman tak /home/roman.
popover_custom_not_all_check_button_tooltip =
    Zabraňuje výběru všech záznamů ve skupině.
    
    Toto je ve výchozím nastavení povoleno, protože ve většině situací, nechcete odstranit originální i duplicitní soubory, ale chcete opustit alespoň jeden soubor.
    
    VAROVÁNÍ: Toto nastavení nefunguje, pokud jste již ručně vybrali všechny výsledky ve skupině.
popover_custom_regex_path_label = Cesta
popover_custom_regex_name_label = Název
popover_custom_regex_regex_label = Regex cesta + Jméno
popover_custom_case_sensitive_check_button = Rozlišit malá a velká písmena
popover_custom_all_in_group_label = Nesbírat všechny záznamy ve skupině
popover_custom_mode_unselect = Zrušit výběr vlastních
popover_custom_mode_select = Vybrat vlastní
popover_sort_file_name = Název souboru
popover_sort_folder_name = Název adresáře
popover_sort_full_name = Jméno a příjmení
popover_sort_size = Velikost
popover_sort_selection = Výběr
popover_invalid_regex = Regex je neplatný
popover_valid_regex = Regex je platný
# Bottom buttons
bottom_search_button = Hledat
bottom_select_button = Vybrat
bottom_delete_button = Vymazat
bottom_save_button = Uložit
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Přesunout
bottom_sort_button = Seřadit
bottom_search_button_tooltip = Začít hledání
bottom_select_button_tooltip = Vyberte záznamy. Pouze vybrané soubory/složky mohou být později zpracovány.
bottom_delete_button_tooltip = Odstranit vybrané soubory/složky.
bottom_save_button_tooltip = Ukládat data o hledání do souboru
bottom_symlink_button_tooltip =
    Vytvořit symbolické odkazy.
    Funguje pouze tehdy, pokud jsou vybrány alespoň dva výsledky ve skupině.
    Nejprve je nezměněna a druhé a později jsou souvztažné s prvními.
bottom_hardlink_button_tooltip =
    Vytvořit hardwarové odkazy.
    Funguje pouze tehdy, pokud jsou vybrány alespoň dva výsledky ve skupině.
    Nejprve je nezměněna a druhé a později jsou těžce propojeny s prvními.
bottom_hardlink_button_not_available_tooltip =
    Vytvořit hardwarové odkazy.
    Tlačítko je zakázáno, protože hardwarové odkazy nelze vytvořit.
    Hardlinky fungují pouze s oprávněními administrátora v systému Windows, tak se ujistěte, že používáte aplikaci jako administrátora.
    Pokud aplikace s takovými oprávněními již funguje, podívejte se na podobné problémy na Githubu.
bottom_move_button_tooltip =
    Přesune soubory do vybraného adresáře.
    Zkopíruje všechny soubory do adresáře bez uchování stromu adresáře.
    Při pokusu přesunout dva soubory se stejným názvem do složky, druhý selže a zobrazí chybu.
bottom_sort_button_tooltip = Seřazuje soubory/složky podle zvolené metody.
bottom_show_errors_tooltip = Zobrazit/skrýt spodní textový panel.
bottom_show_upper_notebook_tooltip = Zobrazit/skrýt horní panel sešitu.
# Progress Window
progress_stop_button = Zastavit
progress_stop_additional_message = Zastavit požadavek
# About Window
about_repository_button_tooltip = Odkaz na stránku repositáře se zdrojovým kódem.
about_donation_button_tooltip = Odkaz na stránku s darováním.
about_instruction_button_tooltip = Odkaz na stránku instrukcí.
about_translation_button_tooltip = Odkaz na stránku Crowdin s překlady aplikací. Oficiálně polština a angličtina jsou podporovány.
about_repository_button = Repozitář
about_donation_button = Darovat
about_instruction_button = Instrukce
about_translation_button = Překlad
# Header
header_setting_button_tooltip = Otevře dialogové okno nastavení.
header_about_button_tooltip = Otevře dialog s informacemi o aplikaci.

# Settings


## General

settings_number_of_threads = Počet použitých vláken
settings_number_of_threads_tooltip = Počet použitých vláken, 0 znamená, že budou použita všechna dostupná vlákna.
settings_use_rust_preview = Místo toho použít externí knihovny gtk k načtení náhledů
settings_use_rust_preview_tooltip =
    Použití gtk náhledů bude někdy rychlejší a bude podporovat více formátů, ale někdy to může být pravý opak.
    
    Pokud máte problémy s načítáním náhledů, můžete zkusit toto nastavení změnit.
    
    Na jiných než linuxových systémech je doporučeno použít tuto možnost, protože gtk-pixbuf není vždy k dispozici, takže vypnutí této možnosti nebude načíst náhledy některých obrázků.
settings_label_restart = Pro použití nastavení je třeba restartovat aplikaci!
settings_ignore_other_filesystems = Ignorovat ostatní souborové systémy (pouze Linux)
settings_ignore_other_filesystems_tooltip =
    ignoruje soubory, které nejsou ve stejném souborovém systému jako prohledávané adresáře.
    
    Funguje stejně jako -xdev možnost najít příkaz na Linuxu
settings_save_at_exit_button_tooltip = Uložit konfiguraci do souboru při zavření aplikace.
settings_load_at_start_button_tooltip =
    Načíst konfiguraci ze souboru při otevírání aplikace.
    
    Pokud není povoleno, budou použita výchozí nastavení.
settings_confirm_deletion_button_tooltip = Zobrazit potvrzovací dialogové okno při kliknutí na tlačítko mazat.
settings_confirm_link_button_tooltip = Zobrazit potvrzovací dialog při kliknutí na tlačítko hard/symbolický odkaz.
settings_confirm_group_deletion_button_tooltip = Zobrazit varovný dialog při pokusu o odstranění všech záznamů ze skupiny.
settings_show_text_view_button_tooltip = Zobrazit textový panel v dolní části uživatelského rozhraní.
settings_use_cache_button_tooltip = Použít cache souborů.
settings_save_also_as_json_button_tooltip = Uložit keš do (lidsky čitelný) formátu JSON. Je možné změnit její obsah. Mezipaměť z tohoto souboru bude automaticky čtena aplikací, pokud chybí binární formát mezipaměti (s rozšířením koše).
settings_use_trash_button_tooltip = Přesune soubory do koše a místo toho je trvale odstraní.
settings_language_label_tooltip = Jazyk uživatelského rozhraní.
settings_save_at_exit_button = Uložit konfiguraci při zavření aplikace
settings_load_at_start_button = Načíst konfiguraci při otevření aplikace
settings_confirm_deletion_button = Zobrazit dialogové okno potvrzení při mazání všech souborů
settings_confirm_link_button = Zobrazit dialogové okno pro pevné / symbolické odkazy
settings_confirm_group_deletion_button = Zobrazit dialogové okno potvrzení při mazání všech souborů ve skupině
settings_show_text_view_button = Zobrazit spodní textový panel
settings_use_cache_button = Použít keš
settings_save_also_as_json_button = Ukládat mezipaměť také jako soubor JSON
settings_use_trash_button = Přesunout smazané soubory do koše
settings_language_label = Jazyk
settings_multiple_delete_outdated_cache_checkbutton = Automaticky odstranit zastaralé položky v mezipaměti
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Odstranit zastaralé výsledky mezipaměti, které ukazují na neexistující soubory.
    
    Pokud je povoleno, aplikace se ujistí, že při načítání záznamů všechny záznamy odkazují na platné soubory (poškozené jsou ignorovány).
    
    Zakázáním této funkce pomůže při skenování souborů na externích discích, takže záznamy keší o nich nebudou vymazány v dalším skenování.
    
    V případě stovky tisíc záznamů v keši, je doporučeno toto povolit, což urychlí načítání/ukládání mezipaměti na začátku/konci skenování.
settings_notebook_general = Obecná ustanovení
settings_notebook_duplicates = Duplikáty
settings_notebook_images = Podobné obrázky
settings_notebook_videos = Podobné video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Zobrazí náhled na pravé straně (při výběru souboru obrázku).
settings_multiple_image_preview_checkbutton = Zobrazit náhled obrázku
settings_multiple_clear_cache_button_tooltip =
    Ručně vymazat mezipaměť zastaralých položek.
    Toto by mělo být použito pouze v případě, že je zakázáno automatické vymazání.
settings_multiple_clear_cache_button = Odstranit zastaralé výsledky z mezipaměti.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Skryje všechny soubory kromě jedné, pokud všechny odkazují na stejná data (jsou hardlinované).
    
    Příklad: V případě, že je na disku sedm souborů, které jsou spojeny s konkrétními daty, a jeden jiný soubor se stejnými daty, ale jiným inodem, pak v hledání duplikátu bude zobrazen pouze jeden unikátní soubor a jeden soubor z hardlinovaných souborů.
settings_duplicates_minimal_size_entry_tooltip =
    Nastavte minimální velikost souboru, který bude uložen do mezipaměti.
    
    Výběr menší hodnoty vygeneruje více záznamů. Toto urychlí vyhledávání, ale zpomalí načítání/ukládání mezipaměti.
settings_duplicates_prehash_checkbutton_tooltip =
    Umožňuje ukládání do mezipaměti (hash vypočtený z malé části souboru), což umožňuje dřívější odstranění neduplikovaných výsledků.
    
    Ve výchozím nastavení je zakázáno, protože v některých situacích může způsobit zpomalení.
    
    Doporučujeme jej použít při skenování stovek tisíc nebo miliónů souborů, protože může urychlit hledání několikrát.
settings_duplicates_prehash_minimal_entry_tooltip = Minimální velikost položky v mezipaměti.
settings_duplicates_hide_hard_link_button = Skrýt pevné odkazy (pouze Linux a macOS)
settings_duplicates_prehash_checkbutton = Použít mezipaměť rozpoznávání
settings_duplicates_minimal_size_cache_label = Minimální velikost souborů (v bajtech) uložených do mezipaměti
settings_duplicates_minimal_size_cache_prehash_label = Minimální velikost souborů (v bajtech) uložených pro zachycení keše

## Saving/Loading settings

settings_saving_button_tooltip = Uložit aktuální nastavení do souboru.
settings_loading_button_tooltip = Načíst nastavení ze souboru a nahradit jejich aktuální konfiguraci.
settings_reset_button_tooltip = Obnovit aktuální konfiguraci na výchozí.
settings_saving_button = Uložit konfiguraci
settings_loading_button = Načíst konfiguraci
settings_reset_button = Obnovit konfiguraci

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Otevře složku, kde jsou uloženy soubory txt v mezipaměti.
    
    Úprava souborů může způsobit zobrazení neplatných výsledků. Změna cesty však může ušetřit čas při přesunu velkého množství souborů do jiného umístění.
    
    Tyto soubory můžete zkopírovat mezi počítači, abyste ušetřili čas při skenování souborů (samozřejmě pokud mají podobnou strukturu adresáře).
    
    V případě problémů s mezipamětí, mohou být tyto soubory odstraněny. Aplikace je automaticky obnoví.
settings_folder_settings_open_tooltip =
    Otevře složku, kde je uloženo nastavení Czkawka.
    
    VAROVÁNÍ: Ruční úprava konfigurace může poškodit váš pracovní postup.
settings_folder_cache_open = Otevřít složku mezipaměti
settings_folder_settings_open = Otevřít složku s nastavením
# Compute results
compute_stopped_by_user = Vyhledávání bylo zastaveno uživatelem
compute_found_duplicates_hash_size = Nalezeno { $number_files } duplikátů v { $number_groups } skupinách, které trvaly { $size }
compute_found_duplicates_name = Nalezeno { $number_files } duplicit v { $number_groups } skupinách
compute_found_empty_folders = Nalezen { $number_files } prázdných složek
compute_found_empty_files = Nalezeno { $number_files } prázdných souborů
compute_found_big_files = Nalezeno { $number_files } velkých souborů
compute_found_temporary_files = Nalezeno { $number_files } dočasných souborů
compute_found_images = Nalezeno { $number_files } podobných obrázků v { $number_groups } skupinách
compute_found_videos = Nalezeno { $number_files } podobných videí v { $number_groups } skupinách
compute_found_music = Nalezeno { $number_files } podobných hudebních souborů v { $number_groups } skupinách
compute_found_invalid_symlinks = Nalezeno { $number_files } neplatných symbolických odkazů
compute_found_broken_files = Nalezeno { $number_files } poškozených souborů
compute_found_bad_extensions = Nalezeno { $number_files } souborů s neplatnými příponami
# Progress window
progress_scanning_general_file = Skenování { $file_number } souboru
progress_scanning_extension_of_files = Kontrola přípony souboru { $file_checked }/{ $all_files }
progress_scanning_broken_files = Kontrola souboru { $file_checked }/{ $all_files }
progress_scanning_video = Hashování { $file_checked }/{ $all_files } videa
progress_scanning_image = Hashování { $file_checked }/{ $all_files } obrázku
progress_comparing_image_hashes = Porovnávám { $file_checked }/{ $all_files } hash obrázků
progress_scanning_music_tags_end = Porovnávání značek s { $file_checked }/{ $all_files } hudebním souborem
progress_scanning_music_tags = Čtení tagů z { $file_checked }/{ $all_files } hudebního souboru
progress_scanning_music_content_end = Porovnání otisku prstu v { $file_checked }/{ $all_files } hudebního souboru
progress_scanning_music_content = Výpočet otisku prstu { $file_checked }/{ $all_files } hudebního souboru
progress_scanning_empty_folders = Skenování { $folder_number } složky
progress_scanning_size = Skenování velikosti { $file_number } souboru
progress_scanning_size_name = Skenování jména a velikosti souboru { $file_number }
progress_scanning_name = Skenování názvu souboru { $file_number }
progress_analyzed_partial_hash = Analyzován částečný hash souborů { $file_checked }/{ $all_files }
progress_analyzed_full_hash = Analyzováno úplné hash souborů { $file_checked }/{ $all_files }
progress_prehash_cache_loading = Načítání mezipaměti rozpoznání
progress_prehash_cache_saving = Ukládání mezipaměti rozpoznání
progress_hash_cache_loading = Načítání hash keše
progress_hash_cache_saving = Ukládání keše hash
progress_cache_loading = Načítání keše
progress_cache_saving = Ukládání keše
progress_current_stage = Aktuální fáze:{ " " }
progress_all_stages = Všechny etapy:{ " " }
# Saving loading 
saving_loading_saving_success = Uložena konfigurace do souboru { $name }.
saving_loading_saving_failure = Nepodařilo se uložit konfigurační data do souboru { $name }.
saving_loading_reset_configuration = Aktuální konfigurace byla vymazána.
saving_loading_loading_success = Správně načtená konfigurace aplikace.
saving_loading_invalid_string = Pro klíč "{ $key }" byl nalezen neplatný výsledek - "{ $result }", což není řetězec.
saving_loading_invalid_int = Pro klíč "{ $key }" byl nalezen neplatný výsledek - "{ $result }", což není celé číslo.
saving_loading_invalid_bool = Pro klíč "{ $key }" byl nalezen neplatný výsledek - "{ $result }", což není bool.
saving_loading_decode_problem_bool = Nepodařilo se dekódovat bool z klíče "{ $key }" nalezeno "{ $result }", ale povolené hodnoty jsou 0, 1, true nebo false.
saving_loading_saving_same_keys = Pokus o uložení nastavení s duplicitním klíčem "{ $key }".
saving_loading_failed_to_get_home_directory = Nepodařilo se získat domovský adresář pro otevření/uložení konfiguračního souboru.
saving_loading_folder_config_instead_file = Nelze vytvořit nebo otevřít konfigurační soubor v cestě "{ $path }", protože již existuje složka.
saving_loading_failed_to_create_configuration_folder = Nepodařilo se vytvořit konfigurační složku "{ $path }", důvod "{ $reason }".
saving_loading_failed_to_create_config_file = Nepodařilo se vytvořit konfigurační soubor "{ $path }", důvod "{ $reason }".
saving_loading_failed_to_read_config_file = Konfiguraci z "{ $path } nelze načíst, protože neexistuje nebo není soubor.
saving_loading_failed_to_read_data_from_file = Nelze číst data ze souboru "{ $path }", důvod "{ $reason }".
saving_loading_orphan_data = Nalezena osiřelá data "{ $data }" v řádku "{ $line }".
saving_loading_not_valid = Nastavení "{ $data }" v aktuální verzi aplikace neexistuje.
# Invalid symlinks
invalid_symlink_infinite_recursion = Nekonečná rekurze
invalid_symlink_non_existent_destination = Neexistující cílový soubor
# Other
selected_all_reference_folders = Hledání nelze spustit, pokud jsou všechny adresáře nastaveny jako referenční složky
searching_for_data = Vyhledávání dat může chvíli trvat, prosím čekejte...
text_view_messages = ZPRÁVY
text_view_warnings = VAROVÁNÍ
text_view_errors = CHYBA
about_window_motto = Tento program je zdarma a bude vždy používán.
# Various dialog
dialogs_ask_next_time = Příště se zeptat
delete_file_failed = Nepodařilo se odstranit soubor { $name }, důvod { $reason }
delete_title_dialog = Potvrzení odstranění
delete_question_label = Jste si jisti, že chcete odstranit soubory?
delete_all_files_in_group_title = Potvrzení odstranění všech souborů ve skupině
delete_all_files_in_group_label1 = V některých skupinách jsou vybrány všechny záznamy.
delete_all_files_in_group_label2 = Jste si jisti, že je chcete odstranit?
delete_folder_failed = Nepodařilo se odstranit složku { $dir } , protože složka neexistuje, nemáte oprávnění nebo složka není prázdná.
delete_items_label = { $items } soubory budou odstraněny.
delete_items_groups_label = { $items } souborů z { $groups } skupin bude smazáno.
hardlink_failed = Nepodařilo se použít hardwarový odkaz
hard_sym_invalid_selection_title_dialog = Neplatný výběr s některými skupinami
hard_sym_invalid_selection_label_1 = V některých skupinách je vybrán pouze jeden záznam a bude ignorován.
hard_sym_invalid_selection_label_2 = Aby bylo možné tyto soubory propojit s pevným/sym, je třeba vybrat alespoň dva výsledky ve skupině.
hard_sym_invalid_selection_label_3 = První ve skupině je uznána jako původní a není změněna, ale druhá a později jsou upraveny.
hard_sym_link_title_dialog = Potvrzení odkazu
hard_sym_link_label = Jste si jisti, že chcete propojit tyto soubory?
move_folder_failed = Nepodařilo se přesunout složku { $name }, důvod { $reason }
move_file_failed = Nepodařilo se přesunout soubor { $name }, důvod { $reason }
move_files_title_dialog = Vyberte složku, do které chcete přesunout duplicitní soubory
move_files_choose_more_than_1_path = Lze vybrat pouze jednu cestu, aby bylo možné zkopírovat jejich duplikované soubory, vybrané { $path_number }.
move_stats = Správně přesunuto { $num_files }/{ $all_files } položek
save_results_to_file = Uloženy výsledky do txt i json souborů do složky { $name }.
search_not_choosing_any_music = CHYBA: Musíte vybrat alespoň jedno zaškrtávací políčko s prohledáváním hudby.
search_not_choosing_any_broken_files = CHYBA: Musíte vybrat alespoň jedno zaškrtávací políčko s typem zkontrolovaných poškozených souborů.
include_folders_dialog_title = Složky, které chcete zahrnout
exclude_folders_dialog_title = Složky k vyloučení
include_manually_directories_dialog_title = Přidat adresář ručně
cache_properly_cleared = Správně vymazaná mezipaměť
cache_clear_duplicates_title = Vymazání cache duplicity
cache_clear_similar_images_title = Vymazání cache podobných obrázků
cache_clear_similar_videos_title = Vymazání cache podobných videí
cache_clear_message_label_1 = Chcete vymazat mezipaměť zastaralých položek?
cache_clear_message_label_2 = Tato operace odstraní všechny položky mezipaměti, které ukazují na neplatné soubory.
cache_clear_message_label_3 = To může mírně urychlit načítání/ukládání do mezipaměti.
cache_clear_message_label_4 = VAROVÁNÍ: Operace odstraní všechna data v mezipaměti z nezapojených externích disků. Každá hash bude muset být obnovena.
# Show preview
preview_image_resize_failure = Nepodařilo se změnit velikost obrázku { $name }.
preview_image_opening_failure = Nepodařilo se otevřít obrázek { $name }, důvod { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Skupina { $current_group }/{ $all_groups } ({ $images_in_group } obrázků)
compare_move_left_button = L
compare_move_right_button = R
