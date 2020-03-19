#[deny(missing_docs)]
pub mod args;
pub mod error;
pub mod template_name;
mod utils;

use crate::args::{NewArgs, RegisterArgs};
use crate::error::Error;
use crate::utils::copy_dir_contents;

use std::path::{Path, PathBuf};

const TEMPLATE_FOLDER: &str = "templates";

/// Instantiate snippet at given location
///
/// Allows to generate a snipped from preexisting template. In short, it's more complicated
/// deep copy function.
///
/// # example
/// ```
/// use bts::args::NewArgs;
/// use bts::new;
///
/// let args = NewArgs {
///     template_name: "cargo/bin".to_string(),
///     target_path: Some("/home/rustcean/Projects/my_awesome_project".into()),
///     max_depth: 8,
///     ..Default::default()
/// };
///
/// new(args, "/home/rustcean/.bts").expect("snippet was instantiated");
/// ```
pub fn new<P: AsRef<Path>>(args: NewArgs, config_location: P) -> Result<(), Error> {
    let target = args
        .target_path
        .unwrap_or(std::env::current_dir().map_err(Error::Lookup)?);
    let template: PathBuf = config_location
        .as_ref()
        .join(TEMPLATE_FOLDER)
        .join(&args.template_name.normalized());

    let mut dir = std::fs::read_dir(template)
        .map_err(Error::Lookup)?
        .peekable();

    if dir.peek().is_none() {
        return Err(Error::EmptyDirectory);
    } else {
        let target = if args.with_parent {
            target.join(&args.template_name.normalized())
        } else {
            target
        };

        copy_dir_contents(dir, target, 0, args.max_depth)?;
    }

    Ok(())
}

/// Register a new snippet
///
/// Allows to create new snipped from existing files.
///
/// # example:
/// ```
/// use bts::args::RegisterArgs;
/// use bts::register;
/// let args = RegisterArgs {
///     template_name: "cargo/amethyst".to_string(),
///     target_path: "/home/rustcean/Projects/my_template_project".into(),
///     max_depth: 8,
///     ..Default::default()
/// };
///
/// register(args, "/home/rustcean/.bts").expect("snippet was registered");
/// ```
pub fn register<P: AsRef<Path>>(args: RegisterArgs, config_location: P) -> Result<(), Error> {
    let target = config_location
        .as_ref()
        .join(TEMPLATE_FOLDER)
        .join(&args.template_name.normalized());

    if !args.append && target.exists() {
        std::fs::remove_dir_all(&target).map_err(Error::CopyError)?;
    }

    let metadata = std::fs::metadata(&args.target_path).map_err(Error::Lookup)?;
    if metadata.is_file() {
        let filename = args
            .target_path
            .file_name()
            .ok_or_else(|| Error::MissingFilename)?;

        std::fs::create_dir_all(&target).map_err(Error::CopyError)?;
        std::fs::copy(&args.target_path, &target.join(filename)).map_err(Error::CopyError)?;
    } else {
        copy_dir_contents(
            std::fs::read_dir(&args.target_path).map_err(Error::Lookup)?,
            &target,
            0,
            args.max_depth,
        )?;
    }

    Ok(())
}
