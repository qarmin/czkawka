# Cedinia - English (fallback)

# App / top bar titles
app_name = Cedinia
tool_duplicate_files = Eş Dosyalar
tool_empty_folders = Boş Klasörler
tool_similar_images = Benzer Resimler
tool_empty_files = Boş Dosyalar
tool_temporary_files = Geçici Dosyalar
tool_big_files = En Büyük Dosyalar
tool_broken_files = Bozuk Dosyalar
tool_bad_extensions = Hatalı Uzantılar
tool_same_music = Müzik Kopyaları
tool_bad_names = Kötü İsimler
tool_exif_remover = EXIF Verileri
tool_directories = Klasörler
tool_settings = Ayarlar
# Home screen tool card descriptions
home_dup_description = Aynı içeriğe sahip dosyaları bulun
home_empty_folders_description = İçerik olmayan dizinler
home_similar_images_description = Görsel olarak benzer fotoğrafları bulun
home_empty_files_description = Sıfır boyutlu dosyalar
home_temp_files_description = Geçici ve önbelleğe alınmış dosyalar
home_big_files_description = Disk üzerindeki en büyük/en küçük dosyalar
home_broken_files_description = PDF, ses, resimler, arşivler
home_bad_extensions_description = Geçersiz uzantıya sahip dosyalar
home_same_music_description = Etiketlere göre benzer ses dosyaları
home_bad_names_description = İsimlerinde sorunlu karakterler içeren dosyalar
home_exif_description = EXIF meta verisi içeren resimler
# Results list
scanning = Taramaya başlanıyor...
stopping = Bekleniyor...
no_results = Sonuç bulunamadı
press_start = TARAMAYA başlamak için START düğmesine basın
select_label = Selam.
deselect_label = Desel.
list_label = Liste
gallery_label = Gal.
# Selection popup
selection_popup_title = Seç
select_all = Tümünü Seç
select_except_one = Bir tanesini hariç tutarak, hepsini seçin
select_except_largest = En büyüğü hariç, hepsini seç
select_except_smallest = En küçük olan hariç hepsini seç
select_largest = En büyüğünü seçin
select_smallest = En küçüğü seç
select_except_highest_res = En yüksek çözünürlük hariç, tüm seçenekleri seçin
select_except_lowest_res = En düşük çözünürlük hariç, tümünü seçin
select_highest_res = En yüksek çözünürlüğü seçin
select_lowest_res = En düşük çözünürlüğü seçin
invert_selection = Seçimi Ters Çevir
close = Kapat
# Deselection popup
deselection_popup_title = Seçimi Kaldır
deselect_all = Seçili olanları kaldırın
deselect_except_one = Tümünü seçili olmaktan çıkarın, sadece bir tanesini seçili bırakın
# Confirm popup
cancel = İptal
delete = Sil
rename = Yeniden Adlandır
# Delete errors popup
delete_errors_title = Bazı dosyaların silinmesi sırasında bir hata oluştu:
ok = Tamam
# Stopping overlay
stopping_overlay_title = Duraklama
stopping_overlay_body = 
        Mevcut tarama işlemi tamamlanıyor...
        Lütfen bekleyin.
