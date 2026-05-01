use anyhow::{Result, anyhow};

pub fn run(args: &[&str]) -> Result<()> {
    anyhow::ensure!(args.len() <= 1, "pwd: too many arguments");

    let code = match args.first() {
        Some(arg) => arg
            .parse::<u8>()
            .map_err(|_| anyhow!("exit: Invalid exit code '{arg}'"))?,
        None => 0,
    };
    std::process::exit(code as i32);
}
