use std::str;

use nom::IResult;

use token::Token;
use error::Result;
use error::Error;

pub struct Lexer;

impl Lexer {
    pub fn new() -> Lexer {
        Lexer
    }

    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>> {
        match lex_tokens(input) {
            IResult::Done("", tokens) => Ok(tokens),
            IResult::Done(i, _) => Err(Error::UnexpectedCharacter(i.chars().nth(0).unwrap())),
            IResult::Error(_) => Err(Error::UnknownLexerError),
            IResult::Incomplete(_) => Err(Error::UnknownLexerError),
        }
    }
}

named!(lex_tokens<&str, Vec<Token>>, ws!(many0!(lex_token)));

named!(lex_token<&str, Token>,
    alt_complete!(
	lex_lparen |
	lex_rparen |
	lex_ident
    )
);

named!(lex_lparen<&str, Token>,
    do_parse!(tag!("(") >> (Token::LParen))
);

named!(lex_rparen<&str, Token>,
    do_parse!(tag!(")") >> (Token::RParen))
);

named!(lex_ident<&str, Token>,
    do_parse!(
	w: re_capture!(r#"^((?:[[:word:]]|/|-|\+|\*|%|=|\.)+)|^("(?:\\.|[^\\"])*")"#) >>
	(Token::Ident(w[0].to_owned()))
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Result<Vec<Token>> {
        Lexer::new().tokenize(input)
    }

    #[test]
    fn lex_ident_alpha() {
        assert_eq!(lex("ls").unwrap(), vec![Token::Ident("ls".to_owned())]);
    }

    #[test]
    fn lex_ident_with_slash() {
        assert_eq!(lex("/bin/echo").unwrap(),
                   vec![Token::Ident("/bin/echo".to_owned())]);
    }

    #[test]
    fn lex_ident_with_dash() {
        assert_eq!(lex("-lol").unwrap(), vec![Token::Ident("-lol".to_owned())]);
    }

    #[test]
    fn lex_ident_with_dots() {
        assert_eq!(lex(".").unwrap(), vec![Token::Ident(".".to_owned())]);
    }

    #[test]
    fn lex_two_idents_with_dash() {
        assert_eq!(lex("ls -la").unwrap(),
                   vec![Token::Ident("ls".to_owned()),
                        Token::Ident("-la".to_owned())]);
    }

    #[test]
    fn lex_multiple_idents() {
        assert_eq!(lex("ls -l -a file").unwrap(),
                   vec![Token::Ident("ls".to_owned()),
                        Token::Ident("-l".to_owned()),
                        Token::Ident("-a".to_owned()),
                        Token::Ident("file".to_owned())]);
    }

    #[test]
    fn lex_ident_with_math_symbols() {
        assert_eq!(lex("+=-*%").unwrap(), vec![Token::Ident("+=-*%".to_owned())]);
    }

    #[test]
    fn lex_ident_with_quotes() {
        assert_eq!(lex("\"abc def\"").unwrap(), vec!(Token::Ident("\"abc def\"".to_owned())));
    }

    #[test]
    fn lex_left_parenthesis() {
        assert_eq!(lex("(").unwrap(), vec!(Token::LParen));
    }

    #[test]
    fn lex_right_parenthesis() {
        assert_eq!(lex(")").unwrap(), vec!(Token::RParen));
    }

    #[test]
    fn lex_illegal() {
        match lex("^").unwrap_err() {
            Error::UnexpectedCharacter(c) => {
                assert_eq!(c, '^');
            }
            _ => {
                assert!(false);
            }
        }
    }
}
