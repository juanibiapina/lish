use std::process::Command;
use std::io::ErrorKind;

use types::{self, Program, LispType, LispValue};
use env::{Env, env_get, env_set};
use error::{Error, Result};

enum FormType {
    Def,
    Function,
}

impl FormType {
    pub fn from(name: &str) -> FormType {
        match name {
            "def" => FormType::Def,
            _ => FormType::Function,
        }
    }
}

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
                self.apply(exprs.as_slice(), env)
            }
            _ => Ok(self.eval_ast(lisp_expr.clone(), env)?),
        }
    }

    fn apply(&self, list: &[LispValue], env: Env) -> Result<LispValue> {
        match list {
            &[] => {
                Ok(types::list(vec![]))
            }
            &[ref head, ref tail..] => {
                let form_type = match *head.clone() {
                    LispType::Symbol(ref name) => {
                        FormType::from(name)
                    }
                    _ => {
                        return Err(Error::ApplyNonFunction(head.clone()));
                    }
                };

                match form_type {
                    FormType::Def => self.eval_def(tail, env),
                    FormType::Function => self.eval_function(list, env),
                }
            }
        }
    }

    fn eval_def(&self, args: &[LispValue], env: Env) -> Result<LispValue> {
        let a1 = args[0].clone();
        let a2 = args[1].clone();
        match *a1 {
            LispType::Symbol(ref name) => {
                let value = self.eval_lisp_expr(a2, env.clone())?;
                env_set(&env, name, value);

                Ok(types::nil())
            },
            _ => panic!("def! of non-symbol"),
        }
    }

    fn eval_function(&self, list: &[LispValue], env: Env) -> Result<LispValue> {
        match list {
            &[] => {
                Err(Error::ApplyEmptyList)
            }
            &[ref head, ref tail..] => {
                let evaluated_head = self.eval_lisp_expr(head.clone(), env.clone())?;
                match *evaluated_head {
                    LispType::NativeFunction(ref data) => {
                        let evaluated_tail = self.eval_list(tail, env)?;

                        (data.body)(&evaluated_tail)
                    }
                    _ => {
                        Err(Error::ApplyNonFunction(evaluated_head.clone()))
                    }
                }
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

    #[test]
    fn eval_def() {
        let env = env_new();

        eval(
            types::list(
                vec![
                    types::symbol("def".to_owned()),
                    types::symbol("a".to_owned()),
                    types::integer(1),
                ]
            ),
            env.clone()
        ).unwrap().unwrap();

        assert_eq!(env_get(&env, "a").unwrap(), types::integer(1));
    }
}
