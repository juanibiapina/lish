extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(input: &str) -> String {
    let mut engine = Engine::new();
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap(), true)
}

#[test]
fn str() {
    assert_eq!(run("(str 1)"), "\"1\"");
    assert_eq!(run("(str -1)"), "\"-1\"");
    assert_eq!(run("(str 1 2 3)"), "\"123\"");
    assert_eq!(run("(str \"value\")"), "\"value\"");
    assert_eq!(run("(str \"value 1\" \" \" \"value 2\")"), "\"value 1 value 2\"");
}
