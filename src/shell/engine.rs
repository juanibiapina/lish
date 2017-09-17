use error::Result;
use env::Env;

use shell::parser::Parser;
use shell::lexer::tokenize;
use shell::evaluator::Evaluator;

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

    pub fn run(&mut self, input: &str, env: Env) -> Result<()> {
        let tokens = tokenize(input)?;
        self.parser.add_tokens(tokens);

        let expr = self.parser.parse()?;

        self.evaluator.eval(expr, env)
    }
}
