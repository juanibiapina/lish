use nom;
use nom::IResult;
use nom::ErrorKind;
use nom::Needed;

use token::{Token, Tokens};
use error::Result;
use error::Error;
use ast;

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(&self, tokens: Tokens) -> Result<ast::Program> {
        match program(tokens) {
            IResult::Done(i, o) => {
                if i.tok.len() == 0 {
                    Ok(o)
                } else {
                    Err(Error::UnexpectedToken(i.tok[0].clone()))
                }
            }
            IResult::Error(err) => {
                match err {
                    nom::Err::Code(_) => Err(Error::ParseError),
                    nom::Err::Node(_, _) => Err(Error::ParseError),
                    nom::Err::Position(_, tokens) => {
                        Err(Error::UnexpectedToken(tokens.tok[0].clone()))
                    }
                    nom::Err::NodePosition(_, tokens, _) => {
                        Err(Error::UnexpectedToken(tokens.tok[0].clone()))
                    }
                }
            }
            IResult::Incomplete(_) => Err(Error::Incomplete),
        }
    }
}

macro_rules! tag_token (
  ($i: expr, $tag: expr) => (
    {
        let (i1, t1) = try_parse!($i, take!(1));
        if t1.tok.len() == 0 {
            IResult::Incomplete::<_,_>(Needed::Size(1))
        } else {
            if t1.tok[0] == $tag {
                IResult::Done(i1, t1)
            } else {
                IResult::Error(error_position!(ErrorKind::Tag, $i))
            }
        }
    }
  );
);

macro_rules! parse_word (
    ($i: expr,) => (
        {
            let (i1, t1) = try_parse!($i, take!(1));
            if t1.tok.len() == 0 {
                IResult::Error(error_position!(ErrorKind::Tag, $i))
            } else {
                match t1.tok[0].clone() {
                    Token::Word(name) => IResult::Done(i1, ast::Word(name)),
                    _ => IResult::Error(error_position!(ErrorKind::Tag, $i)),
                }
            }
        }
  );
);

named!(program<Tokens, ast::Program>,
    alt!(
        shell_expr => { |v| ast::Program::ShellProgram(v) } |
        lisp_expr => { |v| ast::Program::LispProgram(v) }
    )
);

named!(word<Tokens, ast::Word>, parse_word!() );

named!(shell_expr<Tokens, ast::ShellExpr>,
    map!(many1!(word), ast::ShellExpr::from_words)
);

named!(lisp_expr<Tokens, ast::LispExpr>,
    alt!(
        lisp_list |
        lisp_atom
    )
);

named!(lisp_list<Tokens, ast::LispExpr>,
    do_parse!(
        exprs: delimited!(
                   tag_token!(Token::LParen),
                   many0!(lisp_expr),
                   tag_token!(Token::RParen)
               ) >>
        (ast::LispExpr::List(exprs))
    )
);

named!(lisp_atom<Tokens, ast::LispExpr>,
    alt!(
        symbol
    )
);

named!(symbol<Tokens, ast::LispExpr>,
    do_parse!(
        w: parse_word!() >>
        (ast::LispExpr::Symbol(w.0))
    )
);

#[cfg(test)]
mod tests {
    use ast::LispExpr;
    use super::*;
    use lexer::Lexer;

    fn parse(input: &str) -> Result<ast::Program> {
        let tokens = Lexer::new().tokenize(input).unwrap();

        Parser::new().parse(Tokens::new(&tokens))
    }

    fn assert_input_with_ast(input: &str, expected: ast::Program) {
        let program = parse(input).unwrap();

        assert_eq!(program, expected);
    }

    #[test]
    fn parse_shell_expr() {
        let input = "ls -la file";
        let expected = ast::Program::ShellProgram(ast::ShellExpr {
                                                      command: ast::Word("ls".to_string()),
                                                      args: vec![ast::Word("-la".to_string()),
                                                                 ast::Word("file".to_string())],
                                                  });

        assert_input_with_ast(input, expected);
    }

    #[test]
    fn parse_simple_lisp_expression() {
        let input = "(ls a b)";
        let expected = ast::Program::LispProgram(
            LispExpr::List(
              vec!(LispExpr::Symbol("ls".to_owned()), LispExpr::Symbol("a".to_owned()), LispExpr::Symbol("b".to_owned()))
            )
        );

        assert_input_with_ast(input, expected);
    }
}
