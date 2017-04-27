use std::process::Command;

use ast::Ast;
use parser::Parser;

pub struct Engine {
    parser: Parser,
}

use error::Result;

fn eval(ast: Ast) -> Result<()> {
    match ast {
        Ast::ShellAst(shell_expr) => {
            let mut child = Command::new(&shell_expr.command).args(&shell_expr.args).spawn()?;

            child.wait()?;

            Ok(())
        }
    }
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
        }
    }

    pub fn run(&self, input: &str) -> Result<()> {
        let ast = self.parser.parse(input)?;

        eval(ast)
    }
}
