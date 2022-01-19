# Window titles
window_settings_title = Opções
window_main_title = Czkawka (Soluço)
window_progress_title = Verificando
window_compare_images = Comparar imagens
# General
general_ok_button = Ok
general_close_button = Fechar
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_album_title_checkbox = Título do Álbum
music_album_artist_checkbox = Artista do Álbum
music_year_checkbox = Ano
music_comparison_checkbox = Comparação aproximada
music_comparison_checkbox_tooltip =
    Ele procura por arquivos de música semelhantes usando AI, que usa aprendizado de máquina para remover parênteses de uma frase, por exemplo, com esta opção ativada, os arquivos em questão serão considerados duplicatas:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = Nome
duplicate_mode_size_combo_box = Tamanho
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka oferece 3 tipos de hashes, que poderiam ser usados:
    
    Blake3 - função hash criptográfica. Ele é usado como algoritmo de hash padrão porque é muito rápido.
    
    CRC32 - função hash simples. Deveria ser mais rápido do que o Blake3, mas provavelmente poderá ter muito poucas colisões.
    
    XXH3 - muito similar em caso de desempenho e qualidade de hash para o Blake3, então esses modos podem ser facilmente usados.
duplicate_check_method_tooltip =
    Por enquanto, Czkawka oferece três tipos de métodos para encontrar duplicados por:
    
    Nome - Localiza arquivos que têm o mesmo nome.
    
    Tamanho - Localiza arquivos com o mesmo tamanho.
    
    Hash - Localiza arquivos que têm o mesmo conteúdo. Este modo de hashes de arquivo e posteriores compara este hashes para encontrar duplicatas. Este modo é a maneira mais segura de encontrar duplicatas. A ferramenta utiliza fortemente o cache, portanto as verificações de segundo e mais novos dados devem ser muito mais rápidas do que antes.
image_hash_size_tooltip =
    Czkawka oferece o tamanho alterado do hash gerado para cada imagem. A causa hash maior permite encontrar imagens com uma quantidade menor de diferenças entre as imagens, mas também é um pouco mais lenta para usar.
    
    O valor padrão para hash é 8 bytes, o que permite encontrar imagens muito semelhantes e diferentes. 16 e 32 hashes devem ser usados apenas para imagens quase idênticas. Não deve ser usado o hash de 64 bytes, exceto situações em que são necessárias pequenas diferenças para encontrar
image_resize_filter_tooltip = Para calcular o hash de imagem, a biblioteca deve primeiro redimensioná-la. Depende no algoritmo escolhido, a imagem resultante parecerá pouco diferente. O algoritmo mais rápido para usar, mas também aquele que dá os piores resultados é mais rápido.
image_hash_alg_tooltip = Os usuários podem escolher um de muitos algoritmos para calcular o hash. Cada um tem pontos fortes e fracos e às vezes dará resultados melhores e às vezes piores para diferentes imagens, então escolha o melhor, testes manuais são necessários.
main_notebook_image_fast_compare = Comparação rápida
main_notebook_image_fast_compare_tooltip =
    Acelerar busca e comparação de hashes.
    
    No modo oposto ao normal em que cada hash é comparado entre si x vezes, onde x é similaridade que o usuário escolhe, neste modo sempre é usada uma comparação.
    
    Esta opção é recomendada quando se compara >10000 imagens com non 0(muito alta) similaridade.
