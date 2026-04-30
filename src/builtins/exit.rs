use anyhow::{Result, anyhow};

pub fn run(args: &[&str]) -> Result<()> {
    let code = match args.first() {
        Some(arg) => arg
            .parse::<u8>()
            .map_err(|_| anyhow!("invalid exit code: {arg}"))?,
        None => 0,
    };
    std::process::exit(code as i32);
}
