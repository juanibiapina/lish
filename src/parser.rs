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
            IResult::Error(_) => { Err(Error::ParseError) },
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
    re_find!(r"^(?:[[:word:]]|/|-)+")
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_command() {
        assert_eq!(program("ls"), IResult::Done("", Ast::ShellAst(ShellExpr { command: "ls".to_string(), args: Vec::new() })));
    }

    #[test]
    fn parse_command_with_slash() {
        assert_eq!(program("/bin/echo"), IResult::Done("", Ast::ShellAst(ShellExpr { command: "/bin/echo".to_string(), args: Vec::new() })));
    }

    #[test]
    fn parse_command_with_one_argument() {
        assert_eq!(program("ls -la"), IResult::Done("", Ast::ShellAst(ShellExpr { command: "ls".to_string(), args: vec!("-la".to_string()) })));
    }

    #[test]
    fn parse_command_with_several_arguments() {
        assert_eq!(program("ls -l -a file"), IResult::Done("", Ast::ShellAst(ShellExpr { command: "ls".to_string(), args: vec!("-l".to_string(), "-a".to_string(), "file".to_string()) })));
    }
}
