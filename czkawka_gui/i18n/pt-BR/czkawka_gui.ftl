# Window titles
window_settings_title = Confirgurações
window_main_title = Czkawka (Hiccup)
window_progress_title = Verificando
window_compare_images = Comparar imagens
# General
general_ok_button = OK
general_close_button = FECHAR
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_year_checkbox = ano
music_bitrate_checkbox = Bitrate
music_genre_checkbox = gênero
music_length_checkbox = Comprimento
music_comparison_checkbox = Comparação aproximada
music_checking_by_tags = Etiquetas
music_checking_by_content = Conteúdo
same_music_seconds_label = Duração mínima do fragmento segundo tempo
same_music_similarity_label = Diferença máxima
music_compare_only_in_title_group = Comparar somente no título
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    Procurar por arquivos de música semelhantes por seu conteúdo pode ser configurado pela configuração:
    
    - Tempo mínimo de fragmento após o qual os arquivos de música podem ser identificados como similares
    - A diferença máxima entre dois fragmentos testados
    
    A chave para bons resultados é encontrar combinações sensíveis desses parâmetros, para fornecido.
    
    Definir o tempo mínimo para 5s e a diferença máxima para 1.0, irá procurar fragmentos quase idênticos nos arquivos.
    Um tempo de 20s e uma diferença máxima de 6.0, por outro lado, funciona bem para encontrar remixes/versões ao vivo, etc.
    
    Por padrão, cada arquivo de música é comparado entre si, e isso pode levar muito tempo para testar muitos arquivos, portanto, é geralmente melhor usar pastas de referência e especificar quais arquivos devem ser comparados (com a mesma quantidade de arquivos, A comparação de impressões digitais será mais rápida pelo menos 4 vezes do que nas pastas de referência).
music_comparison_checkbox_tooltip =
    Ele procura arquivos de música semelhantes usando AI, que usa aprendizado de máquina para remover parênteses de uma frase. Por exemplo, com essa opção habilitada, os arquivos em questão serão considerados duplicatas:
    
    S├wie^\\dziz├ło├b --- S├wieł dziz├ło├b (Remix Lato 2021)
duplicate_case_sensitive_name = Caso sensível
duplicate_case_sensitive_name_tooltip =
    Quando ativado, grupo apenas registo quando têm exatamente o mesmo nome, por exemplo. Z├ołd - Z├ołd
    
    Desabilitar tal opção irá agrupar nomes de grupos sem verificar se cada letra tem o mesmo tamanho, por exemplo, zweb+graphie://ka-perseus-graphie.s3.amazonaws.com/oŁD - Z³ołd
duplicate_mode_size_name_combo_box = Tamanho e Nome
duplicate_mode_name_combo_box = Nome:
duplicate_mode_size_combo_box = Tamanho
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka oferece 3 tipos de hashes:
    
    Blake3 - função hash criptográfica. Este é o padrão porque é muito rápido.
    
    CRC32 - função hash simples. Isto deveria ser mais rápido do que o Blake3, mas muito raramente pode ter algumas colisões.
    
    XXH3 - muito similar em desempenho e qualidade de hash do Blake3 (mas não criptográfico). Então, esses modos podem ser facilmente interalterados.
duplicate_check_method_tooltip =
    Por enquanto, Czkawka oferece três tipos de métodos para encontrar duplicados por:
    
    Nome - Localiza arquivos que têm o mesmo nome.
    
    Size - Localiza arquivos que têm o mesmo tamanho.
    
    Hash - Localiza arquivos que têm o mesmo conteúdo. Este modo de hashing o arquivo e, mais tarde, compara este hash para encontrar duplicatas. Este modo é a maneira mais segura de encontrar duplicatas. O app usa o cache pesado, então as verificações de segundo e mais longo dados devem ser muito mais rápidas do que o primeiro.
image_hash_size_tooltip =
    Cada imagem marcada produz um hash especial que pode ser comparado entre si. e uma pequena diferença entre eles significa que essas imagens são parecidas. O tamanho hash
    
    8 é muito bom para encontrar imagens que são apenas um pouco semelhantes ao original. Com um maior conjunto de imagens (>1000), isso irá produzir uma grande quantidade de falsos positivos, então eu recomendo usar um tamanho de hash maior neste caso.
    
    16 é o tamanho hash padrão que é um bom compromisso entre encontrar até mesmo algumas imagens semelhantes e ter apenas uma pequena quantidade de colisões hash.
    
    32 e 64 hashes só encontram imagens muito semelhantes, mas quase não devem ter falsos positivos (talvez, exceto algumas imagens com o canal alfa).
