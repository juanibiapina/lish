#[derive(Debug)]
#[derive(PartialEq)]
pub enum Program {
    ShellProgram(ShellExpr),
    LispProgram(LispExpr),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ShellExpr {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LispExpr {
    Integer(i64),
    Symbol(String),
    List(Vec<LispExpr>),
}
