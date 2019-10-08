#![allow(clippy::let_and_return)]

use std::path::PathBuf;
use structopt::StructOpt;

const TEMPLATE_FOLDER_NAME: &str = ".bts";

#[derive(StructOpt)]
#[structopt(name = "bts", about = "Automatic template file generator.")]
/// Generate file snippets at will
pub struct Args {
    #[structopt(env = "BT_HOME", default_value = Self::default_template_folder())]
    /// Location of snippets storage, default path is `~/.bt/templates` for osx and linux and `%HOME%\bt\templates` for windows
    pub config_location: PathBuf,
    #[structopt(flatten)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Instantiate a snippet
    New(NewArgs),
    /// Create new snippet
    Register(RegisterArgs),
}

#[derive(StructOpt)]
pub struct NewArgs {
    #[structopt(long, short)]
    /// Spawn with whole path leading to given snippet
    pub with_parent: bool,
    /// Path to snippet
    pub template_name: String,
    /// Target dir, `pwd` by default
    pub target_path: Option<PathBuf>,
    #[structopt(long, short, default_value = "32")]
    /// Max depth that should be copied
    pub max_depth: u8,
}

#[derive(StructOpt)]
pub struct RegisterArgs {
    /// New snippet name
    pub template_name: String,
    /// Path to single file or directory from which contents new snippet should be created
    pub path: PathBuf,
    #[structopt(long, short)]
    /// Don't delete previous snippet when creating new one, only append
    pub append: bool,
    #[structopt(long, short, default_value = "32")]
    /// Max depth that should be copied
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
