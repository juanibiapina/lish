extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(input: &str) -> String {
    let mut engine = Engine::new();
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap(), true)
}

#[test]
fn no_args() {
    assert_eq!(run("(hash)"), "{}");
}

#[test]
fn one_pair_of_args() {
    assert_eq!(run("(hash \"a\" 1)"), "{ \"a\" 1 }");
}
