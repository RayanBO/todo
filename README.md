<div align="center">

# todo

**A portable, plain-text task manager for your projects — one Rust binary, no database, no cloud.**

Tasks live in a human-readable `TODO.md` you can commit alongside your code. When you want visuals, `todo dashboard` spins up an embedded web server with a list view and a drag-and-drop Kanban board.

[![Rust](https://img.shields.io/badge/Rust-2024-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)](#installation)
[![Made with clap](https://img.shields.io/badge/CLI-clap%204-orange)](https://docs.rs/clap)

🌐 [Landing page](https://rayanbo.github.io/todo-cli/) · 📖 [Full documentation](DOCUMENTATIONS.md)

</div>

---

## Why todo

- **Single binary** — built in Rust, ships with zero runtime dependencies.
- **Plain-text storage** — data is a `TODO.md` file: diff-friendly, reviewable in any editor, versioned with your repo.
- **Works from anywhere in the project** — commands walk up parent directories to find `TODO.md`, git-style.
- **Rich tasks** — priorities, due dates, tags, actors, comments, and blocked reasons.
- **Built-in web dashboard** — list view + Kanban drag & drop, no external server or Node toolchain.
- **Safe by design** — concurrent dashboard requests are serialized and writes are atomic (temp + rename), so the file never corrupts or loses updates.

---

## Installation

### Windows (build + auto-install)

```powershell
git clone https://github.com/RayanBO/todo.git
cd todo
./install.ps1
```

This builds a release binary, copies it to `%LOCALAPPDATA%\Programs\todo\todo.exe`, and adds that folder to your user `PATH`. Restart your terminal, then run `todo`.

### From source (any platform)

```bash
git clone https://github.com/RayanBO/todo.git
cd todo
cargo build --release
```

The binary is at `target/release/todo-cli`. Requires the Rust toolchain — install via [rustup](https://rustup.rs/). Auto-install into `PATH` is Windows-only; on macOS/Linux, copy the binary onto your `PATH` manually.

---

## Quick start

```bash
todo init                                  # create TODO.md in the current directory
todo add --task "Build the login page" --priority high --tag frontend,auth
todo add --actor "Alice" --pic https://example.com/alice.png
todo status <task-id> --set en-cours
todo list                                  # show everything
todo dashboard                             # open the web UI
```

---

## Commands

| Command | Description |
|---|---|
| `todo init [--force]` | Create `TODO.md` (prompts / errors if one exists; `--force` overwrites) |
| `todo add --task "<desc>"` | Add a task — options: `--tag a,b` `--actors id1,id2` `--priority low\|medium\|high` `--due "YYYY-MM-DD HH:MM"` |
| `todo add --actor "<pseudo>" [--pic <url\|path>]` | Add an actor |
| `todo add --comment "<text>" --task-id <id>` | Add a comment to a task |
| `todo list [filters]` | List items — filters: `--tasks` `--actors` `--comments` `--tag a,b` `--priority <p>` `--search <q>` `--overdue` |
| `todo update <id> [fields]` | Update a task, actor, or comment by ID |
| `todo delete <id>` | Delete an item (cascades: removes the ID from every task) |
| `todo status <id> --set <status> [--reason "<text>"]` | Change task status |
| `todo search <query>` | Search tasks by description, tag, or actor |
| `todo tags` | List all tags with task counts |
| `todo dashboard` | Launch the web dashboard |
| `todo completion <shell>` | Generate shell completion scripts |
| `todo install` | Copy the binary into `PATH` (Windows) |

Valid statuses: `todo` (`a-faire`), `en-cours` (`in-progress`), `done` (`fait`), `bloqued` (`blocked`).

See [DOCUMENTATIONS.md](DOCUMENTATIONS.md) for every flag and field.

---

## File format

`TODO.md` has three sections — `# Tasks`, `# Actors`, `# Comments`:

```md
# Tasks

- [~] a1B2 **-** Build the login page
  - **Due**: 2026-12-31 23:59
  - **Actors**: u0v1,u0v2
  - **Priority**: high
  - **Created**: 2026-07-07 12:00
  - **Tags**: frontend, auth

- [B] 2b4D **-** Wire up OAuth
  - **Blocked-reason**: "waiting on API keys"

# Actors

- u0v1
  - **Pseudo**: Alice
  - **Pic**: https://example.com/alice.png
  - **Type**: Human

# Comments

- cm01 **-** Started working on it
  - **Actors**: u0v1
  - **Task**: a1B2
```

- **IDs** are 4-character alphanumeric, auto-generated and guaranteed unique across tasks, actors, and comments.
- **Status tokens:** `[ ]` todo · `[~]` en-cours · `[x]` done · `[B]` bloqued.
- **Actor types:** `Human` or `AgentIa`.

---

## Web dashboard

```bash
todo dashboard
```

Serves on `http://127.0.0.1:8383` (auto-picks the next free port up to 8483). Run it from the project root, where the `dashboard/` assets live.

- **List view** — sidebar grouped by status, detail panel, full CRUD.
- **Kanban view** — four columns (Todo, En cours, Done, Bloqued) with drag & drop.
- **Modals** to add tasks, actors, and comments; edit and delete inline.
- **Avatar upload** — drop an image; it's stored under `dashboard/pics/`.

### HTTP API

| Route | Method | Purpose |
|---|---|---|
| `/api/todo` | GET | Full data as JSON |
| `/api/pic?path=<path>` | GET | Serve an image (image files only) |
| `/api/upload` | POST | Upload an avatar image |
| `/api/add-task` | POST | Create a task |
| `/api/add-actor` | POST | Create an actor |
| `/api/add-comment` | POST | Create a comment |
| `/api/update` | POST | Update a task/comment |
| `/api/update-actor` | POST | Update an actor |
| `/api/delete`, `/api/delete-actor` | POST | Delete an item |
| `/api/status` | POST | Change a task's status |

The dashboard binds to `127.0.0.1` only and has no authentication — it is intended for local single-user use.

---

## Development

```bash
cargo build      # debug build
cargo test       # run the integration suite
cargo run -- list
```

Project layout:

```
src/
  main.rs         CLI definitions (clap) and dispatch
  commands.rs     business logic: init / add / list / update / delete / status / install
  models.rs       Task, Actor, Comment, TodoFile + enums
  parser.rs       TODO.md → structs
  serializer.rs   structs → TODO.md
  id_gen.rs       unique 4-char ID generation
  tags.rs         tag normalization and matching
  dashboard.rs    embedded HTTP server + JSON API
dashboard/        static web UI (HTML/CSS/JS)
tests/            integration tests
```

---

## Credits

- CLI, storage engine, and HTTP server by [Rayan Rav](https://rayan-rav.web.app/).
- The web dashboard UI is based on [RayanBO/kanban](https://github.com/RayanBO/kanban).

## License

Open source under the [MIT License](LICENSE).
