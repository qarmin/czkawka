# Core
core_similarity_original = Origineel
core_similarity_very_high = Zeer hoog
core_similarity_high = Hoog
core_similarity_medium = Gemiddeld
core_similarity_small = Klein
core_similarity_very_small = Zeer klein
core_similarity_minimal = Minimaal

core_cannot_open_dir = Kan map {$dir} niet openen, reden: {$reason}
core_cannot_read_entry_dir = Kan item in map {$dir} niet lezen, reden: {$reason}
core_cannot_read_metadata_dir = Kan metadata in map {$dir} niet lezen, reden: {$reason}
core_cannot_read_metadata_file = Kan metadata van bestand {$file} niet lezen, reden: {$reason}
core_file_modified_before_epoch = Bestand {$name} lijkt te zijn gewijzigd vóór de Unix-epoch
core_folder_modified_before_epoch = Map {$name} lijkt te zijn gewijzigd vóór de Unix-epoch
core_file_no_modification_date = Kan wijzigingsdatum niet ophalen van bestand {$name}, reden: {$reason}
core_folder_no_modification_date = Kan wijzigingsdatum niet ophalen van map {$name}, reden: {$reason}

core_missing_no_chosen_included_path = Geen geldig inclusief pad gekozen (uitgesloten paden kunnen alle inclusieve paden hebben uitgesloten)
core_reference_included_paths_same = Kan scan niet starten waarbij alle geldige inclusieve paden ook referentiepaden zijn; probeer referentiepaden te valideren of uit te schakelen
core_path_must_exists = Opgegeven pad moet bestaan, { $path } wordt genegeerd
core_must_be_directory_or_file = Opgegeven pad moet naar een geldige map of bestand verwijzen, { $path } wordt genegeerd
core_excluded_paths_pointless_slash = Het uitsluiten van / is zinloos, omdat er dan geen enkel bestand wordt gescand
core_paths_unable_to_get_device_id = Kan apparaat-ID niet ophalen van map { $path }

core_needs_allowed_extensions_limited_by_tool = Kan scan niet starten omdat alle extensies die beschikbaar zijn in deze tool ({ $extensions }) zijn uitgesloten
core_needs_allowed_extensions = Kan scan niet starten omdat alle extensies zijn uitgesloten
core_needs_to_set_at_least_one_broken_option = Kan scan niet starten omdat er geen optie voor beschadigde bestanden is geselecteerd
core_needs_to_set_at_least_one_bad_name_option = Kan scan niet starten omdat er geen optie voor ongeldige namen is geselecteerd

core_ffmpeg_not_found = Kan geen correcte installatie van FFmpeg of FFprobe vinden. Dit zijn externe programma's die handmatig moeten worden geïnstalleerd.
core_ffmpeg_not_found_windows = Zorg ervoor dat ffmpeg.exe en ffprobe.exe beschikbaar zijn in het PATH of direct in dezelfde map als het uitvoerbare bestand van de app staan

core_invalid_symlink_infinite_recursion = Oneindige recursie
core_invalid_symlink_non_existent_destination = Niet-bestaande doelbestemming

core_messages_limit_reached_characters = Het aantal berichten heeft de ingestelde limiet overschreden ({$current}/{$limit} tekens), de uitvoer is daarom ingekort. Schakel de limietoptie uit in de instellingen om de volledige uitvoer te lezen.
core_messages_limit_reached_lines = Het aantal berichten heeft de ingestelde limiet overschreden ({$current}/{$limit} regels), de uitvoer is daarom ingekort. Schakel de limietoptie uit in de instellingen om de volledige uitvoer te lezen.

core_error_moving_to_trash = Fout bij verplaatsen van "{ $file }" naar de prullenbak: { $error }
core_error_removing = Fout bij verwijderen van "{ $file }": { $error }

core_no_similarity_method_selected = Kan geen vergelijkbare muziekbestanden vinden zonder geselecteerde vergelijkingsmethode

