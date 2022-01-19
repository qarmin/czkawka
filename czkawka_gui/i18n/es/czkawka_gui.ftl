# Window titles
window_settings_title = Opciones
window_main_title = Czkawka (Hipo)
window_progress_title = Escaneando
window_compare_images = Comparar imágenes
# General
general_ok_button = Ok
general_close_button = Cerrar
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_album_title_checkbox = Título del álbum
music_album_artist_checkbox = Artista del álbum
music_year_checkbox = Año
music_comparison_checkbox = Comparación aproximada
music_comparison_checkbox_tooltip =
    Busca archivos de música similares usando IA, que utiliza el aprendizaje automático para eliminar paréntesis de una frase, p. ej. con esta opción habilitada, los archivos en cuestión se considerarán duplicados:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Nombre
duplicate_mode_size_combo_box = Tamaño
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Kawka ofrece 3 tipos de hashes, que pueden ser usados:
    
    Función de hash Blake3 criptográfica. Se utiliza como algoritmo hash por defecto porque es muy rápido.
    
    CRC32 - función hash simple. Debe ser más rápido que Blake3, pero probablemente tenga algunas colisiones muy raras.
    
    XXH3 - muy similar en caso de rendimiento y calidad de hash a Blake3, por lo que tales modos pueden ser fácilmente usados.
duplicate_check_method_tooltip =
    Por ahora, el kawka ofrece tres tipos de métodos para encontrar duplicados por:
    
    Nombre - Encuentra archivos con el mismo nombre.
    
    Tamaño - Encuentra archivos con el mismo tamaño.
    
    Hash - Encuentra archivos con el mismo contenido. Este modo apaga el archivo y luego compara este hash para encontrar duplicados. Este modo es la manera más segura de encontrar duplicados. La herramienta utiliza la caché en gran medida, por lo que segundo y posterior análisis de los mismos datos debe ser mucho más rápido que el primero.
image_hash_size_tooltip =
    Kawka ofrece un cambio de tamaño del hash generado para cada imagen. La causa hash de Bigger permite encontrar imágenes con menor cantidad de diferencias entre imágenes, pero también es un poco más lento de usar.
    
    El valor predeterminado para el hash es de 8 bytes, lo que permite encontrar imágenes muy similares y diferentes. 16 y 32 hashes deben utilizarse sólo para imágenes casi idénticas. El hash de 64 bytes no debería ser usado, excepto en una situación en la que se necesitan diferencias realmente pequeñas para encontrar
image_resize_filter_tooltip = Para calcular el hash de la imagen, la librería primero debe redimensionarla. Depende del algoritmo elegido, la imagen resultante se verá poco diferente. El algoritmo más rápido de usar, pero también uno que da los peores resultados es el más cercano.
image_hash_alg_tooltip = Los usuarios pueden elegir uno de muchos algoritmos de cálculo de hash. Cada uno tiene puntos fuertes y débiles y dará a veces mejores y a veces peores resultados para diferentes imágenes, para elegir el mejor, se requieren pruebas manuales.
main_notebook_image_fast_compare = Comparación rápida
main_notebook_image_fast_compare_tooltip =
    Acelera la búsqueda y la comparación de hashes.
    
    En frente del modo normal donde cada hash es comparado entre sí x veces, donde x es la similitud que el usuario elija, en este modo siempre se utiliza sólo una comparación.
    
    Esta opción se recomienda cuando se compara >10000 imágenes con una similitud que no es 0(Muy Alta).
