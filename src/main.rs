mod build;
mod commands;
mod conf;
mod generate;
mod io;
mod http;
mod models;

use std::fs::{self, Permissions};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use generate::lang::get_project_language;
use tokio;

use crate::build::{build_image, create_tag, push_image, set_latest_tag};
use crate::conf::read_depploy_conf;
use crate::io::{build_dir, get_info, load_project_file, match_config};
use commands::Command;
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

#[tokio::main]
async fn main() {
    let depploy_dir =  PathBuf::from_str("/etc/depploy").unwrap();
    
    // checks if the depploy directory exists
    if !depploy_dir.exists(){
        if fs::create_dir(&depploy_dir).is_err(){
         panic!("Missing permisson to create depploy directory /etc/depploy, run again with sudo or create it yourself.")
        }
        if fs::set_permissions(&depploy_dir, Permissions::from_mode(0o777)).is_err(){
            panic!("Couldn't set permissions")
        }
    }
    let cli = Depploy::from_args();

    match &cli.cmd {
        Command::Run {
            dir,
            debug,
            dockerfile_name,
            no_latest,
        } => {
            // Gets the depploy config data
            let build_dir = build_dir(dir);
            let depploy = match read_depploy_conf(&depploy_dir) {
                Ok(depploy) => depploy,
                Err(err) => panic!("{}", err),
            };

            // load the project config file

            let filename = match_config(dir);
            let config_data = match load_project_file(dir, &filename) {
                Ok(data) => data,
                Err(err) => panic!("Error: {}", err),
            };
            let data = get_info(config_data);

            // uses docker
            let tag = create_tag(&data, depploy.docker_registry);
            let name = tag.first().expect("Image name not found");
            let tag = tag.last().expect("Image tag not found");

            build_image(&tag, build_dir.as_str(), dockerfile_name);

            push_image(&tag);

            // sets label to latest build and then pushes it also to the registry
            if no_latest.ne(&true) {
                let latest_tag = set_latest_tag(name, &tag);
                push_image(&latest_tag);
            }
        } 
        /*Command::Search { host, debug } => {
            println!("needs to be implemented");
        },*/
        Command::Generate { dir, language } => {
            let languages = get_project_language(depploy_dir).await;
        }
    }
}
