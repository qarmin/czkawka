# Window titles
window_settings_title = Definições
window_main_title = Czkawka (Soluço)
window_progress_title = A verificar
window_compare_images = Comparar Imagens
# General
general_ok_button = Certo
general_close_button = Fechar
# Krokiet info dialog
krokiet_info_title = Aviso de desaprovação
krokiet_info_message =
    Czkawka GTK 12.0 é a versão final. Não estão planeadas atualizações, recursos ou correções de bugs adicionais.
    
    A maioria dos recursos do Czkawka GTK está disponível no Krokiet, geralmente de uma forma mais simples, rápida e estável. O Krokiet também adiciona novos recursos e melhorias que não eram possíveis na versão GTK.
    
    Se ainda estiver a usar o Czkawka GTK, a transição para o Krokiet deve ser fácil, já que ele possui uma interface semelhante, menos dependências e melhor suporte para diferentes plataformas.
    
    P.S.: Esta mensagem deve aparecer apenas uma vez. Se ela aparecer novamente, defina a variável de ambiente CZKAWKA_DONT_ANNOY_ME para qualquer valor não vazio.
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_year_checkbox = Ano
music_bitrate_checkbox = Taxa de Bits
music_genre_checkbox = Género
music_length_checkbox = Comprimento
music_comparison_checkbox = Comparação Aproximada
music_checking_by_tags = Etiquetas
music_checking_by_content = Conteúdo
same_music_seconds_label = Duração mínima de segundos do fragmento
same_music_similarity_label = Diferença máxima
music_compare_only_in_title_group = Comparar dentro de grupos de títulos similares
music_compare_only_in_title_group_tooltip =
    Quando ativado, os ficheiros são agrupados por título e então comparados entre si.
    
    Com 10000 ficheiros, em vez de quase 100 milhões de comparações, haverá geralmente cerca de 20 000.
same_music_tooltip =
    Buscar por ficheiros de música semelhantes por seu conteúdo pode ser configurado definindo:
    
    - O tempo mínimo de fragmento após o qual os ficheiros de música podem ser identificados como semelhantes
    - A diferença máxima entre dois fragmentos testados
    
    A chave para bons resultados é achar combinações sensíveis desses parâmetros, para fornecido.
    
    Definir o tempo mínimo para 5s e a diferença máxima para 1.0 buscará fragmentos quase iguais nos ficheiros.
    Um tempo de 20s e uma diferença máxima de 6.0, por outro lado, funciona bem para achar versões remixes/ao vivo, etc.
    
    Por padrão, cada ficheiro de música é comparado entre si, e isso pode levar muito tempo para testar muitos ficheiros, logo, é geralmente melhor usar pastas de referência e especificar quais ficheiros devem ser comparados entre si (com a mesma quantidade de ficheiros, comparar impressões digitais será pelo menos 4x mais rápido do que sem pastas de referência).
music_comparison_checkbox_tooltip =
    Esta opção pesquisa ficheiros de música semelhantes usando IA, que utiliza aprendizagem automática para remover parênteses de uma frase. Por exemplo, com esta opção ativada, os ficheiros em questão serão considerados duplicados:
    
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
    Por ora, o Czkawka oferece três tipos de métodos para encontrar duplicados:
    
    Nome - Encontra ficheiros que têm o mesmo nome.
    
    Tamanho - Encontra ficheiros que têm o mesmo tamanho.
    
    Hash - Encontra ficheiros que têm o mesmo conteúdo. Este modo calcula o hash do ficheiro e depois compara este hash para encontrar duplicados. Este modo é a forma mais segura de encontrar duplicados. A aplicação utiliza muito o cache, pelo que a segunda e seguintes verificações dos mesmos dados deverão ser muito mais rápidas do que a primeira.
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
    Os utilizadores podem escolher entre um dos muitos algoritmos de cálculo do hash.
    
    Cada um tem pontos fortes e fracos e por vezes darão resultados melhores e por vezes piores para imagens diferentes.
    
    Logo, para determinar o melhor para si, são precisos testes manuais.
