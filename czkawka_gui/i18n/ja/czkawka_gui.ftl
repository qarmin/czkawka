# Window titles
window_settings_title = オプション
window_main_title = Czkawka (しゃっくり)
window_progress_title = スキャン中
window_compare_images = 画像を比較
# General
general_ok_button = Ok
general_close_button = 閉じる
# Main window
music_title_checkbox = タイトル
music_artist_checkbox = アーティスト
music_album_title_checkbox = アルバムタイトル
music_album_artist_checkbox = アルバムアーティスト
music_year_checkbox = 年
music_comparison_checkbox = おおよその比較
music_comparison_checkbox_tooltip =
    AIを使用して類似の音楽ファイルを検索します。例えば、機械学習を使用して句から括弧を削除します。 このオプションが有効な場合 問題のファイルは重複とみなされます:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_mode_name_combo_box = 名前
duplicate_mode_size_combo_box = サイズ
duplicate_mode_hash_combo_box = ハッシュ
duplicate_hash_type_tooltip =
    Czkawkaは、3種類のハッシュを提供しており、これらを使用することができます。
    
    Blake3 - 暗号化ハッシュ関数。非常に高速であるため、デフォルトのハッシュアルゴリズムとして使用される。
    
    CRC32 - 単純なハッシュ関数。Blake3より高速ですが、まれに衝突が発生する可能性があります。
    
    XXH3：性能とハッシュの質がBlake3に非常に似ているので、このようなモードは簡単に使用できます。
duplicate_check_method_tooltip =
    Czkawkaは、今のところ、以下の3種類の方法で重複を見つけることができます。
    
    名前 - 同じ名前のファイルを検索します。
    
    サイズ - 同じサイズのファイルを探す。
    
    ハッシュ - 同じ内容のファイルを探す。このモードは、ファイルをハッシュ化し、後でこのハッシュを比較して、重複を見つけます。このモードは、重複を見つけるための最も安全な方法です. このツールはキャッシュを多用するので、同じデータの2回目以降のスキャンは、最初のスキャンよりずっと速くなるはずです。
image_hash_size_tooltip =
    Czkawkaは各画像に対して生成されたハッシュのサイズを変更することを提供する。 ハッシュが大きいほど、画像間の差異の少ない画像を見つけることができますが、使用が少し遅くなります。
    
    ハッシュ値のデフォルト値は8バイトで、非常に類似した異なる画像を見つけることができます。 16 と 32 ハッシュは、ほぼ同じ画像にのみ使用する必要があります。 64 バイトのハッシュを使用するべきではありません。本当に小さな違いを見つける必要がある状況を除いて、
image_resize_filter_tooltip = 画像のハッシュを計算するには、まずライブラリのサイズを変更する必要があります。選択されたアルゴリズムによって、結果の画像はほとんど異なります。 最速のアルゴリズムだけでなく、最悪の結果をもたらすアルゴリズムはNearestです。
image_hash_alg_tooltip = ハッシュ計算の多くのアルゴリズムから1つを選択することができます。 それぞれが強い点と弱い点の両方を持っており、時には異なる画像のより良い結果と時には悪い結果を与えます。 最良のものを選ぶには手動でテストする必要があります
main_notebook_image_fast_compare = 比較を速くする
main_notebook_image_fast_compare_tooltip =
    ハッシュの検索・比較を高速化。
    
    通常モードでは、各ハッシュをx回比較します（xはユーザが選択した類似度）。
    
    このオプションは、類似度が0(Very High) でない1,000枚以上の画像を比較する場合に推奨されます。
