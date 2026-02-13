# Window titles
window_settings_title = 설정
window_main_title = Czkawka (구글ить)
window_progress_title = 스캔중
window_compare_images = 이미지 비교
# General
general_ok_button = 확인
general_close_button = 닫기
# Krokiet info dialog
krokiet_info_title = Introducing Krokiet - 새로운 버전의 Czkawka
krokiet_info_message = 
        크로키트는 Czkawka GTK GUI의 새로운, 개선된, 더 빠르고 더 안정적인 버전입니다!

        실행하기가 더 쉽고 시스템 변경에 더 강하며, 대부분의 시스템에서 기본적으로 사용 가능한 핵심 라이브러리에만 의존합니다.

        크로키트는 Czkawka에 없는 기능도 제공하며, 비디오 비교 모드에서 미리보기, EXIF 클리너, 파일 이동/복사/삭제 진행률 또는 확장된 정렬 옵션 등이 포함됩니다.

        사용해 보고 차이점을 확인해 보세요!

        Czkawka는 저로부터 버그 수정 및 소규모 업데이트를 계속 받겠지만, 모든 새로운 기능은 크로키트에만 개발되며, 누구나 새로운 기능 추가, 누락된 모드 확장 또는 Czkawka 추가 확장을 자유롭게 기여할 수 있습니다.

        PS: 이 메시지는 한 번만 표시되어야 합니다. 다시 나타나면 CZKAWKA_DONT_ANNOY_ME 환경 변수를 비어 있는 값이 아닌 값으로 설정하십시오.
# Main window
music_title_checkbox = 제목
music_artist_checkbox = 아티스트
music_year_checkbox = 연도
music_bitrate_checkbox = 비트레이트
music_genre_checkbox = 장르
music_length_checkbox = 길이
music_comparison_checkbox = 근사값 비교
music_checking_by_tags = 태그 기준 검사
music_checking_by_content = 내용 기준 검사
same_music_seconds_label = 최소 조각 재생 시간
same_music_similarity_label = 최대 허용 차이
music_compare_only_in_title_group = 유사한 제목들의 그룹 내에서 비교
music_compare_only_in_title_group_tooltip =
    활성화 시, 파일이 제목별로 그룹화된 후에만 서로 비교됩니다.
    
    예: 10000개의 파일이 있을 경우, 거의 1억 번의 비교 대신 보통 약 20000번의 비교로 줄어듭니다.
same_music_tooltip =
    음악 파일 유사도 검색은 아래 설정으로 조정할 수 있습니다:
    
    - 유사도로 식별 가능한 최소 조각 시간
    - 비교할 조각간 허용 가능한 최대 차이 수치
    
    좋은 결과를 얻기 위해서는 이 두 값을 상황에 맞게 적절히 조합하는 것이 중요합니다.
    
    최소 시간 5초 + 최대 차이 1.0 설정 시 -> 거의 동일한 조각을 찾습니다.
    최소 시간 20초 + 최대 차이 6.0 설정 시 -> 리믹스/라이브 버전 등 유사한 경우에 효과적입니다.
    
    기본적으로 모든 음악 파일끼리 비교하게 되므로, 많은 파일을 비교할 때 시간이 오래 걸릴 수 있습니다.  
    따라서 일반적으로 **참조 폴더(reference folders)** 옵션을 사용하고 비교할 파일을 지정하면,  
    지문(fingerprint) 비교는 참조 없이 비교하는 것보다 **최소 4배 빠르게** 진행됩니다.
music_comparison_checkbox_tooltip =
    기계학습을 통해 각 항목의 괄호를 제거합니다. 예를 들어, 다음 두 파일은 같은 파일로 인식될 것입니다.
    
    Świędziżłób     ---     Świędziżłób (Remix Lato 2021)
duplicate_case_sensitive_name = 대소문자 구분
duplicate_case_sensitive_name_tooltip =
    대소문자 구분이 켜져 있으면, 완전히 같은 이름만이 중복 파일로 검색됩니다. 예시: Żołd <-> Żołd
    
    대소문자 구분이 꺼져 있으면, 대문자와 소문자 구별을 하지 않고 중복 파일을 검색합니다. 예시: żoŁD <-> Żołd
duplicate_mode_size_name_combo_box = 크기 및 이름 기준
duplicate_mode_name_combo_box = 파일명
duplicate_mode_size_combo_box = 파일 크기
duplicate_mode_hash_combo_box = 해시
duplicate_hash_type_tooltip =
    Czkawka는 3가지 유형의 해시 함수를 지원합니다.
    
    Blake3 - 암호화에 사용되는 해시입니다. 매우 빠르게 작동하므로, 기본값으로 설정되어 있습니다.
    
    CRC32 - 간단한 해시 함수입니다. Blake3보다는 빠르지만, 매우 드물게 충돌이 발생합니다.
    
    XXH3 - Black3와 해시 품질 및 성능 면에서 매우 유사하지만, 암호화에 쓰이지는 않습니다. 때문에 Black3와 실질적으로 같습니다.