image_geometric_invariance_tooltip = Também comparar variantes espelhadas/viradas e opcionalmente rotacionadas de cada imagem. Isto melhora a correspondência, mas aumenta o tempo de hashing.
big_files_mode_combobox_tooltip = Permite a busca de ficheiros menores/maiores
big_files_mode_label = Ficheiros verificados
big_files_mode_smallest_combo_box = O Menor
big_files_mode_biggest_combo_box = O Maior
main_notebook_duplicates = Arquivos Duplicados
main_notebook_empty_directories = Diretórios Vazios
main_notebook_big_files = Ficheiros Grandes
main_notebook_empty_files = Ficheiros Vazios
main_notebook_temporary = Ficheiros Temporários
main_notebook_similar_images = Imagens Semelhantes
main_notebook_similar_videos = Vídeos Semelhantes
main_notebook_same_music = Músicas Duplicadas
main_notebook_symlinks = Ligações Simbólicas Inválidas
main_notebook_broken_files = Ficheiros Corrompidos
main_notebook_bad_extensions = Extensões Inválidas
main_tree_view_column_file_name = Nome do ficheiro
main_tree_view_column_folder_name = Nome da Pasta
main_tree_view_column_path = Caminho
main_tree_view_column_modification = Data de Modificação
main_tree_view_column_size = Tamanho
main_tree_view_column_similarity = Semelhança
main_tree_view_column_dimensions = Dimensões
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = Ano
main_tree_view_column_bitrate = Taxa de Bits
main_tree_view_column_length = Comprimento
main_tree_view_column_genre = Género
main_tree_view_column_symlink_file_name = Nome do Ficheiro da Ligação Simbólica
main_tree_view_column_symlink_folder = Pasta da Ligação Simbólica
main_tree_view_column_destination_path = Caminho de Destino
main_tree_view_column_type_of_error = Tipo de Erro
main_tree_view_column_current_extension = Extensão Atual
main_tree_view_column_proper_extensions = Extensão Adequada
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Codificador
main_label_check_method = Método de verificação
main_label_hash_type = Tipo de hash
main_label_hash_size = Tamanho do hash
main_label_geometric_invariance = Invariância geométrica
main_label_size_bytes = Tamanho (bytes)
main_label_min_size = Mínimo
main_label_max_size = Máximo
main_label_shown_files = Número de ficheiros mostrados
main_label_resize_algorithm = Algoritmo de redimensionamento
main_label_similarity = Semelhança{ " " }
main_check_box_broken_files_audio = Áudio
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Ficheiro
main_check_box_broken_files_image = Imagem
main_check_box_broken_files_video = Vídeo
main_check_box_broken_files_video_tooltip = Usa ffmpeg/ffprobe para validar ficheiros de vídeo. É bastante lento e pode detetar erros insignificantes mesmo que o ficheiro seja reproduzido corretamente.
check_button_general_same_size = Ignorar ficheiros de tamanho idêntico
check_button_general_same_size_tooltip = Ignorar ficheiros com tamanho idêntico nos resultados - geralmente estes são duplicatas 1:1
main_label_size_bytes_tooltip = Tamanho dos ficheiros utilizados na verificação
# Upper window
upper_tree_view_included_folder_column_title = Pastas para Pesquisar
upper_tree_view_included_reference_column_title = Pastas de Referência
upper_recursive_button = Recursiva
upper_recursive_button_tooltip = Se selecionado, pesquisa também ficheiros que não estão diretamente nas pastas escolhidas.
upper_manual_add_included_button = Adicionar Manual
upper_add_included_button = Adicionar
upper_remove_included_button = Remover
upper_manual_add_excluded_button = Adicionar Manual
upper_add_excluded_button = Adicionar
upper_remove_excluded_button = Remover
upper_manual_add_included_button_tooltip =
    Adicionar o nome do diretório à mão.
    
    Para adicionar vários caminhos de uma vez, separe-os por ;
    
    /home/roman;/home/rozkaz adicionará dois diretórios /home/roman e /home/rozkaz
upper_add_included_button_tooltip = Adicionar novo diretório à pesquisa.
upper_remove_included_button_tooltip = Eliminar diretório da pesquisa.
upper_manual_add_excluded_button_tooltip =
    Adicionar o nome de diretório excluído à mão.
    
    Para adicionar vários caminhos de uma vez, separe-os por ;
    
    /home/roman;/home/krokiet adicionará dois diretórios /home/roman e /home/keokiet
