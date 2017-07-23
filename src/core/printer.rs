use printer::Printer;
use error::{Result};
use types::{LispValue, string};

pub fn display(args: &[LispValue]) -> Result<LispValue> {
    let printer = Printer::new();

    let arg = args[0].clone();

    let result = printer.print(&arg, false);

    Ok(string(result))
}

pub fn write(args: &[LispValue]) -> Result<LispValue> {
    let printer = Printer::new();

    let arg = args[0].clone();

    let result = printer.print(&arg, true);

    Ok(string(result))
}
