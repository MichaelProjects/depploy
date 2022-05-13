use std::{
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

fn analyzse_dir_struct(files: Vec<OsString>) {
    for file in files.iter() {
        let filename = file.to_str().unwrap();
        if filename.contains(".") {
            let y = filename.split(".").last();
            println!("{:?}", y.unwrap());
        }
    }
}

pub async fn get_project_language(
    mut depploy_dir: PathBuf,
) -> Result<Vec<Language>, Box<dyn Error>> {
    depploy_dir.push("languages.json");
    println!("{:?}", depploy_dir);
    if !depploy_dir.exists() {
        println!("Fetching");
        let response = reqwest::get(
            "https://raw.githubusercontent.com/MichaelProjects/depploy/dev/languages.json",
        ).await?;
        let body = response.text().await?;
        let mut out = File::create(&depploy_dir).expect("Could not create file please run again with sudo");
        io::copy(&mut body.as_bytes(), &mut out)?;
    }
    let content = fs::read_to_string(depploy_dir)?;
    let languages: Vec<Language> = serde_json::from_str(content.as_str())?;
    return Ok(languages);
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
    analyzse_dir_struct(abc.unwrap().unwrap());
    assert_eq!(false, true)
}

#[tokio::test]
async fn test_load_languages() {
    let depploy_dir = PathBuf::from_str("/etc/depploy").unwrap();
    let result = get_project_language(depploy_dir).await.unwrap();
    assert_eq!(result.len(), 0)
}
