mod echo;
mod exit;
mod r#type;

use anyhow::bail;

use echo::Echo;
use exit::Exit;
use r#type::Type;

pub trait Command: Sized {
    const NAME: &'static str;
    fn parse(args: &[&str]) -> anyhow::Result<Self>;
    fn run(&self) -> anyhow::Result<()>;
}

pub enum Builtin {
    Echo(Echo),
    Exit(Exit),
    Type(Type),
}

impl TryFrom<&[&str]> for Builtin {
    type Error = anyhow::Error;

    fn try_from(argv: &[&str]) -> anyhow::Result<Self> {
        let (cmd, args) = argv
            .split_first()
            .expect("argv is never empty, checked in main");

        match *cmd {
            Exit::NAME => Exit::parse(args).map(Self::Exit),
            Echo::NAME => Echo::parse(args).map(Self::Echo),
            Type::NAME => Type::parse(args).map(Self::Type),
            _ => bail!("unknown command: {cmd}"),
        }
    }
}

impl Builtin {
    pub const NAMES: &[&str] = &[Exit::NAME, Echo::NAME, Type::NAME];

    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Exit(cmd) => cmd.run(),
            Self::Echo(cmd) => cmd.run(),
            Self::Type(cmd) => cmd.run(),
        }
    }
}
