# Cedinia - English (fallback)

# App / top bar titles
app_name = 세디니아
tool_duplicate_files = 중복 파일
tool_empty_folders = 비어있는 폴더들
tool_similar_images = 유사한 이미지
tool_empty_files = 빈 파일
tool_temporary_files = 임시 파일
tool_big_files = 가장 큰 파일들
tool_broken_files = 손상된 파일
tool_bad_extensions = 잘못된 확장자
tool_same_music = 중복 음악
tool_bad_names = 나쁜 이름
tool_exif_remover = EXIF 데이터
tool_directories = 디렉터리 (또는 폴더)
tool_settings = 설정
# Home screen tool card descriptions
home_dup_description = 동일한 내용을 가진 파일을 찾습니다
home_empty_folders_description = 내용이 없는 폴더
home_similar_images_description = 시각적으로 유사한 사진을 찾아보세요
home_empty_files_description = 크기가 0인 파일들
home_temp_files_description = 임시 파일 및 캐시 파일
home_big_files_description = 디스크에서 가장 큰/가장 작은 파일들
home_broken_files_description = PDF, 오디오, 이미지, 아카이브 파일
home_bad_extensions_description = 유효하지 않은 확장자를 가진 파일들
home_same_music_description = 태그를 기준으로 한 유사 오디오 파일
home_bad_names_description = 이름에 문제가 있는 문자가 포함된 파일
home_exif_description = EXIF 메타데이터가 포함된 이미지
# Results list
scanning = 스캔 진행 중...
stopping = 정지 중...
no_results = 결과가 없습니다
press_start = START 버튼을 눌러 스캔을 시작합니다
select_label = 선택.
deselect_label = Desel.
list_label = 목록
gallery_label = 갈.
# Selection popup
selection_popup_title = 선택
select_all = 전체 선택
select_except_one = 다음 중 하나를 제외하고 모두 선택하세요
select_except_largest = 가장 큰 것 외에는 모두 선택
select_except_smallest = 가장 작은 파일 제외하고 모두 선택
select_largest = 가장 큰 것을 선택하세요
select_smallest = 가장 작은 것 선택
select_except_highest_res = 가장 높은 해상도를 제외하고 모두 선택
select_except_lowest_res = 가장 낮은 해상도를 제외하고 모두 선택합니다
select_highest_res = 가장 높은 해상도를 선택하세요
select_lowest_res = 가장 낮은 해상도 선택
invert_selection = 선택 반전
close = 닫기
# Deselection popup
deselection_popup_title = 선택 해제
deselect_all = 전체 선택 해제
deselect_except_one = 모두 선택 해제하고, 단 하나만 선택하세요
# Confirm popup
cancel = 취소
delete = 삭제
rename = 이름 바꾸기
# Delete errors popup
delete_errors_title = 일부 파일을 삭제하는 데 실패했습니다:
ok = 알겠습니다
# Stopping overlay
stopping_overlay_title = 정지
stopping_overlay_body = 
        현재 스캔이 완료됩니다...
        잠시만 기다려 주십시오.
