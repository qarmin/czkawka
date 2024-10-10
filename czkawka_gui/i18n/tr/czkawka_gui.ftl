# Window titles
window_settings_title = Ayarlar
window_main_title = Czkawka (Hıçkırık)
window_progress_title = Taranıyor...
window_compare_images = Resimleri Karşılaştır
# General
general_ok_button = Tamam
general_close_button = Kapat
# Main window
music_title_checkbox = Başlık
music_artist_checkbox = Sanatçı
music_year_checkbox = Yıl
music_bitrate_checkbox = Bit-hızı
music_genre_checkbox = Müzik Türü
music_length_checkbox = Uzunluk
music_comparison_checkbox = Yaklaşık Karşılaştırma
music_checking_by_tags = Etiketler
music_checking_by_content = İçerik
same_music_seconds_label = Minimal parça saniyesel süresi
same_music_similarity_label = Maksimum fark
music_compare_only_in_title_group = Compare only in title
music_compare_only_in_title_group_tooltip =
    When enabled, files are grouped by title and then compared to each other.
    
    With 10000 files, instead almost 100 million comparisons usually there will be around 20000 comparisons.
same_music_tooltip =
    İçeriğine göre benzer müzik dosyalarının aranması ayarlanarak yapılandırılabilir:
    
    - Müzik dosyalarının benzer olarak tanımlanabileceği minimum parça süresi
    - Test edilen iki parça arasındaki maksimum fark
    
    İyi sonuçlar elde etmenin anahtarı, bu parametrelerin mantıklı kombinasyonlarını bulmaktır.
    
    Minimum süreyi 5 saniye ve maksimum farkı 1.0 olarak ayarlamak, dosyalarda neredeyse aynı parçaları arayacaktır.
    Öte yandan, 20 saniyelik bir süre ve 6.0'lık bir maksimum fark, remiksleri / canlı sürümleri vb. bulmak için iyi çalışır.
    
    Varsayılan olarak, her müzik dosyası birbiriyle karşılaştırılır ve çok sayıda dosyayı test ederken bu çok zaman alabilir, bu nedenle genellikle referans klasörleri kullanmak ve hangi dosyaların birbiriyle karşılaştırılacağını belirtmek daha iyidir (aynı miktarda dosya ile, parmak izlerini karşılaştırmak referans klasörleri olmadan en az 4 kat daha hızlı olacaktır).
music_comparison_checkbox_tooltip =
    Yapay zeka kullanarak benzer müzik dosyalarını arar. 
    Örneğin, bir tümcenin parantezlerini kaldırmak için makine öğrenimini kullanır. 
    Bu seçenek etkinleştirildiğinde, söz konusu dosyalar kopya olarak kabul edilecektir:
    
    Geççek <--> Geççek (Tarkan 2022)
duplicate_case_sensitive_name = Büyük/Küçük harfe Duyarlı
duplicate_case_sensitive_name_tooltip =
    Etkinleştirilse, dosya adları tam olarak aynı olduğunda eşleştirilir
    ve bir grup oluşturulur.
    
    fatih.kavalci <--> fatih.kavalci
    
    Etkisizleştirilirse, her bir harfin büyük/küçük yazılıp yazılmadığını 
    denetlemeden aynı adları eşleyip grup oluşturur.
    
    fatih.kavalci <--> FatiH.KaVaLCi
duplicate_mode_size_name_combo_box = Boyut ve Ad Karşılaştırma
duplicate_mode_name_combo_box = Ad Karşılaştırma
duplicate_mode_size_combo_box = Boyut Karşılaştırma
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip =
    Czkawka, 3 tür Sabit Uzunlukta Çıktı (SUÇ) üretimi sunar:
    
    Blake3 - kriptografik SUÇ üretim işlevi. Bu varsayılandır çünkü çok hızlıdır.
    
    CRC32 - basit SUÇ üretim işlevi. Bu, Blake3'ten daha hızlı olmalıdır, 
    ancak kimi zaman çakışmalar olabilir.
    
    XXH3 - performans ve benzersiz SUÇ üretim kalitesi açısından Blake3'e çok benzer 
    (ancak kriptografik değildir). Böylece, bu tür modlar kolayca değiştirilebilir.
duplicate_check_method_tooltip =
    Czkawka, eş dosyaları bulmak için şimdilik üç tür yöntem sunar:
    
    Ad Karşılaştırma - Aynı ada sahip dosyaları bulur.
    
    Boyut Karşılaştırma - Aynı boyuta sahip dosyaları bulur.
    
    Hash (SUÇ) Karşılaştırma - Aynı içeriğe sahip dosyaları bulur. Bu mod her dosya için 
    veri analizi sonucu sabit uzunlukta benzersiz birer çıktı üretir ve daha sonra eş doşyaları 
    bulmak için bu çıktıları karşılaştırır. Bu mod, eş dosyaları bulmanın en güvenli yoludur. 
    Czkawka, önbelleği yoğun olarak kullanır. Bu nedenle aynı verilerin ikinci ve sonraki taramaları 
    ilkinden çok daha hızlı olmalıdır.
