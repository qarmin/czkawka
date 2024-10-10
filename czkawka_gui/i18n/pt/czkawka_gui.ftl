# Window titles
window_settings_title = Configurações
window_main_title = Czkawka (Soluço)
window_progress_title = Escaneando
window_compare_images = Comparar Imagens
# General
general_ok_button = Ok
general_close_button = Fechar
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_year_checkbox = Ano
music_bitrate_checkbox = Taxa de Bits
music_genre_checkbox = Gênero
music_length_checkbox = Comprimento
music_comparison_checkbox = Comparação Aproximada
music_checking_by_tags = Etiquetas
music_checking_by_content = Conteúdo
same_music_seconds_label = Duração mínima de segundos do fragmento
same_music_similarity_label = Diferença máxima
music_compare_only_in_title_group = Comparar somente no título
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    Buscar por arquivos de música semelhantes por seu conteúdo pode ser configurado definindo:
    
    - O tempo mínimo de fragmento após o qual os arquivos de música podem ser identificados como semelhantes
    - A diferença máxima entre dois fragmentos testados
    
    A chave para bons resultados é achar combinações sensíveis desses parâmetros, para fornecido.
    
    Definir o tempo mínimo para 5s e a diferença máxima para 1.0 buscará fragmentos quase iguais nos arquivos.
    Um tempo de 20s e uma diferença máxima de 6.0, por outro lado, funciona bem para achar versões remixes/ao vivo, etc.
    
    Por padrão, cada arquivo de música é comparado entre si, e isso pode levar muito tempo para testar muitos arquivos, logo, é geralmente melhor usar pastas de referência e especificar quais arquivos devem ser comparados entre si (com a mesma quantidade de arquivos, comparar impressões digitais será pelo menos 4x mais rápido do que sem pastas de referência).
music_comparison_checkbox_tooltip =
    Ele busca arquivos de música semelhantes usando IA, que usa aprendizado de máquina para remover parênteses duma frase. Por exemplo, com esta opção ativada, os arquivos em questão serão considerados duplicatas:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Sensível a Maiúsculas e Minúsculas
duplicate_case_sensitive_name_tooltip =
    Quando ativado, o grupo só registra quando eles têm o mesmo nome, por exemplo, Żołd <-> Żołd
    
    Desativar esta opção agrupará os nomes sem verificar se cada letra é do mesmo tamanho, por exemplo, żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Tamanho e Nome
duplicate_mode_name_combo_box = Nome
duplicate_mode_size_combo_box = Tamanho
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Blake3 - função de hash criptográfico. Este é o padrão, por ser muito rápido.
    
    CRC32 - função de hash simples. Isto deve ser mais rápido que Blake3, mas pode muito raramente ter algumas colisões.
    
    XXH3 - muito semelhante em desempenho e qualidade de hash ao Blake3 (mas não criptográfico). Logo, tais modos podem ser facilmente intercambiáveis.
duplicate_check_method_tooltip =
    Por ora, o Czkawka oferece três tipos de métodos para encontrar duplicatas:
    
    Nome - Acha arquivos que têm o mesmo nome.
    
    Tamanho - Acha arquivos que têm o mesmo tamanho.
    
    Hash - Acha arquivos que têm o mesmo conteúdo. Este modo faz o hash do arquivo e então compara este hash para achar duplicatas. Este modo é o jeito mais seguro de achar duplicatas. O aplicativo usa muito cache, logo, a segunda e outras varreduras dos mesmos dados devem ser muito mais rápidas que a primeira.
image_hash_size_tooltip =
    Cada imagem marcada produz um hash especial que podem ser comparados entre si, e uma pequena diferença entre eles significa que essas imagens são parecidas.
    
    O tamanho de hash 8 é ótimo para achar imagens que são só um pouco semelhantes ao original. Com um maior conjunto de imagens (>1000), isso produzirá muitos falsos positivos, então recomendo usar um tamanho de hash maior neste caso.
    
    16 é o tamanho de hash padrão e um bom compromisso entre achar até mesmo imagens pouco semelhantes e ter poucas colisões de hash.
    
    Hashes 32 e 64 só acham imagens muito semelhantes, mas quase não devem ter falsos positivos (talvez, exceto algumas imagens com o canal alfa).
