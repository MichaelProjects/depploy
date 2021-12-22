use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Run depploy")]
pub enum Command {
    #[structopt(about = "Build and pushes the docker image")]
    Run{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "v")]
        debug: bool,
    },
    #[structopt(about = "Search for docker images in registry")]
    Search{
        host: String,
        #[structopt(short = "v")]
        debug: bool

    }
}
