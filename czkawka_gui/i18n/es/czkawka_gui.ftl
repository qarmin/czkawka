# Window titles
window_settings_title = Configuración
window_main_title = Czkawka (Hipo)
window_progress_title = Escaneando
window_compare_images = Comparar imágenes
# General
general_ok_button = Aceptar
general_close_button = Cerrar
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_year_checkbox = Año
music_bitrate_checkbox = Tasa de bits
music_genre_checkbox = Género
music_length_checkbox = Duración
music_comparison_checkbox = Comparación aproximada
music_checking_by_tags = Etiquetas
music_checking_by_content = Contenido
same_music_seconds_label = Duración mínima del segundo fragmento
same_music_similarity_label = Diferencia máxima
music_compare_only_in_title_group = Comparar sólo en el título
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    Al buscar archivos de música, por su contenido, podemos usar las siguientes opciones:
    
    - El tiempo mínimo de fragmento después del cual los archivos de música pueden ser identificados como similares
    - La diferencia máxima entre dos fragmentos probados
    
    La clave, para lograr los mejores resultados al buscar. Es proporcionando las mejores combinaciones de estos parámetros:
    
    - Establecer el tiempo mínimo a 5 s y la diferencia máxima a 1.0, buscará fragmentos casi idénticos en los archivos.
    - Un tiempo de 20 años y una diferencia máxima de 6.0, por otro lado, funciona bien para encontrar remixes/versiones en vivo, etc.
    
    Por defecto, cada archivo de música se compara entre sí y esto puede llevar mucho tiempo al probar muchos archivos, por lo que normalmente es mejor usar carpetas de referencia y especificar qué archivos deben compararse entre sí (con la misma cantidad de archivos, comparar las huellas dactilares será más rápido al menos 4x que sin carpetas de referencia).
music_comparison_checkbox_tooltip =
    Busca archivos de música similares usando IA, que usa el aprendizaje automático para eliminar paréntesis de una frase. Por ejemplo, con esta opción activada, los archivos en cuestión se considerarán duplicados:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Sensible a mayúsculas
duplicate_case_sensitive_name_tooltip =
    Cuando está habilitado, agrupa registros solo cuando tienen exactamente el mismo nombre. P. ej.
    
     Żołd ↔ Żołd
    
    En cambio, si deshabilitamos dicha opción, agrupará nombres sin comprobar si cada letra tiene el mismo tamaño. P. ej. 
    
    żoŁD ↔ Żołd
duplicate_mode_size_name_combo_box = Tamaño y nombre
duplicate_mode_name_combo_box = Nombre
duplicate_mode_size_combo_box = Tamaño
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka ofrece 3 tipos de hashes, que pueden ser usados:
    
    Blake3 - función de hash criptográfica. Se usa como algoritmo hash predeterminado porque es muy rápido.
    
    CRC32 - función hash simple. Debe ser más rápido que Blake3, pero probablemente tenga algunas colisiones muy raras.
    
    XXH3 - muy similar en caso de rendimiento y calidad de hash a Blake3 (pero no criptográfico). Por lo que tales modos pueden ser fácilmente usados.
duplicate_check_method_tooltip =
    Por ahora, el kawka ofrece tres tipos de métodos para encontrar duplicados por:
    
    Nombre - Encuentra archivos con el mismo nombre.
    
    Tamaño - Encuentra archivos con el mismo tamaño.
    
    Hash - Encuentra archivos con el mismo contenido. Este modo molesta el archivo y luego compara este hash para encontrar duplicados. Este modo es la forma más segura de encontrar duplicados. La aplicación utiliza mucho caché, por lo que segundo y más análisis de los mismos datos debe ser mucho más rápido que el primero.
