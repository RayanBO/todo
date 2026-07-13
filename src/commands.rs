use crate::config::Config;
use crate::id_gen::generate_id;
use crate::models::*;
use crate::parser::parse_todo;
use crate::serializer::serialize_todo;
use crate::tags;
use crate::yaml_util;
use chrono::NaiveDateTime;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn current_file() -> String {
    Config::load().todo_file()
}

fn read_todo() -> Result<TodoFile, String> {
    let cfg = Config::load();
    let file = cfg.todo_file();
    if !Path::new(&file).exists() {
        return Err(format!("{} not found. Run `todo init` first.", file));
    }
    let content = fs::read_to_string(&file).map_err(|e| format!("Read error: {}", e))?;
    if cfg.cwi == "yaml" {
        yaml_util::read_yaml(&content)
    } else {
        parse_todo(&content)
    }
}

fn write_todo(todo: &TodoFile) -> Result<(), String> {
    let cfg = Config::load();
    let file = cfg.todo_file();
    let content = if cfg.cwi == "yaml" {
        yaml_util::write_yaml(todo)?
    } else {
        serialize_todo(todo)
    };
    fs::write(&file, content).map_err(|e| format!("Write error: {}", e))
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

fn prompt_format() -> Result<(bool, bool), String> {
    print!("Initialize as markdown or yaml? (md/yaml/both) [md]: ");
    stdout().flush().map_err(|e| format!("Flush error: {}", e))?;
    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| format!("Read error: {}", e))?;
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "" | "md" | "markdown" => Ok((false, false)),
        "yaml" | "yml" => Ok((true, false)),
        "both" | "all" => Ok((false, true)),
        _ => {
            println!("Invalid choice. Defaulting to markdown.");
            Ok((false, false))
        }
    }
}

fn save_todo(todo: &TodoFile, file: &str) -> Result<(), String> {
    let content = if file.ends_with(".yaml") || file.ends_with(".yml") {
        yaml_util::write_yaml(todo)?
    } else {
        serialize_todo(todo)
    };
    fs::write(file, content).map_err(|e| format!("Write error: {}", e))
}

fn set_project(todo: &mut TodoFile) {
    todo.project = std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()));
}

pub fn init(force: bool, yaml: bool, both: bool) -> Result<(), String> {
    let (use_yaml, use_both) = if yaml {
        (true, false)
    } else if both {
        (false, true)
    } else if !Config::any_exists() {
        // prompt user if no flags and no files exist
        prompt_format()?
    } else {
        (false, false)
    };

    let init_md = !use_yaml || use_both;
    let init_yaml = use_yaml || use_both;

    if init_md {
        let path = Path::new("./TODO.md");
        let exists = path.exists();
        if !exists || force || !is_valid_format(&fs::read_to_string(path).unwrap_or_default()) {
            let mut todo = TodoFile::empty();
            set_project(&mut todo);
            save_todo(&todo, "./TODO.md")?;
            if !exists { println!("Created TODO.md"); }
            else { println!("Overwritten TODO.md"); }
        } else if !force {
            return Err("TODO.md exists with valid format. Use --force to overwrite.".to_string());
        }
    }

    if init_yaml {
        let path = Path::new("./TODO.yaml");
        let exists = path.exists();
        if !exists || force {
            let mut todo = TodoFile::empty();
            set_project(&mut todo);
            save_todo(&todo, "./TODO.yaml")?;
            if !exists { println!("Created TODO.yaml"); }
            else { println!("Overwritten TODO.yaml"); }
        } else if !force {
            return Err("TODO.yaml exists. Use --force to overwrite.".to_string());
        }
    }

    // set cwi to yaml if only yaml was created
    if init_yaml && !init_md {
        let cfg = Config { cwi: "yaml".to_string() };
        cfg.save()?;
    }

    Ok(())
}

pub fn cwi(format: Option<&str>) -> Result<(), String> {
    let mut cfg = Config::load();
    match format {
        Some(f) => {
            let f = f.to_lowercase();
            if f != "md" && f != "yaml" && f != "yml" {
                return Err("Format must be 'md' or 'yaml'".to_string());
            }
            let f = if f == "yml" { "yaml" } else { &f };
            let file = format!("./TODO.{}", f);
            if !Path::new(&file).exists() {
                return Err(format!("{} not found. Run `todo init` first.", file));
            }
            cfg.cwi = f.to_string();
            cfg.save()?;
            println!("Switched to TODO.{}", f);
        }
        None => {
            println!("Current: TODO.{}", cfg.cwi);
        }
    }
    Ok(())
}

