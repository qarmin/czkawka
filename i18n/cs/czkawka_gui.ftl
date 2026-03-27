# Window titles
window_settings_title = Nastavení
window_main_title = Czkawka (Škytavka)
window_progress_title = Skenování
window_compare_images = Porovnat obrázky
# General
general_ok_button = OK
general_close_button = Zavřít
# Krokiet info dialog
krokiet_info_title = Představujeme Krokiet – novou verzi Czkawky
krokiet_info_message =
    Krokiet je nová, vylepšená, rychlejší a spolehlivější verze Czkawky (původní GTK GUI)!
    
    Je snáze provozovatelná a odolnější změnám v systému, protože závisí pouze na základních knihovnách, které jsou standardně dostupné na většině systémů.
    
    Krokiet také přináší funkce, které Czkawka postrádá – včetně náhledů v režimu porovnávání videí, nástroje pro čištění EXIF, ukazatele průběhu přesouvání/kopírování/mazání souborů nebo rozšířených možností třídění.
    
    Vyzkoušejte a uvidíte rozdíl!
    
    Czkawka ode mne bude nadále dostávat opravy chyb a drobné aktualizace, ale všechny nové funkce budou vyvíjeny výhradně pro Krokiet. Nicméně pokud by někdo chtěl, samozřejmě může do Czkawky přispívat novými funkcemi, přidávat chybějící režimy a nadále ji rozvíjet.
    
    PS: Tato zpráva by se měla zobrazit pouze jednou. Pokud se tak stane znovu, nastavte si proměnnou prostředí CZKAWKA_DONT_ANNOY_ME (na libovolnou neprázdnou hodnotu).
# Main window
music_title_checkbox = Název
music_artist_checkbox = Umělec
music_year_checkbox = Rok
music_bitrate_checkbox = Bitová rychlost
music_genre_checkbox = Žánr
music_length_checkbox = Délka
music_comparison_checkbox = Přibližné porovnání
music_checking_by_tags = Štítky
music_checking_by_content = Obsah
same_music_seconds_label = Minimální délka trvání druhého fragmentu
same_music_similarity_label = Rozdíl nejvýše
music_compare_only_in_title_group = Porovnat v rámci skupin podobných názvů
music_compare_only_in_title_group_tooltip =
    Pokud je zapnuto, soubory jsou seskupeny podle názvu a až poté vzájemně porovnávány.
    
    V případě 10 000 souborů, namísto aby se dělalo téměř 100 milionů porovnání, takto obvykle postačí jen přibližně 20 000 porovnání.
same_music_tooltip =
    Vyhledávání podobných hudebních souborů podle jejich obsahu může být nakonfigurováno nastavením:
    
    - Minimální doba fragmentu, po které mohou být hudební soubory identifikovány jako podobné
    - Maximální rozdíl mezi dvěma testovanými fragmenty
    
    Klíč k dobrým výsledkům je najít rozumné kombinace těchto parametrů, pro stanovení.
    
    Nastavení minimální doby na 5 s a maximálního rozdílu na 1,0 bude hledat téměř stejné fragmenty v souborech.
    Doba 20 s a maximální rozdíl 6,0 naproti tomu funguje dobře pro nalezení remixů / nahrávek naživo atd.
    
    Ve výchozím nastavení je každý hudební soubor porovnáván s ostatními a to může trvat opravdu dlouho při testování mnoha souborů, takže je obvykle lepší používat referenční složky a specifikovat, které soubory mají být vzájemně porovnány (se stejným množstvím souborů, porovnávání otisků bude rychlejší alespoň 4x než bez referenčních složek).
music_comparison_checkbox_tooltip =
    Vyhledá podobné hudební soubory pomocí AI, která používá strojové učení k odstranění závorek z fráze. Například, pokud je tato možnost povolena, příslušné soubory budou považovány za duplicitní soubory:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Rozlišovat mezi malými a VELKÝMI písmeny
