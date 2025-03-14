# Window titles
window_settings_title = Configurações
window_main_title = Czkawka (Soluço)
window_progress_title = Verificando
window_compare_images = Comparar as imagens
# General
general_ok_button = OK
general_close_button = Fechar
# Main window
music_title_checkbox = Título
music_artist_checkbox = Artista
music_year_checkbox = Ano
music_bitrate_checkbox = Taxa de bits
music_genre_checkbox = Gênero
music_length_checkbox = Comprimento
music_comparison_checkbox = Comparação aproximada
music_checking_by_tags = Informações do arquivo
music_checking_by_content = Conteúdo
same_music_seconds_label = Duração mínima em segundos do fragmento
same_music_similarity_label = Diferença máxima
music_compare_only_in_title_group = Comparar somente o título
music_compare_only_in_title_group_tooltip =
    Quando esta opção está ativada, os arquivos são agrupados por título e são comparados entre si.
    
    Com 10.000 arquivos, em vez de se obter quase 100 milhões de comparações, normalmente, resultará em cerca de 20.000 comparações.
same_music_tooltip =
    A pesquisa de arquivos de música equivalentes por seu conteúdo pode ser definindo por meio das configurações:
    
    - O tempo mínimo do fragmento após o qual os arquivos de música podem ser identificados como equivalentes
    - A diferença máxima entre os dois fragmentos dos testes
    
    Para obter bons resultados forneça combinações razoáveis destes parâmetros em cada teste.
    
    Definir o tempo mínimo para 5s e a diferença máxima para 1.0, irá pesquisar fragmentos quase idênticos nos arquivos.
    Um tempo de 20s e uma diferença máxima de 6.0, por outro lado, funciona bem para encontrar versões ao vivo, versões modificadas (remixadas), etc.
    
    Por padrão, cada arquivo de música é comparado entre si, o que pode levar muito tempo para testar vários arquivos, portanto, é melhor utilizar pastas de referência e especificar quais são os arquivos devem ser comparados entre si. Com a mesma quantidade de arquivos, a comparação de impressões digitais será pelo menos quatro vezes mais rápida do que sem as pastas de referência.
music_comparison_checkbox_tooltip =
    Pesquisar arquivos de música equivalentes utilizando a inteligência artificial (IA) que utiliza o aprendizado da máquina para remover os parênteses de uma frase. Por exemplo, com esta opção ativada, os arquivos em questão que serão tratados como duplicados:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021) (Santuário --- Santuário (Remixado no verão de 2021))
duplicate_case_sensitive_name = Diferenciar maiúsculas de minúsculas
duplicate_case_sensitive_name_tooltip =
    Quando esta opção está ativada, agrupa apenas os registros se eles tiverem exatamente o mesmo nome. Por exemplo, pagar <-> pagar.
    
    Quando esta opção está desativada, agrupa os registros por nomes e sem verificar a diferença entre as letras maiúsculas e minúsculas. Por exemplo, pagar <-> Pagar.
duplicate_mode_size_name_combo_box = Tamanho e nome
duplicate_mode_name_combo_box = Nome
duplicate_mode_size_combo_box = Tamanho
duplicate_mode_hash_combo_box = Integridade do arquivo
duplicate_hash_type_tooltip =
     O Czkawka oferece três tipos de identificação pela integridade do arquivo por meio do código ‘hash’:
    
     Blake3 - esta opção possui o recurso de criptografia e é o padrão porque é muito rápida.
    
     CRC32 - esta opção é a mais simples e deveria ser mais rápida do que o Blake3, mas muito raramente pode ocorrer algumas colisões.
    
     XXH3 - esta opção não possui o recurso de criptografia e é muito similar em desempenho e qualidade do Blake3.
    
    Estes modos podem ser facilmente alternados.
duplicate_check_method_tooltip =
    Por enquanto, o Czkawka oferece três tipos de métodos para localizar os arquivos duplicados:
    
    Por nome - Localiza os arquivos que têm o mesmo nome.
    
    Por tamanho - Localiza os arquivos que têm o mesmo tamanho.
    
    Por integridade do arquivo - Localiza os arquivos que têm o mesmo conteúdo, ou seja, que possui o mesmo código ‘hash’ (o ‘hash’ de arquivo ou o valor do ‘hash’ de um arquivo é uma sequência de caracteres alfanuméricos distinta, trata-se de um valor único que corresponde ao conteúdo exato de um arquivo, permite verificar a integridade de um arquivo e é como se fosse a assinatura digital do arquivo). Este método cria a assinatura digital ou ‘hash’ do arquivo e, em seguida, compara o código da assinatura digital que foi criada para localizar os arquivos duplicados. Este método é a maneira mais segura e precisa de localizar os arquivos duplicados. O Czkawka utiliza a memória ‘cache’ (é um espaço de armazenamento das configurações, dos resultados das pesquisas, etc. que guarda os dados para que possam ser acessados mais rapidamente), portanto, a segunda verificação e as subsequentes dos mesmos dados deverão ser muito mais rápidas do que na primeira vez.
