extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(input: &str) -> String {
    let mut engine = Engine::new();
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap(), true)
}

#[test]
fn string_append() {
    assert_eq!(run("(string-append 1)"), "\"1\"");
    assert_eq!(run("(string-append -1)"), "\"-1\"");
    assert_eq!(run("(string-append 1 2 3)"), "\"123\"");
    assert_eq!(run("(string-append \"value\")"), "\"value\"");
    assert_eq!(run("(string-append \"value 1\" \" \" \"value 2\")"), "\"value 1 value 2\"");
}