upper_add_excluded_button_tooltip = Adicionar diretório a ser excluído da pesquisa.
upper_remove_excluded_button_tooltip = Remover diretório da lista de exclusões.
upper_notebook_items_configuration = Configuração dos Itens
upper_notebook_excluded_directories = Caminhos Excluídos
upper_notebook_included_directories = Caminhos Incluídos
upper_allowed_extensions_tooltip = ...exe, rar e ficheiros 7z serão verificados.
upper_excluded_extensions_tooltip =
    Lista de ficheiros desabilitados que serão ignorados na verificação.
    
    Ao usar extensões permitidas e desativadas, este tem maior prioridade, então o ficheiro não será marcado.
upper_excluded_items_tooltip =
    Lista de ficheiros desativados que serão ignorados na verificação.
    
    Ao usar extensões permitidas e desativadas, esta última tem maior prioridade, pelo que o ficheiro não será verificado.
upper_excluded_items = Itens excluídos:
upper_allowed_extensions = Extensões permitidas:
upper_excluded_extensions = Extensões desabilitadas:
# Popovers
popover_select_all = Selecionar todos
popover_unselect_all = Desmarcar todos
popover_reverse = Seleção inversa
popover_select_all_except_shortest_path = Selecionar todos, exceto o caminho mais curto
popover_select_all_except_longest_path = Selecionar todos, exceto o caminho mais longo
popover_select_all_except_oldest = Selecionar todos, exceto os mais antigos
popover_select_all_except_newest = Selecionar todos, exceto os mais recentes
popover_select_one_oldest = Selecionar um mais antigo
popover_select_one_newest = Selecionar um mais recente
popover_select_custom = Selecionar um personalizado
popover_unselect_custom = Desmarcar personalizado
popover_select_all_images_except_biggest = Selecionar todos, exceto o maior
popover_select_all_images_except_smallest = Selecionar todos, exceto o menor
popover_custom_path_check_button_entry_tooltip =
    Selecionar registos por caminho.
    
    Exemplo de uso:
    /home/pimpek/rzecz.txt pode ser achado com /home/pim*
popover_custom_name_check_button_entry_tooltip =
    Selecionar registos por nomes de ficheiros.
    
    Exemplo de uso:
    /usr/ping/pong.txt pode ser encontrado com *ong*
