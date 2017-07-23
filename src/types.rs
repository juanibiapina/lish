use std;
use std::rc::Rc;
use std::fmt::Debug;
use std::collections::HashMap;

use error::Result;
use env::Env;

#[derive(PartialEq, Debug)]
pub enum Program {
    ShellProgram(ShellExpr),
    LispProgram(LispValue),
    EmptyProgram,
}

#[derive(PartialEq, Debug)]
pub struct ShellExpr {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum LispType {
    Nil,
    Integer(i64),
    Symbol(String),
    Strn(String),
    Function(FunctionData),
    NativeFunction(NativeFunctionType),
    List(Vec<LispValue>),
    HashMap(HashMap<String, LispValue>),
}

pub type LispValue = Rc<LispType>;

#[derive(PartialEq, Debug)]
pub struct FunctionData {
    pub params: Vec<String>,
    pub body: LispValue,
    pub env: Env,
}

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

pub fn nil() -> LispValue {
    Rc::new(LispType::Nil)
}

pub fn list(seq: Vec<LispValue>) -> LispValue {
    Rc::new(LispType::List(seq))
}

pub fn hash_map(data: HashMap<String, LispValue>) -> LispValue {
    Rc::new(LispType::HashMap(data))
}

pub fn integer(i: i64) -> LispValue {
    Rc::new(LispType::Integer(i))
}

pub fn string(value: String) -> LispValue {
    Rc::new(LispType::Strn(value))
}

pub fn symbol(value: String) -> LispValue {
    Rc::new(LispType::Symbol(value))
}

pub fn native_function(f: (fn(&[LispValue]) -> Result<LispValue>)) -> LispValue {
    Rc::new(LispType::NativeFunction(NativeFunctionType{ body: f }))
}

pub fn function(params: Vec<String>, body: LispValue, env: Env) -> LispValue {
    Rc::new(LispType::Function(FunctionData {
        params: params,
        body: body,
        env: env,
    }))
}
