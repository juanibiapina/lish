extern crate rustyline;

use error::Error;
use error::Result;

use readliner::create_readliner;
use readliner::Readliner;
use lisp::engine::Engine as LispEngine;
use shell::engine::Engine as ShellEngine;

pub struct Repl {
    readliner: Box<Readliner>,
    lisp_engine: LispEngine,
    shell_engine: ShellEngine,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            readliner: create_readliner(),
            lisp_engine: LispEngine::new(),
            shell_engine: ShellEngine::new(),
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
        let result = self.lisp_engine.run(&input)?;

        println!("{}", result);

        Ok(())
    }

    fn process_shell(&mut self, input: String) -> Result<()> {
        let result = self.shell_engine.run(&input)?;

        println!("{}", result);

        Ok(())
    }
}
