use crate::id_gen::generate_id;
use crate::models::*;
use crate::parser::parse_todo;
use crate::serializer::serialize_todo;
use crate::tags;
use chrono::NaiveDateTime;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

const TODO_FILE: &str = "./TODO.md";

fn read_todo() -> Result<TodoFile, String> {
    if !Path::new(TODO_FILE).exists() {
        return Err("TODO.md not found. Run `todo init` first.".to_string());
    }
    let content = fs::read_to_string(TODO_FILE).map_err(|e| format!("Read error: {}", e))?;
    parse_todo(&content)
}

fn write_todo(todo: &TodoFile) -> Result<(), String> {
    let content = serialize_todo(todo);
    fs::write(TODO_FILE, content).map_err(|e| format!("Write error: {}", e))
}

fn is_valid_format(content: &str) -> bool {
    content.contains("# Tasks") || content.contains("# Actors") || content.contains("# Comments")
}

fn prompt_overwrite() -> Result<bool, String> {
    print!("TODO.md exists but format incompatible. Overwrite? (Y/n): ");
    stdout().flush().map_err(|e| format!("Flush error: {}", e))?;
    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| format!("Read error: {}", e))?;
    let input = input.trim().to_lowercase();
    Ok(input != "n" && input != "no")
}

pub fn init(force: bool) -> Result<(), String> {
    if !Path::new(TODO_FILE).exists() {
        let todo = TodoFile::empty();
        write_todo(&todo)?;
        println!("Created TODO.md");
        return Ok(());
    }

    let content = fs::read_to_string(TODO_FILE).map_err(|e| format!("Read error: {}", e))?;

    if is_valid_format(&content) {
        if force {
            let todo = TodoFile::empty();
            write_todo(&todo)?;
            println!("Overwritten TODO.md");
            return Ok(());
        }
        return Err("TODO.md exists with valid format. Use --force to overwrite.".to_string());
    }

    if force || prompt_overwrite()? {
        let todo = TodoFile::empty();
        write_todo(&todo)?;
        println!("Overwritten TODO.md");
    } else {
        println!("Aborted.");
    }
    Ok(())
}

pub fn add_task(description: &str, actor_ids: &[String], tag_list: &[String], priority: Option<Priority>, due: Option<NaiveDateTime>, status: Option<TaskStatus>) -> Result<(), String> {
    let mut todo = read_todo()?;
    let id = generate_id();
    let task = Task {
        id: Some(id.clone()),
        status: status.unwrap_or(TaskStatus::Todo),
        description: description.to_string(),
        due,
        actors: actor_ids.to_vec(),
        comments: Vec::new(),
        blocked_reason: None,
        tags: tags::normalize_tags(tag_list),
        priority,
        created: Some(chrono::Local::now().naive_local()),
    };
    todo.tasks.push(task);
    write_todo(&todo)?;
    println!("Added task {}", id);
    Ok(())
}

pub fn add_actor(pseudo: Option<&str>, pic: Option<&str>) -> Result<(), String> {
    let mut todo = read_todo()?;
    let id = generate_id();
    let actor = Actor {
        id: id.clone(),
        pseudo: pseudo.map(|s| s.to_string()),
        pic: pic.map(|s| s.to_string()),
        actor_type: ActorType::Human,
    };
    todo.actors.push(actor);
    write_todo(&todo)?;
    println!("Added actor {}", id);
    Ok(())
}

pub fn add_comment(text: &str, task_id: &str, actor_ids: &[String]) -> Result<(), String> {
    let mut todo = read_todo()?;

    if !todo.tasks.iter().any(|t| t.id.as_deref() == Some(task_id)) {
        return Err(format!("Task ID {} not found.", task_id));
    }

    let id = generate_id();
    let comment = Comment {
        id: id.clone(),
        text: text.to_string(),
        actors: actor_ids.to_vec(),
        task_id: Some(task_id.to_string()),
    };
    todo.comments.push(comment);

    for task in &mut todo.tasks {
        if task.id.as_deref() == Some(task_id) {
            task.comments.push(id.clone());
        }
    }

    write_todo(&todo)?;
    println!("Added comment {} for task {}", id, task_id);
    Ok(())
}