image_resize_filter_tooltip =
    To compute hash of image, the library must first resize it.
    
    Depend on chosen algorithm, the resulting image used to calculate hash will looks a little different.
    
    The fastest algorithm to use, but also the one which gives the worst results, is Nearest. It is enabled by default, because with 16x16 hash size lower quality it is not really visible.
    
    With 8x8 hash size it is recommended to use a different algorithm than Nearest, to have better groups of images.
image_hash_alg_tooltip =
    Users can choose from one of many algorithms of calculating the hash.
    
    Each has both strong and weaker points and will sometimes give better and sometimes worse results for different images.
    
    So, to determine the best one for you, manual testing is required.
big_files_mode_combobox_tooltip = Permite a busca de arquivos menores/maiores
big_files_mode_label = Arquivos verificados
big_files_mode_smallest_combo_box = O Menor
big_files_mode_biggest_combo_box = O Maior
main_notebook_duplicates = Arquivos duplicados
main_notebook_empty_directories = Diretórios Vazios
main_notebook_big_files = Arquivos grandes
main_notebook_empty_files = Arquivos vazios
main_notebook_temporary = Arquivos Temporários
main_notebook_similar_images = Imagens semelhantes
main_notebook_similar_videos = Vídeos similares
main_notebook_same_music = Músicas duplicadas
main_notebook_symlinks = Links simbólicos inválidos
main_notebook_broken_files = Arquivos quebrados
main_notebook_bad_extensions = Extensões inválidas
main_tree_view_column_file_name = Nome do arquivo
main_tree_view_column_folder_name = Nome da pasta
main_tree_view_column_path = Caminho
main_tree_view_column_modification = Data de modificação
main_tree_view_column_size = Tamanho
main_tree_view_column_similarity = Similaridade
main_tree_view_column_dimensions = cotas
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = ano
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Comprimento
main_tree_view_column_genre = gênero
main_tree_view_column_symlink_file_name = Nome do arquivo Symlink
main_tree_view_column_symlink_folder = Pasta Symlink
main_tree_view_column_destination_path = Caminho de Destino
main_tree_view_column_type_of_error = Tipo de erro
main_tree_view_column_current_extension = Extensão atual
main_tree_view_column_proper_extensions = Extensão adequada
main_label_check_method = Método de verificação
main_label_hash_type = Tipo de hash
main_label_hash_size = Tamanho do hash
main_label_size_bytes = Tamanho (bytes)
main_label_min_size = Mínimo
main_label_max_size = Máximo
main_label_shown_files = Número de arquivos exibidos
main_label_resize_algorithm = Redimensionar algoritmo
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = Áudio
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Arquivo
main_check_box_broken_files_image = Imagem:
check_button_general_same_size = Ignorar o mesmo tamanho
check_button_general_same_size_tooltip = Ignorar arquivos com tamanho idêntico nos resultados - geralmente estes são duplicados 1:1
main_label_size_bytes_tooltip = Tamanho dos arquivos que serão usados na verificação
# Upper window
upper_tree_view_included_folder_column_title = Pastas para Pesquisar
upper_tree_view_included_reference_column_title = Pastas de referência
upper_recursive_button = Recursiva
upper_recursive_button_tooltip = Se selecionado, pesquisar também por arquivos que não são colocados diretamente nas pastas escolhidas.
upper_manual_add_included_button = Adicionar Manual
upper_add_included_button = Adicionar
upper_remove_included_button = Excluir
upper_manual_add_excluded_button = Adicionar Manual
upper_add_excluded_button = Adicionar
upper_remove_excluded_button = Excluir
upper_manual_add_included_button_tooltip =
    Adicione o nome do diretório à mão.
    
    Para adicionar vários caminhos de uma vez, separá-los por ;
    
    /home/roman;/home/rozkaz adicionará dois diretórios /home/roman e /home/rozkaz
upper_add_included_button_tooltip = Adicionar novo diretório à pesquisa.
upper_remove_included_button_tooltip = Excluir diretório da busca
upper_manual_add_excluded_button_tooltip =
    Adicione nome de diretório excluído à mão.
    
    Para adicionar vários caminhos de uma vez, separá-los por ;
    
    /home/roman;/home/krokiet irá adicionar dois diretórios /home/roman e /home/keokiet
