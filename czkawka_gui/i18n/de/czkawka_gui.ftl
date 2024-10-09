# Window titles
window_settings_title = Einstellungen
window_main_title = Czkawka (Schluckauf)
window_progress_title = Scannen
window_compare_images = Bilder vergleichen
# General
general_ok_button = Ok
general_close_button = Schließen
# Main window
music_title_checkbox = Titel
music_artist_checkbox = Künstler
music_year_checkbox = Jahr
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Genre
music_length_checkbox = Dauer
music_comparison_checkbox = Ungefährer Vergleich
music_checking_by_tags = Tags
music_checking_by_content = Inhalt
same_music_seconds_label = Minimale Dauer des Fragments, in Sekunden
same_music_similarity_label = Maximaler Unterschied
music_compare_only_in_title_group = Nur im Titel vergleichen
music_compare_only_in_title_group_tooltip =
    Wenn aktiviert, werden Dateien nach Titel gruppiert und dann miteinander verglichen.
    
    Mit 10000 Dateien, statt fast 100 Millionen Vergleiche wird es in der Regel rund 20000 Vergleiche geben.
same_music_tooltip =
    Die Suche nach ähnlichen Musikdateien nach dem Inhalt kann über die Einstellung konfiguriert werden:
    
    - Die minimale Fragmentzeit, nach der Musikdateien als ähnlich identifiziert werden können
    - Der maximale Unterschied zwischen zwei getesteten Fragmenten
    
    Der Schlüssel zu guten Ergebnissen ist die Suche nach sinnvollen Kombinationen dieser Parameter. für bereitgestellt.
    
    Wenn Sie die minimale Zeit auf 5 Sekunden und den maximalen Unterschied auf 1,0 setzen, werden fast identische Fragmente in den Dateien gesucht.
    Eine Zeit von 20 Sekunden und ein maximaler Unterschied von 6.0 hingegen funktioniert gut um Remix/Live-Versionen zu finden.
    
    Standardmäßig wird jede Musikdatei miteinander verglichen, und dies kann viel Zeit in Anspruch nehmen, wenn viele Dateien getestet werden so ist es in der Regel besser, Referenzordner zu verwenden und festzulegen, welche Dateien miteinander verglichen werden sollen (mit gleicher Dateigröße Der Vergleich von Fingerabdrücken wird mindestens 4x schneller als ohne Referenzordner sein).
music_comparison_checkbox_tooltip =
    Mit Hilfe von einer KI, die maschinelles Lernen nutzt, um Klammern aus Sätzen zu entfernen, wird nach ähnlichen Musikdateien gesucht. Wenn die Option aktiviert ist, werden die folgenden Dateien zum Beispiel als Duplikate betrachtet:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Gross-/Kleinschreibung beachten
duplicate_case_sensitive_name_tooltip =
    Wenn aktiviert, gruppieren Sie nur Datensätze, wenn sie genau denselben Namen haben, z. żoŁD <-> Żołd
    Deaktivieren dieser Option gruppiert Namen ohne zu überprüfen, ob jeder Buchstabe die gleiche Größe wie żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Größe und Name
duplicate_mode_name_combo_box = Name
duplicate_mode_size_combo_box = Größe
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka bietet 3 Arten von Hashes an, die verwendet werden können:
    
    Blake3 - kryptographische Hashfunktion. Wegen ihrer Geschwindikeit wird Sie als Standard-Hash-Algorithmus verwendet.
    
    CRC32 - einfache Hash-Funktion. Sie sollte schneller sein als Blake3, könnte aber in seltenen Fällen Kollisionen haben.
    
    XXH3 - bei Geschwindikeit und Hashqualität vergleichbar mit Blake3 (aber nicht kryptographisch). Beide Modi können somit leicht miteinander ausgetauscht werden.
duplicate_check_method_tooltip =
    Derzeit bietet Czkawka drei Methoden an, um Duplikate zu finden:
    
    Name - Findet Dateien mit gleichem Namen.
    
    Größe - Findet Dateien mit gleicher Größe.
    
    Hash - Findet Dateien mit dem gleichen Inhalt. Dieser Modus hasht die Datei und vergleicht diese Hashes, um Duplikate zu finden. Dieser Modus ist der sicherste Weg, um Duplikate zu finden. Das Tool verwendet einen Cache, daher sollten weiteren Scans der gleichen Dateien viel schneller sein als der erste.
