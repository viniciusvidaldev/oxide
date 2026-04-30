use anyhow::{Result, bail};

use crate::builtins::{echo, exit, r#type};
use crate::external::{path_lookup, run_external};

type BuiltinFn = fn(&[&str]) -> Result<()>;

pub struct Builtin {
    pub name: &'static str,
    pub func: BuiltinFn,
}

const BUILTINS: &[Builtin] = &[
    Builtin { name: "echo", func: echo::run },
    Builtin { name: "exit", func: exit::run },
    Builtin { name: "type", func: r#type::run },
];

pub fn lookup(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|b| b.name == name)
}

pub fn dispatch(argv: &[&str]) -> Result<()> {
    let (name, args) = argv.split_first().expect("argv is never empty");
    if let Some(b) = lookup(name) {
        return (b.func)(args);
    }
    match path_lookup(name) {
        Some(p) => run_external(&p, args),
        None => bail!("oxide: Command '{name}' not found"),
    }
}
