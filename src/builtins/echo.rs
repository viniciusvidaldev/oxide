use std::io::Write;

use anyhow::Result;

pub fn run(args: &[String], out: &mut dyn Write) -> Result<()> {
    writeln!(out, "{}", args.join(" "))?;
    Ok(())
}