main_notebook_duplicates = Duplicar archivos
main_notebook_empty_directories = Directorios vacíos
main_notebook_big_files = Archivos grandes
main_notebook_empty_files = Archivos vacíos
main_notebook_temporary = Archivos temporales
main_notebook_similar_images = Imágenes similares
main_notebook_similar_videos = Videos similares
main_notebook_same_music = Duplicados de música
main_notebook_symlinks = Enlaces simbólicos inválidos
main_notebook_broken_files = Archivos rotos
main_tree_view_column_file_name = Nombre del archivo
main_tree_view_column_folder_name = Nombre de carpeta
main_tree_view_column_path = Ruta
main_tree_view_column_modification = Fecha de modificación
main_tree_view_column_size = Tamaño
main_tree_view_column_similarity = Similaridad
main_tree_view_column_dimensions = Dimensiones
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = Año
main_tree_view_column_album_title = Título del álbum
main_tree_view_column_album_artist = Artista del álbum
main_tree_view_column_symlink_file_name = Nombre del archivo Symlink
main_tree_view_column_symlink_folder = Carpeta de Symlnik
main_tree_view_column_destination_path = Ruta de destino
main_tree_view_column_type_of_error = Tipo de error
main_label_check_method = Comprobar método
main_label_hash_type = Tipo de Hash
main_label_hash_size = Tamaño hash
main_label_size_bytes = Tamaño(bytes)
main_label_min_size = Mínimo
main_label_max_size = Máx
main_label_shown_files = Número de archivos mostrados
main_label_resize_algorithm = Redimensionar algoritmo
main_label_similarity = Similarity{ " " }
check_button_general_same_size = Ignorar el mismo tamaño
check_button_general_same_size_tooltip = Ignorar de los resultados, archivos que tienen el mismo tamaño - generalmente son 1:1 duplicados
main_label_size_bytes_tooltip = Tamaño de los archivos que se utilizarán en el escaneo
# Upper window
upper_tree_view_included_folder_column_title = Carpetas a buscar
upper_tree_view_included_reference_column_title = Carpetas de referencia
upper_recursive_button = Recursivo
upper_recursive_button_tooltip = Si se selecciona, busque también archivos que no se coloquen directamente bajo las carpetas seleccionadas.
upper_manual_add_included_button = Añadir manual
upper_add_included_button = Añadir
upper_remove_included_button = Eliminar
upper_manual_add_excluded_button = Añadir manual
upper_add_excluded_button = Añadir
upper_remove_excluded_button = Eliminar
upper_manual_add_included_button_tooltip = Permite añadir el nombre del directorio para buscar a mano.
upper_add_included_button_tooltip = Añadir nuevo directorio para buscar.
upper_remove_included_button_tooltip = Eliminar directorio de la búsqueda.
upper_manual_add_excluded_button_tooltip = Permite añadir el nombre del directorio excluido a mano.
upper_add_excluded_button_tooltip = Añadir directorio para ser excluido en la búsqueda.
upper_remove_excluded_button_tooltip = Eliminar directorio de excluidos.
upper_notebook_items_configuration = Configuración de artículos
upper_notebook_excluded_directories = Directorios excluidos
upper_notebook_included_directories = Directorios incluidos
upper_allowed_extensions_tooltip =
    Las extensiones permitidas deben estar separadas por comas (por defecto todas están disponibles).
    
    Macros IMAGE, VIDEO, MUSIC, TEXT que añade múltiples extensiones a la vez también están disponibles.
    
    Ejemplo de uso ".exe, IMAGE, VIDEO, .rar, 7z" - esto significa que image(e. . jpg, png), video(p. ej., avi, mp4), archivos exe, rar y 7z serán escaneados.
upper_excluded_items_tooltip =
    Los artículos excluidos deben contener * comodín y deben estar separados por comas.
    Esto es más lento que los Directorios excluidos, así que úsala con cuidado.
upper_excluded_items = Elementos excluidos:
upper_allowed_extensions = Extensiones permitidas:
# Popovers
popover_select_all = Seleccionar todo
popover_unselect_all = Deseleccionar todo
popover_reverse = Invertir selección
popover_select_all_except_oldest = Seleccionar todo excepto más antiguo
popover_select_all_except_newest = Seleccionar todo excepto el más reciente
popover_select_one_oldest = Seleccione uno más antiguo
popover_select_one_newest = Seleccione un nuevo
popover_select_custom = Seleccionar personalizado
popover_unselect_custom = Deseleccionar personalizado
popover_select_all_images_except_biggest = Seleccionar todo excepto mayor
popover_select_all_images_except_smallest = Seleccionar todo excepto menor
popover_custom_path_check_button_entry_tooltip =
    Permite seleccionar registros por su ruta.
    
    Ejemplo de uso:
    /home/pmañk/rzecz.txt puede encontrarse con /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Permite seleccionar registros por nombres de archivo.
    
    Ejemplo:
    /usr/ping/pong.txt puede encontrarse con *a lo largo*
