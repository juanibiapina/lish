use std::env;
use std::path::Path;

use error::Result;

pub fn run(args: Vec<String>) -> Result<()> {
    let target = if args.is_empty() {
        env::home_dir().expect("Could not get your home dir")
    } else {
        Path::new(&args[0]).to_path_buf()
    };

    env::set_current_dir(&target)?;

    Ok(())
}
