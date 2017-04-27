pub struct Ast {
    pub command: String,
    pub args: Vec<String>,
}

impl Ast {
    pub fn new(words: Vec<&str>) -> Ast {
        let mut iter = words.into_iter();
        let command = iter.next().unwrap().to_string();

        let mut args = vec!();
        for part in iter {
            args.push(part.to_string());
        }

        Ast {
            command: command,
            args: args,
        }
    }
}
