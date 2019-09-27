use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "bt", about = "Automatic template file generator.")]
pub enum Args {
    Spawn(SpawnArgs),
    Register(RegisterArgs)
}

#[derive(StructOpt)]
pub struct SpawnArgs {
    #[structopt(long, short)]
    pub raw: bool,
    pub name: String,
    pub target: Option<PathBuf>,
}

#[derive(StructOpt)]
pub struct RegisterArgs {
    pub name: String,
    pub path: PathBuf,
}