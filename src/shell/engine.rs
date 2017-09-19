use error::{Error, Result};
use env::{Env, env_get};
use types::LispType;

use shell::parser::Parser;
use shell::token::Token;
use shell::lexer::tokenize;
use shell::evaluator::Evaluator;

pub struct Engine {
    parser: Parser,
    evaluator: Evaluator,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
            evaluator: Evaluator::new(),
        }
    }

    pub fn run(&mut self, input: &str, env: Env) -> Result<()> {
        let tokens = tokenize(input)?;

        let tokens = resolve_alias(tokens, env)?;

        self.parser.add_tokens(tokens);

        let expr = self.parser.parse()?;

        self.evaluator.eval(expr)
    }
}

fn resolve_alias(tokens: Vec<Token>, env: Env) -> Result<Vec<Token>> {
    if tokens.len() < 1 {
        return Ok(tokens);
    }

    let mapping = get_alias_mapping(&tokens[0], &env)?;

    match mapping {
        None => Ok(tokens),
        Some(value) => {
            let mut words = tokenize(&value)?;

            if let Some((_, args)) = tokens.split_first() {
                words.extend_from_slice(args);

                Ok(words)
            } else {
                panic!("impossible");
            }
        }
    }
}

fn get_alias_mapping(token: &Token, env: &Env) -> Result<Option<String>> {
    let name = match token {
        &Token::Ident(ref data) => data,
    };

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
