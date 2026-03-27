# Window titles
window_settings_title = सेटिंग्स
window_main_title = Czkawka (Hiccup)
window_progress_title = स्कैनिंग
window_compare_images = छवियों की तुलना करें
# General
general_ok_button = Ok
general_close_button = बंद करें
# Krokiet info dialog
krokiet_info_title = पेश है क्रोकीट - Czkawka का नया संस्करण
krokiet_info_message = 
        Krokiet, Czkawka GTK GUI का एक नया, बेहतर, तेज़ और अधिक विश्वसनीय संस्करण है!

        यह चलाने में आसान है और सिस्टम परिवर्तनों के प्रति अधिक लचीला है, क्योंकि यह केवल उन मुख्य पुस्तकालयों पर निर्भर करता है जो अधिकांश प्रणालियों पर डिफ़ॉल्ट रूप से उपलब्ध होते हैं।

        Krokiet में वे सुविधाएँ भी हैं जो Czkawka में नहीं हैं, जिनमें वीडियो तुलना मोड में थंबनेल, एक EXIF क्लीनर, फ़ाइल को स्थानांतरित/कॉपी/हटाने की प्रगति या उन्नत सॉर्टिंग विकल्प शामिल हैं।

        इसे आज़माएं और अंतर देखें!

        मैं Czkawka को बग फिक्स और मामूली अपडेट देना जारी रखूंगा, लेकिन सभी नई सुविधाएँ केवल Krokiet के लिए विकसित की जाएंगी, और कोई भी नई सुविधाएँ जोड़ने, गुम मोड जोड़ने या Czkawka को और विस्तारित करने के लिए स्वतंत्र है।

        PS: यह संदेश केवल एक बार दिखाई देना चाहिए। यदि यह फिर से दिखाई देता है, तो CZKAWKA_DONT_ANNOY_ME पर्यावरण चर को किसी भी गैर-रिक्त मान पर सेट करें।.
# Main window
music_title_checkbox = शीर्षक
music_artist_checkbox = कलाकार
music_year_checkbox = वर्ष
music_bitrate_checkbox = बिटरेट
music_genre_checkbox = शैली
music_length_checkbox = लंबाई
music_comparison_checkbox = अनुमानित तुलना।
music_checking_by_tags = टैग्स
music_checking_by_content = सामग्री
same_music_seconds_label = न्यूनतम खंड, दूसरा अवधि
same_music_similarity_label = अधिकतम अंतर
music_compare_only_in_title_group = समान शीर्षकों वाले समूहों के भीतर तुलना करें
music_compare_only_in_title_group_tooltip = 
        जब यह सुविधा सक्रिय होती है, तो फ़ाइलें शीर्षक के आधार पर समूहीकृत की जाती हैं और फिर एक-दूसरे से तुलना की जाती है।

        10,000 फ़ाइलों के साथ, आमतौर पर लगभग 100 मिलियन तुलनाएँ होती हैं, लेकिन इस सुविधा के साथ लगभग 20,000 तुलनाएँ ही होंगी।.
same_music_tooltip = 
        समान संगीत फ़ाइलों को उनके कंटेंट के आधार पर खोजने के लिए, आप निम्नलिखित सेटिंग्स को कॉन्फ़िगर कर सकते हैं:

        - वह न्यूनतम समय अवधि जिसके बाद संगीत फ़ाइलों को समान के रूप में पहचाना जा सकता है।
        - दो परीक्षण किए जा रहे टुकड़ों के बीच अधिकतम अंतर।

        अच्छे परिणामों के लिए, इन मापदंडों के उचित संयोजनों को खोजना महत्वपूर्ण है, जैसा कि नीचे बताया गया है।

        न्यूनतम समय को 5 सेकंड और अधिकतम अंतर को 1.0 पर सेट करने से, फ़ाइलों में लगभग समान टुकड़े खोजे जाएंगे।
        दूसरी ओर, 20 सेकंड का समय और 6.0 का अधिकतम अंतर, रीमिक्स/लाइव संस्करण आदि खोजने के लिए अच्छा काम करता है।

        डिफ़ॉल्ट रूप से, प्रत्येक संगीत फ़ाइल की तुलना अन्य सभी फ़ाइलों से की जाती है, और जब बड़ी संख्या में फ़ाइलों का परीक्षण किया जाता है, तो इसमें बहुत समय लग सकता है। इसलिए, आमतौर पर, संदर्भ फ़ोल्डरों का उपयोग करना और यह निर्दिष्ट करना बेहतर होता है कि किन फ़ाइलों की एक-दूसरे के साथ तुलना की जानी चाहिए (समान संख्या में फ़ाइलों की तुलना करने से, संदर्भ फ़ोल्डरों के बिना तुलना करने की तुलना में कम से कम 4 गुना तेजी से परिणाम प्राप्त होते हैं)।.
music_comparison_checkbox_tooltip = 
        यह कृत्रिम बुद्धिमत्ता (AI) का उपयोग करके समान संगीत फ़ाइलों की खोज करता है, जो मशीन लर्निंग का उपयोग करके एक वाक्यांश से कोष्ठक को हटा देता है। उदाहरण के लिए, इस विकल्प को सक्षम करने पर, निम्नलिखित फ़ाइलों को डुप्लिकेट माना जाएगा:

        Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = केस सेंसिटिव (Case Sensitive)
duplicate_case_sensitive_name_tooltip = 
        जब यह विकल्प सक्षम होता है, तो केवल उन रिकॉर्डों को एक साथ समूहीकृत किया जाता है जिनके नाम बिल्कुल समान होते हैं, उदाहरण के लिए: Żołd <-> Żołd.

        इस विकल्प को अक्षम करने पर, नामों को समूहीकृत किया जाता है बिना यह जांचे कि क्या प्रत्येक अक्षर का आकार समान है, उदाहरण के लिए: żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = आकार और नाम
