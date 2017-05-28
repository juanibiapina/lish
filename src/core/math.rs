use error::{Error, Result};
use types::{LispType, LispValue};
use types;

pub fn add(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Integer(i1) => {
            match *args[1] {
                LispType::Integer(i2) => {
                    Ok(types::integer(i1 + i2))
                }
                _ => {
                    Err(Error::TypeError)
                }
            }
        }
        _ => {
            Err(Error::TypeError)
        }
    }
}

pub fn sub(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Integer(i1) => {
            match *args[1] {
                LispType::Integer(i2) => {
                    Ok(types::integer(i1 - i2))
                }
                _ => {
                    Err(Error::TypeError)
                }
            }
        }
        _ => {
            Err(Error::TypeError)
        }
    }
}
