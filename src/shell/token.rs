use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Ident(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Ident(ref s) => write!(f, "{}", s),
        }
    }
}
