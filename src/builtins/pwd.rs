use std::env;

use anyhow::Result;

pub fn run(args: &[&str]) -> Result<()> {
    anyhow::ensure!(
        args.is_empty(),
        "pwd: Expected 0 arguments, got {}",
        args.len()
    );

    let cwd = env::current_dir()?;
    println!("{}", cwd.display());
    Ok(())
}
