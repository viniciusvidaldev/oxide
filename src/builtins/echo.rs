use anyhow::Result;

use crate::builtins::Builtin;

pub struct Echo {
    args: Vec<String>,
}

impl Builtin for Echo {
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
