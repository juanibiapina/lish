use std::collections::HashMap;

use error::{Error, Result};
use types::{LispType, LispValue, hash_map};

pub fn hash(args: &[LispValue]) -> Result<LispValue> {
    let mut data = HashMap::new();

    for entry in args.chunks(2) {
        let key = match *entry[0] {
            LispType::Strn(ref value) => value.to_owned(),
            _ => return Err(Error::TypeError),
        };
        let value = entry[1].clone();
        data.insert(key, value);
    }

    Ok(hash_map(data))
}
