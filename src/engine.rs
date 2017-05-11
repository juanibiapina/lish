use token::Tokens;
use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;
use evaluator::EvalResult;
use error::Result;

pub struct Engine {
    lexer: Lexer,
    parser: Parser,
    evaluator: Evaluator,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            parser: Parser::new(),
            lexer: Lexer::new(),
            evaluator: Evaluator::new(),
        }
    }

    pub fn run(&self, input: &str) -> Result<()> {
        let tokens = self.lexer.tokenize(input)?;
        let ast = self.parser.parse(Tokens::new(&tokens))?;
        let result = self.evaluator.eval(ast)?;

        match result {
            EvalResult::Done => Ok(()),
        }
    }
}
