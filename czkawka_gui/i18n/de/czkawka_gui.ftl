# Window titles
window_settings_title = Optionen
window_main_title = Czkawka (Schluckauf)
window_progress_title = Scannen
window_compare_images = Bilder vergleichen
# General
general_ok_button = Ok
general_close_button = Schließen
# Main window
music_title_checkbox = Titel
music_artist_checkbox = Künstler
music_album_title_checkbox = Albumtitel
music_album_artist_checkbox = Album-Interpret
music_year_checkbox = Jahr
music_comparison_checkbox = Ungefährer Vergleich
music_comparison_checkbox_tooltip =
    Es sucht nach ähnlichen Musikdateien mit Hilfe von AI, die Klammern aus einer Phrase entfernen, z. wenn diese Option aktiviert ist, die fraglichen Dateien werden als Duplikate betrachtet:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Name
duplicate_mode_size_combo_box = Größe
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka bietet 3 Arten von Hashes an, die verwendet werden können:
    
    Blake3 - kryptographische Hashfunktion. Er wird als Standard-Hash-Algorithmus verwendet, da er sehr schnell ist.
    
    CRC32 - einfache Hash-Funktion. Es sollte schneller sein als Blake3, aber wahrscheinlich sehr selten einige Kollisionen.
    
    XXH3 - sehr ähnlich bei Leistung und Hashqualität wie Blake3, so dass solche Modi leicht verwendet werden können.
duplicate_check_method_tooltip =
    Zurzeit bietet Czkawka drei Arten von Methoden an, um Duplikate zu finden:
    
    Name - Findet Dateien mit gleichem Namen.
    
    Größe - Findet Dateien mit gleicher Größe.
    
    Hash - Findet Dateien mit dem gleichen Inhalt. Dieser Modus hasht die Datei und vergleicht diese Hashes später, um Duplikate zu finden. Dieser Modus ist der sicherste Weg, um Duplikate zu finden. Das Tool verwendet den Cache, daher sollten die zweiten und weiteren Scans der gleichen Daten viel schneller sein als zuerst.
image_hash_size_tooltip =
    Czkawka bietet eine veränderte Größe des generierten Hashs für jedes Bild. Größere Hash-Ursache erlaubt es, Bilder mit geringerer Anzahl von Unterschieden zwischen den Bildern zu finden, aber auch ist es etwas langsamer zu verwenden.
    
    Der Standardwert für den Hash ist 8 Bytes, wodurch sehr ähnliche und unterschiedliche Bilder gefunden werden können. 16 und 32 Hashes sollten nur für nahezu identische Bilder verwendet werden. 64 Bytes Hash sollte nicht verwendet werden, außer wenn wirklich kleine Unterschiede benötigt werden, um zu finden
image_resize_filter_tooltip = Um den Hash des Bildes zu berechnen, muss die Bibliothek zuerst die Größe des Bildes verändern. Abhängig vom gewählten Algorithmus sieht das Ergebnis wenig anders aus. Der schnellste Algotithmus zu verwenden, aber auch einer, der die schlechtesten Ergebnisse gibt ist in der Nähe.
image_hash_alg_tooltip = Benutzer können einen aus vielen Algorithmen zur Berechnung des Hashs auswählen. Jeder hat sowohl starke als auch schwächere Punkte und wird manchmal bessere und manchmal schlechtere Ergebnisse für verschiedene Bilder liefern um die beste zu wählen, ist eine manuelle Prüfung erforderlich.
main_notebook_image_fast_compare = Schneller Vergleich
main_notebook_image_fast_compare_tooltip =
    Beschleunige die Suche und den Vergleich von Hashes.
    
    Im Gegensatz zum normalen Modus, in dem jeder Hash x-mal miteinander verglichen wird wobei x die Ähnlichkeit ist, die der Benutzer wählt, in diesem Modus wird immer nur ein Vergleich verwendet.
    
    Diese Option wird empfohlen, wenn >10000 Bilder mit nicht 0 (Very High) Ähnlichkeit verglichen werden.
