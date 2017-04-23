extern crate rustyline;

use error::Error;
use error::Result;

use readliner::create_readliner;
use readliner::Readliner;

pub struct Repl {
    readliner: Box<Readliner>,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            readliner: create_readliner(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.rep() {
                Ok(()) => {},
                Err(Error::Interrupted) => {},
                Err(Error::Eof) => {
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
    }

    fn rep(&mut self) -> Result<()> {
        let line = self.readliner.readline()?;

        self.process(line)
    }

    fn process(&mut self, input: String) -> Result<()> {
        if input.starts_with("(") {
            self.process_lisp(input)
        } else {
            self.process_shell(input)
        }
    }

    fn process_lisp(&mut self, input: String) -> Result<()> {
        println!("{}", input);

        Ok(())
    }

    fn process_shell(&mut self, input: String) -> Result<()> {
        println!("{}", input);

        Ok(())
    }
}
