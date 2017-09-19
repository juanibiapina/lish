use std::env;
use std::path::Path;

use error::Result;

pub fn run(args: Vec<String>) -> Result<()> {
    let target = Path::new(&args[0]);

    env::set_current_dir(&target)?;

    Ok(())
}
