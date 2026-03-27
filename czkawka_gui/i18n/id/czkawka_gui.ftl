# Window titles
window_settings_title = Pengaturan
window_main_title = Czwakwa (Hiccup)
window_progress_title = Memindai
window_compare_images = Bandingkan Gambar
# General
general_ok_button = Oke
general_close_button = Tutup
# Krokiet info dialog
krokiet_info_title = Memperkenalkan Krokiet - Versi terbaru dari Czkawka
krokiet_info_message = 
        Krokiet adalah versi baru, yang lebih baik, lebih cepat, dan lebih andal dari GUI Czkawka GTK!

        Krokiet lebih mudah dijalankan dan lebih tahan terhadap perubahan sistem, karena hanya bergantung pada pustaka inti yang tersedia secara default di sebagian besar sistem.

        Krokiet juga menghadirkan fitur-fitur yang tidak ada di Czkawka, termasuk thumbnail dalam mode perbandingan video, pembersih EXIF, indikator progres untuk memindahkan/menyalin/menghapus file, atau opsi pengurutan yang lebih lengkap.

        Cobalah dan lihat perbedaannya!

        Czkawka akan terus menerima perbaikan bug dan pembaruan kecil dari saya, tetapi semua fitur baru akan dikembangkan secara eksklusif untuk Krokiet, dan siapa pun bebas untuk berkontribusi dengan menambahkan fitur baru, mode yang hilang, atau memperluas Czkawka lebih lanjut.

        PS: Pesan ini seharusnya hanya muncul sekali. Jika muncul lagi, atur variabel lingkungan CZKAWKA_DONT_ANNOY_ME ke nilai apa pun yang tidak kosong.
# Main window
music_title_checkbox = Judul
music_artist_checkbox = Artis
music_year_checkbox = Tahun
music_bitrate_checkbox = Bitrate
music_genre_checkbox = Genre
music_length_checkbox = Panjang
music_comparison_checkbox = Perbandingan Kasar
music_checking_by_tags = Tag
music_checking_by_content = Konten
same_music_seconds_label = Fragmen minimal, durasi kedua
same_music_similarity_label = Perbedaan maksimum
music_compare_only_in_title_group = Bandingkan berdasarkan kelompok judul yang serupa
music_compare_only_in_title_group_tooltip = 
        Saat diaktifkan, berkas-berkas dikelompokkan berdasarkan judul, kemudian dibandingkan satu sama lain.

        Dengan 10.000 berkas, alih-alih hampir 100 juta perbandingan yang biasanya terjadi, akan ada sekitar 20.000 perbandingan.
same_music_tooltip = 
        Mencari file musik yang serupa berdasarkan isinya dapat dikonfigurasi dengan mengatur:

        - Durasi minimum fragmen setelahnya file musik dapat dianggap serupa.
        - Perbedaan maksimum antara dua fragmen yang diuji.

        Kunci untuk mendapatkan hasil yang baik adalah menemukan kombinasi parameter yang tepat, seperti yang dijelaskan di bawah ini.

        Mengatur durasi minimum menjadi 5 detik dan perbedaan maksimum menjadi 1.0 akan mencari fragmen yang hampir identik dalam file.
        Sementara itu, mengatur durasi menjadi 20 detik dan perbedaan maksimum menjadi 6.0, cocok untuk menemukan remix/versi live, dll.

        Secara default, setiap file musik dibandingkan dengan file lainnya, dan ini bisa memakan waktu lama saat menguji banyak file. Oleh karena itu, biasanya lebih baik menggunakan folder referensi dan menentukan file mana yang akan dibandingkan (dengan jumlah file yang sama, perbandingan sidik jari akan lebih cepat setidaknya 4 kali lipat dibandingkan tanpa folder referensi).
music_comparison_checkbox_tooltip = 
        Aplikasi ini mencari file musik yang serupa menggunakan kecerdasan buatan (AI), yang menggunakan pembelajaran mesin untuk menghilangkan tanda kurung dari sebuah frasa. Misalnya, dengan opsi ini diaktifkan, file-file berikut akan dianggap sebagai duplikat:

        Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = Sensitif terhadap huruf besar/kecil
duplicate_case_sensitive_name_tooltip = 
        Saat diaktifkan, pengelompokan hanya dilakukan untuk catatan yang memiliki nama yang persis sama, contohnya: Żołd <-> Żołd.

        Menonaktifkan opsi ini akan mengelompokkan nama tanpa memeriksa apakah setiap huruf memiliki ukuran yang sama, contohnya: żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = Ukuran dan Nama
