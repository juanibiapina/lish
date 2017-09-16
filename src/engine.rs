use std::fs::File;
use std::io::Read;
use std::env;

use lexer::Lexer;
use parser::Parser;
use shell_evaluator::ShellEvaluator;
use lisp_evaluator::LispEvaluator;
use error::Result;
use env::Env;
use core;
use types::{Program, LispValue};

pub struct Engine {
    lexer: Lexer,
    parser: Parser,
    shell_evaluator: ShellEvaluator,
    lisp_evaluator: LispEvaluator,
    env: Env,
}

impl Engine {
    pub fn new() -> Engine {
        let core_env = core::env::create();

        Engine {
            parser: Parser::new(),
            lexer: Lexer::new(),
            shell_evaluator: ShellEvaluator::new(),
            lisp_evaluator: LispEvaluator::new(),
            env: core_env,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<Option<LispValue>> {
        let tokens = self.lexer.tokenize(input)?;
        self.parser.add_tokens(tokens);
        let program = self.parser.parse()?;

        match program {
            Program::ShellProgram(shell_expr) => {
                self.shell_evaluator.eval(shell_expr, self.env.clone())
            }
            Program::LispProgram(lisp_expr) => {
                Ok(Some(self.lisp_evaluator.eval(lisp_expr, self.env.clone())?))
            }
            Program::EmptyProgram => {
                Ok(None)
            }
        }
    }

    pub fn load_standard_library(&mut self) -> Result<()> {
        let exe_path = env::current_exe()?;
        let mut dir = match exe_path.parent() {
            Some(path) => path.to_path_buf(),
            None => panic!("cant determine exe directory"),
        };

        dir.push("stdlib");
        dir.push("stdlib.lish");

        let stdlib_path = dir.to_str().unwrap();

        let mut buffer = String::new();
        File::open(stdlib_path).and_then(|mut f| f.read_to_string(&mut buffer))?;

        self.run(&format!("(do {})", buffer))?;

        Ok(())
    }
}
