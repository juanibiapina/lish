use std::process::Command;

pub struct Engine;

use shell::error::Result;

impl Engine {
    pub fn new() -> Engine {
        Engine
    }

    pub fn run(&self, input: &str) -> Result<()> {
        // tokenize
        let parts = input.split(" ");

        let mut result = vec!();
        for part in parts {
            result.push(part.to_string());
        }

        // parse - split command and args
        let mut iter = result.iter();

        let command = iter.next().unwrap().to_string();
        let mut args = vec!();
        for part in iter {
            args.push(part.to_string());
        }

        // run command
        let mut child = Command::new(&command).args(&args).spawn()?;

        child.wait()?;

        Ok(())
    }
}
