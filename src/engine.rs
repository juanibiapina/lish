use lexer::Lexer;
use parser::Parser;
use evaluator::{Evaluator};
use printer::Printer;
use error::Result;
use core;

pub struct Engine {
    lexer: Lexer,
    parser: Parser,
    evaluator: Evaluator,
    printer: Printer,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
            lexer: Lexer::new(),
            evaluator: Evaluator::new(),
            printer: Printer::new(),
        }
    }

    pub fn run(&mut self, input: &str) -> Result<()> {
        let repl_env = core::env::create();

        let tokens = self.lexer.tokenize(input)?;
        self.parser.add_tokens(tokens);
        let ast = self.parser.parse()?;
        let result = self.evaluator.eval(ast, repl_env)?;

        match result {
            None => Ok(()),
            Some(lisp_expr) => {
                println!("{}", self.printer.print(&lisp_expr));
                Ok(())
            }
        }
    }
}
