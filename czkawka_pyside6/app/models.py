from enum import Enum, auto
from dataclasses import dataclass, field
from typing import Optional
from pathlib import Path


class ActiveTab(Enum):
    DUPLICATE_FILES = auto()
    EMPTY_FOLDERS = auto()
    BIG_FILES = auto()
    EMPTY_FILES = auto()
    TEMPORARY_FILES = auto()
    SIMILAR_IMAGES = auto()
    SIMILAR_VIDEOS = auto()
    SIMILAR_MUSIC = auto()
    INVALID_SYMLINKS = auto()
    BROKEN_FILES = auto()
    BAD_EXTENSIONS = auto()
    BAD_NAMES = auto()
    EXIF_REMOVER = auto()
    VIDEO_OPTIMIZER = auto()
    SETTINGS = auto()
    ABOUT = auto()


class SelectMode(Enum):
    SELECT_ALL = auto()
    UNSELECT_ALL = auto()
    INVERT_SELECTION = auto()
    SELECT_BIGGEST_SIZE = auto()
    SELECT_BIGGEST_RESOLUTION = auto()
    SELECT_SMALLEST_SIZE = auto()
    SELECT_SMALLEST_RESOLUTION = auto()
    SELECT_NEWEST = auto()
    SELECT_OLDEST = auto()
    SELECT_SHORTEST_PATH = auto()
    SELECT_LONGEST_PATH = auto()
    SELECT_CUSTOM = auto()


class DeleteMethod(Enum):
    NONE = "NONE"
    DELETE = "DELETE"
    ALL_EXCEPT_NEWEST = "AEN"
    ALL_EXCEPT_OLDEST = "AEO"
    ONE_OLDEST = "OO"
    ONE_NEWEST = "ON"
    HARDLINK = "HARD"
    ALL_EXCEPT_BIGGEST = "AEB"
    ALL_EXCEPT_SMALLEST = "AES"
    ONE_BIGGEST = "OB"
    ONE_SMALLEST = "OS"


class CheckingMethod(Enum):
    HASH = "HASH"
    SIZE = "SIZE"
    NAME = "NAME"
    SIZE_NAME = "SIZE_NAME"


class HashType(Enum):
    BLAKE3 = "BLAKE3"
    CRC32 = "CRC32"
    XXH3 = "XXH3"


class ImageHashAlg(Enum):
    MEAN = "Mean"
    GRADIENT = "Gradient"
    BLOCKHASH = "Blockhash"
    VERT_GRADIENT = "VertGradient"
    DOUBLE_GRADIENT = "DoubleGradient"
    MEDIAN = "Median"


class ImageFilter(Enum):
    LANCZOS3 = "Lanczos3"
    NEAREST = "Nearest"
    TRIANGLE = "Triangle"
    GAUSSIAN = "Gaussian"
    CATMULL_ROM = "CatmullRom"


class MusicSearchMethod(Enum):
    TAGS = "TAGS"
    CONTENT = "CONTENT"


class VideoCropDetect(Enum):
    NONE = "none"
    LETTERBOX = "letterbox"
    MOTION = "motion"


class VideoCropMechanism(Enum):
    BLACKBARS = "blackbars"
    STATICCONTENT = "staticcontent"


class VideoCodec(Enum):
    H264 = "h264"
    H265 = "h265"
    AV1 = "av1"
    VP9 = "vp9"


# Map ActiveTab to CLI subcommand names
TAB_TO_CLI_COMMAND = {
    ActiveTab.DUPLICATE_FILES: "dup",
    ActiveTab.EMPTY_FOLDERS: "empty-folders",
    ActiveTab.BIG_FILES: "big",
    ActiveTab.EMPTY_FILES: "empty-files",
    ActiveTab.TEMPORARY_FILES: "temp",
    ActiveTab.SIMILAR_IMAGES: "image",
    ActiveTab.SIMILAR_VIDEOS: "video",
    ActiveTab.SIMILAR_MUSIC: "music",
    ActiveTab.INVALID_SYMLINKS: "symlinks",
    ActiveTab.BROKEN_FILES: "broken",
    ActiveTab.BAD_EXTENSIONS: "ext",
    ActiveTab.BAD_NAMES: "bad-names",
    ActiveTab.EXIF_REMOVER: "exif-remover",
    ActiveTab.VIDEO_OPTIMIZER: "video-optimizer",
}