main_notebook_duplicates = ファイルを複製
main_notebook_empty_directories = 空のディレクトリ
main_notebook_big_files = 大きなファイル
main_notebook_empty_files = 空のファイル
main_notebook_temporary = 一時ファイル
main_notebook_similar_images = 類似の画像
main_notebook_similar_videos = 類似の動画
main_notebook_same_music = 音楽重複
main_notebook_symlinks = 無効なSymlinks
main_notebook_broken_files = 壊れたファイル
main_tree_view_column_file_name = ファイル名
main_tree_view_column_folder_name = フォルダ名
main_tree_view_column_path = パス
main_tree_view_column_modification = 変更日
main_tree_view_column_size = サイズ
main_tree_view_column_similarity = 類似度
main_tree_view_column_dimensions = 寸法
main_tree_view_column_title = タイトル
main_tree_view_column_artist = アーティスト
main_tree_view_column_year = 年
main_tree_view_column_album_title = アルバムタイトル
main_tree_view_column_album_artist = アルバムアーティスト
main_tree_view_column_symlink_file_name = シンボリックリンクのファイル名
main_tree_view_column_symlink_folder = Symlnik フォルダ
main_tree_view_column_destination_path = 宛先パス
main_tree_view_column_type_of_error = エラーの種類
main_label_check_method = メソッドのチェック
main_label_hash_type = ハッシュタイプ
main_label_hash_size = ハッシュサイズ
main_label_size_bytes = サイズ(バイト)
main_label_min_size = 最小
main_label_max_size = 最大値
main_label_shown_files = 表示するファイルの数
main_label_resize_algorithm = アルゴリズムのサイズ変更
main_label_similarity = Similarity{ " " }
check_button_general_same_size = 同じサイズを無視
check_button_general_same_size_tooltip = 結果から無視し、同じサイズのファイル - 通常、これは1:1重複です
main_label_size_bytes_tooltip = スキャンで使用されるファイルのサイズ
# Upper window
upper_tree_view_included_folder_column_title = 検索するフォルダ
upper_tree_view_included_reference_column_title = 参照フォルダ
upper_recursive_button = 再帰的
upper_recursive_button_tooltip = 選択した場合、選択したフォルダの下に直接配置されていないファイルも検索します。
upper_manual_add_included_button = 手動追加
upper_add_included_button = 追加
upper_remove_included_button = 削除
upper_manual_add_excluded_button = 手動追加
upper_add_excluded_button = 追加
upper_remove_excluded_button = 削除
upper_manual_add_included_button_tooltip = 手動で検索するディレクトリ名を追加できます。
upper_add_included_button_tooltip = 検索に新しいディレクトリを追加します。
upper_remove_included_button_tooltip = 検索からディレクトリを削除する。
upper_manual_add_excluded_button_tooltip = 除外されたディレクトリ名を手動で追加できます。
upper_add_excluded_button_tooltip = 検索で除外するディレクトリを追加します。
upper_remove_excluded_button_tooltip = 除外されたディレクトリを削除します。
upper_notebook_items_configuration = アイテム設定
upper_notebook_excluded_directories = 除外されたディレクトリ
upper_notebook_included_directories = 含まれるディレクトリ
upper_allowed_extensions_tooltip =
    許可する拡張子はカンマで区切る必要があります（デフォルトではすべて使用可能です）。
    
    複数の拡張子を一度に追加するマクロ IMAGE, VIDEO, MUSIC, TEXT も利用可能です。
    
    使用例 ".exe, IMAGE, VIDEO, .rar, 7z" - 画像（jpg、pngなど）、動画（avi、mp4など）、exe、rar、7zファイルがスキャンされることを意味します。
upper_excluded_items_tooltip =
    除外された項目は*ワイルドカードを含んでいる必要があり、コンマで区切る必要があります。
    除外されたディレクトリよりも遅いので注意して使用してください。
upper_excluded_items = 除外するアイテム:
upper_allowed_extensions = 許可される拡張子:
# Popovers
popover_select_all = すべて選択
popover_unselect_all = すべて選択解除
popover_reverse = 選択を逆にする
popover_select_all_except_oldest = 古いもの以外のすべてを選択
popover_select_all_except_newest = 最新以外のすべてを選択
popover_select_one_oldest = 古いものを選択
popover_select_one_newest = 最新を1つ選択してください
popover_select_custom = カスタムを選択
popover_unselect_custom = カスタムの選択を解除
popover_select_all_images_except_biggest = 最大値以外のすべてを選択
popover_select_all_images_except_smallest = 最小以外のすべてを選択
popover_custom_path_check_button_entry_tooltip =
    パスでレコードを選択できます。
    
    使用例:
    /home/pimpek/rzecz.txt は /home/pim* で見つけることができます
popover_custom_name_check_button_entry_tooltip =
    ファイル名でレコードを選択できます。
    
    使用例:
    *ong* で /usr/ping/pong.txt を見つけることができます
popover_custom_regex_check_button_entry_tooltip =
    指定した正規表現でレコードを選択することができます。
    
    このモードでは、検索された文字列はPath with Nameになります。
    
    使用例。
    /usr/bin/ziemniak.txt は /ziem[a-z]+ で検索されます。
    
    これはRustのデフォルトの正規表現実装を使用しているので、詳しくはhttps://docs.rs/regex を参照してください。