image_resize_filter_tooltip =
    Para computar o hash da imagem, a biblioteca deve primeiro redimensioná-la.
    
    Dependendo do algoritmo escolhido, a imagem resultante usada para calcular o hash parecerá um pouco diferente.
    
    O algoritmo mais rápido a ser usado, mas também o que dá os piores resultados, é o Mais Próximo. Ele é ativado por padrão, pois com o tamanho de hash 16x16, a qualidade menor não é realmente visível.
    
    Com o tamanho de hash 8x8, recomenda-se usar um algoritmo diferente do Mais Próximo para ter melhores grupos de imagens.
image_hash_alg_tooltip =
    Os usuários podem escolher entre um dos muitos algoritmos de cálculo do hash.
    
    Cada um tem pontos fortes e fracos e por vezes darão resultados melhores e por vezes piores para imagens diferentes.
    
    Logo, para determinar o melhor para você, são precisos testes manuais.
big_files_mode_combobox_tooltip = Permite a busca de arquivos menores/maiores
big_files_mode_label = Arquivos verificados
big_files_mode_smallest_combo_box = O Menor
big_files_mode_biggest_combo_box = O Maior
main_notebook_duplicates = Arquivos Duplicados
main_notebook_empty_directories = Diretórios Vazios
main_notebook_big_files = Arquivos Grandes
main_notebook_empty_files = Arquivos Vazios
main_notebook_temporary = Arquivos Temporários
main_notebook_similar_images = Imagens Semelhantes
main_notebook_similar_videos = Vídeos Similares
main_notebook_same_music = Músicas Duplicadas
main_notebook_symlinks = Ligações Simbólicas Inválidas
main_notebook_broken_files = Arquivos Quebrados
main_notebook_bad_extensions = Extensões Inválidas
main_tree_view_column_file_name = Nome do arquivo
main_tree_view_column_folder_name = Nome da Pasta
main_tree_view_column_path = Caminho
main_tree_view_column_modification = Data de Modificação
main_tree_view_column_size = Tamanho
main_tree_view_column_similarity = Similaridade
main_tree_view_column_dimensions = Tamanho
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = Ano
main_tree_view_column_bitrate = Taxa de Bits
main_tree_view_column_length = Comprimento
main_tree_view_column_genre = Género
main_tree_view_column_symlink_file_name = Nome do Arquivo de Ligação Simbólica
main_tree_view_column_symlink_folder = Pasta da Ligação Simbólica
main_tree_view_column_destination_path = Caminho de Destino
main_tree_view_column_type_of_error = Tipo de Erro
main_tree_view_column_current_extension = Extensão Atual
main_tree_view_column_proper_extensions = Extensão Adequada
main_label_check_method = Método de verificação
main_label_hash_type = Tipo de hash
main_label_hash_size = Tamanho do hash
main_label_size_bytes = Tamanho (bytes)
main_label_min_size = Mínimo
main_label_max_size = Máximo
main_label_shown_files = Número de arquivos exibidos
main_label_resize_algorithm = Redimensionar algoritmo
main_label_similarity = Similaridade{ " " }
main_check_box_broken_files_audio = Áudio
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Arquivar
main_check_box_broken_files_image = Imagem
check_button_general_same_size = Ignorar do mesmo tamanho
check_button_general_same_size_tooltip = Ignorar arquivos com tamanho idêntico nos resultados — geralmente estes são duplicatas 1:1
main_label_size_bytes_tooltip = Tamanho dos arquivos usados na verificação
# Upper window
upper_tree_view_included_folder_column_title = Pastas para Buscar
upper_tree_view_included_reference_column_title = Pastas de Referência
upper_recursive_button = Recursiva
upper_recursive_button_tooltip = Se selecionado, buscar também arquivos que não são postos diretamente nas pastas escolhidas.
upper_manual_add_included_button = Adicionar Manual
upper_add_included_button = Adicionar
upper_remove_included_button = Excluir
upper_manual_add_excluded_button = Adicionar Manual
upper_add_excluded_button = Adicionar
upper_remove_excluded_button = Excluir
upper_manual_add_included_button_tooltip =
    Adicionar o nome do diretório à mão.
    
    Para adicionar vários caminhos de uma vez, separe-os por ;
    
    /home/roman;/home/rozkaz adicionará dois diretórios /home/roman e /home/rozkaz