main_notebook_duplicates = Dateien Duplizieren
main_notebook_empty_directories = Leere Verzeichnisse
main_notebook_big_files = Große Dateien
main_notebook_empty_files = Leere Dateien
main_notebook_temporary = Temporäre Dateien
main_notebook_similar_images = Ähnliche Bilder
main_notebook_similar_videos = Ähnliche Videos
main_notebook_same_music = Musik Duplikate
main_notebook_symlinks = Ungültige Symlinks
main_notebook_broken_files = Defekte Dateien
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
main_tree_view_column_album_title = Albumtitel
main_tree_view_column_album_artist = Album-Interpret
main_tree_view_column_symlink_file_name = Symlink Dateiname
main_tree_view_column_symlink_folder = Symlnik Ordner
main_tree_view_column_destination_path = Zielpfad
main_tree_view_column_type_of_error = Fehlertyp
main_label_check_method = Prüfmethode
main_label_hash_type = Hash Typ
main_label_hash_size = Hash Größe
main_label_size_bytes = Größe (Bytes)
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Anzahl der angezeigten Dateien
main_label_resize_algorithm = Algorithmus skalieren
main_label_similarity = Ähnlichkeit{ " " }
check_button_general_same_size = Gleiche Größe ignorieren
check_button_general_same_size_tooltip = Ignoriere von Ergebnissen, Dateien mit identischer Größe - normalerweise sind dies 1:1 Duplikate
main_label_size_bytes_tooltip = Größe der Dateien, die beim Scannen verwendet werden
# Upper window
upper_tree_view_included_folder_column_title = Zu suchende Ordner
upper_tree_view_included_reference_column_title = Referenzordner
upper_recursive_button = Rekursiv
upper_recursive_button_tooltip = Falls ausgewählt, suchen Sie auch nach Dateien, die nicht direkt unter den ausgewählten Ordnern platziert werden.
upper_manual_add_included_button = Manuell hinzufügen
upper_add_included_button = Neu
upper_remove_included_button = Entfernen
upper_manual_add_excluded_button = Manuell hinzufügen
upper_add_excluded_button = Neu
upper_remove_excluded_button = Entfernen
upper_manual_add_included_button_tooltip = Ermöglicht das Hinzufügen von Verzeichnisnamen zur Suche per Hand.
upper_add_included_button_tooltip = Neues Verzeichnis zur Suche hinzufügen.
upper_remove_included_button_tooltip = Verzeichnis von der Suche löschen.
upper_manual_add_excluded_button_tooltip = Erlaubt das Hinzufügen ausgeschlossener Verzeichnisnamen von Hand.
upper_add_excluded_button_tooltip = Verzeichnis hinzufügen, das bei der Suche ausgeschlossen werden soll.
upper_remove_excluded_button_tooltip = Verzeichnis von ausgeschlossen löschen.
upper_notebook_items_configuration = Artikelkonfiguration
upper_notebook_excluded_directories = Ausgeschlossene Verzeichnisse
upper_notebook_included_directories = Inklusive Verzeichnisse
upper_allowed_extensions_tooltip =
    Erlaubte Erweiterungen müssen durch Kommas getrennt werden (standardmäßig sind alle verfügbar).
    
    Makros IMAGE, VIDEO, MUSIC, TEXT, die mehrere Erweiterungen gleichzeitig hinzufügen, sind ebenfalls verfügbar.
    
    Beispiel ".exe, IMAGE, VIDEO, .rar, 7z" - das bedeutet, dass image(jpg, png ...), Videodateien (avi, mp4 ...), exe, rar und 7z werden gescannt.
upper_excluded_items_tooltip =
    Ausgeschlossene Elemente müssen * Platzhalter enthalten und durch Kommata getrennt werden.
    Dies ist langsamer als ausgeschlossene Verzeichnisse, also vorsichtig verwenden.
