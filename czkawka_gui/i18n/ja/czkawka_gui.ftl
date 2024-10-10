# Window titles
window_settings_title = 設定
window_main_title = Czkawka (しゃっくり)
window_progress_title = スキャン中
window_compare_images = 画像を比較
# General
general_ok_button = Ok
general_close_button = 閉じる
# Main window
music_title_checkbox = タイトル
music_artist_checkbox = アーティスト
music_year_checkbox = 年
music_bitrate_checkbox = ビットレート
music_genre_checkbox = ジャンル
music_length_checkbox = 長さ
music_comparison_checkbox = おおよその比較
music_checking_by_tags = タグ
music_checking_by_content = コンテンツ
same_music_seconds_label = フラグメント最小秒の持続時間
same_music_similarity_label = 最大差
music_compare_only_in_title_group = タイトルのみ比較
music_compare_only_in_title_group_tooltip =
    有効にすると、ファイルはタイトルでグループ化され、それから比較されます。
    
    10000ファイルでは、その代わりに約100万比較が通常、約20000比較があります。
same_music_tooltip =
    音楽ファイルの内容から類似ファイルを検索するように設定できます：
    
    - 音楽ファイルが類似していると識別されるフラグメントの最小時間
    - テストされた2つのフラグメントの最大差分
    
    良い結果を得るための鍵は、これらのパラメータの賢明な組み合わせを見つけることです。
    
    最小時間を5秒、最大差を1.0に設定すると、ファイル内のほとんど同じフラグメントを探します。
    一方、時間を20秒、差の最大値を6.0に設定すると、リミックスやライブ・バージョンなどを探すのに効果的です。
    
    デフォルトでは、各音楽ファイルは互いに比較され、多数のファイルをテストする場合、これは多くの時間を要します。したがって、通常、参照フォルダを使用し、どのファイルを互いに比較するかを指定する方が良いでしょう（同じ量のファイルでは、フィンガープリントの比較は参照フォルダなしよりも少なくとも 4 倍速くなります）。
music_comparison_checkbox_tooltip =
    機械学習によりフレーズから括弧とその中身を除外するAIを使用して、類似の音楽ファイルを検索します。このオプションが有効な場合、例えば以下のファイルは重複とみなされます:
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = 大文字小文字を区別する
duplicate_case_sensitive_name_tooltip =
    有効な場合、グループのみレコードまったく同じ名前を持っている場合など。 Z<unk> ołd <-> Z<unk> ołd
    
    このようなオプションを無効にすると、各文字のサイズが同じかどうかを確認せずに名前をグループ化します。例: z<unk> o<unk> D <-> Z<unk> ołd
duplicate_mode_size_name_combo_box = サイズと名前
duplicate_mode_name_combo_box = 名前
duplicate_mode_size_combo_box = サイズ
duplicate_mode_hash_combo_box = ハッシュ
duplicate_hash_type_tooltip =
    Czkawkaは3種類のハッシュを提供します:
    
    Blake3 - 暗号学的ハッシュ関数。非常に高速であるため、デフォルトのハッシュ方式として使用されます。
    
    CRC32 - シンプルなハッシュ関数。Blake3より高速ですが、まれに衝突が発生する可能性があります。
    
    XXH3 - パフォーマンスとハッシュの質がBlake3に非常に近いので、このようなモードの代わりに簡単に使用できます。（ただし、暗号学的ではありません）
duplicate_check_method_tooltip =
    Czkawkaは、今のところ以下の3種類の方法で重複を見つけることができます:
    
    名前 - 同じ名前のファイルを検索します。
    
    サイズ - 同じサイズのファイルを探します。
    
    ハッシュ - 同じ内容のファイルを探します。ファイルをハッシュ化して比較することにより重複を見つけます。このモードは、重複を見つけるための最も安全な方法です。このツールはキャッシュを多用するので、同じデータの2回目以降のスキャンは最初の時よりずっと速くなるはずです。
