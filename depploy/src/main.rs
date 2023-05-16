
use std::env;

use depploy_logic::build::{create_tag, set_latest_tag, build_image, push_image};
use depploy_logic::commands::{Command};
use depploy_logic::conf::read_depploy_conf;
use depploy_logic::generate::files::{get_predefined_dockerfiles, load_predefined_languages};
use depploy_logic::generate::lang::{get_project_language, create_project_analysis};
use depploy_logic::io::{build_dir, match_config, load_project_file, get_info};
use depploy_logic::prototype::logic::prototype_logic;
use depploy_logic::self_update::update::{is_new_version_available, download_bin};
use log::{error, info, warn, LevelFilter, debug};
use simple_logger::SimpleLogger;
use std::fs::{self};
use std::path::{PathBuf};
use std::str::FromStr;
use tokio;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Depploy",
    about = "Create docker image of cargo project with Depploy."
)]
#[derive(Debug)]
pub struct Depploy {
    #[structopt(subcommand)]
    pub cmd: Command,
}

fn build_depploy_path() -> String {
    let username = whoami::username();
    match env::consts::OS {
        "macos" => return format!("/Users/{}/.depploy", username),
        "linux" => return format!("/home/{}/.depploy", username),
        "windows" => return format!("C:/Users/{}/.depploy", username),
        other => return String::new(),
    }
}

#[tokio::main]
async fn main() {
    let path = build_depploy_path();
    if path == String::new() {
        panic!("OS not supported")
    }
    let depploy_dir = PathBuf::from_str(&path.as_str()).unwrap();

    // checks if the depploy directory exists
    if !depploy_dir.exists() {
        if fs::create_dir(&depploy_dir).is_err() {
            panic!(
                "Missing permisson to create depploy directory {}, run again with sudo.",
                path
            )
        }

        get_project_language(&depploy_dir).await;
        get_predefined_dockerfiles(&depploy_dir);
    }

    let cli = Depploy::from_args();
    let logger = SimpleLogger::new();

    match &cli.cmd {
        Command::Run {
            dir,
            debug,
            public_repo,
            dockerfile_name,
            config_file,
            no_latest,
            platform
        } => {
            if debug == &true {
                SimpleLogger::with_level(logger, LevelFilter::Debug)
                    .init()
                    .unwrap()
            } else {
                SimpleLogger::with_level(logger, LevelFilter::Info)
                    .init()
                    .unwrap()
            }

            // Gets the depploy config data
            let build_dir = build_dir(dir);
            let depploy = read_depploy_conf(&depploy_dir).await;
            let mut registry = String::new();
            if depploy.is_err() || public_repo == &true {
                if !depploy.is_err() {
                    let conf = depploy.unwrap();
                    if conf.registry.docker_hub_username.is_some() {
                        registry = conf.registry.docker_hub_username.unwrap();
                    }
                } else {
                    warn!("No depploy config found, will push to hub.docker.com");
                }
            } else {
                registry = depploy.unwrap().registry.docker_registry;
            }

            // load the project config file
            let mut filename = match_config(&dir);
            let mut config_data = match load_project_file(&dir, &filename) {
                Ok(data) => data,
                Err(err) => panic!("Error: {}", err),
            };

            // If there is a workspace and the config is in a sub-directory, it will override the default behavior.
            if config_file != "."{
                // Will clone and modify the path and override the read config data
                let abc = dir.clone().join(config_file);
                debug!("Specified config file: {:?}", abc);
                filename = match_config(&abc);
                config_data = match load_project_file(&abc, &filename) {
                    Ok(data) => data,
                    Err(err) => panic!("Error: {}", err),
                };
            }

            
            debug!("Config file: {:?}", config_data);
            let data = get_info(config_data);

            // uses docker
            let tag = create_tag(&data, registry);
            let name = tag.first().expect("Image name not found");
            let tag = tag.last().expect("Image tag not found");
            
            let latest_tag = set_latest_tag(name);

            build_image(&tag, build_dir.as_str(), dockerfile_name, &no_latest, &latest_tag, platform).await;

            push_image(&tag);

            // sets label to latest build and then pushes it also to the registry
            if no_latest.ne(&true) {
                push_image(&latest_tag);
            }
        }
        /*Command::Search { host, debug } => {
            println!("needs to be implemented");
        },*/
        Command::Prototype { command } => {
            prototype_logic(command).await;
         }
        Command::Generate {
            dir,
            language,
            debug,
        } => {
            if debug == &true {
                SimpleLogger::with_level(logger, LevelFilter::Debug)
                    .init()
                    .unwrap()
            } else {
                SimpleLogger::with_level(logger, LevelFilter::Info)
                    .init()
                    .unwrap()
            }
            let mut path = dir.into();
            if dir.eq(&PathBuf::from_str(".").unwrap()) {
                path = std::env::current_dir().unwrap()
            }
            if language != &String::new() {
                match load_predefined_languages(&depploy_dir, language, path.clone()) {
                    Ok(a) => a,
                    Err(err) => error!("Not able to create dockerfile: {:?}", err),
                };
            } else {
                let detected = match create_project_analysis(&path) {
                    Ok(lang) => lang,
                    Err(err) => {
                        warn!("Was not able to get project language");
                        None
                    }
                };
                if detected.is_some() {
                    let lang = detected.unwrap();
                    info!("Detected {} as your project language", &lang);

                    match load_predefined_languages(&depploy_dir, &lang, path.clone()) {
                        Ok(a) => a,
                        Err(err) => error!("Not able to create dockerfile: {:?}", err),
                    };
                }
            }
        }
        Command::Update => {
            let res = is_new_version_available().await.unwrap();
            download_bin(res.unwrap()).await.unwrap();
        }
    }
}