duplicate_check_method_tooltip =
    현재 Czkawka는 중복 파일을 찾는데 3가지 방법을 지원합니다.
    
    파일명 - 같은 이름을 가진 파일들을 찾습니다.
    
    파일 크기 - 같은 크기를 가진 파일들을 찾습니다.
    
    해시 - 같은 내용을 가진 파일들을 찾습니다. 이 모드에서는 먼저 파일을 해시한 다음, 각 해시값들을 비교하여 중복 파일인지 식별합니다. 때문에 중복 파일을 찾는 데 있어 가장 확실한 방법입니다. Czkawka는 캐시에 매우 의존하므로, 같은 데이터를 두 번째 이후로 스캔하는 경우 첫 번째 스캔보다 더욱 빠르게 스캔이 이루어집니다.
image_hash_size_tooltip =
    각 확인된 이미지에 특별한 해시가 생성되어 서로 비교될 수 있으며,  
    작은 해시 차이는 이미지가 유사함을 의미합니다.
    
    해시 크기 8은 원본과 약간 유사한 이미지를 찾기에 적절합니다.  
    다만 이미지 수가 많을 경우(예: 1000개 이상), 거짓 양성(false positives)이 많이 발생할 수 있어  
    이 경우 더 큰 해시 크기 사용을 권장합니다.
    
    기본 해시 크기 16은 유사 이미지 검색과 해시 충돌 최소화를 적절히 균형 잡은 설정입니다.
    
    해시 크기 32 또는 64는 매우 유사한 이미지만 찾아내며, (알파 채널이 있는 일부 이미지 제외하면)  
    거의 거짓 양성이 없습니다.
image_resize_filter_tooltip =
    이미지 해시 계산 전에 라이브러리가 먼저 이미지를 리사이징해야 합니다.
    
    선택된 알고리즘에 따라 해시 계산에 사용하는 이미지의 형태가 약간 달라집니다.
    
    가장 빠른 알고리즘은 `Nearest`이며, 가장 낮은 화질을 제공하지만  
    기본 해시 크기 16x16일 경우 품질 저하가 눈에 잘 띄지 않습니다.
    
    이미지 수가 적고 해시 크기 8x8을 사용할 경우,  
    `Nearest`보다 다른 알고리즘을 사용하면 더 정확한 그룹핑에 도움이 됩니다.
image_hash_alg_tooltip =
    해시를 계산하는 데 사용되는 알고리즘을 선택할 수 있습니다.
    
    각각의 알고리즘은 장단점이 있으므로, 경우마다 더 낫거나 더 나쁜 결과를 보여줄 수 있습니다.
    
    따라서 가장 좋은 알고리즘을 찾으려면 수동으로 테스트해 보는 것이 좋습니다.
