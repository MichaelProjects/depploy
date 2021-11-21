mod build;
mod cli;
mod conf;
mod io;

use crate::build::{build_image, create_tag, push_image};
use crate::conf::read_depploy_conf;
use crate::io::{get_info, load_project_file, match_config, parse_line};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Depploy",
    about = "Create docker image of cargo project with Depploy."
)]
pub struct Depploy {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, PartialEq)]
#[structopt(about = "Run depploy")]
pub enum Command {
    #[structopt(about = "Build and pushes the docker image")]
    Run,
}

pub fn run() {
    println!("building current application");
}

fn main() {
    let depploy_dir = String::from("/etc/depploy/settings.toml");
    let cli = Depploy::from_args();
    if cli.cmd == Command::Run {
        run();
        let depploy = read_depploy_conf(depploy_dir).unwrap();
        let filename = match_config();
        let config_data = load_project_file(filename);
        let data = get_info(config_data.expect("Could not find config"));
        let tag = create_tag(&data, depploy.docker_registry);
        build_image(&tag);
        push_image(&tag);
    }
}
