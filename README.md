# todo — TODO CLI

**todo** is a portable CLI for managing project TODO.md files. Built with Rust, it ships as a single binary — no dependencies, no database, no cloud. When you need visuals, `todo dashboard` starts an embedded web server with two view modes: a classic list and a Kanban board with drag & drop.

🌐 Landing page: [https://rayanbo.github.io/todo-cli/](https://rayanbo.github.io/todo-cli/)

---

## Architecture

```
todo (Rust CLI)
  → TODO.md (tasks, actors, comments)
  → Rust HTTP server (TcpListener)
  → dashboard/index.html (static)
```

## Storage

Project data lives in a plain `TODO.md` file — version-control friendly, readable in any editor:

```
# Tasks

- [ ] a1b2 **Build login page**
    - **Actors**: u0v1
    - **Created**: 2026-07-07 12:00

# Actors

- u0v1 **Alice**
    - **Pseudo**: Alice
    - **Pic**: https://...
    - **Type**: Human

# Comments

- c1d2 **Comment**
    - **Text**: This is a comment
    - **Task**: a1b2
```

## CLI Commands

```bash
todo init [--force]
todo add --task "description" [--actors id1,id2]
todo add --actor "pseudo" [--pic url]
todo add --comment "text" --task-id id
todo list [--tasks] [--actors] [--comments]
todo update <id> [--description text] [--due date] [--name pseudo] [--text text]
todo delete <id>
todo status <id> --set <status> [--reason text]
todo dashboard
```

## Web Dashboard

- **List view** — classic sidebar with tasks grouped by status, detail panel, and CRUD
- **Kanban view** — 4 columns (Todo, En cours, Done, Bloqued) with drag & drop
- Add task, actor, comment modals
- Status changes, editing, and deletion
- Task detail with actors and comments

## HTTP API

| Route | Method | Purpose |
|---|---|---|
| `/api/todo` | GET | Full JSON data |
| `/api/add-task` | POST | Create task |
| `/api/add-actor` | POST | Create actor |
| `/api/add-comment` | POST | Create comment |
| `/api/update` | POST | Update any item |
| `/api/delete` | POST | Delete any item |
| `/api/status` | POST | Change task status |

## Build from Source

```powershell
cargo build
cargo test
```

Requires Rust (install via [rustup](https://rustup.rs/)).

## Notes

- IDs are 4-char alphanumeric, auto-generated
- Status tokens: `[ ]` todo, `[~]` en-cours, `[x]` done, `[B]` bloqued
- `todo delete` cascades: removing an actor/comment strips it from all tasks
- Dashboard auto-finds a free port starting at 8383
- Deleting an actor also removes them from all task assignments

Built by [Rayan Rav](https://rayan-rav.web.app/) · Open source under the [MIT License](LICENSE).