image_hash_size_tooltip =
    A cada imagem que é verificada, um arquivo de assinatura digital ou ‘hash’ é criado e que pode ser comparado entre si, e se uma pequena diferença entre as imagens for encontrada, então significa que as imagens são equivalentes.
    
    O tamanho de 8 do ‘hash’ é muito bom para localizar as imagens que são apenas um pouco equivalentes às originais. Com uma quantidade maior de imagens, maior do que 1000 imagens, irá produzir uma grande quantidade de falsos positivos, então é recomendado utilizar um tamanho maior do ‘hash’ nestes casos.
    
    O tamanho de 16 do ‘hash’ é o tamanho padrão por ser uma boa referência entre localizar as imagens que são um pouco equivalentes e ter uma pequena quantidade de colisões do código ‘hash’.
    
    O tamanho de 32 e 64 do ‘hash’ localizam imagens muito equivalentes, mas quase não deve ter falsos positivos, talvez, exceto algumas imagens que possuem o canal alfa.
image_resize_filter_tooltip =
    Para calcular o ‘hash’ de uma imagem, a biblioteca deve ser primeiro dimensionada.
    
    Escolha o algoritmo que será utilizado para calcular o ‘hash’ da imagem, saiba que poderá ter uma aparência um pouco diferente, dependendo do algoritmo que foi escolhido.
    
    O algoritmo mais rápido é o que produz os piores resultados e está ativado por padrão, porque com um tamanho do ‘hash’ de 16x16 a sua qualidade não é realmente perceptível.
    
    Com o tamanho do ‘hash’ de 8x8, recomenda-se utilizar um algoritmo diferente do mais próximo para obter melhores resultados para as imagens.
image_hash_alg_tooltip =
    É possível escolher um dos vários algoritmos para o cálculo da criação do ‘hash’.
    
    Cada um tem os seus pontos fortes e os seus pontos fracos, às vezes produzem resultados melhores e às vezes produzem resultados piores para imagens diferentes.
    
    É melhor testar qual algoritmo tem os melhores resultados para os diferentes tipos de arquivos, lembre-se de que, nem sempre é facilmente perceptível as diferenças dos resultados.
big_files_mode_combobox_tooltip = Permite pesquisar os arquivos menores ou maiores
big_files_mode_label = Arquivos a serem verificados
big_files_mode_smallest_combo_box = O arquivo menor
big_files_mode_biggest_combo_box = O arquivo maior
main_notebook_duplicates = Arquivos duplicados
main_notebook_empty_directories = Diretórios vazios
main_notebook_big_files = Arquivos grandes
main_notebook_empty_files = Arquivos vazios
main_notebook_temporary = Arquivos temporários
main_notebook_similar_images = Imagens equivalentes
main_notebook_similar_videos = Vídeos equivalentes
main_notebook_same_music = Músicas duplicadas
main_notebook_symlinks = Ligações simbólicas não válidas
main_notebook_broken_files = Arquivos quebrados
main_notebook_bad_extensions = Extensões inválidas
main_tree_view_column_file_name = Nome do arquivo
main_tree_view_column_folder_name = Nome da pasta
main_tree_view_column_path = Caminho
main_tree_view_column_modification = Data da modificação
main_tree_view_column_size = Tamanho
main_tree_view_column_similarity = Equivalentes
main_tree_view_column_dimensions = Dimensões
main_tree_view_column_title = Título
main_tree_view_column_artist = Artista
main_tree_view_column_year = Ano
main_tree_view_column_bitrate = Taxa de bits
main_tree_view_column_length = Comprimento
main_tree_view_column_genre = Gênero
main_tree_view_column_symlink_file_name = Nome do arquivo da ligação simbólica
main_tree_view_column_symlink_folder = Pasta da ligação simbólica
main_tree_view_column_destination_path = Caminho do destino
main_tree_view_column_type_of_error = Tipo de erro
main_tree_view_column_current_extension = Extensão atual
main_tree_view_column_proper_extensions = Extensões válidas
main_label_check_method = Método de verificação
main_label_hash_type = Tipo do hash
main_label_hash_size = Tamanho do hash
main_label_size_bytes = Tamanho (em bytes)
main_label_min_size = Mínimo
main_label_max_size = Máximo
main_label_shown_files = Quantidade de arquivos exibidos
main_label_resize_algorithm = Redimensionar o algoritmo
main_label_similarity = Equivalentes { "   " }
main_check_box_broken_files_audio = Áudio
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Arquivo
main_check_box_broken_files_image = Imagem
check_button_general_same_size = Ignorar os arquivos do mesmo tamanho
check_button_general_same_size_tooltip = Ignorar os arquivos com o mesmo tamanho nos resultados, geralmente estes são os arquivos duplicados (1:1)
main_label_size_bytes_tooltip = Tamanho dos arquivos que serão utilizados na pesquisa
# Upper window
upper_tree_view_included_folder_column_title = Pastas para serem pesquisadas
upper_tree_view_included_reference_column_title = Pastas de referência
upper_recursive_button = Pesquisa recursiva
upper_recursive_button_tooltip = Quando esta opção está ativada, a pesquisa por arquivos ocorre também nas pastas que não foram escolhidas.
upper_manual_add_included_button = Adicionar manualmente
upper_add_included_button = Adicionar
upper_remove_included_button = Remover
upper_manual_add_excluded_button = Remover manualmente
upper_add_excluded_button = Adicionar
upper_remove_excluded_button = Remover
upper_manual_add_included_button_tooltip =
    Adicionar manualmente os nomes dos diretórios ou das pastas para serem pesquisadas.
    
    Para adicionar vários caminhos de uma vez, separe-os com o ponto e vírgula ‘;’.
    
    /home/roman;/home/rozkaz adicionará dois diretórios /home/roman e /home/rozkaz
