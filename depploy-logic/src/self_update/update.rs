use std::{error::Error, str::FromStr, time::Duration};

use reqwest::{Method, StatusCode, Url};

use crate::models::gh_release::GHRelease;

pub async fn is_new_version_available() -> Result<bool, Box<dyn Error>> {
    let uri = "https://api.github.com/repos/MichaelProjects/depploy/releases";
    let res = reqwest::get(uri).await?;
    if res.status() == StatusCode::OK {
        let data: Vec<GHRelease> = res.json().await?;
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let os = determine_os();
        if os.is_some() {
            for x in data {
                let mut vers = x.tag_name.split("").collect::<Vec<&str>>();
                vers.remove(0);
                let full = vers.join("");
                if full.parse::<u128>()? > VERSION.parse::<u128>()? {}
            }
        }
    }
    return Ok(false);
}

fn determine_os() -> Option<String> {
    let os = std::env::var("OSTYPE").unwrap();
    if os.contains("darwin") {
        return Some("darwin".to_string());
    }
    if os.contains("linux-gnu") {
        return Some("linux-gnu".to_string());
    }
    return None;
}