main_notebook_duplicates = Arquivos Duplicados
main_notebook_empty_directories = Diretórios Vazios
main_notebook_big_files = Arquivos Grandes
main_notebook_empty_files = Arquivos Vazios
main_notebook_temporary = Arquivos Temporários
main_notebook_similar_images = Imagens Semelhantes
main_notebook_similar_videos = Vídeos Similares
main_notebook_same_music = Músicas Duplicadas
main_notebook_symlinks = Link Simbólico Inválido
main_notebook_broken_files = Arquivos Quebrados
main_tree_view_column_file_name = Nome do arquivo
main_tree_view_column_folder_name = Nome da pasta
main_tree_view_column_path = Caminho
main_tree_view_column_modification = Data de modificação
main_tree_view_column_size = Tamanho
main_tree_view_column_similarity = Similaridade
main_tree_view_column_dimensions = Tamanho
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = Ano
main_tree_view_column_album_title = Título do Álbum
main_tree_view_column_album_artist = Artista do Álbum
main_tree_view_column_symlink_file_name = Nome do arquivo Symlink
main_tree_view_column_symlink_folder = Pasta do Symlnik
main_tree_view_column_destination_path = Caminho de Destino
main_tree_view_column_type_of_error = Tipo de erro
main_label_check_method = Método de verificação
main_label_hash_type = Tipo de hash
main_label_hash_size = Tamanho do hash
main_label_size_bytes = Tamanho(bytes)
main_label_min_size = Mínimo
main_label_max_size = Máximo
main_label_shown_files = Número de arquivos exibidos
main_label_resize_algorithm = Redimensionar algoritmo
main_label_similarity = Similaridade{ " " }
check_button_general_same_size = Ignorar o mesmo tamanho
check_button_general_same_size_tooltip = Ignorar dos resultados, arquivos que têm tamanho idêntico - geralmente isto são duplicados 1:1
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
upper_manual_add_included_button_tooltip = Permite adicionar o nome do diretório à mão de busca.
upper_add_included_button_tooltip = Adicionar novo diretório à pesquisa.
upper_remove_included_button_tooltip = Excluir diretório da busca.
upper_manual_add_excluded_button_tooltip = Permite adicionar o nome de diretório excluído à mão.
upper_add_excluded_button_tooltip = Adicionar diretório a ser excluído na pesquisa.
upper_remove_excluded_button_tooltip = Diretório excluído para exclusão.
upper_notebook_items_configuration = Configuração dos itens
upper_notebook_excluded_directories = Diretórios excluídos
upper_notebook_included_directories = Diretórios incluídos
upper_allowed_extensions_tooltip =
    Extensões permitidas devem ser separadas por vírgulas (por padrão todas estão disponíveis).
    
    Macros IMAGE, VIDEO, MUSIC, TEXT que adiciona várias extensões de uma só vez também estão disponíveis.
    
    Exemplo de uso ".exe, IMAGE, VIDEO, .rar, 7z" - isto significa que imagem (e. jpg, png), vídeo(por exemplo, avi, mp4), exe, rar e 7z arquivos serão escaneados.
upper_excluded_items_tooltip =
    Itens excluídos devem conter * curinga e devem ser separados por vírgulas.
    Isto é mais lento do que os diretórios excluídos, então use-o com cuidado.
upper_excluded_items = Itens Excluídos:
upper_allowed_extensions = Extensões permitidas:
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
    Permite selecionar registros por seu caminho.
    
    Exemplo de uso:
    /home/pimpek/rzecz.txt pode ser encontrado com /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Permite selecionar registros por nomes de arquivos.
    
    Exemplo de uso:
    /usr/ping/pong.txt pode ser encontrado com *ong*
popover_custom_regex_check_button_entry_tooltip =
    Permite selecionar registros por Regex especificado.
    
    Com este modo, o texto pesquisado é o caminho com Nome.
    
    Exemplo de uso:
    /usr/bin/ziemniak. xt pode ser encontrado com /ziem[a-z]+
    
    Isso usa a implementação regex Rust padrão, então você pode ler mais sobre isso em https://docs.rs/regex.
popover_custom_not_all_check_button_tooltip =
    Impede de selecionar todos os registros no grupo.
    
    Isto é habilitado por padrão, porque na maioria das situações o usuário não quer excluir arquivos originais e duplicados mas quer deixar pelo menos um arquivo.
    
    Aviso: Esta configuração não funciona se o usuário já tiver selecionado todos os resultados em grupo manualmente.
