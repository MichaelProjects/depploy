
use std::fs;
use std::str::FromStr;
use std::{error::Error, path::PathBuf};

use reqwest::StatusCode;

use crate::common::error::PTGenError;
use crate::common::models::{Cfg, PrototypeConfig};



pub async fn upload_config(cfg: Cfg, host: &String, token: String) -> Result<(), Box<dyn Error>> {
    let uri = format!("{host}/api/v1/config/upload");
    let client = reqwest::Client::new();
    let res = client.post(uri)
    .header("Authentication", token)
    .body(serde_json::to_string(&cfg)?).send().await?;
    if res.status() != StatusCode::OK{
        println!("Could not upload the config, error: {:?}", res.text().await?);
        return Ok(())
    }
    let body = res.text().await?;
    let host_uri = format!("{host}/api/v1/pt-service/{body}");
    println!("app-exposed on: \n{host_uri:?}");
    Ok(())
}

pub fn read_project_file(project_path: &String) -> Result<Cfg, Box<dyn Error>> {
    let project_cache = load_project_cache(project_path)?.unwrap();
    
    let path = PathBuf::from_str(format!("{project_path}/conf.toml").as_str())?;

    let fname = path.file_name().unwrap().to_owned().to_string_lossy().to_string();
    let file_type = path.extension().unwrap().to_owned().to_string_lossy().to_string();

    let data = load_project_conf(&path)?;

    let cfg = Cfg::new(fname, file_type, project_cache.app_id, data);
    Ok(cfg)
}


fn load_project_cache(project_path: &String) -> Result<Option<PrototypeConfig>, Box<dyn Error>> {
    let path = PathBuf::from(format!("{}/.depploy/prototype.json", &project_path).as_str());
    if !path.exists() {
        return Err(Box::new(PTGenError::ConfigNotFound));
    }
    
    let text = std::fs::read_to_string(&path).unwrap();
    Ok(Some(serde_json::from_str::<PrototypeConfig>(&text)?))
}

fn load_project_conf(project_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(project_path)?;
    let encoded = base64::encode(data);
    Ok(encoded)
}