image_hash_size_tooltip =
    Kontrol edilen her resim, birbiriyle karşılaştırılabilen özel bir hash üretir ve aralarındaki küçük bir fark, bu görüntülerin benzer olduğu anlamına gelir.
    
    8 hash boyutu, orijinaline çok az benzeyen görüntüleri bulmak için oldukça iyidir. Daha büyük bir görüntü kümesinde (>1000), bu büyük miktarda yanlış pozitif üretecektir, bu nedenle bu durumda daha büyük bir karma boyutu kullanmanızı öneririm.
    
    16 varsayılan hash boyutudur ve az da olsa benzer resimler bulmakla az miktarda hash çakışması olması arasında oldukça iyi bir uzlaşmadır.
    
    32 ve 64 hash'ler yalnızca çok benzer görüntüleri bulur, ancak neredeyse hiç piksel farkı olmamalıdır (belki alfa kanallı bazı görüntüler hariç).
image_resize_filter_tooltip =
    To compute hash of image, the library must first resize it.
    
    Depend on chosen algorithm, the resulting image used to calculate hash will looks a little different.
    
    The fastest algorithm to use, but also the one which gives the worst results, is Nearest. It is enabled by default, because with 16x16 hash size lower quality it is not really visible.
    
    With 8x8 hash size it is recommended to use a different algorithm than Nearest, to have better groups of images.
image_hash_alg_tooltip =
    Kullanıcılar, SUÇ oluşturmanın birçok algoritmasından birini seçebilir. 
    Her birinin hem güçlü hem de zayıf noktaları vardır ve farklı görüntüler için 
    bazen daha iyi, bazen daha kötü sonuçlar verir. Bu nedenle, size göre en iyisini belirlemek için 
    elle test gereklidir.
big_files_mode_combobox_tooltip = Boyut bakımından En Büyük/En Küçük dosyaları aramaya izin verir.
big_files_mode_label = Denetim şekli
big_files_mode_smallest_combo_box = En Küçük
big_files_mode_biggest_combo_box = En Büyük
main_notebook_duplicates = Eş Dosyalar
main_notebook_empty_directories = Boş Dizinler
main_notebook_big_files = Büyük/Küçük Dosyalar
main_notebook_empty_files = Boş Dosyalar
main_notebook_temporary = Geçici Dosyalar
main_notebook_similar_images = Benzer Resimler
main_notebook_similar_videos = Benzer Videolar
main_notebook_same_music = Müzik Kopyaları
main_notebook_symlinks = Geçersiz Sembolik Bağlar
main_notebook_broken_files = Bozuk Dosyalar
main_notebook_bad_extensions = Hatalı Uzantılar
main_tree_view_column_file_name = Dosya Adı
main_tree_view_column_folder_name = Klasör Adı
main_tree_view_column_path = Yol
main_tree_view_column_modification = Düzenleme Tarihi
main_tree_view_column_size = Boyut
main_tree_view_column_similarity = Benzerlik
main_tree_view_column_dimensions = En x Boy
main_tree_view_column_title = Başlık
main_tree_view_column_artist = Sanatçı
main_tree_view_column_year = Yıl
main_tree_view_column_bitrate = Bit-hızı
main_tree_view_column_length = Uzunluk
main_tree_view_column_genre = Tür
main_tree_view_column_symlink_file_name = Sembolik Bağ Dosyası Adı
main_tree_view_column_symlink_folder = Sembolik Bağlantı Klasörü
main_tree_view_column_destination_path = Hedef Yol
main_tree_view_column_type_of_error = Hata türü
main_tree_view_column_current_extension = Geçerli Uzantı
main_tree_view_column_proper_extensions = Uygun Uzantı
main_label_check_method = Denetim yöntemi:
main_label_hash_type = SUÇ türü:
main_label_hash_size = SURÇ boyutu:
main_label_size_bytes = Boyut (bayt):
main_label_min_size = Min
main_label_max_size = Max
main_label_shown_files = Gösterilecek Dosya Sayısı:
main_label_resize_algorithm = Yeniden boyutlandırma algoritması:
main_label_similarity = Benzerlik: { "   " }
main_check_box_broken_files_audio = Ses
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = Arşiv
main_check_box_broken_files_image = Resim
check_button_general_same_size = Aynı boyutu yok say
check_button_general_same_size_tooltip = Sonuçlarda aynı boyutta olan dosyaları yoksay - genellikle bunlar bire bir kopyalardır
main_label_size_bytes_tooltip = Taramada kullanılacak dosyaların boyutu
# Upper window
upper_tree_view_included_folder_column_title = Aranacak Klasörler
upper_tree_view_included_reference_column_title = Başvuru Klasörleri
upper_recursive_button = Özyinelemeli
upper_recursive_button_tooltip = Seçilirse, doğrudan "Aranacak Klasörler" listesindeki dizin altında yer almayan (alt dizinlerdeki dosyaları da) arar.
upper_manual_add_included_button = Dizin Gir...
upper_add_included_button = Ekle
upper_remove_included_button = Kaldır
upper_manual_add_excluded_button = Dizin Gir...
upper_add_excluded_button = Ekle
upper_remove_excluded_button = Kaldır
upper_manual_add_included_button_tooltip =
    Arama yapılacak dizin yolunu doğrudan yazın.
    
    Aynı anda birden fazla girdi eklemek için bunları ";" ile ayırın.
    
    /home/fatih;/home/kavalci girdisi biri /home/fatih öteki /home/kavalci 
    olmak üzere iki dizin ekleyecektir.