upper_excluded_items = Ausgeschlossene Elemente:
upper_allowed_extensions = Erlaubte Erweiterungen:
# Popovers
popover_select_all = Alles auswählen
popover_unselect_all = Alles abwählen
popover_reverse = Auswahl umkehren
popover_select_all_except_oldest = Alle außer Ältesten auswählen
popover_select_all_except_newest = Alle außer neueste auswählen
popover_select_one_oldest = Wähle einen Ältesten
popover_select_one_newest = Wählen Sie eine neueste
popover_select_custom = Eigene auswählen
popover_unselect_custom = Eigene Abwählen
popover_select_all_images_except_biggest = Alle außer Größten auswählen
popover_select_all_images_except_smallest = Alle außer kleinsten auswählen
popover_custom_path_check_button_entry_tooltip =
    Ermöglicht die Auswahl von Datensätzen nach dem Pfad.
    
    Beispielverwendung:
    /home/pimpek/rzecz.txt kann mit /home/pim* gefunden werden
popover_custom_name_check_button_entry_tooltip =
    Ermöglicht die Auswahl von Datensätzen nach Dateinamen.
    
    Beispielnutzung:
    /usr/ping/pong.txt kann mit *ong* gefunden werden
popover_custom_regex_check_button_entry_tooltip =
    Ermöglicht die Auswahl von Datensätzen von angegebenen Regex.
    
    Mit diesem Modus ist gesuchter Text Pfad mit Namen.
    
    Beispielnutzung:
    /usr/bin/ziemniak. xt kann mit /ziem gefunden ziem[a-z]+
    
    Diese verwenden Standard-Rust regex Implementierung, so dass du mehr darüber unter https://docs.rs/regex erfahren kannst.
popover_custom_not_all_check_button_tooltip =
    Verhindert die Auswahl aller Datensätze in der Gruppe.
    
    Dies ist standardmäßig aktiviert, da der Benutzer in den meisten Situationen nicht sowohl Originaldateien als auch Dateien duplizieren möchte aber wollen mindestens eine Datei hinterlassen.
    
    Warnung: Diese Einstellung funktioniert nicht, wenn bereits alle Ergebnisse in der Gruppe manuell ausgewählt wurden.
popover_custom_regex_path_label = Pfad
popover_custom_regex_name_label = Name
popover_custom_regex_regex_label = Regex Pfad + Name
popover_custom_all_in_group_label = Nicht alle Datensätze in der Gruppe auswählen
popover_custom_mode_unselect = Eigene Abwählen
popover_custom_mode_select = Eigene auswählen
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
bottom_search_button_tooltip = Suche nach Dateien/Ordnern starten.
bottom_select_button_tooltip = Wählt Datensätze aus. Nur ausgewählte Dateien/Ordner können später verarbeitet werden.
bottom_delete_button_tooltip = Ausgewählte Dateien/Ordner löschen.
bottom_save_button_tooltip = Daten über die Suche in Datei speichern
bottom_symlink_button_tooltip =
    Erstellt symbolische Links.
    Funktioniert nur, wenn mindestens 2 Ergebnisse in der Gruppe ausgewählt sind.
    Der erste ist unverändert und der zweite und spätere ist mit dem ersten symbolisch verknüpft.
bottom_hardlink_button_tooltip =
    Erstellt Hardlinks.
    Funktioniert nur, wenn mindestens 2 Ergebnisse in der Gruppe ausgewählt sind.
    Der erste ist unverändert und der zweite und spätere ist mit dem ersten fest verknüpft.
bottom_move_button_tooltip =
    Verschiebt Dateien in den ausgewählten Ordner.
    Kopiert alle Dateien in den Ordner, ohne den Verzeichnisbaum zu erhalten.
    Beim Versuch, zwei Dateien mit identischem Namen in einen Ordner zu verschieben, schlägt die Sekunde fehlund zeigt einen Fehler an.