image_hash_size_tooltip =
    Jedes geprüfte Bild erzeugt einen speziellen Hash, der miteinander verglichen werden kann und ein kleiner Unterschied zwischen ihnen bedeutet, dass diese Bilder ähnlich sind.
    
    8 Hashgröße ist sehr gut, um Bilder zu finden, die dem Original nur ein wenig ähneln. Bei einer größeren Anzahl von Bildern (>1000), wird dies eine große Anzahl falscher Positives, also empfehle ich in diesem Fall eine größere Hashgröße.
    
    16 ist die standardmäßige Hashgröße, die einen guten Kompromiss zwischen dem Finden von kleinen ähnlichen Bildern und einer geringen Anzahl von Hash-Kollisionen darstellt.
    
    32 und 64 Hashes finden nur sehr ähnliche Bilder, sollten aber fast keine falschen positiven Bilder haben (vielleicht mit Ausnahme einiger Bilder mit Alphakanal).
image_resize_filter_tooltip =
    Um Hash des Bildes zu berechnen, muss die Bibliothek zuerst die Größe des Hashs verändern.
    
    Abhängig vom gewählten Algorithmus sieht das resultierende Bild, das zur Hashberechnung verwendet wird, ein wenig anders aus.
    
    Der schnellste zu verwendende Algorithmus, aber auch der, der die schlechtesten Ergebnisse liefert, ist in der Nähe. Sie ist standardmäßig aktiviert, da sie mit 16x16 Hash-Größe nicht wirklich sichtbar ist.
    
    Mit 8x8 Hashgröße wird empfohlen, einen anderen Algorithmus als nahe zu verwenden, um bessere Gruppen von Bildern zu haben.
image_hash_alg_tooltip =
    Benutzer können aus einem von vielen Algorithmen zur Berechnung des Hashs wählen.
    
    Jeder hat sowohl starke als auch schwächere Punkte und wird manchmal bessere und manchmal schlechtere Ergebnisse für verschiedene Bilder liefern.
    
    Um also den besten für Sie zu ermitteln, ist eine manuelle Prüfung erforderlich.
big_files_mode_combobox_tooltip = Erlaubt die Suche nach kleinsten/größten Dateien
big_files_mode_label = Überprüfte Dateien
big_files_mode_smallest_combo_box = Die kleinsten
big_files_mode_biggest_combo_box = Die größten
main_notebook_duplicates = Gleiche Dateien
main_notebook_empty_directories = Leere Verzeichnisse
main_notebook_big_files = Große Dateien
main_notebook_empty_files = Leere Dateien
main_notebook_temporary = Temporäre Dateien
main_notebook_similar_images = Ähnliche Bilder
main_notebook_similar_videos = Ähnliche Videos
main_notebook_same_music = Gleiche Musik
main_notebook_symlinks = Ungültige Symlinks
main_notebook_broken_files = Defekte Dateien
main_notebook_bad_extensions = Falsche Erweiterungen
main_tree_view_column_file_name = Dateiname
main_tree_view_column_folder_name = Ordnername
main_tree_view_column_path = Pfad
main_tree_view_column_modification = Änderungsdatum
main_tree_view_column_size = Größe
main_tree_view_column_similarity = Ähnlichkeit
main_tree_view_column_dimensions = Abmessungen
main_tree_view_column_title = Titel
main_tree_view_column_artist = Künstler
main_tree_view_column_year = Jahr
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Dauer
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = Symlink Dateiname
main_tree_view_column_symlink_folder = Symlink-Ordner
main_tree_view_column_destination_path = Zielpfad
main_tree_view_column_type_of_error = Fehlertyp
main_tree_view_column_current_extension = Aktuelle Erweiterung
main_tree_view_column_proper_extensions = Richtige Erweiterung
main_label_check_method = Prüfmethode
main_label_hash_type = Hash Typ
main_label_hash_size = Hash Größe
main_label_size_bytes = Größe (Bytes)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Anzahl der angezeigten Dateien
main_label_resize_algorithm = Algorithmus skalieren
main_label_similarity = Ähnlichkeit{ " " }
main_check_box_broken_files_audio = Ton
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Archiv
main_check_box_broken_files_image = Bild
check_button_general_same_size = Gleiche Größe ignorieren
check_button_general_same_size_tooltip = Ignoriere Dateien mit identischer Größe in den Ergebnissen - in der Regel sind es 1:1 Duplikate
main_label_size_bytes_tooltip = Größe der Dateien, die beim Scannen verwendet werden
# Upper window
upper_tree_view_included_folder_column_title = Zu durchsuchende Ordner
upper_tree_view_included_reference_column_title = Referenzordner
upper_recursive_button = Rekursiv
upper_recursive_button_tooltip = Falls ausgewählt, suchen Sie auch nach Dateien, die nicht direkt unter den ausgewählten Ordnern platziert werden.
upper_manual_add_included_button = Manuell hinzufügen
upper_add_included_button = Neu
upper_remove_included_button = Entfernen
upper_manual_add_excluded_button = Manuell hinzufügen
upper_add_excluded_button = Neu
upper_remove_excluded_button = Entfernen
upper_manual_add_included_button_tooltip =
    Verzeichnisname zur Suche per Hand hinzufügen.
    
    Um mehrere Pfade gleichzeitig hinzuzufügen, trennen Sie sie durch ;
    
    /home/roman;/home/rozkaz fügt zwei Verzeichnisse /home/roman und /home/rozkaz hinzu
