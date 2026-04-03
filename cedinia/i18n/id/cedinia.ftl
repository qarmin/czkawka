# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Duplikat
tool_empty_folders = Folder Kosong
tool_similar_images = Gambar Serupa
tool_empty_files = Berkas Kosong
tool_temporary_files = Berkas Sementara
tool_big_files = File Terbesar
tool_broken_files = Berkas Rusak
tool_bad_extensions = Ekstensi yang Buruk
tool_same_music = Duplikat Musik
tool_bad_names = Nama yang Buruk
tool_exif_remover = Data EXIF
tool_directories = Direktori
tool_settings = Pengaturan
# Home screen tool card descriptions
home_dup_description = Temukan file yang memiliki konten yang sama
home_empty_folders_description = Direktori tanpa konten
home_similar_images_description = Temukan foto-foto yang secara visual mirip
home_empty_files_description = Berkas dengan ukuran nol
home_temp_files_description = Berkas sementara dan berkas yang disimpan dalam cache
home_big_files_description = File terbesar/terkecil di disk
home_broken_files_description = PDF, audio, gambar, arsip
home_bad_extensions_description = Berkas dengan ekstensi yang tidak valid
home_same_music_description = File audio serupa berdasarkan tag
home_bad_names_description = Berkas dengan karakter yang bermasalah dalam namanya
home_exif_description = Gambar dengan metadata EXIF
# Results list
scanning = Proses pemindaian sedang berlangsung...
stopping = Berhenti...
no_results = Tidak ada hasil
press_start = Tekan MULAI untuk memulai pemindaian
select_label = Pilih.
deselect_label = Desel.
list_label = Daftar
gallery_label = Gal.
# Selection popup
selection_popup_title = Pilih
select_all = Pilih Semua
select_except_one = Pilih semua kecuali satu
select_except_largest = Pilih semua kecuali yang terbesar
select_except_smallest = Pilih semua kecuali yang terkecil
select_largest = Pilih yang terbesar
select_smallest = Pilih yang terkecil
select_except_highest_res = Pilih semua kecuali resolusi tertinggi
select_except_lowest_res = Pilih semua kecuali resolusi terendah
select_highest_res = Pilih resolusi tertinggi
select_lowest_res = Pilih resolusi terendah
invert_selection = Balikkan pilihan
close = Tutup
# Deselection popup
deselection_popup_title = Batalkan pilihan
deselect_all = Batalkan semua pilihan
deselect_except_one = Batalkan pilihan semua kecuali satu
# Confirm popup
cancel = Batalkan
delete = Hapus
rename = Ubah nama
# Delete errors popup
delete_errors_title = Gagal menghapus beberapa file:
ok = OK
# Stopping overlay
stopping_overlay_title = Berhenti
stopping_overlay_body = 
        Penyelesaian pemindaian sedang berlangsung...
        Mohon tunggu.