duplicate_case_sensitive_name_tooltip =
    Pokud je povoleno, seskupit pouze záznamy, když mají přesně stejný název, např.Żołd <-> Żołd
    
    Zakázání takové volby bude seskupovat názvy bez kontrolování, zda je každé písmeno stejné velikosti, např. żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Velikost a název
duplicate_mode_name_combo_box = Název
duplicate_mode_size_combo_box = Velikost
duplicate_mode_hash_combo_box = Otisk
duplicate_hash_type_tooltip =
    Czkawka nabízí 3 typy otisků:
    
    Blake3 – kryptografická funkce pro spočítání otisku. Toto je výchozí, protože je velmi rychlé.
    
    CRC32 - jednoduchá funkce pro spočítání otisku. Toto by mělo být rychlejší než Blake3, ale může docházet (byť jen zřídka) k tomu, že různá data budou mít stejný otisk.
    
    XXH3 - velmi podobné ve výkonu a kvalitě jednoznačnosti otisku jako Blake3 (ale nezaloženo na kryptografii). Takže je vlastně jedno, který z těchto dvou použijete.
duplicate_check_method_tooltip =
    Pro tuto chvíli Czkawka nabízí tři typy metod pro vyhledávání duplicitních souborů:
    
    Název – hledá soubory, které mají stejný název.
    
    Velikost – hledá soubory, které mají stejnou velikost.
    
    Otisk – hledá soubory, které mají stejný obsah. V tomto režimu je u každého souboru vypočítán jeho otisk, a následně jejich porovnáním jsou hledány duplicity. Tento režim je nejbezpečnějším způsobem, jak hledat duplicity. Aplikace používá mezipaměť, takže druhé a další skenování stejných dat by mělo být mnohem rychlejší než to první.
image_hash_size_tooltip =
    Pro každý z kontrolovaných obrázků je vytvořen zvlášť otisk, které je mezi sebou možné porovnávat. Malý rozdíl mezi nimi znamená, že obrázky jsou si podobné.
    
    Velikost otisku 8 je docela dobrá k nalezení obrázků, které jsou alespoň trochu podobné originálům. V případě větší sady obrázků (>1000) to ale vytvoří velké množství falešných shod, takže je v takovém případě doporučeno použít větší velikost otisku.
    
    16 je výchozí velikost otisku, což je docela dobrý kompromis mezi nalezením alespoň trochu podobných obrázků a pouze malým množstvím falešných shod.
    
    S otisky velikosti 32 a 64 budou nalezeny jen opravdu velmi podobné obrázky, ale nemělo by docházet k téměř žádným falešným shodám (snad krom některých obrázků, obsahující alfa kanál průhlednosti).
image_resize_filter_tooltip =
    Před vypočítáním otisku obrázku je třeba, aby zpracovávající softwarová knihovna přepočítala rozlišení jeho dočasné kopie na stejnou velikost, jako u ostatních porovnávaných.
    
    V závislosti na zvoleném algoritmu bude výsledný obrázek, použitý k výpočtu otisku, vypadat trochu jinak.
    
    Nejrychlejší algoritmus k používání, ale také ten, který dává nejhorší výsledky, je Nejbližší. Je nastavený jako výchozí, protože v případě velikost otisku 16x16 hash nedělá tato nižší kvalita rozdíl.
    
    V případě velikosti otisku 8x8 je doporučeno použít jiný algoritmus než Nejbližší, aby bylo obrázky seskupovány lépe.
image_hash_alg_tooltip =
    Uživatelé si mohou vybrat z jednoho z mnoha algoritmů pro výpočet otisku.
    
    Každý má silné a slabší stránky a někdy přinese lepší a někdy zase horší výsledky pro různé obrázky.
    
    Takže k určení nejlepšího pro vás je zapotřebí ruční zkoušení.