upper_add_excluded_button_tooltip = Adicionar diretório a ser excluído na pesquisa.
upper_remove_excluded_button_tooltip = Diretório excluído para exclusão.
upper_notebook_items_configuration = Configuração dos itens
upper_notebook_excluded_directories = Diretórios excluídos
upper_notebook_included_directories = Diretórios incluídos
upper_allowed_extensions_tooltip =
    Allowed extensions must be separated by commas (by default all are available).
    
    The following Macros, which add multiple extensions at once, are also available: IMAGE, VIDEO, MUSIC, TEXT.
    
    Usage example  ".exe, IMAGE, VIDEO, .rar, 7z" - this means that images (e.g. jpg, png), videos (e.g. avi, mp4), exe, rar, and 7z files will be scanned.
upper_excluded_extensions_tooltip =
    Lista de arquivos desabilitados que serão ignorados na verificação.
    
    Ao usar extensões permitidas e desativadas, este tem maior prioridade, então o arquivo não será marcado.
upper_excluded_items_tooltip =
    Excluded items must contain * wildcard and should be separated by commas.
    This is slower than Excluded Directories, so use it carefully.
upper_excluded_items = Itens Excluídos:
upper_allowed_extensions = Extensões permitidas:
upper_excluded_extensions = Extensões desabilitadas:
# Popovers
popover_select_all = Selecionar todos
popover_unselect_all = Desmarcar todos
popover_reverse = Seleção inversa
popover_select_all_except_oldest = Selecionar todos, exceto os mais antigos
popover_select_all_except_newest = Selecionar todos, exceto os mais recentes
popover_select_one_oldest = Selecione um mais antigo
popover_select_one_newest = Selecione um mais recente
popover_select_custom = Selecionar um personalizado
popover_unselect_custom = Desmarcar personalizado
popover_select_all_images_except_biggest = Selecionar tudo, exceto o maior
popover_select_all_images_except_smallest = Selecionar todos, exceto menor
popover_custom_path_check_button_entry_tooltip =
    Selecione registros por caminho.
    
    Exemplo de uso:
    /home/pimpek/rzecz.txt pode ser encontrado com /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Select records by file names.
    
    Example usage:
    /usr/ping/pong.txt can be found with *ong*