big_files_mode_combobox_tooltip = 가장 큰 파일 또는 가장 작은 파일을 찾을 수 있습니다
big_files_mode_label = 찾을 파일
big_files_mode_smallest_combo_box = 작은 파일
big_files_mode_biggest_combo_box = 큰 파일
main_notebook_duplicates = 중복 파일
main_notebook_empty_directories = 빈 디렉터리
main_notebook_big_files = 큰 파일
main_notebook_empty_files = 빈 파일
main_notebook_temporary = 임시 파일
main_notebook_similar_images = 비슷한 이미지
main_notebook_similar_videos = 비슷한 영상
main_notebook_same_music = 중복 음악
main_notebook_symlinks = 잘못된 심볼릭 링크
main_notebook_broken_files = 손상된 파일
main_notebook_bad_extensions = 잘못된 확장자
main_tree_view_column_file_name = 파일명
main_tree_view_column_folder_name = 폴더명
main_tree_view_column_path = 경로
main_tree_view_column_modification = 수정한 날짜
main_tree_view_column_size = 파일 크기
main_tree_view_column_similarity = 유사도
main_tree_view_column_dimensions = 크기
main_tree_view_column_title = 제목
main_tree_view_column_artist = 아티스트
main_tree_view_column_year = 연도
main_tree_view_column_bitrate = 비트레이트
main_tree_view_column_length = 길이
main_tree_view_column_genre = 장르
main_tree_view_column_symlink_file_name = 심볼릭 링크 파일명
main_tree_view_column_symlink_folder = 심볼릭 링크 폴더
main_tree_view_column_destination_path = 심볼릭 링크 대상 경로
main_tree_view_column_type_of_error = 손상 유형
main_tree_view_column_current_extension = 현재 확장자
main_tree_view_column_proper_extensions = 올바른 확장자
main_tree_view_column_fps = FPS
main_tree_view_column_codec = 코덱
main_label_check_method = 확인 방법
main_label_hash_type = 해시 유형
main_label_hash_size = 해시 크기
main_label_size_bytes = 파일 크기 (바이트)
main_label_min_size = 최소
main_label_max_size = 최대
main_label_shown_files = 찾을 파일의 개수
main_label_resize_algorithm = 크기 변경 알고리즘
main_label_similarity = 유사도{ "   " }
main_check_box_broken_files_audio = 음악 파일
main_check_box_broken_files_pdf = PDF
main_check_box_broken_files_archive = 압축 파일
main_check_box_broken_files_image = 이미지
main_check_box_broken_files_video = 비디오
main_check_box_broken_files_video_tooltip = ffmpeg/ffprobe를 사용하여 비디오 파일 유효성 검사합니다. 상당히 느리고 파일이 잘 재생되더라도 형식에 민감한 오류를 감지할 수 있습니다.
check_button_general_same_size = 같은 파일크기 무시
check_button_general_same_size_tooltip = 동일한 크기의 파일은 결과에서 제외합니다 – 대부분 1:1 중복일 가능성이 높습니다
main_label_size_bytes_tooltip = 스캔할 파일의 크기입니다
# Upper window
upper_tree_view_included_folder_column_title = 검색할 폴더
upper_tree_view_included_reference_column_title = 기준 폴더
upper_recursive_button = 재귀
upper_recursive_button_tooltip = 켜져 있으면, 하위 폴더 내부의 파일까지 검색합니다.
upper_manual_add_included_button = 수동 추가
upper_add_included_button = 추가
upper_remove_included_button = 제거
upper_manual_add_excluded_button = 수동 추가
upper_add_excluded_button = 추가
upper_remove_excluded_button = 제거
upper_manual_add_included_button_tooltip = 
    직접 검색할 경로를 입력합니다.
    
    여러 경로를 입력하고자 한다면, ';'로 구분하세요.
    
    '/home/roman;/home/rozkaz' 를 입력하면, '/home/roman'와 '/home/rozkaz'가 추가됩니다
upper_add_included_button_tooltip = 검색할 디렉터리를 추가합니다.
upper_remove_included_button_tooltip = 검색할 디렉터리에서 제거합니다.
upper_manual_add_excluded_button_tooltip = 
    직접 제외할 경로를 입력합니다.
    
    여러 경로를 입력하고자 한다면, ';'로 구분하세요.
    
    '/home/roman;/home/krokiet' 를 입력하면, '/home/roman'와 '/home/krokiet'가 추가됩니다
upper_add_excluded_button_tooltip = 제외할 디렉터리를 추가합니다.
upper_remove_excluded_button_tooltip = 제외할 디렉터리에서 제거합니다.
upper_notebook_items_configuration = 항목 설정
upper_notebook_excluded_directories = 제외 경로
upper_notebook_included_directories = 포함된 경로
upper_allowed_extensions_tooltip =
    허용할 확장자는 콤마(',')를 통해 구분해야 합니다. (기본값인 경우 모든 확장자를 허용합니다.)
    
    IMAGE, VIDEO, MUSIC, TEXT를 입력할 경우 해당하는 파일을 모두 지칭할 수 있습니다.
    
    예시: ".exe, IMAGE, VIDEO, .rar, 7z" - 이와 같이 입력하면, 이미지 파일(예. jpg, png), 영상 파일(예. avi, mp4), exe, rar, 그리고 7z 파일을 검색합니다.
upper_excluded_extensions_tooltip =
    검사에서 무시될 비활성화된 파일 목록입니다.
    
    허용된 확장자와 비활성화된 확장자를 둘 다 사용할 경우, 비활성화된 확장자가 더 높은 우선순위를 가지므로 해당 파일은 검사되지 않습니다.
upper_excluded_items_tooltip = 
        제외 항목은 * 와일드카드와 쉼표로 구분되어야 합니다.
        이는 Excluded Paths 보다 느리므로 주의해서 사용하십시오.
upper_excluded_items = 제외할 항목:
upper_allowed_extensions = 허용할 확장자:
upper_excluded_extensions = 비활성 확장자:
# Popovers
popover_select_all = 모두 선택
popover_unselect_all = 모두 선택 해제
popover_reverse = 선택 반전
popover_select_all_except_shortest_path = 선택 모두 제외 짧은 경로
popover_select_all_except_longest_path = 선택 전부 제외 가장 긴 경로
popover_select_all_except_oldest = 가장 오래된 파일 제외하고 모두 선택
popover_select_all_except_newest = 가장 최신인 파일 제외하고 모두 선택
popover_select_one_oldest = 가장 오래된 파일 선택
popover_select_one_newest = 가장 최신인 파일 선택
popover_select_custom = 사용자 지정 선택
popover_unselect_custom = 사용자 지정 선택 해제
popover_select_all_images_except_biggest = 가장 큰 파일 제외하고 모두 선택
popover_select_all_images_except_smallest = 가장 작은 파일 제외하고 모두 선택
popover_custom_path_check_button_entry_tooltip = 
    경로를 기준으로 선택합니다.
    
    사용 예시:
    '/home/pimpek/rzecz.txt' 파일을 선택하려면 '/home/pim*'와 같이 입력하세요