big_files_mode_combobox_tooltip = Umožňuje vyhledávat nejmenší / největší soubory
big_files_mode_label = Zaškrtnuté soubory
big_files_mode_smallest_combo_box = Nejmenší
big_files_mode_biggest_combo_box = Největší
main_notebook_duplicates = Duplicitní soubory
main_notebook_empty_directories = Prázdné složky
main_notebook_big_files = Velké soubory
main_notebook_empty_files = Prázdné soubory
main_notebook_temporary = Dočasné soubory
main_notebook_similar_images = Podobné obrázky
main_notebook_similar_videos = Podobná videa
main_notebook_same_music = Hudební duplicity
main_notebook_symlinks = Neplatné symbolické odkazy
main_notebook_broken_files = Poškozené soubory
main_notebook_bad_extensions = Nesprávné přípony
main_tree_view_column_file_name = Název souboru
main_tree_view_column_folder_name = Název složky
main_tree_view_column_path = Popis umístění
main_tree_view_column_modification = Datum změny
main_tree_view_column_size = Velikost
main_tree_view_column_similarity = Podobnost
main_tree_view_column_dimensions = Rozměry
main_tree_view_column_title = Název
main_tree_view_column_artist = Umělec
main_tree_view_column_year = Rok
main_tree_view_column_bitrate = Bitová rychlost
main_tree_view_column_length = Délka
main_tree_view_column_genre = Žánr
main_tree_view_column_symlink_file_name = Název souboru symbolického odkazu
main_tree_view_column_symlink_folder = Složka symbolického odkazu
main_tree_view_column_destination_path = Cílový popis umístění
main_tree_view_column_type_of_error = Typ chyby
main_tree_view_column_current_extension = Stávající přípona
main_tree_view_column_proper_extensions = Správná přípona
main_tree_view_column_fps = Snímků/s
main_tree_view_column_codec = Kodek
main_label_check_method = Metoda kontroly
main_label_hash_type = Typ otisku
main_label_hash_size = Velikost otisku
main_label_size_bytes = Velikost (bajty)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Počet zobrazených souborů
main_label_resize_algorithm = Algoritmus pro změnu rozlišení
main_label_similarity = Podobnost { " " }
main_check_box_broken_files_audio = Zvuk
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Archiv
main_check_box_broken_files_image = Obrázek
main_check_box_broken_files_video = Video
main_check_box_broken_files_video_tooltip = Pro ověření videosouborů používá ffmpeg/ffprobe. Docela pomalé a může detekovat pedantické chyby i když se soubor přehrává v pořádku.
check_button_general_same_size = Ignorovat stejnou velikost
check_button_general_same_size_tooltip = Ignorovat ve výsledcích soubory se stejnou velikostí – obvykle se jedná o duplicity 1:1
main_label_size_bytes_tooltip = Velikost souborů, které budou použity při skenování
# Upper window
upper_tree_view_included_folder_column_title = Složky k prohledání
upper_tree_view_included_reference_column_title = Referenční složky
upper_recursive_button = Rekurzivní
upper_recursive_button_tooltip = Pokud je vybráno, hledat také soubory, které nejsou umístěny přímo pod vybranými složkami.
upper_manual_add_included_button = Ruční přidání
upper_add_included_button = Přidat
upper_remove_included_button = Odebrat
upper_manual_add_excluded_button = Ruční přidání
upper_add_excluded_button = Přidat
upper_remove_excluded_button = Odebrat
upper_manual_add_included_button_tooltip =
    Ručně přidat název složky pro prohledání.
    
    Pokud chcete přidat vícero popisů umístění naráz, oddělujte je znakem ; (středník)
    
    /home/roman;/home/rozkaz přidá dvě složky: /home/roman a /home/rozkaz
upper_add_included_button_tooltip = Přidat novou složku k prohledání.
upper_remove_included_button_tooltip = Odstranit složku z prohledání.
upper_manual_add_excluded_button_tooltip =
    Přidejte ručně název vyloučené adresáře.
    
    Chcete-li přidat více cest najednou, oddělte je od ;
    
    /home/roman;/home/krokiet přidá dva adresáře /home/roman a /home/keokiet