popover_custom_not_all_check_button_tooltip =
    グループ内のすべてのレコードを選択できないようにします。
    
    これはデフォルトで有効になっています。ほとんどの状況でユーザーは元のファイルと重複ファイルの両方を削除したくないためです。 少なくとも1つのファイルを残したい 
    
    
    
    警告: この設定は、すでにユーザーがグループ内のすべての結果を手動で選択している場合には機能しません。
popover_custom_regex_path_label = パス
popover_custom_regex_name_label = 名前
popover_custom_regex_regex_label = 正規表現パス + 名前
popover_custom_all_in_group_label = グループ内のすべてのレコードを選択しない
popover_custom_mode_unselect = カスタムの選択を解除
popover_custom_mode_select = カスタムを選択
popover_invalid_regex = 正規表現が無効です
popover_valid_regex = 正規表現が有効です
# Bottom buttons
bottom_search_button = 検索
bottom_select_button = 選択
bottom_delete_button = 削除
bottom_save_button = 保存
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = 移動
bottom_search_button_tooltip = ファイル/フォルダの検索を開始します。
bottom_select_button_tooltip = レコードを選択します。選択したファイル/フォルダのみが後で処理できます。
bottom_delete_button_tooltip = 選択したファイル/フォルダを削除します。
bottom_save_button_tooltip = 検索に関するデータをファイルに保存
bottom_symlink_button_tooltip =
    シンボリックリンクを作成します。
    グループ内の2つ以上の結果が選択されている場合のみ機能します。
    最初の結果は変更されず、2番目以降の結果は最初の結果にシンボリックリンクされます。
bottom_hardlink_button_tooltip =
    ハードリンクを作成します。
    グループ内の2つ以上の結果が選択されているときのみ機能します。
    最初の結果は変更されず、2番目以降が最初の結果にハードリンクされます。
bottom_move_button_tooltip =
    選択したフォルダにファイルを移動します。
    ディレクトリツリーを維持したまま、すべてのファイルをフォルダにコピーします。
    同じ名前の2つのファイルをフォルダに移動しようとすると、2番目のファイルが失敗し、エラーが表示されます。
bottom_show_errors_tooltip = 下部のエラーパネルの表示/非表示
bottom_show_upper_notebook_tooltip = 上部のノートブックパネルの表示/非表示
# Progress Window
progress_stop_button = 停止
# About Window
about_repository_button_tooltip = ソースコードでリポジトリページにリンクします。
about_donation_button_tooltip = 寄付ページにリンクします。
about_instruction_button_tooltip = 説明ページにリンクします。
about_translation_button_tooltip = アプリの翻訳とCrowdinのページへのリンク。公式にはポーランド語と英語がサポートされていますが、他の言語のヘルプは歓迎されます。
about_repository_button = リポジトリ
about_donation_button = 寄付
about_instruction_button = 説明
about_translation_button = 翻訳
# Header
header_setting_button_tooltip = 設定ダイアログを開きます。
header_about_button_tooltip = アプリに関する情報を含むダイアログを開きます。

# Settings


## General

settings_save_at_exit_button_tooltip = アプリを閉じるときに設定をファイルに保存します。
settings_load_at_start_button_tooltip =
    ファイルから設定を開始時に読み込みます。
    
    このオプションを選択しないと、デフォルト設定が読み込まれます。