# Permission popup
permission_title = Akses File
permission_body = Untuk memindai berkas, aplikasi memerlukan akses ke penyimpanan perangkat. Tanpa izin ini, proses pemindaian tidak akan dapat dilakukan.
grant = Hibah
no_permission_scan_warning = Tidak dapat mengakses berkas - berikan izin untuk melakukan pemindaian
# Settings screen tabs
settings_tab_general = Umum
settings_tab_tools = Alat
settings_tab_diagnostics = Informasi
# Settings - General tab
settings_use_cache = Gunakan cache
settings_use_cache_desc = Mempercepat pemindaian selanjutnya (hash/gambar)
settings_ignore_hidden = Abaikan file tersembunyi
settings_ignore_hidden_desc = File dan folder yang namanya diawali dengan '.'
settings_show_notification = Beritahu saya ketika pemindaian selesai
settings_show_notification_desc = Tampilkan notifikasi sistem saat pemindaian selesai
settings_notify_only_background = Hanya saat berada di latar belakang
settings_notify_only_background_desc = Lewati notifikasi jika aplikasi sedang terlihat
notifications_disabled_banner = Notifikasi dinonaktifkan
notifications_enable_button = Aktifkan
settings_scan_label = PINDAI
settings_filters_label = FILTER (beberapa alat)
settings_min_file_size = Ukuran file minimum
settings_max_file_size = Ukuran file maksimal
settings_language = Bahasa
settings_language_restart = Membutuhkan restart aplikasi
settings_common_label = PENGATURAN UMUM
settings_excluded_items = ITEM YANG DIKECUALIKAN (pola glob, dipisahkan dengan koma)
settings_excluded_items_placeholder = Contoh: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = EKSTENSI YANG DIIZINKAN (kosong = semua)
settings_allowed_extensions_placeholder = Contoh: jpg, png, mp4
settings_excluded_extensions = EKSTENSI YANG TIDAK TERMASUK
settings_excluded_extensions_placeholder = Contoh: bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = DUPLIKAT
settings_check_method_label = METODE PERBANDINGAN
settings_check_method = Metode
settings_hash_type_label = TIPE HASH
settings_hash_type = Jenis hash
settings_hash_type_desc = Blake3 - merupakan pilihan yang direkomendasikan, karena CRC32 memiliki kemungkinan kecil menghasilkan hasil positif palsu
settings_similar_images_header = GAMBAR YANG MIRIP
settings_similarity_preset = Ambang batas kesamaan
settings_similarity_desc = Sangat Tinggi = hanya sangat mirip
settings_hash_size = Ukuran hash
settings_hash_size_desc = Ukuran yang lebih besar cenderung menghasilkan lebih sedikit hasil positif palsu, tetapi juga menemukan lebih sedikit gambar yang serupa
settings_hash_alg = Algoritma hash
settings_image_filter = Ubah ukuran filter
settings_ignore_same_size = Abaikan gambar-gambar dengan dimensi yang sama
settings_gallery_image_fit_cover = Galeri: potong menjadi bentuk persegi
settings_gallery_image_fit_cover_desc = Isi seluruh area gambar; nonaktifkan fitur ini untuk mempertahankan rasio aspek asli
settings_big_files_header = FILE TERBESAR
settings_search_mode = Mode pencarian
settings_file_count = Jumlah berkas
settings_same_music_header = DUPLIKAT MUSIK
settings_music_check_method = Mode perbandingan
settings_music_compare_tags_label = TAG YANG TERKAIT
settings_music_title = Judul
settings_music_artist = Artis
settings_music_year = Tahun
settings_music_length = Panjang
settings_music_genre = Genre
settings_music_bitrate = Bitrate
settings_music_approx = Perbandingan tag perkiraan
settings_broken_files_header = FILE RUSAK
settings_broken_files_note = Pemindaian membutuhkan banyak sumber daya. Untuk performa terbaik, gunakan Krokiet di desktop.
settings_broken_files_types_label = JENIS YANG TELAH DIPERIKSA
settings_broken_audio = Audio
settings_broken_pdf = PDF
settings_broken_archive = Arsip
settings_broken_image = Gambar
settings_bad_names_header = NAMA YANG BURUK
settings_bad_names_checks_label = CEK
settings_bad_names_uppercase_ext = Ekstensi huruf kapital
settings_bad_names_emoji = Emoji dalam nama
settings_bad_names_space = Spasi di awal/akhir
settings_bad_names_non_ascii = Karakter non-ASCII
settings_bad_names_duplicated = Karakter yang berulang
# Settings - Diagnostics tab
diagnostics_header = DIAGNOSTIK
diagnostics_thumbnails = Cache gambar mini
diagnostics_app_cache = Cache aplikasi
diagnostics_refresh = Segarkan
diagnostics_clear_thumbnails = Cuplikan (thumbnail) yang jelas
diagnostics_open_thumbnails_folder = Buka folder
diagnostics_clear_cache = Hapus cache
diagnostics_open_cache_folder = Buka folder
diagnostics_collect_test = Uji akses berkas
diagnostics_collect_test_desc = Periksa berapa banyak berkas yang dapat diakses
diagnostics_collect_test_run = Jalankan
diagnostics_collect_test_stop = Berhenti
collect_test_cancelled = Dihentikan oleh pengguna
diag_confirm_clear_thumbnails = Hapus semua cache gambar mini?
diag_confirm_clear_cache = Hapus semua cache aplikasi?
about_repo = Repositori
about_translate = Terjemahan
about_donate = Dukungan
# Collect-test result popup
collect_test_title = Hasil pengujian
collect_test_volumes = Volume:
collect_test_folders = Folder:
collect_test_files = Berkas:
collect_test_time = Waktu:
# Licenses
licenses_label = LISENSÍ
third_party_licenses = Lisensi pihak ketiga
licenses_popup_title = Lisensi Pihak Ketiga
# Directories screen
directories_include_header = Sertakan
directories_included = Termasuk
directories_exclude_header = Kecualikan
directories_excluded_header = Dikecualikan
directories_add = Sertakan
no_paths = Tidak ada jalur - tambahkan di bawah ini
directories_volume_header = Volume
directories_volume_refresh = Segarkan
directories_volume_add = Tambahkan
# Bottom navigation
nav_home = Mulai
nav_dirs = Direktori
nav_settings = Pengaturan
# Status messages set from Rust
status_ready = Siap
status_stopped = Berhenti
status_no_results = Tidak ada hasil
status_deleted_selected = Telah dihapus yang dipilih
status_deleted_with_errors = Dihapus dengan kesalahan
scan_not_started = Pemindaian belum dimulai
found_items_prefix = Ditemukan
found_items_suffix = item-item
deleted_items_prefix = Dihapus
deleted_items_suffix = barang-barang
deleted_errors_suffix = kesalahan
renamed_prefix = Diganti namanya
renamed_files_suffix = berkas
renamed_errors_suffix = kesalahan
cleaned_exif_prefix = Menghapus data EXIF dari
cleaned_exif_suffix = berkas
cleaned_exif_errors_suffix = kesalahan
and_more_prefix = ...dan
and_more_suffix = lebih
# Gallery / delete popups
gallery_delete_button = Hapus
gallery_back = Kembali
gallery_confirm_delete = Ya, hapus
deleting_files = Menghapus berkas...
stop = Berhenti
files_suffix = berkas
scanning_fallback = Memindai...
app_subtitle = Sebagai penghormatan kepada Pertempuran Cedynia (tahun 972 Masehi)
app_license = Antarmuka pengguna untuk Czkawka Core - Lisensi GPL-3.0
about_app_label = TENTANG
cache_label = CACHE
# Notification
scan_completed_notification = Pemindaian selesai - { $file_count } item ditemukan
# Confirm popups (set from Rust)
confirm_clean_exif = Apakah Anda yakin ingin menghapus tag EXIF dari { $n } file yang dipilih?
confirm_delete_items = Apakah Anda yakin ingin menghapus { $n } item yang telah dipilih?
gallery_confirm_delete_msg = Anda akan menghapus { $total_images } gambar di { $total_groups } grup.
gallery_confirm_delete_warning = Semua item telah dipilih dari { $unsafe_groups } grup!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Menghitung dan membandingkan sidik jari audio membutuhkan banyak sumber daya dan mungkin memerlukan waktu yang lama. Disarankan untuk menggunakan Krokiet pada sistem desktop untuk tugas ini.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Mengumpulkan berkas
stage_scanning_name = Pencarian berdasarkan nama
stage_scanning_size_name = Pemindaian berdasarkan nama dan ukuran
stage_scanning_size = Pemindaian berdasarkan ukuran
stage_pre_hash = Pra-penghashan
stage_full_hash = Hashing
stage_loading_cache = Memuat cache
stage_saving_cache = Menyimpan data sementara
stage_calculating_image_hashes = Menghitung hash gambar
stage_comparing_images = Membandingkan gambar
stage_calculating_video_hashes = Menghitung hash video
stage_checking_files = Memeriksa berkas
stage_checking_extensions = Memeriksa ekstensi
stage_checking_names = Memeriksa nama
stage_reading_music_tags = Membaca tag musik
stage_comparing_tags = Membandingkan tag
stage_calculating_music_fingerprints = Menghitung sidik jari musik
stage_comparing_fingerprints = Membandingkan sidik jari
stage_extracting_exif = Membaca tag EXIF
stage_creating_video_thumbnails = Membuat thumbnail video
stage_processing_videos = Memproses video
stage_deleting = Menghapus berkas
stage_renaming = Mengganti nama file
stage_moving = Memindahkan berkas
stage_hardlinking = Membuat tautan keras
stage_symlinking = Membuat symlink
stage_optimizing_videos = Mengoptimalkan video
stage_cleaning_exif = Membersihkan data EXIF
# Group headers in scan results
duplicates_group_header = { $count } berkas x { $per_file } / berkas = { $total } total
similar_images_group_header = { $count } gambar serupa
same_music_group_header = { $count } trek serupa
# Rename confirmation
confirm_rename_items = Apakah Anda yakin ingin mengubah nama { $n } berkas yang dipilih?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = Terbesar
option_search_mode_smallest = Terkecil
option_similarity_very_high = V. Tinggi
option_similarity_high = Tinggi
option_similarity_medium = Sedang
option_similarity_low = Rendah
option_similarity_very_low = Sangat Rendah
option_similarity_minimal = Min.
option_check_method_hash = Hash
option_check_method_name = Nama
option_check_method_size_and_name = Ukuran + Nama
option_check_method_size = Ukuran
option_music_method_tags = Tag
option_music_method_audio = Audio
option_min_size_none = Tidak ada
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Tidak terbatas
# Volume labels (shown in the directories screen)
volume_internal_storage = Penyimpanan Internal
volume_sd_card = Kartu Memori (Kartu SD)
volume_storage = Volume Penyimpanan
# Directories screen
directories_referenced_tooltip = Dirujuk (tidak dihapus)
directories_include_section_header = TERMASUK
directories_exclude_section_header = DIKECUALIKAN
directories_custom_paths = Jalur Kustom
directories_check_button = Analisis
directories_check_popup_title = Statistik Direktori
directories_check_label_included = Jalur yang termasuk:
directories_check_label_excluded = Jalur yang dikecualikan:
directories_check_label_referenced = Jalur referensi:
directories_check_label_would_scan = File yang akan dipindai:
directories_check_label_processable = Berkas yang dapat diproses:
directories_check_scanning = Memindai...
directories_check_warning_no_processable = Tidak ditemukan file yang dapat diproses - pastikan folder yang termasuk/dikecualikan sudah benar
path_edit_title_include = Tambahkan ke Daftar Termasuk
path_edit_title_exclude = Tambahkan ke Daftar Pengecualian
path_edit_placeholder = Masukkan jalur...
path_edit_not_exists = Jalur tidak ditemukan
path_edit_is_dir = Direktori
path_edit_is_file = Berkas
path_edit_no_newlines = Jalur tidak boleh mengandung baris baru — tombol Enter tidak diperbolehkan
ctx_menu_title = Buka
ctx_open_file = Item terbuka
ctx_open_folder = Buka folder induk
