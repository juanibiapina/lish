use std;

use error::{Error, Result};
use types::{LispValue, LispType, string, nil};

pub fn env_get(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Strn(ref name) => {
            match std::env::var(name) {
                Ok(value) => Ok(string(value)),
                Err(_) => Ok(nil()),
            }
        },
        _ => Err(Error::TypeError),
    }
}

pub fn env_set(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Strn(ref name) => {
            match *args[1] {
                LispType::Strn(ref value) => {
                    std::env::set_var(name, value);
                    Ok(nil())
                },
                _ => Err(Error::TypeError),
            }
        },
        _ => Err(Error::TypeError),
    }
}