settings_confirm_deletion_button_tooltip = 削除ボタンをクリックしたときに確認ダイアログを表示します。
settings_confirm_link_button_tooltip = Hard/symlink ボタンをクリックしたときに確認ダイアログを表示します。
settings_confirm_group_deletion_button_tooltip = グループからすべてのレコードを削除しようとしたときにダイアログを表示します。
settings_show_text_view_button_tooltip = 下部にエラーパネルを表示
settings_use_cache_button_tooltip = キャッシュ機能を使用しないオプション。
settings_save_also_as_json_button_tooltip = キャッシュを人間JSON形式で読み取り可能に保存します。コンテンツを変更することができます。 バイナリフォーマットキャッシュ(bin拡張子付き) がない場合、このファイルから自動的にキャッシュが読み込まれます。
settings_use_trash_button_tooltip = 有効にすると、ファイルをゴミ箱に移動する代わりに永久に削除します。
settings_language_label_tooltip = 利用可能な言語からインターフェイスの言語を選択できます。
settings_save_at_exit_button = 終了時に設定を保存
settings_load_at_start_button = 開始時に設定を読み込む
settings_confirm_deletion_button = ファイルを削除するときに確認ダイアログを表示する
settings_confirm_link_button = ハードリンク/シンボリックリンク時に確認ダイアログを表示する
settings_confirm_group_deletion_button = グループ内のすべてのファイルを削除するときに確認ダイアログを表示する
settings_show_text_view_button = 下部のテキストパネルを表示
settings_use_cache_button = キャッシュを使用
settings_save_also_as_json_button = JSONファイルにもキャッシュを保存する
settings_use_trash_button = 削除したファイルをゴミ箱に移動する
settings_language_label = 言語
settings_multiple_delete_outdated_cache_checkbutton = 古いキャッシュエントリを自動的に削除
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    存在しないファイルを指している古いキャッシュ結果を削除できるようにします。
    
    このオプションを有効にすると、アプリはレコードを読み込むときに、すべてのポイントが有効なファイルであることを確認し、壊れたファイルを無視します。
    
    このオプションを無効にすると、外部ドライブ上のファイルをスキャンするのに役立ち、それらに関するキャッシュ エントリが次のスキャンで削除されなくなります。
    
    キャッシュに何十万ものレコードがある場合、スキャンの開始時と終了時のキャッシュの読み込みと保存を高速化するために、このオプションを有効にすることが推奨されます。
settings_notebook_general = 全般
settings_notebook_duplicates = 重複
settings_notebook_images = 類似の画像
settings_notebook_videos = 類似の動画

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = 画像ファイルを選択するときに、プレビューを右側に表示します。
settings_multiple_image_preview_checkbutton = 画像のプレビューを表示
settings_multiple_clear_cache_button_tooltip =
    古いエントリから手動でキャッシュをクリアします。
    自動クリアが無効の場合にのみ使用する必要があります。
settings_multiple_clear_cache_button = 画像キャッシュから古い結果を削除します

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    1つを除くすべてのファイルを非表示にします。同じデータを指している場合(ハードリンクされています)。
    
    E.g. ディスク上に特定のデータにハードリンクされている7つのファイルがある場合 同じデータを持つ1つの異なるファイルが異なるが、異なるinodeの 複製されたファインダーでは一意のファイルとハードリンクされたファイルのファイルのみが表示されます
settings_duplicates_minimal_size_entry_tooltip =
    キャッシュされるファイルの最小サイズを設定できます。
    
    値を小さくすると、検索を高速化するレコードが増えますが、キャッシュの読み込み/保存が遅くなります。
settings_duplicates_prehash_checkbutton_tooltip =
    プレハッシュ(ファイルの一部から計算したハッシュ) のキャッシュを有効にし、重複していない結果を早めに捨てられるようにします。
    
    この機能はデフォルトでは無効になっています。
    
    数十万、数百万ファイルをスキャンする場合は、検索を何倍も高速化できるため、使用を強く推奨します。
settings_duplicates_prehash_minimal_entry_tooltip = キャッシュされたエントリの最小サイズ。
settings_duplicates_hide_hard_link_button = ハードリンクを隠す (Linux と MacOSのみ)
settings_duplicates_prehash_checkbutton = プリハッシュキャッシュを使用
settings_duplicates_minimal_size_cache_label = キャッシュに保存されたバイト単位のファイルの最小サイズ
settings_duplicates_minimal_size_cache_prehash_label = プリハッシュキャッシュに保存されたバイト単位のファイルの最小サイズ

## Saving/Loading settings

settings_saving_button_tooltip = 現在の設定をファイルに保存します。
settings_loading_button_tooltip = 設定をファイルから読み込み、現在の設定を置き換えます。
settings_reset_button_tooltip = 現在の設定をデフォルトにリセットします。
settings_saving_button = 設定を保存
settings_loading_button = 構成を読み込む
settings_reset_button = 設定をリセット

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    キャッシュを持つtxtファイルが保存されているフォルダを開きます。
    
    これらのファイルを変更すると、不正な結果を表示することがありますが、パスなどを変更することで、大量のファイルを別の場所に移動する際の時間を短縮することができます。
    
    このファイルをコンピュータ間でコピーすることで、再度ファイルをスキャンする時間を節約できます（もちろん、コンピュータのディレクトリ構造が似ている場合）。
    
    キャッシュに問題がある場合、このファイルを削除することができ、アプリは自動的にそれらを再生成します。
