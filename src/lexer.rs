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
            IResult::Error(_) => Err(Error::Unknown),
            IResult::Incomplete(_) => Err(Error::Unknown),
        }
    }
}

named!(lex_tokens<&str, Vec<Token>>, ws!(many0!(lex_token)));

named!(lex_token<&str, Token>,
    alt_complete!(
	lex_lparen |
	lex_rparen |
	lex_word
    )
);

named!(lex_lparen<&str, Token>,
    do_parse!(tag!("(") >> (Token::LParen))
);

named!(lex_rparen<&str, Token>,
    do_parse!(tag!(")") >> (Token::RParen))
);

named!(lex_word<&str, Token>,
    do_parse!(
	w: re_find!(r"^(?:[[:word:]]|/|-)+") >>
	(Token::Word(w.to_string()))
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Result<Vec<Token>> {
        Lexer::new().tokenize(input)
    }

    #[test]
    fn lex_word_alpha() {
        assert_eq!(lex("ls").unwrap(), vec![Token::Word("ls".to_string())]);
    }

    #[test]
    fn lex_word_with_slash() {
        assert_eq!(lex("/bin/echo").unwrap(),
                   vec![Token::Word("/bin/echo".to_string())]);
    }

    #[test]
    fn lex_word_with_dash() {
        assert_eq!(lex("-lol").unwrap(), vec![Token::Word("-lol".to_string())]);
    }

    #[test]
    fn lex_two_words_with_dash() {
        assert_eq!(lex("ls -la").unwrap(),
                   vec![Token::Word("ls".to_string()),
                        Token::Word("-la".to_string())]);
    }

    #[test]
    fn lex_multiple_words() {
        assert_eq!(lex("ls -l -a file").unwrap(),
                   vec![Token::Word("ls".to_string()),
                        Token::Word("-l".to_string()),
                        Token::Word("-a".to_string()),
                        Token::Word("file".to_string())]);
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
