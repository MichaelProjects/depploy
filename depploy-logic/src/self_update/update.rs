use std::{error::Error, str::FromStr, fs::{self}, io::{Read}, env, os::unix::prelude::OpenOptionsExt};

use reqwest::{Method, StatusCode, Url, Client, Request};
use std::io::{Write};
use crate::models::gh_release::{GHRelease};

pub async fn is_new_version_available() -> Result<Option<String>, Box<dyn Error>> {
    let client = Client::new();
    let uri = "https://api.github.com/repos/MichaelProjects/depploy/releases";
    let mut request = Request::new(Method::GET, Url::from_str(uri)?);
    request.headers_mut().append("User-Agent", "request".parse()?);
    let res = client.execute(request).await?;
    if res.status() == StatusCode::OK {
        let data: Vec<GHRelease> = res.json().await?;
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let version = v_string_to_int(&VERSION.to_string())?;

        let os = determine_os();
        if os.is_some() {
            let os = os.unwrap();
            for x in data {
                let full = v_string_to_int(&x.tag_name)?;
                if full.parse::<u128>()? > version.parse::<u128>()? {
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

fn v_string_to_int(version: &String) -> Result<String, Box<dyn Error>>{
    let mut vers = version.split("").collect::<Vec<&str>>();
    let mut full = "".to_string();
    vers.into_iter().for_each(|x| match x.trim().parse::<u128>(){
        Ok(x) => {
            full = format!("{}{}", full, x);
        },  
        Err(x) => {}
    });
    Ok(full)
}

fn determine_os() -> Option<String> {
    let os =  env::consts::OS;
    if os.contains("darwin") {
        return Some("darwin".to_string());
    }
    if os.contains("linux") {
        return Some("linux".to_string());
    }
    return None;
}

pub async fn download_bin(asset: String) -> Result<(), Box<dyn Error>> {
    // download the artifact from github release
    let mut res = reqwest::get(asset).await.unwrap();
    assert!(res.status() == 200);

    let mut buf: Vec<u8> = Vec::new();
    buf = res.bytes().await.unwrap().to_vec();
    println!("Lenght: {}", buf.len());

    let c: &[u8] = &buf;

    let mut tmpfile = tempfile::tempfile().unwrap();
    tmpfile.write_all(c)?;
    let mut zip = zip::ZipArchive::new(tmpfile)?;
    let mut buf: Vec<u8> = Vec::new();
    zip.by_name("depploy").unwrap().read_to_end(&mut buf)?;
    
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o770)
        .open("depploy")?;
    let c: &[u8] = &buf;
    f.write_all(c)?;
    Ok(())
}


fn user_path() -> String {
    let username = whoami::username();
    match env::consts::OS {
        "macos" => return format!("/Users/{}/.local/bin/depploy", username),
        "linux" => return format!("/home/{}/.local/bin/depploy", username),
        "windows" => return format!("C:/Users/{}/.depploy", username),
        other => return String::new(),
    }
}

// #[tokio::test]
// async fn test_check_for_new_version(){
//     env::set_var("OSTYPE", "linux-gnu");
//     let res = is_new_version_available().await.unwrap();
//     println!("RES: {:?}", res);
//     download_bin(res.unwrap()).await.unwrap();
// }