use nom;
use nom::IResult;

use lexer::{lex_word, lex_lparen, lex_rparen};
use error::Result;
use error::Error;
use ast;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<ast::Program> {
        match program(input) {
            IResult::Done("", o) => Ok(o),
            IResult::Done(i, _) => Err(Error::UnexpectedCharacter(i.chars().next().unwrap())),
            IResult::Error(err) => {
                match err {
                    nom::Err::Code(_) => Err(Error::ParseError),
                    nom::Err::Node(_, _) => Err(Error::ParseError),
                    nom::Err::Position(_, rest) => {
                        Err(Error::UnexpectedCharacter(rest.chars().next().unwrap()))
                    }
                    nom::Err::NodePosition(_, rest, _) => {
                        Err(Error::UnexpectedCharacter(rest.chars().next().unwrap()))
                    }
                }
            }
            IResult::Incomplete(_) => Err(Error::Incomplete),
        }
    }
}

named!(program<&str, ast::Program>,
    alt!(
        shell_expr => { |v| ast::Program::ShellProgram(v) } |
        lisp_expr => { |v| ast::Program::LispProgram(v) }
    )
);

named!(shell_expr<&str, ast::ShellExpr>,
    map!(ws!(many1!(word)), ast::ShellExpr::from_words)
);

named!(lisp_expr<&str, ast::LispExpr>,
    alt!(
        lisp_list |
        lisp_atom
    )
);

named!(word<&str, ast::Word>,
    do_parse!(
        w: lex_word >>
        (ast::Word(w.to_owned()))
    )
);

named!(lisp_list<&str, ast::LispExpr>,
    do_parse!(
        exprs: ws!(delimited!(
                       lex_lparen,
                       many0!(lisp_expr),
                       lex_rparen
                  )) >>
        (ast::LispExpr::List(exprs))
    )
);

named!(lisp_atom<&str, ast::LispExpr>,
    alt!(
        symbol
    )
);

named!(symbol<&str, ast::LispExpr>,
    do_parse!(
        w: lex_word >>
        (ast::LispExpr::Symbol(w.to_owned()))
    )
);

#[cfg(test)]
mod tests {
    use ast::*;
    use super::*;

    fn parse(input: &str) -> Result<Program> {
        Parser::new().parse(input)
    }

    fn assert_input_with_ast(input: &str, expected: Program) {
        let program = parse(input).unwrap();

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_shell_expr() {
        let input = "ls -la file";
        let expected = Program::ShellProgram(ShellExpr {
                                                 command: Word("ls".to_string()),
                                                 args: vec![Word("-la".to_string()),
                                                            Word("file".to_string())],
                                             });

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_simple_lisp_expression() {
        let input = "(ls a b)";
        let expected = Program::LispProgram(LispExpr::List(vec![LispExpr::Symbol("ls".to_owned()),
                                                     LispExpr::Symbol("a".to_owned()),
                                                     LispExpr::Symbol("b".to_owned())]));

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_nested_lisp_expression() {
        let input = "((ls a) b)";
        let expected = Program::LispProgram(LispExpr::List(vec![LispExpr::List(vec![
                        LispExpr::Symbol("ls".to_owned()),
                        LispExpr::Symbol("a".to_owned())
                    ]),
                                                     LispExpr::Symbol("b".to_owned())]));

        assert_input_with_ast(input, expected);
    }
}
