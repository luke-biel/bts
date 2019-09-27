use std::fmt::Debug;
use std::io;

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn Debug>),
    CopyError(io::Error),
    Lookup(io::Error),
}