popover_custom_regex_check_button_entry_tooltip =
    Selecione registros por Regex.
    
    Com este modo, texto pesquisado é o caminho com o nome.
    
    Exemplo de uso:
    /usr/bin/ziemniak. xt pode ser encontrado com /ziem[a-z]+
    
    Isso usa a implementação regex Rust padrão. Você pode ler mais sobre isso aqui: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Ativa a detecção diferenciada de maiúsculas de minúsculas.
    
    Quando desabilitado /home/* encontra ambos /HoMe/roman e /home/roman.
popover_custom_not_all_check_button_tooltip =
    Impede selecionar todos os registros no grupo.
    
    Isso é habilitado por padrão, porque na maioria das situações, você não quer apagar arquivos originais e duplicados, mas quer deixar pelo menos um arquivo.
    
    AVISO: Essa configuração não funciona se você já selecionou manualmente todos os resultados em um grupo.
popover_custom_regex_path_label = Caminho
popover_custom_regex_name_label = Nome:
popover_custom_regex_regex_label = Caminho Regex + Nome
popover_custom_case_sensitive_check_button = Sensível ao caso
popover_custom_all_in_group_label = Não selecionar todos os registros no grupo
popover_custom_mode_unselect = Desmarcar Personalizado
popover_custom_mode_select = Selecionar personalizado
popover_sort_file_name = Nome do arquivo
popover_sort_folder_name = Nome Pasta
popover_sort_full_name = Nome completo
popover_sort_size = Tamanho
popover_sort_selection = Seleção
popover_invalid_regex = Regex inválido
popover_valid_regex = Regex é válido
# Bottom buttons
bottom_search_button = Pesquisa
bottom_select_button = Selecionar
bottom_delete_button = excluir
bottom_save_button = Guardar
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Mover-se
bottom_sort_button = Ordenar
bottom_search_button_tooltip = Iniciar busca
bottom_select_button_tooltip = Selecione registros. Apenas arquivos/pastas selecionadas podem ser processados posteriormente.
bottom_delete_button_tooltip = Excluir arquivos/pastas selecionadas.
bottom_save_button_tooltip = Salvar dados sobre a pesquisa em arquivo
bottom_symlink_button_tooltip =
    Create symbolic links.
    Only works when at least two results in a group are selected.
    First is unchanged and second and later are symlinked to first.
bottom_hardlink_button_tooltip =
    Criar links hardlinks. U
    Só funciona quando pelo menos dois resultados em um grupo são selecionados.
    O primeiro é inalterado e segundo e depois são vinculados ao primeiro.
bottom_hardlink_button_not_available_tooltip =
    Criar links hardlinks.
    O botão está desativado, porque links hardlinks não podem ser criados.
    Hardlinks só funcionam com privilégios de administrador no Windows, então certifique-se de executar o app como administrador.
    Se o aplicativo já funciona com tais privilégios, verifique se há questões semelhantes no Github.
bottom_move_button_tooltip =
    Moves files to chosen directory.
    It copies all files to the directory without preserving the directory tree.
    When trying to move two files with identical name to folder, second will fail and show error.
bottom_sort_button_tooltip = Ordena arquivos/pastas de acordo com o método selecionado.
bottom_show_errors_tooltip = Mostrar/ocultar painel de texto inferior.
bottom_show_upper_notebook_tooltip = Mostrar/ocultar painel superior do caderno.
# Progress Window
progress_stop_button = Interromper
progress_stop_additional_message = Parar pedido
# About Window
about_repository_button_tooltip = Link para a página do repositório com o código-fonte.
about_donation_button_tooltip = Link para a página de doação.
about_instruction_button_tooltip = Link para a página de instrução.
about_translation_button_tooltip = Link para a página do Crowdin com traduções de aplicativo. Oficialmente polonês e inglês são suportados.
about_repository_button = Repositório
about_donation_button = Doação
about_instruction_button = Instrução
about_translation_button = Tradução
# Header
header_setting_button_tooltip = Abre diálogo de configurações.
header_about_button_tooltip = Abre diálogo com informações sobre o aplicativo.

# Settings


## General

settings_number_of_threads = Número de threads usados
settings_number_of_threads_tooltip = Número de tópicos usados, 0 significa que todos os tópicos disponíveis serão usados.
settings_use_rust_preview = Usar bibliotecas externas em vez de gtk para carregar pré-visualizações
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = Você precisa reiniciar o aplicativo para aplicar as configurações!
settings_ignore_other_filesystems = Ignorar outros sistemas de arquivos (somente Linux)
settings_ignore_other_filesystems_tooltip =
    ignora arquivos que não estão no mesmo sistema de arquivos que os diretórios pesquisados.
    
    Funciona a mesma como a opção -xdev no comando localizar no Linux
settings_save_at_exit_button_tooltip = Salvar configuração em arquivo ao fechar o aplicativo.
settings_load_at_start_button_tooltip =
    Carregar configuração do arquivo ao abrir aplicativo.
    
    Se não estiver habilitado, configurações padrão serão usadas.
settings_confirm_deletion_button_tooltip = Mostrar diálogo de confirmação ao clicar no botão excluir.
settings_confirm_link_button_tooltip = Mostrar diálogo de confirmação ao clicar no botão hard/simbólica.
settings_confirm_group_deletion_button_tooltip = Mostrar caixa de diálogo de aviso ao tentar excluir todos os registros do grupo.
settings_show_text_view_button_tooltip = Exibir painel de texto na parte inferior da interface do usuário
settings_use_cache_button_tooltip = Usar cache de arquivo.
settings_save_also_as_json_button_tooltip = Salvar cache no formato JSON (legível humana). É possível modificar seu conteúdo. Cache deste arquivo será lido automaticamente pelo aplicativo se o cache de formato binário (com extensão bin) estiver faltando.
settings_use_trash_button_tooltip = Move arquivos para a lixeira em vez de excluí-los permanentemente.
settings_language_label_tooltip = Idioma para interface de usuário.
settings_save_at_exit_button = Salvar configuração ao fechar o app
settings_load_at_start_button = Carregar configuração ao abrir aplicativo
settings_confirm_deletion_button = Mostrar diálogo de confirmação quando apagar qualquer arquivo
settings_confirm_link_button = Mostrar a caixa de diálogo de confirmação quando vincular qualquer arquivo
settings_confirm_group_deletion_button = Mostrar diálogo de confirmação quando apagar todos os arquivos no grupo
settings_show_text_view_button = Mostrar painel de texto inferior
settings_use_cache_button = Usar cache
settings_save_also_as_json_button = Também salvar o cache como arquivo JSON
settings_use_trash_button = Mover os arquivos apagados para a lixeira
settings_language_label = IDIOMA
settings_multiple_delete_outdated_cache_checkbutton = Excluir entradas de cache desatualizadas automaticamente
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Delete outdated cache results which point to non-existent files.
    
    When enabled, app makes sure when loading records, that all records point to valid files (broken ones are ignored).
    
    Disabling this will help when scanning files on external drives, so cache entries about them will not be purged in the next scan.
    
    In the case of having hundred of thousands records in cache, it is suggested to enable this, which will speedup cache loading/saving at start/end of the scan.
settings_notebook_general = Gerais
settings_notebook_duplicates = Duplicados
settings_notebook_images = Imagens semelhantes
settings_notebook_videos = Vídeo similar

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Exibe pré-visualização no lado direito (ao selecionar um arquivo de imagem).
settings_multiple_image_preview_checkbutton = Mostrar pré-visualização da imagem
settings_multiple_clear_cache_button_tooltip =
    Manually clear the cache of outdated entries.
    This should only be used if automatic clearing has been disabled.
settings_multiple_clear_cache_button = Remover resultados desatualizados do cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Oculta todos os arquivos, exceto um, se todos apontarem para os mesmos dados (são hardlinked).
    
    Exemplo: No caso de haver (em disco) sete arquivos que são vinculados a dados específicos e um arquivo diferente com os mesmos dados, mas um arquivo diferente, em seguida no localizador duplicado, somente um arquivo exclusivo e um arquivo de links físicos serão mostrados.
settings_duplicates_minimal_size_entry_tooltip =
    Set the minimal file size which will be cached.
    
    Choosing a smaller value will generate more records. This will speedup search, but slowdown cache loading/saving.
settings_duplicates_prehash_checkbutton_tooltip =
    Enables caching of prehash (a hash computed from a small part of the file) which allows earlier dismissal of non-duplicated results.
    
    It is disabled by default because it can cause slowdowns in some situations.
    
    It is highly recommended to use it when scanning hundred of thousands or million files, because it can speedup search by multiple times.
settings_duplicates_prehash_minimal_entry_tooltip = Tamanho mínimo da entrada em cache.
settings_duplicates_hide_hard_link_button = Ocultar links rígidos (somente Linux e macOS)
settings_duplicates_prehash_checkbutton = Usar cache de pré-hash
settings_duplicates_minimal_size_cache_label = Tamanho mínimo dos arquivos (em bytes) salvos no cache
settings_duplicates_minimal_size_cache_prehash_label = Tamanho mínimo dos arquivos (em bytes) salvos no cache de pré-hash

## Saving/Loading settings

settings_saving_button_tooltip = Salve as configurações atuais no arquivo.
settings_loading_button_tooltip = Carregar configurações de arquivo e substituir a configuração atual por elas.
settings_reset_button_tooltip = Redefinir a configuração atual para a padrão.
settings_saving_button = Salvar configuração
settings_loading_button = Carregar configuração
settings_reset_button = Redefinir configuração

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Opens the folder where the cache txt files are stored.
    
    Modifying the cache files may cause invalid results to be shown. However, modifying path may save time when moving a big amount of files to a different location.
    
    You can copy these files between computers to save time on scanning again for files (of course if they have similar directory structure).
    
    In the case of problems with the cache, these files can be removed. The app will automatically regenerate them.
settings_folder_settings_open_tooltip =
    Abre a pasta onde a configuração do Czkawka está armazenada.
    
    AVISO: modificar a configuração manualmente pode danificar seu fluxo de trabalho.
settings_folder_cache_open = Abrir pasta cache
settings_folder_settings_open = Abrir pasta de configurações
# Compute results
compute_stopped_by_user = A pesquisa foi interrompida pelo usuário
compute_found_duplicates_hash_size = Encontrada { $number_files } duplicações em { $number_groups } grupos que adotaram { $size }
compute_found_duplicates_name = Encontrada { $number_files } duplicatas em grupos { $number_groups }
compute_found_empty_folders = Encontradas pastas { $number_files } vazias
compute_found_empty_files = Encontrados { $number_files } arquivos vazios
compute_found_big_files = Encontrados { $number_files } arquivos grandes
compute_found_temporary_files = Encontrados { $number_files } arquivos temporários
compute_found_images = Encontradas { $number_files } imagens similares em grupos { $number_groups }
compute_found_videos = Encontrados { $number_files } vídeos similares em grupos { $number_groups }
compute_found_music = Encontrado { $number_files } arquivos de música similares em grupos { $number_groups }
compute_found_invalid_symlinks = { $number_files } links simbólicos inválidos
compute_found_broken_files = Encontrados { $number_files } arquivos quebrados
compute_found_bad_extensions = Encontrados { $number_files } arquivos com extensões inválidas
# Progress window
progress_scanning_general_file = Escaneando arquivo { $file_number }
progress_scanning_extension_of_files = Verificando extensão do arquivo { $file_checked }/{ $all_files }
progress_scanning_broken_files = Checando { $file_checked }/{ $all_files } arquivo
progress_scanning_video = Hashing de { $file_checked }/{ $all_files } vídeo
progress_scanning_image = Hashing de { $file_checked }/{ $all_files } imagem
progress_comparing_image_hashes = Comparando { $file_checked }/{ $all_files } hash de imagem
progress_scanning_music_tags_end = Comparar etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_tags = Lendo etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_content_end = Comparação de impressão digital de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_content = Calculando impressão digital do arquivo { $file_checked }/{ $all_files } de música
progress_scanning_empty_folders = Verificando pasta { $folder_number }
progress_scanning_size = Verificando o tamanho do arquivo { $file_number }
progress_scanning_size_name = Verificando nome e tamanho do arquivo { $file_number }
progress_scanning_name = Verificando nome do arquivo { $file_number }
progress_analyzed_partial_hash = Hash parcial analisado de arquivos { $file_checked }/{ $all_files }
progress_analyzed_full_hash = Hash completo analisado de arquivos { $file_checked }/{ $all_files }
progress_prehash_cache_loading = Carregando cache de pré-hash
progress_prehash_cache_saving = Salvando cache pré-hash
progress_hash_cache_loading = Carregando cache de hash
progress_hash_cache_saving = Salvando cache de hash
progress_cache_loading = Carregando cache
progress_cache_saving = Salvando cache
progress_current_stage = Estágio atual:{ " " }
progress_all_stages = Todos os Stages:{ " " }
# Saving loading 
saving_loading_saving_success = Configuração salva no arquivo { $name }.
saving_loading_saving_failure = Falha ao salvar os dados de configuração do arquivo { $name }.
saving_loading_reset_configuration = A configuração atual foi limpa.
saving_loading_loading_success = Configuração de aplicativo carregada adequadamente.
saving_loading_invalid_string = Para a chave "{ $key }" encontrou um resultado inválido - "{ $result }" que não é uma string.
saving_loading_invalid_int = Para a chave "{ $key }" encontrou um resultado inválido - "{ $result }" que não é um inteiro.
saving_loading_invalid_bool = Para a chave "{ $key }" encontrou um resultado inválido - "{ $result }" que não é um bool.
saving_loading_decode_problem_bool = Falha ao decodificar o bool da tecla "{ $key }" encontrado "{ $result }" mas os valores permitidos são 0, 1, verdadeiro ou falso.
saving_loading_saving_same_keys = Tentando salvar a configuração com chave duplicada "{ $key }".
saving_loading_failed_to_get_home_directory = Falha ao obter o diretório inicial para abrir/salvar o arquivo de configuração.
saving_loading_folder_config_instead_file = Não é possível criar ou abrir o arquivo de configuração no caminho "{ $path }" porque já existe uma pasta.
saving_loading_failed_to_create_configuration_folder = Falha ao criar a pasta de configuração "{ $path }", reason "{ $reason }".
saving_loading_failed_to_create_config_file = Falha ao criar o arquivo de configuração "{ $path }", reason "{ $reason }".
saving_loading_failed_to_read_config_file = Não foi possível carregar a configuração de "{ $path }" porque não existe ou não é um arquivo.
saving_loading_failed_to_read_data_from_file = Não é possível ler dados do arquivo "{ $path }", reason "{ $reason }".
saving_loading_orphan_data = Dados órfãos encontrados "{ $data }" na linha "{ $line }".
saving_loading_not_valid = Definir "{ $data }" não existe na versão atual do aplicativo.
# Invalid symlinks
invalid_symlink_infinite_recursion = recursão infinita
invalid_symlink_non_existent_destination = Arquivo de destino inexistente
# Other
selected_all_reference_folders = Não é possível iniciar a busca, quando todos os diretórios estão definidos como pastas de referência
searching_for_data = Procurando dados, pode demorar um pouco, por favor, aguarde...
text_view_messages = MENSAGEM
text_view_warnings = ATENÇÕES
text_view_errors = ERROS
about_window_motto = Este programa é gratuito para o uso e sempre será.
# Various dialog
dialogs_ask_next_time = Perguntar da próxima vez
delete_file_failed = Falha ao excluir o arquivo { $name }, razão { $reason }
delete_title_dialog = Confirmação de exclusão
delete_question_label = Tem certeza de que deseja excluir arquivos?
delete_all_files_in_group_title = Confirmação de excluir todos os arquivos no grupo
delete_all_files_in_group_label1 = Em alguns grupos todos os registros estão selecionados.
delete_all_files_in_group_label2 = Tem certeza que deseja apagá-los?
delete_folder_failed = Falha ao excluir a pasta { $dir } porque a pasta não existe, você não tem permissão ou a pasta não está vazia.
delete_items_label = { $items } arquivos serão excluídos.
delete_items_groups_label = { $items } arquivos de { $groups } grupos serão excluídos.
hardlink_failed = Falha no hardlink
hard_sym_invalid_selection_title_dialog = Seleção inválida com alguns grupos
hard_sym_invalid_selection_label_1 = Em alguns grupos há apenas um registro selecionado e será ignorado.
hard_sym_invalid_selection_label_2 = Para ser capaz de vincular estes arquivos, pelo menos dois resultados no grupo precisam ser selecionados.
hard_sym_invalid_selection_label_3 = O primeiro no grupo é reconhecido como original e não é alterado, mas o segundo e mais tarde são modificados.
hard_sym_link_title_dialog = Link de confirmação
hard_sym_link_label = Tem certeza que deseja vincular esses arquivos?
move_folder_failed = Falha ao mover a pasta { $name }, razão { $reason }
move_file_failed = Falha ao mover o arquivo { $name }, razão { $reason }
move_files_title_dialog = Escolha a pasta para a qual você deseja mover arquivos duplicados
move_files_choose_more_than_1_path = Apenas um caminho pode ser selecionado para ser capaz de copiar os arquivos duplicados, selecionado { $path_number }.
move_stats = Propriamente movido { $num_files }/{ $all_files } itens
save_results_to_file = Resultados salvos tanto para arquivos txt quanto json na pasta { $name }.
search_not_choosing_any_music = ERRO: Você deve selecionar pelo menos uma caixa de seleção com tipos de busca de música.
search_not_choosing_any_broken_files = ERRO: Você deve selecionar pelo menos uma caixa de seleção com tipo de arquivos quebrados.
include_folders_dialog_title = Pastas para incluir
exclude_folders_dialog_title = Pastas para excluir
include_manually_directories_dialog_title = Adicionar diretório manualmente
cache_properly_cleared = Cache devidamente limpo
cache_clear_duplicates_title = Limpando cache duplicados
cache_clear_similar_images_title = Limpando cache de imagens similares
cache_clear_similar_videos_title = Limpando cache de vídeos similares
cache_clear_message_label_1 = Deseja limpar o cache de entradas desatualizadas?
cache_clear_message_label_2 = Esta operação removerá todas as entradas de cache que apontam para arquivos inválidos.
cache_clear_message_label_3 = Isto pode acelerar um pouco o carregamento/salvamento para o cache.
cache_clear_message_label_4 = AVISO: Operação irá remover todos os dados em cache de unidades externas desconectadas. Então cada hash precisará ser regenerado.
# Show preview
preview_image_resize_failure = Falha ao redimensionar a imagem { $name }.
preview_image_opening_failure = Falha ao abrir a imagem { $name }, razão { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupo { $current_group }/{ $all_groups } ({ $images_in_group } imagens)
compare_move_left_button = O
compare_move_right_button = R
