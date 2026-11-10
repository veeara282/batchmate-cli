use std::path::PathBuf;
use std::{fs::File, io::Write};

use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};

use cfgmatic_paths::PathsBuilder;
use config::Config;
use serde::{Deserialize, Serialize};
use serde_json;
use text_io::read;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Settings {
    calendar_url: String,
}

#[derive(Args)]
pub struct LoginArgs {}

pub fn run(args: LoginArgs) -> Result<()> {
    let finder = PathsBuilder::new("batchmate").build();
    let user_dirs = finder.user_dirs();
    println!("User config dirs: {:?}", user_dirs);

    let mut config_path: PathBuf;
    if let Ok(config_dir) = finder.ensure_user_config_dir() {
        println!("Config: {}", config_dir.display());
        config_path = config_dir.clone();
        config_path.push("settings.json");
    } else {
        return Err(anyhow!("Failed to locate config file"));
    }
    println!("Config file path: {:?}", config_path);

    // let settings = Config::builder()
    //     .add_source(config::File::with_name(config_path.to_str().unwrap()))
    //     .build()?;
    // let settings: Settings = settings.try_deserialize()?;

    println!("Paste your RC calendar feed URL here:");
    let input_url: String = read!();
    // some validation should happen here
    let new_settings = Settings {
        calendar_url: input_url,
    };
    let json_settings = serde_json::to_string(&new_settings)?;
    let mut f = File::create(config_path)?;
    f.write_all(json_settings.as_bytes())?;

    println!("Settings saved!");
    Ok(())
}
