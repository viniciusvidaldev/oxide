use anyhow::{Result, anyhow};

use crate::builtins::Command;

pub struct Exit {
    code: u8,
}

impl Command for Exit {
    fn parse(args: &[&str]) -> Result<Self> {
        let code = match args.first() {
            Some(arg) => arg
                .parse::<u8>()
                .map_err(|_| anyhow!("invalid exit code: {arg}"))?,
            None => 0,
        };
        Ok(Self { code })
    }

    fn run(&self) -> Result<()> {
        std::process::exit(self.code as i32);
    }
}
