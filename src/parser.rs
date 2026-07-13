use crate::id_gen::is_valid_id;
use crate::models::*;
use chrono::NaiveDateTime;
use regex::Regex;

pub fn parse_todo(content: &str) -> Result<TodoFile, String> {
    let mut todo = TodoFile::empty();

    let project_re = Regex::new(r"^<!-- todo-project: (.+) -->$").unwrap();
    let task_re = Regex::new(r"^- \[([ x~B])\] (.+)$").unwrap();
    let meta_re = Regex::new(r"^  - \*\*([A-Za-z-]+)\*\*: (.+)$").unwrap();
    let actor_re = Regex::new(r"^- (.+)$").unwrap();

    let mut current_section: Option<String> = None;
    let mut lines = content.lines().peekable();

    // Parse optional project name from first line
    if let Some(first) = lines.peek() {
        if let Some(caps) = project_re.captures(first) {
            todo.project = Some(caps.get(1).unwrap().as_str().to_string());
            lines.next(); // consume the comment line
            // skip the blank line that follows
            if lines.peek().map(|l| l.trim().is_empty()).unwrap_or(false) {
                lines.next();
            }
        }
    }

    while let Some(line) = lines.next() {
        if line.starts_with("# ") || line.starts_with("## ") {
            let name = line
                .trim_start_matches(|c| c == '#' || c == ' ')
                .trim()
                .to_lowercase();
            current_section = Some(name);
            continue;
        }

        match current_section.as_deref() {
            Some("tasks") => {
                if let Some(caps) = task_re.captures(line) {
                    let status_token = caps.get(1).unwrap().as_str();
                    let content = caps.get(2).unwrap().as_str();
                    let status = TaskStatus::from_token(status_token).unwrap_or(TaskStatus::Todo);

                    let (id, raw_description) = parse_task_line(content);
                    let description = raw_description.replace("\\n", "\n");

                    let mut task = Task {
                        id,
                        status,
                        description,
                        due: None,
                        actors: Vec::new(),
                        comments: Vec::new(),
                        blocked_reason: None,
                        tags: Vec::new(),
                        priority: None,
                        created: None,
                        position: None,
                    };

                    while let Some(peek) = lines.peek() {
                        if peek.starts_with("  - **") {
                            if let Some(meta_caps) = meta_re.captures(peek) {
                                let label = meta_caps.get(1).unwrap().as_str();
                                let value = meta_caps.get(2).unwrap().as_str().trim().to_string();
                                match label.to_lowercase().as_str() {
                                    "due" => {
                                        task.due = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M").ok();
                                    }
                                    "actors" => {
                                        task.actors = value.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                                    }
                                    "comments" => {
                                        task.comments = value.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                                    }
                                    "blocked-reason" => {
                                        task.blocked_reason = Some(value.trim_matches('"').to_string());
                                    }
                                    "tags" => {
                                        task.tags = value.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                                    }
                                    "priority" => {
                                        task.priority = Priority::from_str(&value);
                                    }
                                    "created" => {
                                        task.created = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M").ok();
                                    }
                                    "position" => {
                                        task.position = Some(value.trim_matches('"').to_string());
                                    }
                                    _ => {}
                                }
                            }
                            lines.next();
                        } else {
                            break;
                        }
                    }

                    todo.tasks.push(task);
                }
            }
            Some("actors") => {
                if let Some(caps) = actor_re.captures(line) {
                    let id = caps.get(1).unwrap().as_str().trim();
                    let mut actor = Actor {
                        id: id.to_string(),
                        pseudo: None,
                        pic: None,
                        actor_type: ActorType::Human,
                    };

                    while let Some(peek) = lines.peek() {
                        if peek.starts_with("  - **") {
                            if let Some(meta_caps) = meta_re.captures(peek) {
                                let label = meta_caps.get(1).unwrap().as_str();
                                let value = meta_caps.get(2).unwrap().as_str().trim().to_string();
                                match label.to_lowercase().as_str() {
                                    "pseudo" => actor.pseudo = Some(value),
                                    "pic" => actor.pic = Some(value),
                                    "type" => {
                                        actor.actor_type = ActorType::from_str(&value).unwrap_or(ActorType::Human);
                                    }
                                    _ => {}
                                }
                            }
                            lines.next();
                        } else {
                            break;
                        }
                    }

                    todo.actors.push(actor);
                }
            }
            Some("comments") => {
                if let Some(caps) = actor_re.captures(line) {
                    let after_dash = caps.get(1).unwrap().as_str().trim();
                    let (id, text) = parse_comment_line(after_dash);
                    let mut comment = Comment {
                        id,
                        text,
                        actors: Vec::new(),
                        task_id: None,
                    };

                    while let Some(peek) = lines.peek() {
                        if peek.starts_with("  - **") {
                            if let Some(meta_caps) = meta_re.captures(peek) {
                                let label = meta_caps.get(1).unwrap().as_str();
                                let value = meta_caps.get(2).unwrap().as_str().trim().to_string();
                                match label.to_lowercase().as_str() {
                                    "actors" => {
                                        comment.actors = value.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                                    }
                                    "task" => {
                                        comment.task_id = Some(value);
                                    }
                                    _ => {}
                                }
                            }
                            lines.next();
                        } else {
                            break;
                        }
                    }

                    todo.comments.push(comment);
                }
            }
            _ => {}
        }
    }

    Ok(todo)
}

fn parse_task_line(content: &str) -> (Option<String>, String) {
    let sep = " **-** ";
    if let Some(pos) = content.find(sep) {
        let before = content[..pos].trim();
        let after = content[pos + sep.len()..].trim();
        if is_valid_id(before) {
            return (Some(before.to_string()), after.to_string());
        }
    }
    (None, content.trim().to_string())
}

fn parse_comment_line(content: &str) -> (String, String) {
    let sep = " **-** ";
    if let Some(pos) = content.find(sep) {
        let before = content[..pos].trim();
        let after = content[pos + sep.len()..].trim();
        if is_valid_id(before) {
            return (before.to_string(), after.to_string());
        }
    }
    (String::new(), content.trim().to_string())
}
