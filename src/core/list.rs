use error::Result;
use types::{LispValue, list};

pub fn list_c(args: &[LispValue]) -> Result<LispValue> {
    Ok(list(args.to_vec()))
}
