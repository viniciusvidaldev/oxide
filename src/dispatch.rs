use std::fs::File;
use std::io::Write;
use std::process::Stdio;

use anyhow::{Result, anyhow};

use crate::builtins::{cd, echo, exit, pwd, r#type};
use crate::external::{path_lookup, run_external};
use crate::parser::{Redirect, Statement, WriteKind};

type BuiltinFn = fn(&[String], &mut dyn Write) -> Result<()>;

pub struct Builtin {
    pub name: &'static str,
    pub func: BuiltinFn,
}

#[rustfmt::skip]
const BUILTINS: &[Builtin] = &[
    Builtin { name: "cd", func: cd::run },
    Builtin { name: "echo", func: echo::run },
    Builtin { name: "exit", func: exit::run },
    Builtin { name: "type", func: r#type::run },
    Builtin { name: "pwd", func: pwd::run },
];

pub fn lookup(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|b| b.name == name)
}

pub fn dispatch(stmt: &Statement) -> Result<()> {
    let (name, args) = stmt.argv.split_first().expect("argv is never empty");

    if let Some(b) = lookup(name) {
        let mut stdout = resolve_stdout(stmt)?;
        return (b.func)(args, &mut *stdout);
    }

    let path = path_lookup(name).ok_or_else(|| anyhow!("oxide: Command '{name}' not found"))?;
    let stdout = match &stmt.stdout {
        Some(r) => Stdio::from(open_redirect_file(r)?),
        None => Stdio::inherit(),
    };

    run_external(&path, args, stdout)?;
    Ok(())
}

fn resolve_stdout(stmt: &Statement) -> Result<Box<dyn Write>> {
    match &stmt.stdout {
        Some(e) => Ok(Box::new(open_redirect_file(e)?)),
        None => Ok(Box::new(std::io::stdout())),
    }
}

fn open_redirect_file(redirect: &Redirect) -> Result<File> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(matches!(redirect.kind, WriteKind::Truncate))
        .append(matches!(redirect.kind, WriteKind::Append))
        .open(&redirect.path)?;

    Ok(file)
}
