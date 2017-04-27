pub struct Parser;

use std::str;

use nom::IResult;

use error::Result;
use error::Error;
use ast::{Ast, ShellExpr};

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<Ast> {
        match program(input) {
            IResult::Done(_, o) => { Ok(o) },
            IResult::Error(err) => { Err(Error::ParseError(err)) },
            IResult::Incomplete(_) => { Err(Error::Incomplete) },
        }
    }
}

named!(program<&str, Ast>,
    alt!(
        shell_expr => { |v| Ast::ShellAst(v) }
    )
);

named!(shell_expr<&str, ShellExpr>,
    map!(ws!(many1!(word)), ShellExpr::new)
);

named!(word<&str, &str>,
    re_find!(r"(?:[[:word:]]|/)+")
);