duplicate_mode_name_combo_box = नाम
duplicate_mode_size_combo_box = आकार
duplicate_mode_hash_combo_box = Hash
duplicate_hash_type_tooltip = 
        Czkawka 3 प्रकार के हैश प्रदान करता है:

        Blake3 - एक क्रिप्टोग्राफिक हैश फ़ंक्शन। यह डिफ़ॉल्ट रूप से उपयोग किया जाता है क्योंकि यह बहुत तेज़ है।

        CRC32 - एक सरल हैश फ़ंक्शन। यह Blake3 से तेज़ होना चाहिए, लेकिन इसमें बहुत ही कम मामलों में कुछ टकराव हो सकते हैं।

        XXH3 - प्रदर्शन और हैश गुणवत्ता में Blake3 के समान (लेकिन गैर-क्रिप्टोग्राफिक)। इसलिए, इन मोडों को आसानी से बदला जा सकता है।.
duplicate_check_method_tooltip = 
        फिलहाल, Czkawka डुप्लिकेट फ़ाइलें खोजने के लिए तीन प्रकार के तरीके प्रदान करता है:

        नाम - उन फ़ाइलों को ढूंढता है जिनका नाम समान है।

        आकार - उन फ़ाइलों को ढूंढता है जिनका आकार समान है।

        हैश - उन फ़ाइलों को ढूंढता है जिनकी सामग्री समान है। यह मोड फ़ाइल का हैश उत्पन्न करता है और बाद में डुप्लिकेट खोजने के लिए इस हैश की तुलना करता है। यह डुप्लिकेट खोजने का सबसे सुरक्षित तरीका है। ऐप भारी मात्रा में कैश का उपयोग करता है, इसलिए एक ही डेटा का दूसरा और बाद का स्कैन पहले स्कैन की तुलना में बहुत तेज़ होना चाहिए।.
image_hash_size_tooltip = 
        प्रत्येक जाँच की गई छवि एक विशेष हैश उत्पन्न करती है, जिसकी तुलना एक-दूसरे से की जा सकती है, और उनके बीच एक छोटा अंतर इंगित करता है कि ये छवियाँ समान हैं।

        8 का हैश आकार उन छवियों को खोजने के लिए काफी अच्छा है जो मूल छवि के थोड़े ही समान हैं। बड़ी संख्या में छवियों (>1000) के साथ, यह बड़ी संख्या में गलत सकारात्मक परिणाम देगा, इसलिए मैं इस मामले में एक बड़े हैश आकार का उपयोग करने की अनुशंसा करता हूँ।

        16 डिफ़ॉल्ट हैश आकार है, जो समान छवियों को खोजने और हैश टकरावों की संख्या को कम रखने के बीच एक अच्छा संतुलन प्रदान करता है।

        32 और 64 हैश केवल बहुत समान छवियों को खोजते हैं, लेकिन उनमें लगभग कोई गलत सकारात्मक परिणाम नहीं होना चाहिए (शायद कुछ छवियों में अल्फा चैनल के कारण)।.
image_resize_filter_tooltip = 
        इमेज का हैश निकालने के लिए, लाइब्रेरी को पहले उसे रीसाइज करना होगा।

        चुने गए एल्गोरिदम के आधार पर, हैश की गणना के लिए उपयोग की जाने वाली इमेज थोड़ी अलग दिख सकती है।

        सबसे तेज़ एल्गोरिदम "Nearest" है, लेकिन यह सबसे खराब परिणाम भी देता है। यह डिफ़ॉल्ट रूप से सक्षम है, क्योंकि 16x16 के हैश आकार के साथ, निम्न गुणवत्ता में यह ज़्यादा दिखाई नहीं देता।

        8x8 के हैश आकार के साथ, बेहतर समूहों में छवियों को प्राप्त करने के लिए "Nearest" के अलावा किसी अन्य एल्गोरिदम का उपयोग करने की अनुशंसा की जाती है।.
image_hash_alg_tooltip = 
        उपयोगकर्ता कई एल्गोरिदम में से किसी एक को चुन सकते हैं जिसका उपयोग हैश की गणना करने के लिए किया जाता है।

        प्रत्येक एल्गोरिदम के अपने फायदे और नुकसान हैं, और कभी-कभी वे विभिन्न छवियों के लिए बेहतर परिणाम देते हैं, जबकि कभी-कभी खराब परिणाम देते हैं।

        इसलिए, आपके लिए सबसे अच्छा एल्गोरिदम निर्धारित करने के लिए, मैन्युअल परीक्षण की आवश्यकता होती है।.
