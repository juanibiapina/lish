#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Ident(String),
}
