use std::{error::Error, str::FromStr, time::Duration, fs::File, io::copy};

use flate2::bufread::GzDecoder;
use reqwest::{Method, StatusCode, Url, Client, Request, header::HeaderValue};
use tar::Archive;
use std::io::{self, Write};
use tempdir::TempDir;
use crate::models::gh_release::{GHRelease, Asset};

pub async fn is_new_version_available() -> Result<Option<String>, Box<dyn Error>> {
    let client = Client::new();
    let uri = "https://api.github.com/repos/MichaelProjects/depploy/releases";
    let mut request = Request::new(Method::GET, Url::from_str(uri)?);
    request.headers_mut().append("User-Agent", "request".parse()?);
    let res = client.execute(request).await?;
    if res.status() == StatusCode::OK {
        let data: Vec<GHRelease> = res.json().await?;
        println!("{:?}", data);
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let os = determine_os();
        if os.is_some() {
            let os = os.unwrap();
            for x in data {
                let mut vers = x.tag_name.split("").collect::<Vec<&str>>();
                vers.remove(0);
                let full = vers.join("");
                if full.parse::<u128>()? > VERSION.parse::<u128>()? {
                    for asset in x.assets {
                        if asset.browser_download_url.contains(&os){
                            return Ok(Some(asset.browser_download_url))
                        }
                    }
                }
            }
        }
    }
    return Ok(None);
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

async fn download_bin(asset: String) -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new("example")?;
    let response = reqwest::get(asset).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;


    Ok(())
}


#[tokio::test]
async fn test_check_for_new_version(){
    let res = is_new_version_available().await.unwrap();
    println!("RES: {:?}", res);
    download_bin(res.unwrap()).await;
    assert!(false)
}