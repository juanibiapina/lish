#[derive(Debug)]
#[derive(PartialEq)]
pub enum Ast {
    ShellAst(ShellExpr),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ShellExpr {
    pub command: String,
    pub args: Vec<String>,
}

impl ShellExpr {
    pub fn new(words: Vec<&str>) -> ShellExpr {
        let mut iter = words.into_iter();
        let command = iter.next().unwrap().to_string();

        let mut args = vec!();
        for part in iter {
            args.push(part.to_string());
        }

        ShellExpr {
            command: command,
            args: args,
        }
    }
}