image_hash_size_tooltip =
    Each checked image produces a special hash which can be compared with each other, and a small difference between them means that these images are similar.
    
    8 hash size is quite good to find images that are only a little similar to original. With a bigger set of images (>1000), this will produce a big amount of false positives, so I recommend to use  a bigger hash size in this case.
    
    16 is the default hash size which is quite a good compromise between finding even a little similar images and having only a small amount of hash collisions.
    
    32 and 64 hashes find only very similar images, but should have almost no false positives (maybe except some images with alpha channel).
image_resize_filter_tooltip =
    To compute hash of image, the library must first resize it.
    
    Depend on chosen algorithm, the resulting image used to calculate hash will looks a little different.
    
    The fastest algorithm to use, but also the one which gives the worst results, is Nearest. It is enabled by default, because with 16x16 hash size lower quality it is not really visible.
    
    With 8x8 hash size it is recommended to use a different algorithm than Nearest, to have better groups of images.
image_hash_alg_tooltip =
    ハッシュの計算方法は、多くのアルゴリズムの中からユーザーが選択することができます。
    
    それぞれ長所と短所があり、画像によって良い結果が出る場合もあれば、悪い結果が出る場合もあります。
    
    そのため、最適なものを見極めるには、手動でのテストが必要です。
big_files_mode_combobox_tooltip = 最小/最大のファイルを検索できます
big_files_mode_label = チェックされたファイル
big_files_mode_smallest_combo_box = 最も小さい
big_files_mode_biggest_combo_box = 最大のもの
main_notebook_duplicates = 重複したファイル
main_notebook_empty_directories = 空のディレクトリ
main_notebook_big_files = 大きなファイル
main_notebook_empty_files = 空のファイル
main_notebook_temporary = 一時ファイル
main_notebook_similar_images = 類似の画像
main_notebook_similar_videos = 類似の動画
main_notebook_same_music = 重複した音楽
main_notebook_symlinks = 無効なシンボリックリンク
main_notebook_broken_files = 壊れたファイル
main_notebook_bad_extensions = 不正なエクステンション
main_tree_view_column_file_name = ファイル名
main_tree_view_column_folder_name = フォルダ名
main_tree_view_column_path = パス
main_tree_view_column_modification = 更新日時
main_tree_view_column_size = サイズ
main_tree_view_column_similarity = 類似度
main_tree_view_column_dimensions = 寸法
main_tree_view_column_title = タイトル
main_tree_view_column_artist = アーティスト
main_tree_view_column_year = 年
main_tree_view_column_bitrate = ビットレート
main_tree_view_column_length = 長さ
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = シンボリックリンクのファイル名
main_tree_view_column_symlink_folder = シンボリックリンクフォルダ
main_tree_view_column_destination_path = 宛先パス
main_tree_view_column_type_of_error = エラーの種類
main_tree_view_column_current_extension = 現在のエクステンション
main_tree_view_column_proper_extensions = 適切な拡張
main_label_check_method = メソッドのチェック
main_label_hash_type = ハッシュ方式
main_label_hash_size = ハッシュサイズ
main_label_size_bytes = サイズ(バイト)
main_label_min_size = 最小値
main_label_max_size = 最大値
main_label_shown_files = 表示するファイルの数
main_label_resize_algorithm = アルゴリズムのサイズを変更
main_label_similarity = 類似度{"   "}
main_check_box_broken_files_audio = 音声
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = アーカイブする
main_check_box_broken_files_image = Image
check_button_general_same_size = 同じサイズを無視
check_button_general_same_size_tooltip = 結果として同じサイズのファイルを無視 - 通常、これらは1:1重複です
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
upper_manual_add_included_button_tooltip =
    手動で検索するディレクトリ名を追加します。
    
    一度に複数のパスを追加するには、 ;
    
    /home/roman;/home/rozkazは/home/romanと/home/rozkazの2つのディレクトリを追加します。
