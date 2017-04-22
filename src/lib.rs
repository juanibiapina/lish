use std::io;
use std::io::Write;

pub struct Repl;

impl Repl {
    pub fn new() -> Repl {
        Repl
    }

    pub fn run(&self) {
        print!(":) ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("Could not read standard input");
        println!("{}", line);
    }
}