popover_custom_name_check_button_entry_tooltip = 
    파일 이름을 기준으로 선택합니다.
    
    사용 예시:
    '/usr/ping/pong.txt' 파일을 선택하려면 '*ong*'와 같이 입력하세요
popover_custom_regex_check_button_entry_tooltip =
    정규표현식을 이용해 선택합니다.
    
    이 모드에서는 경로와 이름 모두가 정규표현식에 의해 검색됩니다.
    
    사용 예시:
    '/usr/bin/ziemniak.txt' 파일을 선택하려면 '/ziem[a-z]+'와 같이 입력하세요.
    
    정규 표현식은 Rust 언어에 내장된 구현체를 사용합니다. 더 알고 싶다면 https://docs.rs/regex를 방문하세요.
popover_custom_case_sensitive_check_button_tooltip =
    대소문자를 구분할 지 여부를 선택합니다.
    
    만일 꺼져 있으면, '/home/*'은 '/HoMe/roman'과 '/home/roman'를 모두 선택합니다.
popover_custom_not_all_check_button_tooltip = 
    한 그룹에 있는 모든 항목이 선택되는 것을 방지합니다.
    
    이 옵션은 기본적으로 켜져 있습니다. 대부분의 경우, 원본과 중복 파일을 전부 선택하여 삭제하는 것은 원하지 않는 동작일 것입니다. 즉 각 그룹에서 최소한 하나의 항목은 삭제하지 않고 남겨놓게 됩니다.
    
    경고! 이 설정은 수동으로 그룹의 모든 파일을 이미 선택해 놓았다면 작동하지 않습니다!.
popover_custom_regex_path_label = 경로
popover_custom_regex_name_label = 파일명
popover_custom_regex_regex_label = 경로 및 파일 정규표현식
popover_custom_case_sensitive_check_button = 대소문자 구별
popover_custom_all_in_group_label = 그룹의 모든 항목을 선택하지 않음
popover_custom_mode_unselect = 사용자 지정 선택 해제
popover_custom_mode_select = 사용자 지정 선택
popover_sort_file_name = 파일 이름
popover_sort_folder_name = 폴더 이름
popover_sort_full_name = 본인 이름
popover_sort_size = 파일 크기
popover_sort_selection = 선택
popover_invalid_regex = 정규표현식이 유효하지 않습니다
popover_valid_regex = 정규표현식이 유효합니다
# Bottom buttons
bottom_search_button = 검색
bottom_select_button = 선택
bottom_delete_button = 삭제
bottom_save_button = 저장
bottom_symlink_button = 심볼릭 링크
bottom_hardlink_button = 하드 링크
bottom_move_button = 이동
bottom_sort_button = 종류
bottom_compare_button = 비교
bottom_search_button_tooltip = 검색을 시작합니다
bottom_select_button_tooltip = 항목을 선택합니다. 오직 선택된 것만이 처리됩니다.
bottom_delete_button_tooltip = 선택된 파일 또는 폴더를 삭제합니다.
bottom_save_button_tooltip = 검색 결과를 파일로 저장합니다
bottom_symlink_button_tooltip =
    심볼릭 링크를 생성합니다.
    그룹 내에서 최소한 2개의 파일이 선택되어 있어야 합니다.
    첫 번째 파일은 그대로 남으며, 두 번째 이후 파일은 첫 번째 파일로 향하는 심볼릭 링크가 됩니다.
bottom_hardlink_button_tooltip =
    하드 링크를 생성합니다.
    그룹 내에서 최소한 2개의 파일이 선택되어 있어야 합니다.
    첫 번째 파일은 그대로 남으며, 두 번째 이후 파일은 첫 번째 파일로 향하는 하드 링크가 됩니다.
bottom_hardlink_button_not_available_tooltip =
    하드 링크를 생성합니다.
    현재 하드 링크를 만들 수 없어 버튼이 비활성화되었습니다.
    Windows에서 하드 링크는 관리자 권한으로만 만들 수 있습니다. 프로그램이 관리자 권한으로 실행되었는지 확인하세요.
    만일 프로그램이 이미 관리자 권한으로 실행되었다면, Github에서 비슷한 이슈가 있는지 확인해보세요.
bottom_move_button_tooltip =
    선택된 디렉터리로 파일을 이동합니다.
    이 동작은 원본이 위치한 경로를 전부 무시하고, 선택한 경로로 파일을 전부 복사합니다.
    만일 2개 이상의 파일이 같은 이름을 가지고 있다면, 첫 번째 이후의 파일은 복사에 실패하고 오류 메시지를 보여줄 것입니다.
