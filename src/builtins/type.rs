use std::io::Write;

use anyhow::Result;

use crate::dispatch::lookup;
use crate::external::path_lookup;

pub fn run(args: &[String], out: &mut dyn Write) -> Result<()> {
    for name in args {
        if let Some(b) = lookup(name) {
            writeln!(out, "{} is a shell builtin", b.name)?;
        } else if let Some(p) = path_lookup(name) {
            writeln!(out, "{name} is {}", p.display())?;
        } else {
            writeln!(out, "type: Could not find '{name}'")?;
        }
    }
    Ok(())
}
