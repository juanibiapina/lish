use std;
use std::rc::Rc;
use std::fmt::Debug;

use error::Result;

#[derive(PartialEq, Debug)]
pub enum Program {
    ShellProgram(ShellExpr),
    LispProgram(LispValue),
}

#[derive(PartialEq, Debug)]
pub struct ShellExpr {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum LispType {
    Integer(i64),
    Symbol(String),
    NativeFunction(NativeFunctionType),
    List(Vec<LispValue>),
}

pub type LispValue = Rc<LispType>;

pub struct NativeFunctionType {
    pub body: (fn(&[LispValue]) -> Result<LispValue>),
}

impl PartialEq for NativeFunctionType {
    fn eq(&self, other: &NativeFunctionType) -> bool {
        (self.body as *const fn(&[LispValue]) -> Result<LispValue>) == (other.body as *const  fn(&[LispValue]) -> Result<LispValue>)
    }
}

impl Debug for NativeFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "#<native-function ...>")
    }
}

// Constructors

pub fn list(seq: Vec<LispValue>) -> LispValue {
    Rc::new(LispType::List(seq))
}

pub fn integer(i: i64) -> LispValue {
    Rc::new(LispType::Integer(i))
}

pub fn symbol(value: String) -> LispValue {
    Rc::new(LispType::Symbol(value))
}

pub fn native_function(f: (fn(&[LispValue]) -> Result<LispValue>)) -> LispValue {
    Rc::new(LispType::NativeFunction(NativeFunctionType{ body: f }))
}
