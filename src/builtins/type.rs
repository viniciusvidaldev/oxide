use anyhow::bail;

use crate::{builtins::Builtin, dispatch::Resolved};

pub struct Type {
    names: Vec<String>,
}

impl Builtin for Type {
    const NAME: &'static str = "type";
    fn parse(args: &[&str]) -> anyhow::Result<Self> {
        let names: Vec<_> = args.iter().map(|s| s.to_string()).collect();
        Ok(Self { names })
    }

    fn run(&self) -> anyhow::Result<()> {
        for name in &self.names {
            match Resolved::from_name(name) {
                Some(Resolved::Builtin(n)) => println!("{n} is a shell builtin"),
                None => bail!("{name}: not found"),
            }
        }
        Ok(())
    }
}