upper_add_excluded_button_tooltip = Přidat adresář, který bude při hledání vyloučen.
upper_remove_excluded_button_tooltip = Odstranit adresář z vyloučení.
upper_notebook_items_configuration = Nastavení položek
upper_notebook_excluded_directories = Vynechané popisy umístění
upper_notebook_included_directories = Zahrnuté cesty
upper_allowed_extensions_tooltip =
    Je třeba, aby povolené přípony byly oddělované čárkou (ve výchozím nastavení jsou k dispozici všechny).
    
    Jsou také k dispozici následující makra, která přidávají více rozšíření najednou: IMAGE, VIDEO, MUSIC, TEXT.
    
    Příklad použití „.exe, IMAGE, VIDEO, .rar, 7z“ – toto znamená, že budou skenovány obrázky (např. jpg, png), videa (např. avi, mp4), exe, rar a 7z soubory.
upper_excluded_extensions_tooltip =
    Seznam zakázaných souborů, které budou při skenování ignorovány.
    
    Při používání povolených i zakázaných přípon, má tato vyšší prioritu, takže soubor nebude kontrolován.
upper_excluded_items_tooltip =
    Je třeba, aby vynechané položky obsahovaly zástupný znak * (hvězdička) a byly oddělovány čárkou.
    Toto je pomalejší než „Vynechané popisy umístění“, takže používejte opatrně.
upper_excluded_items = Vynechané položky:
upper_allowed_extensions = Povolené přípony:
upper_excluded_extensions = Zakázané přípony:
# Popovers
popover_select_all = Vybrat vše
popover_unselect_all = Odznačit vše
popover_reverse = Reverzní výběr
popover_select_all_except_shortest_path = Vyberte vše kromě nejkratší cesty
popover_select_all_except_longest_path = Vyberte vše kromě nejdelší cesty
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
bottom_hardlink_button = Pevný odkaz
bottom_move_button = Přesunout
bottom_sort_button = Seřadit
bottom_compare_button = Porovnat
bottom_search_button_tooltip = Zahájit vyhledávání
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
    Přesune soubory do zvolené složky.
    Zkopíruje všechny soubory do složky (bez zachování stromu podsložek).
    Pokud by takto mělo být do složky přesunuto vícero stejně nazvaných souborů, bude provedeno pouze u prvního z nich a pro ty ostatní bude vyhlášen nezdar a zobrazena chyba.
bottom_sort_button_tooltip = Seřazuje soubory/složky podle zvolené metody.
bottom_compare_button_tooltip = Porovnat obrázky ve skupině.
bottom_show_errors_tooltip = Zobrazit/skrýt spodní textový panel.
bottom_show_upper_notebook_tooltip = Zobrazit/skrýt horní panel karet.
# Progress Window
progress_stop_button = Zastavit
progress_stop_additional_message = Vyžádáno zastavení
# About Window
about_repository_button_tooltip = Odkaz na stránku repositáře se zdrojovými kódy.
about_donation_button_tooltip = Odkaz na stránku s darováním.
about_instruction_button_tooltip = Odkaz na stránku s pokyny.
about_translation_button_tooltip = Odkaz na stránku Crowdin s překlady aplikace. Vývojář sám se soustřeďuje na polština a angličtinu.
about_repository_button = Repozitář
about_donation_button = Darovat
about_instruction_button = Pokyny
about_translation_button = Překlad
# Header
header_setting_button_tooltip = Otevře dialogové okno nastavení.
header_about_button_tooltip = Otevře dialog s informacemi o aplikaci.

# Settings


## General

