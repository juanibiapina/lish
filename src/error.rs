extern crate rustyline;

use std::result;
use std::io;

use self::rustyline::error::ReadlineError;

use lisp;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Interrupted,
    Eof,
    ReadlineError(ReadlineError),
    IoError(io::Error),
    LispError(lisp::error::Error),
}

impl From<ReadlineError> for Error {
    fn from(err: ReadlineError) -> Error {
        match err {
            ReadlineError::Interrupted => Error::Interrupted,
            ReadlineError::Eof => Error::Eof,
            _ => Error::ReadlineError(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<lisp::error::Error> for Error {
    fn from(err: lisp::error::Error) -> Error {
        Error::LispError(err)
    }
}
