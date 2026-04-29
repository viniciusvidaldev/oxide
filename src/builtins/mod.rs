use anyhow::bail;

use echo::Echo;
use exit::Exit;

mod echo;
mod exit;

pub trait Command: Sized {
    fn parse(args: &[&str]) -> anyhow::Result<Self>;
    fn run(&self) -> anyhow::Result<()>;
}

pub enum Builtin {
    Echo(Echo),
    Exit(Exit),
}

impl TryFrom<&[&str]> for Builtin {
    type Error = anyhow::Error;

    fn try_from(argv: &[&str]) -> anyhow::Result<Self> {
        let (cmd, args) = argv
            .split_first()
            .expect("argv is never empty, checked in main");

        match *cmd {
            "exit" => Exit::parse(args).map(Self::Exit),
            "echo" => Echo::parse(args).map(Self::Echo),
            _ => bail!("unknown command: {cmd}"),
        }
    }
}

impl Builtin {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Exit(cmd) => cmd.run(),
            Self::Echo(cmd) => cmd.run(),
        }
    }
}