settings_number_of_threads = Počet použitých vláken procesoru
settings_number_of_threads_tooltip = Počet použitých vláken procesoru (0 znamená, že budou použita veškerá, která poskytuje).
settings_use_rust_preview = Pro načítání náhledů používat externí knihovny namísto gtk
settings_use_rust_preview_tooltip =
    Používání gtk náhledů je někdy rychlejší a podporuje vícero formátů, ale v některých případech tomu může být právě naopak.
    
    Pokud máte problémy s načítáním náhledů, je možné vyzkoušet změnu tohoto nastavení.
    
    Na jiných systémech, než jsou ty linuxové, je doporučeno tuto volbu použít, protože gtk-pixbuf nebývá vždy k dispozici – takže (při vypnutí této volby) by nebyly načítány náhledy některých obrázků.
settings_label_restart = Aby byla nastavení uplatněna, je třeba aplikaci restartovat!
settings_ignore_other_filesystems = Nepřekračovat na ostatní souborové systémy (pouze v Linuxu)
settings_ignore_other_filesystems_tooltip =
    ignoruje soubory, které se nenachází na stejném souborovém systému, jako prohledávané složky.
    
    Funguje stejně jako předvolba -xdev u linuxového příkazu find
settings_save_at_exit_button_tooltip = Při ukončování aplikace uložit nastavení do souboru.
settings_load_at_start_button_tooltip =
    Při spouštění aplikace načíst nastavení ze souboru.
    
    Pokud není zapnuto, budou použita výchozí nastavení.
settings_confirm_deletion_button_tooltip = Při kliknutí na tlačítko mazání zobrazit potvrzovací dialog.
settings_confirm_link_button_tooltip = Při kliknutí na tlačítko vytvoření pevného / symbolického odkazu zobrazit potvrzovací dialog.
settings_confirm_group_deletion_button_tooltip = Před pokusem o odstranění všech záznamů ze skupiny zobrazit varovný dialog.
settings_show_text_view_button_tooltip = Zobrazit textový panel v dolní části uživatelského rozhraní.
settings_use_cache_button_tooltip = Použít mezipaměť souborů.
settings_save_also_as_json_button_tooltip = Ukládat mezipaměť do (člověku srozumitelného) formátu JSON. Tím je možné ručně měnit její obsah. V případě, že by chyběla mezipaměť v binárním formátu (s příponou .bin), bude automaticky náhradně načtena právě z JSON souboru.
settings_use_trash_button_tooltip = Namísto jejich nevratného odstranění, přesouvá soubory do koše.
settings_language_label_tooltip = Jazyk uživatelského rozhraní.
settings_save_at_exit_button = Uložit nastavení při zavření aplikace
settings_load_at_start_button = Načíst nastavení při otevření aplikace
settings_confirm_deletion_button = Před mazáním jakýchkoli souborů zobrazit potvrzovací dialog
settings_confirm_link_button = Před vytvářením pevných / symbolických odkazů na jakékoli soubory zobrazit potvrzovací dialog
settings_confirm_group_deletion_button = Před mazáním všech souborů ve skupině zobrazit potvrzovací dialog
settings_show_text_view_button = Zobrazit spodní textový panel
settings_use_cache_button = Použít mezipaměť
settings_save_also_as_json_button = Ukládat mezipaměť také jako soubor JSON
settings_use_trash_button = Přesouvat mazané soubory do koše
settings_language_label = Jazyk
settings_multiple_delete_outdated_cache_checkbutton = Automaticky z mezipaměti odstraňovat už neplatné položky
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Odstranit zastaralé výsledky mezipaměti, které ukazují na neexistující soubory.
    
    Pokud je povoleno, aplikace se ujistí, že při načítání záznamů všechny záznamy odkazují na platné soubory (poškozené jsou ignorovány).
    
    Zakázáním této funkce pomůže při skenování souborů na externích discích, takže záznamy keší o nich nebudou vymazány v dalším skenování.
    
    V případě stovky tisíc záznamů v keši, je doporučeno toto povolit, což urychlí načítání/ukládání mezipaměti na začátku/konci skenování.
