#![allow(clippy::let_and_return)]

use std::path::PathBuf;
use structopt::StructOpt;

const TEMPLATE_FOLDER_NAME: &str = ".bt";

#[derive(StructOpt)]
#[structopt(name = "bt", about = "Automatic template file generator.")]
pub struct Args {
    #[structopt(env = "BT_HOME", default_value = Self::default_template_folder())]
    pub config_location: PathBuf,
    #[structopt(flatten)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    New(NewArgs),
    Register(RegisterArgs),
}

#[derive(StructOpt)]
pub struct NewArgs {
    #[structopt(long, short)]
    pub with_parent: bool,
    pub template_name: String,
    pub target_path: Option<PathBuf>,
    #[structopt(long, short, default_value = "32")]
    pub max_depth: u8,
}

#[derive(StructOpt)]
pub struct RegisterArgs {
    pub template_name: String,
    pub path: PathBuf,
    #[structopt(long, short)]
    pub append: bool,
    #[structopt(long, short, default_value = "32")]
    pub max_depth: u8,
}

impl Args {
    pub fn default_template_folder() -> &'static str {
        match dirs::home_dir() {
            Some(dir) => Box::leak(Box::new(dir.join(TEMPLATE_FOLDER_NAME)))
                .to_str()
                .expect("Couldn't convert to str"),
            None => panic!("Couldn't locate home directory"),
        }
    }
}