upper_add_included_button_tooltip = 検索に新しいディレクトリを追加します。
upper_remove_included_button_tooltip = 検索からディレクトリを削除します。
upper_manual_add_excluded_button_tooltip =
    除外されたディレクトリ名を手動で追加します。
    
    一度に複数のパスを追加するには、 ;
    
    /home/roman;/home/krokiet は /home/roman と /home/keokiet の 2 つのディレクトリを追加します。
upper_add_excluded_button_tooltip = 検索で除外するディレクトリを追加します。
upper_remove_excluded_button_tooltip = 除外されたディレクトリを削除します。
upper_notebook_items_configuration = アイテム設定
upper_notebook_excluded_directories = 除外されたディレクトリ
upper_notebook_included_directories = 含まれるディレクトリ
upper_allowed_extensions_tooltip =
    許可する拡張子はカンマで区切る必要があります（デフォルトではすべてが使用可能です）。
    
    複数の拡張子を一度に追加するマクロ: IMAGE, VIDEO, MUSIC, TEXT も利用可能です。
    
    使用例: ".exe, IMAGE, VIDEO, .rar, 7z" - これは画像（jpg、pngなど）、動画（avi、mp4など）、exe、rar、7zファイルがスキャンされることを意味します。
upper_excluded_extensions_tooltip =
    スキャンで無視される無効なファイルの一覧です。
    
    許可された拡張子と無効化された拡張子の両方を使用する場合、この拡張子の方が優先度が高いので、ファイルはチェックされません。
upper_excluded_items_tooltip =
    除外された項目はワイルドカード * を含んでいる必要があり、カンマで区切る必要があります。
    ディレクトリを除外するよりも遅いので注意してください。
upper_excluded_items = 除外するアイテム:
upper_allowed_extensions = 許可される拡張子:
upper_excluded_extensions = 無効なエクステンション:
# Popovers
popover_select_all = すべて選択
popover_unselect_all = すべて選択解除
popover_reverse = 選択を逆にする
popover_select_all_except_oldest = 一番古いもの以外のすべてを選択
popover_select_all_except_newest = 一番新しいもの以外のすべてを選択
popover_select_one_oldest = 一番古いものを選択
popover_select_one_newest = 一番新しいものを選択
popover_select_custom = カスタム選択
popover_unselect_custom = カスタム選択を解除
popover_select_all_images_except_biggest = 一番大きいもの以外のすべてを選択
popover_select_all_images_except_smallest = 一番小さいもの以外のすべてを選択
popover_custom_path_check_button_entry_tooltip =
    パスによってレコードを選択します。
    
    使用例:
    /home/pimpek/rzecz.txt は /home/pim* で見つけることができます
popover_custom_name_check_button_entry_tooltip =
    ファイル名でレコードを選択します。
    
    使用例:
    /usr/ping/pong.txt は *ong* でを見つけることができます
popover_custom_regex_check_button_entry_tooltip =
    指定した正規表現でレコードを選択することができます。
    
    このモードでは、検索される文字列はパスと文字列になります。
    
    使用例:
    /usr/bin/ziemniak.txt は /ziem[a-z]+ で検索できます。
    
    これはRustのデフォルトの正規表現実装を使用しているので、詳しくは https://docs.rs/regex を参照してください。
