mod cli;
mod io;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Depploy", about = "Create docker image of cargo project with Depploy.")]
pub struct Depploy{
    command: String,
    #[structopt(short = "o", long = "output")]
    output: String
}

fn main(){
    let cli = Depploy::from_args();
}