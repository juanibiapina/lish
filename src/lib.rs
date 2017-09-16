#![feature(slice_patterns)]

#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate maplit;

mod error;
mod readliner;
mod token;
mod types;
mod lexer;
mod parser;
mod env;
mod shell_evaluator;
mod lisp_evaluator;
mod core;

pub mod printer;
pub mod engine;
pub mod repl;