# Permission popup
permission_title = Dosya Erişimi
permission_body = Dosyaları taramak için, uygulamanın cihazınızın depolama alanına erişmesi gerekmektedir. Bu izne sahip olmadan, tarama işlemi mümkün olmayacaktır.
grant = Hibe
no_permission_scan_warning = Dosya erişimi yok - tarama izni verin
# Settings screen tabs
settings_tab_general = Genel
settings_tab_tools = Araçlar
settings_tab_diagnostics = Bilgi
# Settings - General tab
settings_use_cache = Önbellek Kullan
settings_use_cache_desc = Sonraki taramaları (hash/görseller) hızlandırır
settings_ignore_hidden = Gizli dosyaları yoksay
settings_ignore_hidden_desc = "." ile başlayan dosyalar ve klasörler
settings_show_notification = Tarama işlemi tamamlandığında bildirim gönder
settings_show_notification_desc = Taramayı tamamladığında, sistemde bir bildirim göster
settings_notify_only_background = Sadece arka planda çalışırken
settings_notify_only_background_desc = Uygulama görünürse, bildirimi göstermeyi atlayın
notifications_disabled_banner = Bildirimler devre dışı
notifications_enable_button = Etkinleştir
settings_scan_label = TARA
settings_filters_label = FİLTRELER (bazı araçlar)
settings_min_file_size = Minimum dosya boyutu
settings_max_file_size = Maksimum dosya boyutu
settings_language = Dil
settings_language_restart = Uygulamanın yeniden başlatılması gereklidir
settings_common_label = ORTAK AYARLAR
settings_excluded_items = İSTİSNALAR (küre desenleri, virgülle ayrılmış)
settings_excluded_items_placeholder = Örneğin: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = İZİN VERİLEN UZANTILAR (boş bırakıldığında = tüm uzantılar)
settings_allowed_extensions_placeholder = Örneğin: jpg, png, mp4
settings_excluded_extensions = DESTEKLENMEYEN UZANTILAR
settings_excluded_extensions_placeholder = Örneğin: bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = KOPYALAR
settings_check_method_label = KARŞILAŞTIRMA YÖNTEMİ
settings_check_method = Yöntem
settings_hash_type_label = HASH TİPİ
settings_hash_type = SUÇ türü:
settings_hash_type_desc = Blake3 - önerilen seçenektir, CRC32'nin yanlış pozitif sonuç verme olasılığı daha düşüktür
settings_similar_images_header = BENZER GÖRÜNTÜLER
settings_similarity_preset = Benzerlik eşik değeri
settings_similarity_desc = Çok Yüksek = sadece neredeyse aynı olanlar
settings_hash_size = SURÇ boyutu:
settings_hash_size_desc = Daha büyük boyutlardaki modeller, daha az yanlış pozitif sonuç üretir, ancak aynı zamanda daha az benzer görüntü bulur
settings_hash_alg = Karma algoritması
settings_image_filter = Boyutlandırma filtresi
settings_ignore_same_size = Aynı boyutlara sahip resimleri görmezden gelin
settings_gallery_image_fit_cover = Galeri: Kare formatına getirin
settings_gallery_image_fit_cover_desc = Koyu rengi doldurun; orijinal oranları korumak için bu özelliği devre dışı bırakın
settings_big_files_header = EN BÜYÜK DOSYALAR
settings_search_mode = Arama modu
settings_file_count = Dosya sayısı
settings_same_music_header = MÜZİK YAYINLARI/MÜZİK ÇOĞALTMA HİZMETLERİ
settings_music_check_method = Karşılaştırma modu
settings_music_compare_tags_label = KARŞILAŞTIRMA ETIKETLERİ
settings_music_title = Başlık
settings_music_artist = Sanatçı
settings_music_year = Yıl
settings_music_length = Uzunluk
settings_music_genre = Müzik Türü
settings_music_bitrate = Bit Hızı
settings_music_approx = Yaklaşık etiket karşılaştırması
settings_broken_files_header = BOZULMUŞ DOSYALAR
settings_broken_files_note = Yüksek işlem gücü gerektiren bir tarama. En iyi performansı elde etmek için Krokiet'i masaüstü bilgisayarınızda kullanın.
settings_broken_files_types_label = KONTROL EDİLEN TİPLER
settings_broken_audio = Ses
settings_broken_pdf = PDF
settings_broken_archive = Arşiv
settings_broken_image = Görsel
settings_bad_names_header = KÖTÜ İSİMLER
settings_bad_names_checks_label = ÇEKLER
settings_bad_names_uppercase_ext = BÜYÜK HARF EKLENTİSİ
settings_bad_names_emoji = Emoji'de isim
settings_bad_names_space = Başlangıçta/sonunda boşluklar
settings_bad_names_non_ascii = ASCII olmayan karakterler
settings_bad_names_duplicated = Tekrarlayan karakterler
# Settings - Diagnostics tab
diagnostics_header = TEŞHİS
diagnostics_thumbnails = Küçük resim önbelleği
diagnostics_app_cache = Uygulama önbelleği
diagnostics_refresh = Yenile
diagnostics_clear_thumbnails = Açık ve net önizlemeler
diagnostics_open_thumbnails_folder = Klasörü aç
diagnostics_clear_cache = Önbelleği temizle
diagnostics_open_cache_folder = Klasörü aç
diagnostics_collect_test = Dosya erişim testi
diagnostics_collect_test_desc = Kontrol edin, kaç tane dosyanın erişilebilir olduğunu
diagnostics_collect_test_run = Çalıştır
diagnostics_collect_test_stop = Durdur
collect_test_cancelled = Kullanıcı tarafından durduruldu
diag_confirm_clear_thumbnails = Tüm önizleme önbelleğini temizlemek istiyor musunuz?
diag_confirm_clear_cache = Uygulama önbelleğini temizlemek ister misiniz?
about_repo = Depo
about_translate = Çeviriler
about_donate = Destek
# Collect-test result popup
collect_test_title = Test sonuçları
collect_test_volumes = Ciltler:
collect_test_folders = Klasörler:
collect_test_files = Dosyalar:
collect_test_time = Zaman:
# Licenses
licenses_label = LİSANS
third_party_licenses = Üçüncü taraf lisansları
licenses_popup_title = Üçüncü Taraf Lisansları
# Directories screen
directories_include_header = İçerir
directories_included = Ek olarak
directories_exclude_header = Hariç tutun
directories_excluded_header = Hariç tutulan
directories_add = İçermelidir
no_paths = Yol belirtilmemiş - aşağıdaki gibi ekleyin
directories_volume_header = Ciltler
directories_volume_refresh = Yenile
directories_volume_add = Ekle
# Bottom navigation
nav_home = Başla
nav_dirs = Dizinler
nav_settings = Ayarlar
# Status messages set from Rust
status_ready = Hazır
status_stopped = Durduruldu
status_no_results = Sonuç bulunamadı
status_deleted_selected = Seçilen öğeler silindi
status_deleted_with_errors = Hatalarla birlikte silindi
scan_not_started = Taramaya henüz başlanmadı
found_items_prefix = Bulundu
found_items_suffix = ürünler
deleted_items_prefix = Silindi
deleted_items_suffix = ürünler
deleted_errors_suffix = hatalar
renamed_prefix = Yeniden adlandırıldı
renamed_files_suffix = dosyalar
renamed_errors_suffix = hatalar
cleaned_exif_prefix = EXIF verilerinden arındırıldı
cleaned_exif_suffix = dosyalar
cleaned_exif_errors_suffix = Hatalar
and_more_prefix = ...ve
and_more_suffix = daha fazla
# Gallery / delete popups
gallery_delete_button = Sil
gallery_back = Geri
gallery_confirm_delete = Evet, sil
deleting_files = Dosyalar siliniyor...
stop = Durdur
files_suffix = dosyalar
scanning_fallback = Taramaya başlanıyor...
app_subtitle = 972 yılında gerçekleşen Cedynia Savaşı'nın anısına
app_license = Czkawka Core için ön yüz - GPL-3.0
about_app_label = HAKKINDA
cache_label = ÖNBELLEK
# Notification
scan_completed_notification = Tarama tamamlandı - { $file_count } adet öğe bulundu
# Confirm popups (set from Rust)
confirm_clean_exif = { $n } seçili dosyadan EXIF etiketlerini silmek istediğinizden emin misiniz?
confirm_delete_items = Seçili { $n } öğeyi silmek istediğinizden emin misiniz?
gallery_confirm_delete_msg = Şu anda { $total_groups } grup içinde toplam { $total_images } fotoğrafı sileceksiniz.
gallery_confirm_delete_warning = Tüm öğeler, { $unsafe_groups } gruplarından seçildi!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = Ses parmak izlerini hesaplamak ve karşılaştırmak çok fazla kaynak gerektirir ve uzun sürebilir. Bu işlem için Krokiet'in masaüstü bir sistemde kullanılması önerilir.
# Scan stage labels (shown during scan progress)
stage_collecting_files = Dosyaları toplama işlemi
stage_scanning_name = İsimle tarama
stage_scanning_size_name = İsim ve boyuta göre tarama
stage_scanning_size = Boyuta göre tarama
stage_pre_hash = Önceden karma işlemi
stage_full_hash = Karma Alma (veya Karma Fonksiyonları)
stage_loading_cache = Önbellek yükleniyor
stage_saving_cache = Önbellek kaydediliyor
stage_calculating_image_hashes = Görüntü karmalarının hesaplanması
stage_comparing_images = Resimleri karşılaştırma
stage_calculating_video_hashes = Video dosyalarının hash değerlerini hesaplama
stage_checking_files = Dosyaları kontrol ediliyor
stage_checking_extensions = Uzantıların kontrolü
stage_checking_names = İsimlerin kontrol edilmesi
stage_reading_music_tags = Müzik etiketlerini okuma
stage_comparing_tags = Etiketleri karşılaştırma
stage_calculating_music_fingerprints = Müzik parmak izlerini hesaplama
stage_comparing_fingerprints = Parmak izlerini karşılaştırma
stage_extracting_exif = EXIF etiketlerini okuma
stage_creating_video_thumbnails = Video önizleme resimleri oluşturma
stage_processing_videos = Videolar işleniyor
stage_deleting = Dosyaları silme
stage_renaming = Dosya isimlerini değiştirme
stage_moving = Öğeleri Taşı
stage_hardlinking = Sabit bağlantılar oluşturma
stage_symlinking = Sembolik bağlantılar oluşturma
stage_optimizing_videos = Videoları optimize etme
stage_cleaning_exif = EXIF verilerini temizleme
# Group headers in scan results
duplicates_group_header = { $count } dosya x { $per_file } / dosya = { $total } toplam
similar_images_group_header = { $count } benzer resim
same_music_group_header = { $count } benzer şarkı
# Rename confirmation
confirm_rename_items = Seçili olan { $n } dosyayı yeniden adlandırmak istediğinizden emin misiniz?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = En büyük
option_search_mode_smallest = En küçük
option_similarity_very_high = Çok Yüksek
option_similarity_high = Yüksek
option_similarity_medium = Orta
option_similarity_low = Düşük
option_similarity_very_low = Çok Düşük
option_similarity_minimal = Dakika.
option_check_method_hash = Karma
option_check_method_name = Ad Karşılaştırma
option_check_method_size_and_name = Beden+İsim
option_check_method_size = Boyut
option_music_method_tags = Etiketler
option_music_method_audio = Ses
option_min_size_none = Çevrilecek metin bulunmamaktadır
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = Sınırsız
# Volume labels (shown in the directories screen)
volume_internal_storage = İç Depolama Alanı
volume_sd_card = Bellek Kartı (SD Kart)
volume_storage = Depolama Kapasitesi
# Directories screen
directories_referenced_tooltip = Referans alınmış (silinmemiş)
directories_include_section_header = DAHİL
directories_exclude_section_header = HARİÇ TUTULMUŞ
directories_custom_paths = Özel Yollar
directories_check_button = Analiz et
directories_check_popup_title = Dizin İstatistikleri
directories_check_label_included = İçerilen yollar:
directories_check_label_excluded = Hariç tutulan yollar:
directories_check_label_referenced = Referans yolları:
directories_check_label_would_scan = Taranacak dosyalar:
directories_check_label_processable = İşlenebilen dosyalar:
directories_check_scanning = Tarama yapılıyor...
directories_check_warning_no_processable = İşlenebilir dosya bulunamadı - dahil edilen/hariç tutulan klasörlerinizi kontrol edin
path_edit_title_include = Ekle (içermesi için)
path_edit_title_exclude = Hariç Tutulacaklar Listesine Ekle
path_edit_placeholder = Yol belirtin...
path_edit_not_exists = Yol bulunamadı
path_edit_is_dir = Dizin
path_edit_is_file = Dosya
path_edit_no_newlines = Yollar yeni satır karakterleri içermemelidir — Enter tuşunun kullanılmasına izin verilmez
ctx_menu_title = Açık
ctx_open_file = Açık işlem
ctx_open_folder = Üst klasörü aç
