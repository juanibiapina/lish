use parser::Parser;
use evaluator::{Evaluator, EvalResult};
use printer::Printer;
use error::Result;

pub struct Engine {
    parser: Parser,
    evaluator: Evaluator,
    printer: Printer,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
            evaluator: Evaluator::new(),
            printer: Printer::new(),
        }
    }

    pub fn run(&self, input: &str) -> Result<()> {
        let ast = self.parser.parse(input)?;
        let result = self.evaluator.eval(ast)?;

        match result {
            EvalResult::Done => Ok(()),
            EvalResult::LispExpr(lisp_expr) => {
                println!("{}", self.printer.print(&lisp_expr));
                Ok(())
            }
        }
    }
}
