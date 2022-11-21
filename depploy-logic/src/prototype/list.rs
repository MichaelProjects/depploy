use std::error::Error;

use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use reqwest::StatusCode;

use crate::common::models::{Cfg, ServerResponse, DeployedPrototype};


pub async fn list_running_services(cfg: Cfg, host: &String, token: String) -> Result<(), Box<dyn Error>> {
    let uri = format!("{}/api/v1/config/list", host);
    let client = reqwest::Client::new();
    let res = client.post(uri)
    .header("Authentication", token)
    .body(serde_json::to_string(&cfg)?).send().await?;
    if res.status() != StatusCode::OK{
        println!("Could not upload the config");
    }
    let raw_data: ServerResponse = res.json().await?;
    
    let data: Vec<DeployedPrototype> = serde_json::from_str(&raw_data.data.as_str())?;

    print_stdout(data.with_title())?;
    
    Ok(())
}