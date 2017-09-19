use error::Result;

pub fn run(args: Vec<String>) -> Result<()> {
    println!("{}", args.join(" "));

    Ok(())
}
