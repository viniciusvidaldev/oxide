mod builtins;

use crate::builtins::Builtin;
use anyhow::Context;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    loop {
        buf.clear();
        print!("λ ");
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut buf)
            .context("reading from stdin")?;
        let argv: Vec<_> = buf.trim_ascii().split_whitespace().collect();
        if argv.is_empty() {
            continue;
        }

        match Builtin::try_from(argv.as_slice()) {
            Ok(builtin) => builtin.run()?,
            Err(e) => eprintln!("{e}"),
        }
    }
}
