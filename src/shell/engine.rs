pub struct Engine;

use shell::error::Result;

impl Engine {
    pub fn new() -> Engine {
        Engine
    }

    pub fn run(&self, input: &str) -> Result<String> {
        Ok(input.to_string())
    }
}
