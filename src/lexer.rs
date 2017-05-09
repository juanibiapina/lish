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
            IResult::Done(_, tokens) => Ok(tokens),
            IResult::Error(_) => Err(Error::Unknown),
            IResult::Incomplete(_) => Err(Error::Unknown),
        }
    }
}

named!(lex_tokens<&str, Vec<Token>>, ws!(many0!(lex_token)));

named!(lex_token<&str, Token>,
    alt_complete!(
	lex_word |
	lex_illegal
    )
);

named!(lex_word<&str, Token>,
    do_parse!(
	w: re_find!(r"^(?:[[:word:]]|/|-)+") >>
	(Token::Word(w.to_string()))
    )
);

named!(lex_illegal<&str, Token>,
    do_parse!(
	c: take_s!(1) >>
	(Token::Illegal(c.to_string()))
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
    fn lex_illegal() {
        assert_eq!(lex("^").unwrap(), vec![Token::Illegal("^".to_string())]);
    }
}