bottom_sort_button_tooltip = 파일/폴더를 선택한 방법으로 정렬합니다.
bottom_compare_button_tooltip = 그룹 내의 이미지를 비교합니다.
bottom_show_errors_tooltip = 하단 텍스트 패널을 보이거나 숨깁니다.
bottom_show_upper_notebook_tooltip = 상단 패널을 보이거나 숨깁니다.
# Progress Window
progress_stop_button = 정지
progress_stop_additional_message = 정지 요청됨
# About Window
about_repository_button_tooltip = 소스 코드가 있는 리포지토리 페이지 링크입니다.
about_donation_button_tooltip = 기부 페이지 링크입니다.
about_instruction_button_tooltip = 사용방법 페이지 링크입니다.
about_translation_button_tooltip = 번역을 위한 Crowdin 페이지 링크입니다. 공식적으로 지원되는 언어는 폴란드어와 영어입니다.
about_repository_button = 리포지토리
about_donation_button = 기부
about_instruction_button = 사용방법
about_translation_button = 번역
# Header
header_setting_button_tooltip = 설정창을 엽니다.
header_about_button_tooltip = 이 앱에 대한 정보창을 엽니다.

# Settings


## General

settings_number_of_threads = 스레드 수
settings_number_of_threads_tooltip = 사용할 스레드 수입니다. 0이면 가능한 최대 스레드를 사용합니다.
settings_use_rust_preview = 미리보기에 GTK 대신 외부 라이브러리 사용
settings_use_rust_preview_tooltip =
    GTK 미리보기는 일부 경우 더 빠르거나 더 많은 형식을 지원하지만,  
    반대로 성능이 더 떨어질 수도 있습니다.
    
    미리보기 로딩에 문제가 있다면 이 설정을 변경해 보세요.
    
    리눅스가 아닌 환경에서는 `gtk-pixbuf`가 항상 사용 가능하지 않기 때문에  
    이 옵션을 끄면 일부 이미지 미리보기가 로드되지 않을 수 있습니다.
settings_label_restart = 이 설정을 적용하려면 프로그램을 재시작해야 합니다!
settings_ignore_other_filesystems = 다른 파일시스템 무시(Linux에서만)
settings_ignore_other_filesystems_tooltip = 
    검색할 디렉터리와 파일시스템이 다른 디렉터리를 무시합니다.
    
    Linux의 find 명령에서 -xdev 옵션을 준 것과 동일하게 동작합니다
settings_save_at_exit_button_tooltip = 프로그램 종료 시에 설정을 저장합니다.
settings_load_at_start_button_tooltip =
    프로그램을 열 때 저장된 설정을 불러옵니다.
    
    꺼져 있다면, 기본 설정으로 프로그램을 시작합니다.
settings_confirm_deletion_button_tooltip = 삭제 버튼을 누를 때 확인창을 띄웁니다.
settings_confirm_link_button_tooltip = 하드 링크/심볼릭 링크 버튼을 누를 때 확인창을 띄웁니다.
settings_confirm_group_deletion_button_tooltip = 그룹의 모든 항목을 삭제할 경우 경고창을 보여줍니다.
settings_show_text_view_button_tooltip = UI 하단에 텍스트 패널을 보여줍니다.
settings_use_cache_button_tooltip = 파일 캐시를 사용합니다.
settings_save_also_as_json_button_tooltip = 캐시를 (사람이 읽을 수 있는) JSON 포맷으로 저장합니다. 캐시 내용을 수정할 수 있습니다. 만일 bin 확장자를 가진 바이너리 캐시 파일이 없으면, JSON 캐시가 프로그램 시작 시에 대신 로드됩니다.
settings_use_trash_button_tooltip = 파일을 영구 삭제하는 대신 휴지통으로 이동합니다.
settings_language_label_tooltip = UI에 표시될 언어를 설정합니다.
settings_save_at_exit_button = 프로그램을 닫을 때 설정을 저장
settings_load_at_start_button = 프로그램을 열 때 설정을 불러오기
settings_confirm_deletion_button = 항목 삭제 시에 확인창 띄우기
settings_confirm_link_button = 항목 심볼릭 링크/하드 링크 설정시에 확인창 띄우기
settings_confirm_group_deletion_button = 그룹 내의 모든 항목 삭제 시 경고창 띄우기
settings_show_text_view_button = 하단 텍스트 패널 표시하기
settings_use_cache_button = 캐시 사용
settings_save_also_as_json_button = 캐시를 JSON 포맷으로도 저장
settings_use_trash_button = 삭제된 파일을 휴지통으로 이동
settings_language_label = 언어
settings_multiple_delete_outdated_cache_checkbutton = 만료된 파일을 캐시에서 자동으로 삭제
settings_multiple_delete_outdated_cache_checkbutton_tooltip =
    더 이상 존재하지 않는 파일에 대한 정보를 캐시에서 삭제합니다.
    
    이 옵션이 켜져 있으면, 프로그램은 존재하는 파일만이 캐시에 남도록 할 것입니다(망가진 파일은 무시됩니다).
    
    이 옵션을 끄는 것은 외장 저장장치에 존재하는 파일을 스캔했을 때, 외장 저장장치에 있는 파일에 대한 캐시를 보존하는 데 도움이 됩니다.
    
    만일 수백~수천 개의 파일에 해당하는 정보가 캐시에 있다면 이 옵션을 켜는 것을 추천합니다. 이 경우 캐시를 저장하거나 불러오는 시간이 빨라집니다.
