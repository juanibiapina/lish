use lexer::Lexer;
use parser::Parser;
use evaluator::{Evaluator};
use printer::Printer;
use error::Result;
use env::Env;
use core;

pub struct Engine {
    lexer: Lexer,
    parser: Parser,
    evaluator: Evaluator,
    printer: Printer,
    env: Env,
}

impl Engine {
    pub fn new() -> Engine {
        let core_env = core::env::create();

        Engine {
            parser: Parser::new(),
            lexer: Lexer::new(),
            evaluator: Evaluator::new(),
            printer: Printer::new(),
            env: core_env,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<()> {
        let tokens = self.lexer.tokenize(input)?;
        self.parser.add_tokens(tokens);
        let ast = self.parser.parse()?;
        let result = self.evaluator.eval(ast, self.env.clone())?;

        match result {
            None => Ok(()),
            Some(lisp_expr) => {
                println!("{}", self.printer.print(&lisp_expr));
                Ok(())
            }
        }
    }
}
