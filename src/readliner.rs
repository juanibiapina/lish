extern crate libc;
extern crate rustyline;

use std::io;

use self::rustyline::Editor;

use error::Result;

pub trait Readliner {
    fn readline(&mut self) -> Result<String>;
    fn add_history_entry(&mut self, line: &str) -> bool;
}

pub struct RustylineReadliner {
    editor: Editor<()>,
}

impl RustylineReadliner {
    pub fn new() -> RustylineReadliner {
        RustylineReadliner { editor: Editor::<()>::new() }
    }
}

impl Readliner for RustylineReadliner {
    fn readline(&mut self) -> Result<String> {
        Ok(self.editor.readline(":) ")?)
    }

    fn add_history_entry(&mut self, line: &str) -> bool {
        self.editor.add_history_entry(line)
    }
}

pub struct StdinReadliner;

impl StdinReadliner {
    pub fn new() -> StdinReadliner {
        StdinReadliner
    }
}

impl Readliner for StdinReadliner {
    fn readline(&mut self) -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.pop();

        Ok(input)
    }

    fn add_history_entry(&mut self, _: &str) -> bool {
        false
    }
}

pub fn create_readliner() -> Box<Readliner> {
    let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;

    if istty {
        Box::new(RustylineReadliner::new())
    } else {
        Box::new(StdinReadliner::new())
    }
}
