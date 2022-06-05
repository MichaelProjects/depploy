use crate::load_project_file;
use serde::Deserialize;
use std::{path::{Path, PathBuf}, fs::File, io};
use toml;
use log::{debug, error, info, trace, warn};
use std::error::Error;


#[derive(Deserialize, Clone)]
pub struct DepployConfig {
    pub docker_registry: String,
    pub docker_hub_username: Option<String>
}
impl DepployConfig{
    pub fn new(docker_registry: &str, docker_hub_username: Option<String>) -> Self{
        DepployConfig{docker_registry: docker_registry.to_string(), docker_hub_username}
    }
}


pub async fn read_depploy_conf(depploy_dir: &PathBuf) -> Result<DepployConfig, Box<dyn Error>> {
    //! The depploy settings file reader, checks if the depploy settings file is present, if not it will fetch the 
    //! example config from the git repo.
    
    let mut path = depploy_dir.clone();
    path.push("settings.toml");
    if !path.exists() {
        let url = "https://raw.githubusercontent.com/MichaelProjects/depploy/dev/example_settings.toml";
        debug!("Fetching depploy settings file from {}", url);
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let mut out =
            File::create(&path).expect("Could not create file please run again with sudo");
        io::copy(&mut body.as_bytes(), &mut out)?;
    }
    let config = load_project_file(depploy_dir, &"settings.toml".to_string())?;
    let config_data: DepployConfig = toml::from_str(config.as_str())?;
    Ok(config_data)
}
