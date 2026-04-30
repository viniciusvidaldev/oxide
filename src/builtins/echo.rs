use anyhow::Result;

pub fn run(args: &[&str]) -> Result<()> {
    println!("{}", args.join(" "));
    Ok(())
}
