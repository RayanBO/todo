use crate::commands;
use crate::models::*;
use crate::parser::parse_todo;
use serde::Deserialize;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

const TODO_FILE: &str = "./TODO.md";

pub fn serve(start_port: u16) -> Result<(), String> {
    let port = find_port(start_port)?;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .map_err(|e| format!("Bind error: {}", e))?;

    println!("Dashboard: http://127.0.0.1:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                std::thread::spawn(|| handle_client(s));
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
    Ok(())
}

fn find_port(start: u16) -> Result<u16, String> {
    for port in start..(start + 100) {
        if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
            return Ok(port);
        }
    }
    Err("No available port found".to_string())
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let path = parts[1];

    // Read headers
    let mut content_length: usize = 0;
    for line in reader.by_ref().lines() {
        let l = line.unwrap_or_default();
        if l.is_empty() {
            break;
        }
        if l.to_lowercase().starts_with("content-length:") {
            content_length = l.split(':').nth(1).unwrap_or("0").trim().parse().unwrap_or(0);
        }
    }

    if method == "GET" && path.starts_with("/api/pic") {
        let data = api_pic_bytes(path);
        let _ = stream.write_all(&data);
    } else if method == "POST" && path == "/api/upload" && content_length > 0 {
        let mut raw = vec![0u8; content_length];
        if reader.read_exact(&mut raw).is_ok() {
            let resp = api_upload(&raw);
            let _ = stream.write_all(resp.as_bytes());
        } else {
            let resp = json_response(500, serde_json::json!({"error": "Read error"}));
            let _ = stream.write_all(resp.as_bytes());
        }
    } else {
        let mut body = String::new();
        if content_length > 0 {
            let mut buf = vec![0u8; content_length];
            if reader.read_exact(&mut buf).is_ok() {
                body = String::from_utf8_lossy(&buf).to_string();
            }
        }
        let response = match (method, path) {
            ("GET", "/") => serve_html(),
            ("GET", "/api/todo") => api_get_todo(),
            ("POST", "/api/add-task") => api_add_task(&body),
            ("POST", "/api/add-actor") => api_add_actor(&body),
            ("POST", "/api/add-comment") => api_add_comment(&body),
            ("POST", "/api/update") => api_update(&body),
            ("POST", "/api/update-actor") => api_update_actor(&body),
            ("POST", "/api/delete") => api_delete(&body),
            ("POST", "/api/delete-actor") => api_delete(&body),
            ("POST", "/api/status") => api_status(&body),
            _ => json_response(404, serde_json::json!({"error": "Not found"})),
        };
        let _ = stream.write_all(response.as_bytes());
    }
    let _ = stream.flush();
}

fn url_decode(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hi = chars.next().and_then(|c| c.to_digit(16)).unwrap_or(0) as u8;
            let lo = chars.next().and_then(|c| c.to_digit(16)).unwrap_or(0) as u8;
            out.push((hi * 16 + lo) as char);
        } else if c == '+' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

fn api_pic_bytes(uri: &str) -> Vec<u8> {
    let file_path = if let Some(pos) = uri.find('?') {
        let qs = &uri[pos + 1..];
        let mut path = String::new();
        for pair in qs.split('&') {
            let mut kv = pair.splitn(2, '=');
            if kv.next().unwrap_or("") == "path" {
                path = url_decode(kv.next().unwrap_or(""));
                break;
            }
        }
        path
    } else {
        String::new()
    };
    if file_path.is_empty() {
        return json_response(400, serde_json::json!({"error": "Missing path"})).into_bytes();
    }
    match fs::read(&file_path) {
        Ok(data) => {
            let ext = file_path.rsplit('.').next().unwrap_or("").to_lowercase();
            let mime = match ext.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                "webp" => "image/webp",
                "bmp" => "image/bmp",
                "ico" => "image/x-icon",
                _ => "application/octet-stream",
            };
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: no-cache\r\n\r\n",
                mime,
                data.len()
            );
            let mut resp: Vec<u8> = header.into_bytes();
            resp.extend_from_slice(&data);
            resp
        }
        Err(_) => json_response(404, serde_json::json!({"error": "File not found"})).into_bytes(),
    }
}

fn base64_decode(s: &str) -> Vec<u8> {
    // simple base64 decode using a lookup table
    let b64 = s.trim();
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits = 0;
    for c in b64.chars() {
        let val = match c {
            'A'..='Z' => (c as u32) - ('A' as u32),
            'a'..='z' => (c as u32) - ('a' as u32) + 26,
            '0'..='9' => (c as u32) - ('0' as u32) + 52,
            '+' => 62,
            '/' => 63,
            '=' => break,
            _ => continue,
        };
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    out
}

fn api_upload(raw: &[u8]) -> String {
    let body_str = String::from_utf8_lossy(raw);
    let parsed: serde_json::Value = match serde_json::from_str(&body_str) {
        Ok(v) => v,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    let data_url = parsed["data"].as_str().unwrap_or("");
    let name = parsed["name"].as_str().unwrap_or("image.jpg");
    if data_url.is_empty() {
        return json_response(400, serde_json::json!({"error": "Missing data"}));
    }
    // extract base64 part after comma
    let b64 = if let Some(pos) = data_url.find(',') { &data_url[pos + 1..] } else { data_url };
    let bytes = base64_decode(b64);
    if bytes.is_empty() {
        return json_response(400, serde_json::json!({"error": "Empty data"}));
    }
    // ensure pics directory exists
    let _ = fs::create_dir_all("./dashboard/pics");
    // generate unique filename
    let ext = name.rsplit('.').next().unwrap_or("jpg");
    // prefix with timestamp to avoid collisions, but keep original name readable
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
    let base = name.trim_end_matches(&format!(".{}", ext));
    let filename = format!("{}_{}.{}", ts, base, ext);
    let filepath = format!("./dashboard/pics/{}", filename);
    match fs::write(&filepath, &bytes) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true, "path": filepath})),
        Err(e) => json_response(500, serde_json::json!({"error": format!("Write error: {}", e)})),
    }
}

