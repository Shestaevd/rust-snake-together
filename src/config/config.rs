use crate::model::error::ConfigError;
use crate::model::error::ConfigError::{ParsingError, WritingError, WrongPathError};
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct AppConfig {
    pub game_tick_ms: u64,
    pub food_count: u8,
    pub score_multiplier: f32,
    pub input_config: InputMapConfig,
    #[serde(default)]
    pub terminal_min_size: TerminalMinSize,
    #[serde(default)]
    pub difficulty: DifficultyConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            game_tick_ms: 1000,
            food_count: 1,
            score_multiplier: 10.0,
            input_config: InputMapConfig::default(),
            terminal_min_size: TerminalMinSize::default(),
            difficulty: DifficultyConfig::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct DifficultyConfig {
    pub easy: u32,
    pub advanced: u32,
    pub hard: u32,
}

impl Default for DifficultyConfig {
    fn default() -> Self {
        DifficultyConfig {
            easy: 1000,
            advanced: 500,
            hard: 200,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct InputMapConfig {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub back: String,
    pub enter: String,
}

impl Default for InputMapConfig {
    fn default() -> Self {
        InputMapConfig {
            up: String::from("Up"),
            down: String::from("Down"),
            left: String::from("Left"),
            right: String::from("Right"),
            back: String::from("Backspace"),
            enter: String::from("Enter"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TerminalMinSize {
    pub width: u16,
    pub height: u16,
}

impl Default for TerminalMinSize {
    fn default() -> Self {
        TerminalMinSize {
            width: 80,
            height: 24,
        }
    }
}

fn resources_config_path() -> Result<PathBuf, ConfigError> {
    let exe_path = env::current_exe()
        .map_err(|e| WrongPathError(format!("Can not determine executable location: {e}")))?;

    let exe_dir = exe_path.parent().ok_or_else(|| {
        WrongPathError(String::from("Executable path has no parent directory"))
    })?;

    Ok(exe_dir.join("resources").join("config.json"))
}

pub fn get_config() -> Result<AppConfig, ConfigError> {
    let config_path = resources_config_path()?;

    if !config_path.exists() {
        let default_config = AppConfig::default();
        write_config(&default_config, &config_path)?;
        return Ok(default_config);
    }

    read_config_from_path(&config_path)
}

fn read_config_from_path(path: &Path) -> Result<AppConfig, ConfigError> {
    Config::builder()
        .add_source(File::from(path))
        .build()
        .and_then(|settings| settings.try_deserialize::<AppConfig>())
        .map_err(|e| ParsingError(e.to_string()))
}

pub fn write_config(config: &AppConfig, path: &Path) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| WritingError(format!("Can not create config directory: {e}")))?;
    }

    let json = serde_json::to_string_pretty(config).map_err(|e| WritingError(e.to_string()))?;

    fs::write(path, json).map_err(|e| WritingError(e.to_string()))
}
