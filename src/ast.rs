#[derive(Debug)]
#[derive(PartialEq)]
pub enum Program {
    ShellProgram(ShellExpr),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ShellExpr {
    pub command: Word,
    pub args: Vec<Word>,
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Word(pub String);

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
