use std::{fs::{self}, path::{Path, PathBuf}, env};

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ProjectConf {
    pub version: String,
    pub name: String,
}

impl ProjectConf {
    fn new(version: &str, name: &str) -> ProjectConf {
        ProjectConf {
            version: String::from(version),
            name: String::from(name),
        }
    }
}

pub fn build_depploy_path() -> String{
    let username = whoami::username();
    match env::consts::OS{
        "macos" => return format!("/Users/{}/.depploy", username),
        "linux" => return format!("/home/{}/.depploy", username),
        "windows" => return format!("/home/{}/.depploy", username),
        other => return String::new()
    }
}

pub fn match_config(dir: &PathBuf) -> String {
    //!
    //! has given filenames and if one of the files a in the specified directory, then it will return the name of the config file.
    let config_names = vec!["Cargo.toml"];
    let result = find_conf(dir, config_names);
    
    if result.eq(""){
        let configs = vec!["conf", "config"];
        let result = find_conf(dir, configs);
        if result.eq(""){
            panic!("Could not find config file");
        }
        return result;
    }
    return result;
    
}

fn find_conf(dir: &PathBuf, config_names: Vec<&str>) -> String{
    //! searches in the given dir for the specified config files, if the files are found, then it will return the name of the config file.
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let str_path = path.to_str().unwrap();
        for config_name in config_names.iter() {
            if str_path.contains(config_name) {
                return str_path.split("/").last().unwrap().to_string();
            }
        }
    }
    return String::new();
}

pub fn load_project_file(path: &PathBuf, filename: &String) -> std::io::Result<String> {
    //! takes the path and the filename and reads the given file as string and returns it.
    let filename = format!("{}/{}", path.to_str().expect("lul"), filename);
    fs::read_to_string(filename)
}

pub fn parse_line(line: &str) -> String {
    let vec = line.split("=").collect::<Vec<&str>>();
    return vec[1].replace("\"", "").to_string();
}

pub fn get_info(config_data: String) -> ProjectConf {
    let mut content = ProjectConf::new("", "");
    for line in config_data.lines() {
        if line.contains("version") {
            content.version = parse_line(line);
            break;
        }
        if line.contains("name") {
            content.name = parse_line(line);
        }
    }
    content
}

pub fn build_dir(dir: &Path) -> String {
    let dir_str = dir.to_str().expect("Path");
    if dir_str.ne(".") {
        let a = format!("{}",dir_str );
        return a
    }
    ".".to_string()
}