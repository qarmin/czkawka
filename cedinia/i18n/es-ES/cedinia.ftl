# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplicados
tool_empty_folders = Carpetas vacías
tool_similar_images = Imágenes similares
tool_empty_files = Archivos vacíos
tool_temporary_files = Archivos temporales
tool_big_files = Archivos grandiosos
tool_broken_files = Archivos rotos
tool_bad_extensions = Extensiones incorrectas
tool_same_music = Duplicados de música
tool_bad_names = Nombres incorrectos
tool_exif_remover = Datos EXIF
tool_directories = Directorios
tool_settings = Ajustes
# Home screen tool card descriptions
home_dup_description = Buscar archivos con el mismo contenido
home_empty_folders_description = Directorios sin contenido
home_similar_images_description = Buscar fotos similares visualmente
home_empty_files_description = Archivos con tamaño cero
home_temp_files_description = Archivos temporales y cachés
home_big_files_description = Archivos más pequeños en el disco
home_broken_files_description = PDF, audio, imágenes, archivos
home_bad_extensions_description = Archivos con extensión inválida
home_same_music_description = Ficheros de audio similares por etiquetas
home_bad_names_description = Archivos con caracteres problemáticos en el nombre
home_exif_description = Imágenes con metadatos EXIF
# Results list
scanning = Escaneando en progreso...
stopping = Deteniendo...
no_results = Sin resultados
press_start = Presione START para escanear
select_label = Sel.
deselect_label = Desel.
list_label = Lista
gallery_label = Gal.
# Selection popup
selection_popup_title = Seleccionar
select_all = Seleccionar todo
select_except_one = Seleccionar todo excepto uno
select_except_largest = Seleccionar todo excepto mayor
select_except_smallest = Seleccionar todo excepto menor
select_largest = Seleccionar más grande
select_smallest = Seleccionar más pequeño
select_except_highest_res = Seleccionar todo excepto la resolución más alta
select_except_lowest_res = Seleccionar todo excepto la resolución más baja
select_highest_res = Seleccionar resolución más alta
select_lowest_res = Selecciona la resolución más baja
invert_selection = Invertir selección
close = Cerrar
# Deselection popup
deselection_popup_title = Deseleccionar
deselect_all = Deseleccionar todo
deselect_except_one = Deseleccionar todo excepto uno
# Confirm popup
cancel = Cancelar
delete = Eliminar
rename = Renombrar
# Delete errors popup
delete_errors_title = Error al eliminar algunos archivos:
ok = Ok
# Stopping overlay
stopping_overlay_title = Deteniendo
stopping_overlay_body =
    Finalizando el escaneo actual...
    Por favor espere.