upper_add_included_button_tooltip = Adicionar novo diretório à busca.
upper_remove_included_button_tooltip = Excluir diretório da busca.
upper_manual_add_excluded_button_tooltip =
    Adicionar o nome de diretório excluído à mão.
    
    Para adicionar vários caminhos de uma vez, separe-os por ;
    
    /home/roman;/home/krokiet adicionará dois diretórios /home/roman e /home/keokiet
upper_add_excluded_button_tooltip = Adicionar diretório a ser excluído na busca.
upper_remove_excluded_button_tooltip = Excluir diretório da exclusão.
upper_notebook_items_configuration = Configuração dos Itens
upper_notebook_excluded_directories = Diretórios Excluídos
upper_notebook_included_directories = Diretórios Incluídos
upper_allowed_extensions_tooltip =
    Extensões permitidas devem ser separadas por vírgulas (por padrão todas estão disponíveis).
    
    Os seguintes Macros, que adicionam várias extensões de uma só vez, também estão disponíveis: IMAGE, VIDEO, MUSIC, TEXT.
    
    Exemplo de uso ".exe, IMAGE, VIDEO, .rar, 7z" — isto significa que as imagens (ex., jpg, png), vídeos (ex., avi, mp4), exe, rar e arquivos 7z serão escaneados.
upper_excluded_extensions_tooltip =
    Lista de arquivos desabilitados que serão ignorados na verificação.
    
    Ao usar extensões permitidas e desativadas, este tem maior prioridade, então o arquivo não será marcado.
upper_excluded_items_tooltip =
    Itens excluídos devem conter * curinga e devem ser separados por vírgulas.
    Isto é mais lento do que a exclusão de diretórios, logo, use-o com cuidado.
upper_excluded_items = Itens excluídos:
upper_allowed_extensions = Extensões permitidas:
upper_excluded_extensions = Extensões desabilitadas:
# Popovers
popover_select_all = Selecionar todos
popover_unselect_all = Desmarcar todos
popover_reverse = Seleção inversa
popover_select_all_except_oldest = Selecionar todos, exceto os mais antigos
popover_select_all_except_newest = Selecionar todos, exceto os mais recentes
popover_select_one_oldest = Selecionar um mais antigo
popover_select_one_newest = Selecionar um mais recente
popover_select_custom = Selecionar um customizado
popover_unselect_custom = Desmarcar customizado
popover_select_all_images_except_biggest = Selecionar tudo, exceto o maior
popover_select_all_images_except_smallest = Selecionar tudo, exceto o menor
popover_custom_path_check_button_entry_tooltip =
    Selecionar registros por caminho.
    
    Exemplo de uso:
    /home/pimpek/rzecz.txt pode ser achado com /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Selecionar registros por nomes de arquivos.
    
    Exemplo de uso:
    /usr/ping/pong.txt pode ser achado com *ong*
