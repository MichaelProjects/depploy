use cli_table::Table;
use serde::{Deserialize, Serialize};
use cli_table::{format::Justify};


#[derive(Deserialize, Debug)]
pub struct ServerResponse {
    pub http_code: i16,
    pub message: String,
    pub data: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PrototypeConfig {
    pub app_id: String,
    pub docker_repo: String
}

impl PrototypeConfig {
    pub fn new(app_id: String, docker_repo: String) -> Self { Self { app_id, docker_repo } }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cfg {
    pub filename: String,
    pub file_type: String,
    pub app_id: String,
    pub data: String
}

impl Cfg {
    pub fn new(filename: String, file_type: String, app_id: String, data: String) -> Self { Self { filename, file_type, app_id, data } }
}

#[derive(Debug, Serialize, Deserialize, Table)]
pub struct DeployedPrototype {
    #[table(title = "Service-Name", justify = "Justify::Right")]
    pub name: String,
    #[table(title = "Active")]
    pub active: bool,
    #[table(title = "Service-Path")]
    pub service_path: String,
    #[table(title = "is_service_running")]
    pub is_service_running: bool
}