use serde::{Deserialize, Serialize};



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