import datetime
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

CORE_MANIFEST = "czkawka_core/Cargo.toml"
CRATES = ["czkawka_core", "czkawka_cli", "czkawka_gui", "krokiet", "cedinia"]
METAINFOS = ["data/com.github.qarmin.czkawka.metainfo.xml", "data/io.github.qarmin.krokiet.metainfo.xml"]


@dataclass
class Rule:
    path: str
    pattern: str
    replacement: str


def format_red(text: str) -> str:
    return f"\033[91m{text}\033[0m"


def format_green(text: str) -> str:
    return f"\033[92m{text}\033[0m"


def format_yellow(text: str) -> str:
    return f"\033[93m{text}\033[0m"


def read_current_version(root: Path) -> str:
    match = re.search(r'^version = "([^"]+)"$', read_file(root, CORE_MANIFEST), re.MULTILINE)
    if match is None:
        sys.exit(format_red(f"Cannot find package version in {CORE_MANIFEST}"))
    return match.group(1)


def read_file(root: Path, path: str) -> str:
    file = root / path
    if not file.is_file():
        sys.exit(format_red(f"File {path} does not exist - update misc/change_version.py"))
    return file.read_text(encoding="utf-8")


def build_rules(old: str, new: str, iso_date: str) -> list[Rule]:
    esc = re.escape(old)
    rules = [
        Rule("cedinia/android/app/build.gradle.kts", f'versionName = "{esc}"', f'versionName = "{new}"'),
        Rule("cedinia/ui/screens/settings_screen.slint", f'"Cedinia {esc}"', f'"Cedinia {new}"'),
        Rule("krokiet/ui/main_window.slint", f'"Krokiet\\\\n{esc}"', f'"Krokiet\\\\n{new}"'),
        Rule("krokiet/ui/screens/about.slint", f'text: "{esc}";', f'text: "{new}";'),
        Rule("czkawka_gui/ui/about_dialog.ui", f'name="version">{esc}<', f'name="version">{new}<'),
        Rule("czkawka_gui/ui/main_window.ui", f"Czkawka {esc}<", f"Czkawka {new}<"),
        Rule("misc/cargo/PublishCore.sh", f'NUMBER="{esc}"', f'NUMBER="{new}"'),
        Rule("misc/cargo/PublishOther.sh", f'NUMBER="{esc}"', f'NUMBER="{new}"'),
        Rule(".github/ISSUE_TEMPLATE/bug_report.md", f"version: {esc},", f"version: {new},"),
        Rule(".github/ISSUE_TEMPLATE/bug_report.md", f"e.g. {esc} cli/gui", f"e.g. {new} cli/gui"),
        Rule("README.md", f"about the {esc} release", f"about the {new} release"),
    ]
    for crate in CRATES:
        manifest = f"{crate}/Cargo.toml"
        rules.append(Rule(manifest, f'^version = "{esc}"$', f'version = "{new}"'))
        if crate != "czkawka_core":
            rules.append(Rule(manifest, f'(czkawka_core = \\{{[^}}]*version = "){esc}"', f'\\g<1>{new}"'))
    for metainfo in METAINFOS:
        rules.append(
            Rule(metainfo, r'<release version="[^"]*" date="[^"]*"/>', f'<release version="{new}" date="{iso_date}"/>')
        )
    return rules


def apply_rules(root: Path, rules: list[Rule]) -> tuple[dict[str, str], list[str]]:
    contents = {rule.path: read_file(root, rule.path) for rule in rules}
    errors = []
    for rule in rules:
        updated, replaced = re.subn(rule.pattern, rule.replacement, contents[rule.path], flags=re.MULTILINE)
        if replaced != 1:
            errors.append(f"{rule.path}: pattern `{rule.pattern}` matched {replaced} times, expected 1")
        contents[rule.path] = updated
    return contents, errors


def refresh_lockfile(root: Path) -> None:
    if subprocess.run(["cargo", "update", "--workspace"], cwd=root).returncode != 0:
        sys.exit(format_red("`cargo update --workspace` failed - Cargo.lock still holds the old versions"))


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(format_red("Usage: python change_version.py <new_version>"))
    new_version = sys.argv[1]
    if re.fullmatch(r"\d+\.\d+\.\d+", new_version) is None:
        sys.exit(format_red(f"Version `{new_version}` is not in the `major.minor.patch` format"))

    root = Path(__file__).resolve().parent.parent
    old_version = read_current_version(root)
    if old_version == new_version:
        sys.exit(format_red(f"Version {new_version} is already set in {CORE_MANIFEST}"))

    iso_date = datetime.date.today().isoformat()
    contents, errors = apply_rules(root, build_rules(old_version, new_version, iso_date))
    if errors:
        print(format_red("Version was not changed, because some files do not look as expected:"))
        for error in errors:
            print(format_red(f"  - {error}"))
        sys.exit(1)

    for path, content in contents.items():
        (root / path).write_text(content, encoding="utf-8")
    refresh_lockfile(root)

    print(format_green(f"Changed version {old_version} -> {new_version} ({iso_date}) in {len(contents)} files"))
    print(format_yellow("Remaining manual steps:"))
    print(format_yellow("  - add the Changelog.md entry with the release date"))
    print(format_yellow("  - update the release article links in README.md"))
    print(format_yellow("  - regenerate cedinia/THIRD_PARTY_LICENSES.txt with `just gen_cedinia_licenses`"))
    print(format_yellow("  - bump `versionCode` in cedinia/android/app/build.gradle.kts before a Play Store upload"))


main()
