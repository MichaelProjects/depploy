use serde::Deserialize;
use std::{path::{Path, PathBuf},  io, fs::File};
use toml;
use log::{debug, error, info, trace, warn};
use std::error::Error;

use crate::models::depploy::DepployConfig;


pub async fn read_depploy_conf(depploy_dir: &PathBuf) -> Result<DepployConfig, Box<dyn Error>> {
    //! The depploy settings file reader, checks if the depploy settings file is present, if not it will fetch the 
    //! example config from the git repo.
    
    let mut path = depploy_dir.clone();
    if !path.exists() {
        let url = "https://raw.githubusercontent.com/MichaelProjects/depploy/dev/example_settings.toml";
        debug!("Fetching depploy settings file from {}", url);
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let mut out =
            File::create(&path).expect("Could not create file please run again with sudo");
        io::copy(&mut body.as_bytes(), &mut out)?;
    }
    let settings_path = format!("{}/settings.toml", path.to_str().unwrap());
    let config_data = DepployConfig::new(settings_path)?;
    return Ok(config_data)
}
