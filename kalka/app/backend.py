import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Optional

from PySide6.QtCore import QObject, QThread, Signal

from .models import (
    ActiveTab, AppSettings, ToolSettings, ResultEntry, ScanProgress,
    TAB_TO_CLI_COMMAND, GROUPED_TABS, CheckingMethod, MusicSearchMethod,
)


class ScanWorker(QObject):
    """Worker that runs czkawka_cli in a background thread."""
    finished = Signal(object, list)  # (ActiveTab, results)
    progress = Signal(object)  # ScanProgress
    error = Signal(str)

    def __init__(self, tab: ActiveTab, app_settings: AppSettings,
                 tool_settings: ToolSettings):
        super().__init__()
        self.tab = tab
        self.app_settings = app_settings
        self.tool_settings = tool_settings
        self._process: Optional[subprocess.Popen] = None
        self._cancelled = False

    def run(self):
        try:
            cmd = self._build_command()
            if not cmd:
                self.error.emit(f"No CLI command for tab {self.tab}")
                return

            self.progress.emit(ScanProgress(
                step_name="Collecting files...", current=0, total=0
            ))

            with tempfile.NamedTemporaryFile(suffix=".json", delete=False, mode="w") as f:
                json_output_path = f.name

            try:
                # Add JSON output flag (use long form to avoid conflict with -C in broken files)
                cmd.extend(["--compact-file-to-save", json_output_path])
                # Enable JSON progress on stderr, suppress text output
                cmd.extend(["--json-progress", "-N", "-M"])

                # Prepend nice/ionice for low-priority scanning on Linux
                if self.app_settings.low_priority_scan and sys.platform.startswith("linux"):
                    prefix = []
                    if shutil.which("ionice"):
                        prefix.extend(["ionice", "-c", "3"])
                    if shutil.which("nice"):
                        prefix.extend(["nice", "-n", "19"])
                    cmd = prefix + cmd

                self._process = subprocess.Popen(
                    cmd,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.PIPE,
                    text=True,
                )

                # Read stderr line-by-line for JSON progress data
                self._monitor_process_json(json_output_path)

                if self._cancelled:
                    return

                # Check for CLI errors (exit code 11 = results found, not an error)
                returncode = self._process.returncode
                if returncode is not None and returncode != 0 and returncode != 11:
                    error_msg = f"czkawka_cli exited with code {returncode}"
                    self.error.emit(error_msg)
                    return

                # Parse results
                self.progress.emit(ScanProgress(
                    step_name="Loading results...", current=0, total=0
                ))
                results = self._parse_results(json_output_path)
                self.finished.emit(self.tab, results)
            finally:
                self._cleanup(json_output_path)

        except FileNotFoundError:
            self.error.emit(
                f"czkawka_cli not found at '{self.app_settings.czkawka_cli_path}'. "
                "Please install czkawka_cli or set the correct path in settings."
            )
        except Exception as e:
            self.error.emit(str(e))

    def cancel(self):
        self._cancelled = True
        if self._process and self._process.poll() is None:
            self._process.terminate()
            try:
                self._process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self._process.kill()
                self._process.wait()

    def _monitor_process_json(self, json_path: str):
        """Read JSON progress lines from stderr in real-time."""
        import time, logging

        while self._process.poll() is None:
            if self._cancelled:
                return

            line = self._process.stderr.readline()
            if not line:
                time.sleep(0.05)
                continue

            self._parse_progress_line(line.strip())

        # Drain and parse remaining stderr after process exits
        remaining = self._process.stderr.read()
        if remaining:
            for line in remaining.strip().split("\n"):
                self._parse_progress_line(line.strip())

    def _parse_progress_line(self, line: str):
        """Parse a single JSON progress line from stderr."""
        if not line:
            return
        try:
            data = json.loads(line)
            progress = data.get("progress", {})
            stage_name = data.get("stage_name", "Processing...")

            self.progress.emit(ScanProgress(
                step_name=stage_name,
                current=0,
                total=0,
                current_size=progress.get("bytes_checked", 0),
                stage_name=stage_name,
                current_stage_idx=progress.get("current_stage_idx", 0),
                max_stage_idx=progress.get("max_stage_idx", 0),
                entries_checked=progress.get("entries_checked", 0),
                entries_to_check=progress.get("entries_to_check", 0),
                bytes_checked=progress.get("bytes_checked", 0),
                bytes_to_check=progress.get("bytes_to_check", 0),
            ))
        except (json.JSONDecodeError, KeyError, TypeError):
            import logging
            if line.startswith("{"):
                logging.warning("Failed to parse progress JSON: %s", line[:200])

    def _cleanup(self, path: str):
        try:
            os.unlink(path)
        except OSError:
            pass

    def _build_command(self) -> list[str]:
        s = self.app_settings
        ts = self.tool_settings
        cli = s.czkawka_cli_path
        subcmd = TAB_TO_CLI_COMMAND.get(self.tab)
        if not subcmd:
            return []

        # Handle video-optimizer subcommands
        if self.tab == ActiveTab.VIDEO_OPTIMIZER:
            if ts.video_opt_mode == "crop":
                cmd = [cli, "video-optimizer", "crop"]
            else:
                cmd = [cli, "video-optimizer", "transcode"]
        else:
            cmd = [cli, subcmd]

        # Common args
        if s.included_paths:
            cmd.extend(["-d", ",".join(s.included_paths)])
        if s.excluded_paths:
            cmd.extend(["-e", ",".join(s.excluded_paths)])
        if s.excluded_items:
            cmd.extend(["-E", s.excluded_items])
        if s.allowed_extensions:
            cmd.extend(["-x", s.allowed_extensions])
        if s.excluded_extensions:
            cmd.extend(["-P", s.excluded_extensions])
        if not s.recursive_search:
            cmd.append("-R")
        if not s.use_cache:
            cmd.append("-H")
        if s.thread_number > 0:
            cmd.extend(["-T", str(s.thread_number)])

        # Tool-specific args
        if self.tab == ActiveTab.DUPLICATE_FILES:
            cmd.extend(["-s", ts.dup_check_method.value])
            cmd.extend(["-t", ts.dup_hash_type.value])
            if ts.dup_min_size:
                cmd.extend(["-m", ts.dup_min_size])
            if ts.dup_max_size:
                cmd.extend(["-i", ts.dup_max_size])
            if ts.dup_name_case_sensitive:
                cmd.append("-l")
            if not s.hide_hard_links:
                cmd.append("-L")
            if ts.dup_use_prehash:
                cmd.append("-u")
            if ts.dup_check_method == CheckingMethod.FUZZY_NAME:
                cmd.extend(["--name-similarity-threshold", str(ts.dup_name_similarity_threshold)])

        elif self.tab == ActiveTab.SIMILAR_IMAGES:
            cmd.extend(["-g", ts.img_hash_alg.value])
            cmd.extend(["-z", ts.img_filter.value])
            cmd.extend(["-c", str(ts.img_hash_size)])
            cmd.extend(["-s", str(ts.img_max_difference)])
            if ts.img_ignore_same_size:
                cmd.append("-J")

        elif self.tab == ActiveTab.SIMILAR_VIDEOS:
            cmd.extend(["-t", str(ts.vid_max_difference)])
            cmd.extend(["-U", str(ts.vid_skip_forward)])
            cmd.extend(["-B", ts.vid_crop_detect.value])
            cmd.extend(["-A", str(ts.vid_duration)])
            if ts.vid_ignore_same_size:
                cmd.append("-J")

        elif self.tab == ActiveTab.SIMILAR_MUSIC:
            cmd.extend(["-s", ts.music_search_method.value])
            if ts.music_search_method == MusicSearchMethod.TAGS:
                tags = []
                if ts.music_title: tags.append("track_title")
                if ts.music_artist: tags.append("track_artist")
                if ts.music_bitrate: tags.append("bitrate")
                if ts.music_genre: tags.append("genre")
                if ts.music_year: tags.append("year")
                if ts.music_length: tags.append("length")
                if tags:
                    cmd.extend(["-z", ",".join(tags)])
                if ts.music_approximate:
                    cmd.append("-a")
            else:
                cmd.extend(["-Y", str(ts.music_max_difference)])

        elif self.tab == ActiveTab.BIG_FILES:
            cmd.extend(["-n", str(ts.big_files_number)])
            if ts.big_files_mode == "smallest":
                cmd.append("-J")

        elif self.tab == ActiveTab.BROKEN_FILES:
            types = []
            if ts.broken_audio: types.append("AUDIO")
            if ts.broken_pdf: types.append("PDF")
            if ts.broken_archive: types.append("ARCHIVE")
            if ts.broken_image: types.append("IMAGE")
            if ts.broken_video: types.append("VIDEO")
            if types:
                cmd.extend(["-C", ",".join(types)])

        elif self.tab == ActiveTab.BAD_NAMES:
            if ts.bad_names_uppercase_ext:
                cmd.append("-u")
            if ts.bad_names_emoji:
                cmd.append("-j")
            if ts.bad_names_space:
                cmd.append("-w")
            if ts.bad_names_non_ascii:
                cmd.append("-n")
            if ts.bad_names_restricted_charset:
                cmd.extend(["-r", ts.bad_names_restricted_charset])
            if ts.bad_names_remove_duplicated:
                cmd.append("-a")

        elif self.tab == ActiveTab.VIDEO_OPTIMIZER:
            if ts.video_opt_mode == "crop":
                cmd.extend(["-m", ts.video_crop_mechanism.value])
                cmd.extend(["-k", str(ts.video_black_pixel_threshold)])
                cmd.extend(["-b", str(ts.video_black_bar_percentage)])
                cmd.extend(["-s", str(ts.video_max_samples)])
                cmd.extend(["-z", str(ts.video_min_crop_size)])
            else:
                if ts.video_excluded_codecs:
                    cmd.extend(["-c", ts.video_excluded_codecs])
                cmd.extend(["--target-codec", ts.video_codec.value])
                cmd.extend(["--quality", str(ts.video_quality)])
                if ts.video_fail_if_bigger:
                    cmd.append("--fail-if-not-smaller")

        return cmd

    def _parse_results(self, json_path: str) -> list[ResultEntry]:
        try:
            with open(json_path) as f:
                data = json.load(f)
        except (json.JSONDecodeError, FileNotFoundError, OSError):
            return []

        if not data:
            return []

        results = []
        is_grouped = self.tab in GROUPED_TABS

        if is_grouped:
            # Grouped tools return either:
            #   dict {size_key: [[entry, ...], [entry, ...]], ...}  (duplicates by hash)
            #   list [[entry, ...], [entry, ...]]                   (similar images/videos/music)
            all_groups = []
            if isinstance(data, dict):
                # Dict format: flatten all groups from all size buckets
                for size_key, groups_for_size in data.items():
                    if isinstance(groups_for_size, list):
                        for group in groups_for_size:
                            if isinstance(group, list) and len(group) > 0:
                                all_groups.append(group)
            elif isinstance(data, list):
                for item in data:
                    if isinstance(item, list) and len(item) > 0:
                        all_groups.append(item)

            for group_id, group in enumerate(all_groups):
                # Compute total size for the group header
                total_size = sum(e.get("size", 0) for e in group if isinstance(e, dict))
                header_text = (
                    f"Group {group_id + 1} ({len(group)} files, "
                    f"{self._format_size(total_size)})"
                )
                header = ResultEntry(
                    values={"__header": header_text},
                    header_row=True,
                    group_id=group_id,
                )
                results.append(header)
                for entry in group:
                    if isinstance(entry, dict):
                        result = self._parse_entry(entry, group_id)
                        results.append(result)
        else:
            # Flat tools return a list of entries
            if isinstance(data, list):
                for i, entry in enumerate(data):
                    if isinstance(entry, dict):
                        result = self._parse_entry(entry, 0)
                        results.append(result)
            elif isinstance(data, dict):
                # Some tools might also use dict format
                for key, entries in data.items():
                    if isinstance(entries, list):
                        for entry in entries:
                            if isinstance(entry, dict):
                                result = self._parse_entry(entry, 0)
                                results.append(result)

        return results

    def _parse_entry(self, entry: dict, group_id: int) -> ResultEntry:
        path = entry.get("path", "")
        p = Path(path)
        values = {
            "File Name": p.name,
            "Folder Name": p.name,
            "Symlink Name": p.name,
            "Path": str(p.parent),
            "Symlink Path": str(p.parent),
            "Size": self._format_size(entry.get("size", 0)),
            "Modification Date": self._format_date(entry.get("modified_date", 0)),
            "Hash": entry.get("hash", ""),
            "Similarity": str(entry.get("similarity", "")),
            "Resolution": entry.get("dimensions", ""),
            "Title": entry.get("title", ""),
            "Artist": entry.get("artist", ""),
            "Year": entry.get("year", ""),
            "Bitrate": str(entry.get("bitrate", "")),
            "Genre": entry.get("genre", ""),
            "Length": entry.get("length", ""),
            "Destination Path": entry.get("destination_path", ""),
            "Type of Error": entry.get("error_string", entry.get("type_of_error", "")),
            "Error Type": entry.get("error_string", entry.get("error_type", "")),
            "Current Extension": entry.get("current_extension", ""),
            "Proper Extension": entry.get("proper_extension", entry.get("proper_extensions", "")),
            "Codec": entry.get("codec", ""),
        }
        # Store full path for file operations
        values["__full_path"] = path
        values["__size_bytes"] = entry.get("size", 0)
        values["__modified_date_ts"] = entry.get("modified_date", 0)

        return ResultEntry(values=values, group_id=group_id)

    @staticmethod
    def _format_size(size_bytes: int) -> str:
        if size_bytes == 0:
            return "0 B"
        units = ["B", "KB", "MB", "GB", "TB"]
        i = 0
        size = float(size_bytes)
        while size >= 1024 and i < len(units) - 1:
            size /= 1024
            i += 1
        return f"{size:.1f} {units[i]}" if i > 0 else f"{int(size)} B"

    @staticmethod
    def _format_date(timestamp: int) -> str:
        if timestamp == 0:
            return ""
        from datetime import datetime
        try:
            return datetime.fromtimestamp(timestamp).strftime("%Y-%m-%d %H:%M:%S")
        except (ValueError, OSError):
            return str(timestamp)