bottom_show_errors_tooltip = Ein-/Ausblenden des unteren Fehlerfensters.
bottom_show_upper_notebook_tooltip = Ein-/Ausblenden des oberen Notizbuch-Panels.
# Progress Window
progress_stop_button = Stoppen
# About Window
about_repository_button_tooltip = Link zur Repository-Seite mit Quellcode.
about_donation_button_tooltip = Link zur Spendenseite.
about_instruction_button_tooltip = Link zur Anweisungsseite.
about_translation_button_tooltip = Link zur Crowdin-Seite mit App-Übersetzungen. Offiziell werden Polnisch und Englisch unterstützt, aber jede Hilfe mit anderen Sprachen wird geschätzt.
about_repository_button = Projektarchiv
about_donation_button = Spende
about_instruction_button = Anleitung
about_translation_button = Übersetzung
# Header
header_setting_button_tooltip = Öffnet Einstellungsdialog.
header_about_button_tooltip = Öffnet den Dialog mit Informationen über die App.

# Settings


## General

settings_save_at_exit_button_tooltip = Speichert die Konfiguration in Datei beim Schließen der App.
settings_load_at_start_button_tooltip =
    Laden bei der Start-Konfiguration von Datei.
    
    Wenn diese Option nicht ausgewählt wird, werden die Standardeinstellungen geladen.
settings_confirm_deletion_button_tooltip = Zeigt den Bestätigungsdialog an, wenn Sie auf den Löschen-Knopf klicken.
settings_confirm_link_button_tooltip = Zeigt den Bestätigungsdialog an, wenn Sie auf den Knopf Hard/Symlink klicken.
settings_confirm_group_deletion_button_tooltip = Zeigt den Dialog an, wenn versucht wird, alle Datensätze aus der Gruppe zu entfernen.
settings_show_text_view_button_tooltip = Zeigt die Fehleranzeige unten an.
settings_use_cache_button_tooltip = Option, bei der die Cache-Funktion nicht genutzt werden kann.
settings_save_also_as_json_button_tooltip = Speichere Cache im menschlichen JSON-Format. Es ist möglich, den Inhalt zu ändern. Cache aus dieser Datei wird automatisch von der App gelesen, wenn der Binärformat-Cache (mit Bin Erweiterung) fehlt.
settings_use_trash_button_tooltip = Wenn aktiviert, verschiebt es Dateien in den Papierkorb.
settings_language_label_tooltip = Erlaubt es, die Sprache der Schnittstelle von der verfügbaren zu wählen.
settings_save_at_exit_button = Konfiguration beim Beenden speichern
settings_load_at_start_button = Konfiguration beim Start laden
settings_confirm_deletion_button = Bestätigungsdialog beim Löschen von Dateien anzeigen
settings_confirm_link_button = Bestätigungsdialog anzeigen, wenn Hard/Symlinks irgendwelche Dateien
settings_confirm_group_deletion_button = Bestätigungsdialog beim Löschen aller Dateien in der Gruppe anzeigen
settings_show_text_view_button = Unteren Textbereich anzeigen
settings_use_cache_button = Cache verwenden
settings_save_also_as_json_button = Cache auch in JSON-Datei speichern
settings_use_trash_button = Gelöschte Dateien in den Papierkorb verschieben
settings_language_label = Sprache
settings_multiple_delete_outdated_cache_checkbutton = Veraltete Cache-Einträge automatisch löschen
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Ermöglicht das Löschen veralteter Cache-Ergebnisse, die auf nicht existierende Dateien verweisen.
    
    Wenn aktiviert, vergewissern Sie sich, dass beim Laden von Datensätzen alle auf gültige Dateien verweisen und kaputte Dateien ignorieren.
    
    Deaktivieren dieser Option, wird helfen, Dateien auf externen Laufwerken zu scannen, so dass Cache-Einträge über diese nicht beim nächsten Scan gelöscht werden.
    
    Bei hunderttausenden Datensätzen im Cache wird empfohlen, diese Option zu aktivieren, um das Laden und Speichern des Caches am Anfang und am Ende des Scans zu beschleunigen.
