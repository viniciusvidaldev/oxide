use anyhow::Result;

use crate::dispatch::lookup;
use crate::external::path_lookup;

pub fn run(args: &[&str]) -> Result<()> {
    for name in args {
        if let Some(b) = lookup(name) {
            println!("{} is a shell builtin", b.name);
        } else if let Some(p) = path_lookup(name) {
            println!("{name} is {}", p.display());
        } else {
            eprintln!("type: Could not find '{name}'");
        }
    }
    Ok(())
}
