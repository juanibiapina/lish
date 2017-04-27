use std::process::Command;

use parser::Parser;

pub struct Engine {
    parser: Parser,
}

use error::Result;

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
        }
    }

    pub fn run(&self, input: &str) -> Result<()> {
        let ast = self.parser.parse(input)?;

        // run command
        let mut child = Command::new(&ast.command).args(&ast.args).spawn()?;

        child.wait()?;

        Ok(())
    }
}
