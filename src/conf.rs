use crate::load_project_file;
use serde::Deserialize;
use std::path::Path;
use toml::from_str;

#[derive(Deserialize)]
pub struct DepployConfig {
    pub docker_registry: String,
}
impl DepployConfig{
    pub fn new(docker_registry: &str) -> Self{
        DepployConfig{docker_registry: docker_registry.to_string()}
    }
}

pub fn does_depploy_conf_exists(depploy_dir: &String) -> bool {
    let path = Path::new(&depploy_dir);
    path.exists()
}

pub fn read_depploy_conf(depploy_dir: String) -> std::io::Result<DepployConfig> {
    let does_exist = does_depploy_conf_exists(&depploy_dir);
    if does_exist {
        let config = load_project_file(depploy_dir)?;
        let config_data: DepployConfig = toml::from_str(config.as_str())?;
        Ok(config_data)
    } else {
        Ok(DepployConfig::new(""))
    }
}