core_failed_to_spawn_command = Starten van commando mislukt: { $reason }
core_failed_to_check_process_status = Controleren van processtatus mislukt: { $reason }
core_failed_to_wait_for_process = Wachten op proces mislukt: { $reason }
core_failed_to_read_video_properties = Lezen van video-eigenschappen mislukt: { $reason }
core_failed_to_execute_ffmpeg = Uitvoeren van ffmpeg mislukt: { $reason }
core_ffmpeg_failed_with_status = ffmpeg mislukt met status { $status }: { $stderr } (commando: { $command })
core_failed_to_load_image_frame = Laden van afbeeldingsframe mislukt: { $reason }
core_failed_to_extract_frame = Extractie van frame op { $time } seconden uit "{ $file }" mislukt: { $reason }
core_failed_to_save_thumbnail = Opslaan van miniatuur voor "{ $file }" mislukt: { $reason }
core_failed_get_frame_at_timestamp = Ophalen van frame op tijdstip { $timestamp } uit "{ $file }" mislukt: { $reason }
core_failed_get_frame_from_file = Ophalen van frame uit "{ $file }" op tijdstip { $timestamp } mislukt: { $reason }
core_invalid_crop_rectangle = Ongeldige uitsnede-rechthoek: links={ $left }, boven={ $top }, rechts={ $right }, onder={ $bottom }
core_failed_to_crop_video_file = Bijsnijden van videobestand "{ $file }" mislukt: { $reason }
core_cropped_video_not_created = Bijgesneden videobestand is niet aangemaakt: { $temp }
core_unable_check_hash_of_file = Kan hash van bestand "{ $file }" niet controleren, reden: { $reason }
core_error_checking_hash_of_file = Fout bij controleren van hash van bestand "{ $file }", reden: { $reason }
core_image_zero_dimensions = Afbeelding heeft nulbreedte of -hoogte: "{ $path }"
core_image_open_failed = Kan afbeeldingsbestand "{ $path }" niet openen: { $reason }
core_not_directory_remove = Poging tot verwijderen van "{ $path }", wat geen map is
core_cannot_read_directory = Kan map "{ $path }" niet lezen
core_cannot_read_entry_from_directory = Kan item uit map "{ $path }" niet lezen
core_folder_contains_file_inside = Map bevat bestand "{ $entry }" binnen "{ $folder }"
core_unknown_directory_entry = Kan bestandstype van item "{ $entry }" in "{ $path }" niet bepalen
core_video_width_exceeds_limit = Videobreedte { $width } overschrijdt de limiet van { $limit }
core_video_height_exceeds_limit = Videohoogte { $height } overschrijdt de limiet van { $limit }
core_failed_to_process_video = Verwerken van videobestand { $file } mislukt: { $reason }
core_optimized_file_larger = Geoptimaliseerd bestand { $optimized } (grootte: { $new_size }) is niet kleiner dan het origineel { $original } (grootte: { $original_size })
core_unknown_codec = Onbekende codec: { $codec }
core_invalid_video_optimizer_mode = Ongeldige video-optimalisatiemodus: '{ $mode }'. Toegestane waarden: transcode, crop
core_folder_does_not_exist = Map bestaat niet: { $folder }
core_path_not_directory = Pad is geen map: { $folder }
core_test_error_for_folder = Testfout voor map: { $folder }
core_unknown_exif_tag_group = Onbekende EXIF-taggroep: { $tag }
core_error_comparing_fingerprints = Fout bij vergelijken van vingerafdrukken: { $reason }
core_failed_to_generate_thumbnail_frames_different_dimensions = Miniatuur genereren voor "{ $file }" mislukt: geëxtraheerde frames hebben verschillende afmetingen
core_failed_to_generate_thumbnail = Miniatuur genereren voor "{ $file }" mislukt: { $reason }
core_failed_to_extract_frame_at_seek_time = Extractie van frame op { $time } seconden uit "{ $file }" mislukt: { $reason }
core_video_file_does_not_exist = Videobestand bestaat niet (mogelijk verwijderd tussen scan en latere stappen): "{ $path }"
core_image_too_large = Afbeelding is te groot ({ $width }x{ $height }) - meer dan het ondersteunde aantal van { $max } pixels
core_failed_to_get_video_metadata = Ophalen van video-metadata voor bestand "{ $file }" mislukt: { $reason }
core_failed_to_get_video_codec = Ophalen van video-codec voor bestand "{ $file }" mislukt
core_failed_to_get_video_duration = Ophalen van videoduur voor bestand "{ $file }" mislukt
core_failed_to_get_video_dimensions = Ophalen van video-afmetingen voor bestand "{ $file }" mislukt
core_frame_dimensions_mismatch = Frame-afmetingen voor tijdstip { $timestamp } komen niet overeen met de afmetingen van het eerste frame ({ $first_w }x{ $first_h })
core_failed_to_load_data_from_cache = Laden van gegevens uit cachebestand { $file } mislukt, reden: { $reason }
core_failed_to_load_data_from_json_cache = Laden van gegevens uit JSON-cachebestand { $file } mislukt, reden: { $reason }
core_failed_to_replace_with_optimized = Vervangen van bestand "{ $file }" door geoptimaliseerde versie mislukt: { $reason }
core_failed_to_write_data_to_cache = Kan gegevens niet naar cachebestand "{ $file }" schrijven, reden: { $reason }
core_properly_saved_cache_entries = Er zijn { $count } cache-items correct opgeslagen in het bestand.
core_video_processing_stopped_by_user = Videoverwerking gestopt door gebruiker
core_thumbnail_generation_stopped_by_user = Miniatuurgeneratie gestopt door gebruiker
core_failed_to_optimize_video = Optimaliseren van video "{ $file }" mislukt: { $reason }
core_failed_to_crop_video = Bijsnijden van video "{ $file }" mislukt: { $reason }
core_failed_to_get_metadata_of_optimized_file = Ophalen van metadata van geoptimaliseerd bestand "{ $file }" mislukt: { $reason }
core_cannot_create_config_folder = Kan configuratiemap "{ $folder }" niet aanmaken, reden: { $reason }
core_cannot_create_cache_folder = Kan cachemap "{ $folder }" niet aanmaken, reden: { $reason }
core_cannot_create_or_open_cache_file = Kan cachebestand "{ $file }" niet aanmaken of openen, reden: { $reason }
core_cannot_set_config_cache_path = Kan configuratie-/cachepad niet instellen - configuratie en cache worden niet gebruikt.
core_invalid_extension_contains_space = { $extension } is geen geldige extensie omdat deze een spatie bevat
core_invalid_extension_contains_dot = { $extension } is geen geldige extensie omdat deze een punt bevat
