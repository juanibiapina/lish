use std::str;

use nom::IResult;

use shell::token::Token;
use error::Result;
use error::Error;

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    match lex_tokens(input) {
        IResult::Done("", tokens) => Ok(tokens),
        IResult::Done(i, _) => Err(Error::UnexpectedCharacter(i.chars().nth(0).unwrap())),
        IResult::Error(_) => Err(Error::UnknownLexerError),
        IResult::Incomplete(_) => Err(Error::UnknownLexerError),
    }
}

named!(lex_tokens<&str, Vec<Token>>, ws!(many0!(lex_ident)));

named!(lex_ident<&str, Token>,
    do_parse!(
	w: re_capture!(r#"^((?:[[:word:]]|/|-|\+|\*|%|=|\.)+)|^("(?:\\.|[^\\"])*")"#) >>
	(Token::Ident(w[0].to_owned()))
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_ident_alpha() {
        assert_eq!(tokenize("ls").unwrap(), vec![Token::Ident("ls".to_owned())]);
    }

    #[test]
    fn lex_ident_with_slash() {
        assert_eq!(tokenize("/bin/echo").unwrap(),
                   vec![Token::Ident("/bin/echo".to_owned())]);
    }

    #[test]
    fn lex_ident_with_dash() {
        assert_eq!(tokenize("-lol").unwrap(), vec![Token::Ident("-lol".to_owned())]);
    }

    #[test]
    fn lex_ident_with_dots() {
        assert_eq!(tokenize(".").unwrap(), vec![Token::Ident(".".to_owned())]);
    }

    #[test]
    fn lex_two_idents_with_dash() {
        assert_eq!(tokenize("ls -la").unwrap(),
                   vec![Token::Ident("ls".to_owned()),
                        Token::Ident("-la".to_owned())]);
    }

    #[test]
    fn lex_multiple_idents() {
        assert_eq!(tokenize("ls -l -a file").unwrap(),
                   vec![Token::Ident("ls".to_owned()),
                        Token::Ident("-l".to_owned()),
                        Token::Ident("-a".to_owned()),
                        Token::Ident("file".to_owned())]);
    }

    #[test]
    fn lex_ident_with_math_symbols() {
        assert_eq!(tokenize("+=-*%").unwrap(), vec![Token::Ident("+=-*%".to_owned())]);
    }

    #[test]
    fn lex_ident_with_quotes() {
        assert_eq!(tokenize("\"abc def\"").unwrap(), vec!(Token::Ident("\"abc def\"".to_owned())));
    }

    #[test]
    fn lex_illegal() {
        match tokenize("^").unwrap_err() {
            Error::UnexpectedCharacter(c) => {
                assert_eq!(c, '^');
            }
            _ => {
                assert!(false);
            }
        }
    }
}