big_files_mode_combobox_tooltip = यह सुविधा सबसे छोटी/सबसे बड़ी फ़ाइलों को खोजने की अनुमति देती है।
big_files_mode_label = जांच की गई फ़ाइलें
big_files_mode_smallest_combo_box = सबसे छोटा
big_files_mode_biggest_combo_box = सबसे बड़ा।
main_notebook_duplicates = डुप्लिकेट फ़ाइलें
main_notebook_empty_directories = खाली डायरेक्टरीज़
main_notebook_big_files = बड़ी फ़ाइलें
main_notebook_empty_files = खाली फ़ाइलें
main_notebook_temporary = अस्थायी फ़ाइलें
main_notebook_similar_images = समान चित्र
main_notebook_similar_videos = समान वीडियो
main_notebook_same_music = संगीत की प्रतियां
main_notebook_symlinks = अमान्य सिंबोलिक लिंक
main_notebook_broken_files = टूटी हुई फाइलें
main_notebook_bad_extensions = खराब एक्सटेंशन
main_tree_view_column_file_name = फ़ाइल का नाम
main_tree_view_column_folder_name = फ़ोल्डर का नाम
main_tree_view_column_path = रास्ता
main_tree_view_column_modification = संशोधन की तारीख
main_tree_view_column_size = आकार
main_tree_view_column_similarity = समानता
main_tree_view_column_dimensions = आयाम
main_tree_view_column_title = शीर्षक
main_tree_view_column_artist = कलाकार
main_tree_view_column_year = वर्ष
main_tree_view_column_bitrate = बिटरेट
main_tree_view_column_length = लंबाई
main_tree_view_column_genre = शैली
main_tree_view_column_symlink_file_name = सिंबोलिक लिंक फ़ाइल का नाम
main_tree_view_column_symlink_folder = सिंबॉलिक लिंक फ़ोल्डर
main_tree_view_column_destination_path = गंतव्य पथ
main_tree_view_column_type_of_error = त्रुटि का प्रकार
main_tree_view_column_current_extension = वर्तमान एक्सटेंशन
main_tree_view_column_proper_extensions = सही एक्सटेंशन
main_tree_view_column_fps = FPS
main_tree_view_column_codec = कोडेक
main_label_check_method = जांच विधि।
main_label_hash_type = हैश का प्रकार
main_label_hash_size = हैश का आकार
main_label_size_bytes = आकार (बाइट में)
main_label_min_size = न्यूनतम
main_label_max_size = मैक्स
main_label_shown_files = दिखाए गए फ़ाइलों की संख्या
main_label_resize_algorithm = आकार बदलने का एल्गोरिदम
main_label_similarity = Similarity{ "   " }
main_check_box_broken_files_audio = ऑडियो
main_check_box_broken_files_pdf = Pdf
main_check_box_broken_files_archive = संग्रह
main_check_box_broken_files_image = छवि
main_check_box_broken_files_video = वीडियो
main_check_box_broken_files_video_tooltip = यह ffmpeg/ffprobe का उपयोग करके वीडियो फ़ाइलों की जांच करता है। यह प्रक्रिया धीमी हो सकती है और यह छोटी-छोटी गलतियों को भी पकड़ सकता है, भले ही फ़ाइल ठीक से चल रही हो।.
check_button_general_same_size = एक ही आकार को अनदेखा करें
check_button_general_same_size_tooltip = परिणामों में समान आकार वाली फ़ाइलों को अनदेखा करें - आमतौर पर ये 1:1 प्रतियां होती हैं
main_label_size_bytes_tooltip = उन फ़ाइलों का आकार जिनका उपयोग स्कैन में किया जाएगा
# Upper window
upper_tree_view_included_folder_column_title = खोजने के लिए फ़ोल्डर:
upper_tree_view_included_reference_column_title = संदर्भ फ़ोल्डर
upper_recursive_button = पुनरावर्ती
upper_recursive_button_tooltip = यदि चुना गया है, तो उन फ़ाइलों की भी खोज करें जो चुने गए फ़ोल्डरों के सीधे अंदर स्थित नहीं हैं।.
upper_manual_add_included_button = मैन्युअल रूप से जोड़ें
upper_add_included_button = जोड़ें
upper_remove_included_button = हटाएँ।
upper_manual_add_excluded_button = मैन्युअल रूप से जोड़ें
upper_add_excluded_button = जोड़ें
upper_remove_excluded_button = हटाएँ।
upper_manual_add_included_button_tooltip = 
        हाथ से डायरेक्टरी का नाम खोज में जोड़ें।

        एक साथ कई पाथ जोड़ने के लिए, उन्हें ";" से अलग करें।

        "/home/roman;/home/rozkaz" दो डायरेक्टरी /home/roman और /home/rozkaz को जोड़ देगा।
upper_add_included_button_tooltip = खोज में एक नया फ़ोल्डर जोड़ें.
upper_remove_included_button_tooltip = सर्च से डायरेक्टरी हटाएं.
upper_manual_add_excluded_button_tooltip = 
        हाथ से बहिष्कृत निर्देशिका का नाम जोड़ें।

        एक साथ कई पाथ जोड़ने के लिए, उन्हें ";" से अलग करें।

        "/home/roman;/home/krokiet" दो निर्देशिकाएँ "/home/roman" और "/home/keokiet" जोड़ देगा।
upper_add_excluded_button_tooltip = खोज में शामिल न किए जाने वाले डायरेक्टरी को जोड़ें.
upper_remove_excluded_button_tooltip = फ़ोल्डर को "बहिष्कृत" सूची से हटाएँ.
upper_notebook_items_configuration = वस्तुओं का कॉन्फ़िगरेशन
upper_notebook_excluded_directories = बहिष्कृत पथ
upper_notebook_included_directories = शामिल पथ।
upper_allowed_extensions_tooltip = 
        अनुमत एक्सटेंशन अल्पविराम (,) से अलग किए जाने चाहिए (डिफ़ॉल्ट रूप से सभी उपलब्ध हैं)।

        निम्नलिखित मैक्रो भी उपलब्ध हैं, जो एक साथ कई एक्सटेंशन जोड़ते हैं: IMAGE, VIDEO, MUSIC, TEXT.

        उपयोग का उदाहरण: ".exe, IMAGE, VIDEO, .rar, 7z" - इसका मतलब है कि छवियों (जैसे jpg, png), वीडियो (जैसे avi, mp4), exe, rar और 7z फ़ाइलों को स्कैन किया जाएगा।.
upper_excluded_extensions_tooltip = 
        उन फ़ाइलों की सूची जिन्हें स्कैन के दौरान अनदेखा किया जाएगा।

        जब आप अनुमत और अक्षम दोनों एक्सटेंशन का उपयोग करते हैं, तो यह सेटिंग अधिक महत्वपूर्ण होती है, इसलिए फ़ाइल की जांच नहीं की जाएगी।.
upper_excluded_items_tooltip = 
        बहिष्कृत किए जाने वाले आइटमों में एक * वाइल्डकार्ड होना चाहिए और उन्हें अल्पविराम से अलग किया जाना चाहिए।
        यह "Excluded Paths" की तुलना में धीमा है, इसलिए इसका सावधानीपूर्वक उपयोग करें।.