duplicate_mode_name_combo_box = Nama
duplicate_mode_size_combo_box = Ukuran
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip = 
        Czkawka menawarkan 3 jenis fungsi hash:

        Blake3 - fungsi hash kriptografis. Ini adalah pengaturan bawaan karena sangat cepat.

        CRC32 - fungsi hash sederhana. Ini seharusnya lebih cepat daripada Blake3, tetapi mungkin sangat jarang terjadi beberapa tabrakan.

        XXH3 - memiliki kinerja dan kualitas hash yang sangat mirip dengan Blake3 (tetapi bukan fungsi hash kriptografis). Oleh karena itu, mode-mode ini dapat dengan mudah dipertukarkan.
duplicate_check_method_tooltip =
    Sementara, Czkawka mendukung tiga tipe cara untuk mencari file duplikat:
    
    Nama - Mencari file yang memiliki nama yang sama.
    
    Ukuran - Mencari file yang memiliki ukuran yang sama.
    
    Hash - Mencari file yang memiliki konten yang sama. Mode ini melakukan hash pada file dan membandingkannya untuk mencari file yang terduplikat. Ini adalah mode paling aman untuk mencari file duplikat. Aplikasi menyimpan cache yang besar, sehingga jika melakukan pemindaian ulang terhadap file yang sama maka akan relatif lebih cepat.
image_hash_size_tooltip = 
        Setiap gambar yang diperiksa menghasilkan hash khusus yang dapat dibandingkan satu sama lain, dan perbedaan kecil di antara mereka berarti gambar-gambar tersebut serupa.

        Ukuran hash 8 cukup baik untuk menemukan gambar yang hanya sedikit mirip dengan gambar aslinya. Dengan kumpulan gambar yang lebih besar (>1000), ini akan menghasilkan banyak hasil positif palsu, jadi saya merekomendasikan untuk menggunakan ukuran hash yang lebih besar dalam kasus ini.

        16 adalah ukuran hash default yang merupakan kompromi yang cukup baik antara menemukan gambar yang sedikit mirip dan memiliki jumlah tabrakan hash yang kecil.

        Hash 32 dan 64 hanya menemukan gambar yang sangat mirip, tetapi seharusnya menghasilkan hampir tidak ada hasil positif palsu (mungkin kecuali beberapa gambar dengan saluran alpha).
image_resize_filter_tooltip = 
        Untuk menghitung hash dari sebuah gambar, pustaka harus terlebih dahulu mengubah ukurannya.

        Tergantung pada algoritma yang dipilih, gambar yang dihasilkan yang digunakan untuk menghitung hash akan terlihat sedikit berbeda.

        Algoritma yang paling cepat digunakan, tetapi juga yang menghasilkan hasil terburuk, adalah Nearest. Algoritma ini diaktifkan secara default, karena dengan ukuran hash 16x16, perbedaan kualitasnya tidak terlalu terlihat.

        Dengan ukuran hash 8x8, disarankan untuk menggunakan algoritma selain Nearest, untuk mendapatkan pengelompokan gambar yang lebih baik.
image_hash_alg_tooltip = 
        Pengguna dapat memilih dari berbagai algoritma untuk menghitung hash.

        Setiap algoritma memiliki kelebihan dan kekurangan masing-masing, dan terkadang memberikan hasil yang lebih baik, sementara di lain waktu memberikan hasil yang kurang optimal, tergantung pada gambar yang digunakan.

        Oleh karena itu, untuk menentukan algoritma yang paling sesuai untuk Anda, diperlukan pengujian manual.
