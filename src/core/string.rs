use printer::Printer;
use error::{Result};
use types::{LispValue, string};

pub fn string_append(args: &[LispValue]) -> Result<LispValue> {
    let printer = Printer::new();

    let strings: Vec<_> = args.iter().map(|arg| printer.print(arg, false)).collect();

    Ok(string(strings.join("")))
}
