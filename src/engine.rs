use std::process::Command;

use token::Tokens;
use ast::Program;
use lexer::Lexer;
use parser::Parser;
use error::Result;

pub struct Engine {
    lexer: Lexer,
    parser: Parser,
}

fn eval(program: Program) -> Result<()> {
    match program {
        Program::ShellProgram(shell_expr) => {
            let mut child = Command::new(&shell_expr.command.0).args(shell_expr.args.iter().map(|a| &a.0)).spawn()?;

            child.wait()?;

            Ok(())
        }
    }
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
            lexer: Lexer::new(),
        }
    }

    pub fn run(&self, input: &str) -> Result<()> {
        let tokens = self.lexer.tokenize(input)?;
        let ast = self.parser.parse(Tokens::new(&tokens))?;

        eval(ast)
    }
}