settings_notebook_general = Allgemein
settings_notebook_duplicates = Duplikate
settings_notebook_images = Ähnliche Bilder
settings_notebook_videos = Ähnliches Video

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Zeigt eine Vorschau auf der rechten Seite bei der Auswahl der Bilddatei.
settings_multiple_image_preview_checkbutton = Bildvorschau anzeigen
settings_multiple_clear_cache_button_tooltip =
    Leere den Cache manuell aus veralteten Einträgen.
    sollte nur verwendet werden, wenn das automatische Löschen deaktiviert wurde.
settings_multiple_clear_cache_button = Entferne veraltete Ergebnisse aus dem Bildercache

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Versteckt alle Dateien außer einem, wenn sie auf dieselben Daten (Hardlink) verweisen.
    
    Z.B. für den Fall, dass es auf der Festplatte 7 Dateien gibt, die an bestimmte Daten gebunden sind und eine andere Datei mit denselben Daten, aber unterschiedlichem inode, dann wird im Duplikat Finder nur eine eindeutige Datei und eine Datei von hardlinked sichtbar sein.
settings_duplicates_minimal_size_entry_tooltip =
    Erlaubt es, die minimale Dateigröße festzulegen, die zwischengespeichert wird.
    
    Wenn Sie einen kleineren Wert wählen, werden mehr Datensätze generiert, die die Suche beschleunigen, aber das Laden und Speichern des Caches verlangsamen.
settings_duplicates_prehash_checkbutton_tooltip =
    Aktiviert das Caching von prehash(hash-berechneter Hash aus einem kleinen Teil der Datei), was es erlaubt, vorher nicht duplizierte Ergebnisse zu werfen.
    
    Es ist standardmäßig deaktiviert, da es in einigen Situationen zu Verlangsamungen führen kann.
    
    Es wird dringend empfohlen, es beim Scannen von Hunderttausenden oder Millionen Dateien zu verwenden, da es die Suche mehrmals beschleunigen kann.
settings_duplicates_prehash_minimal_entry_tooltip = Minimale Größe des zwischengespeicherten Eintrags.
settings_duplicates_hide_hard_link_button = Verstecke harte Links (nur Linux und MacOS)
settings_duplicates_prehash_checkbutton = Vorhash-Cache verwenden
settings_duplicates_minimal_size_cache_label = Minimale Dateigröße in Bytes im Cache gespeichert
settings_duplicates_minimal_size_cache_prehash_label = Minimale Dateigröße in Bytes im Vorhash-Cache gespeichert

## Saving/Loading settings

settings_saving_button_tooltip = Aktuelle Einstellungen in Datei speichern.
settings_loading_button_tooltip = Laden Sie die Einstellungen aus der Datei und ersetzen Sie die aktuelle Konfiguration mit ihnen.
settings_reset_button_tooltip = Aktuelle Konfiguration auf Standardeinstellung zurücksetzen.
settings_saving_button = Konfiguration speichern
settings_loading_button = Konfiguration laden
settings_reset_button = Konfiguration zurücksetzen

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Öffnet den Ordner, in dem txt-Dateien mit Cache gespeichert werden.
    
    Ändern kann dazu führen, dass ungültige Ergebnisse angezeigt werden, aber auch Modifizieren e.. Der Pfad kann Zeit sparen, wenn große Dateigröße an verschiedene Orte verschoben werden.
    
    Sie können diese Dateien zwischen den Computern kopieren, um Zeit beim erneuten Scannen von Dateien zu sparen (natürlich wenn diese ähnliche Verzeichnisstruktur haben).
    
    Bei Problemen mit dem Cache können diese Dateien entfernt werden, so dass die App sie automatisch neu generiert.
