use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn todo_binary() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let status = Command::new("cargo")
        .args(["build"])
        .current_dir(&root)
        .status()
        .expect("Failed to build");
    assert!(status.success(), "cargo build failed");
    root.join("target/debug/todo-cli.exe")
}

fn setup(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("todo_cli_test").join(name);
    let _ = fs::create_dir_all(&dir);
    let _ = fs::remove_file(dir.join("TODO.md"));
    dir
}

fn teardown(dir: &PathBuf) {
    let _ = fs::remove_file(dir.join("TODO.md"));
}

fn extract_task_id(content: &str) -> String {
    content
        .lines()
        .find(|l| l.starts_with("- ["))
        .and_then(|l| {
            let after_bracket = l.split("] ").nth(1)?;
            after_bracket.split(" **-** ").next().map(|s| s.trim().to_string())
        })
        .expect("No task found")
}

fn run_todo_in(dir: &PathBuf, args: &[&str]) -> Result<String, String> {
    let bin = todo_binary();
    let output = Command::new(&bin)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| format!("Failed to run: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stderr.trim().to_string())
    }
}

#[test]
fn test_init_creates_file() {
    let dir = setup("init");
    let result = run_todo_in(&dir, &["init"]);
    assert!(result.is_ok(), "init failed: {:?}", result.err());
    assert!(dir.join("TODO.md").exists());

    let content = fs::read_to_string(dir.join("TODO.md")).unwrap();
    assert!(content.contains("# Tasks"));

    teardown(&dir);
}

#[test]
fn test_add_and_list_task() {
    let dir = setup("add_task");
    run_todo_in(&dir, &["init"]).unwrap();
    run_todo_in(&dir, &["add", "--task", "Test task"]).unwrap();

    let list_out = run_todo_in(&dir, &["list"]).unwrap();
    assert!(list_out.contains("[ ]"));
    assert!(list_out.contains("Test task"));

    teardown(&dir);
}

#[test]
fn test_add_actor() {
    let dir = setup("add_actor");
    run_todo_in(&dir, &["init"]).unwrap();
    run_todo_in(&dir, &["add", "--actor", "TestUser"]).unwrap();

    let list_out = run_todo_in(&dir, &["list", "--actors"]).unwrap();
    assert!(list_out.contains("TestUser"));

    teardown(&dir);
}

#[test]
fn test_add_comment() {
    let dir = setup("add_comment");
    run_todo_in(&dir, &["init"]).unwrap();
    run_todo_in(&dir, &["add", "--task", "Task for comment"]).unwrap();
    let content = fs::read_to_string(dir.join("TODO.md")).unwrap();
    let task_id = extract_task_id(&content);

    run_todo_in(&dir, &["add", "--comment", "Test comment", "--task-id", &task_id]).unwrap();

    let list_out = run_todo_in(&dir, &["list", "--comments"]).unwrap();
    assert!(list_out.contains("Test comment"));

    teardown(&dir);
}

#[test]
fn test_status_change() {
    let dir = setup("status");
    run_todo_in(&dir, &["init"]).unwrap();
    run_todo_in(&dir, &["add", "--task", "Status test"]).unwrap();

    let content = fs::read_to_string(dir.join("TODO.md")).unwrap();
    let id = extract_task_id(&content);

    run_todo_in(&dir, &["status", &id, "--set", "done"]).unwrap();
    let content2 = fs::read_to_string(dir.join("TODO.md")).unwrap();
    assert!(content2.contains(&format!("[x] {}", id)));

    run_todo_in(&dir, &["status", &id, "--set", "bloqued", "--reason", "test reason"]).unwrap();
    let content3 = fs::read_to_string(dir.join("TODO.md")).unwrap();
    assert!(content3.contains("[B]"));
    assert!(content3.contains("test reason"));

    teardown(&dir);
}

#[test]
fn test_delete() {
    let dir = setup("delete");
    run_todo_in(&dir, &["init"]).unwrap();
    run_todo_in(&dir, &["add", "--task", "To delete"]).unwrap();

    let content = fs::read_to_string(dir.join("TODO.md")).unwrap();
    let id = extract_task_id(&content);

    run_todo_in(&dir, &["delete", &id]).unwrap();
    let content2 = fs::read_to_string(dir.join("TODO.md")).unwrap();
    assert!(!content2.contains(&id));

    teardown(&dir);
}

#[test]
fn test_update_task_description() {
    let dir = setup("update");
    run_todo_in(&dir, &["init"]).unwrap();
    run_todo_in(&dir, &["add", "--task", "Original desc"]).unwrap();

    let content = fs::read_to_string(dir.join("TODO.md")).unwrap();
    let id = extract_task_id(&content);

    run_todo_in(&dir, &["update", &id, "--description", "Updated desc"]).unwrap();
    let content2 = fs::read_to_string(dir.join("TODO.md")).unwrap();
    assert!(content2.contains("Updated desc"));

    teardown(&dir);
}