upper_add_included_button_tooltip = Adicionar um novo diretório para ser pesquisado.
upper_remove_included_button_tooltip = Remover o diretório da pesquisa.
upper_manual_add_excluded_button_tooltip =
    Adicionar manualmente um diretório à lista das exceções.
    
    Para adicionar vários caminhos de uma vez, separe-os com o ponto e vírgula ‘;’.
    
    Por exemplo, ‘/home/roman;/home/krokiet’ irá adicionar os dois diretórios ‘/home/roman’ e ‘/home/keokiet’
upper_add_excluded_button_tooltip = Selecionar o diretório que não será incluído na pesquisa.
upper_remove_excluded_button_tooltip = Selecionar o diretório da lista das exceções.
upper_notebook_items_configuration = Configurações dos itens
upper_notebook_excluded_directories = Diretórios não incluídos
upper_notebook_included_directories = Diretórios incluídos
upper_allowed_extensions_tooltip =
    Allowed extensions must be separated by commas (by default all are available).
    
    The following Macros, which add multiple extensions at once, are also available: IMAGE, VIDEO, MUSIC, TEXT.
    
    Usage example  ".exe, IMAGE, VIDEO, .rar, 7z" - this means that images (e.g. jpg, png), videos (e.g. avi, mp4), exe, rar, and 7z files will be scanned.
upper_excluded_extensions_tooltip =
    Lista de arquivos que serão ignorados na verificação.
    
    Quando você utiliza as extensões permitidas, estas tem maior prioridade em relação as outras, então o arquivo não será verificado.
upper_excluded_items_tooltip =
    Os itens que serão ignorados devem conter o asterisco ‘*’, que corresponde a qualquer cadeia de caracteres e devem ser separados por vírgulas.
    
    Esta opção é mais lenta do que a opção do diretórios não incluídos, portanto, deve ser utilizada com cuidado.
upper_excluded_items = Itens ignorados:
upper_allowed_extensions = Extensões permitidas:
upper_excluded_extensions = Extensões ignoradas:
# Popovers
popover_select_all = Selecionar todos
popover_unselect_all = Desselecionar todos
popover_reverse = Inverter a seleção
popover_select_all_except_oldest = Selecionar todos, exceto os mais antigos
popover_select_all_except_newest = Selecionar todos, exceto os mais recentes
popover_select_one_oldest = Selecionar um mais antigo
popover_select_one_newest = Selecionar um mais recente
popover_select_custom = Selecionar personalizado
popover_unselect_custom = Desselecionar personalizado
popover_select_all_images_except_biggest = Selecionar todos, exceto o maior
popover_select_all_images_except_smallest = Selecionar todos, exceto o menor
popover_custom_path_check_button_entry_tooltip =
    Selecionar os registros por caminho.
    
    Exemplo de uso:
    O caminho ‘/home/pimpek/rzecz.txt’ pode ser encontrado com ‘/home/pim*’
popover_custom_name_check_button_entry_tooltip =
      Selecionar os registros por nome de arquivo.
    
    Exemplo de uso:
    O caminho ‘/usr/ping/pong.txt’ pode ser encontrado com ‘*ong*’