settings_folder_settings_open_tooltip =
    チェコの設定が保存されているフォルダを開きます。
    
    手動で変更すると、ワークフローが壊れる可能性があります。
settings_folder_cache_open = キャッシュフォルダーを開く
settings_folder_settings_open = 設定フォルダを開く
# Compute results
compute_stopped_by_user = 検索はユーザーによって停止されました
compute_found_duplicates_hash_size = { $number_files } 個のグループで { $number_groups } 個の重複が { $size } 個見つかりました
compute_found_duplicates_name = { $number_files } グループ内に { $number_groups } 件の重複が見つかりました
compute_found_empty_folders = 空のフォルダが { $number_files } 個見つかりました
compute_found_empty_files = 空の { $number_files } 個のファイルが見つかりました
compute_found_big_files = { $number_files } 個の大きなファイルが見つかりました
compute_found_temporary_files = { $number_files } 一時ファイルが見つかりました
compute_found_images = { $number_groups } グループで { $number_files } 件の類似した画像が見つかりました
compute_found_videos = { $number_groups } グループで { $number_files } 件の類似した動画が見つかりました
compute_found_music = { $number_groups } グループで { $number_files } 件の類似した音楽ファイルが見つかりました
compute_found_invalid_symlinks = { $number_files } 無効なシンボリックリンクが見つかりました
compute_found_broken_files = 壊れたファイルが { $number_files } 個見つかりました
# Progress window
progress_scanning_general_file = { $file_number } ファイルをスキャン中
progress_scanning_broken_files = { $file_checked }/{ $all_files } ファイルを確認中
progress_scanning_video = { $file_checked }/{ $all_files } ビデオのハッシュ
progress_scanning_image = { $file_checked }/{ $all_files } の画像のハッシュ
progress_comparing_image_hashes = { $file_checked }/{ $all_files } 画像ハッシュの比較
progress_scanning_music_tags_end = { $file_checked }/{ $all_files } 音楽ファイルのタグの比較
progress_scanning_music_tags = { $file_checked }/{ $all_files } 音楽ファイルのタグを読み込み中
progress_scanning_empty_folders = { $folder_number } フォルダをスキャン中
progress_scanning_size = { $file_number } ファイルのサイズをスキャンしています
progress_scanning_name = { $file_number } ファイルの名前をスキャン中
progress_analyzed_partial_hash = { $file_checked }/{ $all_files } ファイルの部分ハッシュを分析しました
progress_analyzed_full_hash = { $file_checked }/{ $all_files } ファイルの完全ハッシュを分析しました
progress_current_stage = 現在のステージ:{ " " }
progress_all_stages = すべてのステージ:{ " " }
# Saving loading 
saving_loading_saving_success = ファイル { $name } に設定を保存しました。
saving_loading_saving_failure = 設定データをファイル { $name } に保存できませんでした。
saving_loading_reset_configuration = 現在の設定がクリアされました。
saving_loading_loading_success = 正しく読み込まれたアプリの設定。
saving_loading_invalid_string = キー "{ $key }" の場合、文字列ではない不正な結果 - "{ $result }" が見つかりました。
saving_loading_invalid_int = キー "{ $key }" の場合、整数ではない "{ $result }" の不正な結果が見つかりました。
saving_loading_invalid_bool = キー "{ $key }" の場合、ブールではない不正な結果 - "{ $result }" が見つかりました。
saving_loading_decode_problem_bool = キー "{ $key }"{ $result }" から bool をデコードできませんでしたが、許可されている値は 0、1、true、または false です。
saving_loading_saving_same_keys = 重複キー「{ $key }」で設定を保存しています。
saving_loading_failed_to_get_home_directory = ホームディレクトリを開く/保存に失敗しました。
saving_loading_folder_config_instead_file = フォルダが既に存在するため、パス「{ $path }」に保存設定ファイルを作成または開くことができません。
saving_loading_failed_to_create_configuration_folder = 設定フォルダ「{ $path }」、理由「{ $reason }」の作成に失敗しました。
saving_loading_failed_to_create_config_file = 設定ファイル「{ $path }」、理由「{ $reason }」の作成に失敗しました。
saving_loading_failed_to_read_config_file = 存在しないかファイルでないため、"{ $path }"から設定を読み込めません。
saving_loading_failed_to_read_data_from_file = ファイル "{ $path }", 理由 "{ $reason } " からデータを読み取ることができません。
saving_loading_orphan_data = "{ $data }" 行に "{ $line } " が見つかりました。
saving_loading_not_valid = 設定 "{ $data }" は現在のアプリのバージョンに存在しません。
# Invalid symlinks
invalid_symlink_infinite_recursion = 無限再帰性
invalid_symlink_non_existent_destination = 保存先ファイルが存在しません
# Other
searching_for_data = データを検索中、しばらくお待ちください...
text_view_messages = メッセージ
text_view_warnings = 警告
text_view_errors = エラー
about_window_motto = このプログラムは自由に使用することができ、常になります。
# Various dialog
dialogs_ask_next_time = 次回に確認
delete_file_failed = ファイル { $name } の削除に失敗しました , 理由 { $reason }
delete_title_dialog = 削除の確認
delete_question_label = ファイルを削除してもよろしいですか？
delete_all_files_in_group_title = グループ内のすべてのファイルを削除することの確認
delete_all_files_in_group_label1 = 一部のグループでは、すべてのレコードが選択されています。
delete_all_files_in_group_label2 = 本当に削除しますか？
delete_folder_failed = フォルダが存在しないため、フォルダ { $dir } の削除に失敗しました。権限がないか、空ではありません。
delete_items_label = { $items } ファイルが削除されます。
delete_items_groups_label = { $items } グループから { $groups } 個のファイルが削除されます。
hardlink_failed = ハードリンクに失敗しました
hard_sym_invalid_selection_title_dialog = いくつかのグループで無効な選択です
hard_sym_invalid_selection_label_1 = 一部のグループでは、選択されたレコードは1つだけで、無視されます。
hard_sym_invalid_selection_label_2 = このファイルをハード/システムにリンクできるようにするには、グループ内の少なくとも2つの結果を選択する必要があります。
hard_sym_invalid_selection_label_3 = 最初のグループは、元として認識され、変更されていませんが、後で変更されます。
hard_sym_link_title_dialog = リンクの確認
hard_sym_link_label = このファイルをリンクしてもよろしいですか？
move_folder_failed = フォルダ { $name } の移動に失敗しました , 理由 { $reason }
move_file_failed = ファイル { $name } を移動できませんでした、理由 { $reason }
move_files_title_dialog = 重複したファイルを移動するフォルダを選択
move_files_choose_more_than_1_path = 重複したファイルをコピーするには、1つのパスのみを選択する必要があります。 { $path_number }
move_stats = { $num_files }/{ $all_files } アイテムを正しく移動しました
save_results_to_file = ファイル { $name } に結果を保存しました
search_not_choosing_any_music = エラー: 音楽検索タイプのチェックボックスを少なくとも1つ選択する必要があります。
include_folders_dialog_title = 含めるフォルダ
exclude_folders_dialog_title = 除外するフォルダ
include_manually_directories_dialog_title = ディレクトリを手動で追加
cache_properly_cleared = キャッシュを適切にクリアしました
cache_clear_duplicates_title = 重複したキャッシュをクリアする
cache_clear_similar_images_title = 類似画像のキャッシュをクリア中
cache_clear_similar_videos_title = 類似の動画キャッシュをクリア中
cache_clear_message_label_1 = 古いエントリからキャッシュを消去しますか？
cache_clear_message_label_2 = この操作は無効なファイルを指すすべてのキャッシュエントリを削除します。
cache_clear_message_label_3 = これは、キャッシュへの読み込み/保存を少し高速化することがあります。
cache_clear_message_label_4 = 警告: 操作により、アンプラグドの外部ドライブからキャッシュされたすべてのデータが削除されます。そのため、ハッシュを再度生成する必要があります。
# Show preview
preview_temporary_file = 一時イメージ ファイル { $name } を開けませんでした、理由 { $reason }。
preview_0_size = 画像 { $name } のプレビューを作成できません。幅または高さが0です。
preview_temporary_image_save = 一時イメージファイルを { $name }、理由 { $reason } に保存できませんでした。
preview_temporary_image_remove = 一時イメージ ファイル { $name } の削除に失敗しました、理由 { $reason }。
preview_failed_to_create_cache_dir = 画像プレビュー、理由 { $name } で必要なdir { $reason } を作成できませんでした。
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = グループ { $current_group }/{ $all_groups } ({ $images_in_group } 画像)
compare_move_left_button = L
compare_move_right_button = R
