extern crate rustyline;

use error::Error;
use error::Result;

use readliner::create_readliner;
use readliner::Readliner;
use engine::Engine;

pub struct Repl {
    readliner: Box<Readliner>,
    engine: Engine,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            readliner: create_readliner(),
            engine: Engine::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.rep() {
                Ok(()) => {}
                Err(Error::Interrupted) => {}
                Err(Error::Eof) => break,
                Err(Error::ReadlineError(e)) => {
                    println!("{}", e);
                }
                Err(Error::IoError(e)) => {
                    println!("{}", e);
                }
                Err(Error::UnexpectedCharacter(c)) => {
                    println!("Unexpected character `{}`", c);
                }
                Err(Error::ParseError) => {
                    println!("Parse error");
                }
                Err(Error::CommandNotFound(command)) => {
                    println!("Command not found: {}", command);
                }
                Err(Error::UndefinedBinding(name)) => {
                    println!("Undefined binding: {}", name);
                }
                Err(Error::ApplyEmptyList) => {
                    println!("apply error: unable to apply empty list");
                }
                Err(Error::ApplyNonFunction) => {
                    println!("Trying to apply non function");
                }
                Err(Error::TypeError) => {
                    println!("Type error");
                }
                Err(Error::UnknownLexerError) => {
                    println!("Unknown lexer error");
                }
            }
        }
    }

    fn rep(&mut self) -> Result<()> {
        let line = self.readliner.readline()?;

        self.engine.run(&line)?;

        self.readliner.add_history_entry(&line);

        Ok(())
    }
}
