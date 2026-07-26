# Cedinia - English (fallback)

# App / top bar titles
tool_duplicate_files = 重複
tool_empty_folders = 空のフォルダ
tool_similar_images = 類似の画像
tool_empty_files = 空のファイル
tool_temporary_files = 一時ファイル
tool_big_files = 最大のファイル
tool_broken_files = 壊れたファイル
tool_bad_extensions = 不正な拡張子
tool_same_music = 音楽重複
tool_bad_names = 不正な名前
tool_exif_remover = EXIFデータ
tool_similar_videos = 類似のビデオ (音声)
tool_directories = ディレクトリ
tool_settings = 設定
# Home screen tool card descriptions
home_dup_description = 同じ内容のファイルを検索
home_empty_folders_description = コンテンツのないディレクトリ
home_similar_images_description = 視覚的に類似した写真を検索
home_empty_files_description = サイズゼロのファイル
home_temp_files_description = 一時ファイルとキャッシュ
home_big_files_description = ディスク上の最大/最小のファイル
home_broken_files_description = PDF、音声、画像、アーカイブ
home_bad_extensions_description = 無効な拡張子を持つファイル
home_same_music_description = タグによる類似のオーディオファイル
home_bad_names_description = 名前に問題のある文字を含むファイル
home_exif_description = EXIF メタデータを持つ画像
home_similar_videos_description = 音声が類似したビデオを検索する
# Results list
scanning = スキャン中...
stopping = 停止中...
no_results = 結果がありません
press_start = STARTを押してスキャンします
select_label = Sel.
deselect_label = Desel.
list_label = リスト
gallery_label = ギャラリー
# Selection popup
selection_popup_title = 選択
select_all = すべて選択
select_except_one = 1つ以外のすべてを選択
select_except_largest = 最大以外のすべてを選択
select_except_smallest = 最小以外のすべてを選択
select_largest = 最大を選択
select_smallest = 最小を選択
select_except_highest_res = 最高解像度以外のすべてを選択
select_except_lowest_res = 最低解像度以外のすべてを選択
select_highest_res = 最高解像度を選択
select_lowest_res = 最低解像度を選択
invert_selection = 選択を反転
close = 閉じる
# Deselection popup
deselection_popup_title = 選択を解除
deselect_all = すべての選択を解除
deselect_except_one = 1つ以外のすべての選択を解除
# Confirm popup
cancel = キャンセル
delete = 削除
rename = 名前の変更
# Delete errors popup
delete_errors_title = いくつかのファイルを削除できませんでした:
ok = 了解
# Stopping overlay
stopping_overlay_title = 停止中
stopping_overlay_body = 現在のスキャンを完了しています... お待ちください。
# Permission popup
permission_title = ファイルアクセス
permission_body = ファイルをスキャンするには、アプリがデバイスのストレージにアクセスする必要があります。この許可がないとスキャンはできません。
grant = 許可
no_permission_scan_warning = ファイルへのアクセス権限がありません - スキャンする権限を付与します
# Settings screen tabs
settings_tab_general = 全般
settings_tab_tools = ツール
settings_tab_diagnostics = 情報
# Settings - General tab
settings_use_cache = キャッシュを使用
settings_use_cache_desc = その後のスキャン速度を上げる（ハッシュ/画像）
settings_ignore_hidden = 隠しファイルを無視
settings_ignore_hidden_desc = '.'で始まるファイルとフォルダ
settings_show_notification = スキャン終了時に通知する
settings_show_notification_desc = スキャン完了時にシステム通知を表示する
settings_notify_only_background = バックグラウンドでのみ有効
settings_notify_only_background_desc = アプリが表示されている場合は通知をスキップ
notifications_disabled_banner = 通知が無効になっています
notifications_enable_button = 有効にする
settings_scan_label = スキャン
settings_filters_label = フィルター (一部のツール)
settings_min_file_size = 最小ファイルサイズ
settings_max_file_size = 最大ファイルサイズ
settings_language = 言語
settings_language_restart = アプリの再起動が必要です
settings_common_label = 共通設定
settings_excluded_items = 除外アイテム（グロブパターン、カンマ区切り）
settings_excluded_items_placeholder = 例: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = 許可された拡張子 (空 = すべて)
settings_allowed_extensions_placeholder = 例: jpg, png, mp4
settings_excluded_extensions = 除外された拡張子
settings_excluded_extensions_placeholder = 例: bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = 重複
settings_check_method_label = 比較方法
settings_check_method = 方法
settings_hash_type_label = ハッシュタイプ
settings_hash_type = ハッシュタイプ
settings_hash_type_desc = Blake3 - 推奨オプション、CRC32 は誤検出の可能性が小さいです
settings_similar_images_header = 類似画像
settings_similarity_preset = 類似度のしきい値
settings_similarity_desc = 非常に高い = 同一に近いのみ
settings_hash_size = ハッシュサイズ
settings_hash_size_desc = サイズが大きいほど誤検出は減りますが、見つかる類似画像の数も減ります
settings_hash_alg = ハッシュアルゴリズム
settings_image_filter = リサイズフィルター
settings_geometric_invariance = 幾何学的不変性
settings_ignore_same_size = 同じサイズの画像を無視
settings_gallery_image_fit_cover = ギャラリー: 正方形にトリミング
settings_gallery_image_fit_cover_desc = タイルいっぱいに表示; 元のアスペクト比を維持するには無効にする
settings_big_files_header = 最大のファイル
settings_search_mode = 検索モード
settings_file_count = ファイル数
settings_same_music_header = 音楽重複
settings_music_check_method = 比較モード
settings_music_compare_tags_label = 比較されたタグ
settings_music_title = タイトル
settings_music_artist = アーティスト
settings_music_year = 年
settings_music_length = 長さ
settings_music_genre = ジャンル
settings_music_bitrate = ビットレート
settings_music_approx = おおよそのタグ比較
settings_temporary_files_header = 一時ファイル
settings_temporary_files_extensions_label = 拡張子
settings_temporary_files_extensions_placeholder = 例：.tmp、.bak、~
settings_temporary_files_reset = 既定にリセット
settings_broken_files_header = 壊れたファイル
settings_broken_files_note = リソース集約的なスキャン。最高のパフォーマンスを得るためにデスクトップでKrokietを使用してください。
settings_broken_files_types_label = チェックした種類
settings_broken_audio = オーディオ
settings_broken_pdf = PDF
settings_broken_archive = アーカイブ
settings_broken_image = 画像
settings_broken_font = フォント
settings_broken_markup = マークアップ (JSON/XML/TOML)
settings_similar_videos_header = 類似ビデオ (音声)
settings_similar_videos_audio_preset = オーディオの類似性プリセット
settings_similar_videos_audio_preset_desc = オーディオがどの程度一致しなければならないかをコントロールします
settings_bad_names_header = 不正な名前
settings_bad_names_checks_label = チェック
settings_bad_names_uppercase_ext = 大文字の拡張子
settings_bad_names_emoji = 名前の絵文字
settings_bad_names_space = 先頭/末尾のスペース
settings_bad_names_non_ascii = 非ASCII文字
settings_bad_names_duplicated = 繰り返し文字
settings_ignore_same_resolution = 同じ解像度の画像を無視
# Settings - Appearance section
settings_appearance_label = 外観
settings_dark_theme = ダークテーマ
settings_dark_theme_desc = 暗い配色を使用
# Settings - Diagnostics tab
diagnostics_header = 診断
diagnostics_thumbnails = サムネイルキャッシュ
diagnostics_app_cache = アプリのキャッシュ
diagnostics_refresh = 更新
diagnostics_clear_thumbnails = サムネイルをクリア
diagnostics_open_thumbnails_folder = フォルダを開く
diagnostics_clear_cache = キャッシュをクリア
diagnostics_open_cache_folder = フォルダを開く
diagnostics_export_logs = ログをエクスポート
logs_label = ログ
logs_export_title = ログをエクスポート
logs_export_saved = ログのコピー先:
logs_export_failed = ログをエクスポートできませんでした
diagnostics_collect_test = ファイルアクセステスト
diagnostics_collect_test_desc = アクセス可能なファイルの数を確認する
diagnostics_collect_test_run = 実行
diagnostics_collect_test_stop = 停止
collect_test_cancelled = ユーザーによって停止されました
diag_confirm_clear_thumbnails = サムネイルキャッシュをすべてクリアしますか？
diag_confirm_clear_cache = アプリのキャッシュをすべてクリアしますか？
about_repo = リポジトリ
about_translate = 翻訳
about_donate = 寄付
# Collect-test result popup
collect_test_title = テスト結果
collect_test_volumes = ボリューム:
collect_test_folders = フォルダ:
collect_test_files = ファイル:
collect_test_time = 時間:
# Licenses
licenses_label = ライセンス
third_party_licenses = サードパーティのライセンス
licenses_popup_title = サードパーティライセンス
# Directories screen
directories_include_header = 含める
directories_included = 含む
directories_exclude_header = 除外
directories_excluded_header = 除外
directories_add = 含める
no_paths = パスなし - 以下に追加
directories_volume_header = ボリューム
directories_volume_refresh = 更新
directories_volume_add = 追加
# Bottom navigation
nav_home = ホーム
nav_dirs = ディレクトリ
nav_settings = 設定
# Status messages set from Rust
status_ready = 準備完了
status_stopped = 停止しました
status_no_results = 結果がありません
status_deleted_selected = 選択したアイテムを削除しました
status_deleted_with_errors = 削除完了（エラーあり）
scan_not_started = スキャンが開始されていません
found_items_prefix = 見つかりました
found_items_suffix = 項目
deleted_items_prefix = 削除しました
deleted_items_suffix = 項目
deleted_errors_suffix = エラー
renamed_prefix = 名前を変更
renamed_files_suffix = ファイル
renamed_errors_suffix = エラー
cleaned_exif_prefix = EXIF情報を削除しました
cleaned_exif_suffix = ファイル
cleaned_exif_errors_suffix = エラー
rename_error_read_file_name = ファイル名を読み取ることができません
rename_error_read_directory = ディレクトリを読み取ることができません
and_more_prefix = ...と
and_more_suffix = 他
# Gallery / delete popups
gallery_delete_button = 削除
gallery_back = 戻る
gallery_confirm_delete = はい、削除します
deleting_files = ファイルを削除しています...
stop = 停止
scanning_fallback = スキャン中...
app_subtitle = セデニアの戦い（972年）に敬意を込めて
app_license = Czkawka Core のフロントエンド-GPL-3.0
about_app_label = アプリについて
cache_label = キャッシュ
# Notification
scan_completed_notification = スキャンが完了しました - { $file_count } 個のアイテムが見つかりました
# Confirm popups (set from Rust)
confirm_clean_exif = 選択した { $n } 個のファイルから EXIF タグを削除してもよろしいですか？
confirm_delete_items = 選択した { $n } 個のアイテムを削除してもよろしいですか？
gallery_confirm_delete_msg = これから、{ $total_groups } のグループにある { $total_images } 枚の画像を削除します。
gallery_confirm_delete_warning = すべての項目が { $unsafe_groups } グループで選択されています！
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = オーディオフィンガープリントの計算と比較は非常にリソースがかかるため、時間がかかる場合があります。このタスクにはデスクトップシステムで Krokiet を使用することをお勧めします。
# Scan stage labels (shown during scan progress)
# Group headers in scan results
duplicates_group_header = { $count } 個 × { $per_file } / 個 = { $total } 合計
similar_images_group_header = { $count } 枚の類似画像
same_music_group_header = { $count } 類似のトラック
similar_videos_group_header = { $count } 件の類似ビデオ
# Rename confirmation
confirm_rename_items = 選択された { $n } 個のファイルを本当に名前を変更しますか？
# Combo-box option labels (translatable display names)
option_search_mode_biggest = 最大
option_search_mode_smallest = 最小
option_similarity_very_high = 非常に高い
option_similarity_high = 高い
option_similarity_medium = 中
option_similarity_low = 低い
option_similarity_very_low = 非常に低い
option_similarity_minimal = 最小
option_check_method_hash = ハッシュ
option_check_method_name = 名前
option_check_method_size_and_name = サイズ+名前
option_check_method_size = サイズ
option_music_method_tags = タグ
option_music_method_audio = オーディオ
option_min_size_none = なし
option_max_size_unlimited = 無制限
option_audio_preset_identical = 同じ
option_audio_preset_clip = 短い方が含まれる場合
option_audio_preset_similar = 類似
# Volume labels (shown in the directories screen)
volume_internal_storage = 内部ストレージ
volume_sd_card = メモリカード（SDカード）
volume_storage = ストレージボリューム
# Directories screen
directories_referenced_tooltip = 参照（削除されていません）
directories_include_section_header = 含まれる
directories_exclude_section_header = 除外される
directories_custom_paths = カスタムパス
directories_check_button = 分析
directories_check_popup_title = ディレクトリの統計
directories_check_label_included = 含まれるパス:
directories_check_label_excluded = 除外パス:
directories_check_label_referenced = 参照パス:
directories_check_label_would_scan = スキャンするファイル:
directories_check_label_processable = 処理可能なファイル:
directories_check_scanning = スキャン中...
directories_check_warning_no_processable = 処理可能なファイルが見つかりません - 含まれるフォルダ/除外フォルダを確認してください
path_edit_title_include = 含めるに追加
path_edit_title_exclude = 除外に追加
path_edit_placeholder = パスを入力...
path_edit_not_exists = パスが存在しません
path_edit_is_dir = ディレクトリ
path_edit_is_file = ファイル
path_edit_no_newlines = パスに改行を含めることはできません - Enterキーは使用できません
ctx_menu_title = 開く
ctx_open_file = アイテムを開く
ctx_open_folder = 親フォルダを開く
dir_open_folder = フォルダを開く
# Compare view
compare_label = 比較
compare_loading = 画像を読み込み中...
compare_cancelling = キャンセル中...
compare_computing = 差分を計算しています...
compare_mode_normal = サイド
compare_mode_split = 分割
compare_mode_overlay = オーバーレイ
compare_mode_diff = 差分
compare_res_mismatch = 異なる解像度 - 差分が不正確である可能性があります
