use std::process::Command;

use ast::Program;
use error::Result;

pub enum EvalResult {
    Done,
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator
    }

    pub fn eval(&self, program: Program) -> Result<EvalResult> {
        match program {
            Program::ShellProgram(shell_expr) => {
                let mut child = Command::new(&shell_expr.command.0)
                    .args(shell_expr.args.iter().map(|a| &a.0))
                    .spawn()?;

                child.wait()?;

                Ok(EvalResult::Done)
            }
        }
    }
}