popover_custom_regex_check_button_entry_tooltip =
    Permite seleccionar registros por Regex.
    
    En este modo, el texto buscado es Ruta con Nombre.
    
    Ejemplo:
    /usr/bin/ziemniak. xt se puede encontrar con /ziem[a-z]+
    
    Esto utiliza la implementación predeterminada de expresiones regulares de Rust, así que puedes leer más al respecto en https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Evita seleccionar todos los registros en el grupo.
    
    Esto está activado por defecto, porque en la mayoría de las situaciones el usuario no quiere eliminar archivos originales y duplicados. pero quiere dejar al menos un archivo.
    
    Advertencia: Esta configuración no funciona si el usuario ya ha seleccionado todos los resultados en el grupo manualmente.
popover_custom_regex_path_label = Ruta
popover_custom_regex_name_label = Nombre
popover_custom_regex_regex_label = Ruta de Regex + Nombre
popover_custom_all_in_group_label = No seleccionar todos los registros en el grupo
popover_custom_mode_unselect = Deseleccionar Personalizado
popover_custom_mode_select = Seleccionar Personalizado
popover_invalid_regex = Regex no es válido
popover_valid_regex = Regex es válido
# Bottom buttons
bottom_search_button = Buscar
bottom_select_button = Seleccionar
bottom_delete_button = Eliminar
bottom_save_button = Guardar
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Mover
bottom_search_button_tooltip = Empezar a buscar archivos/carpetas.
bottom_select_button_tooltip = Selecciona registros. Sólo los archivos/carpetas seleccionados pueden ser procesados más tarde.
bottom_delete_button_tooltip = Eliminar archivos/carpetas seleccionadas.
bottom_save_button_tooltip = Guardar datos sobre la búsqueda en el archivo
bottom_symlink_button_tooltip =
    Crea enlaces simbólicos.
    Solo funciona cuando al menos 2 resultados en grupo son seleccionados.
    El primero no ha cambiado y el segundo y más tarde están enlazados con el primero.
bottom_hardlink_button_tooltip =
    Crea enlaces hardlinks.
    Solo funciona cuando al menos 2 resultados en grupo son seleccionados.
    El primero no ha cambiado y el segundo y más tarde están estrechamente vinculados a la primera.
bottom_move_button_tooltip =
    Mueve los archivos a la carpeta elegida.
    Copia todos los archivos a la carpeta sin preservar el árbol de directorios.
    Al intentar mover 2 archivos con el mismo nombre a la carpeta, el segundo fallará y mostrará el error.
bottom_show_errors_tooltip = Mostrar / Ocultar panel de error inferior.
bottom_show_upper_notebook_tooltip = Mostrar / Ocultar panel de cuaderno superior.
# Progress Window
progress_stop_button = Parar
# About Window
about_repository_button_tooltip = Enlace a la página del repositorio con código fuente.
about_donation_button_tooltip = Enlace a la página de donación.
about_instruction_button_tooltip = Enlace a la página de instrucciones.
about_translation_button_tooltip = Enlace a la página de Crowdin con traducciones de aplicaciones. Oficialmente se admiten polaco e inglés, pero se agradecerá cualquier ayuda con otro idioma.
about_repository_button = Repositorio
about_donation_button = Donación
about_instruction_button = Instrucción
about_translation_button = Traducción
# Header
header_setting_button_tooltip = Abre el diálogo de ajustes.
header_about_button_tooltip = Abre el diálogo con información sobre la aplicación.

# Settings


## General

settings_save_at_exit_button_tooltip = Guarda la configuración en el archivo al cerrar la aplicación.
settings_load_at_start_button_tooltip =
    Cargando la configuración de inicio desde el archivo.
    
    Si no selecciona esta opción cargará la configuración por defecto.
