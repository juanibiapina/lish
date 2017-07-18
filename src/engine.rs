use std::fs::File;
use std::io::Read;
use std::env;

use lexer::Lexer;
use parser::Parser;
use evaluator::{Evaluator};
use error::Result;
use env::Env;
use core;
use types::LispValue;

pub struct Engine {
    lexer: Lexer,
    parser: Parser,
    evaluator: Evaluator,
    env: Env,
}

impl Engine {
    pub fn new() -> Engine {
        let core_env = core::env::create();

        Engine {
            parser: Parser::new(),
            lexer: Lexer::new(),
            evaluator: Evaluator::new(),
            env: core_env,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<Option<LispValue>> {
        let tokens = self.lexer.tokenize(input)?;
        self.parser.add_tokens(tokens);
        let program = self.parser.parse()?;

        self.evaluator.eval(program, self.env.clone())
    }

    pub fn load_standard_library(&mut self) -> Result<()> {
        let exe_path = env::current_exe()?;
        let mut dir = match exe_path.parent() {
            Some(path) => path.to_path_buf(),
            None => panic!("cant determine exe directory"),
        };

        dir.push("stdlib");
        dir.push("stdlib.lish");

        let stdlib_path = dir.to_str().unwrap();

        let mut buffer = String::new();
        File::open(stdlib_path).and_then(|mut f| f.read_to_string(&mut buffer))?;

        self.run(&format!("(do {})", buffer))?;

        Ok(())
    }
}
