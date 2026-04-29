mod builtins;
mod config;

use anyhow::Context;
use std::io::{self, Write};
use {builtins::Builtin, config::Config};

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();

    let user_config = Config::load();
    loop {
        buf.clear();
        print!("{}", user_config.prompt);
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut buf)
            .context("reading from stdin")?;
        let argv: Vec<_> = buf.trim_ascii().split_whitespace().collect();
        if argv.is_empty() {
            continue;
        }

        let result = Builtin::try_from(argv.as_slice()).and_then(|b| b.run());
        if let Err(e) = result {
            eprintln!("{e}");
        }
    }
}