settings_notebook_general = 일반
settings_notebook_duplicates = 중복 파일
settings_notebook_images = 유사한 이미지
settings_notebook_videos = 유사한 영상

## Multiple - settings used in multiple tabs

settings_multiple_image_preview_checkbutton_tooltip = 이미지 파일을 선택하면 우측에 미리보기를 보여줍니다.
settings_multiple_image_preview_checkbutton = 이미지 미리보기 표시
settings_multiple_clear_cache_button_tooltip =
    더 이상 존재하지 않는 파일을 캐시에서 제거합니다.
    캐시를 자동으로 정리하는 옵션이 꺼져 있을 때만 사용하세요.
settings_multiple_clear_cache_button = 캐시에서 오래된 결과 제거.
 
## Duplicates

settings_duplicates_hide_hard_link_button_tooltip =
    하나의 파일에 대한 여러 개의 하드 링크가 존재할 경우, 그 중 하나만을 표시합니다.
    
    예: 만일 특정한 파일에 대한 7개의 하드 링크가 디스크에 존재하고, 그 중 하나가 다른 inode를 갖는다면, 결과창에는 1개의 파일과 1개의 하드 링크만이 표시됩니다.
settings_duplicates_minimal_size_entry_tooltip =
    캐시에 추가되기 위한 최소 파일 사이즈를 설정합니다.
    
    이 값이 작을 수록 더 많은 파일이 캐시에 저장됩니다. 이 경우 검색은 더 빨라지지만, 캐시 저장 및 불러오기는 느려집니다.
settings_duplicates_prehash_checkbutton_tooltip =
    사전 해시(파일 일부만으로 계산되는 해시)에 대한 캐싱을 허용하여, 중복이 아닌 파일을 더 빠르게 결과에서 제거합니다.
    
    이 옵션은 일부 상황에서 검색을 느리게 하기 때문에, 기본적으로 꺼져 있습니다.
    
    만일 수백~수천 개 이상의 파일처럼 매우 많은 파일을 여러 번 검색하는 경우, 이 기능을 반드시 켜는 것을 추천합니다.
settings_duplicates_prehash_minimal_entry_tooltip = 캐싱을 위한 최소 파일크기입니다.
settings_duplicates_hide_hard_link_button = 하드 링크 숨기기
settings_duplicates_prehash_checkbutton = 사전 해시 캐싱하기
settings_duplicates_minimal_size_cache_label = 캐싱하기 위한 최소 파일 크기 (바이트)
settings_duplicates_minimal_size_cache_prehash_label = 사전 해시를 캐싱하기 위한 최소 파일 크기 (바이트)

## Saving/Loading settings

settings_saving_button_tooltip = 현재 설정을 파일에 저장합니다.
settings_loading_button_tooltip = 저장된 설정을 불러와 현재 설정을 덮어씁니다.
settings_reset_button_tooltip = 설정을 기본값으로 되돌립니다.
settings_saving_button = 설정 저장
settings_loading_button = 설정 불러오기
settings_reset_button = 설정 초기화

## Opening cache/config folders

settings_folder_cache_open_tooltip =
    캐시 파일이 저장되는 폴더를 엽니다.
    
    캐시 파일을 편집하는 경우 유효하지 않은 결과가 표시될 수 있습니다. 다만, 많은 양의 파일이 다른 곳으로 이동되었다면 캐시 내의 경로를 수정하는 것이 도움이 됩니다.
    
    만일 비슷한 디렉터리 구조를 가지는 경우, 캐시 파일을 복사하여 다른 컴퓨터에서도 같은 캐시를 재활용할 수 있습니다.
    
    만일 캐시에 문제가 발생한다면 이 폴더의 파일들을 지우십시오. 그렇게 하면 프로그램이 다시 캐시 파일을 생성합니다.
settings_folder_settings_open_tooltip =
    Czkawka의 설정 파일이 있는 폴더를 엽니다.
    
    경고! 설정 파일을 수동으로 편집하는 경우 원치 않는 동작이 일어날 수 있습니다.
