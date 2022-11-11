use std::{error::Error, path::PathBuf};
use serde::{Deserialize, Serialize};

use crate::common::error::PTGenError;
use crate::common::models::{PrototypeConfig};
use crate::common::{models::ServerResponse, utils::create_project_cache};


#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePrototype {
    pub name: String,
    pub repo_url: String,
    pub docker_registry_uri: String
}

impl CreatePrototype {
    pub fn new(name: String, repo_url: String, docker_registry_uri: String) -> Self { Self { name, repo_url, docker_registry_uri } }
}


pub async fn send_creation_prototype(pt: &CreatePrototype, project_path: &String, host: &String, token: &String) -> Result<String, Box<dyn Error>> {
    let uri = format!("{}/api/v1/config/prototype", host);
    let client = reqwest::Client::new();
    let res= client.post(uri)
    .header("Authentication", token)
    .body(serde_json::to_string(&pt)?).send().await?.text().await?;
    let data: ServerResponse = serde_json::from_str(res.as_str())?;
    return Ok(data.data)
}

pub fn check_for_prototype(project_path: &String) -> Result<PathBuf, Box<dyn Error>>{
    create_project_cache(&project_path)?;
    let path = PathBuf::from(format!("{}/.depploy/prototype.json", &project_path).as_str());
    if path.exists() {
        return Err(Box::new(PTGenError::Exists));
    }
    return Ok(path);
}

pub fn presist_creation_prototype(project_path: String, pt_data: PrototypeConfig) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(format!("{}/.depploy/prototype.json", &project_path).as_str());
    std::fs::write(
        path,
        serde_json::to_string_pretty(&pt_data)?,
    )?;
    return Ok(())
}

