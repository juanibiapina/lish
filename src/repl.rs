extern crate rustyline;

use error::Error;

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
            let line = self.readliner.readline();

            match line {
                Ok(line) => {
                    println!("{}", line);
                },
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
}
