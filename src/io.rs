use std::{fs, path::PathBuf};

pub struct Config {
    pub version: String,
    pub name: String,
}
impl Config {
    fn new(version: &str, name: &str) -> Config {
        Config {
            version: String::from(version),
            name: String::from(name),
        }
    }
}

pub fn match_config(dir: &PathBuf) -> String {
    //!
    //! has given filenames and if one of the files a in the specified directory, then it will return the name of the config file.
    //! 
    //! Example:
    //! ```
    //! match_config(&PathBuf::from("/home/user/project/"));
    //! ```
    let config_names = vec!["Cargo.toml", "conf.toml"];
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
    panic!("Could not find config")
}

pub fn load_project_file(path: &PathBuf, filename: &String) -> std::io::Result<String> {
    let filename = format!("{}/{}", path.to_str().expect("lul"), filename);
    fs::read_to_string(filename)
}

pub fn get_info(config_data: String) -> Config {
    let mut content = Config::new("", "");
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

pub fn parse_line(line: &str) -> String {
    let vec = line.split("=").collect::<Vec<&str>>();
    return vec[1].replace("\"", "").to_string();
}

pub fn build_dir(dir: &PathBuf) -> String {
    let mut dir_str = dir.to_str().expect("Path");
    if dir_str.ne(".") {
        let a = format!("{}/.",dir_str );
        return a
    }
    ".".to_string()
}