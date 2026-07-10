# Changelog

## [0.1.1] — 2026-07-09

### Added
- `todo completion {bash|zsh|powershell|fish|elvish}` — shell completion scripts
- `todo tags` — list all tags with usage counts
- `todo search <query>` — search tasks by text
- `todo install` — copy binary to PATH (Windows)
- `todo --version` / `-V` — show version
- `skills/SKILL.md` — AI agent skill for coding assistants
- Landing page: Changelog link in nav and footer

### Changed
- Binary renamed from `todo-cli` to `todo` via `[[bin]]` in Cargo.toml
- Landing page: MSI installer download for Windows
- Landing page: fixed platform card asset matching (bug: all cards showed linux asset)
- CI: WiX MSI build step in release workflow

## [0.1.0] — 2026-07-07

### Added
- Initial release
- `todo init` — create a new TODO.md
- `todo add --task <description>` — add a task
- `todo add --actor <pseudo>` — add an actor
- `todo add --comment <text> --task-id <id>` — add a comment
- `todo list` — list all tasks
- `todo update <id>` — update any item
- `todo delete <id>` — delete any item (with cascade)
- `todo status <id> --set <status>` — change task status
- `todo dashboard` — launch web dashboard
- Dashboard with two view modes: list view and Kanban drag & drop
- Web dashboard has sidebar with task/actor/comment tabs
- 4-char alphanumeric auto-generated IDs
- Status tokens: `[ ]` todo, `[~]` en-cours, `[x]` done, `[B]` bloqued
