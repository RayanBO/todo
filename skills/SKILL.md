---
name: todo
description: >
  TODO CLI — portable terminal-based task manager for project TODO.md files.
  Manage tasks, actors, comments, tags, priorities, due dates.
  Single Rust binary, zero dependencies, Markdown storage.
---

# todo — TODO CLI Agent Skill

## Quick Start

```bash
todo init
todo add --task "My task" --actors u0v1 --priority high
todo list
todo dashboard
```

## Project State

A project lives in a single `TODO.md` file. The file has three sections:

- `# Tasks` — array of task objects
- `# Actors` — array of actor objects
- `# Comments` — array of comment objects

### Task fields

| Field | Type | Description |
|---|---|---|
| `id` | string | 4-char alphanumeric (auto-generated) |
| `description` | string | Task description |
| `status` | "todo"\|"en-cours"\|"done"\|"bloqued" | Current status |
| `priority` | "low"\|"medium"\|"high"\|null | Priority level |
| `tags` | string[] | Labels |
| `actors` | string[] | Actor IDs assigned |
| `created_at` | datetime | Creation timestamp |
| `due` | datetime\|null | Due date |
| `blocked_reason` | string\|null | Reason if blocked |
| `comments` | string[] | Comment IDs |

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
Initialize a new TODO.md file.

```bash
todo init [--force]
```

### `todo add --task`
Create a task.

```bash
todo add --task <description> [--actors <ids>] [--tags <labels>] [--priority <level>] [--due <date>]
```

- `--task` — description (required)
- `--actors` — comma-separated actor IDs
- `--tags` — comma-separated labels
- `--priority` — `low|medium|high`
- `--due` — `YYYY-MM-DD HH:MM`

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
todo update <id> [--description <text>] [--due <date>] [--priority <level>] [--actors <ids>] [--tags <labels>] [--name <pseudo>] [--pic <url>] [--text <text>]
```

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
| POST | `/api/add-task` | Create task |
| POST | `/api/add-actor` | Create actor |
| POST | `/api/add-comment` | Create comment |
| POST | `/api/update` | Update any item |
| POST | `/api/delete` | Delete any item |
| POST | `/api/status` | Change task status |

## Dashboard UI

- **List view** — tasks grouped by status with detail panel and full CRUD
- **Kanban view** — 4 columns (Todo, En cours, Done, Bloqued) with drag & drop
- **Modals** — add/edit tasks, actors, and comments inline
- **Search & filter** — filter by actor, tag, priority, and status
- **Theme** — dark/light mode with system preference detection

## Install via Skills

```sh
npx skills add rayanbo/todo/skills
```

Or copy this SKILL.md to your agent's skills folder (e.g. `.opencode/skills/todo/SKILL.md`).
