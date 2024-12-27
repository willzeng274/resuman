use anyhow::Result;
use dirs;
use serde_derive::Deserialize;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use toml;

pub fn save_to_file(path: &str, content: &str, append: Option<bool>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(append.unwrap_or(false))
        .create(true)
        .open(path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Data {
    // these are the [config] fields
    pub config: Config,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub root_dir: std::path::PathBuf,
}

pub fn load_config(config_path: Option<PathBuf>) -> Result<Data> {
    if let Some(path) = config_path {
        let file = std::fs::read_to_string(path)?;
        return Ok(toml::from_str(&file)?);
    }
    // it doesn't make sense to use a macro here
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    // find the config file with 4 path options
    let config_paths = [
        PathBuf::from("resuman.toml"), // For debugging
        home.join(".config/resuman/config.toml"),
        home.join(".resuman/config.toml"),
        home.join(".resuman.toml"),
    ];
    // prefers .config/ over .resuman/ over .resuman.toml
    for path in &config_paths {
        if path.exists() {
            let file = std::fs::read_to_string(path)?;
            return Ok(toml::from_str(&file)?);
        }
    }
    // must have a config file to use resuman... specify root_dir
    Err(anyhow::anyhow!("Cannot find a config file"))
}

pub fn resolve_path(mut path: PathBuf) -> Result<PathBuf> {
    // expand ~ as home_dir
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    log::debug!("Home directory: {:?}", home);

    if let Some(str_path) = path.to_str() {
        if str_path.starts_with("~/") || str_path == "~" {
            // Replace `~` with the home directory
            path = home.join(&str_path[2..]); // Remove `~/` and append the rest
        }
    }

    // if the path is relative just throw an error
    // this is a CLI tool, we want to be explicit
    if !path.is_absolute() {
        return Err(anyhow::anyhow!("Path must be absolute"));
    }

    Ok(path)
}