popover_custom_regex_check_button_entry_tooltip =
    Selecionar registros por Regex especificada.
    
    Com este modo, o texto buscado é o caminho com o nome.
    
    Exemplo de uso:
    /usr/bin/ziemniak.txt pode ser achado com /ziem[a-z]+
    
    Ele usa a implementação regex padrão do Rust. Você pode ler mais sobre isso aqui: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Ativa a deteção sensível a maiúsculas e minúsculas.
    
    Quando desativado, /home/* acha ambos /HoMe/roman e /home/roman.
popover_custom_not_all_check_button_tooltip =
    Impede a seleção de todo registro no grupo.
    
    Isto está ativado por padrão, pois na maioria das situações, você não quer apagar ambos arquivos originais e duplicados, mas quer deixar ao menos um arquivo.
    
    AVISO: Esta configuração não funciona se você já selecionou manualmente todos os resultados num grupo.
popover_custom_regex_path_label = Caminho
popover_custom_regex_name_label = Nome
popover_custom_regex_regex_label = Caminho da Regex + nome
popover_custom_case_sensitive_check_button = Sensível a maiúsculas e minúsculas
popover_custom_all_in_group_label = Não selecionar todo registro no grupo
popover_custom_mode_unselect = Desmarcar customizado
popover_custom_mode_select = Selecionar customizado
popover_sort_file_name = Nome do arquivo
popover_sort_folder_name = Nome da pasta
popover_sort_full_name = Nome completo
popover_sort_size = Tamanho
popover_sort_selection = Seleção
popover_invalid_regex = Regex inválido
popover_valid_regex = Expressão regular é válida
# Bottom buttons
bottom_search_button = Buscar
bottom_select_button = Selecionar
bottom_delete_button = Excluir
bottom_save_button = Guardar
bottom_symlink_button = Ligação simbólica
bottom_hardlink_button = Ligação hardlink
bottom_move_button = Mover
bottom_sort_button = Ordenar
bottom_search_button_tooltip = Iniciar busca
bottom_select_button_tooltip = Selecionar registros. Só arquivos/diretórios selecionados podem ser processados posteriormente.
bottom_delete_button_tooltip = Excluir arquivos/diretórios selecionados.
bottom_save_button_tooltip = Guardar dados sobre a busca em arquivo
bottom_symlink_button_tooltip =
    Criar ligações simbólicas. Só funciona quando ao menos dois resultados num grupo são selecionados.
    O primeiro é inalterado, e no segundo e mais tarde é feita a ligação simbólica para o primeiro.
bottom_hardlink_button_tooltip =
    Criar ligações hardlinks.
    Só funciona quando ao menos dois resultados num grupo são selecionados.
    O primeiro é inalterado, e no segundo e posterior é feito o hardlink ao primeiro.
bottom_hardlink_button_not_available_tooltip =
    Criar ligações hardlinks.
    O botão está desativado, pois ligações hardlinks não podem ser criadas.
    Hardlinks só funcionam com privilégios de administrador no Windows, logo, certifique-se de executar o aplicativo como administrador.
    Se o aplicativo já funciona com tais privilégios, verifique se há questões semelhantes no GitHub.
bottom_move_button_tooltip =
    Move arquivos para o diretório escolhido.
    Ele copia todos os arquivos para o diretório sem preservar a árvore de diretório.
    Ao tentar mover dois arquivos com nome idêntico para o diretório, a segunda falhará e exibirá um erro.
bottom_sort_button_tooltip = Ordena arquivos/pastas de acordo com o método selecionado.
bottom_show_errors_tooltip = Exibir/ocultar painel de texto inferior.
bottom_show_upper_notebook_tooltip = Exibir/ocultar painel superior do caderno.
# Progress Window
progress_stop_button = Parar
progress_stop_additional_message = Parada pedida
# About Window
about_repository_button_tooltip = Link para a página do repositório com o código-fonte.
about_donation_button_tooltip = Link para a página de doação.
about_instruction_button_tooltip = Link para a página de instrução.
about_translation_button_tooltip = Link para a página do Crowdin com traduções do aplicativo. Oficialmente, polonês e inglês são suportados.
about_repository_button = Repositório
about_donation_button = Doação
about_instruction_button = Instrução
about_translation_button = Tradução
# Header
header_setting_button_tooltip = Abre diálogo de configurações.
header_about_button_tooltip = Abre diálogo com informações sobre o aplicativo.

# Settings


## General

settings_number_of_threads = Número de threads usadas
settings_number_of_threads_tooltip = Numero de thread usadas. Zero significa que toda thread disponível será usada.
settings_use_rust_preview = Usar bibliotecas externas em vez de gtk para carregar pré-visualizações
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = Você tem de reiniciar o aplicativo para aplicar as configurações!
settings_ignore_other_filesystems = Ignorar outros sistemas de arquivos (só Linux)
settings_ignore_other_filesystems_tooltip =
    Ignora arquivos que não estão no mesmo sistema de arquivos que os diretórios buscados.
    
    Funciona como a opção -xdev no comando find no Linux
settings_save_at_exit_button_tooltip = Guardar configuração em arquivo ao fechar o aplicativo.
settings_load_at_start_button_tooltip =
    Carregar configuração do arquivo ao abrir aplicativo.
    
    Se não estiver ativado, as configurações padrão serão usadas.
settings_confirm_deletion_button_tooltip = Exibir diálogo de confirmação ao clicar no botão excluir.
settings_confirm_link_button_tooltip = Exibir diálogo de confirmação ao clicar no botão de ligação hardlink/simbólica.
settings_confirm_group_deletion_button_tooltip = Exibir caixa de diálogo de aviso ao tentar excluir todo registro do grupo.
settings_show_text_view_button_tooltip = Exibir painel de texto na parte inferior da interface do usuário.
settings_use_cache_button_tooltip = Usar cache de arquivos.
settings_save_also_as_json_button_tooltip = Salvar cache no formato JSON (legível por humanos). É possível modificar o seu conteúdo. O cache deste arquivo será lido automaticamente pelo aplicativo se o cache de formato binário (com extensão bin) estiver faltando.
settings_use_trash_button_tooltip = Move arquivos para a lixeira em vez de os excluir para sempre.
settings_language_label_tooltip = Idioma para a interface de usuário.
settings_save_at_exit_button = Guardar configuração ao fechar o aplicativo
settings_load_at_start_button = Carregar configuração ao abrir o aplicativo
settings_confirm_deletion_button = Exibir diálogo de confirmação ao excluir qualquer arquivo
settings_confirm_link_button = Exibir a caixa de diálogo de confirmação ao fazer a ligação hardlink/simbólica de qualquer arquivo
settings_confirm_group_deletion_button = Exibir diálogo de confirmação ao apagar todo arquivo no grupo
settings_show_text_view_button = Exibir painel de texto inferior
settings_use_cache_button = Usar cache
settings_save_also_as_json_button = Também guardar o cache como arquivo JSON
settings_use_trash_button = Mover os arquivos excluídos para a lixeira
settings_language_label = Idioma
settings_multiple_delete_outdated_cache_checkbutton = Excluir entradas de cache desatualizadas automaticamente
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Excluir resultados de cache desatualizados que apontam para arquivos inexistentes.
    
    Quando ativado, o aplicativo garante que ao carregar os registros, todos os registros apontem para arquivos válidos (aqueles com problemas são ignorados).
    
    Desativar isto ajudará ao escanear arquivos em unidades externas, então as entradas de cache sobre eles não serão removidas na próxima verificação.
    
    No caso de ter centenas de milhares de registros no cache, é sugerido ativar isto, o que acelerará o carregamento/armazenamento de cache/salvamento no início/fim do escaneamento.
settings_notebook_general = Geral
settings_notebook_duplicates = Duplicatas
settings_notebook_images = Imagens Semelhantes
settings_notebook_videos = Vídeo Semelhante

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Exibe pré-visualização no lado direito (ao selecionar um arquivo de imagem).
settings_multiple_image_preview_checkbutton = Exibir pré-visualização da imagem
settings_multiple_clear_cache_button_tooltip =
    Limpar manualmente o cache de entradas desatualizadas.
    Isto só deve ser usado se a limpeza automática houver sido desativada.
settings_multiple_clear_cache_button = Remover resultados desatualizados do cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Oculta todos os arquivos, exceto um, se todos apontarem para os mesmos dados (são ligados por hardlink).
    
    Exemplo: No caso de existirem (em disco) sete arquivos que são vinculados por hardlink a dados específicos e um arquivo diferente com os mesmos dados, mas um inode diferente, em seguida no achador de duplicatas, só um arquivo único e um arquivo dos que foi feita a ligação hardlink serão exibidos.
settings_duplicates_minimal_size_entry_tooltip =
    Definir o tamanho mínimo do arquivo que será armazenado em cache.
    
    Escolher um valor menor gerará mais registros. Isto acelerará a busca, mas diminuirá o carregamento/armazenamento do cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Permite o cache de pré-hash (um hash calculado a partir duma pequena parte do arquivo) que permite a demissão de resultados não duplicados anteriormente.
    
    Está desativado por padrão, pois pode causar lentidões nalguns casos.
    
    É altamente recomendado o usar ao escanear centenas de milhares ou milhões de arquivos, pois pode acelerar a pesquisa em várias vezes.
settings_duplicates_prehash_minimal_entry_tooltip = Tamanho mínimo da entrada em cache.
settings_duplicates_hide_hard_link_button = Ocultar ligações hardlink (só Linux e macOS)
settings_duplicates_prehash_checkbutton = Usar cache de pré-hash
settings_duplicates_minimal_size_cache_label = Tamanho mínimo dos arquivos (em bytes) guardados no cache
settings_duplicates_minimal_size_cache_prehash_label = Tamanho mínimo dos arquivos (em bytes) guardados no cache de pré-hash

## Saving/Loading settings

settings_saving_button_tooltip = Guardar as configurações atuais em arquivo.
settings_loading_button_tooltip = Carregar configurações de arquivo e substituir a configuração atual por elas.
settings_reset_button_tooltip = Redefinir a configuração atual para a padrão.
settings_saving_button = Guardar configuração
settings_loading_button = Carregar configuração
settings_reset_button = Redefinir configuração

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Abre o diretório onde os arquivos txt são armazenados.
    
    Modificar os arquivos de cache pode fazer com que resultados inválidos sejam exibidos. Porém, modificar o caminho pode economizar tempo ao mover uma grande quantidade de arquivos para um local diferente.
    
    Você pode copiar esses arquivos entre computadores para economizar tempo em outra verficação de arquivos (claro, se eles tiverem uma estrutura de diretórios semelhante).
    
    No caso de problemas com o cache, esses arquivos podem ser removidos. O aplicativo os regenerará automaticamente.
settings_folder_settings_open_tooltip =
    Abre o diretório onde a configuração do Czkawka está armazenada.
    
    AVISO: Modificar manualmente a configuração pode quebrar seu fluxo de trabalho.
settings_folder_cache_open = Abrir diretório do cache
settings_folder_settings_open = Abrir diretório de configurações
# Compute results
compute_stopped_by_user = A busca foi parada pelo usuário
compute_found_duplicates_hash_size = Achadas { $number_files } duplicatas em { $number_groups } grupos que levaram { $size }
compute_found_duplicates_name = Achadas { $number_files } duplicatas em { $number_groups } grupos
compute_found_empty_folders = Achados { $number_files } diretórios vazios
compute_found_empty_files = Achados { $number_files } arquivos vazios
compute_found_big_files = Achados { $number_files } arquivos grandes
compute_found_temporary_files = Achados { $number_files } arquivos temporários
compute_found_images = Achadas { $number_files } imagens similares em { $number_groups } grupos
compute_found_videos = Achados { $number_files } vídeos similares em { $number_groups } grupos
compute_found_music = Achados { $number_files } arquivos de música similares em { $number_groups } grupos
compute_found_invalid_symlinks = Achadas { $number_files } ligações simbólicas inválidas
compute_found_broken_files = Achados { $number_files } arquivos quebrados
compute_found_bad_extensions = Achados { $number_files } arquivos com extensões inválidas
# Progress window
progress_scanning_general_file = Escaneando { $file_number } arquivo
progress_scanning_extension_of_files = Verificando extensão de { $file_checked }/{ $all_files } de arquivo
progress_scanning_broken_files = Verificando { $file_checked }/{ $all_files } arquivo
progress_scanning_video = Hash de { $file_checked }/{ $all_files } vídeo
progress_scanning_image = Hash de { $file_checked }/{ $all_files } imagem
progress_comparing_image_hashes = Comparando de { $file_checked }/{ $all_files } hash de imagem
progress_scanning_music_tags_end = Comparando etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_tags = Lendo etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_content_end = Comparação de impressão digital de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_content = Calculando impressão digital de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_empty_folders = Verificando { $folder_number } diretório
progress_scanning_size = Verificando tamanho de { $file_number } arquivo
progress_scanning_size_name = Verificando nome e tamanho de { $file_number } arquivo
progress_scanning_name = Verificando nome de { $file_number } arquivo
progress_analyzed_partial_hash = Hash parcial analisado de { $file_checked }/{ $all_files } arquivos
progress_analyzed_full_hash = Hash completo analisado de { $file_checked }/{ $all_files } arquivos
progress_prehash_cache_loading = Carregando cache de pré-hash
progress_prehash_cache_saving = Salvando cache pré-hash
progress_hash_cache_loading = Carregando cache de hash
progress_hash_cache_saving = Salvando cache de hash
progress_cache_loading = Carregando cache
progress_cache_saving = Salvando cache
progress_current_stage = Estágio atual:{ " " }
progress_all_stages = Todo estágio:{ " " }
# Saving loading 
saving_loading_saving_success = Configuração guardada no arquivo { $name }.
saving_loading_saving_failure = Falha ao guardar dados de configuração no arquivo { $name }.
saving_loading_reset_configuration = A configuração atual foi limpa.
saving_loading_loading_success = Configuração de aplicativo devidamente carregada.
saving_loading_invalid_string = Para a chave "{ $key }" foi achado um resultado inválido — "{ $result }" que não é uma string.
saving_loading_invalid_int = Para a chave "{ $key }" foi achado um resultado inválido — "{ $result }" que não é um inteiro.
saving_loading_invalid_bool = Para a chave "{ $key }" foi achado um resultado inválido — "{ $result }" que não é um bool.
saving_loading_decode_problem_bool = Falha ao decodificar o bool da chave "{ $key }" achado "{ $result }", mas valores permitidos são 0, 1, verdadeiro ou falso.
saving_loading_saving_same_keys = Tentando guardar a configuração com chave duplicada "{ $key }".
saving_loading_failed_to_get_home_directory = Falha ao obter o diretório inicial para abrir/guardar o arquivo de configuração.
saving_loading_folder_config_instead_file = Não se pode criar ou abrir o arquivo de configuração no caminho "{ $path }", pois já existe um diretório.
saving_loading_failed_to_create_configuration_folder = Falha ao criar o diretório de configuração "{ $path }", razão "{ $reason }".
saving_loading_failed_to_create_config_file = Falha ao criar o arquivo de configuração "{ $path }", razão "{ $reason }".
saving_loading_failed_to_read_config_file = Não se pode carregar a configuração de "{ $path }", pois ela não existe ou não é um arquivo.
saving_loading_failed_to_read_data_from_file = Não se pode ler dados do arquivo "{ $path }", razão "{ $reason }".
saving_loading_orphan_data = Dados órfãos achados "{ $data }" na linha "{ $line }".
saving_loading_not_valid = A configuração "{ $data }" não existe na versão atual do aplicativo.
# Invalid symlinks
invalid_symlink_infinite_recursion = Recursão infinita
invalid_symlink_non_existent_destination = Arquivo de destino não existe
# Other
selected_all_reference_folders = Não é possível iniciar a busca quando todo diretório está definido como pasta de referência
searching_for_data = Buscando dados, pode demorar um pouco, aguarde...
text_view_messages = MENSAGENS
text_view_warnings = AVISOS
text_view_errors = ERROS
about_window_motto = Este programa é gratuito para o uso e sempre será.
# Various dialog
dialogs_ask_next_time = Perguntar na próxima vez
delete_file_failed = Falha ao excluir o arquivo { $name }, razão { $reason }
delete_title_dialog = Confirmação de exclusão
delete_question_label = Tem certeza de que quer excluir arquivos?
delete_all_files_in_group_title = Confirmação da exclusão de todo arquivo no grupo
delete_all_files_in_group_label1 = Em alguns grupos todo registro está selecionado.
delete_all_files_in_group_label2 = Tem certeza de que quer os excluir?
delete_folder_failed = Falha ao excluir a pasta { $dir }, pois a pasta não existe, você não tem permissão ou a pasta não está vazia.
delete_items_label = { $items } arquivos serão excluídos.
delete_items_groups_label = { $items } arquivos de { $groups } grupos serão excluídos.
hardlink_failed = Falha na ligação
hard_sym_invalid_selection_title_dialog = Seleção inválida com alguns grupos
hard_sym_invalid_selection_label_1 = Em alguns grupos só há um registro selecionado e ele será ignorado.
hard_sym_invalid_selection_label_2 = Para poder ligar estes arquivos, ao menos dois resultados no grupo têm de ser selecionados.
hard_sym_invalid_selection_label_3 = O primeiro no grupo é reconhecido como original e não é mudado, mas o segundo e posterior são modificados.
hard_sym_link_title_dialog = Link de confirmação
hard_sym_link_label = Tem certeza de que quer vincular estes arquivos?
move_folder_failed = Falha ao mover a pasta { $name }, razão { $reason }
move_file_failed = Falha ao mover o arquivo { $name }, razão { $reason }
move_files_title_dialog = Escolha a pasta para a qual você quer mover arquivos duplicados
move_files_choose_more_than_1_path = Só um caminho pode ser selecionado para poder copiar seus arquivos duplicados, selecionado { $path_number }.
move_stats = Devidamente movidos { $num_files }/{ $all_files } itens
save_results_to_file = Resultados salvos tanto para arquivos txt quanto json na pasta { $name }.
search_not_choosing_any_music = ERRO: Você deve selecionar ao menos uma caixa de seleção com tipos de busca de música.
search_not_choosing_any_broken_files = ERRO: Você deve selecionar ao menos uma caixa de seleção com tipo de arquivos quebrados.
include_folders_dialog_title = Pastas para incluir
exclude_folders_dialog_title = Pastas para excluir
include_manually_directories_dialog_title = Adicionar diretório manualmente
cache_properly_cleared = Cache devidamente limpo
cache_clear_duplicates_title = Limpando o cache de duplicatas
cache_clear_similar_images_title = Limpando o cache de imagens similares
cache_clear_similar_videos_title = Limpando o cache de vídeos similares
cache_clear_message_label_1 = Deseja limpar o cache de entradas desatualizadas?
cache_clear_message_label_2 = Esta operação removerá toda entrada de cache que aponta para arquivos inválidos.
cache_clear_message_label_3 = Isto pode acelerar um pouco o carregamento/salvamento para o cache.
cache_clear_message_label_4 = AVISO: A operação removerá todo dado em cache de unidades externas desconectadas. Logo, cada hash terá de ser regenerado.
# Show preview
preview_image_resize_failure = Falha ao redimensionar a imagem { $name }.
preview_image_opening_failure = Falha ao abrir a imagem { $name }, razão { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupo { $current_group }/{ $all_groups } ({ $images_in_group } imagens)
compare_move_left_button = L
compare_move_right_button = R
