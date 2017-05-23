use std::process::Command;

use ast::Program;
use ast::LispExpr;
use error::Result;

pub enum EvalResult {
    Done,
    LispExpr(LispExpr),
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator
    }

    pub fn eval(&self, program: Program) -> Result<EvalResult> {
        match program {
            Program::ShellProgram(shell_expr) => {
                let mut child = Command::new(&shell_expr.command)
                    .args(&shell_expr.args)
                    .spawn()?;

                child.wait()?;

                Ok(EvalResult::Done)
            }
            Program::LispProgram(lisp_expr) => {
                Ok(EvalResult::LispExpr(lisp_expr))
            }
        }
    }
}