settings_confirm_deletion_button_tooltip = Muestra el diálogo de confirmación al hacer clic en el botón Eliminar.
settings_confirm_link_button_tooltip = Muestra el diálogo de confirmación al hacer clic en el botón duro/symlink.
settings_confirm_group_deletion_button_tooltip = Muestra el diálogo cuando intenta eliminar todos los registros del grupo.
settings_show_text_view_button_tooltip = Muestra el panel de error en la parte inferior.
settings_use_cache_button_tooltip = Opción que permite no utilizar la función de caché.
settings_save_also_as_json_button_tooltip = Guardar caché en formato JSON humano. Es posible modificar su contenido. La caché de este archivo será leída automáticamente por la aplicación si la caché del formato binario (con la extensión binaria) faltará.
settings_use_trash_button_tooltip = Cuando está habilitado mueve archivos a la papelera en su lugar eliminándolos permanentemente.
settings_language_label_tooltip = Permite elegir el idioma de la interfaz entre los disponibles.
settings_save_at_exit_button = Guardar configuración al salir
settings_load_at_start_button = Cargar configuración al iniciar
settings_confirm_deletion_button = Mostrar diálogo de confirmación al eliminar cualquier archivo
settings_confirm_link_button = Mostrar diálogo de confirmación cuando vincule archivos de forma dura o simbólica
settings_confirm_group_deletion_button = Mostrar diálogo de confirmación al eliminar todos los archivos del grupo
settings_show_text_view_button = Mostrar panel de texto inferior
settings_use_cache_button = Usar caché
settings_save_also_as_json_button = Guardar caché también en archivo JSON
settings_use_trash_button = Mover archivos borrados a la papelera
settings_language_label = Idioma
settings_multiple_delete_outdated_cache_checkbutton = Borrar automáticamente entradas de caché obsoletas
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Permite eliminar resultados de caché obsoletos que apuntan a archivos inexistentes.
    
    Cuando esté habilitada, asegúrese de que al cargar registros, todos los puntos a archivos válidos e ignore los rotos.
    
    Desactivar esta opción, ayudará a escanear archivos en unidades externas, por lo que las entradas de caché sobre ellas no serán purgadas en el próximo escaneo.
    
    En caso de tener cientos de miles de registros en caché, se sugiere habilitar esta opción, para acelerar la carga y guardado de caché al inicio y final de la búsqueda.
settings_notebook_general = General
settings_notebook_duplicates = Duplicados
settings_notebook_images = Imágenes similares
settings_notebook_videos = Vídeo similar

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Muestra la vista previa en el lado derecho, al seleccionar el archivo de imagen.
settings_multiple_image_preview_checkbutton = Mostrar vista previa de la imagen
settings_multiple_clear_cache_button_tooltip =
    Borrar manualmente la caché de entradas desactualizadas.
    Debe utilizarse sólo si se deshabilita la limpieza automática.
settings_multiple_clear_cache_button = Eliminar resultados desactualizados de la caché de imágenes

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Oculta todos los archivos excepto uno, si son puntos a los mismos datos(están en línea dura).
    
    Por ejemplo, en caso de que en el disco haya 7 archivos que están enlazados duramente a datos específicos y un archivo diferente con los mismos datos pero diferente inode, entonces en el buscador duplicado sólo será visible un archivo único y un archivo de los enlazados.
settings_duplicates_minimal_size_entry_tooltip =
    Permite establecer el tamaño mínimo del archivo, que será almacenado en caché.
    
    Eligiendo un valor menor, generará más registros que acelerarán la búsqueda, pero ralentizarán la carga/guardado de la caché.
settings_duplicates_prehash_checkbutton_tooltip =
    Activa el almacenamiento en caché de prehash(hash calculado desde una pequeña parte del archivo) que permite descartar resultados no duplicados anteriormente.
    
    Está deshabilitado por defecto porque puede causar en algunas situaciones ralentizaciones.
    
    Es muy recomendable usarlo para escanear cientos de miles o millones de archivos, ya que puede acelerar la búsqueda varias veces.
settings_duplicates_prehash_minimal_entry_tooltip = Tamaño mínimo de la entrada en caché.
settings_duplicates_hide_hard_link_button = Ocultar enlaces duros (sólo Linux y MacOS)
settings_duplicates_prehash_checkbutton = Usar caché prehash
settings_duplicates_minimal_size_cache_label = Tamaño mínimo de los archivos en bytes guardados en la caché
settings_duplicates_minimal_size_cache_prehash_label = Tamaño mínimo de archivos en bytes guardados en caché prehash

## Saving/Loading settings

settings_saving_button_tooltip = Guardar configuración de configuración actual al archivo.
settings_loading_button_tooltip = Carga los ajustes desde el archivo y reemplaza la configuración actual con ellos.
settings_reset_button_tooltip = Restablecer la configuración actual a la predeterminada.
settings_saving_button = Guardar configuración
settings_loading_button = Cargar configuración
settings_reset_button = Restablecer configuración

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Abre la carpeta donde se almacenan archivos txt con caché.
    
    Modificarlos puede causar que se muestren resultados inválidos pero también se modifique e., puede ahorrar tiempo al mover gran cantidad de archivos a un lugar diferente.
    
    Puede copiar estos archivos entre ordenadores para ahorrar tiempo al escanear de nuevo para archivos (por supuesto, si tienen una estructura de directorios similar).
    
    En caso de problemas con la caché, estos archivos pueden ser eliminados, por lo que la aplicación los regenerará automáticamente.
