extern crate lish;

use lish::Repl;

fn main() {
    let repl = Repl::new();

    repl.run();
}
