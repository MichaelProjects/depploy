use std::{fs};
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

pub struct Config{
    pub version: String,
    pub name: String
}
impl Config{
    fn new(version: &str, name: &str) -> Config{
        Config{version: String::from(version), name: String::from(name)}
    }
}


pub fn match_config() -> String {
    let config_names = vec!["Cargo.toml", "conf.toml"];
    let mut found = String::new();
    let current_dir = std::env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap(){
        let entry = entry.unwrap();
        let path = entry.path();
        let str_path = path.to_str().unwrap();
        for config_name in config_names.iter(){
            if str_path.contains(config_name){
                found = str_path.to_string();
                return found;
            }
        }
    }
    panic!("Could not find config")
}


pub fn load_project_file() -> std::io::Result<String> {
    let filename = match_config();
    let content = fs::read_to_string(filename);
    content
}

pub fn get_info(config_data: String) -> Config{
    let mut content = Config::new("", "");
    for line in config_data.lines() {
        if line.contains("version") { content.version = parse_line(line); break;}
        else if line.contains("name") { content.name = parse_line(line);}
    }
    content
}

pub fn parse_line(line: &str) -> String{
    let vec = line.split("=").collect::<Vec<&str>>();
    return vec[1].replace("\"", "").to_string();
}