settings_folder_settings_open_tooltip =
    Abre la carpeta donde se almacenan las configuraciones del kawka.
    
    Modificándolas a mano, puede causar que se rompa el flujo de trabajo.
settings_folder_cache_open = Abrir carpeta de caché
settings_folder_settings_open = Abrir carpeta de ajustes
# Compute results
compute_stopped_by_user = El usuario ha detenido la búsqueda
compute_found_duplicates_hash_size = Se encontraron { $number_files } duplicados en { $number_groups } grupos que tomaron { $size }
compute_found_duplicates_name = Se encontraron { $number_files } duplicados en { $number_groups } grupos
compute_found_empty_folders = Se encontraron { $number_files } carpetas vacías
compute_found_empty_files = Se encontraron { $number_files } archivos vacíos
compute_found_big_files = { $number_files } archivos grandes encontrados
compute_found_temporary_files = Se encontraron { $number_files } archivos temporales
compute_found_images = Se encontraron { $number_files } imágenes similares en { $number_groups } grupos
compute_found_videos = Se encontraron { $number_files } vídeos similares en { $number_groups } grupos
compute_found_music = Se encontraron { $number_files } archivos de música similares en { $number_groups } grupos
compute_found_invalid_symlinks = Se encontraron { $number_files } enlaces simbólicos no válidos
compute_found_broken_files = Se encontraron { $number_files } archivos rotos
# Progress window
progress_scanning_general_file = Escaneando archivo { $file_number }
progress_scanning_broken_files = Comprobando { $file_checked }/{ $all_files } archivo
progress_scanning_video = Hash de { $file_checked }/{ $all_files } vídeo
progress_scanning_image = Hash de { $file_checked }/{ $all_files } imagen
progress_comparing_image_hashes = Comparando hash de imagen { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Comparando etiquetas de { $file_checked }/{ $all_files } archivo de música
progress_scanning_music_tags = Leyendo etiquetas del archivo de música { $file_checked }/{ $all_files }
progress_scanning_empty_folders = Escaneando carpeta { $folder_number }
progress_scanning_size = Escaneando tamaño del archivo { $file_number }
progress_scanning_name = Escaneando nombre del archivo { $file_number }
progress_analyzed_partial_hash = Has analizado el hash parcial de { $file_checked }/{ $all_files } archivos
progress_analyzed_full_hash = Se ha analizado el hash completo de { $file_checked }/{ $all_files } archivos
progress_current_stage = Etapa actual:{ " " }
progress_all_stages = Todas las etapas:{ " " }
# Saving loading 
saving_loading_saving_success = Configuración guardada en el archivo { $name }.
saving_loading_saving_failure = Error al guardar los datos de configuración en el archivo { $name }.
saving_loading_reset_configuration = La configuración actual fue borrada.
saving_loading_loading_success = Configuración de la aplicación cargada correctamente.
saving_loading_invalid_string = Para la clave "{ $key }" se encontró un resultado inválido - "{ $result }" que no es una cadena.
saving_loading_invalid_int = Para la clave "{ $key }" se encontró un resultado inválido - "{ $result }" que no es un entero.
saving_loading_invalid_bool = Para la clave "{ $key }" se encontró un resultado inválido - "{ $result }" que no es un bool.
saving_loading_decode_problem_bool = Fallo al descifrar el bool de la clave "{ $key }" encontrado "{ $result }" pero los valores permitidos son 0, 1, verdadero o falso.
saving_loading_saving_same_keys = Intentando guardar la configuración con la clave duplicada "{ $key }".
saving_loading_failed_to_get_home_directory = No se pudo obtener el directorio de inicio para abrir/guardar el archivo de configuración.
saving_loading_folder_config_instead_file = No se puede crear o abrir el archivo de configuración de guardado en la ruta "{ $path }" porque ya hay una carpeta.
saving_loading_failed_to_create_configuration_folder = Error al crear la carpeta de configuración "{ $path }", razón "{ $reason }".
saving_loading_failed_to_create_config_file = Error al crear el archivo de configuración "{ $path }", razón "{ $reason }".
saving_loading_failed_to_read_config_file = No se puede cargar la configuración de "{ $path }" porque no existe o no es un archivo.
saving_loading_failed_to_read_data_from_file = No se pueden leer los datos del archivo "{ $path }", razón "{ $reason }".
saving_loading_orphan_data = Se encontraron datos huérfanos "{ $data }" en la línea "{ $line }".
saving_loading_not_valid = Ajustando "{ $data }" no existe en la versión actual de la aplicación.
# Invalid symlinks
invalid_symlink_infinite_recursion = Recursión infinita
invalid_symlink_non_existent_destination = Archivo de destino no existente
# Other
searching_for_data = Buscando datos, puede tardar un tiempo, por favor espere...
text_view_messages = MENSAJES
text_view_warnings = ADVERTENCIA
text_view_errors = ERRORES
about_window_motto = Este programa es gratuito y siempre lo será.
# Various dialog
dialogs_ask_next_time = Preguntar la próxima vez
delete_file_failed = Error al eliminar el archivo { $name }, razón { $reason }
delete_title_dialog = Confirmación de eliminación
delete_question_label = ¿Está seguro que desea eliminar los archivos?
delete_all_files_in_group_title = Confirmación de borrar todos los archivos del grupo
delete_all_files_in_group_label1 = En algunos grupos se seleccionan todos los registros.
delete_all_files_in_group_label2 = ¿Estás seguro de que quieres eliminarlos?
delete_folder_failed = Error al eliminar la carpeta { $dir } porque la carpeta no existe, no tiene permisos o no está vacía.
delete_items_label = { $items } archivos serán eliminados.
delete_items_groups_label = { $items } archivos de { $groups } grupos serán eliminados.
hardlink_failed = Error al vincular
hard_sym_invalid_selection_title_dialog = Selección no válida con algunos grupos
hard_sym_invalid_selection_label_1 = En algunos grupos sólo hay 1 registro seleccionado y será ignorado.
hard_sym_invalid_selection_label_2 = Para poder enlazar estos archivos, al menos 2 resultados en grupo deben ser seleccionados.
hard_sym_invalid_selection_label_3 = El primero en el grupo es reconocido como original y no se cambia, pero el segundo y posterior son modificados.
hard_sym_link_title_dialog = Confirmación de enlace
hard_sym_link_label = ¿Está seguro que desea enlazar estos archivos?
move_folder_failed = Error al mover la carpeta { $name }, razón { $reason }
move_file_failed = Error al mover el archivo { $name }, razón { $reason }
move_files_title_dialog = Elija la carpeta a la que desea mover los archivos duplicados
move_files_choose_more_than_1_path = Solo debe seleccionarse una ruta para poder copiar los archivos duplicados, seleccionados { $path_number }.
move_stats = Mudado correctamente { $num_files }/{ $all_files } elementos
save_results_to_file = Resultados guardados en el archivo { $name }
search_not_choosing_any_music = ERROR: Debe seleccionar al menos una casilla de verificación con tipos de búsqueda de música.
include_folders_dialog_title = Carpetas a incluir
exclude_folders_dialog_title = Carpetas a excluir
include_manually_directories_dialog_title = Añadir directorio manualmente
cache_properly_cleared = Caché correctamente borrada
cache_clear_duplicates_title = Limpiando caché duplicada
cache_clear_similar_images_title = Limpiando caché de imágenes similares
cache_clear_similar_videos_title = Limpiando caché de vídeos similares
cache_clear_message_label_1 = ¿Quieres borrar la caché de entradas desactualizadas?
cache_clear_message_label_2 = Esta operación eliminará todas las entradas de caché que apunten a archivos no válidos.
cache_clear_message_label_3 = Esto puede acelerar un poco la carga/guardado en la caché.
cache_clear_message_label_4 = ATENCIÓN: La operación eliminará todos los datos almacenados en caché de unidades externas desconectadas, así que el hash tendrá que ser generado de nuevo.
# Show preview
preview_temporary_file = Error al abrir el archivo de imagen temporal { $name }, razón { $reason }.
preview_0_size = No se puede crear la vista previa de la imagen { $name }, con 0 de ancho o alto.
preview_temporary_image_save = Error al guardar el archivo de imagen temporal en { $name }, razón { $reason }.
preview_temporary_image_remove = Error al eliminar el archivo de imagen temporal { $name }, razón { $reason }.
preview_failed_to_create_cache_dir = Error al crear el directorio { $name } necesario para la vista previa de la imagen, razón { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupo { $current_group }/{ $all_groups } ({ $images_in_group } imágenes)
compare_move_left_button = L
compare_move_right_button = R