class ScanRunner(QObject):
    """Manages scan execution in a background thread."""
    finished = Signal(object, list)
    progress = Signal(object)
    error = Signal(str)

    def __init__(self, parent=None):
        super().__init__(parent)
        self._thread: Optional[QThread] = None
        self._worker: Optional[ScanWorker] = None

    def start_scan(self, tab: ActiveTab, app_settings: AppSettings,
                   tool_settings: ToolSettings):
        if self._thread and self._thread.isRunning():
            return

        self._thread = QThread()
        self._worker = ScanWorker(tab, app_settings, tool_settings)
        self._worker.moveToThread(self._thread)

        self._thread.started.connect(self._worker.run)
        self._worker.finished.connect(self._on_finished)
        self._worker.progress.connect(self.progress.emit)
        self._worker.error.connect(self._on_error)

        self._thread.start()

    def stop_scan(self):
        if self._worker:
            self._worker.cancel()
            # Wait briefly for the thread to finish after killing the process
            if self._thread and self._thread.isRunning():
                self._thread.quit()
                if not self._thread.wait(5000):
                    self._thread.terminate()
                    self._thread.wait()
            self._thread = None
            self._worker = None
            # Emit an empty result to signal completion
            self.finished.emit(ActiveTab.DUPLICATE_FILES, [])

    def _on_finished(self, tab, results):
        self.finished.emit(tab, results)
        self._cleanup()

    def _on_error(self, msg):
        self.error.emit(msg)
        self._cleanup()

    def _cleanup(self):
        if self._thread:
            self._thread.quit()
            self._thread.wait(5000)
            self._thread = None
            self._worker = None


