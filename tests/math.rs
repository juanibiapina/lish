extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(input: &str) -> String {
    let mut engine = Engine::new();
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap())
}

#[test]
fn sums() {
    assert_eq!(run("(+ 1 2)"), "3");
    assert_eq!(run("(+ 2 3)"), "5");
    assert_eq!(run("(+ -2 3)"), "1");
}

#[test]
fn subtraction() {
    assert_eq!(run("(- 1 2)"), "-1");
    assert_eq!(run("(- 10 3)"), "7");
    assert_eq!(run("(- -2 -7)"), "5");
}
