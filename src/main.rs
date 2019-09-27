#![feature(box_syntax)]

mod args;
mod error;

use structopt::StructOpt;

use crate::args::{Args, SpawnArgs};
use crate::error::Error;
use std::path::PathBuf;

const TEMPLATE_FOLDER: &str = "/Users/lukaszbiel/.bt/templates/";

fn main() {
    let args: Args = Args::from_args();

    match args {
        Args::Spawn(spawn_args) => { spawn(spawn_args).unwrap() }, // FIXME: display error and quit gracefully
        Args::Register(register_args) => {  },
    }
}

fn spawn(args: SpawnArgs) -> Result<(), Error> {
    let target = args.target.unwrap_or(std::env::current_dir().map_err(|e| Error::Other(box e))?);
    let template: PathBuf = PathBuf::from(TEMPLATE_FOLDER).join(&args.name);

    // FIXME: don't eat up errors quietly
    let dir = std::fs::read_dir(template)
        .map_err(|e| Error::Lookup(e))?
        .filter_map(|de| de.ok())
        .collect::<Vec<_>>();

    if dir.len() == 0 {
        return Err(Error::Other(box "cannot instantiate empty template"))
    } else if dir.len() == 1 {
        // spawn single file

        let file = &dir[0];

        std::fs::copy(file.path(), target.join(file.file_name())).map_err(|e| Error::CopyError(e))?;
    } else {
        // spawn whole folder

        let target = if args.raw {
            target
        } else {
            let new_target = target.join(&args.name);
            std::fs::create_dir_all(&new_target);
            new_target
        };

        for file in dir {
            std::fs::copy(file.path(), target.join(file.file_name())).map_err(|e| Error::CopyError(e))?;
        }
    }

    Ok(())
}
