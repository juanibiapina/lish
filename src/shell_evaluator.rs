use std::process::Command;
use std::io::ErrorKind;

use types::{LispType, LispValue, ShellExpr};
use env::{Env, env_get};
use error::{Error, Result};

pub struct ShellEvaluator;

fn get_alias_mapping(name: &str, env: &Env) -> Result<Option<String>> {
    let value = env_get(env, "ALIASES").ok();
    match value {
        Some(ref value) => {
            match *value.clone() {
                LispType::HashMap(ref data) => {
                    match data.get(name) {
                        Some(value) => {
                            match **value {
                                LispType::Strn(ref value_str) => Ok(Some(value_str.to_owned())),
                                _ => Err(Error::TypeError),
                            }
                        },
                        None => {
                            Ok(None)
                        }
                    }
                },
                _ => Err(Error::TypeError),
            }
        }
        None => {
            Ok(None)
        }
    }
}

fn resolve_alias(shell_expr: ShellExpr, env: Env) -> Result<ShellExpr> {
    let mapping = get_alias_mapping(&shell_expr.words[0], &env)?;

    match mapping {
        Some(value) => {
            let mut words = vec![];

            words.push(value);

            if let Some((_, args)) = shell_expr.words.split_first() {
                words.extend_from_slice(args);

                Ok(ShellExpr{
                    words: words
                })
            } else {
                panic!("impossible");
            }
        },
        None => Ok(shell_expr),
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

impl ShellEvaluator {
    pub fn new() -> ShellEvaluator {
        ShellEvaluator
    }

    pub fn eval(&self, shell_expr: ShellExpr, env: Env) -> Result<Option<LispValue>> {
        let expr = resolve_alias(shell_expr, env)?;

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

        Ok(None)
    }
}