big_files_mode_combobox_tooltip = Memungkinkan untuk mencari file terkecil/terbesar
big_files_mode_label = Berkas yang telah diperiksa
big_files_mode_smallest_combo_box = Yang Terkecil
big_files_mode_biggest_combo_box = Yang Terbesar
main_notebook_duplicates = Berkas Duplikat
main_notebook_empty_directories = Direktori Kosong
main_notebook_big_files = Berkas Besar
main_notebook_empty_files = Berkas Kosong
main_notebook_temporary = Berkas Sementara
main_notebook_similar_images = Gambar Serupa
main_notebook_similar_videos = Video Serupa
main_notebook_same_music = Duplikat Musik
main_notebook_symlinks = Tautan Simbolik Tidak Valid
main_notebook_broken_files = Berkas Rusak
main_notebook_bad_extensions = Ekstensi yang Buruk
main_tree_view_column_file_name = Nama Berkas
main_tree_view_column_folder_name = Nama Folder
main_tree_view_column_path = Jalur
main_tree_view_column_modification = Tangga Modifikasi
main_tree_view_column_size = Ukuran
main_tree_view_column_similarity = Kesamaan
main_tree_view_column_dimensions = Dimensi
main_tree_view_column_title = Judul
main_tree_view_column_artist = Artis
main_tree_view_column_year = Tahun
main_tree_view_column_bitrate = Bitrate
main_tree_view_column_length = Panjang
main_tree_view_column_genre = Genre
main_tree_view_column_symlink_file_name = Nama File Simbolik (Symlink)
main_tree_view_column_symlink_folder = Simbolik Link Folder
main_tree_view_column_destination_path = Jalur Tujuan
main_tree_view_column_type_of_error = Jenis Kesalahan
main_tree_view_column_current_extension = Ekstensi Saat Ini
main_tree_view_column_proper_extensions = Ekstensi yang Tepat
main_tree_view_column_fps = FPS
main_tree_view_column_codec = Codec
main_label_check_method = Metode cek
main_label_hash_type = Jenis hash
main_label_hash_size = Ukuran hash
main_label_size_bytes = Ukuran (byte)
main_label_min_size = Min
main_label_max_size = Maks
main_label_shown_files = Jumlah berkas yang ditampilkan
main_label_resize_algorithm = Algoritma perubahan ukuran
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = Audio
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = Arsip
main_check_box_broken_files_image = Gambar
main_check_box_broken_files_video = Video
main_check_box_broken_files_video_tooltip = Menggunakan ffmpeg/ffprobe untuk memvalidasi berkas video. Prosesnya cukup lambat dan mungkin mendeteksi kesalahan-kesalahan kecil, bahkan jika berkas video tersebut dapat diputar dengan baik.
check_button_general_same_size = Abaikan ukuran sama
check_button_general_same_size_tooltip = Abaikan file dengan ukuran yang sama dalam hasil pencarian - biasanya ini adalah duplikat yang identik
main_label_size_bytes_tooltip = Ukuran berkas yang akan digunakan dalam proses pemindaian
# Upper window
upper_tree_view_included_folder_column_title = Folder yang Akan Dicari
upper_tree_view_included_reference_column_title = Folder Referensi
upper_recursive_button = Rekursif
upper_recursive_button_tooltip = Jika opsi ini dipilih, pencarian juga akan mencakup file-file yang tidak terletak langsung di dalam folder yang dipilih.
upper_manual_add_included_button = Tambah Manual
upper_add_included_button = Tambahkan
upper_remove_included_button = Hapus
upper_manual_add_excluded_button = Tambah Manual
upper_add_excluded_button = Tambahkan
upper_remove_excluded_button = Hapus
upper_manual_add_included_button_tooltip = 
        Tambahkan nama direktori secara manual untuk pencarian.

        Untuk menambahkan beberapa jalur sekaligus, pisahkan dengan tanda ";".

        `/home/roman;/home/rozkaz` akan menambahkan dua direktori, yaitu `/home/roman` dan `/home/rozkaz`
upper_add_included_button_tooltip = Tambahkan direktori baru ke daftar pencarian.
upper_remove_included_button_tooltip = Hapus direktori dari pencarian.
upper_manual_add_excluded_button_tooltip = 
        Tambahkan nama direktori yang dikecualikan secara manual.

        Untuk menambahkan beberapa jalur sekaligus, pisahkan dengan tanda ";".

        `/home/roman;/home/krokiet` akan menambahkan dua direktori: `/home/roman` dan `/home/keokiet`
upper_add_excluded_button_tooltip = Tambahkan direktori yang akan dikecualikan dari pencarian.
upper_remove_excluded_button_tooltip = Hapus direktori dari daftar yang dikecualikan.
upper_notebook_items_configuration = Konfigurasi Item
upper_notebook_excluded_directories = Jalur yang Dikecualikan
upper_notebook_included_directories = Jalur yang Termasuk
upper_allowed_extensions_tooltip = 
        Ekstensi yang diizinkan harus dipisahkan dengan koma (secara default, semua tersedia).

        Makro berikut, yang menambahkan beberapa ekstensi sekaligus, juga tersedia: IMAGE, VIDEO, MUSIC, TEXT.

        Contoh penggunaan: ".exe, IMAGE, VIDEO, .rar, 7z" - ini berarti bahwa file gambar (misalnya, jpg, png), video (misalnya, avi, mp4), file exe, rar, dan 7z akan dipindai.
upper_excluded_extensions_tooltip = 
        Daftar berkas yang dinonaktifkan dan akan diabaikan selama pemindaian.

        Ketika menggunakan baik ekstensi yang diizinkan maupun yang dinonaktifkan, yang dinonaktifkan memiliki prioritas lebih tinggi, sehingga berkas tidak akan diperiksa.
upper_excluded_items_tooltip = 
        Item-item yang dikecualikan harus mengandung karakter "*wildcard*" dan dipisahkan dengan koma.
        Ini lebih lambat dibandingkan dengan "Excluded Paths", jadi gunakan dengan hati-hati.
