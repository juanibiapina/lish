extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(engine: &mut Engine, input: &str) -> String {
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap(), true)
}

#[test]
fn hash_no_args() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "(hash)"), "{}");
}

#[test]
fn hash_one_pair_of_args() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "(hash \"a\" 1)"), "{ \"a\" 1 }");
}

#[test]
fn assoc() {
    let mut engine = Engine::new();

    run(&mut engine, "(def h1 (hash))");
    run(&mut engine, "(def h2 (hash-set h1 \"a\" 1))");

    assert_eq!(run(&mut engine, "(display h1)"), "\"{}\"");
    assert_eq!(run(&mut engine, "(display h2)"), "\"{ \"a\" 1 }\"");
}
