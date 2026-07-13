use std::fs;
use std::path::Path;

const CONFIG_DIR: &str = ".todo";
const CONFIG_FILE: &str = ".todo/config";

pub struct Config {
    pub cwi: String,
}

impl Config {
    pub fn load() -> Config {
        let path = Path::new(CONFIG_FILE);
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(s) => {
                    let cwi = s.trim().to_lowercase();
                    if cwi == "yaml" || cwi == "yml" {
                        return Config { cwi: "yaml".to_string() };
                    }
                }
                Err(_) => {}
            }
        }
        Config { cwi: "md".to_string() }
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = Path::new(CONFIG_DIR);
        if !dir.exists() {
            fs::create_dir_all(dir).map_err(|e| format!("Config dir error: {}", e))?;
        }
        fs::write(CONFIG_FILE, &self.cwi).map_err(|e| format!("Config write error: {}", e))
    }

    pub fn todo_file(&self) -> String {
        format!("./TODO.{}", self.cwi)
    }

    pub fn md_exists() -> bool {
        Path::new("./TODO.md").exists()
    }

    pub fn yaml_exists() -> bool {
        Path::new("./TODO.yaml").exists() || Path::new("./TODO.yml").exists()
    }

    pub fn any_exists() -> bool {
        Self::md_exists() || Self::yaml_exists()
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.todo_file()).exists()
    }
}
