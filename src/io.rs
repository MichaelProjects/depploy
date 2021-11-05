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
    for line in config_data.lines(){
        match line{
            "version" => content.version = parse_line(line),
            "" => content.name = parse_line(line),
            _ => println!("Nothing to parse")

        }
    }
    content
}

pub fn parse_line(line: &str) -> String{
    let vec = line.split("=").collect::<Vec<&str>>();
    println!("{}", &vec[2]);
    return vec[2].to_string()
}