settings_folder_cache_open = 캐시 폴더 열기
settings_folder_settings_open = 설정 폴더 열기
# Compute results
compute_stopped_by_user = 사용자에 의해 검색이 중단됨
compute_found_duplicates_hash_size = { $number_files }개의 중복 파일을 { $number_groups }개 그룹에서 발견했으며 이는 { $size }를 차지하고 { $time }에 걸쳐 수행되었습니다
compute_found_duplicates_name = { $number_files } 개의 중복 파일을 { $number_groups } 그룹에서 { $time }에 발견했습니다
compute_found_empty_folders = { $number_files }개의 비어있는 폴더를 { $time } 안에 발견했습니다
compute_found_empty_files = { $number_files }개의 빈 파일을 { $time }에 발견했습니다
compute_found_big_files = { $number_files }개의 큰 파일을 { $time }에 찾았습니다
compute_found_temporary_files = { $number_files }개의 임시 파일을 { $time } 안에 찾았습니다
compute_found_images = { $number_files } 개의 유사한 이미지를 { $number_groups } 그룹에서 { $time } 내에 발견했습니다
compute_found_videos = { $number_files } 개의 비슷한 동영상을 { $number_groups } 그룹에서 { $time } 내에 찾았습니다
compute_found_music = { $number_files }개의 비슷한 음악 파일을 { $number_groups } 그룹에서 { $time } 내에 찾았습니다
compute_found_invalid_symlinks = { $number_files }개의 유효하지 않은 심볼릭 링크를 { $time }에서 찾았습니다
compute_found_broken_files = { $number_files }개의 봉인된 파일을 { $time }에 발견했습니다
compute_found_bad_extensions = { $number_files } 확장자에 문제가 있는 파일을 { $time } 내로找到了
# Progress window
progress_scanning_general_file =
    { $file_number ->
        [one] { $file_number }개 파일 스캔 완료
       *[other] { $file_number }개 파일 스캔 완료
    }
progress_scanning_extension_of_files = { $file_checked }/{ $all_files }개의 파일 확장자 확인
progress_scanning_broken_files = { $file_checked }/{ $all_files }개 파일 확인 (데이터: { $data_checked } / { $all_data })
progress_scanning_video = { $file_checked }/{ $all_files }개의 비디오 해시 생성
progress_creating_video_thumbnails = Created thumbnails of { $file_checked }/{ $all_files } video
progress_scanning_image = { $file_checked }/{ $all_files }개의 이미지 해시 생성 (데이터: { $data_checked } / { $all_data })
progress_comparing_image_hashes = { $file_checked }/{ $all_files }개의 이미지 해시 비교
progress_scanning_music_tags_end = { $file_checked }/{ $all_files }개의 음악 파일 태그 비교 완료
progress_scanning_music_tags = { $file_checked }/{ $all_files }개의 음악 파일 태그 읽는 중
progress_scanning_music_content_end = { $file_checked }/{ $all_files }개의 음악 파일 지문 비교 완료
progress_scanning_music_content = { $file_checked }/{ $all_files }개의 음악 파일 지문 계산 중 (데이터: { $data_checked } / { $all_data })
progress_scanning_empty_folders =
    { $folder_number ->
        [one] { $folder_number }개 폴더 스캔 완료
       *[other] { $folder_number }개 폴더 스캔 완료
    }
