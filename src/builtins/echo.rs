use anyhow::Result;

use super::Command;

pub struct Echo {
    args: Vec<String>,
}

impl Command for Echo {
    const NAME: &'static str = "echo";
    fn parse(args: &[&str]) -> Result<Self> {
        Ok(Self {
            args: args.iter().map(|s| s.to_string()).collect(),
        })
    }

    fn run(&self) -> Result<()> {
        println!("{}", self.args.join(" "));
        Ok(())
    }
}
