use std::{env, fs, io, path::PathBuf};

use serde::Deserialize;

const APP_NAME: &str = "oxide";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prompt: String::from("$ "),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = get_config_path();

        let content = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Self::default(),
            Err(e) => {
                eprintln!(
                    "warning: read config file at {}: {e}; using defaults",
                    path.display()
                );
                return Self::default();
            }
        };

        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!(
                "warning: parse config toml at {}: {e}; using defaults",
                path.display()
            );
            Self::default()
        })
    }
}

fn get_config_path() -> PathBuf {
    let base = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("$HOME is not set");
            PathBuf::from(home).join(".config")
        });
    base.join(APP_NAME).join(CONFIG_FILE)
}
