use crate::error::Error;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

pub fn copy_dir_contents(
    from: impl Iterator<Item = io::Result<DirEntry>>,
    to: impl AsRef<Path>,
    depth: u8,
    max_depth: u8,
) -> Result<(), Error> {
    if depth > max_depth {
        return Ok(());
    }

    let to = to.as_ref();
    if !to.exists() {
        std::fs::create_dir_all(&to).map_err(Error::CopyError)?;
    }

    for entry in from {
        let entry = match entry {
            Ok(dir_entry) => dir_entry,
            Err(e) => return Err(Error::Lookup(e)),
        };

        if entry.metadata().map_err(Error::Lookup)?.is_file() {
            std::fs::copy(entry.path(), to.join(entry.file_name())).map_err(Error::CopyError)?;
        } else {
            copy_dir_contents(
                std::fs::read_dir(entry.path()).map_err(Error::Lookup)?,
                to.join(entry.file_name()),
                depth + 1,
                max_depth,
            )?;
        }
    }

    Ok(())
}
