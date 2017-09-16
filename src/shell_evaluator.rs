use std::process::Command;
use std::io::ErrorKind;

use types::{LispType, LispValue, ShellExpr};
use env::{Env, env_get};
use error::{Error, Result};

pub struct ShellEvaluator;

impl ShellEvaluator {
    pub fn new() -> ShellEvaluator {
        ShellEvaluator
    }

    pub fn eval(&self, shell_expr: ShellExpr, env: Env) -> Result<Option<LispValue>> {
        let resolved_command = self.resolve_alias(&shell_expr.command, env)?;

        let mut command = Command::new(&resolved_command);
        command.args(&shell_expr.args);

        let mut child = match command.spawn() {
            Ok(result) => result,
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        return Err(Error::CommandNotFound(shell_expr.command.to_owned()));
                    }
                    _ => {
                        return Err(Error::IoError(err));
                    }
                }
            }
        };

        child.wait()?;

        Ok(None)
    }

    fn resolve_alias(&self, name: &str, env: Env) -> Result<String> {
        let value = env_get(&env, "ALIASES").ok();
        match value {
            Some(ref value) => {
                match *value.clone() {
                    LispType::HashMap(ref data) => {
                        match data.get(name) {
                            Some(value) => {
                                match **value {
                                    LispType::Strn(ref value_str) => Ok(value_str.to_owned()),
                                    _ => Err(Error::TypeError),
                                }
                            },
                            None => {
                                Ok(name.to_owned())
                            }
                        }
                    },
                    _ => Err(Error::TypeError),
                }
            }
            None => {
                Ok(name.to_owned())
            }
        }
    }
}