pub fn list(tasks: bool, actors: bool, comments: bool, tag_filter: &[String], priority_filter: Option<Priority>, search_query: Option<&str>, overdue: bool) -> Result<(), String> {
    let todo = read_todo()?;

    let show_tasks = !tasks && !actors && !comments || tasks;
    let show_actors = !tasks && !actors && !comments || actors;
    let show_comments = !tasks && !actors && !comments || comments;

    if show_tasks {
        let now = chrono::Local::now().naive_local();
        let filtered: Vec<&Task> = todo.tasks.iter().filter(|t| {
            if !tag_filter.is_empty() && !tags::tags_match(&t.tags, tag_filter) {
                return false;
            }
            if let Some(ref pf) = priority_filter {
                if t.priority.as_ref() != Some(pf) {
                    return false;
                }
            }
            if let Some(query) = search_query {
                let q = query.to_lowercase();
                let desc_match = t.description.to_lowercase().contains(&q);
                let tags_match = t.tags.iter().any(|tag| tag.contains(&q));
                let actors_match = t.actors.iter().any(|a| a.to_lowercase().contains(&q));
                if !desc_match && !tags_match && !actors_match {
                    return false;
                }
            }
            if overdue {
                match t.due {
                    Some(due) if due < now => {}
                    _ => return false,
                }
            }
            true
        }).collect();
        println!("# Tasks");
        for task in &filtered {
            let id_display = task.id.as_deref().unwrap_or("-");
            let priority_str = task.priority.as_ref().map(|p| format!(" [{}]", p.as_str())).unwrap_or_default();
            println!("  [{}]{} {} — {}", task.status.token(), priority_str, id_display, task.description);
            if let Some(created) = &task.created {
                println!("      Created: {}", created.format("%Y-%m-%d %H:%M"));
            }
            if let Some(due) = &task.due {
                println!("      Due: {}", due.format("%Y-%m-%d %H:%M"));
            }
            if !task.actors.is_empty() {
                println!("      Actors: {}", task.actors.join(", "));
            }
            if !task.tags.is_empty() {
                println!("      Tags: {}", task.tags.join(", "));
            }
            if let Some(p) = &task.priority {
                println!("      Priority: {}", p);
            }
            if !task.comments.is_empty() {
                println!("      Comments: {}", task.comments.join(", "));
            }
            if let Some(reason) = &task.blocked_reason {
                println!("      Blocked: {}", reason);
            }
        }
    }

    if show_actors {
        println!("\n# Actors");
        for actor in &todo.actors {
            let pseudo = actor.pseudo.as_deref().unwrap_or("-");
            let pic = actor.pic.as_deref().unwrap_or("");
            if !pic.is_empty() {
                println!("  {} ({}) pic={} type: {}", actor.id, pseudo, pic, actor.actor_type.as_str());
            } else {
                println!("  {} ({}) type: {}", actor.id, pseudo, actor.actor_type.as_str());
            }
        }
    }

    if show_comments {
        println!("\n# Comments");
        for comment in &todo.comments {
            let task_info = comment.task_id.as_deref().map(|id| format!(" [task:{}]", id)).unwrap_or_default();
            println!("  {}: {}{}", comment.id, comment.text, task_info);
            if !comment.actors.is_empty() {
                println!("      Actors: {}", comment.actors.join(", "));
            }
        }
    }

    Ok(())
}

