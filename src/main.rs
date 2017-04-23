extern crate lish;

use lish::repl::Repl;

fn main() {
    let mut repl = Repl::new();

    repl.run();
}