upper_add_included_button_tooltip = "Aranacak Klasörler" listesine yeni bir dizin ekler.
upper_remove_included_button_tooltip = Seçili dizini "Aranacak Klasörler" listesinden kaldırır.
upper_manual_add_excluded_button_tooltip =
    Hariç tutulacak dizin yolunu doğrudan yazın.
    
    Aynı anda birden fazla girdi eklemek için bunları ";" ile ayırın.
    
    /home/fatih;/home/kavalci girdisi biri /home/fatih öteki /home/kavalci 
    olmak üzere iki dizin ekleyecektir.
upper_add_excluded_button_tooltip = "Hariç Tutulacak Klasörler" listesine yeni bir dizin ekler.
upper_remove_excluded_button_tooltip = Seçili dizini "Hariç Tutulacak Klasörler" listesinden kaldırır.
upper_notebook_items_configuration = Öğe Yapılandırması
upper_notebook_excluded_directories = Hariç Tutulan Dizinler
upper_notebook_included_directories = Aranacak Dizinler
upper_allowed_extensions_tooltip =
    İzin verilen uzantılar virgülle ayrılmalıdır (varsayılan olarak her uzantı kullanılır).
    
    Aynı anda birden fazla (aynı tür) uzantı ekleyen makrolar da kullanılabilir: IMAGE, VIDEO, MUSIC, TEXT.
    
    Kullanım örneği: ".exe, IMAGE, VIDEO, .rar, .7z" -- Bu girdi, resimlerin (ör. jpg, png ...), 
    videoların (ör. avi, mp4 ...), exe, rar ve 7z dosyalarının taranacağı anlamına gelir.
upper_excluded_extensions_tooltip =
    Taramada göz ardı edilecek devre dışı bırakılmış dosyaların listesi.
    
    İzin verilen ve devre dışı bırakılan uzantılar kullanıldığında, bu daha yüksek önceliğe sahiptir, bu nedenle dosya kontrol edilmeyecektir.
upper_excluded_items_tooltip =
    Hariç tutulan öğeler * joker karakterini içermeli ve virgülle ayrılmalıdır.
    Bu işlev, Hariç Tutulan Dizinlerden daha yavaştır, bu yüzden dikkatli kullanın.
upper_excluded_items = Hariç Tutulan Öğeler:
upper_allowed_extensions = İzin Verilen Uzantılar:
upper_excluded_extensions = Devre Dışı Uzantılar:
# Popovers
popover_select_all = Tümünü seç
popover_unselect_all = Tümünün seçimini kaldır
popover_reverse = Seçimi Ters Çevir
popover_select_all_except_oldest = En eski olan hariç hepsini seç
popover_select_all_except_newest = En yeni olan hariç hepsini seç
popover_select_one_oldest = En eski olanı seç
popover_select_one_newest = En yeni olanı seç
popover_select_custom = Özel girdi ile seçim yap
popover_unselect_custom = Özel girdi ile seçimi kaldır
popover_select_all_images_except_biggest = En büyük olan hariç hepsini seç
popover_select_all_images_except_smallest = En küçük olan hariç hepsini seç
popover_custom_path_check_button_entry_tooltip =
    Kayıtları, kısmi yol girdisine göre seçer.
    
    Örnek kullanım:
    /home/fatih/kavalci.txt dosyası, /home/fat* girdisi ile bulunabilir
popover_custom_name_check_button_entry_tooltip =
    Kayıtları, kısmi dosya adı girdisine göre seçer.
    
    Örnek kullanım:
    /home/fatih/kavalci.txt dosyası, *val* girdisi ile bulunabilir
