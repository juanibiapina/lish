use std::io;
use std::io::Write;

fn main() {
    print!(":) ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Could not read standard input");
}
