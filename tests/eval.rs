extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(input: &str) -> String {
    let mut engine = Engine::new();
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap())
}

#[test]
fn special_form_eval() {
    assert_eq!(run("(eval (list + 2 3))"), "5");
}
