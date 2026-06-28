# TODO

## Missing screenshots in instructions/

The files below contain `[[[Image: ...]]]` placeholders that need to be replaced
with actual screenshots before release.

---

### instructions/Instruction_Krokiet.md

| Line | Screenshot needed |
|------|------------------|
| 90 | Full Krokiet main window - annotated with 6 numbered callouts: (1) left tool panel, (2) top bar with Scan/Stop + progress, (3) included/excluded dirs panel, (4) main results list, (5) bottom action panel, (6) right image preview pane |
| 109 | Directory panel - included paths list (one regular + one reference path), excluded paths list, extensions field with "jpg,png", min/max size fields |
| 149 | Tool settings panel for Duplicate Files - check method dropdown (Hash), hash type dropdown (BLAKE3), min file size field, prehash cache toggle |
| 194 | Similar Images settings panel - similarity value 10, hash alg "Mean", hash size "16", resize filter "Lanczos3", geometric invariance off, exclude same size/resolution toggles |
| 208 | Similar Images results with right preview pane - two groups with header + file rows on left, thumbnail with resolution and path in right pane |
| 221 | Similar Videos results - two video groups, header "2 similar videos", file rows with filename/path/size/duration, thumbnail in right pane |
| 279 | Settings screen - General tab open: language dropdown, Light/Dark theme toggle, UI scale factor input, audio notifications toggle, tab bar (General / Performance) |

---

### instructions/Instruction_CLI.md

| Line | Screenshot needed |
|------|------------------|
| 76 | Terminal: `czkawka_cli --help` output - colorized, listing all subcommands (dup, empty-folders, big, empty-files, temp, image, music, symlinks, broken, video, ext, bad-names, video-optimizer, exif-remover) with one-line descriptions and EXAMPLES block |
| 186 | Terminal: `czkawka_cli dup --help` output - colorized, all flags with short forms, defaults, descriptions, EXAMPLE line at the bottom |
| 526 | Terminal: `czkawka_cli video-optimizer -d /home transcode -c h265,av1 -f results.txt` output - list of video paths with detected codec in parentheses, summary line at the end |