pub fn update(id: &str, description: Option<&str>, due: Option<&str>, name: Option<&str>, text: Option<&str>, actors: Option<&str>, comments: Option<&str>, pic: Option<&str>, tags: Option<&str>, priority: Option<&str>, blocked_reason: Option<&str>, actor_type: Option<&str>) -> Result<(), String> {
    let mut todo = read_todo()?;
    let mut found = false;

    for task in &mut todo.tasks {
        if task.id.as_deref() == Some(id) {
            if let Some(desc) = description {
                task.description = desc.to_string();
            }
            if let Some(d) = due {
                task.due = NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M").ok();
            }
            if let Some(a) = actors {
                task.actors = a.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            if let Some(c) = comments {
                task.comments = c.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            if let Some(t) = tags {
                let raw: Vec<String> = t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                task.tags = crate::tags::normalize_tags(&raw);
            }
            if let Some(p) = priority {
                task.priority = Priority::from_str(p);
                if task.priority.is_none() {
                    return Err(format!("Invalid priority '{}'. Use: low, medium, high", p));
                }
            }
            if let Some(r) = blocked_reason {
                let trimmed = r.trim();
                task.blocked_reason = if trimmed.is_empty() { None } else { Some(trimmed.to_string()) };
            }
            found = true;
        }
    }

    for actor in &mut todo.actors {
        if actor.id == id {
            if let Some(n) = name {
                actor.pseudo = Some(n.to_string());
            }
            if let Some(p) = pic {
                actor.pic = Some(p.to_string());
            }
            if let Some(t) = actor_type {
                if let Some(at) = ActorType::from_str(t) {
                    actor.actor_type = at;
                }
            }
            found = true;
        }
    }

    for comment in &mut todo.comments {
        if comment.id == id {
            if let Some(t) = text {
                comment.text = t.to_string();
            }
            found = true;
        }
    }

    if !found {
        return Err(format!("ID {} not found.", id));
    }

    write_todo(&todo)?;
    println!("Updated {}", id);
    Ok(())
}

pub fn delete(id: &str) -> Result<(), String> {
    let mut todo = read_todo()?;
    let before = todo.tasks.len() + todo.actors.len() + todo.comments.len();

    let is_actor = todo.actors.iter().any(|a| a.id == id);
    let is_comment = todo.comments.iter().any(|c| c.id == id);

    todo.tasks.retain(|t| t.id.as_deref() != Some(id));
    todo.actors.retain(|a| a.id != id);
    todo.comments.retain(|c| c.id != id);

    if is_actor {
        for task in &mut todo.tasks {
            task.actors.retain(|a| a != id);
        }
    }
    if is_comment {
        for task in &mut todo.tasks {
            task.comments.retain(|c| c != id);
        }
    }

    let after = todo.tasks.len() + todo.actors.len() + todo.comments.len();
    if before == after {
        return Err(format!("ID {} not found.", id));
    }

    write_todo(&todo)?;
    println!("Deleted {}", id);
    Ok(())
}

pub fn set_status(id: &str, new_status: &str, reason: Option<&str>) -> Result<(), String> {
    let mut todo = read_todo()?;
    let status = TaskStatus::from_str(new_status)
        .ok_or_else(|| format!("Invalid status '{}'. Use: todo, en-cours, done, bloqued", new_status))?;

    let mut found = false;
    for task in &mut todo.tasks {
        if task.id.as_deref() == Some(id) {
            task.status = status.clone();
            if status == TaskStatus::Blocked {
                task.blocked_reason = reason.map(|s| s.to_string());
            } else {
                task.blocked_reason = None;
            }
            found = true;
        }
    }

    if !found {
        return Err(format!("Task ID {} not found.", id));
    }

    write_todo(&todo)?;
    println!("Task {} set to {}", id, status);
    Ok(())
}

pub fn search(query: &str) -> Result<(), String> {
    list(false, false, false, &[], None, Some(query), false)
}

pub fn list_tags() -> Result<(), String> {
    let todo = read_todo()?;
    let mut counts: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for task in &todo.tasks {
        for tag in &task.tags {
            *counts.entry(tag.to_lowercase()).or_insert(0) += 1;
        }
    }
    if counts.is_empty() {
        println!("No tags found.");
        return Ok(());
    }
    let max_name = counts.keys().map(|k| k.len()).max().unwrap_or(0);
    for (tag, count) in &counts {
        println!("  {:<width$}  {}", tag, count, width = max_name);
    }
    Ok(())
}

pub fn upgrade(force: bool) -> Result<(), String> {
    let current_version = env!("CARGO_PKG_VERSION");

    let api_script = "(Invoke-RestMethod 'https://api.github.com/repos/rayanbo/todo/releases/latest').tag_name";
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", api_script])
        .output()
        .map_err(|e| format!("Failed to check latest version: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to fetch latest release:\n{}", stderr));
    }

    let latest_tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let latest_version = latest_tag.trim_start_matches('v');

    if current_version >= latest_version {
        println!("  ✓ Already up-to-date (v{})", current_version);
        return Ok(());
    }

    println!("  Current: v{}", current_version);
    println!("  Latest:  v{}", latest_version);

    if !force {
        print!("  Upgrade? (Y/n): ");
        stdout().flush().map_err(|e| format!("Flush error: {}", e))?;
        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|e| format!("Read error: {}", e))?;
        let input = input.trim().to_lowercase();
        if !input.is_empty() && input != "y" && input != "yes" {
            println!("  Aborted.");
            return Ok(());
        }
    }

    let asset = "todo-windows-x64.exe";
    let dl_url = format!(
        "https://github.com/rayanbo/todo/releases/download/{}/{}",
        latest_tag, asset
    );

    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("todo-upgrade-tmp.exe");
    let bak_file = temp_dir.join("todo-upgrade-bak.exe");

    let dl_script = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
        dl_url, temp_file.to_string_lossy().replace('\'', "''")
    );

    print!("  Downloading v{}...", latest_version);
    stdout().flush().ok();
    let dl = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &dl_script])
        .output()
        .map_err(|e| format!("Download failed: {}", e))?;

    if !dl.status.success() {
        let stderr = String::from_utf8_lossy(&dl.stderr);
        let _ = std::fs::remove_file(&temp_file);
        return Err(format!("Download failed:\n{}", stderr));
    }
    println!(" done");

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Cannot locate current binary: {}", e))?;

    // Remove old backup if it exists
    let _ = std::fs::remove_file(&bak_file);

    // Rename current → .bak, then copy new → current
    std::fs::rename(&current_exe, &bak_file)
        .map_err(|e| format!("Backup failed: {}", e))?;

    std::fs::copy(&temp_file, &current_exe)
        .map_err(|e| format!("Install failed: {}", e))?;

    // Cleanup temp download
    let _ = std::fs::remove_file(&temp_file);

    println!("  ✓ Upgraded from v{} to v{}", current_version, latest_version);
    println!("  ⚠ Restart your terminal to use the new version");

    Ok(())
}

