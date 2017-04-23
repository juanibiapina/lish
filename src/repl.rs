extern crate libc;
extern crate rustyline;

use error::Error;

use readliner::Readliner;
use readliner::RustylineReadliner;
use readliner::StdinReadliner;

pub struct Repl {
    readliner: Box<Readliner>,
}

impl Repl {
    pub fn new() -> Repl {
        let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;

        let readliner: Box<Readliner>;

        if istty {
            readliner = Box::new(RustylineReadliner::new());
        } else {
            readliner = Box::new(StdinReadliner::new());
        }

        Repl {
            readliner: readliner,
        }
    }

    pub fn run(&mut self) {
        loop {
            let line = self.readliner.readline();

            match line {
                Ok(line) => {
                    println!("{}", line);
                },
                Err(Error::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(Error::Eof) => {
                    println!("CTRL-D");
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
