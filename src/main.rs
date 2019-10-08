//! Command line utility to create simple file snippets available to be instantiated at any time in future
//!
//! It allows to *register* and **spawn** snippets at will
//!
//! # Installation
//! `cargo install bts`
//!
//! # Usage
//! ## NEW
//! Instantiate copy of an existing snippet
//!
//! `bts new SOURCE [DESTINATION] [-w/--with-parent] [-m/--max-depth _]`
//!
//! ### **SOURCE**
//! - is snippet name.
//!
//! It's good idea to remember that for the time being snippets are stored in directories.
//! Therefore nesting is allowed and advised.
//! For example,
//! `config/mysql`, `config/psql`, `config/sqlite` are good examples of template names
//! and
//! `config_mysql`, `config_psql`, `config_sqlite` are, while correct, discouraged.
//! But hey, these are your snippets. Consider it only to be an advice.
//!
//! ### **DESTINATION**
//! - is target folder name
//!
//! Place where you want to instantiate a snippet. By default `pwd` is used.
//!
//! ### **WITH-PARENT**
//! - defines whether folder should be spawned preserving snippet name
//!
//! This means that `bts basic/template01 -w` will spawn files in `./basic/template01/` instead of `.`
//!
//! ### **MAX-DEPTH**
//! - how deep the copy should go
//!
//! Copies snippet only until given depth is reached in directory tree.
//! For example,
//! ```text
//! basic/template01/
//!                 |- file.txt
//!                 |-/ subdir
//!
//! bts basic/template01 -m 1
//! ```
//! will produce only file.txt in current directory.
//! This parameter accepts numbers in range 0..255, default value is 32.
//!
//! ## Register
//! Create new snippet from files
//!
//! `bts register TEMPLATE_NAME SOURCE [-a/--append] [-m/--max-depth]`
//!
//! ### **TEMPLATE_NAME**
//! - is snippet name
//!
//! It clears previous snippets stored at given namespace, so calling `bts basic .` will also remove `basic/template01`.
//!
//! ### **SOURCE**
//! - is a path to snippet files
//!
//! When **SOURCE** is a file, this file will be stored at snippet namespace.
//! When **SOURCE** is a directory, all contents of that directory will be stored at snippet namespace.
//!
//! ### **APPEND**
//! - defines whether we want to append to existing snippet
//!
//! This allows to create snippet only from selected files in directory.
//!
//! ### **MAX-DEPTH**
//! - defines how deep should `bts` search for files when creating a snippet.
//!
//! Accepts numbers between 0 and 255, default value is 32.
//!
//! # Contribution
//! I may accept new features, but that will only happen if I can see that it's useful.
//! It's better to create issue at github before attempting to implement something.
//! Bug fixes are always welcome.
//! You can look into `.travis.yml` for build steps, but tldr is that I will merge only features that pass
//! `cargo clippy --all-targets --all-features -- -D warnings`
//! and
//! `cargo fmt --all -- --check`
//!
//! # TBD
//! * test coverage
//! * default snippets (sample rust projects etc.)
//! * pass-through for selected applications (eg. `bts spawn cargo/bin` calls `cargo new --bin`

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
