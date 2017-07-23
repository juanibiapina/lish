extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(input: &str) -> String {
    let mut engine = Engine::new();
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap(), true)
}

#[test]
fn write_string() {
    assert_eq!(run("(write \"a\")"), "\"\"a\"\"");
}

#[test]
fn display_string() {
    assert_eq!(run("(display \"a\")"), "\"a\"");
}