image_hash_size_tooltip =
    Cada imagen seleccionada produce un hash especial que se puede comparar entre sí y una pequeña diferencia entre ellas significa que estas imágenes son similares.
    
    tamaño de 8 hash es bastante bueno para encontrar imágenes que son un poco similares a las originales. Con un conjunto más grande de imágenes (>1000), esto producirá una gran cantidad de falsos positivos, así que recomiendo usar un mayor tamaño de hash en este caso.
    
    16 es el tamaño de hash predeterminado, lo cual es un buen compromiso entre encontrar incluso un poco de imágenes similares y tener sólo una pequeña cantidad de colisiones hash.
    
    32 y 64 hashes sólo encuentran imágenes muy similares, pero no deberían tener casi falsos positivos (tal vez excepto algunas imágenes con canal alfa).
image_resize_filter_tooltip =
    Al calcular el hash de una imagen, lo primero que hace la librería es redimensionar esta.
    
    Dependiendo del algoritmo que elijamos, la imagen resultante usada para calcular el hash puede ser demasiado parecida.
    
    El algoritmo más rápido, es también uno de los que da los peores es el de aproximación cercana o Nearest. Ya que de forma predeterminada, usa un tamaño de hash de 16x16, insuficiente para poder verse.
    
    Con el tamaño hash de 8x8 se recomienda usar un algoritmo diferente al tipo Nearest, para tener mejores grupos de imágenes.
image_hash_alg_tooltip =
    Los usuarios pueden elegir uno de los muchos algoritmos de cálculo del hash.
    
    Cada uno tiene puntos fuertes y débiles y a veces dará mejores y a veces peores resultados para diferentes imágenes.
    
    Así que, para determinar la mejor para ti, se requiere una prueba manual.
big_files_mode_combobox_tooltip = Permite buscar archivos de un menor/mayor tamaño
big_files_mode_label = Archivos marcados
big_files_mode_smallest_combo_box = El más pequeño
big_files_mode_biggest_combo_box = El más grande
main_notebook_duplicates = Archivos Duplicados
main_notebook_empty_directories = Directorios vacíos
main_notebook_big_files = Archivos grandes
main_notebook_empty_files = Archivos vacíos
main_notebook_temporary = Archivos temporales
main_notebook_similar_images = Imágenes similares
main_notebook_similar_videos = Videos similares
main_notebook_same_music = Canciones duplicadas
main_notebook_symlinks = Enlaces simbólicos rotos
main_notebook_broken_files = Archivos rotos
main_notebook_bad_extensions = Extensiones incorrectas
main_tree_view_column_file_name = Nombre del archivo
main_tree_view_column_folder_name = Nombre de carpeta
main_tree_view_column_path = Ruta
main_tree_view_column_modification = Fecha de modificación
main_tree_view_column_size = Tamaño
main_tree_view_column_similarity = Similitud
main_tree_view_column_dimensions = Dimensiones
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = Año
main_tree_view_column_bitrate = Tasa de bits
main_tree_view_column_length = Duración
main_tree_view_column_genre = Género
main_tree_view_column_symlink_file_name = Nombre del archivo Symlink
main_tree_view_column_symlink_folder = Carpeta Symlink
main_tree_view_column_destination_path = Ruta de destino
main_tree_view_column_type_of_error = Tipo de error
main_tree_view_column_current_extension = Extensión actual
main_tree_view_column_proper_extensions = Extensión adecuada
main_label_check_method = Método de comprobación
main_label_hash_type = Tipo de Hash
main_label_hash_size = Tamaño hash
main_label_size_bytes = Tamaño (bytes)
main_label_min_size = Mínimo
main_label_max_size = Máximo
main_label_shown_files = Número de archivos mostrados
main_label_resize_algorithm = Algoritmo de Redimensionado
main_label_similarity = Similitud{ "   " }
main_check_box_broken_files_audio = Sonido
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Guardar
main_check_box_broken_files_image = Imagen
check_button_general_same_size = Ignorar el mismo tamaño
check_button_general_same_size_tooltip = Ignorar archivos con idéntico tamaño en resultados - usualmente son 1:1 duplicados
main_label_size_bytes_tooltip = Tamaño de los archivos que se utilizarán en el escaneo
# Upper window
upper_tree_view_included_folder_column_title = Carpetas a buscar
upper_tree_view_included_reference_column_title = Carpetas de referencia
upper_recursive_button = Recursivo
upper_recursive_button_tooltip = Si se selecciona, busca cualquier archivo, sin importar si está o no en una sub-carpeta.
upper_manual_add_included_button = Añadir manualmente
upper_add_included_button = Añadir
upper_remove_included_button = Eliminar
upper_manual_add_excluded_button = Añadir manual
upper_add_excluded_button = Añadir
upper_remove_excluded_button = Eliminar
upper_manual_add_included_button_tooltip =
    Añade el nombre del directorio para buscar a mano.
    
    Para agregar múltiples rutas a la vez, sepáralas con ;
    
    /home/roman;/home/rozkaz añadirá dos directorios /home/roman y /home/rozkaz
