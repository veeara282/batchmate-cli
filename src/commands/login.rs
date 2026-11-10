use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use cfgmatic_paths::PathsBuilder;
use serde::{Deserialize, Serialize};
use serde_json;
use text_io::read;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Settings {
    calendar_url: String,
}

#[derive(Args)]
pub struct LoginArgs {}

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

pub fn run(args: LoginArgs) -> Result<()> {
    let config_path = get_config_path()?;

    println!("Paste your RC calendar feed URL here:");
    let input_url: String = read!();
    // some validation should happen here
    let new_settings = Settings {
        calendar_url: input_url,
    };
    let json_settings = serde_json::to_string(&new_settings)?;
    fs::write(config_path, &json_settings)?;

    println!("Settings saved!");
    Ok(())
}
