use crate::models::*;

pub fn serialize_todo(todo: &TodoFile) -> String {
    let mut out = String::new();

    if let Some(ref project) = todo.project {
        out.push_str(&format!("<!-- todo-project: {} -->\n\n", project));
    }

    out.push_str("# Tasks\n\n");

    for task in &todo.tasks {
        let id_part = match &task.id {
            Some(id) => format!("{} **-** ", id),
            None => String::new(),
        };
        let desc = task.description.replace('\n', "\\n");
        out.push_str(&format!("- [{}] {}{}\n", task.status.token(), id_part, desc));

        if let Some(due) = &task.due {
            out.push_str(&format!("  - **Due**: {}\n", due.format("%Y-%m-%d %H:%M")));
        }
        if !task.actors.is_empty() {
            out.push_str(&format!("  - **Actors**: {}\n", task.actors.join(",")));
        }
        if !task.comments.is_empty() {
            out.push_str(&format!("  - **Comments**: {}\n", task.comments.join(",")));
        }
        if let Some(reason) = &task.blocked_reason {
            out.push_str(&format!("  - **Blocked-reason**: \"{}\"\n", reason));
        }
        if let Some(priority) = &task.priority {
            out.push_str(&format!("  - **Priority**: {}\n", priority.as_str()));
        }
        if let Some(created) = &task.created {
            out.push_str(&format!("  - **Created**: {}\n", created.format("%Y-%m-%d %H:%M")));
        }
        if !task.tags.is_empty() {
            out.push_str(&format!("  - **Tags**: {}\n", task.tags.join(", ")));
        }
        out.push('\n');
    }

    out.push_str("# Actors\n\n");
    for actor in &todo.actors {
        out.push_str(&format!("- {}\n", actor.id));
        if let Some(pseudo) = &actor.pseudo {
            out.push_str(&format!("  - **Pseudo**: {}\n", pseudo));
        }
        if let Some(pic) = &actor.pic {
            out.push_str(&format!("  - **Pic**: {}\n", pic));
        }
        out.push_str(&format!("  - **Type**: {}\n", actor.actor_type.as_str()));
        out.push('\n');
    }

    out.push_str("# Comments\n\n");
    for comment in &todo.comments {
        if !comment.id.is_empty() {
            out.push_str(&format!("- {} **-** {}\n", comment.id, comment.text));
        } else {
            out.push_str(&format!("- {}\n", comment.text));
        }
        if !comment.actors.is_empty() {
            out.push_str(&format!("  - **Actors**: {}\n", comment.actors.join(",")));
        }
        if let Some(task_id) = &comment.task_id {
            out.push_str(&format!("  - **Task**: {}\n", task_id));
        }
        out.push('\n');
    }

    out
}
