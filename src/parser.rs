pub struct Parser;

use std::str;

use nom::IResult;

use error::Result;
use error::Error;
use ast::Ast;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<Ast> {
        match command(input) {
            IResult::Done(_, o) => { Ok(o) },
            IResult::Error(err) => { Err(Error::ParseError(err)) },
            IResult::Incomplete(_) => { Err(Error::Incomplete) },
        }
    }
}

named!(word<&str, &str>,
    re_find!(r"(?:[[:word:]]|/)+")
);

named!(command<&str, Ast>,
    map!(ws!(many1!(word)), Ast::new)
);
