mod build;
mod cli;
mod conf;
mod io;

use std::path::PathBuf;

use crate::build::{build_image, create_tag, push_image};
use crate::conf::read_depploy_conf;
use crate::io::{get_info, load_project_file, match_config, parse_line};
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

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Run depploy")]
pub enum Command {
    #[structopt(about = "Build and pushes the docker image")]
    Run{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "d")]
        debug: bool,
    },
}


fn main() {
    let depploy_dir = String::from("/etc/depploy/settings.toml");
    let cli = Depploy::from_args();

    match &cli.cmd {
        Command::Run { dir, debug } => {
            let depploy = read_depploy_conf(depploy_dir).unwrap();
            let filename = match_config();
            let config_data = load_project_file(filename);
            let data = get_info(config_data.expect("Could not find config"));
            let tag = create_tag(&data, depploy.docker_registry);
            build_image(&tag, &dir.to_str().unwrap().to_string());
            push_image(&tag);
        }
    }

}
