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

        println!("{}", line);

        Ok(())
    }
}
