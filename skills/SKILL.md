---
name: todo
description: >
  TODO CLI — portable terminal-based task manager for project TODO.md files.
  Manage tasks, actors, comments, tags, priorities, due dates.
  Single Rust binary, zero dependencies, Markdown storage.
---

# todo — TODO CLI Agent Skill

## Links

- **Source**: <https://github.com/RayanBO/todo>
- **Official site**: <https://rayanbo.github.io/todo/>
- **Documentation**: <https://rayanbo.github.io/todo/documentation/>
- **Creator**: <https://github.com/RayanBO>

## Install

### Windows (MSI)
```sh
# Download and run the installer
curl -LO https://github.com/RayanBO/todo/releases/latest/download/todo-x64.msi
todo-x64.msi
```

### Windows (exe)
```sh
curl -LO https://github.com/RayanBO/todo/releases/latest/download/todo-x64.exe
```

### macOS (ARM64)
```sh
curl -LO https://github.com/RayanBO/todo/releases/latest/download/todo-macos-arm64
chmod +x todo-macos-arm64
sudo mv todo-macos-arm64 /usr/local/bin/todo
```

### Linux (x64)
```sh
curl -LO https://github.com/RayanBO/todo/releases/latest/download/todo-linux-x64
chmod +x todo-linux-x64
sudo mv todo-linux-x64 /usr/local/bin/todo
```

### Cargo
```sh
cargo install todo-cli
```

## Quick Start

```bash
todo init
todo add --task "My task" --actors u0v1 --priority high
todo list
todo dashboard
```

### Auto-detect TODOs in source code
```bash
todo scan
```

Scans all source files for `TODO:` comments and adds them as tasks with code positions. Switch formats anytime:

```bash
todo cwi yaml   # use TODO.yaml instead of TODO.md
todo cwi md     # switch back to TODO.md
```

## Project State

A project lives in a single file — either `TODO.md` (Markdown) or `TODO.yaml` (YAML). Switch between formats with `todo cwi`. The file has three sections:

- `# Tasks` — array of task objects
- `# Actors` — array of actor objects
- `# Comments` — array of comment objects

### Task fields

| Field | Type | Description |
|---|---|---|---|
| `id` | string | 4-char alphanumeric (auto-generated) |
| `description` | string | Task description |
| `status` | "todo"\|"en-cours"\|"done"\|"bloqued" | Current status |
| `priority` | "low"\|"medium"\|"high"\|null | Priority level |
| `tags` | string[] | Labels |
| `actors` | string[] | Actor IDs assigned |
| `created` | datetime\|null | Creation timestamp |
| `due` | datetime\|null | Due date |
| `blocked_reason` | string\|null | Reason if blocked |
| `comments` | string[] | Comment IDs |
| `position` | string\|null | Code position URL (e.g. `file:///path/to/file#L42:10`) |

### Actor fields

| Field | Type |
|---|---|
| `id` | 4-char alphanumeric |
| `pseudo` | string |
| `pic` | string\|null |
| `actor_type` | "Human"\|"AgentIa"\|null |

### Comment fields

| Field | Type |
|---|---|
| `id` | 4-char alphanumeric |
| `text` | string |
| `task` | string (parent task ID) |
| `actors` | string[] |

## CLI Commands

### `todo init`
Initialize a new TODO file. Supports both Markdown and YAML formats.

```bash
todo init [--force] [--yaml] [--both]
```

- `--yaml` — create `TODO.yaml` instead of `TODO.md`
- `--both` — create both `TODO.md` and `TODO.yaml`
- `--force` — overwrite existing file(s)

### `todo add --task`
Create a task.

```bash
todo add --task <description> [--actors <ids>] [--tags <labels>] [--priority <level>] [--due <date>] [--position <url>]
```

- `--task` — description (required)
- `--actors` — comma-separated actor IDs
- `--tags` — comma-separated labels
- `--priority` — `low|medium|high`
- `--due` — `YYYY-MM-DD HH:MM`
- `--position` — code position URL (e.g. `file:///path/to/file#L42:10`)

### `todo add --actor`
Create an actor.

```bash
todo add --actor <pseudo> [--pic <url>]
```

### `todo add --comment`
Add a comment to a task.

```bash
todo add --comment <text> --task-id <id>
```

### `todo list`
List items.

```bash
todo list [--tasks] [--actors] [--comments] [--status <status>] [--search <query>] [--tag <labels>] [--priority <level>] [--overdue]
```

### `todo update`
Update any item by ID.

```bash
todo update <id> [--description <text>] [--due <date>] [--priority <level>] [--actors <ids>] [--tags <labels>] [--name <pseudo>] [--pic <url>] [--text <text>] [--position <url>]
```

- `--position` — change the code position URL

### `todo status`
Change task status.

```bash
todo status <id> --set <status> [--reason <text>]
```

- `--set` — `todo|en-cours|done|bloqued`
- `--reason` — required when setting to `bloqued`

### `todo delete`
Delete an item by ID. Cascades to all references.

```bash
todo delete <id>
```

### `todo scan`
Recursively scan source files for `TODO:` comments and add them as tasks with code positions.

```bash
todo scan
```

Scans all files (except `TODO.md`, `TODO.yaml`, `.todo/`, `.git/`, `node_modules/`, `target/`, binary files). Each found comment becomes a new task with its `position` set to the exact file location.

### `todo cwi`
Switch between Markdown and YAML format.

```bash
todo cwi [md|yaml]
```

With no argument, shows the current format. Requires the target format's file to exist.

### `todo tags`
List all tags with task counts.

```bash
todo tags
```

### `todo search`
Search tasks by query text.

```bash
todo search <query>
```

### `todo completion`
Generate shell completion scripts.

```bash
todo completion bash|zsh|powershell|fish|elvish
```

### `todo install`
Copy binary to PATH (Windows).

```bash
todo install
```

### `todo dashboard`
Start the web dashboard server.

```bash
todo dashboard
```

## HTTP API

The dashboard server exposes a JSON API at `/api` on `http://localhost:<port>` (default 8383):

| Method | Route | Purpose |
|---|---|---|
| GET | `/api/todo` | Full project data |
| GET | `/api/formats` | List available formats + current |
| POST | `/api/cwi` | Switch active format |
| POST | `/api/add-task` | Create task |
| POST | `/api/add-actor` | Create actor |
| POST | `/api/add-comment` | Create comment |
| POST | `/api/update` | Update any item |
| POST | `/api/delete` | Delete any item |
| POST | `/api/status` | Change task status |

## Dashboard UI

- **List view** — tasks grouped by status with detail panel and full CRUD, displays Position column with clickable links
- **Kanban view** — 4 columns (Todo, En cours, Done, Bloqued) with drag & drop, shows position links
- **Format switcher** — toggle between TODO.md and TODO.yaml from the header
- **Modals** — add/edit tasks, actors, and comments inline, including the Position field
- **Search & filter** — filter by actor, tag, priority, and status
- **Theme** — dark/light mode with system preference detection

## Agent Install

For AI coding agents (opencode, Cline, etc.), install this skill with one command:

```sh
npx skills add rayanbo/todo/skills
```

Or copy `skills/SKILL.md` to your agent's skills folder (e.g. `.opencode/skills/todo/SKILL.md`).
