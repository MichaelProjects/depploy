use crate::load_project_file;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use toml;

#[derive(Deserialize)]
pub struct DepployConfig {
    pub docker_registry: String,
}
impl DepployConfig{
    pub fn new(docker_registry: &str) -> Self{
        DepployConfig{docker_registry: docker_registry.to_string()}
    }
}

pub fn does_depploy_conf_exists(depploy_dir: &Path) -> bool {
    depploy_dir.exists()
}

pub fn read_depploy_conf(depploy_dir: &PathBuf) -> std::io::Result<DepployConfig> {
    
    let does_exist = does_depploy_conf_exists(&depploy_dir.as_path());
    if does_exist {
        let config = load_project_file(depploy_dir, &"settings.toml".to_string())?;
        let config_data: DepployConfig = toml::from_str(config.as_str())?;
        Ok(config_data)
    } else {
        Ok(DepployConfig::new(""))
    }
}
