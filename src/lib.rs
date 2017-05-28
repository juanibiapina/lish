#![feature(slice_patterns)]

#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod error;
mod readliner;
mod token;
mod types;
mod lexer;
mod parser;
mod env;
mod evaluator;
mod printer;
mod core;
mod engine;

pub mod repl;