upper_excluded_items = Barang yang Tidak Termasuk:
upper_allowed_extensions = Ekstensi yang Diizinkan:
upper_excluded_extensions = Ekstensi yang Dinonaktifkan:
# Popovers
popover_select_all = Pilih Semua
popover_unselect_all = Batal Pilih
popover_reverse = Seleksi Terbalik
popover_select_all_except_shortest_path = Pilih semua kecuali jalur terpendek
popover_select_all_except_longest_path = Pilih semua kecuali jalur terpanjang
popover_select_all_except_oldest = Pilih semua kecuali yang paling tua
popover_select_all_except_newest = Pilih semua kecuali yang terbaru
popover_select_one_oldest = Pilih salah satu yang tertua
popover_select_one_newest = Pilih salah satu yang terbaru
popover_select_custom = Pilih kustom
popover_unselect_custom = Batalkan pilihan khusus
popover_select_all_images_except_biggest = Pilih semua kecuali yang terbesar
popover_select_all_images_except_smallest = Pilih semua kecuali yang terkecil
popover_custom_path_check_button_entry_tooltip = 
        Pilih catatan berdasarkan jalur.

        Contoh penggunaan:
        File "/home/pimpek/rzecz.txt" dapat ditemukan dengan menggunakan "/home/pim*"
popover_custom_name_check_button_entry_tooltip = 
        Pilih catatan berdasarkan nama file.

        Contoh penggunaan:
        File `/usr/ping/pong.txt` dapat ditemukan dengan menggunakan pola `*ong*`
popover_custom_regex_check_button_entry_tooltip = 
        Pilih catatan berdasarkan ekspresi reguler (Regex) yang ditentukan.

        Dalam mode ini, teks yang dicari adalah Jalur (Path) dengan Nama.

        Contoh penggunaan:
        File `/usr/bin/ziemniak.txt` dapat ditemukan dengan ekspresi `/ziem[a-z]+`.

        Ini menggunakan implementasi ekspresi reguler (regex) bawaan Rust. Anda dapat membaca lebih lanjut tentangnya di sini: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip = 
        Mengaktifkan deteksi yang memperhatikan huruf besar/kecil.

        Ketika dinonaktifkan, `/home/*` akan menemukan baik `/HoMe/roman` maupun `/home/roman`.
popover_custom_not_all_check_button_tooltip = 
        Mencegah pemilihan semua catatan dalam suatu grup.

        Fitur ini aktif secara bawaan, karena dalam banyak situasi, Anda tidak ingin menghapus baik file asli maupun file duplikat, melainkan ingin menyisakan setidaknya satu file.

        PERINGATAN: Pengaturan ini tidak berfungsi jika Anda telah memilih semua hasil secara manual dalam suatu grup.
popover_custom_regex_path_label = Jalur
popover_custom_regex_name_label = Nama
popover_custom_regex_regex_label = Jalur Regex + Nama
popover_custom_case_sensitive_check_button = Sensitif terhadap huruf besar/kecil
popover_custom_all_in_group_label = Jangan pilih semua catatan dalam grup
popover_custom_mode_unselect = Batalkan pilihan Kustom
popover_custom_mode_select = Pilih Kustom
popover_sort_file_name = Nama file
popover_sort_folder_name = Nama folder
popover_sort_full_name = Nama lengkap
popover_sort_size = Ukuran
popover_sort_selection = Pemilihan
popover_invalid_regex = Ekspresi reguler tidak valid
popover_valid_regex = Ekspresi reguler valid
# Bottom buttons
bottom_search_button = Cari
bottom_select_button = Pilih
bottom_delete_button = Hapus
bottom_save_button = Simpan
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = Pindah
bottom_sort_button = Urutkan
bottom_compare_button = Bandingkan
bottom_search_button_tooltip = Mulai pencarian
bottom_select_button_tooltip = Pilih rekaman. Hanya berkas/folder yang dipilih yang dapat diproses selanjutnya.
bottom_delete_button_tooltip = Hapus berkas/folder yang dipilih.
bottom_save_button_tooltip = Simpan data tentang pencarian ke dalam file
bottom_symlink_button_tooltip = 
        Buat tautan simbolik.
        Hanya berfungsi jika setidaknya dua hasil dalam suatu grup dipilih.
        Hasil pertama tidak akan berubah, sedangkan hasil kedua dan seterusnya akan menjadi tautan simbolik ke hasil pertama.
bottom_hardlink_button_tooltip = 
        Buat tautan keras.
        Hanya berfungsi jika setidaknya dua hasil dipilih dalam suatu grup.
        Hasil pertama tidak berubah, dan hasil kedua dan seterusnya akan ditautkan secara keras ke hasil pertama.
bottom_hardlink_button_not_available_tooltip = 
        Buat tautan keras.
        Tombol ini dinonaktifkan, karena tautan keras tidak dapat dibuat.
        Tautan keras hanya berfungsi dengan hak administrator di Windows, jadi pastikan untuk menjalankan aplikasi sebagai administrator.
        Jika aplikasi sudah berjalan dengan hak tersebut, periksa masalah serupa di Github.