upper_add_included_button_tooltip = Añadir nuevo directorio para buscar.
upper_remove_included_button_tooltip = Eliminar directorio de la búsqueda.
upper_manual_add_excluded_button_tooltip =
    Añadir el nombre del directorio excluido a mano.
    
    Para agregar múltiples rutas a la vez, separalas por ;
    
    /home/roman;/home/krokiet añadirá dos directorios /home/roman y /home/keokiet
upper_add_excluded_button_tooltip = Añadir directorio para ser excluido en la búsqueda.
upper_remove_excluded_button_tooltip = Eliminar directorio de excluidos.
upper_notebook_items_configuration = Configuración de artículos
upper_notebook_excluded_directories = Directorios excluidos
upper_notebook_included_directories = Directorios incluidos
upper_allowed_extensions_tooltip =
    Las extensiones permitidas deben estar separadas por comas (por defecto todas están disponibles).
    
    Las siguientes Macros, que añaden múltiples extensiones a la vez, también están disponibles: IMAGE, VIDEO, MUSIC, TEXT.
    
    Ejemplo de uso ".exe, IMAGE, VIDEO, .rar, 7z" - esto significa que imágenes (e. . jpg, png), videos (ej: avi, mp4), archivos exe, rar, y 7z serán escaneados.
upper_excluded_extensions_tooltip =
    Lista de archivos ignorados, durante el escaneo.
    
    Cuando desactivamos las extensiones permitidas, estas tienen mayor prioridad, haciendo que los archivos no sean comprobados.
upper_excluded_items_tooltip =
    Los artículos excluidos deben contener * comodín y deben estar separados por comas.
    Esto es más lento que los Directorios Excluidos, así que úselo con cuidado.
upper_excluded_items = Elementos excluidos:
upper_allowed_extensions = Extensiones permitidas:
upper_excluded_extensions = Extensiones desactivadas:
# Popovers
popover_select_all = Seleccionar todo
popover_unselect_all = Deseleccionar todo
popover_reverse = Invertir selección
popover_select_all_except_oldest = Seleccionar todo excepto más antiguo
popover_select_all_except_newest = Seleccionar todo excepto el más reciente
popover_select_one_oldest = Seleccione uno más antiguo
popover_select_one_newest = Seleccione uno más nuevo
popover_select_custom = Seleccionar personalizado
popover_unselect_custom = Deseleccionar personalizado
popover_select_all_images_except_biggest = Seleccionar todo excepto mayor
popover_select_all_images_except_smallest = Seleccionar todo excepto menor
popover_custom_path_check_button_entry_tooltip =
    Seleccionar registros por ruta.
    
    Ejemplo:
    /home/pmañk/rzecz.txt se puede encontrar con /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Seleccionar registros por nombres de archivos.
    
    Ejemplo:
    /usr/ping/pong.txt puede encontrarse con *a lo largo*
