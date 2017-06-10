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
        let ast = self.parser.parse()?;

        self.evaluator.eval(ast, self.env.clone())
    }
}