popover_custom_regex_path_label = Caminho
popover_custom_regex_name_label = Nome
popover_custom_regex_regex_label = Caminho Regex + Nome
popover_custom_all_in_group_label = Não selecionar todos os registros no grupo
popover_custom_mode_unselect = Desmarcar Personalizado
popover_custom_mode_select = Selecionar personalizado
popover_invalid_regex = Regex inválido
popover_valid_regex = Regex é válido
# Bottom buttons
bottom_search_button = Pesquisa
bottom_select_button = Selecionar
bottom_delete_button = Excluir
bottom_save_button = Guardar
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Mover-se
bottom_search_button_tooltip = Comece a procurar arquivos/pastas.
bottom_select_button_tooltip = Seleciona registros. Apenas arquivos/pastas selecionadas podem ser processados mais tarde.
bottom_delete_button_tooltip = Excluir arquivos/pastas selecionadas.
bottom_save_button_tooltip = Salvar dados sobre a pesquisa em arquivo
bottom_symlink_button_tooltip =
    Cria links simbólicos.
    Só funciona quando pelo menos 2 resultados no grupo são selecionados.
    O primeiro é inalterado e o segundo e mais tarde são associados aos primeiros pontos simbólicos.
bottom_hardlink_button_tooltip =
    Cria links básicos.
    Funciona somente quando pelo menos dois resultados no grupo são selecionados.
    O primeiro é inalterado e o segundo e posterior são vinculados ao primeiro.
bottom_move_button_tooltip =
    Move os arquivos para a pasta escolhida.
    Ele copia todos os arquivos para a pasta sem preservar a árvore de diretório.
    Quando tentar mover 2 arquivos com nome idêntico para a pasta, a segunda irá falhar e mostrar erro.
bottom_show_errors_tooltip = Mostrar/ocultar painel de erros inferior.
bottom_show_upper_notebook_tooltip = Mostrar/ocultar painel superior do caderno.
# Progress Window
progress_stop_button = Interromper
# About Window
about_repository_button_tooltip = Link para a página do repositório com o código-fonte.
about_donation_button_tooltip = Link para a página de doação.
about_instruction_button_tooltip = Link para a página de instrução.
about_translation_button_tooltip = Link para a página do Crowdin com traduções do aplicativo. Oficialmente polonês e inglês são suportados, mas qualquer ajuda com outro idioma será apreciada.
about_repository_button = Repositório
about_donation_button = Doação
about_instruction_button = Instrução
about_translation_button = Tradução
# Header
header_setting_button_tooltip = Abre diálogo de configurações.
header_about_button_tooltip = Abre diálogo com informações sobre o aplicativo.

# Settings


## General

settings_save_at_exit_button_tooltip = Salva a configuração no arquivo quando fechar o app.
settings_load_at_start_button_tooltip =
    Carregando ao iniciar a configuração do arquivo.
    
    Não selecionar esta opção carregará as configurações padrão.
settings_confirm_deletion_button_tooltip = Mostra o diálogo de confirmação quando clicar no botão excluir.
settings_confirm_link_button_tooltip = Mostra o diálogo de confirmação quando clicar no botão hard/simbólico.
settings_confirm_group_deletion_button_tooltip = Mostra a caixa de diálogo ao tentar remover todos os registros do grupo.
settings_show_text_view_button_tooltip = Mostra o painel de erros embaixo.
settings_use_cache_button_tooltip = Opção para a qual não permite usar o recurso de cache.
settings_save_also_as_json_button_tooltip = Salve o cache para ser legível pelo formato JSON humano. É possível modificar seu conteúdo. Cache deste arquivo será lido automaticamente pelo aplicativo se o cache de formato binário (com extensão bin) estiver faltando.
settings_use_trash_button_tooltip = Quando ativado, ele move os arquivos para a lixeira em vez de excluí-los permanentemente.
settings_language_label_tooltip = Permite escolher o idioma da interface dos disponíveis.
settings_save_at_exit_button = Salvar configuração ao sair
settings_load_at_start_button = Carregar configuração no início
settings_confirm_deletion_button = Mostrar diálogo de confirmação quando apagar qualquer arquivo
settings_confirm_link_button = Mostrar a caixa de diálogo de confirmação quando vincular qualquer arquivo
settings_confirm_group_deletion_button = Mostrar diálogo de confirmação quando apagar todos os arquivos no grupo
settings_show_text_view_button = Mostrar painel de texto inferior
settings_use_cache_button = Usar cache
settings_save_also_as_json_button = Salvar o cache também em arquivo JSON
settings_use_trash_button = Mover os arquivos apagados para a lixeira
settings_language_label = Idioma
settings_multiple_delete_outdated_cache_checkbutton = Excluir entradas de cache desatualizadas automaticamente
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Permite excluir os resultados de cache desatualizados que apontam para arquivos inexistentes.
    
    Quando ativado, o aplicativo garante ao carregar os registros, que todos apontam para arquivos válidos e ignoram arquivos quebrados.
    
    Desativar esta opção, ajudará a escanear arquivos em unidades externas, então as entradas de cache sobre eles não serão removidas na próxima verificação.
    
    No caso de ter centenas de milhares de registros no cache, é sugerido para ativar esta opção, para acelerar o carregamento e salvamento do cache no início e fim do escaneamento.
