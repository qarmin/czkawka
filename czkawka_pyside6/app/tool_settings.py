from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QLabel, QComboBox, QCheckBox,
    QSlider, QLineEdit, QGroupBox, QFormLayout, QScrollArea,
    QHBoxLayout, QSpinBox, QPushButton, QSizePolicy
)
from PySide6.QtCore import Qt, Signal

from .models import (
    ActiveTab, ToolSettings, CheckingMethod, HashType,
    ImageHashAlg, ImageFilter, MusicSearchMethod,
    VideoCropDetect, VideoCropMechanism, VideoCodec,
)


class ToolSettingsPanel(QWidget):
    """Per-tool settings panel shown on the right side."""
    settings_changed = Signal()

    def __init__(self, tool_settings: ToolSettings, parent=None):
        super().__init__(parent)
        self._ts = tool_settings
        self._active_tab = ActiveTab.DUPLICATE_FILES
        self.setMinimumWidth(250)
        self.setMaximumWidth(350)
        self._panels: dict[ActiveTab, QWidget] = {}
        self._setup_ui()

    def _setup_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(4, 4, 4, 4)

        title = QLabel("Tool Settings")
        font = title.font()
        font.setBold(True)
        title.setFont(font)
        main_layout.addWidget(title)

        self._scroll = QScrollArea()
        self._scroll.setWidgetResizable(True)
        self._scroll.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOff)

        self._container = QWidget()
        self._container_layout = QVBoxLayout(self._container)
        self._container_layout.setContentsMargins(0, 0, 0, 0)

        # Create all panel widgets
        self._panels[ActiveTab.DUPLICATE_FILES] = self._create_duplicate_panel()
        self._panels[ActiveTab.SIMILAR_IMAGES] = self._create_similar_images_panel()
        self._panels[ActiveTab.SIMILAR_VIDEOS] = self._create_similar_videos_panel()
        self._panels[ActiveTab.SIMILAR_MUSIC] = self._create_similar_music_panel()
        self._panels[ActiveTab.BIG_FILES] = self._create_big_files_panel()
        self._panels[ActiveTab.BROKEN_FILES] = self._create_broken_files_panel()
        self._panels[ActiveTab.BAD_NAMES] = self._create_bad_names_panel()
        self._panels[ActiveTab.EXIF_REMOVER] = self._create_exif_panel()
        self._panels[ActiveTab.VIDEO_OPTIMIZER] = self._create_video_optimizer_panel()

        for panel in self._panels.values():
            self._container_layout.addWidget(panel)
            panel.setVisible(False)

        self._container_layout.addStretch()
        self._scroll.setWidget(self._container)
        main_layout.addWidget(self._scroll)

    def set_active_tab(self, tab: ActiveTab):
        self._active_tab = tab
        for t, panel in self._panels.items():
            panel.setVisible(t == tab)

    def _create_duplicate_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        # Check method
        self._dup_method = QComboBox()
        self._dup_method.addItems(["Hash", "Size", "Name", "Size and Name"])
        method_map = {CheckingMethod.HASH: 0, CheckingMethod.SIZE: 1,
                      CheckingMethod.NAME: 2, CheckingMethod.SIZE_NAME: 3}
        self._dup_method.setCurrentIndex(method_map.get(self._ts.dup_check_method, 0))
        self._dup_method.currentIndexChanged.connect(self._on_dup_method_changed)
        layout.addRow("Check Method:", self._dup_method)

        # Hash type
        self._dup_hash = QComboBox()
        self._dup_hash.addItems(["Blake3", "CRC32", "XXH3"])
        hash_map = {HashType.BLAKE3: 0, HashType.CRC32: 1, HashType.XXH3: 2}
        self._dup_hash.setCurrentIndex(hash_map.get(self._ts.dup_hash_type, 0))
        self._dup_hash.currentIndexChanged.connect(self._on_dup_hash_changed)
        layout.addRow("Hash Type:", self._dup_hash)

        # Case sensitive
        self._dup_case = QCheckBox("Case sensitive name comparison")
        self._dup_case.setChecked(self._ts.dup_name_case_sensitive)
        self._dup_case.toggled.connect(lambda v: setattr(self._ts, 'dup_name_case_sensitive', v))
        layout.addRow(self._dup_case)

        return panel

    def _on_dup_method_changed(self, idx):
        methods = [CheckingMethod.HASH, CheckingMethod.SIZE,
                   CheckingMethod.NAME, CheckingMethod.SIZE_NAME]
        self._ts.dup_check_method = methods[idx]
        self.settings_changed.emit()

    def _on_dup_hash_changed(self, idx):
        hashes = [HashType.BLAKE3, HashType.CRC32, HashType.XXH3]
        self._ts.dup_hash_type = hashes[idx]
        self.settings_changed.emit()

    def _create_similar_images_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        # Hash size
        self._img_hash_size = QComboBox()
        self._img_hash_size.addItems(["8", "16", "32", "64"])
        size_map = {8: 0, 16: 1, 32: 2, 64: 3}
        self._img_hash_size.setCurrentIndex(size_map.get(self._ts.img_hash_size, 1))
        self._img_hash_size.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'img_hash_size', [8, 16, 32, 64][idx])
        )
        layout.addRow("Hash Size:", self._img_hash_size)

        # Resize algorithm
        self._img_filter = QComboBox()
        self._img_filter.addItems(["Lanczos3", "Gaussian", "CatmullRom", "Triangle", "Nearest"])
        filters = [ImageFilter.LANCZOS3, ImageFilter.GAUSSIAN, ImageFilter.CATMULL_ROM,
                   ImageFilter.TRIANGLE, ImageFilter.NEAREST]
        filter_idx = filters.index(self._ts.img_filter) if self._ts.img_filter in filters else 4
        self._img_filter.setCurrentIndex(filter_idx)
        self._img_filter.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'img_filter', filters[idx])
        )
        layout.addRow("Resize Algorithm:", self._img_filter)

        # Hash type
        self._img_hash_alg = QComboBox()
        self._img_hash_alg.addItems(["Mean", "Gradient", "BlockHash", "VertGradient",
                                      "DoubleGradient", "Median"])
        algs = [ImageHashAlg.MEAN, ImageHashAlg.GRADIENT, ImageHashAlg.BLOCKHASH,
                ImageHashAlg.VERT_GRADIENT, ImageHashAlg.DOUBLE_GRADIENT, ImageHashAlg.MEDIAN]
        alg_idx = algs.index(self._ts.img_hash_alg) if self._ts.img_hash_alg in algs else 1
        self._img_hash_alg.setCurrentIndex(alg_idx)
        self._img_hash_alg.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'img_hash_alg', algs[idx])
        )
        layout.addRow("Hash Type:", self._img_hash_alg)

        # Ignore same size
        self._img_ignore_size = QCheckBox("Ignore same size")
        self._img_ignore_size.setChecked(self._ts.img_ignore_same_size)
        self._img_ignore_size.toggled.connect(
            lambda v: setattr(self._ts, 'img_ignore_same_size', v)
        )
        layout.addRow(self._img_ignore_size)

        # Max difference slider
        diff_layout = QHBoxLayout()
        self._img_diff_slider = QSlider(Qt.Horizontal)
        self._img_diff_slider.setRange(0, 40)
        self._img_diff_slider.setValue(self._ts.img_max_difference)
        self._img_diff_label = QLabel(str(self._ts.img_max_difference))
        self._img_diff_slider.valueChanged.connect(self._on_img_diff_changed)
        diff_layout.addWidget(self._img_diff_slider)
        diff_layout.addWidget(self._img_diff_label)
        layout.addRow("Max Difference:", diff_layout)

        return panel

    def _on_img_diff_changed(self, value):
        self._ts.img_max_difference = value
        self._img_diff_label.setText(str(value))
        self.settings_changed.emit()

    def _create_similar_videos_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        # Ignore same size
        vid_ignore = QCheckBox("Ignore same size")
        vid_ignore.setChecked(self._ts.vid_ignore_same_size)
        vid_ignore.toggled.connect(lambda v: setattr(self._ts, 'vid_ignore_same_size', v))
        layout.addRow(vid_ignore)

        # Crop detect
        self._vid_crop = QComboBox()
        self._vid_crop.addItems(["LetterBox", "Motion", "None"])
        crops = [VideoCropDetect.LETTERBOX, VideoCropDetect.MOTION, VideoCropDetect.NONE]
        crop_idx = crops.index(self._ts.vid_crop_detect) if self._ts.vid_crop_detect in crops else 0
        self._vid_crop.setCurrentIndex(crop_idx)
        self._vid_crop.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'vid_crop_detect', crops[idx])
        )
        layout.addRow("Crop Detect:", self._vid_crop)

        # Max difference
        diff_layout = QHBoxLayout()
        self._vid_diff_slider = QSlider(Qt.Horizontal)
        self._vid_diff_slider.setRange(0, 20)
        self._vid_diff_slider.setValue(self._ts.vid_max_difference)
        self._vid_diff_label = QLabel(str(self._ts.vid_max_difference))
        self._vid_diff_slider.valueChanged.connect(lambda v: (
            setattr(self._ts, 'vid_max_difference', v),
            self._vid_diff_label.setText(str(v))
        ))
        diff_layout.addWidget(self._vid_diff_slider)
        diff_layout.addWidget(self._vid_diff_label)
        layout.addRow("Max Difference:", diff_layout)

        # Skip forward
        skip_layout = QHBoxLayout()
        self._vid_skip_slider = QSlider(Qt.Horizontal)
        self._vid_skip_slider.setRange(1, 360)
        self._vid_skip_slider.setValue(self._ts.vid_skip_forward)
        self._vid_skip_label = QLabel(str(self._ts.vid_skip_forward))
        self._vid_skip_slider.valueChanged.connect(lambda v: (
            setattr(self._ts, 'vid_skip_forward', v),
            self._vid_skip_label.setText(str(v))
        ))
        skip_layout.addWidget(self._vid_skip_slider)
        skip_layout.addWidget(self._vid_skip_label)
        layout.addRow("Skip Forward (s):", skip_layout)

        # Duration
        dur_layout = QHBoxLayout()
        self._vid_dur_slider = QSlider(Qt.Horizontal)
        self._vid_dur_slider.setRange(1, 60)
        self._vid_dur_slider.setValue(self._ts.vid_duration)
        self._vid_dur_label = QLabel(str(self._ts.vid_duration))
        self._vid_dur_slider.valueChanged.connect(lambda v: (
            setattr(self._ts, 'vid_duration', v),
            self._vid_dur_label.setText(str(v))
        ))
        dur_layout.addWidget(self._vid_dur_slider)
        dur_layout.addWidget(self._vid_dur_label)
        layout.addRow("Hash Duration (s):", dur_layout)

        return panel

    def _create_similar_music_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        # Search method
        self._music_method = QComboBox()
        self._music_method.addItems(["Tags", "Fingerprint"])
        self._music_method.setCurrentIndex(
            0 if self._ts.music_search_method == MusicSearchMethod.TAGS else 1
        )
        self._music_method.currentIndexChanged.connect(self._on_music_method_changed)
        layout.addRow("Audio Check Type:", self._music_method)

        # Tags group
        self._tags_group = QGroupBox("Tag Matching")
        tags_layout = QVBoxLayout(self._tags_group)

        self._music_approx = QCheckBox("Approximate comparison")
        self._music_approx.setChecked(self._ts.music_approximate)
        self._music_approx.toggled.connect(lambda v: setattr(self._ts, 'music_approximate', v))
        tags_layout.addWidget(self._music_approx)

        for attr, label in [
            ("music_title", "Title"), ("music_artist", "Artist"),
            ("music_bitrate", "Bitrate"), ("music_genre", "Genre"),
            ("music_year", "Year"), ("music_length", "Length"),
        ]:
            cb = QCheckBox(label)
            cb.setChecked(getattr(self._ts, attr))
            cb.toggled.connect(lambda v, a=attr: setattr(self._ts, a, v))
            tags_layout.addWidget(cb)

        layout.addRow(self._tags_group)

        # Fingerprint group
        self._fp_group = QGroupBox("Fingerprint Matching")
        fp_layout = QFormLayout(self._fp_group)

        fp_similar = QCheckBox("Compare with similar titles")
        fp_similar.setChecked(self._ts.music_compare_fingerprints_similar_titles)
        fp_similar.toggled.connect(
            lambda v: setattr(self._ts, 'music_compare_fingerprints_similar_titles', v)
        )
        fp_layout.addRow(fp_similar)

        diff_layout = QHBoxLayout()
        self._music_diff_slider = QSlider(Qt.Horizontal)
        self._music_diff_slider.setRange(0, 100)
        self._music_diff_slider.setValue(int(self._ts.music_max_difference * 10))
        self._music_diff_label = QLabel(f"{self._ts.music_max_difference:.1f}")
        self._music_diff_slider.valueChanged.connect(lambda v: (
            setattr(self._ts, 'music_max_difference', v / 10.0),
            self._music_diff_label.setText(f"{v / 10.0:.1f}")
        ))
        diff_layout.addWidget(self._music_diff_slider)
        diff_layout.addWidget(self._music_diff_label)
        fp_layout.addRow("Max Difference:", diff_layout)

        layout.addRow(self._fp_group)
        self._on_music_method_changed(self._music_method.currentIndex())

        return panel

    def _on_music_method_changed(self, idx):
        if idx == 0:
            self._ts.music_search_method = MusicSearchMethod.TAGS
        else:
            self._ts.music_search_method = MusicSearchMethod.CONTENT
        self._tags_group.setVisible(idx == 0)
        self._fp_group.setVisible(idx == 1)
        self.settings_changed.emit()

    def _create_big_files_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        self._big_mode = QComboBox()
        self._big_mode.addItems(["The Biggest", "The Smallest"])
        self._big_mode.setCurrentIndex(0 if self._ts.big_files_mode == "biggest" else 1)
        self._big_mode.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'big_files_mode', "biggest" if idx == 0 else "smallest")
        )
        layout.addRow("Method:", self._big_mode)

        self._big_count = QSpinBox()
        self._big_count.setRange(1, 100000)
        self._big_count.setValue(self._ts.big_files_number)
        self._big_count.valueChanged.connect(
            lambda v: setattr(self._ts, 'big_files_number', v)
        )
        layout.addRow("Number of Files:", self._big_count)

        return panel

    def _create_broken_files_panel(self) -> QWidget:
        panel = QWidget()
        layout = QVBoxLayout(panel)
        layout.addWidget(QLabel("File types to check:"))

        for attr, label in [
            ("broken_audio", "Audio"), ("broken_pdf", "PDF"),
            ("broken_archive", "Archive"), ("broken_image", "Image"),
            ("broken_video", "Video"),
        ]:
            cb = QCheckBox(label)
            cb.setChecked(getattr(self._ts, attr))
            cb.toggled.connect(lambda v, a=attr: setattr(self._ts, a, v))
            layout.addWidget(cb)

        layout.addStretch()
        return panel

    def _create_bad_names_panel(self) -> QWidget:
        panel = QWidget()
        layout = QVBoxLayout(panel)
        layout.addWidget(QLabel("Check for:"))

        checks = [
            ("bad_names_uppercase_ext", "Uppercase extension",
             "Files with .JPG, .PNG etc."),
            ("bad_names_emoji", "Emoji in name",
             "Files containing emoji characters"),
            ("bad_names_space", "Space at start/end",
             "Leading or trailing whitespace"),
            ("bad_names_non_ascii", "Non-ASCII characters",
             "Characters outside ASCII range"),
            ("bad_names_remove_duplicated", "Remove duplicated non-alphanumeric",
             "e.g. file--name..txt"),
        ]

        for attr, label, hint in checks:
            cb = QCheckBox(label)
            cb.setChecked(getattr(self._ts, attr))
            cb.setToolTip(hint)
            cb.toggled.connect(lambda v, a=attr: setattr(self._ts, a, v))
            layout.addWidget(cb)

        # Restricted charset
        layout.addWidget(QLabel("Restricted charset:"))
        self._bad_names_charset = QLineEdit(self._ts.bad_names_restricted_charset)
        self._bad_names_charset.setPlaceholderText("Allowed special chars, comma-separated")
        self._bad_names_charset.textChanged.connect(
            lambda t: setattr(self._ts, 'bad_names_restricted_charset', t)
        )
        layout.addWidget(self._bad_names_charset)

        layout.addStretch()
        return panel

    def _create_exif_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        self._exif_tags = QLineEdit(self._ts.exif_ignored_tags)
        self._exif_tags.setPlaceholderText("Tags to ignore, comma-separated")
        self._exif_tags.textChanged.connect(
            lambda t: setattr(self._ts, 'exif_ignored_tags', t)
        )
        layout.addRow("Ignored EXIF Tags:", self._exif_tags)

        return panel

    def _create_video_optimizer_panel(self) -> QWidget:
        panel = QWidget()
        layout = QFormLayout(panel)

        # Mode
        self._vo_mode = QComboBox()
        self._vo_mode.addItems(["Crop", "Transcode"])
        self._vo_mode.setCurrentIndex(0 if self._ts.video_opt_mode == "crop" else 1)
        self._vo_mode.currentIndexChanged.connect(self._on_vo_mode_changed)
        layout.addRow("Mode:", self._vo_mode)

        # Crop settings
        self._crop_group = QGroupBox("Crop Settings")
        crop_layout = QFormLayout(self._crop_group)

        self._vo_crop_type = QComboBox()
        self._vo_crop_type.addItems(["Black Bars", "Static Content"])
        mechs = [VideoCropMechanism.BLACKBARS, VideoCropMechanism.STATICCONTENT]
        mech_idx = mechs.index(self._ts.video_crop_mechanism) if self._ts.video_crop_mechanism in mechs else 0
        self._vo_crop_type.setCurrentIndex(mech_idx)
        self._vo_crop_type.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'video_crop_mechanism', mechs[idx])
        )
        crop_layout.addRow("Crop Type:", self._vo_crop_type)

        self._vo_threshold = QSpinBox()
        self._vo_threshold.setRange(0, 128)
        self._vo_threshold.setValue(self._ts.video_black_pixel_threshold)
        self._vo_threshold.valueChanged.connect(
            lambda v: setattr(self._ts, 'video_black_pixel_threshold', v)
        )
        crop_layout.addRow("Black Pixel Threshold:", self._vo_threshold)

        self._vo_bar_pct = QSpinBox()
        self._vo_bar_pct.setRange(50, 100)
        self._vo_bar_pct.setValue(self._ts.video_black_bar_percentage)
        self._vo_bar_pct.valueChanged.connect(
            lambda v: setattr(self._ts, 'video_black_bar_percentage', v)
        )
        crop_layout.addRow("Black Bar Min %:", self._vo_bar_pct)

        self._vo_samples = QSpinBox()
        self._vo_samples.setRange(5, 1000)
        self._vo_samples.setValue(self._ts.video_max_samples)
        self._vo_samples.valueChanged.connect(
            lambda v: setattr(self._ts, 'video_max_samples', v)
        )
        crop_layout.addRow("Max Samples:", self._vo_samples)

        self._vo_min_crop = QSpinBox()
        self._vo_min_crop.setRange(1, 1000)
        self._vo_min_crop.setValue(self._ts.video_min_crop_size)
        self._vo_min_crop.valueChanged.connect(
            lambda v: setattr(self._ts, 'video_min_crop_size', v)
        )
        crop_layout.addRow("Min Crop Size:", self._vo_min_crop)

        layout.addRow(self._crop_group)

        # Transcode settings
        self._transcode_group = QGroupBox("Transcode Settings")
        tc_layout = QFormLayout(self._transcode_group)

        self._vo_codecs = QLineEdit(self._ts.video_excluded_codecs)
        self._vo_codecs.textChanged.connect(
            lambda t: setattr(self._ts, 'video_excluded_codecs', t)
        )
        tc_layout.addRow("Excluded Codecs:", self._vo_codecs)

        self._vo_codec = QComboBox()
        self._vo_codec.addItems(["H264", "H265", "AV1", "VP9"])
        codecs = [VideoCodec.H264, VideoCodec.H265, VideoCodec.AV1, VideoCodec.VP9]
        codec_idx = codecs.index(self._ts.video_codec) if self._ts.video_codec in codecs else 1
        self._vo_codec.setCurrentIndex(codec_idx)
        self._vo_codec.currentIndexChanged.connect(
            lambda idx: setattr(self._ts, 'video_codec', codecs[idx])
        )
        tc_layout.addRow("Target Codec:", self._vo_codec)

        self._vo_quality = QSpinBox()
        self._vo_quality.setRange(0, 51)
        self._vo_quality.setValue(self._ts.video_quality)
        self._vo_quality.valueChanged.connect(
            lambda v: setattr(self._ts, 'video_quality', v)
        )
        tc_layout.addRow("Quality:", self._vo_quality)

        self._vo_fail_bigger = QCheckBox("Fail if not smaller")
        self._vo_fail_bigger.setChecked(self._ts.video_fail_if_bigger)
        self._vo_fail_bigger.toggled.connect(
            lambda v: setattr(self._ts, 'video_fail_if_bigger', v)
        )
        tc_layout.addRow(self._vo_fail_bigger)

        layout.addRow(self._transcode_group)

        self._on_vo_mode_changed(self._vo_mode.currentIndex())
        return panel

    def _on_vo_mode_changed(self, idx):
        mode = "crop" if idx == 0 else "transcode"
        self._ts.video_opt_mode = mode
        self._crop_group.setVisible(idx == 0)
        self._transcode_group.setVisible(idx == 1)
        self.settings_changed.emit()
