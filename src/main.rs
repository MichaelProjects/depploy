mod cli;
mod io;
mod build;

use std::fmt;

use structopt::StructOpt;
use crate::build::{build_image, create_tag, push_image};
use crate::io::{get_info, load_project_file, parse_line};

#[derive(StructOpt)]
#[structopt(name = "Depploy", about = "Create docker image of cargo project with Depploy.")]
pub struct Depploy{
    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(StructOpt, PartialEq)]
#[structopt(about = "Run depploy")]
pub enum Command{
    #[structopt(about = "Build and pushes the docker image")]
    Run
}


pub fn run(){
    println!("building current application");
}

fn main(){
    let cli = Depploy::from_args();
    if cli.cmd == Command::Run {
        run();
        let config_data = load_project_file();
        let data = get_info(config_data.expect("Could not find config"));
        let tag = create_tag(&data, String::from("docker-dev.stackblog.io:5000"));
        build_image(&tag);
        push_image(&tag);
    }
}