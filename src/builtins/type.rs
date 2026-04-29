use anyhow::bail;

use super::Builtin;

use super::Command;

pub struct Type {
    names: Vec<String>,
}

impl Command for Type {
    const NAME: &'static str = "type";
    fn parse(args: &[&str]) -> anyhow::Result<Self> {
        let names: Vec<_> = args.iter().map(|s| s.to_string()).collect();
        Ok(Self { names })
    }

    fn run(&self) -> anyhow::Result<()> {
        for name in &self.names {
            if Builtin::NAMES.contains(&name.as_str()) {
                println!("{name} is a shell builtin");
            } else {
                bail!("{name}: not found");
            }
        }
        Ok(())
    }
}
