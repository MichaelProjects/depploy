use log::{debug, error, info, trace, warn};
use std::{
    collections::HashMap,
    error::Error,
    ffi::OsString,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

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
    //! iterates over each file and counts the file extensions, the one with the most will be returned.
    let mut map = HashMap::new();
    for file in files.iter() {
        let filename = file.to_str().unwrap();
        if filename.contains(".") {
            let y = filename.split(".").last().unwrap().to_string();
            let count = map.entry(y).or_insert(0);
            *count += 1;
        }
    }
    debug!("{:?}", map);
    let key_with_max_value = map.iter().max_by_key(|entry| entry.1);
    if key_with_max_value.is_some() {
        return Some(key_with_max_value.unwrap().0.clone());
    }
    return None
}

pub async fn get_project_language(depploy_dir: &PathBuf) -> Result<Vec<Language>, Box<dyn Error>> {
    let mut path = depploy_dir.clone();
    path.push("languages.json");
    let url = "https://raw.githubusercontent.com/MichaelProjects/depploy/dev/languages.json";
    if !path.exists() {
        debug!("Fetching languages from {}", url);
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let mut out =
            File::create(&path).expect("Could not create file please run again with sudo");
        io::copy(&mut body.as_bytes(), &mut out)?;
    }
    let content = fs::read_to_string(path)?;
    let languages: Vec<Language> = serde_json::from_str(content.as_str())?;
    return Ok(languages);
}

fn read_git_ignore(path: &PathBuf) -> Result<Option<Vec<String>>, Box<dyn Error>> {
    let mut to_ignore = vec!["git".to_string()];

    let mut complete_path = path.clone();
    complete_path.push(".gitignore");
    if complete_path.exists() {
        let content = fs::read_to_string(complete_path)?;
        let mut splitted: Vec<String> = content
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|c| prepare_ignore(c))
            .collect();
        // manual ignores like .git
        to_ignore.append(&mut splitted);
        return Ok(Some(to_ignore));
    }
    error!("No gitignore found, will also index build files");
    return Ok(Some(to_ignore));
}
fn prepare_ignore(s: &&str) -> String {
    //! gets a string slice and checks if it contains a slash, if so it will be removed.
    if s.contains("/") {
        let x = s.split("/").collect::<String>();
        return x;
    }
    return s.to_string();
}

pub fn create_project_analysis(path: &PathBuf) -> Result<Option<String>, Box<dyn Error>> {
    let excluded = read_git_ignore(&path)?.unwrap_or(Vec::new());
    let files = get_filesnames(&path, &excluded)?.unwrap_or(Vec::new());
    let analysis = analyzse_dir_struct(files);
    Ok(analysis)
}

#[test]
fn test_dir_structre() {
    let path = PathBuf::from_str("/home/michael/development/depploy").unwrap();
    let exclude_dirs = vec![
        "target".to_string(),
        "__pycache__".to_string(),
        "git".to_string(),
    ];
    let abc = get_filesnames(&path, &exclude_dirs);
    assert_eq!(abc.unwrap().unwrap().len(), 0)
}

#[test]
fn test_analyse_dir_structures() {
    let path = PathBuf::from_str("/home/michael/Development/depploy").unwrap();
    let exclude_dirs = vec![
        "target".to_string(),
        "__pycache__/".to_string(),
        "git".to_string(),
    ];
    let abc = get_filesnames(&path, &exclude_dirs);
    let project_lang = analyzse_dir_struct(abc.unwrap().unwrap()).unwrap();
    assert_eq!(project_lang, "rs")
}

#[tokio::test]
async fn test_load_languages() {
    let depploy_dir = PathBuf::from_str("/etc/depploy").unwrap();
    let result = get_project_language(&depploy_dir).await.unwrap();
    assert_eq!(result.len(), 0)
}

#[test]
fn test_load_gitignore() {
    let path = PathBuf::from_str("/home/michael/Development/depploy").unwrap();
    let result = read_git_ignore(&path).unwrap().unwrap();
    println!("{:?}", result);
    assert_eq!(result.len(), 0)
}
