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
        #[structopt(short = "v")]
        debug: bool,
        #[structopt(short = "nl", long = "no-latest", help="Should not build and push the latest tag to the registry")]
        no_latest: bool
    },
    #[structopt(about = "Search for docker images in registry")]
    Search{
        host: String,
        #[structopt(short = "v")]
        debug: bool

    },
    #[structopt(about = "Generate a dockerfile with minimal instrcutions, detects language")]
    Generate{
        #[structopt(parse(from_os_str), default_value = ".")]
        dir: PathBuf,
        #[structopt(short = "l", long = "language", default_value = "", help = "Specify the language the project uses")]
        language: String
    }
}
