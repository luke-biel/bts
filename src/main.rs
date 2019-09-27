#![feature(box_syntax)]

mod args;
mod error;

use structopt::StructOpt;

use crate::args::{Args, SpawnArgs, RegisterArgs};
use crate::error::Error;
use std::path::{PathBuf, Path};
use std::fs::DirEntry;
use std::io;

const TEMPLATE_FOLDER: &str = ".bt/templates/";

fn main() {
    let args: Args = Args::from_args();

    let result = match args {
        Args::Spawn(spawn_args) => { spawn(spawn_args) },
        Args::Register(register_args) => { register(register_args) },
    };

    if let Err(e) = result {
        println!("Critical error occured!\n{:?}", e);
    }
}

fn spawn(args: SpawnArgs) -> Result<(), Error> {
    let target = args.target_path.unwrap_or(std::env::current_dir().map_err(|e| Error::Other(box e))?);
    let template: PathBuf = template_folder()?.join(&args.template_name);

    let mut dir = collect_dir_entries(template)?.peekable();

    if dir.peek().is_none() {
        return Err(Error::Other(box "cannot instantiate empty template"))
    } else {
        let target = if args.with_parent {
            target.join(&args.template_name)
        } else {
            target
        };

        copy_dir_contents(dir, target)?;
    }

    Ok(())
}

fn register(args: RegisterArgs) -> Result<(), Error> {
    let target = template_folder()?.join(args.template_name);

    let metadata = std::fs::metadata(&args.path).map_err(Error::Lookup)?;
    if metadata.is_file() {
        std::fs::create_dir_all(&target).map_err(Error::CopyError)?;
        std::fs::copy(&args.path, &target).map_err(Error::CopyError)?;
    } else {
        copy_dir_contents(collect_dir_entries(&args.path)?, &target)?;
    }

    Ok(())
}

fn collect_dir_entries(path: impl AsRef<Path>) -> Result<impl Iterator<Item= io::Result<DirEntry>>, Error> {
    std::fs::read_dir(path)
        .map_err(Error::Lookup)
}

fn template_folder() -> Result<PathBuf, Error> {
    Ok(
        dirs::home_dir()
            .ok_or_else(|| Error::Other(box "couldn't locate home folder"))?
            .join(TEMPLATE_FOLDER)
    )
}

fn copy_dir_contents(from: impl Iterator<Item= io::Result<DirEntry>>, to: impl AsRef<Path>) -> Result<(), Error> {
    let to = to.as_ref();
    if !to.exists() {
        std::fs::create_dir_all(&to).map_err(Error::CopyError)?;
    }

    for entry in from {
        let entry = match entry {
            Ok(dir_entry) => { dir_entry },
            Err(e) => { return Err(Error::Lookup(e)) },
        };

        if entry.metadata().map_err(Error::Lookup)?.is_file() {
            std::fs::copy(entry.path(), to.join(entry.file_name())).map_err(Error::CopyError)?;
        } else {
            copy_dir_contents(collect_dir_entries(entry.path())?, to.join(entry.file_name()))?;
        }
    }

    Ok(())
}