popover_custom_case_sensitive_check_button_tooltip =
    大文字小文字を区別する検出を有効にします。
    
    /home/* を無効にすると、/Home/roman と /home/roman の両方が検出されます。
popover_custom_not_all_check_button_tooltip =
    グループ内のすべてのレコードを選択できないようにします。
    
    ほとんどの状況でユーザーは元のファイルと重複ファイルの両方を削除したくないため、これはデフォルトで有効になっています。 少なくとも1つのファイルを残したい。
    
    警告: この設定は、すでにユーザーがグループ内のすべての結果を手動で選択している場合には機能しません。
popover_custom_regex_path_label = パス
popover_custom_regex_name_label = 名前
popover_custom_regex_regex_label = 正規表現（パス + 名前）
popover_custom_case_sensitive_check_button = 大文字と小文字を区別
popover_custom_all_in_group_label = グループ内のすべてのレコードを選択しない
popover_custom_mode_unselect = カスタム選択を解除
popover_custom_mode_select = カスタム選択
popover_sort_file_name = ファイル名
popover_sort_folder_name = フォルダー名
popover_sort_full_name = カード名義人
popover_sort_size = サイズ
popover_sort_selection = 選択
popover_invalid_regex = 正規表現が無効です
popover_valid_regex = 正規表現が有効です
# Bottom buttons
bottom_search_button = 検索
bottom_select_button = 選択
bottom_delete_button = 削除
bottom_save_button = 保存
bottom_symlink_button = シンボリックリンク
bottom_hardlink_button = ハードリンク
bottom_move_button = 移動
bottom_sort_button = 並び替え
bottom_search_button_tooltip = 検索を開始
bottom_select_button_tooltip = レコードを選択します。選択したファイル/フォルダのみが後で処理できます。
bottom_delete_button_tooltip = 選択したファイル/フォルダを削除します。
bottom_save_button_tooltip = 検索に関するデータをファイルに保存します。
bottom_symlink_button_tooltip =
    シンボリックリンクを作成します。
    グループ内の2つ以上の結果が選択されている場合にのみ機能します。
    最初の結果は変更されず、2番目以降の結果が最初の結果にシンボリックリンクされます。
bottom_hardlink_button_tooltip =
    ハードリンクを作成します。
    グループ内の2つ以上の結果が選択されている場合にのみ機能します。
    最初の結果は変更されず、2番目以降の結果が最初の結果にハードリンクされます。
bottom_hardlink_button_not_available_tooltip =
    ハードリンクを作成する。
    ハードリンクを作成できないため、ボタンは無効になっています。
    ハードリンクはWindowsの管理者権限でのみ動作するので、アプリは必ず管理者として実行してください。
    アプリがすでにそのような権限で動作している場合は、Githubに同様の問題がないか確認してください。
bottom_move_button_tooltip =
    選択したフォルダにファイルを移動します。
    ディレクトリツリーを維持したまま、すべてのファイルをフォルダにコピーします。
    同じ名前の2つのファイルをフォルダに移動しようとすると、2番目のファイルが失敗し、エラーが表示されます。
bottom_sort_button_tooltip = 選択した方法に従ってファイル/フォルダを並べ替えます。
bottom_show_errors_tooltip = 下部のエラーパネルを表示/非表示にします。
bottom_show_upper_notebook_tooltip = 上部のノートブックパネルを表示/非表示にします。
# Progress Window
progress_stop_button = 停止
progress_stop_additional_message = リクエストを停止する
# About Window
about_repository_button_tooltip = ソースコードのあるリポジトリページへのリンク
about_donation_button_tooltip = 寄付ページへのリンク
about_instruction_button_tooltip = 使い方ページへのリンク
about_translation_button_tooltip = Crowdin ページにアプリの翻訳をリンクします。公式にポーランド語と英語がサポートされています。
about_repository_button = リポジトリ
about_donation_button = 寄付
about_instruction_button = 使い方
about_translation_button = 翻訳
# Header
header_setting_button_tooltip = 設定ダイアログを開きます。
header_about_button_tooltip = アプリに関する情報を含むダイアログを開きます。

# Settings


## General

settings_number_of_threads = 使用されるスレッドの数
settings_number_of_threads_tooltip = 使用するスレッドの数、0 は、使用可能なすべてのスレッドが使用されることを意味します。
settings_use_rust_preview = プレビューの読み込みにgtkの代わりに外部ライブラリを使用する
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = 設定を適用するにはアプリを再起動する必要があります！
settings_ignore_other_filesystems = 他のファイルシステムを無視(Linuxのみ)
settings_ignore_other_filesystems_tooltip =
    検索されたディレクトリと同じファイルシステムにないファイルを無視します。
    
    Linux の find コマンドで -xdev オプションのように動作します。
settings_save_at_exit_button_tooltip = 終了時に設定をファイルに保存します。
settings_load_at_start_button_tooltip =
    起動時にファイルから設定を読み込みます。
    
    このオプションが無効の場合、デフォルトの設定が使用されます。
settings_confirm_deletion_button_tooltip = 削除ボタンをクリックしたときに確認ダイアログを表示します。
settings_confirm_link_button_tooltip = ハードリンク/シンボリックリンクボタンをクリックしたときに確認ダイアログを表示します。
settings_confirm_group_deletion_button_tooltip = グループからすべてのレコードを削除しようとしたときに警告ダイアログを表示します。
settings_show_text_view_button_tooltip = 下部にテキストパネルを表示します。
settings_use_cache_button_tooltip = ファイルキャッシュを使用します。
settings_save_also_as_json_button_tooltip = キャッシュを人間にも読みやすい形のJSON形式で保存します。この内容は編集可能です。コンテンツを変更することができます。 バイナリ形式のキャッシュ（bin拡張子のもの）がない場合、このファイルから自動的にキャッシュが読み込まれます。
settings_use_trash_button_tooltip = ファイルを永久に削除する代わりにゴミ箱に移動します。
settings_language_label_tooltip = 操作画面における言語を選択します。
settings_save_at_exit_button = 終了時に設定を保存
settings_load_at_start_button = 起動時に設定を読み込む
settings_confirm_deletion_button = ファイルを削除するときに確認ダイアログを表示する
settings_confirm_link_button = ハードリンク/シンボリックリンク時に確認ダイアログを表示する
settings_confirm_group_deletion_button = グループ内のすべてのファイルを削除するときに確認ダイアログを表示する
settings_show_text_view_button = 下部にテキストパネルを表示
settings_use_cache_button = キャッシュを使用
settings_save_also_as_json_button = JSONファイルにもキャッシュを保存する
settings_use_trash_button = 削除したファイルをゴミ箱に移動する
settings_language_label = 言語
settings_multiple_delete_outdated_cache_checkbutton = 古いキャッシュエントリを自動的に削除
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    存在しないファイルを指している古いキャッシュエントリを削除できるようにします。
    
    このオプションを有効にすると、アプリはレコードを読み込むときにすべてのポイントが有効なファイルであることを確認します（壊れたファイルは無視されます）。
    
    無効にすると、それらに関するキャッシュエントリが次のスキャンで削除されなくなり、外部ドライブ上のファイルをスキャンする際に役立ちます。
    
    キャッシュに何十万ものレコードがある場合、スキャンの開始時・終了時のキャッシュの読み込み・保存を高速化するためにこのオプションを有効にすることが推奨されます。
settings_notebook_general = 全般
settings_notebook_duplicates = 重複
settings_notebook_images = 類似の画像
settings_notebook_videos = 類似の動画

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = 画像ファイルを選択しているとき、右側にプレビューを表示します。
settings_multiple_image_preview_checkbutton = 画像のプレビューを表示
settings_multiple_clear_cache_button_tooltip =
    古いキャッシュエントリを手動でクリアします。
    自動クリアが無効の場合にのみ使用する必要があります。
settings_multiple_clear_cache_button = キャッシュから古い結果を削除します。

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    ハードリンクされていてかつ同じデータを指している場合、1つを除くすべてのファイルを非表示にします。
    
    例： ディスク上に特定のデータにハードリンクされている同じデータを持つ7つのファイルと、1つの異なるinodeのファイルがある場合、 重複検索では一意のファイルとハードリンクされたファイルのみが表示されます。
settings_duplicates_minimal_size_entry_tooltip =
    キャッシュされるファイルの最小サイズを設定します。
    
    値を小さくするとキャッシュが生成されるレコードが増え検索が高速化しますが、キャッシュの読み込みと保存が遅くなります。
settings_duplicates_prehash_checkbutton_tooltip =
    プレハッシュ(ファイルの一部から計算したハッシュ) のキャッシュを有効にし、重複していない検索結果をより早く捨てられるようにします。
    
    いくつかの場面では低速化の要因になりうるので、この機能はデフォルトでは無効になっています。
    
    数十万・数百万のファイルをスキャンする場合には、検索を何倍も高速化できるため使用を強く推奨します。
settings_duplicates_prehash_minimal_entry_tooltip = キャッシュされたエントリの最小サイズ。
settings_duplicates_hide_hard_link_button = ハードリンクを非表示にする（LinuxとMacOSのみ）
settings_duplicates_prehash_checkbutton = プリハッシュキャッシュを使用
settings_duplicates_minimal_size_cache_label = キャッシュに保存するファイルの最小サイズ（バイト単位）
settings_duplicates_minimal_size_cache_prehash_label = プリハッシュキャッシュに保存するファイルの最小サイズ（バイト単位）

## Saving/Loading settings

settings_saving_button_tooltip = 現在の設定をファイルに保存します。
settings_loading_button_tooltip = 設定をファイルから読み込み、現在の設定を置き換えます。
settings_reset_button_tooltip = 設定をデフォルトにリセットします。
settings_saving_button = 設定を保存
settings_loading_button = 構成を読み込む
settings_reset_button = 設定をリセット

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    キャッシュを持つtxtファイルが保存されているフォルダを開きます。
    
    これらのファイルを変更すると不正な結果を表示することがありますが、パスなどを変更することで、大量のファイルを別の場所に移動する際の時間を短縮することができます。
    
    このファイルをコンピュータ間でコピーすることで、再度ファイルをスキャンするときの時間を節約できます（もちろん、コンピュータのディレクトリ構造が似ている場合に限り）。
    
    キャッシュに問題がある場合、このファイルを削除することができます。アプリは自動的にそれらを再生成します。
settings_folder_settings_open_tooltip =
    Czkawkaの設定ファイルが保存されているフォルダを開きます。
    
    警告: 手動で変更すると、ワークフローが壊れる可能性があります。
settings_folder_cache_open = キャッシュフォルダーを開く
settings_folder_settings_open = 設定フォルダを開く
# Compute results
compute_stopped_by_user = 検索はユーザーによって停止されました
compute_found_duplicates_hash_size = { $size } の { $number_groups } グループ内で { $number_files } 件の重複が見つかりました
compute_found_duplicates_name = { $number_groups } グループ内で { $number_files } 件の重複が見つかりました
compute_found_empty_folders = 空のフォルダが { $number_files } 個見つかりました
compute_found_empty_files = 空のファイルが { $number_files } 個見つかりました
compute_found_big_files = 大きなファイルが { $number_files } 個見つかりました
compute_found_temporary_files = 一時ファイルが { $number_files } 個見つかりました
compute_found_images = { $number_groups } グループで { $number_files } 件の類似した画像が見つかりました
compute_found_videos = { $number_groups } グループで { $number_files } 件の類似した動画が見つかりました
compute_found_music = { $number_groups } グループで { $number_files } 件の類似した音楽ファイルが見つかりました
compute_found_invalid_symlinks = 無効なシンボリックリンクが { $number_files } 個見つかりました
compute_found_broken_files = 壊れたファイルが { $number_files } 個見つかりました
compute_found_bad_extensions = 無効な拡張子を持つ { $number_files } 個のファイルが見つかりました
# Progress window
progress_scanning_general_file = { $file_number } ファイルをスキャン中
progress_scanning_extension_of_files = { $file_checked }/{ $all_files } ファイルの拡張子を確認中
progress_scanning_broken_files = { $file_checked }/{ $all_files } ファイルを確認中
progress_scanning_video = { $file_checked }/{ $all_files } ビデオのハッシュ
progress_scanning_image = { $file_checked }/{ $all_files } の画像のハッシュ
progress_comparing_image_hashes = { $file_checked }/{ $all_files } 画像ハッシュの比較
progress_scanning_music_tags_end = { $file_checked }/{ $all_files } 音楽ファイルのタグの比較
progress_scanning_music_tags = { $file_checked }/{ $all_files } 音楽ファイルのタグを読み込み中
progress_scanning_music_content_end = { $file_checked }/{ $all_files } 音楽ファイルのフィンガープリントの比較
progress_scanning_music_content = { $file_checked }/{ $all_files } 音楽ファイルのフィンガープリントを計算中
progress_scanning_empty_folders = { $folder_number } フォルダをスキャン中
progress_scanning_size = { $file_number } ファイルのサイズをスキャン中
progress_scanning_size_name = 名前と { $file_number } ファイルのサイズをスキャンしています
progress_scanning_name = { $file_number } ファイルの名前をスキャン中
progress_analyzed_partial_hash = { $file_checked }/{ $all_files } ファイルの部分ハッシュを分析中
progress_analyzed_full_hash = { $file_checked }/{ $all_files } ファイルの完全ハッシュを分析中
progress_prehash_cache_loading = プレハッシュキャッシュを読み込み中
progress_prehash_cache_saving = プレハッシュキャッシュを保存しています
progress_hash_cache_loading = ハッシュキャッシュを読み込み中
progress_hash_cache_saving = ハッシュキャッシュを保存中
progress_cache_loading = キャッシュを読み込み中
progress_cache_saving = キャッシュを保存中
progress_current_stage = 現在のステージ:{ "  " }
progress_all_stages = すべてのステージ:{ "  " }
# Saving loading 
saving_loading_saving_success = ファイル { $name } に設定を保存しました。
saving_loading_saving_failure = ファイル { $name } への設定データの保存に失敗しました。
saving_loading_reset_configuration = 現在の設定がクリアされました。
saving_loading_loading_success = 設定の読み込みが正常に完了しました。
saving_loading_invalid_string = キー "{ $key }" の場合、文字列ではない不正な結果 - "{ $result }" が見つかりました。
saving_loading_invalid_int = キー "{ $key }" の場合、整数ではない不正な結果 - "{ $result }" が見つかりました。
saving_loading_invalid_bool = キー "{ $key }" の場合、ブールではない不正な結果 - "{ $result }" が見つかりました。
saving_loading_decode_problem_bool = キー "{ $key }" から bool をデコードできませんでした。"{ $result }"が見つかりましたが、許可されている値は 0、1、true、または false です。
saving_loading_saving_same_keys = 重複したキー "{ $key }" で設定を保存しようとしています。
saving_loading_failed_to_get_home_directory = ホームディレクトリの取得に失敗したため、設定ファイルの読み込み/保存ができませんでした。
saving_loading_folder_config_instead_file = フォルダが既に存在するため、パス "{ $path }" に設定ファイルを作成または読み込み・保存することができません。
saving_loading_failed_to_create_configuration_folder = 設定フォルダ "{ $path }" の作成に失敗しました、理由 "{ $reason }" 。
saving_loading_failed_to_create_config_file = 設定ファイル "{ $path }" の作成に失敗しました、理由 "{ $reason }"。
saving_loading_failed_to_read_config_file = 存在しないか設定ファイルでないため、"{ $path }" から設定を読み込めません。
saving_loading_failed_to_read_data_from_file = ファイル "{ $path }" からデータを読み取ることができません、理由 "{ $reason } "。
saving_loading_orphan_data = { $line } 行目に孤立したデータ "{ $data }" が見つかりました。
saving_loading_not_valid = 設定 "{ $data }" は現在のバージョンのアプリには存在しません。
# Invalid symlinks
invalid_symlink_infinite_recursion = 無限再帰性
invalid_symlink_non_existent_destination = 保存先ファイルが存在しません
# Other
selected_all_reference_folders = すべてのディレクトリが参照フォルダとして設定されている場合、検索を開始できません
searching_for_data = データを検索中、しばらくお待ちください...
text_view_messages = メッセージ
text_view_warnings = 警告
text_view_errors = エラー
about_window_motto = このプログラムは自由に使用することができます、常に。
# Various dialog
dialogs_ask_next_time = 次回に確認
delete_file_failed = ファイル { $name } の削除に失敗しました、理由 { $reason }
delete_title_dialog = 削除の確認
delete_question_label = ファイルを削除してもよろしいですか？
delete_all_files_in_group_title = グループ内のすべてのファイルを削除することの確認
delete_all_files_in_group_label1 = いくつかのグループでは、すべてのレコードが選択されています。
delete_all_files_in_group_label2 = 本当に削除しますか？
delete_folder_failed = フォルダが存在しないため、フォルダ { $dir } の削除に失敗しました。権限がないか、そのフォルダは空ではありません。
delete_items_label = { $items } ファイルが削除されます。
delete_items_groups_label = { $groups } グループから { $items } 個のファイルが削除されます。
hardlink_failed = ハードリンクに失敗しました
hard_sym_invalid_selection_title_dialog = いくつかのグループで無効な選択です
hard_sym_invalid_selection_label_1 = いくつかのグループでは一つのレコードしか選択されていないため、それらは無視されます。
hard_sym_invalid_selection_label_2 = これらのファイルをハード/シンボリックにリンクできるようにするには、グループ内の少なくとも2つの結果を選択する必要があります。
hard_sym_invalid_selection_label_3 = グループ内で最初のものはオリジナルとして認識され変更されませんが、二つ目以降は変更されます。
hard_sym_link_title_dialog = リンクの確認
hard_sym_link_label = このファイルをリンクしてもよろしいですか？
move_folder_failed = フォルダ { $name } の移動に失敗しました、理由 { $reason }
move_file_failed = ファイル { $name } を移動できませんでした、理由 { $reason }
move_files_title_dialog = 重複したファイルの移動先フォルダを選択
move_files_choose_more_than_1_path = 重複したファイルをコピーするには、1つのパスのみを選択する必要があります、{ $path_number } つ選択されました。
move_stats = { $num_files }/{ $all_files } アイテムを適切に移動しました
save_results_to_file = txtファイルとjsonファイルの両方を { $name } フォルダに保存しました。
search_not_choosing_any_music = エラー: 音楽検索タイプのチェックボックスを少なくとも1つ選択する必要があります。
search_not_choosing_any_broken_files = エラー: チェックされた壊れたファイルの種類のチェックボックスを少なくとも1つ選択する必要があります。
include_folders_dialog_title = 含めるフォルダ
exclude_folders_dialog_title = 除外するフォルダ
include_manually_directories_dialog_title = ディレクトリを手動で追加
cache_properly_cleared = キャッシュを適切にクリアしました
cache_clear_duplicates_title = 重複したキャッシュをクリアする
cache_clear_similar_images_title = 類似した画像のキャッシュをクリア中
cache_clear_similar_videos_title = 類似した動画のキャッシュをクリア中
cache_clear_message_label_1 = 古いエントリのキャッシュを消去しますか？
cache_clear_message_label_2 = この操作は無効なファイルを指すすべてのキャッシュエントリを削除します。
cache_clear_message_label_3 = これはキャッシュへの読み込みと保存を少し高速化することがあります。
cache_clear_message_label_4 = 警告: 操作により、現在接続されていない外部ドライブからキャッシュされたすべてのデータが削除されます。そのため、それらのハッシュは再度生成する必要があります。
# Show preview
preview_image_resize_failure = 画像 { $name } のリサイズに失敗しました。
preview_image_opening_failure = イメージ { $name } を開けませんでした、理由 { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = グループ { $current_group }/{ $all_groups } ({ $images_in_group } 画像)
compare_move_left_button = L
compare_move_right_button = R