# Permission popup
permission_title = 파일 접근 권한
permission_body = 파일을 스캔하려면 앱이 기기 저장 공간에 접근해야 합니다. 이 권한이 없으면 스캔이 불가능합니다.
grant = 수혜
no_permission_scan_warning = 파일 접근 권한 없음 - 스캔 권한을 부여하세요
# Settings screen tabs
settings_tab_general = 일반
settings_tab_tools = 도구들
settings_tab_diagnostics = 정보
# Settings - General tab
settings_use_cache = 캐시 사용
settings_use_cache_desc = 이후 스캔(해시/이미지) 속도를 향상시킵니다
settings_ignore_hidden = 숨김 파일은 무시합니다
settings_ignore_hidden_desc = '.'로 시작하는 파일 및 폴더
settings_show_notification = 스캔이 완료되면 알려주세요
settings_show_notification_desc = 스캔 완료 시 시스템 알림을 표시합니다
settings_notify_only_background = 배경으로 전환되었을 때만
settings_notify_only_background_desc = 앱이 화면에 보이는 경우 알림을 표시하지 않습니다
notifications_disabled_banner = 알림 기능이 비활성화되었습니다
notifications_enable_button = 활성화
settings_scan_label = 스캔
settings_filters_label = 필터 (일부 도구)
settings_min_file_size = 최소 파일 크기
settings_max_file_size = 최대 파일 크기
settings_language = 언어
settings_language_restart = 앱을 다시 시작해야 합니다
settings_common_label = 일반 설정
settings_excluded_items = 제외 대상 항목 (글로브 패턴, 쉼표로 구분)
settings_excluded_items_placeholder = 예: *.tmp, */.git/*, */node_modules/*
settings_allowed_extensions = 허용되는 확장자 (비어 있음 = 모두 허용)
settings_allowed_extensions_placeholder = 예: jpg, png, mp4
settings_excluded_extensions = 제외 대상 확장 프로그램
settings_excluded_extensions_placeholder = 예: bak, tmp, log
# Settings - Tools section labels
settings_duplicates_header = 중복 항목
settings_check_method_label = 비교 방법
settings_check_method = 방식
settings_hash_type_label = 해시 유형
settings_hash_type = 해시 유형
settings_hash_type_desc = Blake3 - 권장되는 옵션이며, CRC32는 오탐 가능성이 낮습니다
settings_similar_images_header = 유사 이미지
settings_similarity_preset = 유사성 임계값
settings_similarity_desc = 매우 높음 = 거의 동일한 것만
settings_hash_size = 해시 크기
settings_hash_size_desc = 더 큰 크기의 모델은 오탐이 적은 반면, 유사한 이미지를 찾는 능력도 떨어집니다
settings_hash_alg = 해시 알고리즘
settings_image_filter = 크기 조절 필터
settings_ignore_same_size = 같은 크기의 이미지는 무시합니다
settings_gallery_image_fit_cover = 갤러리: 정사각형으로 자르기
settings_gallery_image_fit_cover_desc = 타일을 채우기; 원본 비율을 유지하려면 비활성화하세요
settings_big_files_header = 가장 큰 파일들
settings_search_mode = 검색 모드
settings_file_count = 파일 개수
settings_same_music_header = 음악 복제본
settings_music_check_method = 비교 모드
settings_music_compare_tags_label = 비교 태그
settings_music_title = 제목
settings_music_artist = 아티스트
settings_music_year = 연도
settings_music_length = 길이
settings_music_genre = 장르
settings_music_bitrate = 비트레이트
settings_music_approx = 대략적인 태그 비교
settings_broken_files_header = 손상된 파일들
settings_broken_files_note = 많은 시스템 자원을 사용하는 스캔 작업입니다. 최상의 성능을 위해 데스크톱 환경에서 Krokiet을 사용하십시오.
settings_broken_files_types_label = 확인된 유형 목록
settings_broken_audio = 오디오 파일
settings_broken_pdf = PDF
settings_broken_archive = 아카이브 파일
settings_broken_image = 이미지 파일
settings_bad_names_header = 나쁜 이름들
settings_bad_names_checks_label = 수표
settings_bad_names_uppercase_ext = 대문자 확장
settings_bad_names_emoji = 이모지 이름에
settings_bad_names_space = 시작/끝에 있는 공백
settings_bad_names_non_ascii = ASCII 문자가 아닌 문자들
settings_bad_names_duplicated = 반복되는 문자
# Settings - Diagnostics tab
diagnostics_header = 진단
diagnostics_thumbnails = 미리보기 이미지 캐시
diagnostics_app_cache = 앱 캐시
diagnostics_refresh = 새로 고침
diagnostics_clear_thumbnails = 선명한 썸네일
diagnostics_open_thumbnails_folder = 폴더 열기
diagnostics_clear_cache = 캐시 삭제
diagnostics_open_cache_folder = 폴더 열기
diagnostics_collect_test = 파일 접근 테스트
diagnostics_collect_test_desc = 접근 가능한 파일의 개수를 확인하세요
diagnostics_collect_test_run = 실행
diagnostics_collect_test_stop = 정지
collect_test_cancelled = 사용자 의해 중단됨
diag_confirm_clear_thumbnails = 섬네일 캐시를 모두 삭제하시겠습니까?
diag_confirm_clear_cache = 앱 캐시를 모두 삭제하시겠습니까?
about_repo = 리포지토리
about_translate = 번역
about_donate = 지원
# Collect-test result popup
collect_test_title = 테스트 결과
collect_test_volumes = 권수:
collect_test_folders = 폴더:
collect_test_files = 파일:
collect_test_time = 시간:
# Licenses
licenses_label = 라이선스
third_party_licenses = 제3자 라이선스
licenses_popup_title = 제3자 라이선스
# Directories screen
directories_include_header = 포함하세요
directories_included = 포함됨
directories_exclude_header = 제외
directories_excluded_header = 제외됨
directories_add = 다음 항목을 포함합니다
no_paths = 경로는 없습니다. 아래에 추가해주세요
directories_volume_header = 권 (卷)
directories_volume_refresh = 새로 고침
directories_volume_add = 추가
# Bottom navigation
nav_home = 시작
nav_dirs = 폴더 목록
nav_settings = 설정
# Status messages set from Rust
status_ready = 준비 완료
status_stopped = 정지됨
status_no_results = 결과가 없습니다
status_deleted_selected = 선택된 항목 삭제
status_deleted_with_errors = 오류와 함께 삭제됨
scan_not_started = 스캔이 시작되지 않았습니다
found_items_prefix = 발견됨
found_items_suffix = 아이템들
deleted_items_prefix = 삭제됨
deleted_items_suffix = 아이템들
deleted_errors_suffix = 오류들
renamed_prefix = 이름이 변경되었습니다
renamed_files_suffix = 파일들
renamed_errors_suffix = 오류들
cleaned_exif_prefix = EXIF 정보 제거 완료
cleaned_exif_suffix = 파일들
cleaned_exif_errors_suffix = 오류들
and_more_prefix = ...그리고
and_more_suffix = 더
# Gallery / delete popups
gallery_delete_button = 삭제
gallery_back = 뒤로
gallery_confirm_delete = 네, 삭제합니다
deleting_files = 파일 삭제 중...
stop = 정지
files_suffix = 파일들
scanning_fallback = 스캔중...
app_subtitle = 체디니아 전투(972년)를 기리며
app_license = Czkawka Core의 프론트엔드 - GPL-3.0 라이선스
about_app_label = 소개
cache_label = 캐시(CACHE)
# Notification
scan_completed_notification = 검색 완료 - { $file_count } 개의 항목이 발견되었습니다
# Confirm popups (set from Rust)
confirm_clean_exif = 선택된 { $n }개의 파일에서 EXIF 태그를 삭제하시겠습니까?
confirm_delete_items = 선택하신 { $n }개의 항목을 삭제하시겠습니까?
gallery_confirm_delete_msg = { $total_groups } 개의 그룹에 있는 { $total_images } 개의 이미지를 삭제하시려고 합니다.
gallery_confirm_delete_warning = 모든 항목은 { $unsafe_groups } 그룹에서 선택되었습니다!
# Settings - SameMusic fingerprint warning
same_music_fingerprint_warning = 오디오 지문 계산 및 비교는 많은 자원을 소모하며, 시간이 오래 걸릴 수 있습니다. 따라서 이 작업은 데스크톱 시스템에서 Krokiet을 사용하는 것이 좋습니다.
# Scan stage labels (shown during scan progress)
stage_collecting_files = 파일 수집 중
stage_scanning_name = 이름으로 검색
stage_scanning_size_name = 이름과 크기 기준으로 검색
stage_scanning_size = 크기에 따른 스캔
stage_pre_hash = 사전 해싱
stage_full_hash = 해싱 (Hashing)
stage_loading_cache = 캐시 로드 중
stage_saving_cache = 캐시 저장 중
stage_calculating_image_hashes = 이미지 해시 값 계산 중
stage_comparing_images = 이미지 비교하기
stage_calculating_video_hashes = 동영상 해시 값 계산 중
stage_checking_files = 파일 확인 중
stage_checking_extensions = 확장 프로그램 확인 중
stage_checking_names = 이름 확인 중
stage_reading_music_tags = 음악 파일의 태그 읽기
stage_comparing_tags = 태그 비교
stage_calculating_music_fingerprints = 음악 지문 계산하기
stage_comparing_fingerprints = 지문 비교하기
stage_extracting_exif = EXIF 태그 읽기
stage_creating_video_thumbnails = 비디오 썸네일 제작하기
stage_processing_videos = 동영상 처리 중입니다
stage_deleting = 파일 삭제하기
stage_renaming = 파일 이름 변경중
stage_moving = 항목 이동
stage_hardlinking = 하드 링크 생성하기
stage_symlinking = 심볼릭 링크 생성하기
stage_optimizing_videos = 동영상 최적화하기
stage_cleaning_exif = EXIF 정보 제거
# Group headers in scan results
duplicates_group_header = { $count } 개의 파일 x { $per_file } / 파일 = 총 { $total }
similar_images_group_header = { $count } 개의 유사 이미지
same_music_group_header = { $count } 개의 유사한 트랙
# Rename confirmation
confirm_rename_items = 선택된 { $n }개의 파일 이름을 정말 변경하시겠습니까?
# Combo-box option labels (translatable display names)
option_search_mode_biggest = 가장 큰
option_search_mode_smallest = 가장 작음
option_similarity_very_high = V. 매우 높음
option_similarity_high = 높음
option_similarity_medium = 보통
option_similarity_low = 낮음
option_similarity_very_low = V. 매우 낮음
option_similarity_minimal = 최소.
option_check_method_hash = 해시
option_check_method_name = 파일명
option_check_method_size_and_name = 사이즈 + 이름
option_check_method_size = 파일 크기
option_music_method_tags = 태그 기준 검사
option_music_method_audio = 오디오 파일
option_min_size_none = 없음
option_min_size_1kb = 1 KB
option_min_size_8kb = 8 KB
option_min_size_64kb = 64 KB
option_min_size_1mb = 1 MB
option_max_size_16kb = 16 KB
option_max_size_1mb = 1 MB
option_max_size_10mb = 10 MB
option_max_size_100mb = 100 MB
option_max_size_unlimited = 제한 없음
# Volume labels (shown in the directories screen)
volume_internal_storage = 내부 저장 공간
volume_sd_card = 메모리 카드 (SD 카드)
volume_storage = 저장 공간 용량
# Directories screen
directories_referenced_tooltip = 참조 (삭제되지 않음)
directories_include_section_header = 포함됨
directories_exclude_section_header = 제외됨
directories_custom_paths = 사용자 정의 경로
directories_check_button = 분석하십시오
directories_check_popup_title = 디렉터리 통계 정보
directories_check_label_included = 포함된 경로:
directories_check_label_excluded = 제외된 경로:
directories_check_label_referenced = 참조 경로:
directories_check_label_would_scan = 검사할 파일:
directories_check_label_processable = 처리 가능한 파일 형식:
directories_check_scanning = 스캔중...
directories_check_warning_no_processable = 처리 가능한 파일이 없습니다. 포함/제외된 폴더 설정을 확인해주세요
path_edit_title_include = 포함 목록에 추가
path_edit_title_exclude = 제외 목록에 추가
path_edit_placeholder = 경로를 입력하세요...
path_edit_not_exists = 경로가 존재하지 않습니다
path_edit_is_dir = 디렉토리
path_edit_is_file = 파일
path_edit_no_newlines = 경로는 줄 바꿈을 포함할 수 없습니다. Enter 키는 사용할 수 없습니다
ctx_menu_title = 열기
ctx_open_file = 미결 항목
ctx_open_folder = 상위 폴더 열기
