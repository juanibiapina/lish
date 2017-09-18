use std::process::Command;
use std::io::ErrorKind;

use types::ShellExpr;
use error::{Error, Result};

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator
    }

    pub fn eval(&self, expr: ShellExpr) -> Result<()> {
        let mut command = make_command(&expr);

        let mut child = match command.spawn() {
            Ok(result) => result,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        return Err(Error::CommandNotFound(expr.words[0].to_owned()));
                    }
                    _ => {
                        return Err(Error::IoError(err));
                    }
                }
            }
        };

        child.wait()?;

        Ok(())
    }
}

fn make_command(shell_expr: &ShellExpr) -> Command {
    if let Some((cmd, args)) = shell_expr.words.split_first() {
        let mut command = Command::new(cmd);
        command.args(args);
        return command;
    } else {
        panic!("impossible");
    }
}
