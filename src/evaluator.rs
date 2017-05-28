use std::process::Command;
use std::io::ErrorKind;

use types::{self, Program, LispType, LispValue};
use env::{Env, env_get};
use error::{Error, Result};

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator
    }

    pub fn eval(&self, program: Program, env: Env) -> Result<Option<LispValue>> {
        match program {
            Program::ShellProgram(shell_expr) => {
                let mut command = Command::new(&shell_expr.command);
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
            Program::LispProgram(lisp_expr) => {
                Ok(Some(self.eval_lisp_expr(lisp_expr, env)?))
            }
        }
    }

    fn eval_lisp_expr(&self, lisp_expr: LispValue, env: Env) -> Result<LispValue> {
        match *lisp_expr {
            LispType::List(ref exprs) => {
                let exprs = self.eval_list(exprs, env)?;
                match exprs.as_slice() {
                    &[] => {
                        Ok(lisp_expr.clone())
                    }
                    &[ref head, ref tail..] => {
                        self.apply(head.clone(), tail.clone())
                    }
                }
            }
            _ => Ok(self.eval_ast(lisp_expr.clone(), env)?),
        }
    }

    fn apply(&self, f: LispValue, args: &[LispValue]) -> Result<LispValue> {
        match *f.clone() {
            LispType::NativeFunction(ref data) => {
                (data.body)(args)
            }
            _ => {
                Err(Error::ApplyNonFunction)
            }
        }
    }

    fn eval_ast(&self, lisp_expr: LispValue, env: Env) -> Result<LispValue> {
        match *lisp_expr {
            LispType::Symbol(ref s) => {
                env_get(&env, s)
            },
            LispType::List(ref list) => {
                Ok(types::list(self.eval_list(list, env)?))
            },
            _ => Ok(lisp_expr.clone()),
        }
    }

    fn eval_list(&self, list: &[LispValue], env: Env) -> Result<Vec<LispValue>> {
        let mut e = vec![];

        for expr in list {
            e.push(self.eval_lisp_expr(expr.clone(), env.clone())?);
        }

        Ok(e)
    }
}

#[cfg(test)]
mod tests {
    use env::{env_new, env_set};
    use super::*;
    use core;

    fn eval(expr: LispValue, env: Env) -> Result<Option<LispValue>> {
        let evaluator = Evaluator::new();

        evaluator.eval(types::Program::LispProgram(expr), env)
    }

    #[test]
    fn eval_lisp_integer() {
        assert_eq!(eval(types::integer(1), env_new()).unwrap().unwrap(), types::integer(1));
    }

    #[test]
    fn eval_lisp_symbol() {
        let env = env_new();
        env_set(&env, "name", types::integer(42));

        assert_eq!(eval(types::symbol("name".to_owned()), env).unwrap().unwrap(), types::integer(42));
    }

    #[test]
    fn eval_lisp_list() {
        let env = core::env::create();

        assert_eq!(
            eval(
                types::list(
                    vec![
                        types::symbol("+".to_owned()),
                        types::integer(3),
                        types::integer(2),
                    ]
                ), env).unwrap().unwrap(),
            types::integer(5)
        );
    }

    #[test]
    fn eval_nested_list() {
        let env = core::env::create();

        assert_eq!(
            eval(
                types::list(
                    vec![
                        types::symbol("+".to_owned()),
                        types::integer(3),
                        types::list(
                            vec![
                                types::symbol("-".to_owned()),
                                types::integer(4),
                                types::integer(2),
                            ]
                        ),
                    ]
                ), env).unwrap().unwrap(),
            types::integer(5)
        );
    }
}