settings_folder_settings_open_tooltip =
    Öffnet den Ordner, in dem Czkawka Konfiguration gespeichert wird.
    
    Ändern Sie ihn von der Hand, kann dazu führen, dass Ihr Workflow nicht mehr funktioniert.
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
# Progress window
progress_scanning_general_file = Scanne { $file_number } Datei
progress_scanning_broken_files = Überprüfe { $file_checked }/{ $all_files } Datei
progress_scanning_video = Hashing von { $file_checked }/{ $all_files } Video
progress_scanning_image = Hashing von { $file_checked }/{ $all_files } Bild
progress_comparing_image_hashes = Vergleicht { $file_checked }/{ $all_files } Bild-Hash
progress_scanning_music_tags_end = Vergleicht Tags von { $file_checked }/{ $all_files } Musikdatei
progress_scanning_music_tags = Lese Tags von { $file_checked }/{ $all_files } Musikdatei
progress_scanning_empty_folders = Scanne { $folder_number } Ordner
progress_scanning_size = Scanne Größe der { $file_number } Datei
progress_scanning_name = Scanne Name der { $file_number } Datei
progress_analyzed_partial_hash = Analysierter Teilhash von { $file_checked }/{ $all_files } Dateien
progress_analyzed_full_hash = Analysiert voller Hash von { $file_checked }/{ $all_files } Dateien
progress_current_stage = Aktuelle Stufe:{ " " }
progress_all_stages = Alle Stufen:{ " " }
# Saving loading 
saving_loading_saving_success = Konfiguration in Datei { $name } gespeichert.
saving_loading_saving_failure = Konfigurationsdaten konnten nicht in Datei { $name } gespeichert werden.
saving_loading_reset_configuration = Aktuelle Konfiguration wurde gelöscht.
saving_loading_loading_success = Richtig geladene App-Konfiguration.
saving_loading_invalid_string = Für Schlüssel "{ $key }" fand ungültiges Ergebnis - "{ $result }" das keine Zeichenkette ist.
saving_loading_invalid_int = Für Schlüssel "{ $key }" fand ungültiges Ergebnis - "{ $result }" das keine ganze Zahl ist.
saving_loading_invalid_bool = Für Schlüssel "{ $key }" fand ungültiges Ergebnis - "{ $result }" das kein Bool.
saving_loading_decode_problem_bool = Fehler beim Dekodieren des Bools von Schlüssel "{ $key }" gefunden "{ $result }" aber erlaubte Werte sind 0, 1, true oder false.
saving_loading_saving_same_keys = Versucht die Einstellung mit doppelter Taste "{ $key } " zu speichern.
saving_loading_failed_to_get_home_directory = Home-Verzeichnis konnte nicht zum Öffnen und Speichern der Konfigurationsdatei geladen werden.
saving_loading_folder_config_instead_file = Konfigurationsdatei im Pfad "{ $path }" kann nicht erstellt oder geöffnet werden, da bereits ein Ordner vorhanden ist.
saving_loading_failed_to_create_configuration_folder = Konfigurationsordner konnte nicht erstellt werden "{ $path }", Grund "{ $reason }".
saving_loading_failed_to_create_config_file = Fehler beim Erstellen der Konfigurationsdatei "{ $path }", Grund "{ $reason }".
saving_loading_failed_to_read_config_file = Konfiguration kann nicht von "{ $path }" geladen werden, da es nicht existiert oder keine Datei ist.
saving_loading_failed_to_read_data_from_file = Daten von Datei "{ $path }" können nicht gelesen werden, Grund "{ $reason }".
saving_loading_orphan_data = Verwaiste Daten "{ $data }" in Zeile "{ $line }".
saving_loading_not_valid = Einstellung "{ $data }" existiert nicht in der aktuellen App-Version.
# Invalid symlinks
invalid_symlink_infinite_recursion = Endlose Rekursion
invalid_symlink_non_existent_destination = Nicht existierende Zieldatei
# Other
searching_for_data = Suche nach Daten, es kann eine Weile dauern, bitte warten...
text_view_messages = NACHRICHT
text_view_warnings = WARNUNGEN
text_view_errors = FEHLER
about_window_motto = Dieses Programm ist kostenlos zu benutzen und wird es immer sein.
# Various dialog
dialogs_ask_next_time = Nächstes Mal fragen
delete_file_failed = Fehler beim Entfernen der Datei { $name }, Grund { $reason }
delete_title_dialog = Lösche Bestätigung
delete_question_label = Sind Sie sicher, dass Sie Dateien löschen möchten?
delete_all_files_in_group_title = Löschen aller Dateien in der Gruppe bestätigen
delete_all_files_in_group_label1 = In einigen Gruppen werden alle Datensätze ausgewählt.
delete_all_files_in_group_label2 = Sind Sie sicher, dass Sie sie löschen möchten?
delete_folder_failed = Fehler beim Entfernen des Ordners { $dir } , da der Ordner nicht existiert. Sie haben keine Berechtigung oder sind nicht leer.
delete_items_label = { $items } Dateien werden entfernt.
delete_items_groups_label = { $items } Dateien aus { $groups } Gruppen werden entfernt.
hardlink_failed = Hardlink fehlgeschlagen
hard_sym_invalid_selection_title_dialog = Ungültige Auswahl mit einigen Gruppen
hard_sym_invalid_selection_label_1 = In einigen Gruppen ist nur 1 Datensatz ausgewählt und wird ignoriert.
hard_sym_invalid_selection_label_2 = Um diese Dateien zu verlinken, müssen mindestens 2 Ergebnisse in der Gruppe ausgewählt werden.
hard_sym_invalid_selection_label_3 = Zuerst wird in der Gruppe als Original erkannt und nicht geändert, sondern als zweites und späteres modifiziert.
hard_sym_link_title_dialog = Link-Bestätigung
hard_sym_link_label = Sind Sie sicher, dass Sie diese Dateien verknüpfen möchten?
move_folder_failed = Fehler beim Verschieben des Ordners { $name }, Grund { $reason }
move_file_failed = Fehler beim Verschieben der Datei { $name }, Grund { $reason }
move_files_title_dialog = Wählen Sie den Ordner aus, in den Sie doppelte Dateien verschieben möchten
move_files_choose_more_than_1_path = Es muss nur 1 Pfad ausgewählt sein, um Duplikate kopieren zu können, ausgewählt { $path_number }.
move_stats = Zu Recht verschoben { $num_files }/{ $all_files } Elemente
save_results_to_file = Ergebnisse in Datei { $name } gespeichert
search_not_choosing_any_music = FEHLER: Sie müssen mindestens ein Kontrollkästchen mit Art der Musiksuche auswählen.
include_folders_dialog_title = Ordner zum Einbinden
exclude_folders_dialog_title = Ordner ausschließen
include_manually_directories_dialog_title = Verzeichnis manuell hinzufügen
cache_properly_cleared = Richtig geleerter Cache
cache_clear_duplicates_title = Leere Duplikaten-Cache
cache_clear_similar_images_title = Leere ähnlichen Bildercache
cache_clear_similar_videos_title = Lösche ähnlichen Videocache
cache_clear_message_label_1 = Wollen Sie den Cache von veralteten Einträgen löschen?
cache_clear_message_label_2 = Dieser Vorgang entfernt alle Cache-Einträge, die auf ungültige Dateien verweisen.
cache_clear_message_label_3 = Dies kann das Laden und Speichern im Cache beschleunigen.
cache_clear_message_label_4 = WARNUNG: Die Operation wird alle zwischengespeicherten Daten von externen Laufwerken entfernen, also muss der Hash erneut generiert werden.
# Show preview
preview_temporary_file = Fehler beim Öffnen der temporären Bilddatei { $name }, Grund { $reason }.
preview_0_size = Vorschau des Bildes { $name } mit 0 Breite oder Höhe kann nicht erstellt werden.
preview_temporary_image_save = Fehler beim Speichern der temporären Bilddatei in { $name }, Grund { $reason }.
preview_temporary_image_remove = Fehler beim Löschen der temporären Bilddatei { $name }, Grund { $reason }.
preview_failed_to_create_cache_dir = Fehler beim Erstellen des Verzeichnisses { $name } durch die Bildvorschau, Grund { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Gruppe { $current_group }/{ $all_groups } ({ $images_in_group } Bilder)
compare_move_left_button = L
compare_move_right_button = R
