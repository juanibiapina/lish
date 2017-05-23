extern crate regex;

use std::collections::VecDeque;

use token::Token;
use error::Result;
use error::Error;
use ast;

lazy_static! {
    static ref INTEGER_REGEX: regex::Regex = regex::Regex::new(r"^-?[0-9]+$").unwrap();
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: VecDeque::new(),
        }
    }

    pub fn add_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens.extend(tokens);
    }

    pub fn parse(&mut self) -> Result<ast::Program> {
        self.read_program()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.front().cloned()
    }

    fn read_program(&mut self) -> Result<ast::Program> {
        let token = self.peek();

        match token {
            Some(token) => {
                match token {
                    Token::LParen => {
                        Ok(ast::Program::LispProgram(self.read_lisp()?))
                    }
                    _ => {
                        Ok(ast::Program::ShellProgram(self.read_shell()?))
                    }
                }
            },
            None => {
                Err(Error::ParseError)
            }
        }
    }

    fn read_lisp(&mut self) -> Result<ast::LispExpr> {
        let token = self.peek();

        match token {
            Some(token) => {
                match token {
                    Token::LParen => {
                        Ok(self.read_list()?)
                    }
                    _ => {
                        Ok(self.read_atom()?)
                    }
                }
            },
            None => {
                Err(Error::ParseError)
            }
        }
    }

    fn read_list(&mut self) -> Result<ast::LispExpr> {
        let token = self.next();

        match token {
            Some(token) => {
                match token {
                    Token::LParen => { }
                    _ => {
                        return Err(Error::ParseError);
                    }
                }
            },
            None => {
                return Err(Error::ParseError);
            }
        }

        let mut forms = vec![];

        loop {
            let token = self.peek();

            match token {
                Some(token) => {
                    match token {
                        Token::RParen => {
                            break;
                        }
                        _ => {
                            forms.push(self.read_lisp()?);
                        }
                    }
                }
                None => {
                    return Err(Error::ParseError);
                }
            }
        }

        self.next();

        Ok(ast::LispExpr::List(forms))
    }

    fn read_atom(&mut self) -> Result<ast::LispExpr> {
        let token = self.next();

        match token {
            Some(Token::Ident(token)) => {
                if INTEGER_REGEX.is_match(&token) {
                    let value: i64 = token.parse().unwrap();
                    Ok(ast::LispExpr::Integer(value))
                } else {
                    Ok(ast::LispExpr::Symbol(token))
                }
            },
            Some(_) => {
                Err(Error::ParseError)
            },
            None => {
                Err(Error::ParseError)
            }
        }

    }

    fn read_shell(&mut self) -> Result<ast::ShellExpr> {
        let command = self.read_ident()?;

        let mut args = vec![];

        while let Some(token) = self.next() {
            match token {
                Token::Ident(token) => {
                    args.push(token);
                },
                _ => {
                    return Err(Error::ParseError);
                }
            }
        }

        Ok(ast::ShellExpr {
            command: command,
            args: args,
        })
    }

    fn read_ident(&mut self) -> Result<String> {
        let token = self.next();

        match token {
            Some(Token::Ident(token)) => {
                Ok(token)
            },
            Some(_) => {
                Err(Error::ParseError)
            },
            None => {
                Err(Error::ParseError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::*;
    use super::*;
    use lexer::Lexer;

    fn parse(input: &str) -> Result<Program> {
        let tokens = Lexer::new().tokenize(input).unwrap();

        let mut parser = Parser::new();
        parser.add_tokens(tokens);

        parser.parse()
    }

    fn assert_input_with_ast(input: &str, expected: Program) {
        let program = parse(input).unwrap();

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_shell_expr() {
        let input = "ls -la file";
        let expected = Program::ShellProgram(ShellExpr {
            command: "ls".to_owned(),
            args: vec![
                "-la".to_owned(),
                "file".to_owned()
            ],
        });

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_simple_lisp_expression() {
        let input = "(ls a b)";
        let expected = Program::LispProgram(
            LispExpr::List(
                vec![
                    LispExpr::Symbol("ls".to_owned()),
                    LispExpr::Symbol("a".to_owned()),
                    LispExpr::Symbol("b".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_nested_lisp_expression() {
        let input = "((ls -42) b)";
        let expected = Program::LispProgram(
            LispExpr::List(
                vec![
                    LispExpr::List(
                        vec![
                            LispExpr::Symbol("ls".to_owned()),
                            LispExpr::Integer(-42)
                        ]
                    ),
                    LispExpr::Symbol("b".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }
}
