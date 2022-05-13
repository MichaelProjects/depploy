use std::{
    collections::HashMap,
    error::Error,
    ffi::OsString,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};
use log::{info, debug, error, trace, warn};

use crate::models::language::Language;

fn get_filesnames(
    path: &PathBuf,
    exclude_dirs: &Vec<String>,
) -> Result<Option<Vec<OsString>>, Box<dyn Error>> {
    //! gets a path to a directory and recrusively gets root files but also all file from the subdirectorys.
    //! appends the paths to a list and returns it
    let mut list = Vec::new();
    // check if the current path is a directory
    if path.is_dir() {
        let str_path = path.to_str().unwrap();
        for x in exclude_dirs.iter() {
            if str_path.split("/").last().unwrap().contains(x) {
                return Ok(None);
            }
        }

        for element in fs::read_dir(path)? {
            let y = element?;
            // calls it self here again
            let mut result = get_filesnames(&y.path(), exclude_dirs)?;
            if result.is_some() {
                list.append(&mut result.unwrap());
            }
        }
    } else {
        list.push(path.as_os_str().into())
    }
    return Ok(Some(list));
}

fn analyzse_dir_struct(files: Vec<OsString>) -> Option<String> {
    let mut map = HashMap::new();
    for file in files.iter() {
        let filename = file.to_str().unwrap();
        if filename.contains(".") {
            let y = filename.split(".").last().unwrap().to_string();
            let count = map.entry(y).or_insert(0);
            *count += 1;
        }
    }
    let key_with_max_value = map.iter().max_by_key(|entry | entry.1).unwrap();
    Some(key_with_max_value.0.clone())   
}

pub async fn get_project_language(
    mut depploy_dir: PathBuf,
) -> Result<Vec<Language>, Box<dyn Error>> {
    depploy_dir.push("languages.json");
    let url = "https://raw.githubusercontent.com/MichaelProjects/depploy/dev/languages.json";
    if !depploy_dir.exists() {
        debug!("Fetching languages from {}", url);
        let response = reqwest::get(
            url
        )
        .await?;
        let body = response.text().await?;
        let mut out =
            File::create(&depploy_dir).expect("Could not create file please run again with sudo");
        io::copy(&mut body.as_bytes(), &mut out)?;
    }
    let content = fs::read_to_string(depploy_dir)?;
    let languages: Vec<Language> = serde_json::from_str(content.as_str())?;
    return Ok(languages);
}

fn read_git_ignore(path: &PathBuf) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
    let mut complete_path = path.clone();
    complete_path.push(".gitignore");
    if complete_path.exists() {
        let content = fs::read_to_string(complete_path)?;
        let splitted = content.split("\n").collect::<Vec<&str>>();
        Ok(Some(splitted));
    }
    error!("No gitignore found, will also index build files");
    return Ok(None)
}

#[test]
fn test_dir_structre() {
    let path = PathBuf::from_str("/home/michaell/development/depploy").unwrap();
    let exclude_dirs = vec![
        "target".to_string(),
        "__pycache__/".to_string(),
        "git".to_string(),
    ];
    let abc = get_filesnames(&path, &exclude_dirs);
    assert_eq!(abc.unwrap().unwrap().len(), 0)
}

#[test]
fn test_analyse_dir_structures() {
    let path = PathBuf::from_str("/home/michaell/development/depploy").unwrap();
    let exclude_dirs = vec![
        "target".to_string(),
        "__pycache__/".to_string(),
        "git".to_string(),
    ];
    let abc = get_filesnames(&path, &exclude_dirs);
    let project_lang = analyzse_dir_struct(abc.unwrap().unwrap());
    assert_eq!(project_lang.unwrap(), "rs")
}

#[tokio::test]
async fn test_load_languages() {
    let depploy_dir = PathBuf::from_str("/etc/depploy").unwrap();
    let result = get_project_language(depploy_dir).await.unwrap();
    assert_eq!(result.len(), 0)
}

#[test]
fn test_load_gitignore(){
    let path = PathBuf::from_str("/home/michaell/development/depploy").unwrap();
    read_git_ignore(&path).unwrap().unwrap();
    assert!(false)
}