popover_custom_regex_check_button_entry_tooltip =
    Kayıtları, belirtilen Regex girdisine göre seçer.
    
    Bu mod ile aranan metin, tam yol dosya adıdır.
    
    Örnek kullanım:
    /home/fatih/kavalcı.txt dosyası, h/ka[a-z]+ ile bulunabilir
    
    Bu işlev, varsayılan Rust regex uygulamasını kullanır. 
    Daha fazla bilgi için bakınız: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip =
    Büyük/Küçük harfe duyarlı algılamayı etkinleştirir.
    
    Etkisizleştirilir ise;
    /home/fatih/* girdisi, hem /home/fatih/ hem de /home/FaTiH dizinlerini algılar.
popover_custom_not_all_check_button_tooltip =
    Gruptaki tüm kayıtların seçilmesini engeller.
    
    Bu varsayılan olarak etkindir. Çünkü, çoğu durumda hem asıl dosyayı hem de kopyaları 
    silmek istemezsiniz. En az bir dosya bırakmak istersiniz.
    
    UYARI: Bir gruptaki tüm sonuçlar zaten elle seçilmiş ise bu ayar çalışmaz.
popover_custom_regex_path_label = Yol
popover_custom_regex_name_label = Ad
popover_custom_regex_regex_label = Regex Yolu + Adı
popover_custom_case_sensitive_check_button = Büyük/Küçük harfe duyarlı
popover_custom_all_in_group_label = Gruptaki tüm kayıtları seçme
popover_custom_mode_unselect = Özel Girdi ile Seçimi Kaldır
popover_custom_mode_select = Özel Girdi ile Seç
popover_sort_file_name = Dosya adı
popover_sort_folder_name = Klasör adı
popover_sort_full_name = Tam ad
popover_sort_size = Boyut
popover_sort_selection = Seçim
popover_invalid_regex = Regex geçersiz (hatalı)
popover_valid_regex = Regex geçerli (doğru)
# Bottom buttons
bottom_search_button = Ara
bottom_select_button = Seç
bottom_delete_button = Sil
bottom_save_button = Kaydet
bottom_symlink_button = Sembolik bağlantı
bottom_hardlink_button = Sabit bağlantı
bottom_move_button = Taşı
bottom_sort_button = Sırala
bottom_search_button_tooltip = Aramayı başlatır.
bottom_select_button_tooltip = Kayıtları seçer. Yalnızca seçilen dosyalara/klasörlere işlem uygulanabilir.
bottom_delete_button_tooltip = Seçili dosyaları/klasörleri siler.
bottom_save_button_tooltip = Aramayla ilgili verileri dosyaya kaydeder.
bottom_symlink_button_tooltip =
    Sembolik bağlantılar oluşturur.
    Yalnızca bir gruptaki en az iki sonuç seçildiğinde çalışır.
    Birincisi değişmez, ikincisi ve sonrası birinciye sembolik olarak bağlanır.
bottom_hardlink_button_tooltip =
    Sabit bağlantılar oluşturur.
    Yalnızca bir gruptaki en az iki sonuç seçildiğinde çalışır.
    Birincisi değişmez, ikincisi ve sonrası birinciye sabit olarak bağlanır.
bottom_hardlink_button_not_available_tooltip =
    Hardlinkler oluştur.
    Düğme devre dışı, çünkü hardlinkler oluşturulamaz.
    Hardlinkler Windows üzerinde yalnızca administrator ayrıcalıklarıyla çalışır, bu yüzden uygulamayı yönetici olarak çalıştırdığınızdan emin olun.
    Eğer uygulama zaten yeterli ayrıcalıklarla çalışıyorsa Github üzerindeki benzer sorunları gözden geçirin.
bottom_move_button_tooltip =
    Dosyaları seçilen dizine taşır.
    Dizin ağacını korumadan tüm dosyaları dizine taşır.
    Aynı ada sahip iki dosyayı klasöre taşımaya çalışırken, ikincisi başarısız olur ve hata gösterir.
bottom_sort_button_tooltip = Dosyaları/Dizinleri seçilen metoda göre sırala.
bottom_show_errors_tooltip = Alt çıktı panelini göster/gizle.
bottom_show_upper_notebook_tooltip = Üst denetim panelini göster/gizle.
# Progress Window
progress_stop_button = Durdur
progress_stop_additional_message = İşlem durduruldu.
# About Window
about_repository_button_tooltip = Kaynak kodu depo sayfasına bağlanır.
about_donation_button_tooltip = Bağış sayfasına bağlanır.
about_instruction_button_tooltip = Kullanım yönergeleri sayfasına bağlanır.
about_translation_button_tooltip = Czkawka çevirileriyle Crowdin sayfasına bağlanır. Resmi olarak Lehçe ve İngilizce desteklenmektedir.
about_repository_button = Depo
about_donation_button = Bağış
about_instruction_button = Yönerge
about_translation_button = Çeviri
# Header
header_setting_button_tooltip = Ayarlar iletişim kutusunu açar.
header_about_button_tooltip = Czkawka hakkında bilgi içeren iletişim kutusunu açar.

# Settings


## General

settings_number_of_threads = Kullanılan iş parçacığı sayısı
settings_number_of_threads_tooltip = Kullanılan iş parçacığı sayısı, 0 tüm uygun iş parçacıklarının kullanılacağı anlamına gelir.
settings_use_rust_preview = Use external libraries instead gtk to load previews
settings_use_rust_preview_tooltip =
    Using gtk previews will sometimes be faster and support more formats, but sometimes this could be exactly the opposite.
    
    If you have problems with loading previews, you may can to try to change this setting.
    
    On non-linux systems, it is recommended to use this option, because gtk-pixbuf are not always available there so disabling this option will not load previews of some images.
settings_label_restart = Ayarları uygulamak için uygulamayı yeniden başlatmanız gerekir!
settings_ignore_other_filesystems = Öteki dosya sistemlerini yoksay (sadece Linux)
settings_ignore_other_filesystems_tooltip =
    Aranan dizinlerle aynı dosya sisteminde olmayan dosyaları yoksayar.
    
    Linux'ta find komutundaki -xdev seçeneği ile aynı şekilde çalışır.
settings_save_at_exit_button_tooltip = Uygulamayı kapatırken yapılandırmayı dosyaya kaydeder.
settings_load_at_start_button_tooltip =
    Uygulamayı açarken yapılandırmayı dosyadan yükler.
    
    Etkinleştirilmezse, varsayılan ayarlar kullanılır.
settings_confirm_deletion_button_tooltip = Sil düğmesine tıklandığında onay iletişim kutusunu gösterir.
settings_confirm_link_button_tooltip = Sabit/sembolik bağlantı düğmesine tıklandığında onay iletişim kutusunu göster.
settings_confirm_group_deletion_button_tooltip = Gruptan tüm kayıtları silmeye çalışırken uyarı iletişim kutusunu gösterir.
settings_show_text_view_button_tooltip = Kullanıcı arayüzünün altında çıktı panelini gösterir.
settings_use_cache_button_tooltip = Dosya önbelleğini kullanır.
settings_save_also_as_json_button_tooltip =
    Önbelleği (kullanıcı tarafından okunabilir) JSON biçiminde kaydeder. 
    İçeriğini değiştirmek mümkündür. İkili biçim önbelleği (bin uzantılı) eksikse, 
    bu dosyadaki önbellek uygulama tarafından otomatik olarak okunacaktır.
settings_use_trash_button_tooltip = Dosyaları kalıcı olarak silmek yerine çöp kutusuna taşır.
settings_language_label_tooltip = Kullanıcı arayüzü dilini değiştirir.
settings_save_at_exit_button = Uygulamayı kapatırken yapılandırmayı kaydet
settings_load_at_start_button = Uygulamayı açarken yapılandırmayı yükle
settings_confirm_deletion_button = Herhangi bir dosyayı silerken onay iletişim kutusunu göster
settings_confirm_link_button = Herhangi bir dosyaya sabit/sembolik bağlantı yapıldığında onay iletişim kutusunu göster
settings_confirm_group_deletion_button = Gruptaki tüm dosyaları silerken onay iletişim kutusunu göster
settings_show_text_view_button = Alt çıktı panelini göster
settings_use_cache_button = Önbelleği kullan
settings_save_also_as_json_button = Önbelleği JSON dosyası olarak da kaydet
settings_use_trash_button = Silinen dosyaları çöp kutusuna taşı
settings_language_label = Dil
settings_multiple_delete_outdated_cache_checkbutton = Güncel olmayan önbellek girişlerini otomatik olarak sil
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    Var olmayan dosyalara işaret eden eski önbellek girdilerini siler.
    
    Etkinleştirildiğinde, uygulama kayıtları yüklerken tüm kayıtların geçerli dosyalara 
    işaret etmesini sağlar (bozuk olanlar yoksayılır).
    
    Bunu devre dışı bırakmak, harici sürücülerdeki dosyaları tararken yardımcı olacaktır, 
    bu nedenle bunlarla ilgili önbellek girdileri bir sonraki taramada temizlenmez.
    
    Önbellekte yüzbinlerce kayıt olması durumunda, taramanın başlangıcında/sonunda 
    önbellek yükleme/kaydetme işlemini hızlandıracak olan bu özelliği etkinleştirmeniz önerilir.
settings_notebook_general = Genel
settings_notebook_duplicates = Eş Dosyalar
settings_notebook_images = Benzer Resimler
settings_notebook_videos = Benzer Videolar

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = Sağ tarafta önizlemeyi gösterir (bir resim dosyası seçiliyken).
settings_multiple_image_preview_checkbutton = Resim önizlemesini göster
settings_multiple_clear_cache_button_tooltip =
    Güncel olmayan girişlerin önbelleğini el ile temizleyin.
    Bu, yalnızca otomatik temizleme devre dışı bırakılmışsa kullanılmalıdır.
settings_multiple_clear_cache_button = Güncel olmayan girdileri önbellekten kaldır.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    Hepsi aynı verilere işaret ediyorsa (sabit bağlantılıysa), biri dışındaki tüm dosyaları gizler.
    
    Örnek: (Diskte) belirli verilere sabit bağlantılı yedi dosya ve aynı veriye ancak farklı 
    bir düğüme sahip bir farklı dosya olması durumunda, yinelenen bulucuda yalnızca bir benzersiz dosya ve 
    sabit bağlantılı dosyalardan bir dosya gösterilecektir.
settings_duplicates_minimal_size_entry_tooltip =
    Önbelleğe alınacak minimum dosya boyutunu ayarlayın.
    
    Daha küçük bir değer seçmek daha fazla kayıt üretecektir. 
    Bu, aramayı hızlandıracak, ancak önbellek yüklemeyi/kaydetmeyi yavaşlatacaktır.
settings_duplicates_prehash_checkbutton_tooltip =
    Yinelenmeyen sonuçların daha önce reddedilmesine izin veren kısmi-SUÇ 
    (dosyanın küçük bir bölümünden hesaplanan bir SUÇ) değerinin önbelleğe alınmasını sağlar.
    
    Bazı durumlarda yavaşlamaya neden olabileceğinden varsayılan olarak devre dışıdır.
    
    Aramayı birden çok kez hızlandırabileceğinden, yüz binlerce veya milyonlarca dosyayı 
    tararken kullanılması şiddetle tavsiye edilir.
settings_duplicates_prehash_minimal_entry_tooltip = Önbelleğe alınacak girişlerin minimum boyutu.
settings_duplicates_hide_hard_link_button = Sabit bağlantıları gizle (yalnızca Linux ve macOS)
settings_duplicates_prehash_checkbutton = kısmi-SUÇ önbelleği kullan
settings_duplicates_minimal_size_cache_label = Önbelleğe kaydedilen minimum dosya boyutu (bayt cinsinden):
settings_duplicates_minimal_size_cache_prehash_label = kısmi-SUÇ önbelleğine kaydedilen minimum dosya boyutu (bayt cinsinden):

## Saving/Loading settings

settings_saving_button_tooltip = Geçerli ayar yapılandırmasını dosyaya kaydeder.
settings_loading_button_tooltip = Dosyadan ayarları yükler ve geçerli yapılandırmayı bunlarla değiştirir.
settings_reset_button_tooltip = Geçerli yapılandırmayı varsayılana sıfırlar.
settings_saving_button = Yapılandırmayı kaydet
settings_loading_button = Yapılandırma yükle
settings_reset_button = Yapılandırmayı sıfırla

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    Önbellek txt dosyalarının depolandığı klasörü açar.
    
    Önbellek dosyalarının değiştirilmesi geçersiz sonuçların gösterilmesine neden olabilir. 
    Ancak, büyük miktarda dosyayı farklı bir konuma taşırken yolu değiştirmek zaman kazandırabilir.
    
    Dosyaları tekrar taramaktan zaman kazanmak için bu dosyaları bilgisayarlar arasında 
    kopyalayabilirsiniz (tabii ki benzer dizin yapısına sahiplerse).
    
    Önbellekte sorun olması durumunda bu dosyalar kaldırılabilir. Uygulama onları 
    otomatik olarak yeniden oluşturacaktır.
settings_folder_settings_open_tooltip =
    Czkawka yapılandırmasının depolandığı klasörü açar.
    
    UYARI: Yapılandırmayı elle değiştirmek iş akışınızı bozabilir.
settings_folder_cache_open = Önbellek klasörünü aç
settings_folder_settings_open = Ayarlar klasörünü aç
# Compute results
compute_stopped_by_user = Arama, kullanıcı tarafından durduruldu.
compute_found_duplicates_hash_size = { $number_groups } grupta, { $size } yer kaplayan, toplam { $number_files } adet kopya bulundu.
compute_found_duplicates_name = { $number_groups } grupta, { $number_files } adet kopya bulundu.
compute_found_empty_folders = { $number_files } adet boş klasör bulundu.
compute_found_empty_files = { $number_files } adet boş dosya bulundu.
compute_found_big_files = { $number_files } adet büyük/küçük dosya bulundu.
compute_found_temporary_files = { $number_files } adet geçici dosya bulundu.
compute_found_images = { $number_groups } grupta, { $number_files } adet benzer resim bulundu.
compute_found_videos = { $number_groups } grupta, { $number_files } adet benzer video bulundu.
compute_found_music = { $number_groups } grupta, { $number_files } adet benzer müzik dosyası bulundu.
compute_found_invalid_symlinks = { $number_files } adet geçersiz sembolik bağlantı bulundu.
compute_found_broken_files = { $number_files } adet bozuk dosya bulundu.
compute_found_bad_extensions = { $number_files } adet geçersiz uzantıya sahip dosya bulundu.
# Progress window
progress_scanning_general_file = { $file_number } dosya tarandı.
progress_scanning_extension_of_files = { $file_checked }/{ $all_files } dosyanın uzantısı kontrol edildi.
progress_scanning_broken_files = { $file_checked }/{ $all_files } dosya kontrol edildi.
progress_scanning_video = { $file_checked }/{ $all_files } videonun SUÇ kaydı oluşturuldu. ;-)
progress_scanning_image = { $file_checked }/{ $all_files } resmin SURÇ kaydı oluşturuldu. ;-)
progress_comparing_image_hashes = { $file_checked }/{ $all_files } resim SURÇ kaydı karşılaştırıldı.
progress_scanning_music_tags_end = { $file_checked }/{ $all_files } müzik dosyasının etiketleri karşılaştırıldı.
progress_scanning_music_tags = { $file_checked }/{ $all_files } müzik dosyasının etiketleri okundu.
progress_scanning_music_content_end = Müzik dosyası { $file_checked }/{ $all_files } için parmak izi karşılaştırılıyor
progress_scanning_music_content = Müzik dosyası { $file_checked }/{ $all_files } için parmak izi hesaplanıyor
progress_scanning_empty_folders = { $folder_number } klasör tarandı.
progress_scanning_size = { $file_number } dosyanın boyutu tarandı.
progress_scanning_size_name = { $file_number } dosyasının ismi ve boyutu aranıyor
progress_scanning_name = { $file_number } dosyanın adı tarandı.
progress_analyzed_partial_hash = { $file_checked }/{ $all_files } dosyanın kısmi-SUÇ kaydı analiz edildi. ;-)
progress_analyzed_full_hash = { $file_checked }/{ $all_files } dosyanın tam SUÇ kaydı analiz edildi. ;-)
progress_prehash_cache_loading = Prehash önbelleği yükleniyor
progress_prehash_cache_saving = Prehash önbelleği kaydediliyor
progress_hash_cache_loading = Hash önbelleği yükleniyor
progress_hash_cache_saving = Hash önbelleği kaydediliyor
progress_cache_loading = Önbellek yükleniyor
progress_cache_saving = Önbellek kaydediliyor
progress_current_stage = Geçerli Aşama: { " " }
progress_all_stages = Tüm Aşamalar: { " " }
# Saving loading 
saving_loading_saving_success = Yapılandırma { $name } dosyasına kaydedildi.
saving_loading_saving_failure = Yapılandırma verileri { $name } dosyasına kaydedilemedi.
saving_loading_reset_configuration = Geçerli yapılandırma temizlendi.
saving_loading_loading_success = Uygulama yapılandırması düzgünce yüklendi.
saving_loading_invalid_string = "{ $key }" anahtarı için geçersiz sonuç bulundu. "{ $result }" bir dize(tümce) değil.
saving_loading_invalid_int = "{ $key }" anahtarı için geçersiz sonuç bulundu. "{ $result }" tam sayı değil.
saving_loading_invalid_bool = "{ $key }" anahtarı için geçersiz sonuç bulundu. "{ $result }" D/Y türünde değil.
saving_loading_decode_problem_bool = "{ $key }" anahtarından D/Y kodu çözülemedi, "{ $result }" bulundu ancak izin verilen değerler 0, 1, doğru veya yanlış.
saving_loading_saving_same_keys = Ayar, yinelenen "{ $key }" anahtarıyla kaydedilmeye çalışılıyor.
saving_loading_failed_to_get_home_directory = Yapılandırma dosyasını açmak/kaydetmek için /home dizinine erşilemedi.
saving_loading_folder_config_instead_file = "{ $path }" yolunda kaydetme yapılandırma dosyası oluşturulamıyor veya açılamıyor çünkü zaten bir klasör var.
saving_loading_failed_to_create_configuration_folder = Yapılandırma klasörü "{ $path }" dizini oluşturulamadı, nedeni: "{ $reason }".
saving_loading_failed_to_create_config_file = "{ $path }" dizininde yapılandırma dosyası oluşturulamadı, nedeni:  "{ $reason }".
saving_loading_failed_to_read_config_file = "{ $path }" dizininden yapılandırma dosyası yüklenemiyor, böyle dosya yok ya da bir dosya değil.
saving_loading_failed_to_read_data_from_file = "{ $path }" dosyasından veri okunamıyor, nedeni: "{ $reason }".
saving_loading_orphan_data = "{ $line }" satırda "{ $data }" ilişiksiz veri bulundu.
saving_loading_not_valid = "{ $data }" ayarı geçerli uygulama sürümünde bulunmuyor.
# Invalid symlinks
invalid_symlink_infinite_recursion = Sonsuz özyineleme
invalid_symlink_non_existent_destination = Var olmayan hedef dosya
# Other
selected_all_reference_folders = Tüm dizinler, "Başvuru Klasörü" olarak ayarlandığında arama başlatılamaz.
searching_for_data = İşleminiz yürütülüyor, bu biraz zaman alabilir, lütfen bekleyin...
text_view_messages = MESAJLAR
text_view_warnings = UYARILAR
text_view_errors = HATALAR
about_window_motto = Bu programın kullanımı ücretsizdir ve her zaman öyle kalacaktır.
# Various dialog
dialogs_ask_next_time = Bir dahaki sefere sor
delete_file_failed = { $name } dosyası silinemedi, nedeni: { $reason }
delete_title_dialog = Silmeyi onaylayın.
delete_question_label = Dosyaları silmek istediğinizden emin misiniz?
delete_all_files_in_group_title = Gruptaki tüm dosyaları silmeyi onaylayın.
delete_all_files_in_group_label1 = Kimi gruplarda tüm kayıtlar seçilir.
delete_all_files_in_group_label2 = Bunları silmek istediğinizden emin misiniz?
delete_folder_failed = { $dir } klasörü; bulunmadığı, izniniz olmadığı veya klasör boş olmadığı için silinemedi.
delete_items_label = { $items } dosya silinecek.
delete_items_groups_label = { $groups } gruptan { $items } dosya silinecek.
hardlink_failed = Sabit bağlantı kurulamadı.
hard_sym_invalid_selection_title_dialog = Kimi gruplarda geçersiz seçim
hard_sym_invalid_selection_label_1 = Bazı gruplarda sadece bir kayıt seçilmiştir ve bu kayıt yok sayılacaktır.
hard_sym_invalid_selection_label_2 = Bu dosyaları sabit/sembolik bağlayabilmek için gruptaki en az iki sonucun seçilmesi gerekir.
hard_sym_invalid_selection_label_3 = Gruptaki ilk resim asıl olarak tanınır ve değiştirilmez, ancak ikinci ve sonrakiler değiştirilir.
hard_sym_link_title_dialog = Bağlantı vermeyi onaylayın
hard_sym_link_label = Bu dosyaları bağlamak istediğinizden emin misiniz?
move_folder_failed = { $name } klasörü taşınamadı, nedeni: { $reason }
move_file_failed = { $name } dosyası taşınamadı, nedeni: { $reason }
move_files_title_dialog = Eş dosyaları taşımak istediğiniz klasörü seçin
move_files_choose_more_than_1_path = Eş dosyaları taşıyabilmek için yalnızca bir yol seçilebilir, { $path_number } seçildi.
move_stats = { $num_files }/{ $all_files } öğe düzgün şekilde taşındı.
save_results_to_file = Sonuçları hem txt hem de json formatında { $name } klasörüne kaydeder.
search_not_choosing_any_music = HATA: Müzik araması için en az bir onay kutusu seçmelisiniz.
search_not_choosing_any_broken_files = HATA: Bozuk dosya araması için en az bir onay kutusu seçmelisiniz.
include_folders_dialog_title = Aranacak Klasörler
exclude_folders_dialog_title = Hariç Tutulan Klasörler
include_manually_directories_dialog_title = Dizini elle ekle
cache_properly_cleared = Önbellek, uygun şekilde temizlendi.
cache_clear_duplicates_title = Eş dosyalar önbelleğini temizle
cache_clear_similar_images_title = Benzer resimler önbelleğini temizle
cache_clear_similar_videos_title = Benzer videolar önbelleğini temizle
cache_clear_message_label_1 = Güncel olmayan girişleri önbellekten temizlemek istiyor musunuz?
cache_clear_message_label_2 = Bu işlem, geçersiz dosyalara işaret eden tüm önbellek girişlerini kaldıracak.
cache_clear_message_label_3 = Bu, önbelleğe yükleme/kaydetme işlemini biraz hızlandırabilir.
cache_clear_message_label_4 = UYARI: İşlem, takılı olmayan harici sürücülerden önbelleğe alınmış tüm verileri kaldıracaktır. Yani her hash kaydının yeniden oluşturulması gerekecek.
# Show preview
preview_image_resize_failure = { $name } adlı resim yeniden boyutlandırılamadı.
preview_image_opening_failure = { $name } adlı resim dosyası açılamadı, nedeni: { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = Grup: { $current_group }/{ $all_groups } ({ $images_in_group } resim)
compare_move_left_button = <-
compare_move_right_button = ->
