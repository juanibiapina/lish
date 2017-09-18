use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Ident(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Ident(ref s) => write!(f, "{}", s),
        }
    }
}
