extern crate regex;

use std::collections::VecDeque;

use token::Token;
use error::Result;
use error::Error;
use types;

lazy_static! {
    static ref INTEGER_REGEX: regex::Regex = regex::Regex::new(r"^-?[0-9]+$").unwrap();
    static ref STRING_REGEX: regex::Regex = regex::Regex::new(r#"^".*"$"#).unwrap();
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

    pub fn parse(&mut self) -> Result<types::Program> {
        self.read_program()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.front().cloned()
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        let token = self.next();

        match token {
            Some(token) => {
                if token == expected {
                    Ok(())
                } else {
                    Err(Error::ParseError)
                }
            },
            None => {
                Err(Error::ParseError)
            }
        }
    }

    fn read_program(&mut self) -> Result<types::Program> {
        let token = self.peek();

        match token {
            Some(token) => {
                match token {
                    Token::LParen => {
                        Ok(types::Program::LispProgram(self.read_lisp()?))
                    }
                    _ => {
                        Ok(types::Program::ShellProgram(self.read_shell()?))
                    }
                }
            },
            None => {
                Ok(types::Program::EmptyProgram)
            }
        }
    }

    fn read_lisp(&mut self) -> Result<types::LispValue> {
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

    fn read_list(&mut self) -> Result<types::LispValue> {
        self.expect(Token::LParen)?;

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

        self.expect(Token::RParen)?;

        Ok(types::list(forms))
    }

    fn read_atom(&mut self) -> Result<types::LispValue> {
        let token = self.next();

        match token {
            Some(Token::Ident(token)) => {
                if INTEGER_REGEX.is_match(&token) {
                    let value: i64 = token.parse().unwrap();
                    Ok(types::integer(value))
                } else if token == "nil" {
                    Ok(types::nil())
                } else if STRING_REGEX.is_match(&token) {
                    Ok(types::string(token[1..token.len()-1].to_owned()))
                } else {
                    Ok(types::symbol(token))
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

    fn read_shell(&mut self) -> Result<types::ShellExpr> {
        let mut words = vec![];

        while let Some(token) = self.next() {
            match token {
                Token::Ident(token) => {
                    words.push(token);
                },
                _ => {
                    return Err(Error::ParseError);
                }
            }
        }

        Ok(types::ShellExpr {
            words: words,
        })
    }
}

#[cfg(test)]
mod tests {
    use types::*;
    use super::*;
    use lexer::tokenize;

    fn parse(input: &str) -> Result<Program> {
        let tokens = tokenize(input).unwrap();

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
            words: vec![
                "ls".to_owned(),
                "-la".to_owned(),
                "file".to_owned()
            ],
        });

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_nil() {
        let input = "(nil)";
        let expected = Program::LispProgram(
            types::list(
                vec![
                    types::nil(),
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_simple_lisp_expression() {
        let input = "(ls a b)";
        let expected = Program::LispProgram(
            types::list(
                vec![
                    types::symbol("ls".to_owned()),
                    types::symbol("a".to_owned()),
                    types::symbol("b".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_nested_lisp_expression() {
        let input = "((ls -42) b)";
        let expected = Program::LispProgram(
            types::list(
                vec![
                    types::list(
                        vec![
                            types::symbol("ls".to_owned()),
                            types::integer(-42)
                        ]
                    ),
                    types::symbol("b".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_string() {
        let input = "(\"string value\")";
        let expected = Program::LispProgram(
            types::list(
                vec![
                    types::string("string value".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_two_strings_on_same_line() {
        let input = "(\"a\" \"b\")";
        let expected = Program::LispProgram(
            types::list(
                vec![
                    types::string("a".to_owned()),
                    types::string("b".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_string_with_quotes() {
        let input = "(\"a \\\" \" \"b\")";
        let expected = Program::LispProgram(
            types::list(
                vec![
                    types::string("a \\\" ".to_owned()),
                    types::string("b".to_owned())
                ]
            )
        );

        assert_input_with_ast(input, expected);
    }
}
