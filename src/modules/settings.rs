use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use cfgmatic_paths::PathsBuilder;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub calendar_url: String,
}

/// load config file path
pub fn get_config_path() -> Result<PathBuf> {
    let finder = PathsBuilder::new("batchmate").build();
    let user_dirs = finder.user_dirs();
    println!("User config dirs: {:?}", user_dirs);

    let mut config_path = finder.ensure_user_config_dir()?;
    config_path.push("settings.json");
    println!("Config file path: {:?}", config_path);
    Ok(config_path)
}

/// write settings to file
pub fn write_settings(settings: &Settings) -> Result<()> {
    let config_path = get_config_path()?;
    let json_settings = serde_json::to_string(settings)?;
    fs::write(config_path, &json_settings)?;
    Ok(())
}

/// read settings from file
pub fn read_settings() -> Result<Settings> {
    let config_path = get_config_path()?;
    let settings_bytes = fs::read(config_path)?;
    let settings = serde_json::from_slice::<Settings>(&settings_bytes)?;
    Ok(settings)
}