bottom_move_button_tooltip = 
        Memindahkan berkas ke direktori yang dipilih.
        Berkas-berkas tersebut disalin ke direktori tanpa mempertahankan struktur direktori aslinya.
        Jika mencoba memindahkan dua berkas dengan nama yang sama ke folder, proses untuk berkas kedua akan gagal dan menampilkan pesan kesalahan.
bottom_sort_button_tooltip = Mengurutkan berkas/folder berdasarkan metode yang dipilih.
bottom_compare_button_tooltip = Bandingkan gambar-gambar dalam grup ini.
bottom_show_errors_tooltip = Tampilkan/Sembunyikan panel teks di bagian bawah.
bottom_show_upper_notebook_tooltip = Tampilkan/Sembunyikan panel notebook atas.
# Progress Window
progress_stop_button = Berhenti
progress_stop_additional_message = Permintaan berhenti telah diterima
# About Window
about_repository_button_tooltip = Tautan ke halaman repositori yang berisi kode sumber.
about_donation_button_tooltip = Tautan ke halaman donasi.
about_instruction_button_tooltip = Tautan ke halaman instruksi.
about_translation_button_tooltip = Tautan ke halaman Crowdin yang berisi terjemahan aplikasi. Secara resmi, bahasa Polandia dan Inggris didukung.
about_repository_button = Repositori
about_donation_button = Donasi
about_instruction_button = Instruksi
about_translation_button = Terjemahan
# Header
header_setting_button_tooltip = Membuka dialog pengaturan.
header_about_button_tooltip = Membuka dialog yang berisi informasi tentang aplikasi.

# Settings


## General

settings_number_of_threads = Jumlah thread yang digunakan
settings_number_of_threads_tooltip = Jumlah thread yang digunakan, 0 berarti semua thread yang tersedia akan digunakan.
settings_use_rust_preview = Gunakan pustaka eksternal sebagai pengganti gtk untuk memuat pratinjau
settings_use_rust_preview_tooltip = 
        Menggunakan pratinjau berbasis gtk terkadang bisa lebih cepat dan mendukung lebih banyak format, tetapi terkadang hal ini justru bisa menjadi sebaliknya.

        Jika Anda mengalami masalah saat memuat pratinjau, Anda mungkin perlu mencoba mengubah pengaturan ini.

        Pada sistem non-Linux, disarankan untuk menggunakan opsi ini, karena gtk-pixbuf tidak selalu tersedia di sana, sehingga menonaktifkan opsi ini akan mencegah pemuatan pratinjau untuk beberapa gambar.
settings_label_restart = Anda perlu memulai ulang aplikasi untuk menerapkan pengaturan!
settings_ignore_other_filesystems = Abaikan sistem berkas lainnya (hanya untuk Linux)
settings_ignore_other_filesystems_tooltip = 
        Mengabaikan berkas yang tidak berada dalam sistem berkas yang sama dengan direktori yang dicari.

        Berfungsi sama seperti opsi -xdev pada perintah `find` di Linux
settings_save_at_exit_button_tooltip = Simpan konfigurasi ke file saat aplikasi ditutup.
settings_load_at_start_button_tooltip = 
        Muat konfigurasi dari file saat aplikasi dibuka.

        Jika fitur ini tidak diaktifkan, pengaturan bawaan akan digunakan.
settings_confirm_deletion_button_tooltip = Tampilkan dialog konfirmasi saat tombol hapus diklik.
settings_confirm_link_button_tooltip = Tampilkan dialog konfirmasi saat tombol "hard link" atau "symlink" diklik.
settings_confirm_group_deletion_button_tooltip = Tampilkan dialog peringatan ketika pengguna mencoba menghapus semua catatan dari grup tersebut.
settings_show_text_view_button_tooltip = Tampilkan panel teks di bagian bawah antarmuka pengguna.
settings_use_cache_button_tooltip = Gunakan *file cache*.
settings_save_also_as_json_button_tooltip = Simpan *cache* ke dalam format JSON (yang mudah dibaca oleh manusia). Anda dapat memodifikasi isinya. *Cache* dari file ini akan dibaca secara otomatis oleh aplikasi jika *cache* dalam format biner (dengan ekstensi .bin) tidak tersedia.
settings_use_trash_button_tooltip = Memindahkan berkas ke tempat sampah, bukan menghapusnya secara permanen.
settings_language_label_tooltip = Bahasa untuk antarmuka pengguna.
settings_save_at_exit_button = Simpan konfigurasi saat aplikasi ditutup
settings_load_at_start_button = Muat konfigurasi saat aplikasi dibuka
settings_confirm_deletion_button = Tampilkan kotak dialog konfirmasi saat menghapus file apa pun
settings_confirm_link_button = Tampilkan dialog konfirmasi saat membuat tautan keras/simbolik ke file apa pun
settings_confirm_group_deletion_button = Tampilkan dialog konfirmasi saat menghapus semua file dalam grup
settings_show_text_view_button = Tampilkan panel teks bagian bawah
settings_use_cache_button = Gunakan *cache*
settings_save_also_as_json_button = Simpan juga cache sebagai file JSON
settings_use_trash_button = Pindahkan berkas yang dihapus ke tempat sampah
settings_language_label = Bahasa
settings_multiple_delete_outdated_cache_checkbutton = Hapus entri cache yang sudah kedaluwarsa secara otomatis
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        Hapus hasil cache yang sudah kedaluwarsa dan mengarah ke file yang tidak ada.

        Saat diaktifkan, aplikasi memastikan bahwa saat memuat data, semua data mengarah ke file yang valid (data yang rusak akan diabaikan).

        Menonaktifkan fitur ini akan membantu saat memindai file di drive eksternal, sehingga entri cache tentang file tersebut tidak akan dihapus pada pemindaian berikutnya.

        Jika Anda memiliki ratusan ribu data dalam cache, disarankan untuk mengaktifkan fitur ini, karena akan mempercepat proses pemuatan/penyimpanan cache di awal/akhir pemindaian.