popover_custom_regex_check_button_entry_tooltip = ...Esta opção utiliza a implementação padrão de regex do Rust. Pode ler mais sobre isso aqui: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Ativa a deteção sensível a maiúsculas e minúsculas.
    
    Quando desativado, /home/* acha ambos /HoMe/roman e /home/roman.
popover_custom_not_all_check_button_tooltip =
    Impede a seleção de todos os registos num grupo.
    
    Isto está ativado por padrão, pois na maioria das situações, não se pretende apagar tanto os ficheiros originais como os duplicados, mas sim deixar pelo menos um ficheiro.
    
    AVISO: Esta configuração não funciona se já tiver selecionado manualmente todos os resultados num grupo.
popover_custom_regex_path_label = Caminho
popover_custom_regex_name_label = Nome
popover_custom_regex_regex_label = Regex: Caminho + Nome
popover_custom_case_sensitive_check_button = Sensível a maiúsculas e minúsculas
popover_custom_all_in_group_label = Não selecionar todos os registos no grupo
popover_custom_mode_unselect = Desmarcar personalizado
popover_custom_mode_select = Selecionar personalizado
popover_sort_file_name = Nome do ficheiro
popover_sort_folder_name = Nome da pasta
popover_sort_full_name = Nome completo
popover_sort_size = Tamanho
popover_sort_selection = Seleção
popover_invalid_regex = Expressão regular inválida
popover_valid_regex = Expressão regular é válida
# Bottom buttons
bottom_search_button = Pesquisar
bottom_select_button = Selecionar
bottom_delete_button = Eliminar
bottom_save_button = Guardar
bottom_symlink_button = Ligação simbólica
bottom_hardlink_button = Ligação rígida
bottom_move_button = Mover
bottom_sort_button = Ordenar
bottom_compare_button = Comparar
bottom_search_button_tooltip = Iniciar busca
bottom_select_button_tooltip = Selecionar registos. Só ficheiros/diretórios selecionados podem ser processados posteriormente.
bottom_delete_button_tooltip = Eliminar ficheiros/diretórios selecionados.
bottom_save_button_tooltip = Guardar dados da pesquisa num ficheiro
bottom_symlink_button_tooltip =
    Criar ligações simbólicas. Só funciona quando ao menos dois resultados num grupo são selecionados.
    O primeiro é inalterado, e no segundo e mais tarde é feita a ligação simbólica para o primeiro.
bottom_hardlink_button_tooltip =
    Criar ligações rígidas.
    Só funciona quando pelo menos dois resultados num grupo são selecionados.
    O primeiro fica inalterado, e ao segundo e seguintes é criada uma ligação rígida para o primeiro.
bottom_hardlink_button_not_available_tooltip =
    Criar ligações rígidas.
    O botão está desativado, pois as ligações rígidas não podem ser criadas.
    As ligações rígidas só funcionam com privilégios de administrador no Windows, logo, certifique-se de executar a aplicação como administrador.
    Se a aplicação já funciona com tais privilégios, verifique se há questões semelhantes no GitHub.
bottom_move_button_tooltip =
    Move ficheiros para o diretório escolhido.
    Ele copia todos os ficheiros para o diretório sem preservar a árvore de diretórios.
    Ao tentar mover dois ficheiros com nome idêntico para o diretório, o segundo falhará e mostrará um erro.
bottom_sort_button_tooltip = Ordena ficheiros/pastas de acordo com o método selecionado.
bottom_compare_button_tooltip = Compare as imagens do grupo.
bottom_show_errors_tooltip = Mostrar/ocultar painel de texto inferior.
bottom_show_upper_notebook_tooltip = Mostrar/ocultar o painel superior de separadores.
# Progress Window
progress_stop_button = Parar
progress_stop_additional_message = Parada pedida
# About Window
about_repository_button_tooltip = Link para a página do repositório com o código-fonte.
about_donation_button_tooltip = Link para a página de doação.
about_instruction_button_tooltip = Link para a página de instrução.
about_translation_button_tooltip = Link para a página do Crowdin com as traduções da aplicação. Oficialmente são suportados o polaco e o inglês.
about_repository_button = Repositório
about_donation_button = Doação
about_instruction_button = Instrução
about_translation_button = Tradução
about_other_apps_button = Outras aplicações
about_other_apps_dialog_title = Outras aplicações de qarmin
about_other_apps_open_source_note = Todas as aplicações listadas são gratuitas e de código aberto.
about_other_apps_open_button = Abertas
about_other_apps_szyszka_desc = Renomeador de ficheiros rápido e poderoso.
about_other_apps_mykrut_desc = Gestor de ficheiros Linux simples, rápido e com opinião própria.
about_other_apps_dcmki_viewer_desc = Visualizador DICOM simples.
about_other_apps_video_thumbnailer_desc = Embrulho em torno do gerador de miniaturas de vídeo usado em Czkawka.
about_other_apps_space_finder_desc = Localizador simples dos maiores ficheiros no seu sistema.
about_other_apps_system_info_collector_desc = Coleta uso de RAM/CPU do sistema operacional e o mostra como gráficos.
# Header
header_setting_button_tooltip = Abre o diálogo de definições.
header_about_button_tooltip = Abre diálogo com informações sobre a aplicação.
header_krokiet_button_tooltip = Experimente o Krokiet - a versão nova e melhorada!
# Krokiet promo dialog
krokiet_promo_title = Conheça o Krokiet!
krokiet_promo_message = Transferir Krokiet/Cedinia.
krokiet_promo_link_download = Transferir Krokiet/Cedinia
krokiet_promo_link_project = Página do projeto

# Settings


## General

settings_number_of_threads = Número de threads usadas
settings_number_of_threads_tooltip = Numero de thread usadas. Zero significa que toda thread disponível será usada.
settings_use_rust_preview = Usar bibliotecas externas em vez de gtk para carregar pré-visualizações
settings_use_rust_preview_tooltip =
    A utilização de pré-visualizações com GTK será por vezes mais rápida e suportará mais formatos, mas outras vezes ocorre exatamente o inverso.
    
    Se tiver problemas com o carregamento de pré-visualizações, tente alterar esta configuração.
    
    Em sistemas não-GNU/Linux, é recomendado usar esta opção porque o GTK-Pixbuf nem sempre está disponível lá, então desativar esta opção irá parar as tentativas falhadas de carregar pré-visualizações de algumas imagens.
settings_label_restart = Tem de reiniciar a aplicação para aplicar as definições!
settings_ignore_other_filesystems = Ignorar outros sistemas de ficheiros (só Linux)
settings_ignore_other_filesystems_tooltip =
    Ignora ficheiros que não estão no mesmo sistema de ficheiros que os diretórios buscados.
    
    Funciona como a opção -xdev no comando find no Linux
settings_save_at_exit_button_tooltip = Guardar a configuração em ficheiro ao fechar a aplicação.
settings_load_at_start_button_tooltip =
    Carregar configuração do ficheiro ao abrir a aplicação.
    
    Se não estiver ativado, as definições padrão serão usadas.
settings_confirm_deletion_button_tooltip = Mostrar diálogo de confirmação ao clicar no botão eliminar.
settings_confirm_link_button_tooltip = Mostrar diálogo de confirmação ao clicar no botão de ligação rígida/simbólica.
settings_confirm_group_deletion_button_tooltip = Mostrar caixa de diálogo de aviso ao tentar eliminar todos os registos do grupo.
settings_show_text_view_button_tooltip = Mostrar painel de texto na parte inferior da interface do utilizador.
settings_use_cache_button_tooltip = Usar cache de ficheiros.
settings_save_also_as_json_button_tooltip = Guardar o cache no formato JSON (legível por humanos). É possível modificar o seu conteúdo. O cache deste ficheiro será lido automaticamente pela aplicação se o cache em formato binário (com extensão bin) estiver em falta.
settings_use_trash_button_tooltip = Move ficheiros para o lixo em vez de os eliminar permanentemente.
settings_language_label_tooltip = Idioma para a interface do utilizador.
settings_save_at_exit_button = Guardar configuração ao fechar a aplicação
settings_load_at_start_button = Carregar configuração ao abrir a aplicação
settings_confirm_deletion_button = Mostrar diálogo de confirmação ao eliminar qualquer ficheiro
settings_confirm_link_button = Mostrar a caixa de diálogo de confirmação ao criar a ligação rígida/simbólica de qualquer ficheiro
settings_confirm_group_deletion_button = Mostrar diálogo de confirmação ao apagar todos os ficheiros do grupo
settings_show_text_view_button = Mostrar painel de texto inferior
settings_use_cache_button = Usar cache
settings_save_also_as_json_button = Também guardar o cache como ficheiro JSON
settings_use_trash_button = Mover os ficheiros eliminados para o lixo
settings_language_label = Idioma
settings_multiple_delete_outdated_cache_checkbutton = Eliminar automaticamente as entradas de cache desatualizadas
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Eliminar resultados de cache desatualizados que apontam para ficheiros inexistentes.
    
    Quando ativado, a aplicação garante que, ao carregar os registos, todos apontam para ficheiros válidos (os que têm problemas são ignorados).
    
    Desativar isto ajudará ao verificar ficheiros em unidades externas, para que as entradas de cache sobre eles não sejam removidas na próxima verificação.
    
    No caso de ter centenas de milhares de registos no cache, sugere-se ativar isto, o que acelerará o carregamento/gravação do cache no início/fim da verificação.
settings_notebook_general = Geral
settings_notebook_duplicates = Duplicatas
settings_notebook_images = Imagens Semelhantes
settings_notebook_videos = Vídeo Semelhante

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Mostra a pré-visualização no lado direito (ao selecionar um ficheiro de imagem).
settings_multiple_image_preview_checkbutton = Mostrar pré-visualização da imagem
settings_multiple_clear_cache_button_tooltip =
    Limpar manualmente o cache de entradas desatualizadas.
    Isto só deve ser usado se a limpeza automática houver sido desativada.
settings_multiple_clear_cache_button = Remover resultados desatualizados do cache.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Oculta todos os ficheiros, exceto um, se todos apontarem para os mesmos dados (estiverem ligados por ligação rígida).
    
    Exemplo: no caso de existirem (em disco) sete ficheiros ligados por ligação rígida a dados específicos e um ficheiro diferente com os mesmos dados, mas com um inode diferente, o localizador de duplicados mostrará apenas um ficheiro único e um dos ficheiros ligados por ligação rígida.
settings_duplicates_minimal_size_entry_tooltip =
    Definir o tamanho mínimo do ficheiro que será armazenado em cache.
    
    Escolher um valor menor gerará mais registos. Isto acelerará a pesquisa, mas tornará mais lento o carregamento/gravação do cache.
settings_duplicates_prehash_checkbutton_tooltip =
    Permite o cache de pré-hash (um hash calculado a partir de uma pequena parte do ficheiro) que permite o descarte antecipado de resultados não duplicados.
    
    Está desativado por padrão, pois pode causar lentidão nalguns casos.
    
    É altamente recomendável utilizá-lo ao verificar centenas de milhares ou milhões de ficheiros, pois pode acelerar a pesquisa várias vezes.
settings_duplicates_prehash_minimal_entry_tooltip = Tamanho mínimo da entrada em cache.
settings_duplicates_hide_hard_link_button = Ocultar as ligações rígidas
settings_duplicates_prehash_checkbutton = Usar cache de pré-hash
settings_duplicates_minimal_size_cache_label = Tamanho mínimo dos ficheiros (em bytes) guardados no cache
settings_duplicates_minimal_size_cache_prehash_label = Tamanho mínimo dos ficheiros (em bytes) guardados no cache de pré-hash

## Saving/Loading settings

settings_saving_button_tooltip = Guardar as definições atuais em ficheiro.
settings_loading_button_tooltip = Carregar definições do ficheiro e substituir a configuração atual por elas.
settings_reset_button_tooltip = Redefinir a configuração atual para a padrão.
settings_saving_button = Guardar configuração
settings_loading_button = Carregar configuração
settings_reset_button = Redefinir configuração

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Abre o diretório onde são armazenados os ficheiros txt do cache.
    
    Modificar os ficheiros de cache pode causar a apresentação de resultados inválidos. No entanto, modificar o caminho pode poupar tempo ao mover uma grande quantidade de ficheiros para um local diferente.
    
    É possível copiar estes ficheiros entre computadores para poupar tempo numa nova verificação de ficheiros (claro, se tiverem uma estrutura de diretórios semelhante).
    
    Em caso de problemas com o cache, estes ficheiros podem ser removidos. A aplicação irá regená-los automaticamente.
settings_folder_settings_open_tooltip =
    Abre o diretório onde a configuração do Czkawka está armazenada.
    
    AVISO: Modificar manualmente a configuração pode quebrar seu fluxo de trabalho.
settings_folder_cache_open = Abrir diretório do cache
settings_folder_settings_open = Abrir pasta das definições
# Compute results
compute_stopped_by_user = A pesquisa foi interrompida pelo utilizador
compute_found_duplicates_hash_size = Encontradas { $number_files } duplicatas em { $number_groups } grupos que ocuparam { $size } em { $time }
compute_found_duplicates_name = Encontradas { $number_files } duplicatas em { $number_groups } grupos em { $time }
compute_found_empty_folders = Encontradas pastas { $number_files } vazias em { $time }
compute_found_empty_files = Encontrados { $number_files } ficheiros vazios em { $time }
compute_found_big_files = Encontrados { $number_files } ficheiros grandes em { $time }
compute_found_temporary_files = { $number_files } ficheiros temporários encontrados em { $time }
compute_found_images = Encontradas { $number_files } imagens similares em { $number_groups } grupos em { $time }
compute_found_videos = Encontrados { $number_files } vídeos similares em { $number_groups } grupos em { $time }
compute_found_music = Encontrados { $number_files } ficheiros de música semelhantes em { $number_groups } grupos em { $time }
compute_found_invalid_symlinks = Encontradas { $number_files } ligações simbólicas inválidas em { $time }
compute_found_broken_files = Encontrados { $number_files } ficheiros corrompidos em { $time }
compute_found_bad_extensions = Encontrados { $number_files } ficheiros com extensões inválidas em { $time }
# Progress window
progress_current_stage = Fase atual:{ " " }
progress_all_stages = Todas as fases:{ " " }
# Saving loading 
saving_loading_saving_success = Configuração guardada no ficheiro { $name }.
saving_loading_saving_failure = Falha ao guardar os dados de configuração no ficheiro { $name }, motivo { $reason }.
saving_loading_reset_configuration = A configuração atual foi limpa.
saving_loading_loading_success = Configuração da aplicação devidamente carregada.
saving_loading_no_config_file = Nenhum ficheiro de configuração encontrado, utilizando as definições padrão.
saving_loading_failed_to_create_config_file = Falha ao criar o ficheiro de configuração "{ $path }", razão "{ $reason }".
saving_loading_failed_to_read_config_file = Não se pode carregar a configuração de "{ $path }", pois não existe ou não é um ficheiro.
saving_loading_failed_to_read_data_from_file = Não se pode ler dados do ficheiro "{ $path }", razão "{ $reason }".
# Other
selected_all_reference_folders = Não é possível iniciar a busca quando todo diretório está definido como pasta de referência
searching_for_data = A pesquisar dados, pode demorar um pouco, aguarde...
text_view_messages = MENSAGENS
text_view_warnings = AVISOS
text_view_errors = ERROS
about_window_motto = Este programa é gratuito e sempre o será.
krokiet_new_app = Czkawka está em modo de manutenção, o que significa que apenas erros críticos serão corrigidos e nenhum novo recurso será adicionado. Para novos recursos, por favor, veja a nova aplicação Krokiet, que é mais estável e com desempenho e ainda está em desenvolvimento ativo.
# Various dialog
dialogs_ask_next_time = Perguntar na próxima vez
symlink_failed = Falha ao criar a ligação simbólica de { $name } para { $target }, motivo { $reason }
delete_title_dialog = Confirmação de eliminação
delete_question_label = Tem a certeza de que quer eliminar ficheiros?
delete_all_files_in_group_title = Confirmação da eliminação de todos os ficheiros do grupo
delete_all_files_in_group_label1 = Em alguns grupos todo registo está selecionado.
delete_all_files_in_group_label2 = Tem a certeza de que os quer eliminar?
delete_items_label = { $items } ficheiros serão eliminados.
delete_items_groups_label = { $items } ficheiros de { $groups } grupos serão eliminados.
hardlink_failed = Falha ao criar a ligação rígida de { $name } para { $target }, motivo { $reason }
hard_sym_invalid_selection_title_dialog = Seleção inválida com alguns grupos
hard_sym_invalid_selection_label_1 = Em alguns grupos só há um registo selecionado e este será ignorado.
hard_sym_invalid_selection_label_2 = Para poder ligar estes ficheiros, pelo menos dois resultados no grupo têm de ser selecionados.
hard_sym_invalid_selection_label_3 = O primeiro no grupo é reconhecido como original e não é mudado, mas o segundo e posterior são modificados.
hard_sym_link_title_dialog = Confirmação da ligação
hard_sym_link_label = Tem certeza de que quer vincular estes ficheiros?
move_folder_failed = Falha ao mover a pasta { $name }, razão { $reason }
move_file_failed = Falha ao mover o ficheiro { $name }, razão { $reason }
move_files_title_dialog = Escolha a pasta para a qual quer mover ficheiros duplicados
move_files_choose_more_than_1_path = Só um caminho pode ser selecionado para poder copiar os seus ficheiros duplicados, selecionado { $path_number }.
move_stats = Devidamente movidos { $num_files }/{ $all_files } itens
save_results_to_file = Resultados guardados tanto em ficheiros txt como json na pasta "{ $name }".
search_not_choosing_any_music = ERRO: É necessário selecionar pelo menos uma caixa de seleção com os tipos de música a pesquisar.
search_not_choosing_any_broken_files = ERRO: É necessário selecionar pelo menos uma caixa de seleção com o tipo de ficheiros danificados a verificar.
include_folders_dialog_title = Pastas para incluir
exclude_folders_dialog_title = Pastas para excluir
include_manually_directories_dialog_title = Adicionar diretório manualmente
cache_properly_cleared = Cache devidamente limpo
cache_clear_duplicates_title = Limpando o cache de duplicatas
cache_clear_similar_images_title = Limpando o cache de imagens similares
cache_clear_similar_videos_title = Limpando o cache de vídeos similares
cache_clear_message_label_1 = Deseja limpar o cache de entradas desatualizadas?
cache_clear_message_label_2 = Esta operação removerá todas as entradas de cache que apontam para ficheiros inválidos.
cache_clear_message_label_3 = Isto pode acelerar um pouco o carregamento/salvamento para o cache.
cache_clear_message_label_4 = AVISO: A operação removerá todo dado em cache de unidades externas desconectadas. Logo, cada hash terá de ser regenerado.
# Show preview
preview_image_resize_failure = Falha ao redimensionar a imagem { $name }.
preview_image_opening_failure = Falha ao abrir a imagem { $name }, razão { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grupo { $current_group }/{ $all_groups } ({ $images_in_group } imagens)
compare_move_left_button = L
compare_move_right_button = R
