pub struct Engine;

use lisp::error::Result;

impl Engine {
    pub fn new() -> Engine {
        Engine
    }

    pub fn run(&self, input: &str) -> Result<String> {
        Ok(input.to_string())
    }
}