settings_notebook_general = Obecná
settings_notebook_duplicates = Duplicity
settings_notebook_images = Podobné obrázky
settings_notebook_videos = Podobná videa

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Při výběru obrázku zobrazí vpravo jeho náhled.
settings_multiple_image_preview_checkbutton = Zobrazovat náhled obrázku
settings_multiple_clear_cache_button_tooltip =
    Ručně spustit vyčištění mezipaměť od už neaktuálních položek.
    Toto je zde pouze pro případy, kdy je vypnuté automatické čištění.
settings_multiple_clear_cache_button = Odstranit už neaktuální výsledky z mezipaměti.

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
settings_duplicates_hide_hard_link_button = Skrýt pevné odkazy
settings_duplicates_prehash_checkbutton = Použít mezipaměť rozpoznávání
settings_duplicates_minimal_size_cache_label = Minimální velikost souborů (v bajtech) uložených do mezipaměti
settings_duplicates_minimal_size_cache_prehash_label = Minimální velikost souborů (v bajtech) uložených pro zachycení keše

## Saving/Loading settings

settings_saving_button_tooltip = Uložit stávající nastavení do souboru.
settings_loading_button_tooltip = Načíst nastavení ze souboru a nahradit jimi ta stávající.
settings_reset_button_tooltip = Vrátit stávající nastavení na výchozí hodnoty.
settings_saving_button = Uložit konfiguraci
settings_loading_button = Načíst nastavení
settings_reset_button = Vrátit nastavení na výchozí hodnoty

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Otevře složku, ve které jsou uloženy txt soubory mezipaměti.
    
    Úprava souborů může způsobit zobrazení neplatných výsledků. Nicméně upravení popisu umístění může ušetřit čas v případě přesunutí velkého množství souborů do jiného umístění.
    
    Tyto soubory je možné zkopírovat mezi počítači a ušetřit tak čas při opětovném skenování souborů (samozřejmě pokud mají podobnou adresářovou strukturu).
    
    V případě problémů s mezipamětí je možné tyto soubory odstranit. Aplikace je automaticky znovu vytvořít.
settings_folder_settings_open_tooltip =
    Otevře složku, ve kterou jsou uložena nastavení pro Czkawku.
    
    VAROVÁNÍ: Ruční úprava nastavení může rozbít váš pracovní postup.
settings_folder_cache_open = Otevřít složku mezipaměti
settings_folder_settings_open = Otevřít složku s nastaveními
# Compute results
compute_stopped_by_user = Vyhledávání bylo zastaveno uživatelem
compute_found_duplicates_hash_size = Nalezeno { $number_files } duplikátů v { $number_groups } skupinách, které trvaly { $size } v { $time }
compute_found_duplicates_name = Nalezeno { $number_files } duplikátů v { $number_groups } skupinách v { $time }
compute_found_empty_folders = Nalezeno { $number_files } prázdné složky v { $time }
compute_found_empty_files = Nalezeno { $number_files } prázdných souborů v { $time }
compute_found_big_files = Nalezeno { $number_files } velkých souborů v { $time }
compute_found_temporary_files = Nalezeno { $number_files } dočasných souborů v { $time }
compute_found_images = Nalezeno { $number_files } podobných obrázků v { $number_groups } skupinách v { $time }
compute_found_videos = Nalezeno { $number_files } podobných videí v { $number_groups } skupinách v { $time }
compute_found_music = Nalezeno { $number_files } podobných hudebních souborů v { $number_groups } skupinách v { $time }
compute_found_invalid_symlinks = Nalezeno { $number_files } neplatných symbolických odkazů v { $time }
compute_found_broken_files = Nalezeno { $number_files } rozbitých souborů v { $time }
compute_found_bad_extensions = Nalezeno { $number_files } souborů s nesprávnými příponami za { $time }
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] Naskenovaný { $file_number } soubor
       *[other] Skenované { $file_number } soubory
    }