upper_excluded_items = बहिष्कृत वस्तुएँ:
upper_allowed_extensions = अनुमत एक्सटेंशन:
upper_excluded_extensions = अक्षम किए गए एक्सटेंशन:
# Popovers
popover_select_all = सभी का चयन करें
popover_unselect_all = सभी का चयन रद्द करें
popover_reverse = उलट चयन
popover_select_all_except_shortest_path = सभी का चयन करें, सिवाय सबसे छोटे मार्ग के
popover_select_all_except_longest_path = सभी का चयन करें, सिवाय सबसे लंबे पथ के
popover_select_all_except_oldest = सभी को चुनें, सिवाय सबसे पुराने के
popover_select_all_except_newest = सभी का चयन करें, सिवाय सबसे नए वाले के
popover_select_one_oldest = एक सबसे पुराना विकल्प चुनें
popover_select_one_newest = एक नवीनतम विकल्प चुनें।
popover_select_custom = कस्टम विकल्प चुनें
popover_unselect_custom = अनुकूलित विकल्प को हटा दें
popover_select_all_images_except_biggest = सभी को चुनें, लेकिन सबसे बड़े को छोड़कर
popover_select_all_images_except_smallest = सभी का चयन करें, लेकिन सबसे छोटा नहीं।
popover_custom_path_check_button_entry_tooltip = 
        पाथ के आधार पर रिकॉर्ड चुनें।

        उदाहरण उपयोग:
        `/home/pimpek/rzecz.txt` को `/home/pim*` का उपयोग करके खोजा जा सकता है
popover_custom_name_check_button_entry_tooltip = 
        फ़ाइल नामों के आधार पर रिकॉर्ड चुनें।

        उदाहरण:
        `/usr/ping/pong.txt` को `*ong*` का उपयोग करके खोजा जा सकता है।
popover_custom_regex_check_button_entry_tooltip = 
        निर्दिष्ट रेगुलर एक्सप्रेशन (Regex) का उपयोग करके रिकॉर्ड चुनें।

        इस मोड में, खोजा गया टेक्स्ट "पाथ विद नेम" होता है।

        उदाहरण उपयोग:
        `/usr/bin/ziemniak.txt` को `/ziem[a-z]+/` के साथ खोजा जा सकता है।

        यह डिफ़ॉल्ट रस्ट रेगुलर एक्सप्रेशन कार्यान्वयन का उपयोग करता है। इसके बारे में आप यहां अधिक जानकारी प्राप्त कर सकते हैं: https://docs.rs/regex.
