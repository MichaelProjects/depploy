use config::{ConfigError, Config, File};
use serde::Deserialize;


#[derive(Deserialize, Clone, Debug)]
pub struct Registry {
    pub docker_registry: String,
    pub docker_registry_username: Option<String>,
    pub docker_registry_password: Option<String>,
    pub docker_hub_username: Option<String>
}
#[derive(Deserialize, Clone, Debug)]
pub struct PrototypeSettings {
    pub prototype_host: String,
    pub prototype_app_token: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct DepployConfig {
    pub registry: Registry,
    pub prototype: Option<PrototypeSettings>,
}

impl DepployConfig {
    pub fn new(path: String) -> Result<DepployConfig, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name(path.as_str()))?;

        s.try_into()
    }
}
