use crate::models::TodoFile;

pub fn write_yaml(todo: &TodoFile) -> Result<String, String> {
    serde_yaml::to_string(todo).map_err(|e| format!("YAML serialization error: {}", e))
}

pub fn read_yaml(content: &str) -> Result<TodoFile, String> {
    serde_yaml::from_str(content).map_err(|e| format!("YAML parse error: {}", e))
}