progress_scanning_size = { $file_number }개의 파일 크기 스캔 완료
progress_scanning_size_name = { $file_number }개의 파일 이름 및 크기 스캔 완료
progress_scanning_name = { $file_number }개의 파일 이름 스캔 완료
progress_analyzed_partial_hash = { $file_checked }/{ $all_files }개 파일 부분 해시 분석 완료 (데이터: { $data_checked } / { $all_data })
progress_analyzed_full_hash = { $file_checked }/{ $all_files }개 파일 전체 해시 분석 완료 (데이터: { $data_checked } / { $all_data })
progress_prehash_cache_loading = PreHash 캐시 로드 중
progress_prehash_cache_saving = PreHash 캐시 저장 중
progress_hash_cache_loading = 해시 캐시 로드 중
progress_hash_cache_saving = 해시 캐시 저장 중
progress_cache_loading = 캐시 로드 중
progress_cache_saving = 캐시 저장 중
progress_current_stage = 현재 단계:{ "  " }
progress_all_stages = 전체 단계:{ "  " }
# Saving loading 
saving_loading_saving_success = 파일 { $name }에 설정 저장함.
saving_loading_saving_failure = кон피그레이션 데이터를 파일 { $name }에 저장하지 못했습니다, 이유는 { $reason }입니다.
saving_loading_reset_configuration = 현재 설정이 초기화됨.
saving_loading_loading_success = 앱 설정 불러오기 성공.
saving_loading_failed_to_create_config_file = "{ $path }" 파일에 설정을 저장할 수 없습니다. 이유: "{ $reason }".
saving_loading_failed_to_read_config_file = "{ $path }" 파일에서 설정을 불러올 수 없습니다. 파일이 없거나, 파일이 아닙니다.
saving_loading_failed_to_read_data_from_file = "{ $path }" 파일을 읽을 수 없습니다. 이유: "{ $reason }".
# Other
selected_all_reference_folders = 모든 디렉터리가 기준 폴더이므로, 검색을 시작할 수 없습니다
searching_for_data = 검색 중. 잠시만 기다려주세요...
text_view_messages = 알림
text_view_warnings = 경고
text_view_errors = 오류
about_window_motto = 이 프로그램은 무료이며, 앞으로도 항상 그럴 것이다.
krokiet_new_app = 'Czkawka'는 유지보수 모드에 있습니다,这意味着仅会修复关键错误且不会添加新功能。对于新功能，请查看新的Krokiet应用，该应用更加稳定性能更强并且仍在积极开发中。.
# Various dialog
dialogs_ask_next_time = 다음에도 묻기
symlink_failed = Failed to symlink { $name } to { $target }, reason { $reason }
delete_title_dialog = 삭제 확인
delete_question_label = 정말로 파일들을 삭제합니까?
delete_all_files_in_group_title = 그룹의 모든 파일 삭제 확인
delete_all_files_in_group_label1 = 일부 그룹 내에 있는 모든 파일이 선택되어 있습니다.
delete_all_files_in_group_label2 = 정말로 해당 파일을 모두 삭제합니까?
delete_items_label = { $items }개의 파일이 삭제됩니다.
delete_items_groups_label = { $groups }개 그룹에서 { $items }개의 파일이 삭제됩니다.
hardlink_failed = 하드 링크를 생성하지 못했습니다 { $name }을 { $target }으로, 이유는 { $reason }입니다
hard_sym_invalid_selection_title_dialog = 일부 그룹의 선택이 유효하지 않습니다
hard_sym_invalid_selection_label_1 = 일부 그룹에서 1개의 항목만이 선택되었으며, 해당 항목은 무시됩니다.
hard_sym_invalid_selection_label_2 = 하드 링크/심볼릭 링크를 생성하려면, 그룹 내에서 최소 2개의 파일이 선택되어야 합니다.
hard_sym_invalid_selection_label_3 = 그룹 내의 첫 번째가 원본으로 설정되며, 나머지는 수정될 것입니다.
hard_sym_link_title_dialog = 링크 생성 확인
hard_sym_link_label = 정말로 해당 파일들을 링크합니까?
move_folder_failed = { $name } 폴더 이동 실패. 이유: { $reason }
move_file_failed = { $name } 파일 이동 실패. 이유: { $reason }
move_files_title_dialog = 중복 파일을 이동할 폴더를 선택하세요
move_files_choose_more_than_1_path = 중복 파일을 복사할 1개의 폴더만 지정해야 하지만, { $path_number }개의 경로를 선택했습니다.
move_stats = { $num_files }/{ $all_files }개의 항목을 이동함
save_results_to_file = 결과를 txt 및 json 파일로 ‘{ $name }’ 폴더에 저장했습니다.
search_not_choosing_any_music = 경고: 최소한 하나의 검색 방법을 선택해야 합니다.
search_not_choosing_any_broken_files = 경고: 최소한 하나 이상의 검색할 파일 분류를 선택해야 합니다.
include_folders_dialog_title = 검색할 폴더 추가
exclude_folders_dialog_title = 제외할 폴더 추가
include_manually_directories_dialog_title = 수동으로 디렉터리 추가
cache_properly_cleared = 캐시를 성공적으로 정리했습니다
cache_clear_duplicates_title = 중복 파일 캐시 정리
cache_clear_similar_images_title = 유사한 이미지 캐시 정리
cache_clear_similar_videos_title = 유사한 영상 캐시 정리
cache_clear_message_label_1 = 유효하지 않은 캐시 항목을 제거할까요?
cache_clear_message_label_2 = 이 동작은 더 이상 유효하지 않은 파일에 대한 캐시 항목을 제거합니다.
cache_clear_message_label_3 = 이를 통해 더 빠른 캐시 저장/불러오기가 가능할 수 있습니다.
cache_clear_message_label_4 = 경고! 이 동작은 연결되지 않은 외장 저장장치에 위치한 모든 항목을 제거합니다. 따라서 해당 파일들에 대한 캐시는 다시 생성되어야 합니다.
# Show preview
preview_image_resize_failure = { $name } 이미지 크기 조정 실패.
preview_image_opening_failure = { $name } 이미지 열기 실패. 이유: { $reason }
# Compare images (L is short Left, R is short Right - they can't take too much space)
compare_groups_number = 그룹 { $current_group } / { $all_groups } ({ $images_in_group } 이미지)
compare_move_left_button = 이전
compare_move_right_button = 다음
