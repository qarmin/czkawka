# Contributing to Kalka

Thanks for your interest in contributing to Kalka! This guide will help you get started.

---

## Branching Model

> **`master`** is the single source-of-truth branch.
>
> **How contributors should work:**
> 1. Fork the repository
> 2. Create a `feat/*` or `fix/*` branch from `master`
> 3. Open a PR targeting `master`

## First-Time Contributors

Welcome — contributions of all sizes are valued. If this is your first contribution, here is how to get started:

1. **Find an issue.** Look for issues labeled `good first issue` — these are scoped for newcomers and include context to get moving quickly.

2. **Pick a scope.** Good first contributions include:
   - Typo and documentation fixes
   - Translation improvements (see [Adding Translations](#adding-translations))
   - Test additions or improvements
   - Small bug fixes with clear reproduction steps

3. **Follow the fork → branch → change → test → PR workflow:**
   - Fork the repository and clone your fork
   - Create a feature branch (`git checkout -b feat/my-change` or `git checkout -b fix/my-change`)
   - Make your changes and run the quality checks (see [Development Setup](#development-setup))
   - Open a PR against `master`

4. **Start with Track A.** Kalka uses three [collaboration tracks](#collaboration-tracks-risk-based) (A/B/C) based on risk. First-time contributors should target **Track A** (docs, tests, translations) — these require lighter review and are the fastest path to a merged PR.

If you get stuck, open a draft PR early and ask questions in the description.

## Development Setup

### Prerequisites

- Python 3.10+
- PySide6 >= 6.6.0
- `czkawka_cli` binary (built from the project root or installed via `cargo install`)

### Getting Started

```bash
# Clone the repo
git clone https://github.com/qarmin/czkawka.git
cd czkawka/kalka

# Create a virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Build czkawka_cli (from the project root)
cd ..
cargo build --release -p czkawka_cli
cd kalka

# Run the application
python main.py

# Run tests
python -m pytest tests/ -v

# Format & lint (required before PR)
python -m ruff check app/ --fix
python -m ruff format app/
```

### Pre-commit Checks

Before submitting a PR, ensure:

```bash
# Syntax check all Python files
python -c "import ast, glob; [ast.parse(open(f).read()) for f in glob.glob('app/**/*.py', recursive=True)]; print('OK')"

# Verify translation keys match across all locales
diff <(grep -oP '^[a-z][-a-z0-9]+' i18n/en/kalka.ftl | sort) <(grep -oP '^[a-z][-a-z0-9]+' i18n/pl/kalka.ftl | sort)

# Run the app to verify no import errors
python -c "from app.localizer import tr, init; init(); print('i18n OK')"
```

## Project Architecture

```
kalka/
├── main.py                    # Entry point
├── requirements.txt           # Python dependencies
├── i18n.toml                  # Fluent i18n configuration
├── i18n/                      # Translation files (Fluent .ftl format)
│   ├── en/kalka.ftl           # English (fallback)
│   ├── pl/kalka.ftl           # Polish
│   └── .../kalka.ftl          # Other locales
├── icons/                     # Application icons
├── app/
│   ├── localizer.py           # i18n infrastructure (Fluent)
│   ├── main_window.py         # Main window with all panels
│   ├── left_panel.py          # Tool selection sidebar (14 tools)
│   ├── results_view.py        # Results tree with grouping, selection, sorting
│   ├── action_buttons.py      # Scan/Stop/Delete/Move/Save/Sort buttons
│   ├── tool_settings.py       # Per-tool settings (9 tool panels)
│   ├── settings_panel.py      # Global settings (General/Directories/Filters/Preview)
│   ├── progress_widget.py     # Two-bar progress: current stage + overall
│   ├── preview_panel.py       # Image preview panel
│   ├── bottom_panel.py        # Directory management + error display
│   ├── backend.py             # CLI subprocess interface with JSON progress parsing
│   ├── models.py              # Data models, enums, column definitions
│   ├── state.py               # Application state with Qt signals
│   ├── icons.py               # SVG icon resources from Krokiet icon set
│   └── dialogs/               # Delete, Move, Select, Sort, Save, Rename, About
└── tests/                     # Test files
```

### How It Works

Kalka is a **PySide6/Qt 6 GUI frontend** that uses `czkawka_cli` as its backend:

1. **Scanning**: Spawns `czkawka_cli` as a subprocess with `--compact-file-to-save` for JSON results and `--json-progress` for real-time progress data on stderr.
2. **Progress**: JSON lines on stderr provide stage index, entry counts, byte counts — displayed as two progress bars (current stage and overall).
3. **Results**: JSON results are parsed and displayed in a tree view with group headers for duplicate/similar file tools.
4. **File operations**: Delete, move, hardlink, symlink, and rename are performed directly in Python. EXIF cleaning and extension/name fixing use `czkawka_cli` subcommands.

## Adding Translations

Kalka uses [Project Fluent](https://projectfluent.org/) (.ftl files) for internationalization, matching the same format as krokiet.

### Adding a New Language

1. Create a new directory under `i18n/` with the locale code (e.g., `i18n/de/`)
2. Copy `i18n/en/kalka.ftl` as a starting point
3. Translate all message values (keep the message IDs unchanged)
4. Test by running: `LANG=de_DE.UTF-8 python main.py`

### Translation Guidelines

- **Keep message IDs unchanged** — only translate the values after `=`
- **Preserve placeholders** — `{ $count }`, `{ $name }`, etc. must remain in the translation
- **Match the English file's key set** — every key in `en/kalka.ftl` must exist in your translation
- **Use natural phrasing** — don't translate word-for-word; adapt to how the target language naturally expresses the concept
- **Test your translation** — run the app with your locale to verify layout doesn't break with longer strings

### Translation File Format

```fluent
# Simple string
scan-button = Skanuj

# String with placeholder
status-scan-complete = Skanowanie zakończone: znaleziono { $count } pozycji

# Multiline string
about-description =
    Kalka to prosty, szybki i darmowy program do usuwania
    zbędnych plików z komputera.
```

### Using Translations in Code

All user-visible strings must use the `tr()` function:

```python
from .localizer import tr

# Simple string
label.setText(tr("scan-button"))

# String with parameters
status.setText(tr("status-scan-complete", count=42))
```

## Collaboration Tracks (Risk-Based)

To keep review throughput high without lowering quality, every PR should map to one track:

| Track | Typical scope | Required review depth |
|---|---|---|
| **Track A (Low risk)** | docs, translations, tests, isolated refactors | 1 maintainer review + CI green |
| **Track B (Medium risk)** | UI behavior changes, backend parsing, settings, new tool support | 1 subsystem-aware review + validation evidence |
| **Track C (High risk)** | CLI interface changes, file operations (delete/move/hardlink), i18n infrastructure, `czkawka_core` changes | 2-pass review, rollback plan required |

When in doubt, choose the higher track.

## PR Definition of Ready (DoR)

Before requesting review, ensure all of the following are true:

- Scope is focused to a single concern.
- Relevant local validation has been run (syntax check, lint, manual testing).
- No personal/sensitive data is introduced in code/docs/tests.
- If translations were changed, all locale files have matching key sets.
- Linked issue (or rationale for no issue) is included.

## PR Definition of Done (DoD)

A PR is merge-ready when:

- CI is green.
- Required reviewers approved.
- User-visible behavior changes are documented.
- Follow-up TODOs are explicit and tracked in issues.
- Translation files are consistent across all locales.

## Code Style

- **Python 3.10+ features** — use type hints, dataclasses, match statements where appropriate
- **PySide6 patterns** — signals/slots, Qt naming conventions for UI code
- **Minimal dependencies** — every package adds installation complexity
- **Translatable strings** — all user-visible text must go through `tr()` from `localizer.py`
- **No hardcoded paths** — use `Path` objects and relative references
- **Security by default** — never execute user-provided strings as code; validate file paths before operations

## Code Naming Conventions

- **Python casing**: modules/files `snake_case`, classes `PascalCase`, functions/variables `snake_case`, constants `SCREAMING_SNAKE_CASE`
- **Qt widgets**: prefix private widgets with `_` (e.g., `self._scan_btn`)
- **Translation keys**: `kebab-case` with section prefixes (e.g., `settings-cli-path`, `delete-dialog-title`)
- **Fluent message IDs**: match krokiet conventions where possible for cross-project consistency

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add PDF preview support
feat(i18n): add German translation
fix: handle exit code 2 from czkawka_cli gracefully
docs: update contributing guide
test: add backend JSON parsing tests
refactor: extract progress formatting to utility
chore: bump PySide6 to 6.8.0
```

Recommended scope keys:

- `i18n`, `ui`, `backend`, `settings`, `dialogs`, `preview`, `results`, `docs`, `tests`

## Reporting Issues

- **Bugs**: Include OS, Python version, PySide6 version, `czkawka_cli` version, steps to reproduce, expected vs actual behavior
- **Features**: Describe the use case and which component would be affected
- **Translations**: Note the language, the incorrect string, and the suggested correction

## Agent Collaboration Guidance

Agent-assisted contributions are welcome and treated as first-class contributions.

For smoother review:

- Keep PR summaries concrete (problem, change, non-goals).
- Include reproducible validation evidence (syntax check, manual testing screenshots).
- Agent-assisted PRs are welcome, but contributors remain accountable for understanding what the code does.
- Call out uncertainty and risky edges explicitly.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