popover_custom_regex_check_button_entry_tooltip =
    Selecionar os registros por meio das expressões regulares.
    
    Com o uso das expressões regulares (ou o modo ‘Regex’) o texto da pesquisa é o caminho completo (incluindo o nome do arquivo).
    
    Exemplo de uso:
    O caminho ‘/usr/bin/ziemniak.txt’ pode ser encontrado com ‘/ziem[a-z]+’
    
    Esta opção utiliza a implementação padrão das expressões regulares do ‘Rust’. Você pode obter mais informações acessando a página eletrônica https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Ativar a detecção da distinção entre as letras maiúsculas e minúsculas.
    
    Quando esta opção está ativada, o caminho ‘/home/*’ encontra ambos ‘HoMe/roman’ e ‘/home/roman’.
popover_custom_not_all_check_button_tooltip =
    Impedir que todos os registros de um grupo sejam selecionados.
    
    Esta opção está ativada por padrão, porque na maioria das situações, você provavelmente não quer excluir (ou apagar) os arquivos originais que estejam duplicados, mas quer manter pelo menos um dos arquivos.
    
    Aviso: Esta configuração não funcionará se você tiver selecionado manualmente todos os registros de um grupo.
popover_custom_regex_path_label = Caminho
popover_custom_regex_name_label = Nome
popover_custom_regex_regex_label = Expressão regular junto com o nome
popover_custom_case_sensitive_check_button = Diferenciar entre maiúsculas e minúsculas
popover_custom_all_in_group_label = Não selecionar todos os registros em um grupo
popover_custom_mode_unselect = Desselecionar o personalizado
popover_custom_mode_select = Selecionar o personalizado
popover_sort_file_name = Nome do arquivo
popover_sort_folder_name = Nome do diretório
popover_sort_full_name = Nome completo
popover_sort_size = Tamanho
popover_sort_selection = Seleção
popover_invalid_regex = A expressão regular não é válida
popover_valid_regex = A expressão regular é válida
# Bottom buttons
bottom_search_button = Pesquisar
bottom_select_button = Selecionar
bottom_delete_button = Excluir
bottom_save_button = Salvar
bottom_symlink_button = Ligação simbólica
bottom_hardlink_button = Ligação simbólica rígida
bottom_move_button = Mover
bottom_sort_button = Ordenar
bottom_compare_button = Comparar
bottom_search_button_tooltip = Iniciar a pesquisa
bottom_select_button_tooltip = Selecionar os registros. Apenas os arquivos e as pastas selecionadas podem ser processadas posteriormente.
bottom_delete_button_tooltip = Excluir os arquivos e as pastas selecionadas.
bottom_save_button_tooltip = Salvar as informações da pesquisa em arquivo
bottom_symlink_button_tooltip =
    Criar ligações simbólicas ou vínculos simbólicos (‘symbolic links’ ou ‘symlinks’ ou ‘soft links’) ou ‘atalho’ para um outro arquivo ou para um outro diretório.
    Esta opção só funciona se pelo menos dois resultados do grupo estiverem selecionados.
    O primeiro permanece inalterado, o segundo e os subsequentes estão vinculados ou ligados simbolicamente ao primeiro.
bottom_hardlink_button_tooltip =
    Criar ligações simbólicas rígidas ou vínculos simbólicos rígidos (‘hard links’) ou ‘atalho’ para um outro arquivo original ou para um outro diretório original.
    Esta opção só funciona se pelo menos dois resultados do grupo estiverem selecionados.
    O primeiro permanece inalterado, o segundo e os subsequentes estão vinculados ou ligados simbolicamente ao primeiro.
bottom_hardlink_button_not_available_tooltip =
    Criar ligações simbólicas rígidas ou vínculos simbólicos rígidos (‘hard links’) ou ‘atalho’ para um outro arquivo original ou para um outro diretório original.
    O botão está desativado, porque as ligações simbólicas rígidas não podem ser criadas.
    Este tipo de ligação simbólica só pode ser criada por um administrador no Windows, portanto, certifique-se de executar o programa com estas permissões de administrador.
    Se o programa estiver sendo executado com as permissões de administrador, verifique se existe questões equivalentes no GitHub do Czkawka (https://github.com/qarmin/czkawka).
bottom_move_button_tooltip =
    Mover os arquivos para o diretório que foi selecionado.
    Esta opção permite copiar todos os arquivos para o diretório sem preservar a estrutura dos diretórios e dos arquivos.
    Ao tentar mover dois arquivos com nomes idênticos para um diretório, o segundo arquivo não será movido e ocorrerá um erro.
bottom_sort_button_tooltip = Ordenar os arquivos ou os diretórios de acordo com o método que foi selecionado.
bottom_compare_button_tooltip = Compare as imagens do grupo.
bottom_show_errors_tooltip = Exibir ou ocultar o painel de texto inferior.
bottom_show_upper_notebook_tooltip = Exibir ou ocultar o painel de texto superior.
# Progress Window
progress_stop_button = Parar
progress_stop_additional_message = Parar a pesquisa
# About Window
about_repository_button_tooltip = Endereço da página eletrônica do repositório com o código-fonte do programa Czkawka.
about_donation_button_tooltip = Endereço da página eletrônica para fazer doação ao programador do Czkawka.
about_instruction_button_tooltip = Endereço da página eletrônica para obter ajuda.
about_translation_button_tooltip = Endereço da página eletrônica da plataforma de tradução ‘Crowdin’ com as traduções do programa Czkawka. Os idiomas polonês e inglês são fornecidos oficialmente pelo Rafał Mikrut, que também é conhecido por ‘qarmin’ (https://github.com/qarmin) e o idioma português do Brasil foi gentilmente traduzido por marcelocripe (marcelocripe@gmail.com) em novembro de 2024.
about_repository_button = Repositório
about_donation_button = Doar
about_instruction_button = Ajuda
about_translation_button = Tradução
# Header
header_setting_button_tooltip = Abrir a janela das configurações do programa Czkawka.
header_about_button_tooltip = Abrir a janela com as informações sobre o programa Czkawka.

# Settings


## General

settings_number_of_threads = Quantidade de tópicos utilizados
settings_number_of_threads_tooltip = Quantidade de tópicos utilizados, o zero ‘0’ significa que todos os tópicos estão disponíveis e poderão ser utilizados.
settings_use_rust_preview = Utilizar as bibliotecas externas em vez do GTK para carregar a pré-visualização
settings_use_rust_preview_tooltip =
    Utilizar a pré-visualização do GTK, às vezes é mais rápido e oferece suporte a mais formatos, mas às vezes pode ser exatamente o contrário.
    
    Se você tiver problemas para carregar pré-visualização, pode tentar alterar esta configuração.
    
    Nos sistemas operacionais que não são da família do GNU/Linux, é recomendável utilizar esta opção, porque o pacote ‘gtk-pixbuf’ nem sempre está disponível, portanto, a desativação desta opção não irá carregar a pré-visualização de algumas imagens.
settings_label_restart = Você precisa reiniciar o programa para aplicar as novas configurações!
settings_ignore_other_filesystems = Ignorar outros sistemas de arquivos (somente para o GNU/Linux)
settings_ignore_other_filesystems_tooltip =
    Ignorar os arquivos que não estão no mesmo sistema de arquivos dos diretórios que estão sendo pesquisados.
    
    Funciona da mesma forma que a opção ‘-xdev’ no comando ‘find’ (localizar) no GNU/Linux
settings_save_at_exit_button_tooltip = Salvar as configurações em arquivo ao fechar o programa.
settings_load_at_start_button_tooltip =
    Carregar as configurações a partir de um arquivo ao abrir o programa.
    
    Se esta opção não estiver ativada, as configurações padrão serão utilizadas.
settings_confirm_deletion_button_tooltip = Exibir a janela de confirmação de exclusão ao clicar no botão excluir.
settings_confirm_link_button_tooltip = Exibir a janela de confirmação ao clicar no botão da ligação simbólica.
settings_confirm_group_deletion_button_tooltip = Exibir a janela de confirmação de exclusão ao tentar excluir todos os registros de um grupo.
settings_show_text_view_button_tooltip = Exibir o painel de texto na parte inferior da interface gráfica do usuário
settings_use_cache_button_tooltip = Utilizar o arquivo de ‘cache’.
settings_save_also_as_json_button_tooltip = Salvar o ‘cache’ no formato JSON (que é legível por seres humanos), permite modificar o seu conteúdo. O arquivo de ‘cache’ será lido automaticamente pelo programa, se o formato do ‘cache’ for binário com a extensão ‘.bin’ ou se não tiver uma extensão do arquivo.
settings_use_trash_button_tooltip = Mover os arquivos para a lixeira em vez de excluí-los permanentemente.
settings_language_label_tooltip = Idioma da interface gráfica do usuário.
settings_save_at_exit_button = Salvar as configurações ao fechar o programa
settings_load_at_start_button = Carregar as configurações ao abrir o programa
settings_confirm_deletion_button = Exibir a janela de confirmação quando for excluir qualquer arquivo
settings_confirm_link_button = Exibir a janela de confirmação quando for criar qualquer arquivo de ligação simbólica ou de vínculo simbólico.
settings_confirm_group_deletion_button = Exibir a janela de confirmação quando for excluir todos os arquivos do grupo
settings_show_text_view_button = Exibir o painel de texto inferior
settings_use_cache_button = Utilizar o arquivo de ‘cache’
settings_save_also_as_json_button = Salvar o arquivo de ‘cache’ com o formato JSON
settings_use_trash_button = Mover os arquivos excluídos para a lixeira
settings_language_label = Configurações do idioma
settings_multiple_delete_outdated_cache_checkbutton = Excluir automaticamente os registros que estejam desatualizados no arquivo de ‘cache’
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
     Excluir os registros que estejam desatualizados no arquivo de ‘cache’.
    
    Quando esta opção está ativada, o programa se certifica de que, quando os registros são carregados, todos eles apontam para os arquivos válidos, enquanto que, os arquivos corrompidos ou alterados são ignorados.
    
    Quando esta opção está desativada, ajudará na verificação dos arquivos que estão nos dispositivos de armazenamento externos, de modo que os registros relacionados a eles não sejam excluídos na próxima verificação.
    
    No caso de ter centenas de milhares de registros no arquivo de ‘cache’, recomenda-se que esta opção seja ativada, pois ela irá acelerar o carregamento ou o salvamento do ‘cache’ no início ou no fim da pesquisa.
settings_notebook_general = Configurações gerais
settings_notebook_duplicates = Arquivos duplicados
settings_notebook_images = Imagens equivalentes
settings_notebook_videos = Vídeos equivalentes

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Exibir a pré-visualização no lado direito ao selecionar um arquivo de imagem.
settings_multiple_image_preview_checkbutton = Exibir a pré-visualização das imagens
settings_multiple_clear_cache_button_tooltip =
    Limpar manualmente as entradas que estão desatualizadas no arquivo de ‘cache’.
    Esta opção só deve ser utilizada se a limpeza automática estiver desativada.
settings_multiple_clear_cache_button = Remover os resultados que estejam desatualizados no arquivo de ‘cache’.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Ocultar todos os arquivos, exceto um, se todos eles apontarem para os mesmos dados, se são ligações simbólicas rígidas ou vínculos simbólicos rígidos (‘hard links’).
    
    Por exemplo: Se houver no dispositivo de armazenamento sete arquivos de ligações simbólicas rígidas para dados específicos e um arquivo é diferente com os mesmos dados, então, o pesquisador de arquivos duplicados irá identificar apenas um arquivo exclusivo e será exibido um arquivo de ligação simbólica rígida.
settings_duplicates_minimal_size_entry_tooltip =
     Configurar o tamanho mínimo do arquivo de ‘cache’ que será salvo no dispositivo de armazenamento.
    
    Definir um valor menor irá gerar mais registros, com isso, irá acelerar a pesquisa, mas irá tornar mais lento o carregamento ou o salvamento dos dados no arquivo de ‘cache’.
settings_duplicates_prehash_checkbutton_tooltip =
    Permite que os códigos de ‘hash’ (integridade do arquivo) parciais sejam salvos no arquivo de ‘cache’ (o ‘hash’ é calculado a partir de uma pequena parte do arquivo), o que permite que os arquivos únicos sejam descartados antecipadamente dos resultados da pesquisa dos arquivos que não são duplicados.
    
    Esta opção está ativada por padrão, pois pode causar lentidão em algumas situações.
    
    Recomenda-se utilizar esta opção ao fazer a pesquisa de centenas de milhares ou de milhões de arquivos, porque esta opção pode acelerar os resultados da pesquisa e pode desativar esta opção ao fazer a pesquisa de uma pequena quantidade de dados.
settings_duplicates_prehash_minimal_entry_tooltip = Tamanho mínimo do código ‘hash’ parcial que será gravado no arquivo de ‘cache’.
settings_duplicates_hide_hard_link_button = Ocultar as ligações rígidas (somente no GNU/Linux e no macOS)
settings_duplicates_prehash_checkbutton = Utilizar os ‘hash’ parciais no arquivo de ‘cache’
settings_duplicates_minimal_size_cache_label = Tamanho mínimo dos arquivos (em bytes) salvos no arquivo de ‘cache’
settings_duplicates_minimal_size_cache_prehash_label = Tamanho mínimo dos arquivos (em bytes) ao salvar o ‘hash’ parcial no arquivo de ‘cache’

## Saving/Loading settings

settings_saving_button_tooltip = Salvar as configurações atuais no arquivo.
settings_loading_button_tooltip = Carregar as configurações do arquivo e substituir a configuração atual.
settings_reset_button_tooltip = Restaurar a configurações padrão.
settings_saving_button = Salvar as configurações
settings_loading_button = Carregar as configurações
settings_reset_button = Restaurar as configurações

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Abrir a pasta onde os arquivos ‘.txt’ do ‘cache’ do programa está armazenado.
    
    A modificação manual dos arquivos de ‘cache’ pode causar a exibição de resultados que não são corretos ou os danos nos dados dos arquivos resultarão na necessidade de gerar novos arquivos de ‘cache’. No entanto, modificar o caminho pode economizar tempo ao mover uma grande quantidade de arquivos para um local diferente.
    
    Os arquivos de ‘cache’ podem ser copiados entre computadores diferentes para economizar o tempo da criação do ‘hash’ dos arquivos. Esta opção só é possível se os dados estiverem armazenados em uma estrutura de diretórios idêntica nos computadores.
    
    Se ocorrer problemas nos arquivos de ‘cache’, os arquivos podem ser excluídos permanentemente. O programa irá criar novos arquivos de ‘cache’ automaticamente.
settings_folder_settings_open_tooltip =
    Abrir a pasta onde está armazenada as configurações do Czkawka.
    
    Aviso: a modificação manual das configurações pode interromper no seu fluxo de trabalho.
settings_folder_cache_open = Abrir a pasta do ‘cache’
settings_folder_settings_open = Abrir a pasta das configurações
# Compute results
compute_stopped_by_user = A pesquisa foi interrompida pelo usuário
compute_found_duplicates_hash_size = Foram encontrados { $number_files } arquivos duplicados nos { $number_groups } grupos que adotaram o { $size } de tamanho
compute_found_duplicates_name = Foram encontrados { $number_files } arquivos duplicados nos { $number_groups } grupos
compute_found_empty_folders = Foram encontradas { $number_files } pastas vazias
compute_found_empty_files = Foram encontrados { $number_files } arquivos vazios
compute_found_big_files = Foram encontrados { $number_files } arquivos grandes
compute_found_temporary_files = Foram encontrados { $number_files } arquivos temporários
compute_found_images = Foram encontrados { $number_files } arquivos de imagem equivalentes nos { $number_groups } grupos
compute_found_videos = Foram encontrados { $number_files } arquivos de vídeo equivalentes nos { $number_groups } grupos
compute_found_music = Foram encontrados { $number_files } arquivos de música equivalentes nos { $number_groups } grupos
compute_found_invalid_symlinks = As { $number_files } ligações simbólicas não são válidas
compute_found_broken_files = Foram encontrados { $number_files } arquivos corrompidos
compute_found_bad_extensions = Foram encontrados { $number_files } arquivos com extensões que não são válidas
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] Verificado { $file_number } arquivo
       *[other] Escaneado { $file_number } arquivos
    }
progress_scanning_extension_of_files = Extensão marcada do arquivo { $file_checked }/{ $all_files }
progress_scanning_broken_files = Verificado { $file_checked }/{ $all_files } arquivo ({ $data_checked }/{ $all_data })
progress_scanning_video = Hash de { $file_checked }/{ $all_files } de vídeo
progress_scanning_image = Hash de { $file_checked }/{ $all_files } imagem ({ $data_checked }/{ $all_data })
progress_comparing_image_hashes = Comparado a { $file_checked }/{ $all_files } hash de imagem
progress_scanning_music_tags_end = Etiquetas comparadas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_tags = Ler etiquetas de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_content_end = Impressão digital comparada de { $file_checked }/{ $all_files } arquivo de música
progress_scanning_music_content = Calculada impressão digital de { $file_checked }/ arquivo de música{ $all_files } ({ $data_checked }/{ $all_data })
progress_scanning_empty_folders =
    { $folder_number ->
        [one] Pasta { $folder_number } escaneada
       *[other] Escaneado { $folder_number } pastas
    }
progress_scanning_size = Tamanho digitalizado do arquivo { $file_number }
progress_scanning_size_name = Nome digitalizado e tamanho do arquivo { $file_number }
progress_scanning_name = Nome digitalizado do arquivo { $file_number }
progress_analyzed_partial_hash = Hash parcial analisado de arquivos { $file_checked }/{ $all_files } ({ $data_checked }/{ $all_data })
progress_analyzed_full_hash = Hash completo analisado de arquivos { $file_checked }/{ $all_files } ({ $data_checked }/{ $all_data })
progress_prehash_cache_loading = Carregando o ‘hash’ parcial dos arquivos do ‘cache’
progress_prehash_cache_saving = Salvando o ‘hash’ parcial dos arquivos no ‘cache’
progress_hash_cache_loading = Carregando o ‘hash’ dos arquivos do ‘cache’
progress_hash_cache_saving = Salvando o ‘hash’ dos arquivos no ‘cache’
progress_cache_loading = Carregando as informações do ‘cache’
progress_cache_saving = Salvando as informações no ‘cache’
progress_current_stage = Estágio atual: { "  " }
progress_all_stages = Todos os estágios: { "  " }
# Saving loading 
saving_loading_saving_success = As configurações foram salvas no arquivo { $name }.
saving_loading_saving_failure = Ocorreu uma falha ao tentar salvar as configurações no arquivo { $name }.
saving_loading_reset_configuration = As configurações padrão foram restauradas.
saving_loading_loading_success = As configurações do programa foram carregadas com sucesso.
saving_loading_invalid_string = Para o conteúdo "{ $key }" foi encontrado o resultado "{ $result }" que não é um texto válido.
saving_loading_invalid_int = Para o conteúdo "{ $key }" foi encontrado o resultado "{ $result }" que não é um número inteiro.
saving_loading_invalid_bool = Para o conteúdo "{ $key }" foi encontrado o resultado "{ $result }" que não é do tipo ‘booleano’, ou seja, ‘true’ = verdadeiro = 1 (um) ou ‘false’ = falso = 0 (zero).
saving_loading_decode_problem_bool = Ocorreu uma falha ao tentar decodificar o dado ‘booleano’ o conteúdo "{ $key }", foi encontrado o resultado "{ $result }", mas os valores permitidos podem ser 0, 1, ‘true’ (verdadeiro) ou ‘false’ (falso).
saving_loading_saving_same_keys = Tentando salvar as configurações com o conteúdo duplicado "{ $key }".
saving_loading_failed_to_get_home_directory = Ocorreu uma falha ao acessar o diretório pessoal para abrir e salvar o arquivo de configurações.
saving_loading_folder_config_instead_file = Não foi possível criar ou abrir o arquivo de configurações no caminho "{ $path }", porque já existe uma pasta neste caminho.
saving_loading_failed_to_create_configuration_folder = Ocorreu uma falha ao criar a pasta de configurações no caminho "{ $path }", por causa de "{ $reason }".
saving_loading_failed_to_create_config_file = Ocorreu uma falha ao criar o arquivo de configurações no caminho "{ $path }", por causa de "{ $reason }".
saving_loading_failed_to_read_config_file = Não foi possível carregar o arquivo de configurações do caminho "{ $path }", porque o arquivo não existe ou porque não é um arquivo de configurações.
saving_loading_failed_to_read_data_from_file = Não foi possível ler os dados do arquivo do caminho "{ $path }", por causa de "{ $reason }".
saving_loading_orphan_data = Foram encontrados dados órfãos "{ $data }" na linha "{ $line }".
saving_loading_not_valid = A configuração "{ $data }" não existe na versão atual do programa.
# Invalid symlinks
invalid_symlink_infinite_recursion = Ocorreu um erro de execução na recursão infinita
invalid_symlink_non_existent_destination = O arquivo de destino não existe
# Other
selected_all_reference_folders = Não foi possível iniciar a pesquisa se todas as pastas estiverem definidas como pastas de origem (ou pastas de referência)
searching_for_data = Pesquisando os dados. Esta ação pode demorar. Por favor, aguarde...
text_view_messages = Exibir as mensagens
text_view_warnings = Exibir os avisos
text_view_errors = Exibir os erros
about_window_motto =
    Este programa é e sempre será de uso gratuito.
    
    Talvez a interface do programa não seja tão ergonômica, mas pelo menos, o programa é de código aberto.
# Various dialog
dialogs_ask_next_time = Perguntar na próxima vez que for exibida a janela
delete_file_failed = Ocorreu uma falha ao excluir o arquivo { $name }, por causa de { $reason }
delete_title_dialog = Confirmação de exclusão
delete_question_label = Você tem certeza de que quer excluir os arquivos?
delete_all_files_in_group_title = Confirmação de exclusão de todos os arquivos do grupo
delete_all_files_in_group_label1 = Em alguns grupos, todos os registros estão selecionados.
delete_all_files_in_group_label2 = Você tem certeza de que quer excluí-los?
delete_folder_failed = Ocorreu uma falha ao excluir a pasta { $dir }, porque a pasta não existe ou porque você não tem permissões para apagá-la ou porque a pasta não está vazia.
delete_items_label = Os arquivos { $items } serão excluídos.
delete_items_groups_label = Os arquivos { $items } dos grupos { $groups } serão excluídos.
hardlink_failed = Ocorreu uma falha ao criar a ligação simbólica rígida
hard_sym_invalid_selection_title_dialog = Alguns grupos não são válidos para serem selecionados
hard_sym_invalid_selection_label_1 = Em alguns grupos, existe apenas um registro selecionado e será ignorado.
hard_sym_invalid_selection_label_2 = Para criar a ligação simbólica rígida dos arquivos, pelo menos dois registros de um grupo devem ser selecionados.
hard_sym_invalid_selection_label_3 = O primeiro registro no grupo é reconhecido como original e não é alterado, mas o segundo registro e os subsequentes são vinculados ou ligados ao primeiro.
hard_sym_link_title_dialog = Confirmação da ligação simbólica
hard_sym_link_label = Você tem certeza de que quer criar a ligação simbólica para estes arquivos?
move_folder_failed = Ocorreu uma falha ao mover a pasta { $name }, por causa de { $reason }
move_file_failed = Ocorreu uma falha ao mover o arquivo { $name }, por causa de { $reason }
move_files_title_dialog = Escolha a pasta para a qual você quer mover os arquivos duplicados
move_files_choose_more_than_1_path = Apenas um caminho pode ser selecionado para copiar os arquivos duplicados. A pasta { $path_number } foi selecionada.
move_stats = Os itens { $num_files } de { $all_files } foram movidos corretamente.
save_results_to_file = Os resultados foram salvos no arquivo com o formato ‘.txt’ e no arquivo com o formato ‘.json’ na pasta { $name }.
search_not_choosing_any_music = Ocorreu um erro: Você deve selecionar pelo menos uma caixa de seleção com o tipo dos arquivos de música que serão pesquisadas.
search_not_choosing_any_broken_files = Ocorreu um erro: Você deve selecionar pelo menos uma caixa de seleção com o tipo dos arquivos quebrados que serão pesquisados.
include_folders_dialog_title = Pastas a serem pesquisadas
exclude_folders_dialog_title = Pastas a serem ignoradas
include_manually_directories_dialog_title = Adicionar as pastas manualmente
cache_properly_cleared = O ‘cache’ foi limpo com sucesso
cache_clear_duplicates_title = Limpando os arquivos duplicados do ‘cache’
cache_clear_similar_images_title = Limpando as imagens equivalentes do ‘cache’
cache_clear_similar_videos_title = Limpando os vídeos equivalentes do ‘cache’
cache_clear_message_label_1 = Você quer limpar as entradas que estão desatualizadas no ‘cache’?
cache_clear_message_label_2 = Esta ação irá excluir todos os registros do ‘cache’ que apontam para os arquivos que não são válidos ou que não existentes.
cache_clear_message_label_3 = Esta opção pode acelerar um pouco o carregamento ou o salvamento do ‘cache’.
cache_clear_message_label_4 = Aviso: Esta ação irá excluir todos os dados que estão armazenados no ‘cache’ das unidades externas que não estão conectadas. Portanto, todos os ‘hash’ terão que de ser gerados novamente.
# Show preview
preview_image_resize_failure = Ocorreu uma falha ao redimensionar a imagem { $name }.
preview_image_opening_failure = Ocorreu uma falha ao abrir a imagem { $name }, por causa de { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = O grupo { $current_group } de { $all_groups } grupos possui { $images_in_group } imagens
compare_move_left_button = E
compare_move_right_button = D