TAB_DISPLAY_NAMES = {
    ActiveTab.DUPLICATE_FILES: "Duplicate Files",
    ActiveTab.EMPTY_FOLDERS: "Empty Folders",
    ActiveTab.BIG_FILES: "Big Files",
    ActiveTab.EMPTY_FILES: "Empty Files",
    ActiveTab.TEMPORARY_FILES: "Temporary Files",
    ActiveTab.SIMILAR_IMAGES: "Similar Images",
    ActiveTab.SIMILAR_VIDEOS: "Similar Videos",
    ActiveTab.SIMILAR_MUSIC: "Similar Music",
    ActiveTab.INVALID_SYMLINKS: "Invalid Symlinks",
    ActiveTab.BROKEN_FILES: "Broken Files",
    ActiveTab.BAD_EXTENSIONS: "Bad Extensions",
    ActiveTab.BAD_NAMES: "Bad Names",
    ActiveTab.EXIF_REMOVER: "EXIF Remover",
    ActiveTab.VIDEO_OPTIMIZER: "Video Optimizer",
}

# Which tabs support grouping (results are in groups)
GROUPED_TABS = {
    ActiveTab.DUPLICATE_FILES,
    ActiveTab.SIMILAR_IMAGES,
    ActiveTab.SIMILAR_VIDEOS,
    ActiveTab.SIMILAR_MUSIC,
}

# Which tabs have per-tool settings
TABS_WITH_SETTINGS = {
    ActiveTab.DUPLICATE_FILES,
    ActiveTab.SIMILAR_IMAGES,
    ActiveTab.SIMILAR_VIDEOS,
    ActiveTab.SIMILAR_MUSIC,
    ActiveTab.BIG_FILES,
    ActiveTab.BROKEN_FILES,
    ActiveTab.BAD_NAMES,
    ActiveTab.EXIF_REMOVER,
    ActiveTab.VIDEO_OPTIMIZER,
}

# Column definitions per tab
TAB_COLUMNS = {
    ActiveTab.DUPLICATE_FILES: ["Selection", "Size", "File Name", "Path", "Modification Date", "Hash"],
    ActiveTab.EMPTY_FOLDERS: ["Selection", "Folder Name", "Path", "Modification Date"],
    ActiveTab.BIG_FILES: ["Selection", "Size", "File Name", "Path", "Modification Date"],
    ActiveTab.EMPTY_FILES: ["Selection", "File Name", "Path", "Modification Date"],
    ActiveTab.TEMPORARY_FILES: ["Selection", "File Name", "Path", "Modification Date"],
    ActiveTab.SIMILAR_IMAGES: ["Selection", "Similarity", "Size", "Resolution", "File Name", "Path", "Modification Date", "Hash"],
    ActiveTab.SIMILAR_VIDEOS: ["Selection", "Size", "File Name", "Path", "Modification Date"],
    ActiveTab.SIMILAR_MUSIC: ["Selection", "Size", "File Name", "Path", "Title", "Artist", "Year", "Bitrate", "Genre", "Length"],
    ActiveTab.INVALID_SYMLINKS: ["Selection", "Symlink Name", "Symlink Path", "Destination Path", "Type of Error"],
    ActiveTab.BROKEN_FILES: ["Selection", "File Name", "Path", "Error Type", "Size", "Modification Date"],
    ActiveTab.BAD_EXTENSIONS: ["Selection", "File Name", "Path", "Current Extension", "Proper Extension"],
    ActiveTab.BAD_NAMES: ["Selection", "File Name", "Path", "Error Type"],
    ActiveTab.EXIF_REMOVER: ["Selection", "File Name", "Path"],
    ActiveTab.VIDEO_OPTIMIZER: ["Selection", "File Name", "Path", "Size", "Codec"],
}


