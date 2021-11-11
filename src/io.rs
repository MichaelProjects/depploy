pub struct Config{
    pub version: String,
    pub name: String
}
impl Config{
    fn new(version: &str, name: &str) -> Config{
        Config{version: String::from(version), name: String::from(name)}
    }
}

pub fn load_project_file() -> std::io::Result<String> {
    std::fs::read_to_string("./Cargo.toml")
}

pub fn get_info(config_data: String) -> Config{
    let mut content = Config::new("", "");
    for line in config_data.lines() {
        if line.contains("version") { content.version = parse_line(line); }
        else if line.contains("name") { content.name = parse_line(line); }

    }
    content
}

pub fn parse_line(line: &str) -> String{
    let vec = line.split("=").collect::<Vec<&str>>();
    println!("{}", &vec[1]);
    return vec[1].to_string()
}