fn json_response(status: u16, data: serde_json::Value) -> String {
    let body = serde_json::to_string(&data).unwrap_or_default();
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        status,
        if status == 200 { "OK" } else { "Error" },
        body.len(),
        body
    )
}

fn html_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        status,
        if status == 200 { "OK" } else { "Error" },
        body.len(),
        body
    )
}

fn serve_html() -> String {
    let path = Path::new("./dashboard/index.html");
    if !path.exists() {
        return html_response(404, "<h1>dashboard/index.html not found</h1>");
    }
    match fs::read_to_string(path) {
        Ok(content) => html_response(200, &content),
        Err(e) => html_response(500, &format!("<h1>Read error: {}</h1>", e)),
    }
}

fn read_todo_json() -> Result<String, String> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(serde_json::to_string(&TodoFile::empty()).unwrap_or_default());
    }
    let content = fs::read_to_string(TODO_FILE).map_err(|e| format!("Read error: {}", e))?;
    let todo = parse_todo(&content)?;
    serde_json::to_string(&todo).map_err(|e| format!("JSON error: {}", e))
}

fn api_get_todo() -> String {
    match read_todo_json() {
        Ok(json) => json_response(200, serde_json::from_str(&json).unwrap_or_default()),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct AddTaskBody {
    description: String,
    actors: Option<String>,
    tags: Option<String>,
    priority: Option<String>,
    due: Option<String>,
    status: Option<String>,
}

fn api_add_task(body: &str) -> String {
    let data: AddTaskBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    let actor_ids: Vec<String> = data.actors
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let tag_list: Vec<String> = data.tags
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let prio = data.priority.as_deref().and_then(Priority::from_str);
    let due_date = data.due.as_deref().and_then(|d| chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M").ok());
    let st = data.status.as_deref().and_then(TaskStatus::from_str);
    match commands::add_task(&data.description, &actor_ids, &tag_list, prio, due_date, st) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct AddActorBody {
    pseudo: String,
    pic: Option<String>,
}

fn api_add_actor(body: &str) -> String {
    let data: AddActorBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    match commands::add_actor(Some(&data.pseudo), data.pic.as_deref()) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct UpdateActorBody {
    id: String,
    pseudo: Option<String>,
    pic: Option<String>,
    actor_type: Option<String>,
}

fn api_update_actor(body: &str) -> String {
    let data: UpdateActorBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    match commands::update(
        &data.id, None, None, data.pseudo.as_deref(), None,
        None, None, data.pic.as_deref(), None, None, None,
        data.actor_type.as_deref(),
    ) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct AddCommentBody {
    text: String,
    task_id: String,
    actors: Option<String>,
}

fn api_add_comment(body: &str) -> String {
    let data: AddCommentBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    let actor_ids: Vec<String> = data.actors
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    match commands::add_comment(&data.text, &data.task_id, &actor_ids) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct UpdateBody {
    id: String,
    description: Option<String>,
    due: Option<String>,
    actors: Option<String>,
    comments: Option<String>,
    name: Option<String>,
    pic: Option<String>,
    text: Option<String>,
    tags: Option<String>,
    priority: Option<String>,
    blocked_reason: Option<String>,
}

fn api_update(body: &str) -> String {
    let data: UpdateBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    match commands::update(
        &data.id, data.description.as_deref(), data.due.as_deref(),
        data.name.as_deref(), data.text.as_deref(),
        data.actors.as_deref(), data.comments.as_deref(),
        data.pic.as_deref(), data.tags.as_deref(), data.priority.as_deref(),
        data.blocked_reason.as_deref(), None,
    ) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct DeleteBody {
    id: String,
}

fn api_delete(body: &str) -> String {
    let data: DeleteBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    match commands::delete(&data.id) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}

#[derive(Deserialize)]
struct StatusBody {
    id: String,
    set: String,
    reason: Option<String>,
}

fn api_status(body: &str) -> String {
    let data: StatusBody = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => return json_response(400, serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
    };
    match commands::set_status(&data.id, &data.set, data.reason.as_deref()) {
        Ok(()) => json_response(200, serde_json::json!({"ok": true})),
        Err(e) => json_response(500, serde_json::json!({"error": e})),
    }
}