@dataclass
class ResultEntry:
    """A single result entry from a scan."""
    values: dict  # column_name -> value
    checked: bool = False
    header_row: bool = False  # Group header for grouped results
    group_id: int = 0


@dataclass
class ScanProgress:
    """Progress information during scanning."""
    step_name: str = ""
    current: int = 0
    total: int = 0
    current_size: int = 0
    # Raw fields from czkawka_cli --json-progress
    stage_name: str = ""
    current_stage_idx: int = 0
    max_stage_idx: int = 0
    entries_checked: int = 0
    entries_to_check: int = 0
    bytes_checked: int = 0
    bytes_to_check: int = 0


@dataclass
class ToolSettings:
    """Per-tool settings that map to CLI arguments."""
    # Duplicates
    dup_check_method: CheckingMethod = CheckingMethod.HASH
    dup_hash_type: HashType = HashType.BLAKE3
    dup_name_case_sensitive: bool = False
    dup_min_size: str = "8192"
    dup_max_size: str = ""
    dup_min_cache_size: str = "257144"
    dup_use_prehash: bool = True
    dup_min_prehash_cache_size: str = "257144"

    # Similar Images
    img_hash_size: int = 16  # 8, 16, 32, 64
    img_filter: ImageFilter = ImageFilter.NEAREST
    img_hash_alg: ImageHashAlg = ImageHashAlg.GRADIENT
    img_ignore_same_size: bool = False
    img_max_difference: int = 5  # 0-40

    # Similar Videos
    vid_ignore_same_size: bool = False
    vid_crop_detect: VideoCropDetect = VideoCropDetect.LETTERBOX
    vid_max_difference: int = 10  # 0-20
    vid_skip_forward: int = 15  # 0-300
    vid_duration: int = 10  # 1-60

    # Similar Music
    music_search_method: MusicSearchMethod = MusicSearchMethod.TAGS
    music_approximate: bool = False
    music_title: bool = True
    music_artist: bool = True
    music_bitrate: bool = False
    music_genre: bool = False
    music_year: bool = False
    music_length: bool = False
    music_compare_fingerprints_similar_titles: bool = False
    music_max_difference: float = 2.0
    music_min_segment_duration: float = 10.0

    # Big Files
    big_files_mode: str = "biggest"  # biggest or smallest
    big_files_number: int = 50

    # Broken Files
    broken_audio: bool = True
    broken_pdf: bool = True
    broken_archive: bool = True
    broken_image: bool = True
    broken_video: bool = False

    # Bad Names
    bad_names_uppercase_ext: bool = True
    bad_names_emoji: bool = True
    bad_names_space: bool = True
    bad_names_non_ascii: bool = True
    bad_names_restricted_charset: str = ""
    bad_names_remove_duplicated: bool = False

    # EXIF Remover
    exif_ignored_tags: str = ""

    # Video Optimizer
    video_opt_mode: str = "crop"  # crop or transcode
    video_crop_mechanism: VideoCropMechanism = VideoCropMechanism.BLACKBARS
    video_black_pixel_threshold: int = 32
    video_black_bar_percentage: int = 90
    video_max_samples: int = 20
    video_min_crop_size: int = 10
    video_excluded_codecs: str = "h265,hevc,av1,vp9"
    video_codec: VideoCodec = VideoCodec.H265
    video_quality: int = 23
    video_fail_if_bigger: bool = False
    video_overwrite: bool = False
    video_max_width: int = 1920
    video_max_height: int = 1080


@dataclass
class AppSettings:
    """Global application settings."""
    included_paths: list = field(default_factory=lambda: [str(Path.home())])
    excluded_paths: list = field(default_factory=list)
    excluded_items: str = ""
    allowed_extensions: str = ""
    excluded_extensions: str = ""
    minimum_file_size: str = ""
    maximum_file_size: str = ""
    recursive_search: bool = True
    use_cache: bool = True
    save_as_json: bool = False
    move_to_trash: bool = True
    hide_hard_links: bool = False
    thread_number: int = 0  # 0 = all available
    dark_theme: bool = True
    show_image_preview: bool = True
    czkawka_cli_path: str = "czkawka_cli"  # path to CLI binary
