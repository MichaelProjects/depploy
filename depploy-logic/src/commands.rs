use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Run depploy")]
pub enum Command {
    #[structopt(about = "Build and pushes the docker image")]
    Run{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "f", long = "file", default_value = "dockerfile", help = "Which should be used to get the instrcutions from")]
        dockerfile_name: String,
        #[structopt(short = "p", long = "public", help="Will push the image to hub.docker.com container registry")]
        public_repo: bool,
        #[structopt(short = "nl", long = "no-latest", help="Should not build and push the latest tag to the registry")]
        no_latest: bool,
        #[structopt(default_value = ".", short = "c", long = "config", help = "if you are working with workspaces and the config is in a sub directory")]
        config_file: String,
        #[structopt(short = "v", long = "verbose", help = "Show debug information about the build process")]
        debug: bool
    },
    #[structopt(about = "Search for docker images in registry")]
    /*Search{
        host: String,
        #[structopt(short = "v")]
        debug: bool

    },*/
    #[structopt(about = "Generate a dockerfile with minimal instrcutions, detects language")]
    Generate{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "l", long = "language", default_value = "", help = "Specify the language the project uses")]
        language: String,
        #[structopt(short = "v", long = "verbose", help = "Show debug information about the build process")]
        debug: bool
    },
    #[structopt(about = "Upload and manage your prototype on a server")]
    Prototype{
        #[structopt(subcommand)]
        command: Prototype
    },
    #[structopt(about = "Update the depploy-cli tool")]
    Update
}

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Prototype subcommand")]
pub enum Prototype {
    #[structopt(about = "Create a new Prototype")]
    Create{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "v", long = "verbose", help = "Show debug information about the build process")]
        debug: bool
    },
    #[structopt(about = "Upload config file")]
    Upload{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "v", long = "verbose", help = "Show debug information about the build process")]
        debug: bool
    },
    #[structopt(about = "Get a list of the running services on your prototyping server")]
    List
}