pub fn scan() -> Result<(), String> {
    let todo = read_todo()?;
    let cwd = std::env::current_dir().map_err(|e| format!("CWD error: {}", e))?;
    let cwd_str = cwd.to_string_lossy().to_string();
    let re = regex::Regex::new(r"TODO:\s*(.*)").map_err(|e| format!("Regex error: {}", e))?;
    let skip_dirs = [".todo", ".git", "node_modules", "target", ".opencode", ".agents"];
    let skip_exts = [".exe", ".dll", ".so", ".dylib", ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".woff", ".woff2", ".ttf", ".eot", ".o", ".obj", ".pyc", ".class"];
    let skip_files = ["TODO.md", "TODO.yaml", "TODO.yml"];

    let mut found: Vec<(String, usize, String)> = Vec::new();

    fn walk(dir: &std::path::Path, cwd_str: &str, re: &regex::Regex, skip_dirs: &[&str], skip_exts: &[&str], skip_files: &[&str], found: &mut Vec<(String, usize, String)>) -> Result<(), String> {
        let entries = std::fs::read_dir(dir).map_err(|e| format!("Read dir {:?} error: {}", dir, e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Entry error: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if skip_dirs.contains(&fname) { continue; }
                walk(&path, cwd_str, re, skip_dirs, skip_exts, skip_files, found)?;
                continue;
            }

            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if skip_files.contains(&fname) { continue; }

            let ext = path.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e.to_lowercase())).unwrap_or_default();
            if skip_exts.contains(&ext.as_str()) { continue; }

            let content = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for (line_no, line) in content.lines().enumerate() {
                if let Some(m) = re.find(line) {
                    let text = m.as_str().trim_start_matches("TODO:").trim();
                    if text.is_empty() { continue; }
                    let absolute = path.canonicalize().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| path.to_string_lossy().to_string());
                    let col = m.start() + 5;
                    let pos = format!("file:///{file}#L{line}:{col}", file = absolute.replace('\\', "/"), line = line_no + 1);
                    if !found.iter().any(|(_, _, t)| t == &pos) {
                        found.push((text, line_no + 1, pos));
                    }
                }
            }
        }
        Ok(())
    }

    walk(&cwd, &cwd_str, &re, &skip_dirs, &skip_exts, &skip_files, &mut found)?;

    if found.is_empty() {
        println!("No TODO: comments found in source files.");
        return Ok(());
    }

    let mut todo = todo;
    let mut count = 0;
    for (text, _line, pos) in &found {
        let id = crate::id_gen::generate_id();
        let task = Task {
            id: Some(id.clone()),
            status: TaskStatus::Todo,
            description: text.clone(),
            due: None,
            actors: Vec::new(),
            comments: Vec::new(),
            blocked_reason: None,
            tags: Vec::new(),
            priority: None,
            created: Some(chrono::Local::now().naive_local()),
            position: Some(pos.clone()),
        };
        todo.tasks.push(task);
        count += 1;
    }
    write_todo(&todo)?;
    println!("Found {} TODO: comment{} in source files.", count, if count == 1 { "" } else { "s" });
    for (text, _line, pos) in &found {
        println!("  {}: \"{}\"", pos, text);
    }
    Ok(())
}

pub fn add_task(description: &str, actor_ids: &[String], tag_list: &[String], priority: Option<Priority>, due: Option<NaiveDateTime>, status: Option<TaskStatus>, position: Option<String>) -> Result<(), String> {
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
        position,
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

// Rest of the file unchanged from here
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
            if let Some(pos) = &task.position {
                println!("      Position: {}", pos);
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

pub fn update(id: &str, description: Option<&str>, due: Option<&str>, name: Option<&str>, text: Option<&str>, actors: Option<&str>, comments: Option<&str>, pic: Option<&str>, tags: Option<&str>, priority: Option<&str>, blocked_reason: Option<&str>, actor_type: Option<&str>, position: Option<&str>) -> Result<(), String> {
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
            if let Some(p) = position {
                task.position = if p.trim().is_empty() { None } else { Some(p.trim().to_string()) };
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

    let asset = "todo-x64.exe";
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

    let _ = std::fs::remove_file(&bak_file);

    std::fs::rename(&current_exe, &bak_file)
        .map_err(|e| format!("Backup failed: {}", e))?;

    std::fs::copy(&temp_file, &current_exe)
        .map_err(|e| format!("Install failed: {}", e))?;

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
