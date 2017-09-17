use std::collections::VecDeque;

use token::Token;
use error::Result;
use error::Error;
use types;

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

    pub fn parse(&mut self) -> Result<types::ShellExpr> {
        self.read_shell()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
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
    use shell::lexer::tokenize;

    fn parse(input: &str) -> Result<ShellExpr> {
        let tokens = tokenize(input).unwrap();

        let mut parser = Parser::new();
        parser.add_tokens(tokens);

        parser.parse()
    }

    fn assert_input_with_ast(input: &str, expected: ShellExpr) {
        let parsed = parse(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse_shell_expr() {
        let input = "ls -la file";
        let expected = ShellExpr {
            words: vec![
                "ls".to_owned(),
                "-la".to_owned(),
                "file".to_owned()
            ],
        };

        assert_input_with_ast(input, expected);
    }
}