upper_add_included_button_tooltip = Neues Verzeichnis zur Suche hinzufügen.
upper_remove_included_button_tooltip = Verzeichnis von der Suche löschen.
upper_manual_add_excluded_button_tooltip =
    Ausgeschlossenen Verzeichnisnamen per Hand hinzufügen.
    
    Um mehrere Pfade gleichzeitig hinzuzufügen, trennen Sie sie durch ;
    
    /home/roman;/home/krokiet wird zwei Verzeichnisse /home/roman und /home/keokiet hinzufügen
upper_add_excluded_button_tooltip = Verzeichnis hinzufügen, das bei der Suche ausgeschlossen werden soll.
upper_remove_excluded_button_tooltip = Ausgeschlossene Verzeichnisse löschen.
upper_notebook_items_configuration = Suchbedingungen
upper_notebook_excluded_directories = Ausgeschlossene Verzeichnisse
upper_notebook_included_directories = Einbezogene Verzeichnisse
upper_allowed_extensions_tooltip =
    Erlaubte Erweiterungen müssen durch Kommas getrennt werden (standardmäßig sind alle verfügbar).
    
    Folgende Makros, die mehrere Erweiterungen gleichzeitig hinzufügen, sind ebenfalls verfügbar: IMAGE, VIDEO, MUSIC, TEXT.
    
    Nutzungsbeispiel ".exe, IMAGE, VIDEO, .rar, 7z" - bedeutet, dass Bilder (jpg, png ...), Videodateien (avi, mp4 ...), exe, rar und 7z gescannt werden.
upper_excluded_extensions_tooltip =
    Liste der deaktivierten Dateien, die beim Scannen ignoriert werden.
    
    Wenn sowohl erlaubte als auch deaktivierte Erweiterungen verwendet werden, hat diese eine höhere Priorität, so dass die Datei nicht ausgewählt wird.
upper_excluded_items_tooltip =
    Ausgeschlossene Elemente müssen * als Platzhalter enthalten und durch Kommata getrennt werden.
    Dies ist langsamer als Verzeichnisse auszuschließen, daher mit Vorsicht verwenden.
upper_excluded_items = Ausgeschlossene Elemente:
upper_allowed_extensions = Erlaubte Erweiterungen:
upper_excluded_extensions = Deaktivierte Erweiterungen:
# Popovers
popover_select_all = Alles auswählen
popover_unselect_all = Gesamte Auswahl aufheben
popover_reverse = Auswahl umkehren
popover_select_all_except_oldest = Alle außer Ältester auswählen
popover_select_all_except_newest = Alle außer Neuester auswählen
popover_select_one_oldest = Älteste auswählen
popover_select_one_newest = Neueste auswählen
popover_select_custom = Individuell auswählen
popover_unselect_custom = Individuell Auswahl aufheben
popover_select_all_images_except_biggest = Alle außer Größter auswählen
popover_select_all_images_except_smallest = Alle außer Kleinster auswählen
popover_custom_path_check_button_entry_tooltip =
    Ermöglicht die Auswahl von Datensätzen nach Dateipfad.
    
    Beispielnutzung:
    /home/pimpek/rzecz.txt kann mit /home/pim* gefunden werden
popover_custom_name_check_button_entry_tooltip =
    Ermöglicht die Auswahl von Datensätzen nach Dateinamen.
    
    Beispielnutzung:
    /usr/ping/pong.txt kann mit *ong* gefunden werden