class FileOperations:
    """File operations: delete, move, hardlink, symlink, rename."""

    @staticmethod
    def delete_files(entries: list[ResultEntry], move_to_trash: bool = True,
                     dry_run: bool = False) -> tuple[int, list[str]]:
        deleted = 0
        errors = []
        for entry in entries:
            path = entry.values.get("__full_path", "")
            if not path:
                continue
            try:
                p = Path(path)
                if not p.exists():
                    errors.append(f"File not found: {path}")
                    continue
                if dry_run:
                    action = "trash" if move_to_trash else "delete"
                    errors.append(f"[DRY RUN] Would {action}: {path}")
                    deleted += 1
                    continue
                if move_to_trash:
                    try:
                        from send2trash import send2trash
                        send2trash(str(p))
                    except ImportError:
                        p.unlink()
                else:
                    if p.is_dir():
                        import shutil
                        shutil.rmtree(p)
                    else:
                        p.unlink()
                deleted += 1
            except OSError as e:
                errors.append(f"Error deleting {path}: {e}")
        return deleted, errors

    @staticmethod
    def move_files(entries: list[ResultEntry], destination: str,
                   preserve_structure: bool = False, copy_mode: bool = False,
                   dry_run: bool = False) -> tuple[int, list[str]]:
        import shutil
        moved = 0
        errors = []
        dest = Path(destination)
        if not dry_run:
            dest.mkdir(parents=True, exist_ok=True)

        for entry in entries:
            src_path = entry.values.get("__full_path", "")
            if not src_path:
                continue
            try:
                src = Path(src_path)
                if preserve_structure:
                    rel = src.parent
                    target_dir = dest / rel.relative_to(rel.anchor)
                    target = target_dir / src.name
                else:
                    target = dest / src.name

                if dry_run:
                    action = "copy" if copy_mode else "move"
                    errors.append(f"[DRY RUN] Would {action}: {src_path} -> {target}")
                    moved += 1
                    continue

                # Create dirs and handle name conflicts for real operations
                if preserve_structure:
                    target.parent.mkdir(parents=True, exist_ok=True)

                if target.exists():
                    stem = target.stem
                    suffix = target.suffix
                    counter = 1
                    while target.exists():
                        target = target.parent / f"{stem}_{counter}{suffix}"
                        counter += 1

                if copy_mode:
                    shutil.copy2(str(src), str(target))
                else:
                    shutil.move(str(src), str(target))
                moved += 1
            except OSError as e:
                errors.append(f"Error moving {src_path}: {e}")
        return moved, errors

    @staticmethod
    def create_hardlinks(entries: list[ResultEntry], reference_path: str) -> tuple[int, list[str]]:
        created = 0
        errors = []
        for entry in entries:
            path = entry.values.get("__full_path", "")
            if not path or path == reference_path:
                continue
            try:
                p = Path(path)
                p.unlink()
                os.link(reference_path, str(p))
                created += 1
            except OSError as e:
                errors.append(f"Error creating hardlink for {path}: {e}")
        return created, errors

    @staticmethod
    def create_symlinks(entries: list[ResultEntry], reference_path: str) -> tuple[int, list[str]]:
        created = 0
        errors = []
        for entry in entries:
            path = entry.values.get("__full_path", "")
            if not path or path == reference_path:
                continue
            try:
                p = Path(path)
                p.unlink()
                os.symlink(reference_path, str(p))
                created += 1
            except OSError as e:
                errors.append(f"Error creating symlink for {path}: {e}")
        return created, errors

    @staticmethod
    def fix_extensions(cli_path: str, app_settings, tool_settings) -> tuple[bool, str]:
        """Run czkawka_cli ext with -F flag to fix extensions."""
        cmd = [cli_path, "ext", "-F"]
        if app_settings.included_paths:
            cmd.extend(["-d", ",".join(app_settings.included_paths)])
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
            return True, result.stdout
        except Exception as e:
            return False, str(e)

    @staticmethod
    def fix_bad_names(cli_path: str, app_settings, tool_settings) -> tuple[bool, str]:
        """Run czkawka_cli bad-names with -F flag to fix names."""
        cmd = [cli_path, "bad-names", "-F"]
        if app_settings.included_paths:
            cmd.extend(["-d", ",".join(app_settings.included_paths)])
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
            return True, result.stdout
        except Exception as e:
            return False, str(e)

    @staticmethod
    def clean_exif(cli_path: str, entries: list[ResultEntry],
                   ignored_tags: str = "", overwrite: bool = True) -> tuple[int, list[str]]:
        """Remove EXIF data from selected files."""
        cleaned = 0
        errors = []
        for entry in entries:
            path = entry.values.get("__full_path", "")
            if not path:
                continue
            try:
                from PIL import Image
                img = Image.open(path)
                data = list(img.getdata())
                clean_img = Image.new(img.mode, img.size)
                clean_img.putdata(data)
                if overwrite:
                    clean_img.save(path)
                else:
                    p = Path(path)
                    new_path = p.parent / f"{p.stem}_clean{p.suffix}"
                    clean_img.save(str(new_path))
                cleaned += 1
            except Exception as e:
                errors.append(f"Error cleaning EXIF for {path}: {e}")
        return cleaned, errors
