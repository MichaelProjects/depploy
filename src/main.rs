mod cli;
mod io;
use std::fmt;

use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "Depploy", about = "Create docker image of cargo project with Depploy.")]
struct Depploy{
    #[structopt(subcommand)]
    command: Command
}

#[derive(StructOpt)]
enum Command{
    run,
    build,
    push,
}
impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


fn main(){
    let cli = Depploy::from_args();
    println!("{:?}", cli);
}