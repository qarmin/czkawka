# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplicados
tool_empty_folders = Pastas vazias
tool_similar_images = Imagens semelhantes
tool_empty_files = Arquivos vazios
tool_temporary_files = Arquivos Temporários
tool_big_files = Maiores arquivos
tool_broken_files = Arquivos quebrados
tool_bad_extensions = Extensões inválidas
tool_same_music = Músicas duplicadas
tool_bad_names = Nomes ruins
tool_exif_remover = Dados EXIF
tool_similar_videos = Vídeos similares (Áudio)
tool_directories = Diretórios
tool_settings = Confirgurações
# Home screen tool card descriptions
home_dup_description = Localizar arquivos com o mesmo conteúdo
home_empty_folders_description = Diretórios sem conteúdo
home_similar_images_description = Encontrar fotos visualmente semelhantes
home_empty_files_description = Arquivos com tamanho zero
home_temp_files_description = Arquivos temporários e em cache
home_big_files_description = Maior/Pequenos arquivos em disco
home_broken_files_description = PDF, áudio, imagens, arquivos
home_bad_extensions_description = Arquivos com extensão inválida
home_same_music_description = Arquivos de áudio similares por tags
home_bad_names_description = Arquivos com caracteres problemáticos no nome
home_exif_description = Imagens com metadados EXIF
home_similar_videos_description = Encontrar vídeos com áudio similar (sem necessidade do FFmpeg)
# Results list
scanning = Verificando em andamento...
stopping = Parando...
no_results = Nenhum resultado
press_start = Pressione INICIAR para verificar
select_label = Sel.
deselect_label = Desel.
list_label = Lista
gallery_label = Gal.
# Selection popup
selection_popup_title = Selecionar
select_all = Selecionar todos
select_except_one = Selecionar todos, exceto um
select_except_largest = Selecionar todos, exceto o maior
select_except_smallest = Selecionar todos, exceto menor
select_largest = Selecionar maior
select_smallest = Selecionar menor
select_except_highest_res = Selecionar todos, exceto a maior resolução
select_except_lowest_res = Selecionar todos, exceto resolução mais baixa
select_highest_res = Selecionar a maior resolução
select_lowest_res = Selecionar resolução mais baixa
invert_selection = Inverter seleção
close = FECHAR
# Deselection popup
deselection_popup_title = Desselecionar
deselect_all = Desmarcar todos
deselect_except_one = Desmarcar todos, exceto um
# Confirm popup
cancel = cancelar
delete = excluir
rename = Renomear
# Delete errors popup
delete_errors_title = Falha ao excluir alguns arquivos:
ok = Certo
# Stopping overlay
stopping_overlay_title = Parando
stopping_overlay_body =
    Finalizando a varredura atual...
    Por favor, aguarde.
