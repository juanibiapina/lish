use error::Result;
use types::LispValue;
use env::Env;

use lisp::parser::Parser;
use lisp::lexer::tokenize;
use lisp::evaluator::Evaluator;

pub struct Engine {
    parser: Parser,
    evaluator: Evaluator,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
            evaluator: Evaluator::new(),
        }
    }

    pub fn run(&mut self, input: &str, env: Env) -> Result<LispValue> {
        let tokens = tokenize(input)?;
        self.parser.add_tokens(tokens);

        let expr = self.parser.parse()?;

        self.evaluator.eval(expr, env)
    }
}
