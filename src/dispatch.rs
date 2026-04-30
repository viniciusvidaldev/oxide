use std::path::PathBuf;

use crate::{
    builtins::{Builtin, Echo, Exit, Type},
    external::{External, path_lookup},
};

pub enum Dispatch {
    Echo(Echo),
    Exit(Exit),
    Type(Type),
    External(External),
}

impl TryFrom<&[&str]> for Dispatch {
    type Error = anyhow::Error;

    fn try_from(argv: &[&str]) -> anyhow::Result<Self> {
        let (name, args) = argv.split_first().expect("argv is never empty");
        match *name {
            Echo::NAME => Echo::parse(args).map(Self::Echo),
            Exit::NAME => Exit::parse(args).map(Self::Exit),
            Type::NAME => Type::parse(args).map(Self::Type),
            _ => External::from_name(name, args).map(Self::External),
        }
    }
}

impl Dispatch {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Echo(c) => c.run(),
            Self::Exit(c) => c.run(),
            Self::Type(c) => c.run(),
            Self::External(c) => c.run(),
        }
    }
}

pub enum Resolved {
    Builtin(&'static str),
    External(PathBuf),
}

impl Resolved {
    pub fn from_name(name: &str) -> Option<Resolved> {
        match name {
            Echo::NAME => Some(Resolved::Builtin(Echo::NAME)),
            Exit::NAME => Some(Resolved::Builtin(Exit::NAME)),
            Type::NAME => Some(Resolved::Builtin(Type::NAME)),
            _ => path_lookup(name).map(Resolved::External),
        }
    }
}