# Permission popup
permission_title = Acceso al archivo
permission_body = Para escanear archivos, la aplicación necesita acceso al almacenamiento del dispositivo. Sin este permiso, no será posible escanear.
grant = Otorgar
no_permission_scan_warning = Sin acceso al archivo - conceder permiso para escanear
# Settings screen tabs
settings_tab_general = General
settings_tab_tools = Herramientas
settings_tab_diagnostics = Información
# Settings - General tab
settings_use_cache = Usar caché
settings_use_cache_desc = Acelera los análisis subsiguientes (hash/imágenes)
settings_ignore_hidden = Ignorar archivos ocultos
settings_ignore_hidden_desc = Archivos y carpetas que empiezan con '.'
settings_show_notification = Notificar cuando finalice el escaneo
settings_show_notification_desc = Mostrar una notificación del sistema al completar el escaneo
settings_notify_only_background = Sólo en segundo plano
settings_notify_only_background_desc = Omitir notificación si la aplicación es visible
notifications_disabled_banner = Notificaciones desactivadas
notifications_enable_button = Activar
settings_scan_label = ESCAN
settings_filters_label = ARCHIVOS (algunas herramientas)
settings_min_file_size = Tamaño mínimo del archivo
settings_max_file_size = Tamaño máximo de archivo
settings_language = Idioma
settings_language_restart = Requiere reiniciar la aplicación
settings_common_label = SETINGOS DE COMON
settings_excluded_items = ITEMS EXCLUIDO (patrones de glob, separados por comas)
settings_excluded_items_placeholder = ej. *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EXTENSIONES ALLOWADAS (vacío = todo)
settings_allowed_extensions_placeholder = ej. jpg, png, mp4
settings_excluded_extensions = EXTENSIONES EXCLUIDAS
settings_excluded_extensions_placeholder = ej. bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLICADOS
settings_check_method_label = MÉTODO DE COMPARISÓN
settings_check_method = Método
settings_hash_type_label = TIPO HASH
settings_hash_type = Tipo de Hash
settings_hash_type_desc = Blake3 - es la opción recomendada, CRC32 tiene pequeñas probabilidades de falsos positivos
settings_similar_images_header = IMÁGENES DE SIMILAR
settings_similarity_preset = Umbral de similitud
settings_similarity_desc = Muy alto = sólo casi idéntico
settings_hash_size = Tamaño hash
settings_hash_size_desc = Tamaños más grandes, tienen menos falsos positivos, pero también encuentran menos imágenes similares
settings_hash_alg = Algoritmo hash
settings_image_filter = Redimensionar filtro
settings_ignore_same_size = Ignorar imágenes con las mismas dimensiones
settings_gallery_image_fit_cover = Galería: recortar a cuadrado
settings_gallery_image_fit_cover_desc = Rellena el azulejo; desactiva para mantener la relación de aspecto original
settings_big_files_header = ARCHIVOS DE MEJOR
settings_search_mode = Modo de búsqueda
settings_file_count = Número de archivos
settings_same_music_header = DUPLICADOS MÚSICOS
settings_music_check_method = Modo de comparación
settings_music_compare_tags_label = TARJETAS COMPARADAS
settings_music_title = Título
settings_music_artist = Artista
settings_music_year = Año
settings_music_length = Longitud
settings_music_genre = Género
settings_music_bitrate = Tasa de bits
settings_music_approx = Comparación aproximada de etiquetas
settings_broken_files_header = ARCHIVOS DE BROKEN
settings_broken_files_note = Escaneo intensivo de recursos. Para un mejor rendimiento utilice Krokiet en el ordenador.
settings_broken_files_types_label = TIPOS CHECKADOS
settings_broken_audio = Audio
settings_broken_pdf = DF
settings_broken_archive = Archivar
settings_broken_image = Imagen
settings_bad_names_header = NOMBRES DE BAD
settings_bad_names_checks_label = CHEQUES
settings_bad_names_uppercase_ext = Extensión mayúsculas
settings_bad_names_emoji = Emoji en el nombre
settings_bad_names_space = Espacios al inicio/fin
settings_bad_names_non_ascii = Caracteres no ASCII
settings_bad_names_duplicated = Caracteres repetidos
# Settings - Diagnostics tab
diagnostics_header = DIAGNÓSTICO
diagnostics_thumbnails = Caché de miniaturas
diagnostics_app_cache = Caché de App
diagnostics_refresh = Refrescar
diagnostics_clear_thumbnails = Limpiar uñas
diagnostics_open_thumbnails_folder = Abrir carpeta
diagnostics_clear_cache = Limpiar caché
diagnostics_open_cache_folder = Abrir carpeta
diagnostics_collect_test = Prueba de acceso de archivo
diagnostics_collect_test_desc = Comprobar cuántos archivos son accesibles
diagnostics_collect_test_run = Ejecutar
diagnostics_collect_test_stop = Parar
collect_test_cancelled = Detenido por el usuario
diag_confirm_clear_thumbnails = ¿Borrar todos los cachés en miniatura?
diag_confirm_clear_cache = ¿Borrar todo el caché de aplicaciones?
about_repo = Repositorio
about_translate = Traducciones
about_donate = Soporte
# Collect-test result popup
collect_test_title = Resultados de la prueba
collect_test_volumes = Volúmenes:
collect_test_folders = Carpetas:
collect_test_files = Archivos:
collect_test_time = Tiempo:
# Licenses
licenses_label = LICENCIA
third_party_licenses = Licencias de terceros
licenses_popup_title = Licencias de terceros
# Directories screen
directories_include_header = Incluye
directories_included = Incluye
directories_exclude_header = Excluir
directories_excluded_header = Excluido
directories_add = Incluye
no_paths = No hay rutas - añadir abajo
directories_volume_header = Volúmenes
directories_volume_refresh = Refrescar
directories_volume_add = Añadir
# Bottom navigation
nav_home = Comenzar
nav_dirs = Directorios
nav_settings = Ajustes
# Status messages set from Rust
status_ready = Listo
status_stopped = Detenido
status_no_results = Sin resultados
status_deleted_selected = Borrado seleccionado
status_deleted_with_errors = Eliminado con errores
scan_not_started = Escaneo no iniciado
found_items_prefix = Encontrado
found_items_suffix = objetos
deleted_items_prefix = Eliminado
deleted_items_suffix = objetos
deleted_errors_suffix = errores
renamed_prefix = Renombrado
renamed_files_suffix = archivos
renamed_errors_suffix = errores
cleaned_exif_prefix = EXIF limpiado de
cleaned_exif_suffix = archivos
cleaned_exif_errors_suffix = errores
and_more_prefix = ...y
and_more_suffix = más
# Gallery / delete popups
gallery_delete_button = Eliminar
gallery_back = Atrás
gallery_confirm_delete = Sí, eliminar
deleting_files = Eliminando archivos...
stop = Parar
files_suffix = archivos
scanning_fallback = Escaneando...
app_subtitle = En honor a la batalla de Cedynia (972 CE)
app_license = Frontend para el núcleo del kawka - GPL-3.0
about_app_label = AYUDA
cache_label = CACHÉ
# Notification
scan_completed_notification = Escaneo completado - { $file_count } elementos encontrados
# Confirm popups (set from Rust)
confirm_clean_exif = ¿Está seguro que desea limpiar etiquetas EXIF de { $n } archivos seleccionados?
confirm_delete_items = ¿Está seguro que desea eliminar { $n } elementos seleccionados?
gallery_confirm_delete_msg = Estás a punto de eliminar las imágenes { $total_images } en los grupos { $total_groups}.
gallery_confirm_delete_warning = Todos los elementos están seleccionados en grupos { $unsafe_groups}!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Calcular y comparar las huellas dactilares de audio es muy intensivo en recursos y puede llevar mucho tiempo. Se recomienda usar Krokiet en un sistema de escritorio para esta tarea.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Recopilando archivos
stage_scanning_name = Escaneando por nombre
stage_scanning_size_name = Escaneando por nombre y tamaño
stage_scanning_size = Escaneo por tamaño
stage_pre_hash = Pre-hash
stage_full_hash = Hashing
stage_loading_cache = Cargando caché
stage_saving_cache = Guardando caché
stage_calculating_image_hashes = Calculando hash de imagen
stage_comparing_images = Comparando imágenes
stage_calculating_video_hashes = Calculando hash de vídeo
stage_checking_files = Comprobando archivos
stage_checking_extensions = Comprobando extensiones
stage_checking_names = Comprobando nombres
stage_reading_music_tags = Leyendo etiquetas de música
stage_comparing_tags = Comparando etiquetas
stage_calculating_music_fingerprints = Calculando huellas musicales
stage_comparing_fingerprints = Comparando huellas dactilares
stage_extracting_exif = Leyendo etiquetas EXIF
stage_creating_video_thumbnails = Creando miniaturas de vídeo
stage_processing_videos = Procesando vídeos
stage_deleting = Eliminando archivos
stage_renaming = Renombrar archivos
stage_moving = Moviendo archivos
stage_hardlinking = Creando enlaces duros
stage_symlinking = Creando enlaces simbólicos
stage_optimizing_videos = Optimizando vídeos
stage_cleaning_exif = Limpiando EXIF
# Group headers in scan results
duplicates_group_header = { $count } archivos x { $per_file } / file = { $total } en total
similar_images_group_header = { $count } imágenes similares
same_music_group_header = { $count } pistas similares
# Rename confirmation
confirm_rename_items = ¿Está seguro que desea renombrar los archivos seleccionados { $n}?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Grandeza
option_search_mode_smallest = Pequeño
option_similarity_very_high = V.alto
option_similarity_high = Alta
option_similarity_medium = Medio
option_similarity_low = Baja
option_similarity_very_low = V.Muy bajo
option_similarity_minimal = Mín.
option_check_method_hash = Hash
option_check_method_name = Nombre
option_check_method_size_and_name = Tamaño+Nombre
option_check_method_size = Tamaño
option_music_method_tags = Etiquetas
option_music_method_audio = Audio
option_min_size_none = Ninguna
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Ilimitado
# Volume labels (shown in the directories screen)
volume_internal_storage = Almacenamiento interno
volume_sd_card = Tarjeta de memoria (tarjeta SD)
volume_storage = Volumen de almacenamiento
# Directories screen
directories_referenced_tooltip = Referenciado (no eliminado)
directories_include_section_header = INCLUIDO
directories_exclude_section_header = EXCLUIDO
directories_custom_paths = Rutas personalizadas
directories_check_button = Analizar
directories_check_popup_title = Estadísticas del directorio
directories_check_label_included = Rutas incluidas:
directories_check_label_excluded = Rutas excluidas:
directories_check_label_referenced = Rutas de referencia:
directories_check_label_would_scan = Archivos a escanear:
directories_check_label_processable = Archivos procesables:
directories_check_scanning = Escaneando...
directories_check_warning_no_processable = No se encontraron archivos procesables - verifique sus carpetas incluidas/excluidas
path_edit_title_include = Añadir a inclusión
path_edit_title_exclude = Añadir a Excluir
path_edit_placeholder = Entrar ruta...
path_edit_not_exists = La ruta no existe
path_edit_is_dir = Directorio
path_edit_is_file = Fichero
path_edit_no_newlines = Las rutas no pueden contener nuevas líneas — No se permite introducir la clave
ctx_menu_title = Abrir
ctx_open_file = Abrir elemento
ctx_open_folder = Abrir carpeta padre