progress_scanning_extension_of_files = Kontrola přípony { $file_checked }/{ $all_files } souboru
progress_scanning_broken_files = Kontrola { $file_checked }/{ $all_files } soubor ({ $data_checked }/{ $all_data })
progress_scanning_video = Hashováno { $file_checked }/{ $all_files } videa
progress_creating_video_thumbnails = Vytvořeny náhledy { $file_checked }/{ $all_files } videí
progress_scanning_image = Vytvořen otisk pro { $file_checked }/{ $all_files } obrázek ({ $data_checked }/{ $all_data })
progress_comparing_image_hashes = Porovnáno { $file_checked }/{ $all_files } otisků obrázků
progress_scanning_music_tags_end = Porovnávané štítky hudebního souboru { $file_checked }/{ $all_files }
progress_scanning_music_tags = Číst štížky { $file_checked }/{ $all_files } hudebního souboru
progress_scanning_music_content_end = Porovnávaný otisk souboru s hudbou { $file_checked }/{ $all_files }
progress_scanning_music_content = Vypočítaný otisk hudby { $file_checked }/{ $all_files } ({ $data_checked }/{ $all_data })
progress_scanning_empty_folders =
    { $folder_number ->
        [one] Naskenovaná složka { $folder_number }
       *[other] Naskenované { $folder_number } složky
    }
