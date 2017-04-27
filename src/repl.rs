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
                Ok(()) => {},
                Err(Error::Interrupted) => {},
                Err(Error::Eof) => {
                    break
                },
                Err(Error::IoError(e)) => {
                    println!("{}", e);
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

        self.engine.run(&line)?;

        Ok(())
    }
}
