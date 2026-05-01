use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_colored")]
    pub colored: bool,
    #[serde(default)]
    pub default_level: Option<String>,
    #[serde(default)]
    pub default_pattern: Option<String>,
    #[serde(default = "default_json")]
    pub json_output: bool,
    #[serde(default = "default_dedup")]
    pub dedup_window: usize,
    #[serde(default)]
    pub time_format: Option<String>,
}

fn default_colored() -> bool { true }
fn default_json() -> bool { false }
fn default_dedup() -> usize { 0 }

impl Default for Config {
    fn default() -> Self {
        Self {
            colored: true,
            default_level: None,
            default_pattern: None,
            json_output: false,
            dedup_window: 0,
            time_format: None,
        }
    }
}

pub fn load_config() -> Config {
    let paths = [
        PathBuf::from(".oxideflow.toml"),
        dirs_home().join(".config/oxideflow/config.toml"),
    ];

    for path in &paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                match toml::from_str(&content) {
                    Ok(cfg) => return cfg,
                    Err(e) => eprintln!("warn: config parse error: {}", e),
                }
            }
        }
    }

    Config::default()
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}
