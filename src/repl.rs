use error::Error;
use error::Result;

use readliner::create_readliner;
use readliner::Readliner;
use engine::Engine;
use printer::Printer;
use types::LispValue;

pub struct Repl {
    readliner: Box<Readliner>,
    engine: Engine,
    printer: Printer,
}

impl Repl {
    pub fn new() -> Repl {
        let mut engine = Engine::new();

        match engine.load_standard_library() {
            Ok(()) => {},
            Err(e) => panic!("{:?}", e),
        };

        Repl {
            readliner: create_readliner(),
            engine: engine,
            printer: Printer::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.read_eval() {
                Ok(None) => {}
                Ok(Some(value)) => {
                    println!("{}", self.printer.print(&value, true));
                }
                Err(Error::Interrupted) => {}
                Err(Error::Eof) => break,
                Err(Error::ReadlineError(e)) => {
                    println!("readline error: {}", e);
                }
                Err(Error::IoError(e)) => {
                    println!("io error: {}", e);
                }
                Err(Error::UnexpectedCharacter(c)) => {
                    println!("lexer error: unexpected character `{}`", c);
                }
                Err(Error::ParseError) => {
                    println!("parser error");
                }
                Err(Error::CommandNotFound(command)) => {
                    println!("shell error: command not found: {}", command);
                }
                Err(Error::UndefinedBinding(name)) => {
                    println!("lookup error: undefined binding: {}", name);
                }
                Err(Error::ApplyEmptyList) => {
                    println!("apply error: unable to apply empty list");
                }
                Err(Error::ApplyNonFunction(value)) => {
                    println!("apply error: expected function, got: {}", self.printer.print(&value, true));
                }
                Err(Error::TypeError) => {
                    println!("type error");
                }
                Err(Error::UnknownLexerError) => {
                    println!("lexer error: unknown");
                }
            }
        }
    }

    fn read_eval(&mut self) -> Result<Option<LispValue>> {
        let line = self.readliner.readline()?;

        self.readliner.add_history_entry(&line);

        self.engine.run(&line)
    }
}
