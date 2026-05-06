use std::{env, io::Write};

use anyhow::Result;

pub fn run(args: &[String], out: &mut dyn Write) -> Result<()> {
    anyhow::ensure!(
        args.is_empty(),
        "pwd: Expected 0 arguments, got {}",
        args.len()
    );

    let cwd = env::current_dir()?;
    writeln!(out, "{}", cwd.display())?;
    Ok(())
}
