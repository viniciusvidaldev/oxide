use std::io::Write;

use anyhow::{Result, anyhow};

pub fn run(args: &[String], _out: &mut dyn Write) -> Result<()> {
    anyhow::ensure!(args.len() <= 1, "exit: Too many arguments");

    let code = match args.first() {
        Some(arg) => arg
            .parse::<u8>()
            .map_err(|_| anyhow!("exit: Invalid exit code '{arg}'"))?,
        None => 0,
    };
    std::process::exit(code as i32);
}
