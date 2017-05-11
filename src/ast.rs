#[derive(Debug)]
#[derive(PartialEq)]
pub enum Program {
    ShellProgram(ShellExpr),
    LispProgram(LispExpr),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ShellExpr {
    pub command: Word,
    pub args: Vec<Word>,
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Word(pub String);

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LispExpr {
    Symbol(String),
    List(Vec<LispExpr>),
}

impl ShellExpr {
    pub fn from_words(words: Vec<Word>) -> ShellExpr {
        let mut iter = words.into_iter();
        let command = iter.next().unwrap();

        let mut args = vec![];
        for part in iter {
            args.push(part);
        }

        ShellExpr {
            command: command,
            args: args,
        }
    }
}
