use std::{
    io::{self, Write},
    process,
};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    // TODO: Uncomment the code below to pass the first stage

    let mut buf = String::new();
    loop {
        buf.clear();
        print!("λ ");
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut buf)
            .context("reading from stdin")?;

        let input = buf.trim_ascii();
        let parts: Vec<_> = input.split_whitespace().collect();
        let Some((cmd, args)) = parts.split_first() else {
            continue;
        };

        match *cmd {
            "exit" => process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            _ => println!("{}: command not found", input),
        }
    }
}
