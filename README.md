<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/status-active-4A6CF7?style=flat-square">
    <img src="https://img.shields.io/badge/status-active-4A6CF7?style=flat-square">
  </picture>
  <img src="https://img.shields.io/badge/Rust-1.75+-deploy?style=flat-square&logo=rust&color=4A6CF7">
  <img src="https://img.shields.io/badge/license-MIT-4A6CF7?style=flat-square">
</p>

<h1 align="center">todo</h1>
<p align="center">Portable CLI for project TODO.md files. Single binary, zero dependencies.</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#cli-commands">CLI</a> •
  <a href="#web-dashboard">Dashboard</a> •
  <a href="#http-api">API</a> •
  <a href="#build-from-source">Build</a>
</p>

---

## Overview

`todo` manages tasks, actors, and comments directly in a plain-text `TODO.md` file — no database, no cloud, no lock-in. Everything stays version-control friendly and editable in any text editor.

When you need a visual interface, `todo dashboard` spawns an embedded web server with a full-featured dashboard.

## Quick Start

```bash
# Initialize a new TODO.md
todo init

# Add your first task
todo add --task "Set up CI pipeline" --actors u0v1

# Add an actor
todo add --actor "Alice" --pic https://example.com/alice.png

# Open the web dashboard
todo dashboard
```

## Storage

All data lives in a single `TODO.md` file at the project root.

```
# Tasks

- [ ] a1b2 **Set up CI pipeline**
    - **Actors**: u0v1
    - **Created**: 2026-07-07 12:00

# Actors

- u0v1 **Alice**
    - **Pseudo**: Alice
    - **Pic**: https://...
    - **Type**: Human

# Comments

- c1d2 **Comment**
    - **Text**: Let's use GitHub Actions
    - **Task**: a1b2
```

## CLI Commands

| Command | Description |
|---|---|
| `todo init [--force]` | Create a new TODO.md (overwrite with `--force`) |
| `todo add --task <desc> [--actors ids]` | Create a task |
| `todo add --actor <pseudo> [--pic url]` | Create an actor |
| `todo add --comment <text> --task-id <id>` | Add a comment to a task |
| `todo list [--tasks] [--actors] [--comments]` | List items in the terminal |
| `todo update <id> [--description] [--due] [--name] [--text]` | Update any item |
| `todo delete <id>` | Delete an item (cascades to references) |
| `todo status <id> --set <status> [--reason]` | Change task status |
| `todo dashboard` | Start the web dashboard |

**Status tokens**: `[ ]` todo · `[~]` in progress · `[x]` done · `[B]` blocked

## Web Dashboard

Start it with `todo dashboard`, then open the provided URL in your browser.

- **List view** — tasks grouped by status with a detail panel, full CRUD
- **Kanban view** — 4 columns (Todo, In Progress, Done, Blocked) with drag & drop
- **Modals** — add/edit tasks, actors, and comments inline
- **Search & filter** — filter by actor, tag, priority, and status
- **Theme** — dark/light mode with system preference detection

## HTTP API

| Route | Method | Purpose |
|---|---|---|
| `/api/todo` | GET | Fetch full project data (JSON) |
| `/api/add-task` | POST | Create a task |
| `/api/add-actor` | POST | Create an actor |
| `/api/add-comment` | POST | Create a comment |
| `/api/update` | POST | Update any item |
| `/api/delete` | POST | Delete any item |
| `/api/status` | POST | Change task status |

## Build from Source

Requires [Rust](https://rustup.rs/) 1.75+.

```bash
cargo build --release
cargo test
```

The binary is placed at `target/release/todo.exe`.

---

## Architecture

```
todo (Rust CLI)
  → TODO.md (flat-file storage)
  → Rust HTTP server (TcpListener)
  → dashboard/index.html (static SPA)
```

- IDs are 4-character alphanumeric, auto-generated
- Deleting an actor or comment strips it from all tasks (cascading delete)
- Dashboard auto-finds a free port starting at 8383

---

<p align="center">
  Built by <a href="https://rayan-rav.web.app/">Rayan Rav</a> ·
  Forked from <a href="https://github.com/RayanBO/kanban.git">kanban</a> ·
  MIT License
</p>