settings_notebook_general = Umum
settings_notebook_duplicates = Duplikat
settings_notebook_images = Gambar Serupa
settings_notebook_videos = Video Serupa

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Menampilkan pratinjau di sisi kanan (saat memilih berkas gambar).
settings_multiple_image_preview_checkbutton = Tampilkan pratinjau gambar
settings_multiple_clear_cache_button_tooltip = 
        Bersihkan cache secara manual dari entri yang sudah kedaluwarsa.
        Ini hanya boleh digunakan jika penghapusan otomatis telah dinonaktifkan.
settings_multiple_clear_cache_button = Hapus hasil yang sudah kedaluwarsa dari penyimpanan sementara.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip = 
        Menyembunyikan semua berkas kecuali satu, jika semua berkas tersebut menunjuk ke data yang sama (melalui hard link).

        Contoh: Jika terdapat tujuh berkas di disk yang memiliki hard link ke data yang sama, dan satu berkas lain dengan data yang sama tetapi inode yang berbeda, maka pada fitur pencari duplikat, hanya akan ditampilkan satu berkas unik dan satu berkas dari berkas-berkas yang memiliki hard link.
settings_duplicates_minimal_size_entry_tooltip = 
        Atur ukuran file minimum yang akan disimpan dalam cache.

        Memilih nilai yang lebih kecil akan menghasilkan lebih banyak catatan. Hal ini akan mempercepat pencarian, tetapi memperlambat proses pemuatan/penyimpanan cache.
settings_duplicates_prehash_checkbutton_tooltip = 
        Memungkinkan penyimpanan sementara (cache) dari *prehash* (sebuah nilai hash yang dihitung dari sebagian kecil file), yang memungkinkan penghapusan hasil yang tidak duplikat lebih cepat.

        Fitur ini dinonaktifkan secara bawaan karena dapat menyebabkan perlambatan dalam beberapa situasi.

        Sangat disarankan untuk mengaktifkannya saat memindai ratusan ribu atau jutaan file, karena dapat mempercepat pencarian berkali-kali lipat.
settings_duplicates_prehash_minimal_entry_tooltip = Ukuran minimal entri yang disimpan dalam cache.
settings_duplicates_hide_hard_link_button = Sembunyikan tautan keras
settings_duplicates_prehash_checkbutton = Gunakan *cache* pra-hash
settings_duplicates_minimal_size_cache_label = Ukuran file terkecil (dalam byte) yang disimpan ke cache
settings_duplicates_minimal_size_cache_prehash_label = Ukuran minimum file (dalam byte) yang disimpan ke cache pra-hash

## Saving/Loading settings

settings_saving_button_tooltip = Simpan konfigurasi pengaturan saat ini ke dalam file.
settings_loading_button_tooltip = Muat pengaturan dari file dan ganti konfigurasi yang sedang digunakan dengan pengaturan tersebut.
settings_reset_button_tooltip = Kembalikan konfigurasi saat ini ke pengaturan bawaan.
settings_saving_button = Simpan konfigurasi
settings_loading_button = Muat konfigurasi
settings_reset_button = Atur ulang konfigurasi

## Opening cache/config folders

settings_folder_cache_open_tooltip = 
        Membuka folder tempat file cache disimpan.

        Memodifikasi file cache dapat menyebabkan hasil yang tidak valid ditampilkan. Namun, mengubah jalur dapat menghemat waktu saat memindahkan sejumlah besar file ke lokasi yang berbeda.

        Anda dapat menyalin file-file ini antar komputer untuk menghemat waktu karena tidak perlu melakukan pemindaian ulang (tentu saja jika struktur direktori mereka serupa).

        Jika terjadi masalah dengan cache, file-file ini dapat dihapus. Aplikasi akan secara otomatis membuat ulang file-file tersebut.