popover_custom_regex_check_button_entry_tooltip =
    Ermöglicht die Auswahl von Datensätzen nach spezifizierter Regex.
    
    Mit diesem Modus ist der gesuchte Text der Pfad einschließlich Dateinamen.
    
    Beispielnutzung:
    /usr/bin/ziemniak.txt kann mit /ziem[a-z]+ gefunden werden.
    
    Dies verwendet die Standard-Implementierung von Regex in Rust. Mehr dazu unter https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Aktiviert die Erkennung von Groß- und Kleinschreibungen.
    
    Wenn /home/* deaktiviert ist, findet /hoMe/roman und /home/roman.
popover_custom_not_all_check_button_tooltip =
    Verhindert die Auswahl aller Elemente in der Gruppe.
    
    Dies ist standardmäßig aktiviert, da der Benutzer in den meisten Situationen nicht sowohl Originaldateien als auch Duplikate löschen möchte, sondern mindestens eine der Dateien behalten will.
    
    Warnung: Diese Einstellung funktioniert nicht, wenn bereits alle Ergebnisse in der Gruppe manuell ausgewählt wurden.
popover_custom_regex_path_label = Pfad
popover_custom_regex_name_label = Name
popover_custom_regex_regex_label = Regex Pfad + Name
popover_custom_case_sensitive_check_button = Groß-/Kleinschreibung beachten
popover_custom_all_in_group_label = Nicht alle Datensätze in der Gruppe auswählen
popover_custom_mode_unselect = Eigene Abwählen
popover_custom_mode_select = Eigene auswählen
popover_sort_file_name = Dateiname
popover_sort_folder_name = Verzeichnisname
popover_sort_full_name = Vollständiger Name
popover_sort_size = Größe
popover_sort_selection = Auswahl
popover_invalid_regex = Regex ist ungültig
popover_valid_regex = Regex ist gültig
# Bottom buttons
bottom_search_button = Suchen
bottom_select_button = Auswählen
bottom_delete_button = Löschen
bottom_save_button = Speichern
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Bewegen
bottom_sort_button = Sortierung
bottom_search_button_tooltip = Suche starten
bottom_select_button_tooltip = Datensätze auswählen. Nur ausgewählte Dateien/Ordner können später verarbeitet werden.
bottom_delete_button_tooltip = Ausgewählte Dateien/Ordner löschen.
bottom_save_button_tooltip = Daten über die Suche in Datei speichern
bottom_symlink_button_tooltip =
    Erstelle symbolische Links.
    Funktioniert nur, wenn mindestens zwei Ergebnisse einer Gruppe ausgewählt sind.
    Das Erste bleibt dabei unverändert, und das Zweite und alle Weiteren werden zu symbolischen Links auf das Erste umgewandelt.
bottom_hardlink_button_tooltip =
    Erstelle Hardlinks.
    Funktioniert nur, wenn mindestens zwei Ergebnisse einer Gruppe ausgewählt sind.
    Das Erste bleibt dabei unverändert, und das Zweite und alle Weiteren werden zu Hardlinks auf das Erste umgewandelt.
bottom_hardlink_button_not_available_tooltip =
    Erstellen von Hardlinks.
    Button ist deaktiviert, da Hardlinks nicht erstellt werden können.
    Hardlinks funktionieren nur mit Administratorrechten unter Windows, also stellen Sie sicher, dass Sie die App als Administrator ausführen.
    Wenn die App bereits mit solchen Rechten arbeitet, überprüfen Sie auf Github auf ähnliche Probleme.
bottom_move_button_tooltip =
    Verschiebt Dateien in den ausgewählten Ordner.
    Kopiert alle Dateien in den Ordner, ohne den Verzeichnisbaum zu erhalten.
    Beim Versuch, zwei Dateien mit identischem Namen in einen Ordner zu verschieben, schlägt das Kopieren der Zweiten fehl und zeigt einen Fehler an.
bottom_sort_button_tooltip = Sortiert Dateien/Ordner nach der gewählten Methode.
bottom_show_errors_tooltip = Ein-/Ausblenden des unteren Textfeldes.
bottom_show_upper_notebook_tooltip = Ein-/Ausblenden des oberen Notizbuch-Panels.
# Progress Window
progress_stop_button = Stoppen
progress_stop_additional_message = Stopp angefordert
# About Window
about_repository_button_tooltip = Link zur Repository-Seite mit Quellcode.
about_donation_button_tooltip = Link zur Spendenseite.
about_instruction_button_tooltip = Link zur Anweisungsseite.
about_translation_button_tooltip = Link zur Crowdin-Seite mit App-Übersetzungen. Offiziell werden Polnisch und Englisch unterstützt.
about_repository_button = Projektarchiv
about_donation_button = Spende
about_instruction_button = Anleitung
about_translation_button = Übersetzung
# Header
header_setting_button_tooltip = Öffnet Einstellungsdialog.
header_about_button_tooltip = Öffnet den Dialog mit Informationen über die App.

# Settings


## General

settings_number_of_threads = Anzahl der verwendeten Threads
settings_number_of_threads_tooltip = Anzahl der verwendeten Threads, 0 bedeutet, dass alle verfügbaren Threads verwendet werden.
settings_use_rust_preview = Verwenden Sie stattdessen externe Bibliotheken gtk, um Vorschaubilder zu laden
settings_use_rust_preview_tooltip =
    Die Verwendung von gtk-Vorschauen ist manchmal schneller und unterstützt mehr Formate, aber manchmal kann dies genau das Gegenteil sein.
    
    Wenn Sie Probleme mit dem Laden von Vorschauen haben, können Sie versuchen, diese Einstellung zu ändern.
    
    Auf Nicht-Linux-Systemen wird empfohlen, diese Option zu verwenden, da gtk-pixbuf dort nicht immer verfügbar ist, so dass die Deaktivierung dieser Option keine Vorschau einiger Bilder laden wird.
settings_label_restart = Sie müssen die App neu starten, um die Einstellungen anzuwenden!
settings_ignore_other_filesystems = Andere Dateisysteme ignorieren (nur Linux)
settings_ignore_other_filesystems_tooltip =
    ignoriert Dateien, die nicht im selben Dateisystem sind wie durchsuchte Verzeichnisse.
    
    Funktioniert genauso wie die -xdev Option beim Finden des Befehls unter Linux
settings_save_at_exit_button_tooltip = Speichert die Konfiguration in einer Datei, wenn das Programm geschlossen wird.
settings_load_at_start_button_tooltip =
    Konfiguration aus der Datei laden, wenn App geöffnet wird.
    
    Falls das nicht aktiviert ist, werden die Standardeinstellungen verwendet.
settings_confirm_deletion_button_tooltip = Bestätigungsdialog anzeigen, wenn der Löschen-Knopf gedrückt wird.
settings_confirm_link_button_tooltip = Bestätigungsdialog anzeigen, wenn auf den Knopf Hard/Symlink gedrückt wird.
settings_confirm_group_deletion_button_tooltip = Warndialog anzeigen, wenn versucht wird, alle Datensätze aus einer Gruppe zu löschen.
settings_show_text_view_button_tooltip = Textfenster am unteren Rand der Benutzeroberfläche anzeigen.
settings_use_cache_button_tooltip = Datei-Cache verwenden.
settings_save_also_as_json_button_tooltip = Cache im (menschlich lesbaren) JSON-Format speichern. Es ist möglich, den Inhalt zu ändern. Der Cache wird automatisch aus dieser Datei von der App gelesen, wenn der Binärformat-Cache (mit .bin Erweiterung) fehlt.
settings_use_trash_button_tooltip = Wenn aktiviert, verschiebt Dateien in den Papierkorb, anstatt sie permanent zu löschen.
settings_language_label_tooltip = Sprache der Benutzeroberfläche.
settings_save_at_exit_button = Speichert die Konfiguration beim Schließen der App
settings_load_at_start_button = Konfiguration beim Öffnen der App laden
settings_confirm_deletion_button = Bestätigungsdialog beim Löschen von Dateien anzeigen
settings_confirm_link_button = Bestätigungsdialog anzeigen, wenn Hard/Symlinks irgendwelche Dateien
settings_confirm_group_deletion_button = Bestätigungsdialog beim Löschen aller Dateien in der Gruppe anzeigen
settings_show_text_view_button = Unteren Textbereich anzeigen
settings_use_cache_button = Cache verwenden
settings_save_also_as_json_button = Cache auch als JSON-Datei speichern
settings_use_trash_button = Gelöschte Dateien in den Papierkorb verschieben
settings_language_label = Sprache
settings_multiple_delete_outdated_cache_checkbutton = Veraltete Cache-Einträge automatisch löschen
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Ermöglicht das Löschen veralteter Cache-Ergebnisse, die auf nicht existierende Dateien verweisen.
    
    Wenn aktiviert, stellt das Programm sicher, dass beim Laden von Datensätzen alle auf gültige Dateien verweisen (kaputte Verweise werden ignorieren).
    
    Deaktivieren dieser Option wird helfen, Dateien auf externen Laufwerken zu scannen, so dass Cache-Einträge über diese nicht beim nächsten Scan gelöscht werden.
    
    Bei hunderttausenden Datensätzen im Cache wird empfohlen, diese Option zu aktivieren, um das Laden und Speichern des Caches am Anfang und am Ende des Scans zu beschleunigen.
settings_notebook_general = Allgemein
settings_notebook_duplicates = Duplikate
settings_notebook_images = Ähnliche Bilder
settings_notebook_videos = Ähnliches Video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Zeigt eine Vorschau auf der rechten Seite (bei der Auswahl einer Bilddatei).
settings_multiple_image_preview_checkbutton = Bildvorschau anzeigen
settings_multiple_clear_cache_button_tooltip =
    Leere den Cache manuell aus veralteten Einträgen.
    Das sollte nur verwendet werden, wenn das automatische Löschen deaktiviert wurde.
settings_multiple_clear_cache_button = Entferne veraltete Ergebnisse aus dem Cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Versteckt alle Dateien außer einem, wenn sie auf dieselben Daten (Hardlink) verweisen.
    
    Z.B. für den Fall, dass es (auf der Festplatte) sieben Dateien gibt, die an bestimmte Daten gelinkt sind und eine weiter Datei existiert mit denselben Daten, aber mit unterschiedlichem Inode, dann wird im Duplikatsucher nur eine Datei der sieben gelinkten und die seperate Datei sichtbar sein.
settings_duplicates_minimal_size_entry_tooltip =
    Legen Sie die minimale Dateigröße fest, die zwischengespeichert wird.
    
    Wenn Sie einen kleineren Wert wählen, werden mehr Datensätze generiert. Dies wird die Suche beschleunigen, aber das Laden und Speichern zum Cache verlangsamen.
settings_duplicates_prehash_checkbutton_tooltip =
    Aktiviert das Caching von Prehashes (Hash aus einem kleinen Teil der Datei), was es erlaubt, nicht duplizierte Ergebnisse schneller zu verwerfen.
    
    Es ist standardmäßig deaktiviert, da es in einigen Situationen zu Verlangsamungen führen kann.
    
    Es wird dringend empfohlen, es beim Scannen von Hunderttausenden oder Millionen Dateien zu verwenden, da es die Suche um ein Vielfaches beschleunigen kann.
settings_duplicates_prehash_minimal_entry_tooltip = Minimale Größe des zwischengespeicherten Eintrags.
settings_duplicates_hide_hard_link_button = Verstecke Hardlinks (nur Linux und macOS)
settings_duplicates_prehash_checkbutton = Prehash-Cache verwenden
settings_duplicates_minimal_size_cache_label = Minimale Dateigröße (in Bytes), die im Cache gespeichert wird
settings_duplicates_minimal_size_cache_prehash_label = Minimale Dateigröße (in Bytes), die im Prehash-Cache gespeichert wird

## Saving/Loading settings

settings_saving_button_tooltip = Aktuelle Einstellungen in Datei speichern.
settings_loading_button_tooltip = Lade die Einstellungen aus einer Datei und ersetze die aktuellen Einstellungen mit diesen.
settings_reset_button_tooltip = Aktuelle Konfiguration auf Standardeinstellung zurücksetzen.
settings_saving_button = Konfiguration speichern
settings_loading_button = Konfiguration laden
settings_reset_button = Konfiguration zurücksetzen

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Öffnet den Ordner, in dem txt-Dateien mit Cache-Daten gespeichert sind.
    
    Änderungen an den Cache-Dateien kann dazu führen, dass ungültige Ergebnisse angezeigt werden. Aber Modifikation des Pfades kann Zeit sparen, wenn eine große Anzahl von Dateien verschoben werden.
    
    Sie können diese Dateien zwischen Computern kopieren, um Zeit beim erneuten Scannen von Dateien zu sparen (natürlich nur, wenn diese eine ähnliche Verzeichnisstruktur haben).
    
    Bei Problemen mit dem Cache können diese Dateien entfernt werden, sodass die App sie automatisch neu generiert.
settings_folder_settings_open_tooltip =
    Öffnet den Ordner, in dem die Czkawka Konfiguration gespeichert ist.
    
    WARNUNG: Manuelle Änderung der Konfiguration kann Ihren Workflow stören.
settings_folder_cache_open = Cache-Ordner öffnen
settings_folder_settings_open = Einstellungsordner öffnen
# Compute results
compute_stopped_by_user = Suche wurde vom Benutzer gestoppt
compute_found_duplicates_hash_size = { $number_files } Duplikate in { $number_groups } Gruppen gefunden, die { $size } benötigt haben
compute_found_duplicates_name = { $number_files } Duplikate in { $number_groups } Gruppen gefunden
compute_found_empty_folders = { $number_files } leere Ordner gefunden
compute_found_empty_files = { $number_files } leere Dateien gefunden
compute_found_big_files = { $number_files } große Dateien gefunden
compute_found_temporary_files = { $number_files } temporäre Dateien gefunden
compute_found_images = { $number_files } ähnliche Bilder in { $number_groups } Gruppen gefunden
compute_found_videos = { $number_files } ähnliche Videos in { $number_groups } Gruppen gefunden
compute_found_music = { $number_files } ähnliche Musikdateien in { $number_groups } Gruppen gefunden
compute_found_invalid_symlinks = { $number_files } ungültige Symlinks gefunden
compute_found_broken_files = { $number_files } fehlerhafte Dateien gefunden
compute_found_bad_extensions = { $number_files } Dateien mit ungültigen Endungen gefunden
# Progress window
progress_scanning_general_file = Scanne { $file_number } Dateien
progress_scanning_extension_of_files = Überprüfe Erweiterung von { $file_checked }/{ $all_files } Datei
progress_scanning_broken_files = Überprüfe { $file_checked }/{ $all_files } Datei
progress_scanning_video = Hashwerte für Videos berechnet: { $file_checked }/{ $all_files }
progress_scanning_image = Hashwerte für Bilder berechnet:{ $file_checked }/{ $all_files }
progress_comparing_image_hashes = Vergleicht { $file_checked }/{ $all_files } Bild-Hash
progress_scanning_music_tags_end = Vergleicht Tags von { $file_checked }/{ $all_files } Musikdatei
progress_scanning_music_tags = Lese Tags von { $file_checked }/{ $all_files } Musikdatei
progress_scanning_music_content_end = Vergleiche Fingerabdruck von { $file_checked }/{ $all_files } Musikdatei
progress_scanning_music_content = Berechne Fingerabdruck von { $file_checked }/{ $all_files } Musikdatei
progress_scanning_empty_folders = Scanne { $folder_number } Ordner
progress_scanning_size = Scanne Größe der { $file_number } Dateien
progress_scanning_size_name = Scanne Namen und Größe der { $file_number } Dateien
progress_scanning_name = Scanne Name der { $file_number } Dateien
progress_analyzed_partial_hash = Teilhash von { $file_checked }/{ $all_files } Dateien analysiert
progress_analyzed_full_hash = Vollen Hash von { $file_checked } von { $all_files } Dateien analysiert
progress_prehash_cache_loading = Lade Vorhash-Cache
progress_prehash_cache_saving = Speichere Vorhash-Cache
progress_hash_cache_loading = Hash-Cache wird geladen
progress_hash_cache_saving = Speichere Hash-Cache
progress_cache_loading = Cache wird geladen
progress_cache_saving = Cache speichern
progress_current_stage = Aktueller Prozess:{ " " }
progress_all_stages = Gesamtprozess:{ " " }
# Saving loading 
saving_loading_saving_success = Konfiguration in Datei { $name } gespeichert.
saving_loading_saving_failure = Konfigurationsdaten konnten nicht in Datei { $name } gespeichert werden.
saving_loading_reset_configuration = Aktuelle Konfiguration wurde gelöscht.
saving_loading_loading_success = Richtig geladene App-Konfiguration.
saving_loading_invalid_string = Für Schlüssel "{ $key }" ungültiges Ergebnis gefunden: "{ $result }", welches keine Zeichenkette ist.
saving_loading_invalid_int = Für Schlüssel "{ $key }" ungültiges Ergebnis gefunden: "{ $result }", welches keine ganze Zahl ist.
saving_loading_invalid_bool = Für Schlüssel "{ $key }" ungültiges Ergebnis gefunden: "{ $result }", welches kein Boolean ist.
saving_loading_decode_problem_bool = Fehler beim Dekodieren von Schlüssel "{ $key }" gefunden: "{ $result }". Erlaubte Werte sind: 0, 1, true oder false.
saving_loading_saving_same_keys = Versucht die Einstellung mit doppelter Taste "{ $key } " zu speichern.
saving_loading_failed_to_get_home_directory = Home-Verzeichnis konnte nicht zum Öffnen und Speichern der Konfigurationsdatei geladen werden.
saving_loading_folder_config_instead_file = Konfigurationsdatei im Pfad "{ $path }" kann nicht erstellt oder geöffnet werden, da bereits ein Ordner vorhanden ist.
saving_loading_failed_to_create_configuration_folder = Konfigurationsordner konnte nicht erstellt werden "{ $path }", Grund "{ $reason }".
saving_loading_failed_to_create_config_file = Fehler beim Erstellen der Konfigurationsdatei "{ $path }", Grund "{ $reason }".
saving_loading_failed_to_read_config_file = Konfiguration kann nicht von "{ $path }" geladen werden, da sie nicht existiert oder keine Datei ist.
saving_loading_failed_to_read_data_from_file = Daten von Datei "{ $path }" können nicht gelesen werden, Grund "{ $reason }".
saving_loading_orphan_data = Verwaiste Daten "{ $data }" in Zeile "{ $line }".
saving_loading_not_valid = Einstellung "{ $data }" existiert nicht in der aktuellen App-Version.
# Invalid symlinks
invalid_symlink_infinite_recursion = Endlose Rekursion
invalid_symlink_non_existent_destination = Nicht existierende Zieldatei
# Other
selected_all_reference_folders = Suche kann nicht gestartet werden, wenn alle Verzeichnisse als Referenzordner gesetzt sind
searching_for_data = Suche nach Daten, es kann eine Weile dauern, bitte warten...
text_view_messages = NACHRICHT
text_view_warnings = WARNUNGEN
text_view_errors = FEHLER
about_window_motto = Dieses Programm ist kostenlos zu benutzen und wird immer frei sein.
# Various dialog
dialogs_ask_next_time = Nächstes Mal fragen
delete_file_failed = Fehler beim Entfernen der Datei { $name }, aufgrund von { $reason }
delete_title_dialog = Löschen bestätigen
delete_question_label = Sind Sie sicher, dass Sie Dateien löschen möchten?
delete_all_files_in_group_title = Löschen aller Dateien in der Gruppe bestätigen
delete_all_files_in_group_label1 = In einigen Gruppen sind alle Datensätze ausgewählt.
delete_all_files_in_group_label2 = Sind Sie sicher, dass Sie sie löschen möchten?
delete_folder_failed = Fehler beim Entfernen des Ordners { $dir }, da der Ordner nicht existiert, Sie keine Berechtigung dafür haben oder der Ordner nicht leer ist.
delete_items_label = { $items } Dateien werden gelöscht.
delete_items_groups_label = { $items } Dateien aus { $groups } Gruppen werden gelöscht.
hardlink_failed = Hardlink fehlgeschlagen
hard_sym_invalid_selection_title_dialog = Ungültige Auswahl bei einigen Gruppen
hard_sym_invalid_selection_label_1 = In einigen Gruppen ist nur ein Datensatz ausgewählt und dieser wird ignoriert.
hard_sym_invalid_selection_label_2 = Um diese Dateien hart/symbolisch zu verlinken, müssen mindestens zwei Ergebnisse in der Gruppe ausgewählt werden.
hard_sym_invalid_selection_label_3 = Erster der Gruppe als Original erkannt und nicht geändert, sondern zweiter und weitere modifiziert.
hard_sym_link_title_dialog = Link-Bestätigung
hard_sym_link_label = Sind Sie sicher, dass Sie diese Dateien verknüpfen möchten?
move_folder_failed = Fehler beim Verschieben des Ordners { $name }, Grund { $reason }
move_file_failed = Fehler beim Verschieben der Datei { $name }, Grund { $reason }
move_files_title_dialog = Wählen Sie den Ordner aus, in den Sie doppelte Dateien verschieben möchten
move_files_choose_more_than_1_path = Es darf nur ein Pfad ausgewählt sein, um Duplikate von dort kopieren zu können, ausgewählt sind { $path_number }.
move_stats = { $num_files }/{ $all_files } Elemente korrekt verschoben
save_results_to_file = Ergebnisse sowohl in txt als auch in json Dateien in den Ordner { $name } gespeichert.
search_not_choosing_any_music = FEHLER: Sie müssen mindestens ein Kontrollkästchen mit Art der Musiksuche auswählen.
search_not_choosing_any_broken_files = FEHLER: Sie müssen mindestens ein Kontrollkästchen mit der Art der markierten fehlerhaften Dateien auswählen.
include_folders_dialog_title = Einbezogene Ordner
exclude_folders_dialog_title = Ausgeschlossene Ordner
include_manually_directories_dialog_title = Verzeichnis manuell hinzufügen
cache_properly_cleared = Cache vollständig geleert
cache_clear_duplicates_title = Leere Duplikate-Cache
cache_clear_similar_images_title = Leere Bilder-Cache
cache_clear_similar_videos_title = Leere Video-Cache
cache_clear_message_label_1 = Wollen Sie veraltete Einträge aus dem Cache löschen?
cache_clear_message_label_2 = Dieser Vorgang entfernt alle Cache-Einträge, die auf ungültige Dateien verweisen.
cache_clear_message_label_3 = Dies kann das Laden und Speichern zum Cache leicht beschleunigen.
cache_clear_message_label_4 = ACHTUNG: Die Operation wird alle zwischengespeicherten Daten von nicht verbundenen externen Laufwerken entfernen, somit muss jeder Hash erneut generiert werden.
# Show preview
preview_image_resize_failure = Fehler beim Ändern der Bildgröße { $name }.
preview_image_opening_failure = Konnte Bild { $name } nicht öffnen, Grund { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Gruppe { $current_group }/{ $all_groups } ({ $images_in_group } Bilder)
compare_move_left_button = L
compare_move_right_button = R
