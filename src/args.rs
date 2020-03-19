#![allow(clippy::let_and_return)]

//! Data representation of args passable to the app

use crate::template_name::TemplateName;
use std::path::PathBuf;
use structopt::StructOpt;

const TEMPLATE_FOLDER_NAME: &str = ".bts";

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "bts", about = "Automatic template file generator.")]
/// Generate file snippets at will
pub struct Args {
    #[structopt(env = "BT_HOME", default_value = Self::default_template_folder())]
    /// Location of snippets storage
    pub config_location: PathBuf,

    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub command: Command,
}

#[derive(StructOpt, Debug, Clone)]
/// All available commands in an app
pub enum Command {
    /// Instantiate a snippet
    New(NewArgs),
    /// Create new snippet
    Register(RegisterArgs),
}

#[derive(StructOpt, Default, Debug, Clone)]
/// Arguments necessary to spawn a snippet
pub struct NewArgs {
    #[structopt(long, short)]
    /// Spawn with whole path leading to given snippet
    pub with_parent: bool,
    /// Path to snippet
    pub template_name: TemplateName,
    /// Target dir, `pwd` by default
    pub target_path: Option<PathBuf>,
    #[structopt(long, short, default_value = "32")]
    /// Max depth that should be copied
    pub max_depth: u8,
}

#[derive(StructOpt, Default, Debug, Clone)]
/// Arguments necessary to create new snippet
pub struct RegisterArgs {
    /// New snippet name
    pub template_name: TemplateName,
    /// Path to single file or directory from which contents new snippet should be created
    pub target_path: PathBuf,
    #[structopt(long, short)]
    /// Don't delete previous snippet when creating new one, only append
    pub append: bool,
    #[structopt(long, short, default_value = "32")]
    /// Max depth that should be copied
    pub max_depth: u8,
}

impl Args {
    /// Returns default location where all templates would be stored
    pub fn default_template_folder() -> &'static str {
        match dirs::home_dir() {
            // This is not looking nice, but program is anyway short-lived and
            // by making it static this way we avoid adding lazy-static crate dependency
            Some(dir) => Box::leak(Box::new(dir.join(TEMPLATE_FOLDER_NAME)))
                .to_str()
                .expect("Couldn't convert to str"),
            None => panic!("Couldn't locate home directory"),
        }
    }
}
