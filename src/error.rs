extern crate rustyline;

use std::result;
use std::io;

use self::rustyline::error::ReadlineError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // input
    Interrupted,
    Eof,

    // lexer
    UnexpectedCharacter(char),

    // parser
    ParseError,

    // crates
    ReadlineError(ReadlineError),
    IoError(io::Error),

    // shell
    CommandNotFound(String),

    // lisp
    UndefinedBinding(String),
    ApplyNonFunction,
    TypeError,

    // unknown
    Unknown,
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
