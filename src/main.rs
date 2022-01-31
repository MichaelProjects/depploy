mod build;
mod cli;
mod conf;
mod io;
mod commands;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use commands::Command;
use crate::build::{build_image, create_tag, push_image, set_latest_tag};
use crate::conf::read_depploy_conf;
use crate::io::{build_dir, get_info, load_project_file, match_config};
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

fn main() {
    let depploy_dir =  PathBuf::from_str("/etc/depploy").unwrap();
    let cli = Depploy::from_args();

    match &cli.cmd {
        Command::Run { dir, debug } => {            
            let build_dir = build_dir(dir);
            let depploy = match read_depploy_conf(&depploy_dir){
                Ok(depploy) => depploy,
                Err(err) => panic!("{}", err),
            };

            let filename = match_config(dir);
            let config_data = match load_project_file(dir, &filename){
                Ok(data) => data,
                Err(err) => panic!("Error: {}", err),
            };
            let data = get_info(config_data);
            let tag = create_tag(&data, depploy.docker_registry); 
            
            build_image(&tag, build_dir.as_str());
            
            push_image(&tag);
            let latest_tag = set_latest_tag(&tag);
            push_image(&latest_tag);
        }
        Command::Search { host, debug } => todo!(),
    }

}
