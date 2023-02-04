use std::error::Error;

use cli_table::{print_stdout, WithTitle};
use reqwest::{StatusCode};

use crate::common::models::{ServerResponse, DeployedPrototype};


pub async fn list_running_services(host: &String, token: String) -> Result<(), Box<dyn Error>> {
    let uri = format!("{host}/api/v1/config/list");
    let client = reqwest::Client::new();
    let res = client.get(uri)
    .header("Authentication", token)
    .send().await?;
    if res.status() != StatusCode::OK{
        println!("Cloud not get running services");
    }
    let raw_data: ServerResponse = res.json().await?;
    
    let data: Vec<DeployedPrototype> = serde_json::from_str(raw_data.data.as_str())?;

    print_stdout(data.with_title())?;
    
    Ok(())
}