settings_folder_settings_open_tooltip = 
        Membuka folder tempat konfigurasi Czkawka disimpan.

        PERINGATAN: Mengubah konfigurasi secara manual dapat merusak alur kerja Anda.
settings_folder_cache_open = Buka folder cache
settings_folder_settings_open = Buka folder pengaturan
# Compute results
compute_stopped_by_user = Pencarian dihentikan oleh pengguna
compute_found_duplicates_hash_size = Ditemukan { $number_files } duplikat dalam { $number_groups } grup, yang memakan ruang sebesar { $size } dan membutuhkan waktu { $time}
compute_found_duplicates_name = Ditemukan { $number_files } duplikat dalam { $number_groups } grup dalam waktu { $time }
compute_found_empty_folders = Ditemukan { $number_files } folder kosong dalam waktu { $time }
compute_found_empty_files = Ditemukan { $number_files } berkas kosong dalam { $time }
compute_found_big_files = Ditemukan { $number_files } berkas besar dalam waktu { $time }
compute_found_temporary_files = Ditemukan { $number_files } berkas sementara dalam { $time }
compute_found_images = Ditemukan { $number_files } gambar serupa dalam { $number_groups } kelompok dalam waktu { $time }
compute_found_videos = Ditemukan { $number_files } video serupa dalam { $number_groups } grup, dalam waktu { $time }
compute_found_music = Ditemukan { $number_files } file musik serupa dalam { $number_groups } kelompok dalam waktu { $time }
compute_found_invalid_symlinks = Ditemukan { $number_files } tautan simbolik yang tidak valid dalam { $time }
compute_found_broken_files = Ditemukan { $number_files } berkas yang rusak dalam waktu { $time }
compute_found_bad_extensions = Ditemukan { $number_files } berkas dengan ekstensi yang tidak valid dalam waktu { $time }
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] Scanned { $file_number } file
       *[other] Scanned { $file_number } files
    }
progress_scanning_extension_of_files = Checked extension of { $file_checked }/{ $all_files } file
progress_scanning_broken_files = Checked { $file_checked }/{ $all_files } file ({ $data_checked }/{ $all_data })
progress_scanning_video = Hashed of { $file_checked }/{ $all_files } video
progress_creating_video_thumbnails = Created thumbnails of { $file_checked }/{ $all_files } video
progress_scanning_image = Hashed of { $file_checked }/{ $all_files } image ({ $data_checked }/{ $all_data })
progress_comparing_image_hashes = Compared { $file_checked }/{ $all_files } image hash
progress_scanning_music_tags_end = Compared tags of { $file_checked }/{ $all_files } music file
progress_scanning_music_tags = Read tags of { $file_checked }/{ $all_files } music file
progress_scanning_music_content_end = Compared fingerprint of { $file_checked }/{ $all_files } music file
progress_scanning_music_content = Calculated fingerprint of { $file_checked }/{ $all_files } music file ({ $data_checked }/{ $all_data })
progress_scanning_empty_folders =
    { $folder_number ->
        [one] Scanned { $folder_number } folder
       *[other] Scanned { $folder_number } folders
    }