# Permission popup
permission_title = Acesso ao arquivo
permission_body = Para verificar arquivos, o app precisa de acesso ao armazenamento do dispositivo. Sem esta permissão, o escaneamento não será possível.
grant = Conceder
no_permission_scan_warning = Sem acesso de arquivo - conceder permissão para escanear
# Settings screen tabs
settings_tab_general = Gerais
settings_tab_tools = Ferramentas
settings_tab_diagnostics = Informações
# Settings - General tab
settings_use_cache = Usar cache
settings_use_cache_desc = Acelera verificações subsequentes (hash/imagens)
settings_ignore_hidden = Ignorar arquivos ocultos
settings_ignore_hidden_desc = Arquivos e pastas que começam com '.'
settings_show_notification = Notificar quando terminar a varredura
settings_show_notification_desc = Mostrar uma notificação do sistema na conclusão de verificação
settings_notify_only_background = Somente quando em segundo plano
settings_notify_only_background_desc = Pular notificação se o aplicativo estiver visível
notifications_disabled_banner = Notificações desativadas
notifications_enable_button = Habilitado
settings_scan_label = PROCURAR
settings_filters_label = FILTROS (algumas ferramentas)
settings_min_file_size = Tamanho mínimo do arquivo
settings_max_file_size = Tamanho máximo do arquivo
settings_language = IDIOMA
settings_language_restart = Requer reinicialização do aplicativo
settings_common_label = CONFIGURAÇÕES COMUM
settings_excluded_items = ITENS EXCLUÍDOS (padrões globas, separados por vírgula)
settings_excluded_items_placeholder = ex.: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EXTENSÕES PERMITIDAS (vazio = todos)
settings_allowed_extensions_placeholder = ex. jpg, png, mp4
settings_excluded_extensions = EXTENSÕES EXCLUÍDAS
settings_excluded_extensions_placeholder = por exemplo, bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLICADOS
settings_check_method_label = MÉTODO DE COMPARISÃO
settings_check_method = Método
settings_hash_type_label = TIPO DE HASH
settings_hash_type = Tipo de hash
settings_hash_type_desc = Blake3 - é recomendada opção, CRC32 têm uma pequena chance de falsos positivos
settings_similar_images_header = IMAGENS DO SIMILAR
settings_similarity_preset = Limite de similaridade
settings_similarity_desc = Muito alta = apenas quase idêntica
settings_hash_size = Tamanho do hash
settings_hash_size_desc = Tamanho maior, tem menos falsos positivos, mas também encontra menos imagens semelhantes
settings_hash_alg = Algoritmo de hash
settings_image_filter = Redimensionar filtro
settings_geometric_invariance = invariação geométrica
settings_ignore_same_size = Ignorar imagens com as mesmas dimensões
settings_gallery_image_fit_cover = Galeria: cortar para um quadrado
settings_gallery_image_fit_cover_desc = Preencher o bloco; desativar para manter a proporção original
settings_big_files_header = ARQUIVOS MAIS
settings_search_mode = Modo de pesquisa
settings_file_count = Contagem de arquivos
settings_same_music_header = DUPLICADOS MÚSICAS
settings_music_check_method = Modo de comparação
settings_music_compare_tags_label = TAGS COMPARADAS
settings_music_title = Título
settings_music_artist = Artista
settings_music_year = ano
settings_music_length = Comprimento
settings_music_genre = gênero
settings_music_bitrate = Taxa de bits
settings_music_approx = Comparação de tag aproximada
settings_temporary_files_header = ARQUIVOS DO TEMPORÁRIO
settings_temporary_files_extensions_label = EXTENSÕES
settings_temporary_files_extensions_placeholder = Exemplos: .tmp, .bak, ~
settings_temporary_files_reset = Restaurar padrões
settings_broken_files_header = ARQUIVOS DE TREINO
settings_broken_files_note = Varredura intensiva de recursos. Para obter melhor desempenho, use Krokiet no desktop.
settings_broken_files_types_label = TIPOS DE ALTERAÇÃO
settings_broken_audio = Áudio
settings_broken_pdf = Pdf
settings_broken_archive = Arquivo
settings_broken_image = Imagem:
settings_broken_font = Fonte
settings_broken_markup = Marcação (JSON/XML/TOML)
settings_similar_videos_header = SIMILAR VÍDEOS (AUDIO)
settings_similar_videos_audio_preset = Predefinição de similaridade de áudio
settings_similar_videos_audio_preset_desc = Controla com que rigor o áudio deve corresponder
settings_bad_names_header = NOMES DO BAD
settings_bad_names_checks_label = CORAÇÕES
settings_bad_names_uppercase_ext = Extensão maiúscula
settings_bad_names_emoji = Emoji em nome
settings_bad_names_space = Espaços ao início/final
settings_bad_names_non_ascii = Caracteres não ASCII
settings_bad_names_duplicated = Caracteres repetidos
settings_ignore_same_resolution = Ignorar imagens com a mesma resolução
# Settings - Appearance section
settings_appearance_label = APARÊNCIA
settings_dark_theme = Tema escuro
settings_dark_theme_desc = Use esquema de cores escuras
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTICAS
diagnostics_thumbnails = Cache de miniaturas
diagnostics_app_cache = Cache do app
diagnostics_refresh = atualizar
diagnostics_clear_thumbnails = Limpar miniaturas
diagnostics_open_thumbnails_folder = Abrir pasta
diagnostics_clear_cache = Limpar cache
diagnostics_open_cache_folder = Abrir pasta
diagnostics_export_logs = Exportar logs
logs_label = REGISTROS
logs_export_title = Exportar logs
logs_export_saved = Registros copiados para:
logs_export_failed = Não foi possível exportar os logs
diagnostics_collect_test = Teste de acesso a arquivos
diagnostics_collect_test_desc = Verifique quantos arquivos estão acessíveis
diagnostics_collect_test_run = Executar
diagnostics_collect_test_stop = Interromper
collect_test_cancelled = Parado pelo usuário
diag_confirm_clear_thumbnails = Limpar todo o cache de miniaturas?
diag_confirm_clear_cache = Limpar todo o cache de apps?
about_repo = Repositório
about_translate = Traduções
about_donate = SUPORTE
# Collect-test result popup
collect_test_title = Resultados do teste
collect_test_volumes = Volumes:
collect_test_folders = Pastas:
collect_test_files = Arquivos:
collect_test_time = Hora:
# Licenses
licenses_label = LICENÇA
third_party_licenses = Licenças de terceiros
licenses_popup_title = Licenças de terceiros
# Directories screen
directories_include_header = incluir
directories_included = Incluído
directories_exclude_header = Excluir
directories_excluded_header = Excluído
directories_add = incluir
no_paths = Sem caminhos - adicionar abaixo
directories_volume_header = Volume
directories_volume_refresh = atualizar
directories_volume_add = Adicionar
# Bottom navigation
nav_home = Iniciar
nav_dirs = Diretórios
nav_settings = Confirgurações
# Status messages set from Rust
status_ready = pronto
status_stopped = Parado
status_no_results = Nenhum resultado
status_deleted_selected = Excluído selecionado
status_deleted_with_errors = Excluído com erros
scan_not_started = Verificação não iniciada
found_items_prefix = Encontrado
found_items_suffix = Itens
deleted_items_prefix = Excluído
deleted_items_suffix = Itens
deleted_errors_suffix = erros
renamed_prefix = Renomeado
renamed_files_suffix = Arquivos
renamed_errors_suffix = erros
cleaned_exif_prefix = EXIF limpo de
cleaned_exif_suffix = Arquivos
cleaned_exif_errors_suffix = erros
rename_error_read_file_name = Impossível ler nome do arquivo
rename_error_read_directory = Impossível ler o diretório
and_more_prefix = ...e
and_more_suffix = Mais
# Gallery / delete popups
gallery_delete_button = excluir
gallery_back = Anterior
gallery_confirm_delete = Sim, excluir
deleting_files = Excluindo arquivos...
stop = Interromper
scanning_fallback = Escaneando...
app_subtitle = Em honra da Batalha de Cedynia (972 CE)
app_license = Frontend para o Núcleo Czkawka - GPL-3.0
about_app_label = SOBRE
cache_label = CACHAR
# Notification
scan_completed_notification = Verificação concluída - { $file_count } itens encontrados
# Confirm popups (set from Rust)
confirm_clean_exif = Tem certeza que deseja limpar as tags EXIF de { $n } arquivos selecionados?
confirm_delete_items = Tem certeza que deseja excluir { $n } itens selecionados?
gallery_confirm_delete_msg = Você está prestes a excluir { $total_images } imagens em { $total_groups } grupos.
gallery_confirm_delete_warning = Todos os itens estão selecionados em grupos de { $unsafe_groups }!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = O cálculo e a comparação das impressões digitais de áudio é muito intenso em recursos e pode demorar muito tempo. É recomendado usar o Krokiet em um computador para esta tarefa.
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } arquivos x { $per_file } / arquivo = { $total } no total
similar_images_group_header = { $count } imagens semelhantes
same_music_group_header = { $count } faixas semelhantes
similar_videos_group_header = { $count } vídeos semelhantes
# Rename confirmation
confirm_rename_items = Tem certeza que deseja renomear { $n } arquivos selecionados?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Maior
option_search_mode_smallest = Menor
option_similarity_very_high = Alto
option_similarity_high = alta
option_similarity_medium = Média
option_similarity_low = baixa
option_similarity_very_low = V.Baixa
option_similarity_minimal = Min.
option_check_method_hash = Hash
option_check_method_name = Nome:
option_check_method_size_and_name = Tamanho+Nome
option_check_method_size = Tamanho
option_music_method_tags = Etiquetas
option_music_method_audio = Áudio
option_min_size_none = Nenhuma
option_max_size_unlimited = Ilimitado
option_audio_preset_identical = idêntico
option_audio_preset_clip = Clipe mais longo
option_audio_preset_similar = Parecido
# Volume labels (shown in the directories screen)
volume_internal_storage = Armazenamento Interno
volume_sd_card = Cartão de memória (Cartão SD)
volume_storage = Volume de Armazenamento
# Directories screen
directories_referenced_tooltip = Referenciado (não excluído)
directories_include_section_header = INCLUÍDO
directories_exclude_section_header = EXCLUÍDO
directories_custom_paths = Caminhos Personalizados
directories_check_button = Analisar
directories_check_popup_title = Estatísticas do diretório
directories_check_label_included = Caminhos incluídos:
directories_check_label_excluded = Caminhos excluídos:
directories_check_label_referenced = Rotas de referência:
directories_check_label_would_scan = Arquivos para verificar:
directories_check_label_processable = Arquivos processáveis:
directories_check_scanning = Escaneando...
directories_check_warning_no_processable = Nenhum arquivo processável encontrado - verifique suas pastas incluídas/excluídas
path_edit_title_include = Adicionar para incluir
path_edit_title_exclude = Adicionar à Exclusão
path_edit_placeholder = Digite o caminho...
path_edit_not_exists = Caminho não existe
path_edit_is_dir = Diretório
path_edit_is_file = Arquivo
path_edit_no_newlines = Caminhos não podem conter novas linhas - Chave Enter não é permitida
ctx_menu_title = Abertas
ctx_open_file = Abrir item
ctx_open_folder = Abrir pasta pai
dir_open_folder = Abrir pasta
# Compare view
compare_label = Comparar
compare_loading = Carregando imagens…
compare_cancelling = Cancelando…
compare_computing = Calculando diff…
compare_mode_normal = Lado
compare_mode_split = Divisão
compare_mode_overlay = Camada
compare_mode_diff = SG
compare_res_mismatch = Diferentes resoluções - a diferença pode estar imprecisa
