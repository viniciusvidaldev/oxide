use std::io::{self, Write};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    // TODO: Uncomment the code below to pass the first stage

    let mut buf = String::new();
    loop {
        buf.clear();
        print!("λ ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .context("reading from stdin")?;
        println!("{}: command not found", buf.trim());
    }
}
