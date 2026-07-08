# Changelog

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