settings_notebook_general = Gerais
settings_notebook_duplicates = Duplicados
settings_notebook_images = Imagens Semelhantes
settings_notebook_videos = Vídeo similar

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Mostrar pré-visualização no lado direito, ao selecionar o arquivo de imagem.
settings_multiple_image_preview_checkbutton = Mostrar pré-visualização da imagem
settings_multiple_clear_cache_button_tooltip =
    Limpar cache manualmente a partir de entradas desatualizadas.
    Deve ser usado apenas se a limpeza automática foi desativada.
settings_multiple_clear_cache_button = Remover resultados desatualizados do cache de imagens

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Oculta todos os arquivos com exceção de um, se for apontado para os mesmos dados (são linkados).
    
    Por exemplo, no caso de quando em disco existirem 7 arquivos que são vinculados a dados específicos e um arquivo diferente com os mesmos dados, mas com informações diferentes, então no localizador duplicado será visível apenas um arquivo exclusivo e um arquivo de links robustos.
settings_duplicates_minimal_size_entry_tooltip =
    Permite definir o tamanho mínimo de arquivo que será armazenado em cache.
    
    Escolhendo um valor menor, gerará mais registros que acelerarão a pesquisa, mas desacelerará o carregamento/salvamento do cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Habilita o cache de prehash (hash calculado a partir de uma pequena parte do arquivo), o que permite o lançamento anterior não duplicado de resultados.
    
    Isto está desabilitado por padrão porque pode causar lentidões em algumas situações.
    
    É altamente recomendado usá-lo durante o escaneamento de centenas de milhares de arquivos ou milhões, porque pode acelerar a busca várias vezes.
settings_duplicates_prehash_minimal_entry_tooltip = Tamanho mínimo da entrada em cache.
settings_duplicates_hide_hard_link_button = Ocultar links rígidos (apenas Linux e MacOS)
settings_duplicates_prehash_checkbutton = Usar cache de pré-hash
settings_duplicates_minimal_size_cache_label = Tamanho mínimo de arquivos em bytes salvos no cache
settings_duplicates_minimal_size_cache_prehash_label = Tamanho mínimo de arquivos em bytes salvos no cache de pré-hash

## Saving/Loading settings

settings_saving_button_tooltip = Salvar as configurações atuais no arquivo.
settings_loading_button_tooltip = Carregar configurações de arquivo e substituir a configuração atual por elas.
settings_reset_button_tooltip = Redefinir configuração atual para padrão.
settings_saving_button = Salvar configuração
settings_loading_button = Carregar configuração
settings_reset_button = Redefinir configuração

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Abre a pasta onde são armazenados arquivos txt com cache.
    
    Modificá-los pode causar a exibição de resultados inválidos, mas também modificar.. O caminho pode economizar tempo ao mover uma grande quantidade de arquivos para um lugar diferente.
    
    Você pode copiar este arquivo entre computadores para economizar tempo ao verificar novamente por arquivos (é claro que se eles tiverem uma estrutura de diretório semelhante).
    
    Em caso de problemas com o cache, estes arquivos podem ser removidos, então a aplicação irá regenerá-los automaticamente.
settings_folder_settings_open_tooltip =
    Abre uma pasta onde a configuração do Czkawka está armazenada.
    
    Modificá-los por mão, pode causar ruptura no seu fluxo de trabalho.
