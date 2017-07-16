use lexer::Lexer;
use parser::Parser;
use error::{Error, Result};
use types::{LispValue, LispType, Program};

pub fn read(args: &[LispValue]) -> Result<LispValue> {
    match *args[0] {
        LispType::Strn(ref value) => {
            let lexer = Lexer::new();
            let mut parser = Parser::new();

            let tokens = lexer.tokenize(value)?;
            parser.add_tokens(tokens);

            match parser.parse()? {
                Program::LispProgram(value) => Ok(value),
                _ => Err(Error::TypeError),
            }
        },
        _ => Err(Error::TypeError),
    }
}