progress_scanning_size = Scanned size of { $file_number } file
progress_scanning_size_name = Scanned name and size of { $file_number } file
progress_scanning_name = Scanned name of { $file_number } file
progress_analyzed_partial_hash = Analyzed partial hash of { $file_checked }/{ $all_files } files ({ $data_checked }/{ $all_data })
progress_analyzed_full_hash = Analyzed full hash of { $file_checked }/{ $all_files } files ({ $data_checked }/{ $all_data })
progress_prehash_cache_loading = Memuat cache pra-hash
progress_prehash_cache_saving = Menyimpan cache pra-hash
progress_hash_cache_loading = Memuat cache hash
progress_hash_cache_saving = Menyimpan cache hash
progress_cache_loading = Memuat cache
progress_cache_saving = Menyimpan data sementara
progress_current_stage = Current Stage:{ "  " }
progress_all_stages = All Stages:{ "  " }
# Saving loading 
saving_loading_saving_success = Konfigurasi telah disimpan ke file { $name }.
saving_loading_saving_failure = Gagal menyimpan data konfigurasi ke file { $name }, alasannya { $reason }.
saving_loading_reset_configuration = Konfigurasi saat ini telah dihapus.
saving_loading_loading_success = Konfigurasi aplikasi telah dimuat dengan benar.
saving_loading_failed_to_create_config_file = Gagal membuat file konfigurasi "{ $path }", alasannya "{ $reason }".
saving_loading_failed_to_read_config_file = Tidak dapat memuat konfigurasi dari "{ $path }" karena file tersebut tidak ada atau bukan merupakan sebuah file.
saving_loading_failed_to_read_data_from_file = Tidak dapat membaca data dari file "{ $path }", alasannya "{ $reason }".
# Other
selected_all_reference_folders = Tidak dapat memulai pencarian jika semua direktori telah ditetapkan sebagai folder referensi
searching_for_data = Sedang mencari data, mungkin membutuhkan waktu, mohon tunggu...
text_view_messages = PESAN
text_view_warnings = PERINGATAN
text_view_errors = KESALAHAN
about_window_motto = Program ini gratis untuk digunakan dan akan selalu demikian.
krokiet_new_app = Czkawka sedang dalam masa pemeliharaan, yang berarti hanya *bug* penting yang akan diperbaiki dan tidak ada fitur baru yang akan ditambahkan. Untuk fitur-fitur baru, silakan coba aplikasi Krokiet yang baru, yang lebih stabil dan memiliki kinerja lebih baik, dan masih dalam tahap pengembangan aktif.
# Various dialog
dialogs_ask_next_time = Tanyakan lagi lain waktu
symlink_failed = Failed to symlink { $name } to { $target }, reason { $reason }
delete_title_dialog = Konfirmasi penghapusan
delete_question_label = Apakah Anda yakin ingin menghapus berkas-berkas ini?
delete_all_files_in_group_title = Konfirmasi penghapusan semua berkas dalam grup
delete_all_files_in_group_label1 = Dalam beberapa kelompok, semua catatan dipilih.
delete_all_files_in_group_label2 = Apakah Anda yakin ingin menghapusnya?
delete_items_label = { $items } berkas akan dihapus.
delete_items_groups_label = { $items } berkas dari { $groups } grup akan dihapus.
hardlink_failed = Gagal membuat tautan keras untuk { $name } ke { $target }, alasannya: { $reason }
hard_sym_invalid_selection_title_dialog = Pemilihan tidak valid dengan beberapa grup
hard_sym_invalid_selection_label_1 = Dalam beberapa grup, hanya ada satu catatan yang dipilih, dan catatan tersebut akan diabaikan.
hard_sym_invalid_selection_label_2 = Untuk dapat membuat tautan keras/simbolik ke file-file ini, setidaknya dua hasil dalam grup tersebut harus dipilih.
hard_sym_invalid_selection_label_3 = Yang pertama dalam kelompok dianggap sebagai yang asli dan tidak diubah, tetapi yang kedua dan selanjutnya dimodifikasi.
hard_sym_link_title_dialog = Konfirmasi tautan
hard_sym_link_label = Apakah Anda yakin ingin menautkan file-file ini?
move_folder_failed = Failed to move folder { $name }, reason { $reason }
move_file_failed = Failed to move file { $name }, reason { $reason }
move_files_title_dialog = Pilih folder tempat Anda ingin memindahkan file-file yang terduplikasi
move_files_choose_more_than_1_path = Only one path may be selected to be able to copy their duplicated files, selected { $path_number }.
move_stats = Properly moved { $num_files }/{ $all_files } items
save_results_to_file = Saved results both to txt and json files into "{ $name }" folder.
search_not_choosing_any_music = ERROR: Anda harus memilih setidaknya satu kotak centang dengan jenis pencarian musik.
search_not_choosing_any_broken_files = ERROR: Anda harus memilih setidaknya satu kotak centang yang menunjukkan jenis file rusak.
include_folders_dialog_title = Folder yang akan disertakan
exclude_folders_dialog_title = Folder yang akan dikecualikan
include_manually_directories_dialog_title = Tambahkan direktori secara manual
cache_properly_cleared = Cache berhasil dihapus
cache_clear_duplicates_title = Menghapus cache duplikat
cache_clear_similar_images_title = Menghapus cache gambar-gambar yang serupa
cache_clear_similar_videos_title = Menghapus cache untuk video yang serupa
cache_clear_message_label_1 = Apakah Anda ingin menghapus cache entri yang sudah kedaluwarsa?
cache_clear_message_label_2 = Operasi ini akan menghapus semua entri cache yang mengarah ke berkas-berkas yang tidak valid.
cache_clear_message_label_3 = Ini mungkin sedikit mempercepat proses memuat/menyimpan data ke dalam cache.
cache_clear_message_label_4 = PERINGATAN: Operasi ini akan menghapus semua data yang tersimpan sementara dari perangkat eksternal yang tidak terhubung. Oleh karena itu, setiap hash perlu dibuat ulang.
# Show preview
preview_image_resize_failure = Failed to resize image { $name }.
preview_image_opening_failure = Failed to open image { $name }, reason { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grup { $current_group }/{ $all_groups } ({ $images_in_group } gambar)
compare_move_left_button = L
compare_move_right_button = R