settings_folder_cache_open = Abrir pasta cache
settings_folder_settings_open = Abrir pasta de configurações
# Compute results
compute_stopped_by_user = A pesquisa foi interrompida pelo usuário
compute_found_duplicates_hash_size = Encontrados { $number_files } duplicados em { $number_groups } grupos que se tiveram { $size }
compute_found_duplicates_name = Encontrado { $number_files } duplicados em { $number_groups } grupos
compute_found_empty_folders = Encontradas { $number_files } pastas vazias
compute_found_empty_files = Encontrado { $number_files } arquivos vazios
compute_found_big_files = Encontrado { $number_files } arquivos grandes
compute_found_temporary_files = Encontrado { $number_files } arquivos temporários
compute_found_images = Encontrado { $number_files } imagens similares em { $number_groups } grupos
compute_found_videos = Encontrados { $number_files } vídeos similares em { $number_groups } grupos
compute_found_music = Encontrado { $number_files } arquivos de música similares em { $number_groups } grupos
compute_found_invalid_symlinks = Encontrado { $number_files } links simbólicos inválidos
compute_found_broken_files = Encontrado { $number_files } arquivos quebrados
# Progress window
progress_scanning_general_file = Escaneando { $file_number } arquivo
progress_scanning_broken_files = Verificando { $file_checked }/{ $all_files } arquivo
progress_scanning_video = Hashing de { $file_checked }/{ $all_files } vídeo
progress_scanning_image = Hashing de { $file_checked }/{ $all_files } imagem
progress_comparing_image_hashes = Comparação de { $file_checked }/{ $all_files } hash de imagem
progress_scanning_music_tags_end = Comparar etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_tags = Lendo etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_empty_folders = Verificando { $folder_number } pasta
progress_scanning_size = Verificando tamanho de { $file_number } arquivo
progress_scanning_name = Verificando nome de { $file_number } arquivo
progress_analyzed_partial_hash = Hash parcial analisado de { $file_checked }/{ $all_files } arquivos
progress_analyzed_full_hash = Hash completo analisado de { $file_checked }/{ $all_files } arquivos
progress_current_stage = Estágio atual:{ " " }
progress_all_stages = Todos os Stages:{ " " }
# Saving loading 
saving_loading_saving_success = Configuração salva no arquivo { $name }.
saving_loading_saving_failure = Falha ao salvar dados de configuração no arquivo { $name }.
saving_loading_reset_configuration = A configuração atual foi limpa.
saving_loading_loading_success = Configuração de aplicativo carregada adequadamente.
saving_loading_invalid_string = Para a chave "{ $key }" encontrou um resultado inválido - "{ $result }" que não é uma string.
saving_loading_invalid_int = Para a chave "{ $key }" encontrou um resultado inválido - "{ $result }" que não é um inteiro.
saving_loading_invalid_bool = Para a chave "{ $key }" encontrou um resultado inválido - "{ $result }" que não é um bool.
saving_loading_decode_problem_bool = Falha ao decodificar o bool da chave "{ $key }" encontrado "{ $result }" mas valores permitidos são 0, 1, verdadeiro ou falso.
saving_loading_saving_same_keys = Tentando salvar a configuração com chave duplicada "{ $key }".
saving_loading_failed_to_get_home_directory = Falha ao obter o diretório inicial para abrir/salvar o arquivo de configuração.
saving_loading_folder_config_instead_file = Não é possível criar ou abrir o arquivo de configuração no caminho "{ $path }" porque já existe uma pasta.
saving_loading_failed_to_create_configuration_folder = Falha ao criar a pasta de configuração "{ $path }", reason "{ $reason }".
saving_loading_failed_to_create_config_file = Falha ao criar o arquivo de configuração "{ $path }", reason "{ $reason }".
saving_loading_failed_to_read_config_file = Não foi possível carregar a configuração a partir de "{ $path }" porque não existe ou não é um arquivo.
saving_loading_failed_to_read_data_from_file = Não é possível ler dados do arquivo "{ $path }", reason "{ $reason }".
saving_loading_orphan_data = Dados órfãos encontrados "{ $data }" na linha "{ $line }".
saving_loading_not_valid = Definindo "{ $data }" não existe na versão atual do aplicativo.
# Invalid symlinks
invalid_symlink_infinite_recursion = Infinito recursão
invalid_symlink_non_existent_destination = Arquivo de destino inexistente
# Other
searching_for_data = Procurando dados, pode demorar um pouco, por favor, aguarde...
text_view_messages = MENSAGEM
text_view_warnings = ATENÇÕES
text_view_errors = ERROS
about_window_motto = Este programa é gratuito para o uso e sempre será.
# Various dialog
dialogs_ask_next_time = Perguntar da próxima vez
delete_file_failed = Falha ao remover o arquivo { $name }, razão { $reason }
delete_title_dialog = Confirmação de exclusão
delete_question_label = Tem certeza de que deseja excluir arquivos?
delete_all_files_in_group_title = Confirmação de excluir todos os arquivos no grupo
delete_all_files_in_group_label1 = Em alguns grupos há registros selecionados.
delete_all_files_in_group_label2 = Tem certeza que deseja apagá-los?
delete_folder_failed = Falha ao remover a pasta { $dir } porque a pasta não existe, você não tem permissões ou não está vazia.
delete_items_label = { $items } arquivos serão removidos.
delete_items_groups_label = { $items } arquivos de { $groups } grupos serão removidos.
hardlink_failed = Falha no hardlink
hard_sym_invalid_selection_title_dialog = Seleção inválida com alguns grupos
hard_sym_invalid_selection_label_1 = Em alguns grupos há apenas 1 registro selecionado e será ignorado.
hard_sym_invalid_selection_label_2 = Para poder hard/sym vincular esses arquivos, pelo menos 2 resultados no grupo precisam ser selecionados.
hard_sym_invalid_selection_label_3 = O primeiro no grupo é reconhecido como original e não é alterado, mas o segundo e mais tarde são modificados.
hard_sym_link_title_dialog = Link de confirmação
hard_sym_link_label = Tem certeza que deseja vincular este arquivo?
move_folder_failed = Falha ao mover a pasta { $name }, razão { $reason }
move_file_failed = Falha ao mover o arquivo { $name }, razão { $reason }
move_files_title_dialog = Escolha a pasta para a qual você deseja mover arquivos duplicados
move_files_choose_more_than_1_path = Apenas 1 caminho deve ser selecionado para poder copiar arquivos duplicados, selecionado { $path_number }.
move_stats = Devidamente movidos { $num_files }/{ $all_files } itens
save_results_to_file = Resultados salvos no arquivo { $name }
search_not_choosing_any_music = ERRO: Você deve selecionar pelo menos uma caixa de seleção com tipos de busca de música.
include_folders_dialog_title = Pastas para incluir
exclude_folders_dialog_title = Pastas para excluir
include_manually_directories_dialog_title = Adicionar diretório manualmente
cache_properly_cleared = Cache devidamente limpo
cache_clear_duplicates_title = Limpando cache duplicados
cache_clear_similar_images_title = Limpando cache de imagens similares
cache_clear_similar_videos_title = Limpando cache de vídeos similares
cache_clear_message_label_1 = Você deseja limpar o cache de entradas desatualizadas?
cache_clear_message_label_2 = Esta operação removerá todas as entradas de cache que apontam para arquivos inválidos.
cache_clear_message_label_3 = Isso pode acelerar um pouco o carregamento/salvamento para o cache.
cache_clear_message_label_4 = AVISO: Operação irá remover todos os dados em cache de unidades externas desconectadas, então o hash precisará ser gerado novamente.
# Show preview
preview_temporary_file = Falha ao abrir o arquivo de imagem temporário { $name }, reason { $reason }.
preview_0_size = Não é possível criar pré-visualização da imagem { $name }, com 0 de largura ou altura.
preview_temporary_image_save = Falha ao salvar o arquivo de imagem temporário para { $name }, reason { $reason }.
preview_temporary_image_remove = Falha ao excluir o arquivo de imagem temporária { $name }, reason { $reason }.
preview_failed_to_create_cache_dir = Falha ao criar o diretório { $name } necessário para a pré-visualização da imagem, razão { $reason }.
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupo { $current_group }/{ $all_groups } ({ $images_in_group } imagens)
compare_move_left_button = L
compare_move_right_button = R
