mod build;
mod commands;
mod conf;
mod generate;
mod io;
mod models;
use std::env;

use generate::files::get_predefined_dockerfiles;
use generate::lang::{create_project_analysis, get_project_language};
use log::{error, info, trace, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::fs::{self, Permissions};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio;

use crate::build::{build_image, create_tag, push_image, set_latest_tag};
use crate::conf::read_depploy_conf;
use crate::generate::files::load_predefined_languages;
use crate::io::{build_dir, get_info, load_project_file, match_config};
use commands::Command;
use structopt::StructOpt;
use text_io::scan;

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
    match (env::consts::OS) {
        "macos" => return format!("/Users/{}/.depploy", username),
        "linux" => return format!("/home/{}/.depploy", username),
        "windows" => return format!("/home/{}/.depploy", username),
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
            no_latest,
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
                    if conf.docker_hub_username.is_some() {
                        registry = conf.docker_hub_username.unwrap();
                    }
                } else {
                    warn!("No depploy config found, will push to hub.docker.com");
                }
            } else {
                registry = depploy.unwrap().docker_registry;
            }

            // load the project config file

            let filename = match_config(dir);
            let config_data = match load_project_file(dir, &filename) {
                Ok(data) => data,
                Err(err) => panic!("Error: {}", err),
            };
            let data = get_info(config_data);

            // uses docker
            let tag = create_tag(&data, registry);
            let name = tag.first().expect("Image name not found");
            let tag = tag.last().expect("Image tag not found");
            
            let latest_tag = set_latest_tag(name, &tag);

            build_image(&tag, build_dir.as_str(), dockerfile_name, &no_latest, &latest_tag);

            push_image(&tag);

            // sets label to latest build and then pushes it also to the registry
            if no_latest.ne(&true) {
                push_image(&latest_tag);
            }
        }
        /*Command::Search { host, debug } => {
            println!("needs to be implemented");
        },*/
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
    }
}
