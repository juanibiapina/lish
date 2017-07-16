use std::fs::File;
use std::io::Read;

use error::{Error, Result};
use types::{LispValue, LispType, string};

pub fn slurp(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Strn(ref value) => {
            let mut buffer = String::new();
            File::open(value).and_then(|mut f| f.read_to_string(&mut buffer))?;

            Ok(string(buffer))
        },
        _ => Err(Error::TypeError),
    }
}
