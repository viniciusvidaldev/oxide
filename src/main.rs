mod builtins;
mod config;
mod dispatch;
mod external;
mod parser;

use anyhow::Context;
use std::{
    env,
    io::{self, Write},
};

use crate::{config::Config, parser::Statement};

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();

    let user_config = Config::load();

    loop {
        buf.clear();

        let cwd = env::current_dir()
            .map(|c| c.display().to_string())
            .unwrap_or_else(|_| "?".into());

        print!("{cwd} {}", user_config.prompt);
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut buf)
            .context("oxide: Could not read from stdin")?;

        let trimmed = buf.trim();
        if trimmed.is_empty() {
            continue;
        }

        let statement = match Statement::from_buf(trimmed) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        if let Err(e) = dispatch::dispatch(&statement) {
            eprintln!("{e}");
        }
    }
}