pub fn is_installed() -> bool {
    let current = std::env::current_exe().ok();
    let installed = Some(install_dir().join("todo.exe"));
    current == installed
}

#[cfg(windows)]
pub fn install() -> Result<(), String> {
    let dir = install_dir();
    let exe = std::env::current_exe()
        .map_err(|e| format!("Cannot locate binary: {e}"))?;

    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    let dest = dir.join("todo.exe");
    std::fs::copy(&exe, &dest)
        .map_err(|e| format!("Copy failed: {e}"))?;

    let user_path = std::env::var("Path").unwrap_or_default();
    let dir_str = dir.to_string_lossy().to_string();
    let path_updated = !user_path.split(';').any(|p| p == dir_str);

    if path_updated {
        let new_path = format!("{};{}", user_path, dir_str);
        let ps_cmd = format!(
            "[Environment]::SetEnvironmentVariable('Path','{}','User')",
            new_path.replace('\'', "''")
        );
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_cmd])
            .output()
            .map_err(|e| format!("PATH update failed: {e}"))?;
        if !output.status.success() {
            return Err("Failed to update user PATH.".to_string());
        }
    }

    println!("  ✓ todo installed in PATH");
    println!("    Location: {}", dest.to_string_lossy());
    if path_updated {
        println!("  ➕ PATH updated (restart terminal)");
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn install() -> Result<(), String> {
    Err("Auto-install is only supported on Windows.".to_string())
}

fn install_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(
        std::env::var("LOCALAPPDATA").unwrap_or_default()
    )
    .join("Programs")
    .join("todo")
}