progress_scanning_size = Naskenovaná velikost souboru { $file_number }
progress_scanning_size_name = Naskenovaný název a velikost { $file_number } souboru
progress_scanning_name = Naskenován název { $file_number } souboru
progress_analyzed_partial_hash = Analyzováno částečný otisk souborů { $file_checked }/{ $all_files } ({ $data_checked }/{ $all_data })
progress_analyzed_full_hash = Analyzováno plné hash souborů { $file_checked }/{ $all_files } ({ $data_checked }/{ $all_data })
progress_prehash_cache_loading = Načítání mezipaměti rozpoznání
progress_prehash_cache_saving = Ukládání mezipaměti rozpoznání
progress_hash_cache_loading = Načítání mezipaměti otisků
progress_hash_cache_saving = Ukládání mezipaměti otisků
progress_cache_loading = Načítání mezipaměti
progress_cache_saving = Ukládání mezipaměti
progress_current_stage = Aktuální fáze: { " " }
progress_all_stages = Všechny etapy: { " " }
# Saving loading 
saving_loading_saving_success = Nastavení uložena do souboru { $name }.
saving_loading_saving_failure = Nepodařilo se uložit data nastavení do souboru { $name } – důvod: { $reason }.
saving_loading_reset_configuration = Stávající nastavení byla vymazána.
saving_loading_loading_success = Nastavení aplikace v pořádku načtena.
saving_loading_failed_to_create_config_file = Nepodařilo se vytvořit soubor s nastaveními „{ $path }“ – důvod: „{ $reason }“.
saving_loading_failed_to_read_config_file = Nebylo možné načíst nastavení z „{ $path }“, protože neexistuje nebo to není soubor.
saving_loading_failed_to_read_data_from_file = Nebylo možné načíst data ze souboru „{ $path }“ – důvod: „{ $reason }“.
# Other
selected_all_reference_folders = Hledání nelze spustit, pokud jsou všechny adresáře nastaveny jako referenční složky
searching_for_data = Vyhledávání dat může chvíli trvat, prosím čekejte...
text_view_messages = ZPRÁVY
text_view_warnings = VAROVÁNÍ
text_view_errors = CHYBY
about_window_motto = Tuto aplikaci je možné bezplatně používat (nyní i v budoucnu).
krokiet_new_app = Czkawka je v režimu údržby, což znamená, že budou opraveny pouze kritické chyby a nebudou přidány žádné nové funkce. Pro nové funkce si prosím přečtěte novou aplikaci Krokiet , která je stabilnější a výkonnější a je stále v aktivním vývoji.
# Various dialog
dialogs_ask_next_time = Zeptat se příště
symlink_failed = Nepodařilo se vytvořit symbolický odkaz { $name } na { $target } – důvod: { $reason }
delete_title_dialog = Potvrzení odstranění
delete_question_label = Opravdu chcete soubory smazat?
delete_all_files_in_group_title = Potvrzení odstranění všech souborů ve skupině
delete_all_files_in_group_label1 = V některých skupinách jsou vybrány všechny záznamy.
delete_all_files_in_group_label2 = Jste si jisti, že je chcete odstranit?
delete_items_label = { $items } souborů bude smazáno.
delete_items_groups_label = { $items } souborů z { $groups } skupin bude smazáno.
hardlink_failed = Nepodařilo se propojit { $name } na { $target }, důvod { $reason }
hard_sym_invalid_selection_title_dialog = Neplatný výběr s některými skupinami
hard_sym_invalid_selection_label_1 = V některých skupinách je vybrán pouze jeden záznam a bude ignorován.
hard_sym_invalid_selection_label_2 = Aby bylo možné tyto soubory propojit s pevným/sym, je třeba vybrat alespoň dva výsledky ve skupině.
hard_sym_invalid_selection_label_3 = První ve skupině je uznána jako původní a není změněna, ale druhá a později jsou upraveny.
hard_sym_link_title_dialog = Potvrzení odkazu
hard_sym_link_label = Opravdu chcete tyto soubory propojit?
move_folder_failed = Nepodařilo se přesunout složku { $name } – důvod: { $reason }
move_file_failed = Nepodařilo se přesunout soubor { $name } – důvod: { $reason }
move_files_title_dialog = Vyberte složku, do které chcete přesunout duplicitní soubory
move_files_choose_more_than_1_path = Aby bylo možné zkopírovat jejich duplikované soubory, lze vybrat pouze jeden popis umístnění – nyní ale vybráno { $path_number }.
move_stats = { $num_files }/{ $all_files } položek v pořádku přesunuto
save_results_to_file = Výsledky uloženy jako txt i json soubory do složky „{ $name }“.
search_not_choosing_any_music = CHYBA: je třeba vybrat alespoň jednu zaškrtávací kolonku typu vyhledávání hudby.
search_not_choosing_any_broken_files = CHYBA: Je třeba vybrat alespoň jednu zaškrtávací kolonku typu souborů, kontrolovaných ohledně poškození.
include_folders_dialog_title = Složky k zahrnutí
exclude_folders_dialog_title = Složky k vynechání
include_manually_directories_dialog_title = Přidat složku ručně
cache_properly_cleared = Mezipaměť v pořádku vyčištěna
cache_clear_duplicates_title = Čištění mezipaměti duplicit
cache_clear_similar_images_title = Čištění mezipaměti podobných obrázků
cache_clear_similar_videos_title = Čištění mezipaměti podobných videí
cache_clear_message_label_1 = Chcete z mezipaměti vyčistit už neaktuální položky?
cache_clear_message_label_2 = Tato operace odstraní všechny položky mezipaměti, které odkazují na neplatné soubory.
cache_clear_message_label_3 = Toto může mírně urychlit načítání/ukládání do mezipaměti.
cache_clear_message_label_4 = VAROVÁNÍ: Operace odstraní z mezipaměti vše co se týká dat na v tuto chíli nepřipojených externích discích. V budoucnu tak případně bude potřeba otisky z nich znovu spočítat.
# Show preview
preview_image_resize_failure = Nepodařilo se změnit rozlišení obrázku { $name }.
preview_image_opening_failure = Nepodařilo se otevřít obrázek { $name } – důvod: { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Skupina { $current_group }/{ $all_groups } ({ $images_in_group } obrázků)
compare_move_left_button = L
compare_move_right_button = R