popover_custom_case_sensitive_check_button_tooltip = 
        केस-संवेदी पहचान को सक्षम करता है।

        जब अक्षम किया जाता है, तो /home/* दोनों /HoMe/roman और /home/roman को खोजता है।.
popover_custom_not_all_check_button_tooltip = 
        समूह में सभी रिकॉर्ड का चयन करने से रोकता है।

        यह डिफ़ॉल्ट रूप से सक्षम होता है, क्योंकि अधिकांश स्थितियों में, आप मूल और डुप्लिकेट दोनों फ़ाइलों को डिलीट नहीं करना चाहते हैं, बल्कि कम से कम एक फ़ाइल को रखना चाहते हैं।

        चेतावनी: यह सेटिंग काम नहीं करेगी यदि आपने पहले से ही किसी समूह में सभी परिणामों को मैन्युअल रूप से चुना है।.
popover_custom_regex_path_label = रास्ता
popover_custom_regex_name_label = नाम
popover_custom_regex_regex_label = रेगेक्स पाथ + नाम
popover_custom_case_sensitive_check_button = केस संवेदी
popover_custom_all_in_group_label = समूह में सभी रिकॉर्ड का चयन न करें
popover_custom_mode_unselect = अनुकूलित विकल्प को अनचेक करें
popover_custom_mode_select = कस्टम विकल्प चुनें
popover_sort_file_name = फ़ाइल का नाम
popover_sort_folder_name = फ़ोल्डर का नाम
popover_sort_full_name = पूरा नाम
popover_sort_size = आकार
popover_sort_selection = चयन
popover_invalid_regex = रेगेक्स अमान्य है
popover_valid_regex = रेगेक्स मान्य है
# Bottom buttons
bottom_search_button = खोजें
bottom_select_button = चयन करें।
bottom_delete_button = हटाएँ
bottom_save_button = सहेजें।
bottom_symlink_button = Symlink
bottom_hardlink_button = Hardlink
bottom_move_button = आगे बढ़ें।
bottom_sort_button = क्रमबद्ध करें।
bottom_compare_button = तुलना करें।
bottom_search_button_tooltip = खोज शुरू करें
bottom_select_button_tooltip = रिकॉर्ड चुनें। केवल चुने गए फ़ाइलें/फ़ोल्डर ही बाद में संसाधित किए जा सकते हैं।.
bottom_delete_button_tooltip = चुने गए फ़ाइलों/फ़ोल्डरों को हटाएं.
bottom_save_button_tooltip = खोज के बारे में जानकारी को फ़ाइल में सहेजें
bottom_symlink_button_tooltip = 
        सिम्बॉलिक लिंक बनाएँ।
        यह केवल तभी काम करता है जब किसी समूह में कम से कम दो परिणाम चुने गए हों।
        पहला परिणाम अपरिवर्तित रहता है, और दूसरा और उसके बाद के सभी परिणाम पहले परिणाम से सिम्बॉलिक लिंक किए जाते हैं।.
bottom_hardlink_button_tooltip = 
        हार्डलिंक बनाएँ।
        यह केवल तभी काम करता है जब किसी समूह में कम से कम दो परिणाम चुने गए हों।
        पहला परिणाम अपरिवर्तित रहेगा, और दूसरा और बाद के परिणाम पहले परिणाम से हार्डलिंक किए जाएंगे।.
bottom_hardlink_button_not_available_tooltip = 
        हार्डलिंक बनाएँ।
        बटन निष्क्रिय है, क्योंकि हार्डलिंक नहीं बनाए जा सकते।
        हार्डलिंक केवल विंडोज पर व्यवस्थापक अधिकारों के साथ काम करते हैं, इसलिए सुनिश्चित करें कि आप ऐप को व्यवस्थापक के रूप में चला रहे हैं।
        यदि ऐप पहले से ही इस तरह के अधिकारों के साथ काम कर रहा है, तो गिटहब पर इसी तरह की समस्याओं की जांच करें।.
bottom_move_button_tooltip = 
        फ़ाइलों को चुने हुए फ़ोल्डर में ले जाया जाता है।
        यह सभी फ़ाइलों को बिना किसी फ़ोल्डर संरचना को बनाए उस फ़ोल्डर में कॉपी करता है।
        यदि आप एक ही नाम की दो फ़ाइलों को किसी फ़ोल्डर में ले जाने की कोशिश करते हैं, तो दूसरी फ़ाइल विफल हो जाएगी और एक त्रुटि दिखाई देगी।.
bottom_sort_button_tooltip = फ़ाइलों/फ़ोल्डरों को चयनित विधि के अनुसार क्रमबद्ध करता है.
bottom_compare_button_tooltip = समूह में मौजूद चित्रों की तुलना करें.
bottom_show_errors_tooltip = नीचे दिए गए टेक्स्ट पैनल को दिखाएं/छिपाएं।.
bottom_show_upper_notebook_tooltip = ऊपरी नोटबुक पैनल दिखाएँ/छिपाएँ.
# Progress Window
progress_stop_button = रुकें।
progress_stop_additional_message = अनुरोधित रोक लागू की गई।
# About Window
about_repository_button_tooltip = स्रोत कोड वाले रिपॉजिटरी पेज का लिंक।.
about_donation_button_tooltip = दान करने के लिए पेज का लिंक।.
about_instruction_button_tooltip = निर्देश पृष्ठ का लिंक.
about_translation_button_tooltip = एप्लिकेशन के अनुवादों के लिए क्राउडिन पेज का लिंक। आधिकारिक तौर पर पोलिश और अंग्रेजी भाषाएं समर्थित हैं।.
about_repository_button = रिपॉजिटरी
about_donation_button = दान
about_instruction_button = निर्देश।
about_translation_button = अनुवाद
# Header
header_setting_button_tooltip = सेटिंग्स डायलॉग बॉक्स खोलता है।.
header_about_button_tooltip = ऐप के बारे में जानकारी वाली एक संवाद विंडो खुलती है.

# Settings


## General

settings_number_of_threads = उपयोग किए गए थ्रेड्स की संख्या
settings_number_of_threads_tooltip = उपयोग किए गए थ्रेड की संख्या, 0 का अर्थ है कि सभी उपलब्ध थ्रेड का उपयोग किया जाएगा.
settings_use_rust_preview = जीटीके (GTK) के बजाय, पूर्वावलोकन लोड करने के लिए बाहरी पुस्तकालयों का उपयोग करें
settings_use_rust_preview_tooltip = 
        gtk प्रीव्यू का उपयोग करने से कभी-कभी यह अधिक तेज़ हो सकता है और अधिक फ़ॉर्मेट का समर्थन कर सकता है, लेकिन कभी-कभी यह बिल्कुल विपरीत भी हो सकता है।

        यदि आपको प्रीव्यू लोड करने में समस्या हो रही है, तो आप इस सेटिंग को बदलने का प्रयास कर सकते हैं।

        लिनक्स के अलावा अन्य प्रणालियों पर, इस विकल्प का उपयोग करने की अनुशंसा की जाती है, क्योंकि gtk-pixbuf हमेशा उपलब्ध नहीं होते हैं, इसलिए इस विकल्प को अक्षम करने से कुछ छवियों के प्रीव्यू लोड नहीं होंगे।.
settings_label_restart = आपको सेटिंग्स लागू करने के लिए ऐप को पुनः चालू करना होगा!
settings_ignore_other_filesystems = अन्य फ़ाइल सिस्टम को अनदेखा करें (केवल लिनक्स के लिए)।
settings_ignore_other_filesystems_tooltip = 
        यह उन फ़ाइलों को अनदेखा कर देता है जो खोजे गए डायरेक्टरीज़ के समान फ़ाइल सिस्टम में नहीं हैं।

        यह लिनक्स में `find` कमांड में `-xdev` विकल्प की तरह ही काम करता है।
settings_save_at_exit_button_tooltip = एप्लिकेशन बंद करते समय, कॉन्फ़िगरेशन को फ़ाइल में सहेजें।.
settings_load_at_start_button_tooltip = 
        ऐप खोलते समय, फ़ाइल से कॉन्फ़िगरेशन लोड करें।

        यदि यह सक्षम नहीं है, तो डिफ़ॉल्ट सेटिंग्स का उपयोग किया जाएगा।.
settings_confirm_deletion_button_tooltip = जब डिलीट बटन पर क्लिक किया जाए, तो एक पुष्टिकरण संवाद प्रदर्शित करें।.
settings_confirm_link_button_tooltip = जब आप हार्ड लिंक/सिम्बॉलिक लिंक बटन पर क्लिक करते हैं, तो एक पुष्टिकरण संवाद दिखाएं.
settings_confirm_group_deletion_button_tooltip = जब आप किसी समूह से सभी रिकॉर्ड हटाने की कोशिश करते हैं, तो एक चेतावनी संवाद प्रदर्शित करें.
settings_show_text_view_button_tooltip = उपयोगकर्ता इंटरफ़ेस के निचले भाग में एक टेक्स्ट पैनल प्रदर्शित करें.
settings_use_cache_button_tooltip = फ़ाइल कैश का उपयोग करें.
settings_save_also_as_json_button_tooltip = कैश को (मानव-पठनीय) JSON प्रारूप में सहेजें। इसकी सामग्री को संशोधित करना संभव है। यदि बाइनरी प्रारूप में कैश (जिसका एक्सटेंशन "bin" है) मौजूद नहीं है, तो ऐप इस फ़ाइल से स्वचालित रूप से कैश पढ़ेगा।.
settings_use_trash_button_tooltip = फ़ाइलों को स्थायी रूप से हटाने के बजाय, उन्हें ट्रैश में स्थानांतरित कर दिया जाता है।.
settings_language_label_tooltip = उपयोगकर्ता इंटरफ़ेस के लिए भाषा।.
settings_save_at_exit_button = ऐप बंद करते समय कॉन्फ़िगरेशन सहेजें
settings_load_at_start_button = ऐप खोलने पर कॉन्फ़िगरेशन लोड करें
settings_confirm_deletion_button = किसी भी फ़ाइल को डिलीट करते समय, पुष्टि संवाद प्रदर्शित करें
settings_confirm_link_button = जब भी कोई फ़ाइल हार्ड लिंक या सिम्लिंक की जाती है, तो एक पुष्टिकरण डायलॉग दिखाएं
settings_confirm_group_deletion_button = जब आप किसी समूह में मौजूद सभी फ़ाइलें डिलीट करते हैं, तो एक पुष्टि संवाद दिखाएं।
settings_show_text_view_button = नीचे वाला टेक्स्ट पैनल दिखाएँ
settings_use_cache_button = कैश का उपयोग करें
settings_save_also_as_json_button = इसके अतिरिक्त, कैश को JSON फ़ाइल के रूप में भी सहेजें
settings_use_trash_button = डिलीट की गई फ़ाइलों को रीसायकल बिन में ले जाएं
settings_language_label = भाषा
settings_multiple_delete_outdated_cache_checkbutton = स्वचालित रूप से पुरानी कैशे प्रविष्टियों को हटाएं
settings_multiple_delete_outdated_cache_checkbutton_tooltip = 
        पुरानी कैश्ड प्रविष्टियों को हटा दें जो अब मौजूद नहीं वाली फ़ाइलों की ओर इशारा करती हैं।

        जब यह सुविधा सक्षम होती है, तो ऐप यह सुनिश्चित करता है कि रिकॉर्ड लोड करते समय, सभी रिकॉर्ड मान्य फ़ाइलों की ओर इशारा करें (खराब रिकॉर्ड को अनदेखा कर दिया जाता है)।

        इस सुविधा को अक्षम करने से, बाहरी ड्राइव पर फ़ाइलों को स्कैन करते समय मदद मिलेगी, ताकि उन फ़ाइलों के बारे में कैश्ड प्रविष्टियाँ अगले स्कैन में हटाए नहीं जाएं।

        यदि आपके पास कैशे में लाखों रिकॉर्ड हैं, तो इस सुविधा को सक्षम करने की अनुशंसा की जाती है, जिससे स्कैन की शुरुआत/अंत में कैशे को लोड/सेव करने की गति बढ़ जाएगी।.
settings_notebook_general = सामान्य
settings_notebook_duplicates = डुप्लिकेट।
settings_notebook_images = समान छवियां
settings_notebook_videos = समान वीडियो

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = दाहिने तरफ एक पूर्वावलोकन प्रदर्शित होता है (जब आप कोई छवि फ़ाइल चुनते हैं)।.
settings_multiple_image_preview_checkbutton = छवि का पूर्वावलोकन दिखाएं
settings_multiple_clear_cache_button_tooltip = 
        पुराने प्रविष्टियों की कैशे को मैन्युअल रूप से साफ़ करें.
        इसका उपयोग केवल तभी किया जाना चाहिए जब स्वचालित सफाई को अक्षम कर दिया गया हो.
settings_multiple_clear_cache_button = कैश से पुरानी प्रविष्टियों को हटा दें.

## Duplicates

settings_duplicates_hide_hard_link_button_tooltip = 
        यह सभी फ़ाइलों को छिपा देता है, सिवाय एक के, यदि सभी एक ही डेटा की ओर इशारा करते हैं (यानी, हार्डलिंक्ड हैं)।

        उदाहरण: यदि डिस्क पर सात फ़ाइलें हैं जो किसी विशिष्ट डेटा से हार्डलिंक्ड हैं, और एक अलग फ़ाइल है जिसमें वही डेटा है लेकिन एक अलग इनोड है, तो डुप्लिकेट फाइंडर में, केवल एक अद्वितीय फ़ाइल और हार्डलिंक्ड फ़ाइलों में से एक फ़ाइल दिखाई देगी।.
settings_duplicates_minimal_size_entry_tooltip = 
        कैश में संग्रहीत किए जाने वाले न्यूनतम फ़ाइल आकार को सेट करें।

        एक छोटा मान चुनने से अधिक रिकॉर्ड उत्पन्न होंगे। इससे खोज तेज हो जाएगी, लेकिन कैश लोड/सेव करने की प्रक्रिया धीमी हो जाएगी।.
settings_duplicates_prehash_checkbutton_tooltip = 
        यह सुविधा "प्रीहैश" (फ़ाइल के एक छोटे हिस्से से गणना किया गया हैश) को कैश करने की अनुमति देती है, जिससे डुप्लिकेट नहीं होने वाले परिणामों को जल्दी से हटाया जा सकता है।

        यह डिफ़ॉल्ट रूप से अक्षम है क्योंकि कुछ स्थितियों में यह प्रदर्शन को धीमा कर सकता है।

        यदि आप सैकड़ों हजारों या लाखों फ़ाइलों को स्कैन कर रहे हैं, तो इसका उपयोग करना अत्यधिक अनुशंसित है, क्योंकि यह खोज को कई गुना तेज कर सकता है।.
settings_duplicates_prehash_minimal_entry_tooltip = कैश किए गए प्रविष्टि का न्यूनतम आकार.
settings_duplicates_hide_hard_link_button = हार्ड लिंक छिपाएं
settings_duplicates_prehash_checkbutton = प्री-हैश कैश का उपयोग करें
settings_duplicates_minimal_size_cache_label = कैश में सहेजी जाने वाली फ़ाइलों का न्यूनतम आकार (बाइट में)।
settings_duplicates_minimal_size_cache_prehash_label = प्री-हैश कैश में सेव की जाने वाली फ़ाइलों का न्यूनतम आकार (बाइट्स में)

## Saving/Loading settings

settings_saving_button_tooltip = वर्तमान सेटिंग्स कॉन्फ़िगरेशन को फ़ाइल में सहेजें.
settings_loading_button_tooltip = फ़ाइल से सेटिंग्स लोड करें और वर्तमान कॉन्फ़िगरेशन को उनसे बदल दें.
settings_reset_button_tooltip = वर्तमान कॉन्फ़िगरेशन को डिफ़ॉल्ट पर रीसेट करें.
settings_saving_button = कॉन्फ़िगरेशन सहेजें
settings_loading_button = कॉन्फ़िगरेशन लोड करें
settings_reset_button = कॉन्फ़िगरेशन रीसेट करें

## Opening cache/config folders

settings_folder_cache_open_tooltip = 
        फ़ोल्डर खोलता है जहाँ कैशे txt फ़ाइलें संग्रहीत हैं।

        कैशे फ़ाइलों को बदलने से गलत परिणाम दिखाई दे सकते हैं। हालाँकि, पथ को बदलने से समय की बचत हो सकती है जब बड़ी संख्या में फ़ाइलों को किसी अन्य स्थान पर ले जाया जा रहा हो।

        आप इन फ़ाइलों को कंप्यूटरों के बीच कॉपी कर सकते हैं ताकि फ़ाइलों को फिर से स्कैन करने में लगने वाला समय बचाया जा सके (बेशक, अगर उनकी निर्देशिका संरचना समान है)।

        यदि कैशे में कोई समस्या है, तो इन फ़ाइलों को हटाया जा सकता है। ऐप स्वचालित रूप से उन्हें फिर से उत्पन्न कर देगा।.
settings_folder_settings_open_tooltip = 
        यह फ़ोल्डर खोलता है जहाँ Czkawka की कॉन्फ़िगरेशन फ़ाइल संग्रहीत है।

        चेतावनी: मैन्युअल रूप से कॉन्फ़िगरेशन को बदलने से आपकी कार्यप्रणाली बाधित हो सकती है।.
settings_folder_cache_open = कैश फ़ोल्डर खोलें
settings_folder_settings_open = सेटिंग्स फ़ोल्डर खोलें
# Compute results
compute_stopped_by_user = खोज उपयोगकर्ता द्वारा रोक दी गई।
compute_found_duplicates_hash_size = कुल { $number_files } डुप्लिकेट फ़ाइलें { $number_groups } समूहों में पाई गईं, जिनका आकार { $size } है और जिन्हें खोजने में { $time } लगा।
compute_found_duplicates_name = { $time } में, { $number_groups } समूहों में { $number_files } डुप्लिकेट फ़ाइलें पाई गईं।
compute_found_empty_folders = मिले { $number_files } खाली फ़ोल्डर, समय: { $time }
compute_found_empty_files = पाए गए { $number_files } खाली फ़ाइलें, समय: { $time }
compute_found_big_files = ढूंढें गए { $number_files } बड़ी फ़ाइलें, इसमें { $time } समय लगा
compute_found_temporary_files = यहाँ { $number_files } अस्थायी फ़ाइलें पाई गईं, जिन्हें { $time } में बनाया गया था।
compute_found_images = { $time } में, { $number_groups } समूहों में { $number_files } समान चित्र पाए गए।
compute_found_videos = { $time } में, { $number_groups } समूहों में { $number_files } समान वीडियो पाए गए
compute_found_music = { $time } में, { $number_groups } समूहों में { $number_files } समान संगीत फ़ाइलें मिलीं।
compute_found_invalid_symlinks = मिलीं { $number_files } अमान्य सिंबॉलिक लिंक, समय: { $time }
compute_found_broken_files = मिलीं { $number_files } भ्रष्ट फ़ाइलें, समय: { $time }
compute_found_bad_extensions = कुल { $number_files } फ़ाइलें मिलीं जिनमें अमान्य एक्सटेंशन हैं, और यह प्रक्रिया { $time } में पूरी हुई
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
progress_prehash_cache_loading = प्री-हैश कैश लोड हो रहा है
progress_prehash_cache_saving = प्रीहैश कैश को सहेज रहा है
progress_hash_cache_loading = हैश कैश लोड हो रहा है
progress_hash_cache_saving = हैश कैश को सुरक्षित किया जा रहा है
progress_cache_loading = कैश लोड हो रहा है
progress_cache_saving = कैश को सुरक्षित करना
progress_current_stage = Current Stage:{ "  " }
progress_all_stages = All Stages:{ "  " }
# Saving loading 
saving_loading_saving_success = कॉन्फ़िगरेशन को फ़ाइल { $name } में सहेजा गया.
saving_loading_saving_failure = फ़ाइल { $name } में कॉन्फ़िगरेशन डेटा सहेजने में विफल, कारण: { $reason }.
saving_loading_reset_configuration = वर्तमान कॉन्फ़िगरेशन साफ़ कर दिया गया है।.
saving_loading_loading_success = सही तरीके से ऐप कॉन्फ़िगरेशन लोड हुआ।.
saving_loading_failed_to_create_config_file = कॉन्फ़िगरेशन फ़ाइल "{ $path }" बनाने में विफल, कारण: "{ $reason }"।.
saving_loading_failed_to_read_config_file = "{ $path }" से कॉन्फ़िगरेशन लोड नहीं किया जा सका क्योंकि यह मौजूद नहीं है या यह कोई फ़ाइल नहीं है.
saving_loading_failed_to_read_data_from_file = फ़ाइल "{ $path }" से डेटा पढ़ने में असमर्थ, कारण: "{ $reason }"۔.
# Other
selected_all_reference_folders = खोज शुरू नहीं हो पा रही है, जब सभी डायरेक्टरी को रेफरेंस फ़ोल्डर के रूप में सेट किया गया हो
searching_for_data = डेटा खोज रहा हूँ, इसमें थोड़ा समय लग सकता है, कृपया प्रतीक्षा करें...
text_view_messages = संदेश
text_view_warnings = चेतावनीयां
text_view_errors = त्रुटियाँ
about_window_motto = यह प्रोग्राम उपयोग करने के लिए मुफ्त है और हमेशा रहेगा.
krokiet_new_app = Czkawka फिलहाल रखरखाव मोड में है, जिसका मतलब है कि केवल गंभीर बग ही ठीक किए जाएंगे और कोई नई सुविधाएँ नहीं जोड़ी जाएंगी। नई सुविधाओं के लिए, कृपया नया Krokiet ऐप देखें, जो अधिक स्थिर और बेहतर प्रदर्शन वाला है और अभी भी सक्रिय विकास के अधीन है।.
# Various dialog
dialogs_ask_next_time = अगली बार पूछना।
symlink_failed = Failed to symlink { $name } to { $target }, reason { $reason }
delete_title_dialog = हटाने की पुष्टि करें
delete_question_label = क्या आप वाकई में फ़ाइलों को डिलीट करना चाहते हैं?
delete_all_files_in_group_title = समूह में सभी फ़ाइलों को हटाने की पुष्टि
delete_all_files_in_group_label1 = कुछ समूहों में, सभी रिकॉर्ड चुने जाते हैं।.
delete_all_files_in_group_label2 = क्या आप निश्चित हैं कि आप उन्हें हटाना चाहते हैं?
delete_items_label = { $items } फ़ाइलें डिलीट की जाएंगी.
delete_items_groups_label = { $items } फ़ाइलें, { $groups } समूहों से हटाई जाएंगी.
hardlink_failed = हार्डलिंक बनाने में विफलता: "{ $name }" को "{ $target }" पर लिंक नहीं किया जा सका, कारण: "{ $reason }"
hard_sym_invalid_selection_title_dialog = अमान्य चयन, कुछ समूहों के साथ
hard_sym_invalid_selection_label_1 = कुछ समूहों में केवल एक रिकॉर्ड ही चुना जाता है, और उसे अनदेखा कर दिया जाएगा.
hard_sym_invalid_selection_label_2 = इन फ़ाइलों को हार्ड/सिम्बोलिक लिंक करने के लिए, समूह में कम से कम दो परिणाम चुने जाने चाहिए.
hard_sym_invalid_selection_label_3 = समूह में पहला आइटम मूल के रूप में पहचाना जाता है और उसमें कोई बदलाव नहीं किया जाता, लेकिन दूसरे और बाद के आइटमों में संशोधन किया जाता है।.
hard_sym_link_title_dialog = लिंक की पुष्टि।
hard_sym_link_label = क्या आप निश्चित हैं कि आप इन फ़ाइलों को लिंक करना चाहते हैं?
move_folder_failed = Failed to move folder { $name }, reason { $reason }
move_file_failed = Failed to move file { $name }, reason { $reason }
move_files_title_dialog = उस फ़ोल्डर का चयन करें जिसमें आप डुप्लिकेट फ़ाइलों को स्थानांतरित करना चाहते हैं
move_files_choose_more_than_1_path = Only one path may be selected to be able to copy their duplicated files, selected { $path_number }.
move_stats = Properly moved { $num_files }/{ $all_files } items
save_results_to_file = Saved results both to txt and json files into "{ $name }" folder.
search_not_choosing_any_music = त्रुटि: आपको संगीत खोज के कम से कम एक प्रकार के लिए बॉक्स का चयन करना होगा।.
search_not_choosing_any_broken_files = त्रुटि: आपको कम से कम एक ऐसा चेकबॉक्स चुनना होगा जिसमें टूटे हुए फ़ाइलों के प्रकार निर्दिष्ट हों.
include_folders_dialog_title = शामिल करने के लिए फ़ोल्डर
exclude_folders_dialog_title = उन फ़ोल्डरों को छोड़कर जिन्हें शामिल नहीं करना है
include_manually_directories_dialog_title = निर्देशिका को मैन्युअल रूप से जोड़ें
cache_properly_cleared = सही तरीके से कैशे साफ़ किया गया।
cache_clear_duplicates_title = डुप्लिकेट डेटा की अस्थायी फ़ाइल को साफ़ करना
cache_clear_similar_images_title = समान छवियों की कैश साफ़ करना
cache_clear_similar_videos_title = समान वीडियो के कैश को साफ़ करना
cache_clear_message_label_1 = क्या आप पुराने प्रविष्टियों की कैश साफ़ करना चाहते हैं?
cache_clear_message_label_2 = यह ऑपरेशन उन सभी कैश प्रविष्टियों को हटा देगा जो अब मौजूद नहीं वाली फ़ाइलों की ओर इशारा करती हैं.
cache_clear_message_label_3 = यह शायद कैश में डेटा लोड/सेव करने की प्रक्रिया को थोड़ा तेज कर सकता है।.
cache_clear_message_label_4 = चेतावनी: यह ऑपरेशन सभी कैश्ड डेटा को अनप्लग किए गए बाहरी ड्राइव से हटा देगा। इसलिए, प्रत्येक हैश को फिर से जेनरेट करने की आवश्यकता होगी।.
# Show preview
preview_image_resize_failure = Failed to resize image { $name }.
preview_image_opening_failure = Failed to open image { $name }, reason { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = ग्रुप: { $current_group }/{ $all_groups } ({ $images_in_group } तस्वीरें)
compare_move_left_button = L
compare_move_right_button = R
