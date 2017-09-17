use lisp::lexer::tokenize;
use lisp::parser::Parser;
use error::{Error, Result};
use types::{LispValue, LispType};

pub fn read(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Strn(ref value) => {
            let mut parser = Parser::new();

            let tokens = tokenize(value)?;
            parser.add_tokens(tokens);

            parser.parse()
        },
        _ => Err(Error::TypeError),
    }
}
