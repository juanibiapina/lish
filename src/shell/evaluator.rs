use std::process::Command;
use std::io::ErrorKind;

use shell::builtins;
use types::ShellExpr;
use error::{Error, Result};

pub struct Evaluator;

enum Builtin {
    Echo,
    Cd,
}

enum CommandType {
    Builtin(Builtin),
    External(String),
}

fn resolve_command_type(cmd: String) -> CommandType {
    match cmd.as_ref() {
        "cd" => CommandType::Builtin(Builtin::Cd),
        "echo" => CommandType::Builtin(Builtin::Echo),
        _ => CommandType::External(cmd),
    }
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator
    }

    pub fn eval(&self, mut expr: ShellExpr) -> Result<()> {
        let args = expr.words.split_off(1);
        let cmd = expr.words.remove(0);

        let cmd_type = resolve_command_type(cmd);

        match cmd_type {
            CommandType::External(name) => run_external(name, args),
            CommandType::Builtin(name) => run_builtin(name, args),
        }

    }
}

fn run_external(cmd: String, args: Vec<String>) -> Result<()> {
    let mut command = Command::new(&cmd);
    command.args(&args);

    let mut child = match command.spawn() {
        Ok(result) => result,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    return Err(Error::CommandNotFound(cmd));
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

fn run_builtin(builtin: Builtin, args: Vec<String>) -> Result<()> {
    match builtin {
        Builtin::Cd => builtins::cd::run(args),
        Builtin::Echo => builtins::echo::run(args),
    }
}