popover_custom_regex_check_button_entry_tooltip =
    Seleccione registros por Regex.
    
    En este modo, el texto buscado es Ruta con Nombre.
    
    Ejemplo:
    /usr/bin/ziemniak. xt se puede encontrar con /ziem[a-z]+
    
    Esto utiliza la implementación predeterminada de expresiones regulares de Rust. Puedes leer más al respecto aquí: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Habilita la detección de mayúsculas y minúsculas.
    
    Cuando se desactiva /home/* encuentra /HoMe/roman y /home/roman.
popover_custom_not_all_check_button_tooltip =
    Previene la selección de todos los registros en grupo.
    
    Esto está activado por defecto, porque en la mayoría de las situaciones, no quiere eliminar tanto los archivos originales como los duplicados, pero quiere dejar al menos un archivo.
    
    ADVERTENCIA: Esta configuración no funciona si ya has seleccionado manualmente todos los resultados en un grupo.
popover_custom_regex_path_label = Ruta
popover_custom_regex_name_label = Nombre
popover_custom_regex_regex_label = Ruta de Regex + Nombre
popover_custom_case_sensitive_check_button = Distingue mayúsculas y minúsculas
popover_custom_all_in_group_label = No seleccionar todos los registros en el grupo
popover_custom_mode_unselect = Deseleccionar Personalizado
popover_custom_mode_select = Seleccionar Personalizado
popover_sort_file_name = Nombre de archivo
popover_sort_folder_name = Nombre de la carpeta
popover_sort_full_name = Nombre completo
popover_sort_size = Tamaño
popover_sort_selection = Selección
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
bottom_sort_button = Ordenar
bottom_search_button_tooltip = Iniciar búsqueda
bottom_select_button_tooltip = Seleccionar registros. Sólo los archivos/carpetas seleccionados pueden ser procesados más tarde.
bottom_delete_button_tooltip = Eliminar archivos/carpetas seleccionadas.
bottom_save_button_tooltip = Guardar datos sobre la búsqueda en el archivo
bottom_symlink_button_tooltip =
    Crear enlaces simbólicos.
    Sólo funciona cuando al menos dos resultados en grupo son seleccionados.
    El primero no ha cambiado y el segundo y más tarde están enlazados con el primero.
bottom_hardlink_button_tooltip =
    Crear enlaces hardlinks.
    Solo funciona cuando al menos dos resultados en grupo son seleccionados.
    El primero no ha cambiado y el segundo y más tarde están enlazados por hardlinks a la primera.
bottom_hardlink_button_not_available_tooltip =
    Crear enlaces duros.
    Botón deshabilitado, porque no se pueden crear enlaces duros.
    Hardlinks sólo funciona con privilegios de administrador en Windows, así que asegúrese de ejecutar la aplicación como administrador.
    Si la aplicación ya funciona con dichos privilegios, compruebe si hay problemas similares en Github.
bottom_move_button_tooltip =
    Mover los archivos a la carpeta elegida.
    Copia todos los archivos a la carpeta sin preservar el árbol de directorios.
    Al intentar mover dos archivos con el mismo nombre a la carpeta, el segundo fallará y mostrará el error.
bottom_sort_button_tooltip = Ordenar archivos/carpetas de acuerdo al método seleccionado.
bottom_show_errors_tooltip = Mostrar/Ocultar panel de texto inferior.
bottom_show_upper_notebook_tooltip = Mostrar / Ocultar panel de cuaderno superior.
# Progress Window
progress_stop_button = Parar
progress_stop_additional_message = Parar solicitado
# About Window
about_repository_button_tooltip = Enlace a la página del repositorio con código fuente.
about_donation_button_tooltip = Enlace a la página de donación.
about_instruction_button_tooltip = Enlace a la página de instrucciones.
about_translation_button_tooltip = Enlace a la página de Crowdin con traducciones de aplicaciones. Oficialmente se admiten polaco e inglés.
about_repository_button = Repositorio
about_donation_button = Donativo
about_instruction_button = Instrucción
about_translation_button = Traducción
# Header
header_setting_button_tooltip = Abre el diálogo de ajustes.
header_about_button_tooltip = Abre el diálogo con información sobre la aplicación.

# Settings


## General

settings_number_of_threads = Número de hilos usados
settings_number_of_threads_tooltip = Número de hilos usados, 0 significa que se utilizarán todos los hilos disponibles.
settings_use_rust_preview = Usar librerías externas en su lugar gtk para cargar vistas previas
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = ¡Necesitas reiniciar la aplicación para aplicar la configuración!
settings_ignore_other_filesystems = Ignorar otros sistemas de ficheros (sólo Linux)
settings_ignore_other_filesystems_tooltip =
    ignora los archivos que no están en el mismo sistema de archivos que los directorios buscados.
    
    Funciona igual que la opción -xdev en encontrar el comando en Linux
settings_save_at_exit_button_tooltip = Guardar configuración en archivo al cerrar la aplicación.
settings_load_at_start_button_tooltip =
    Cargar la configuración desde el archivo al abrir la aplicación.
    
    Si no está habilitado, se usarán los ajustes por defecto.
settings_confirm_deletion_button_tooltip = Mostrar el diálogo de confirmación al hacer clic en el botón borrar.
settings_confirm_link_button_tooltip = Mostrar el diálogo de confirmación al hacer clic en el botón hard/symlink.
settings_confirm_group_deletion_button_tooltip = Mostrar el diálogo de advertencia al intentar eliminar todos los registros del grupo.
settings_show_text_view_button_tooltip = Mostrar el panel de texto en la parte inferior de la interfaz de usuario.
settings_use_cache_button_tooltip = Usar caché de archivos.
settings_save_also_as_json_button_tooltip = Guardar caché en formato JSON (legible por seres humanos). Es posible modificar su contenido. La caché de este archivo será leída automáticamente por la aplicación si la caché del formato binario (con la extensión binaria) no se encuentra.
settings_use_trash_button_tooltip = Mueve archivos a la papelera en su lugar eliminándolos permanentemente.
settings_language_label_tooltip = Idioma para la interfaz de usuario.
settings_save_at_exit_button = Guardar configuración al cerrar la aplicación
settings_load_at_start_button = Cargar configuración al abrir la aplicación
settings_confirm_deletion_button = Mostrar diálogo de confirmación al eliminar cualquier archivo
settings_confirm_link_button = Mostrar diálogo de confirmación cuando vincule archivos de forma dura o simbólica
settings_confirm_group_deletion_button = Mostrar diálogo de confirmación al eliminar todos los archivos del grupo
settings_show_text_view_button = Mostrar panel de texto inferior
settings_use_cache_button = Usar caché
settings_save_also_as_json_button = Guarda también la caché como archivo JSON
settings_use_trash_button = Mover archivos borrados a la papelera
settings_language_label = Idioma
settings_multiple_delete_outdated_cache_checkbutton = Borrar automáticamente entradas de caché obsoletas
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Eliminar resultados de caché obsoletos que apuntan a archivos inexistentes.
    
    Cuando está activado, la aplicación se asegura al cargar registros, de que todos los registros apuntan a archivos válidos (los rotos son ignorados).
    
    Desactivar esto ayudará al escanear archivos en unidades externas, por lo que las entradas de caché sobre ellas no serán purgadas en el siguiente escaneo.
    
    En el caso de tener cientos de miles de registros en caché, se sugiere habilitar esto, lo que acelerará la carga/guardado del caché al inicio/final del escaneo.
settings_notebook_general = General
settings_notebook_duplicates = Duplicados
settings_notebook_images = Imágenes similares
settings_notebook_videos = Vídeos similares

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Muestra la vista previa en el lado derecho (al seleccionar un archivo de imagen).
settings_multiple_image_preview_checkbutton = Mostrar vista previa de la imagen
settings_multiple_clear_cache_button_tooltip =
    Limpiar manualmente la caché de entradas desactualizadas.
    Esto solo debe utilizarse si se ha desactivado la limpieza automática.
settings_multiple_clear_cache_button = Eliminar resultados obsoletos de la caché.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Oculta todos los archivos excepto uno, si todos apuntan a los mismos datos (están en línea dura).
    
    Ejemplo: En el caso en que hay (en el disco) siete archivos que están estrechamente vinculados a datos específicos y un archivo diferente con los mismos datos pero un inodio diferente, luego en el buscador duplicado, sólo se mostrará un archivo único y un archivo de los enlazados.
settings_duplicates_minimal_size_entry_tooltip =
    Establece el tamaño mínimo de archivo que se almacenará en caché.
    
    Al elegir un valor más pequeño se generarán más registros. Esto acelerará la búsqueda, pero ralentizará la carga/guardado de la caché.
settings_duplicates_prehash_checkbutton_tooltip =
    Activa el almacenamiento en caché de prehash (un hash calculado desde una pequeña parte del archivo) que permite despedir los resultados no duplicados anteriormente.
    
    Está deshabilitado por defecto porque puede causar derribos lentos en algunas situaciones.
    
    Es altamente recomendable usarlo para escanear cientos de miles o millones de archivos, ya que puede acelerar la búsqueda varias veces.
settings_duplicates_prehash_minimal_entry_tooltip = Tamaño mínimo de la entrada en caché.
settings_duplicates_hide_hard_link_button = Ocultar enlaces duros (sólo Linux y macOS)
settings_duplicates_prehash_checkbutton = Usar caché prehash
settings_duplicates_minimal_size_cache_label = Tamaño mínimo de los archivos (en bytes) guardados en la caché
settings_duplicates_minimal_size_cache_prehash_label = Tamaño mínimo de archivos (en bytes) guardados en caché prehash

## Saving/Loading settings

settings_saving_button_tooltip = Guardar la configuración de configuración actual en el archivo.
settings_loading_button_tooltip = Cargar los ajustes desde el archivo y reemplazar la configuración actual con ellos.
settings_reset_button_tooltip = Restablecer la configuración actual a la predeterminada.
settings_saving_button = Guardar configuración
settings_loading_button = Cargar configuración
settings_reset_button = Restablecer configuración

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Abre la carpeta donde se almacenan los archivos txt.
    
    Modificar los archivos de caché puede causar que se muestren resultados no válidos. Sin embargo, modificar la ruta puede ahorrar tiempo al mover una gran cantidad de archivos a una ubicación diferente.
    
    Puede copiar estos archivos entre ordenadores para ahorrar tiempo al escanear de nuevo para archivos (por supuesto, si tienen una estructura de directorios similar).
    
    En caso de problemas con la caché, estos archivos pueden ser eliminados. La aplicación los regenerará automáticamente.
settings_folder_settings_open_tooltip =
    Abrir la carpeta donde se almacena la configuración de Czkawka.
    
    ADVERTENCIA: Modificar manualmente la configuración puede romper su flujo de trabajo.
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
compute_found_bad_extensions = Se encontraron { $number_files } archivos con extensiones no válidas
# Progress window
progress_scanning_general_file = Escaneando archivo { $file_number }
progress_scanning_extension_of_files = Comprobando extensión de { $file_checked }/{ $all_files } archivo
progress_scanning_broken_files = Comprobando { $file_checked }/{ $all_files } archivo
progress_scanning_video = Hash de { $file_checked }/{ $all_files } vídeo
progress_scanning_image = Hash de { $file_checked }/{ $all_files } imagen
progress_comparing_image_hashes = Comparando hash de imagen { $file_checked }/{ $all_files }
progress_scanning_music_tags_end = Comparando etiquetas de { $file_checked }/{ $all_files } archivo de música
progress_scanning_music_tags = Leyendo etiquetas del archivo de música { $file_checked }/{ $all_files }
progress_scanning_music_content_end = Comparando huella dactilar de { $file_checked }/{ $all_files } archivo de música
progress_scanning_music_content = Calculando huella dactilar de { $file_checked }/{ $all_files } archivo de música
progress_scanning_empty_folders = Escaneando carpeta { $folder_number }
progress_scanning_size = Escaneando tamaño del archivo { $file_number }
progress_scanning_size_name = Escaneando nombre y tamaño del archivo { $file_number }
progress_scanning_name = Escaneando nombre del archivo { $file_number }
progress_analyzed_partial_hash = Has analizado el hash parcial de { $file_checked }/{ $all_files } archivos
progress_analyzed_full_hash = Se ha analizado el hash completo de { $file_checked }/{ $all_files } archivos
progress_prehash_cache_loading = Cargando caché prehash
progress_prehash_cache_saving = Guardando caché prehash
progress_hash_cache_loading = Cargando caché hash
progress_hash_cache_saving = Guardando caché hash
progress_cache_loading = Cargando caché
progress_cache_saving = Guardando caché
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
saving_loading_not_valid = La configuración "{ $data }" no existe en la versión actual de la aplicación.
# Invalid symlinks
invalid_symlink_infinite_recursion = Recursión infinita
invalid_symlink_non_existent_destination = Archivo de destino inexistente
# Other
selected_all_reference_folders = No se puede iniciar la búsqueda, cuando todos los directorios están establecidos como carpetas de referencia
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
delete_folder_failed = Error al eliminar la carpeta { $dir } porque la carpeta no existe, no tiene permiso o la carpeta no está vacía.
delete_items_label = { $items } archivos serán eliminados.
delete_items_groups_label = { $items } archivos de { $groups } grupos serán eliminados.
hardlink_failed = Error al vincular
hard_sym_invalid_selection_title_dialog = Selección no válida con algunos grupos
hard_sym_invalid_selection_label_1 = En algunos grupos sólo hay un registro seleccionado y será ignorado.
hard_sym_invalid_selection_label_2 = Para poder vincular estos archivos con el sistema, al menos dos resultados en el grupo deben ser seleccionados.
hard_sym_invalid_selection_label_3 = El primero en el grupo es reconocido como original y no se cambia, pero el segundo y posterior son modificados.
hard_sym_link_title_dialog = Confirmación de enlace
hard_sym_link_label = ¿Está seguro que desea enlazar estos archivos?
move_folder_failed = Error al mover la carpeta { $name }, razón { $reason }
move_file_failed = Error al mover el archivo { $name }, razón { $reason }
move_files_title_dialog = Elija la carpeta a la que desea mover los archivos duplicados
move_files_choose_more_than_1_path = Solo se puede seleccionar una ruta para poder copiar sus archivos duplicados, seleccionado { $path_number }.
move_stats = Mudado correctamente { $num_files }/{ $all_files } elementos
save_results_to_file = Resultados guardados en archivos txt y json en la carpeta { $name }
search_not_choosing_any_music = ERROR: Debe seleccionar al menos una casilla de verificación con tipos de búsqueda de música.
search_not_choosing_any_broken_files = ERROR: Debe seleccionar al menos una casilla de verificación con el tipo de ficheros rotos comprobados.
include_folders_dialog_title = Carpetas a incluir
exclude_folders_dialog_title = Carpetas a excluir
include_manually_directories_dialog_title = Añadir directorio manualmente
cache_properly_cleared = Caché correctamente borrada
cache_clear_duplicates_title = Limpiando caché duplicada
cache_clear_similar_images_title = Limpiando caché de imágenes similares
cache_clear_similar_videos_title = Limpiando caché de vídeos similares
cache_clear_message_label_1 = ¿Quiere borrar la caché de entradas obsoletas?
cache_clear_message_label_2 = Esta operación eliminará todas las entradas de caché que apunten a archivos no válidos.
cache_clear_message_label_3 = Esto puede acelerar ligeramente la carga/guardado en caché.
cache_clear_message_label_4 = ATENCIÓN: La operación eliminará todos los datos almacenados en caché de unidades externas desconectadas. Por lo tanto, cada hash tendrá que ser regenerado.
# Show preview
preview_image_resize_failure = Error al redimensionar la imagen { $name }.
preview_image_opening_failure = Error al abrir la imagen { $name }, razón { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupo { $current_group }/{ $all_groups } ({ $images_in_group } imágenes)
compare_move_left_button = L
compare_move_right_button = R
