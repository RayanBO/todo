# TODO CLI — Documentation

## Commands

### `init`

Create or overwrite TODO.md.

```
todo init [--force]
```

| Flag | Description |
|------|-------------|
| `--force` | Overwrite existing TODO.md without prompt |

If TODO.md doesn't exist → creates it.
If TODO.md exists with valid format → error unless `--force`.
If TODO.md exists with incompatible format → prompts `(Y/n)`.

---

### `add`

Add a task, actor, or comment.

**Task:**
```
todo add --task "<description>" [--actors "<id1>,<id2>"]
```

| Flag | Description |
|------|-------------|
| `--task` | Task description (required for task) |
| `--actors` | Comma-separated actor IDs to assign |

**Actor:**
```
todo add --actor "<pseudo>" [--pic "<url|path>"]
```

| Flag | Description |
|------|-------------|
| `--actor` | Actor pseudo (required for actor) |
| `--pic` | Picture URL or local path |

**Comment:**
```
todo add --comment "<text>" --task-id <id>
```

| Flag | Description |
|------|-------------|
| `--comment` | Comment text (required for comment) |
| `--task-id` | Task ID to attach comment to (required) |

---

### `list`

List items.

```
todo list [--tasks] [--actors] [--comments]
```

| Flag | Description |
|------|-------------|
| `--tasks` | Show only tasks |
| `--actors` | Show only actors |
| `--comments` | Show only comments |

Without flags → shows everything.

---

### `update`

Update fields of a task, actor, or comment by ID.

```
todo update <id> [--description "<text>"] [--due "<date>"] [--actors "<ids>"] [--comments "<ids>"] [--name "<pseudo>"] [--pic "<url>"] [--text "<text>"]
```

| Argument | Description |
|----------|-------------|
| `id` | ID of item to update (required) |

| Flag | Applies to | Description |
|------|-----------|-------------|
| `--description` | task | New description |
| `--due` | task | New due date (format: `YYYY-MM-DD HH:mm`) |
| `--actors` | task | Replace actor IDs (comma-separated) |
| `--comments` | task | Replace comment IDs (comma-separated) |
| `--name` | actor | New pseudo |
| `--pic` | actor | New picture URL/path |
| `--text` | comment | New comment text |

---

### `delete`

Delete a task, actor, or comment by ID.

```
todo delete <id>
```

When deleting an actor → auto-removes that actor ID from all tasks' actors lists.
When deleting a comment → auto-removes that comment ID from all tasks' comments lists.

---

### `status`

Change a task's status.

```
todo status <id> --set <status> [--reason "<text>"]
```

| Flag | Description |
|------|-------------|
| `--set` | New status (required) |
| `--reason` | Blocked reason (only when status is `bloqued`) |

Valid statuses: `todo`, `en-cours`, `done`, `bloqued`

---

## File format

### Tasks

```md
- [ ] a1B2 **-** Description text here
  - **Due**: 2026-12-31 23:59
  - **Actors**: u0v1,u0v2
  - **Comments**: cm01,cm02

- [B] 2b4D **-** Blocked task
  - **Due**: 2026-12-31 23:59
  - **Actors**: u0v1
  - **Blocked-reason**: "reason here"
```

Tasks without ID:

```md
- [ ] simple task without ID
  - **Due**: 2026-12-31 23:59
```

### Actors

```md
- u0v1
  - **Pseudo**: Builder
  - **Pic**: https://example.com/avatar.png
  - **Type**: Human
```

`Pic` is optional. `Type`: `Human` or `AgentIa`.

### Comments

```md
- cm01 **-** Comment text
  - **Actors**: u0v1
  - **Task**: 1b3D
```

`Task` is the ID of the task this comment belongs to.

---

## Status tokens

| Token | Status |
|---|---|
| `[ ]` | À faire (todo) |
| `[~]` | En cours |
| `[x]` | Fait (done) |
| `[B]` | Bloqued |

---

## ID generation

IDs are 4-character alphanumeric (a-z, A-Z, 0-9). Auto-generated on add.

## Parsing: ID split

Task line split via ` **-** `. If token after `- [ ] ` matches 4-char alphanumeric → parsed as ID. Otherwise → task without ID.

```
- [ ] a1B2 **-** description  → ID: a1B2
- [ ] description             → no ID
```
