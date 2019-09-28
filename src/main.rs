mod args;
mod error;
mod utils;

use crate::args::{Args, Command, NewArgs, RegisterArgs};
use crate::error::Error;
use crate::utils::copy_dir_contents;

use std::path::PathBuf;
use structopt::StructOpt;

const TEMPLATE_FOLDER: &str = "templates";

fn main() {
    let args: Args = Args::from_args();

    let result = match args.command {
        Command::New(spawn_args) => spawn(spawn_args, args.config_location),
        Command::Register(register_args) => register(register_args, args.config_location),
    };

    if let Err(e) = result {
        println!("Critical error occurred!\n{:?}", e);
    }
}

fn spawn(args: NewArgs, config_location: PathBuf) -> Result<(), Error> {
    let target = args
        .target_path
        .unwrap_or(std::env::current_dir().map_err(|e| Error::Other(Box::new(e)))?);
    let template: PathBuf = config_location
        .join(TEMPLATE_FOLDER)
        .join(&args.template_name);

    let mut dir = std::fs::read_dir(template)
        .map_err(Error::Lookup)?
        .peekable();

    if dir.peek().is_none() {
        return Err(Error::Other(Box::new("cannot instantiate empty template")));
    } else {
        let target = if args.with_parent {
            target.join(&args.template_name)
        } else {
            target
        };

        copy_dir_contents(dir, target, 0, args.max_depth)?;
    }

    Ok(())
}

fn register(args: RegisterArgs, config_location: PathBuf) -> Result<(), Error> {
    let target = config_location
        .join(TEMPLATE_FOLDER)
        .join(args.template_name);

    if !args.append {
        std::fs::remove_dir_all(&target).map_err(Error::CopyError)?;
    }

    let metadata = std::fs::metadata(&args.path).map_err(Error::Lookup)?;
    if metadata.is_file() {
        let filename = args
            .path
            .file_name()
            .ok_or_else(|| Error::Other(Box::new("unable to retrieve filename")))?;

        std::fs::create_dir_all(&target).map_err(Error::CopyError)?;
        std::fs::copy(&args.path, &target.join(filename)).map_err(Error::CopyError)?;
    } else {
        copy_dir_contents(
            std::fs::read_dir(&args.path).map_err(Error::Lookup)?,
            &target,
            0,
            args.max_depth,
        )?;
    }

    Ok(())
}
