mod builtins;
mod config;
mod dispatch;
mod external;

use anyhow::Context;
use std::{
    env,
    io::{self, Write},
};

use crate::config::Config;

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
            .context("reading from stdin")?;
        let argv: Vec<_> = buf.trim_ascii().split_whitespace().collect();
        if argv.is_empty() {
            continue;
        }

        if let Err(e) = dispatch::dispatch(&argv) {
            eprintln!("{e}");
        }
    }
}
