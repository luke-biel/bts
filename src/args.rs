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
    pub with_parent: bool,
    pub template_name: String,
    pub target_path: Option<PathBuf>,
}

#[derive(StructOpt)]
pub struct RegisterArgs {
    pub template_name: String,
    pub path: PathBuf,
}