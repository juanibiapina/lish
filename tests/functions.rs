extern crate lish;

use lish::engine::Engine;
use lish::printer::Printer;

fn run(engine: &mut Engine, input: &str) -> String {
    let printer = Printer::new();

    printer.print(&engine.run(input).unwrap().unwrap())
}

#[test]
fn no_args_and_simple_body() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "((fn () 1))"), "1");
}

#[test]
fn no_args_and_function_body() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "((fn () (+ 2 3)))"), "5");
}

#[test]
fn combining_with_do() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "((fn () (do (+ 2 3) (+ 4 5))))"), "9");
}

#[test]
fn capture_outer_variable() {
    let mut engine = Engine::new();

    run(&mut engine, "(def x 3)");

    assert_eq!(run(&mut engine, "((fn () x))"), "3");
}

#[test]
fn with_one_argument() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "((fn (a) a) 3)"), "3");
}

#[test]
fn argument_and_do() {
    let mut engine = Engine::new();

    assert_eq!(run(&mut engine, "((fn (a) (do (+ a 1))) 3)"), "4");
}
