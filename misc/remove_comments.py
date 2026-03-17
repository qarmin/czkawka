#!/usr/bin/env python3

import os
import sys
from pathlib import Path

if len(sys.argv) < 2:
    print("Usage: python remove_comments.py <path>")
    sys.exit(1)

TARGET_DIR = Path(sys.argv[1]).resolve()


def remove_comments_from_text(s: str) -> str:
    lines = s.splitlines(keepends=True)
    # Pre-pass: drop full-line // comments (first non-whitespace is //)
    preprocessed = []
    for line in lines:
        stripped = line.lstrip()
        if stripped.startswith("//"):
            # keep newline if present (preserve line count)
            if line.endswith("\n"):
                preprocessed.append("\n")
            else:
                preprocessed.append("")
        else:
            preprocessed.append(line)
    lines = preprocessed

    out_lines = []
    block_depth = 0

    # helper to detect raw string start at position i in a line
    def raw_start_at(line: str, i: int) -> int | None:
        # match r#*" or br#*"
        if i < 0 or i >= len(line):
            return None
        j = i
        if line[j] == "b":
            j += 1
            if j >= len(line) or line[j] != "r":
                return None
        if line[j] != "r":
            return None
        j += 1
        k = j
        while k < len(line) and line[k] == "#":
            k += 1
        if k < len(line) and line[k] == '"':
            return k - j  # number of hashes
        return None

    for line in lines:
        i = 0
        n = len(line)
        if block_depth > 0:
            # we are inside a block comment that started earlier; scan for end
            new_line_parts = []
            while i < n:
                if line.startswith("/*", i):
                    block_depth += 1
                    i += 2
                elif line.startswith("*/", i):
                    block_depth -= 1
                    i += 2
                    if block_depth == 0:
                        # rest of line should be processed normally
                        new_line_parts.append(line[i:])
                        break
                else:
                    i += 1
            out_lines.append("".join(new_line_parts))
            continue

        out = []
        while i < n:
            ch = line[i]
            # detect block comment start
            if line.startswith("/*", i):
                block_depth = 1
                i += 2
                # consume until end of block (possibly multi-line)
                while i < n:
                    if line.startswith("/*", i):
                        block_depth += 1
                        i += 2
                    elif line.startswith("*/", i):
                        block_depth -= 1
                        i += 2
                        break
                    else:
                        i += 1
                if block_depth > 0:
                    # block continues to next lines; stop processing this line
                    break
                else:
                    continue
            # detect line comment
            if line.startswith("//", i):
                # drop rest of the line
                break
            # detect raw string
            raw = raw_start_at(line, i)
            if raw is not None:
                # copy raw string start
                start = i
                # find the closing " followed by same number of hashes
                hashes = raw
                # move i to the char after opening quote
                j = i
                if line[j] == "b":
                    j += 1
                j += 1  # skip 'r'
                while j < n and line[j] == "#":
                    j += 1
                # now j points at opening quote
                i = j + 1
                # scan until closing
                while True:
                    rest = line[i:]
                    idx = rest.find('"' + ("#" * hashes))
                    if idx != -1:
                        endpos = i + idx + 1 + hashes
                        out.append(line[start:endpos])
                        i = endpos
                        break
                    else:
                        out.append(line[start:])
                        i = n
                        break
                continue
            # detect normal string
            if ch == '"':
                out.append(ch)
                i += 1
                while i < n:
                    out.append(line[i])
                    if line[i] == "\\":
                        if i + 1 < n:
                            out.append(line[i + 1])
                            i += 2
                        else:
                            i += 1
                    elif line[i] == '"':
                        i += 1
                        break
                    else:
                        i += 1
                continue
            # detect char
            if ch == "'":
                out.append(ch)
                i += 1
                while i < n:
                    out.append(line[i])
                    if line[i] == "\\":
                        if i + 1 < n:
                            out.append(line[i + 1])
                            i += 2
                        else:
                            i += 1
                    elif line[i] == "'":
                        i += 1
                        break
                    else:
                        i += 1
                continue
            # default copy
            out.append(ch)
            i += 1
        out_lines.append("".join(out))
    return "".join(out_lines)


def process_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    new_text = remove_comments_from_text(text)
    if new_text != text:
        path.write_text(new_text, encoding="utf-8")
        print(f"Updated {path} (overwritten, no backup)")
        return True
    else:
        print(f"No changes: {path}")
        return False


def main() -> None:
    if not TARGET_DIR.exists():
        print("Target directory not found:", TARGET_DIR)
        sys.exit(1)
    rs_files = list(TARGET_DIR.rglob("*.rs"))
    if not rs_files:
        print("No .slint files found under", TARGET_DIR)
        sys.exit(0)
    changed = 0
    for f in rs_files:
        try:
            if process_file(f):
                changed += 1
        except Exception as e:
            print("ERROR processing", f, e)
    print(f"Completed. Files modified: {changed}/{len(rs_files)}")


if __name__ == "__main__":
    main()
