use std::{
    env::{self, set_current_dir},
    io::{self, Write},
    path::PathBuf,
};

use anyhow::Result;

fn expand_tilde(raw: &str) -> Result<PathBuf> {
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("$HOME is not set"))?;

    if raw == "~" {
        return Ok(home);
    }

    if let Some(rest) = raw.strip_prefix("~/") {
        return Ok(home.join(rest));
    }

    Ok(PathBuf::from(raw))
}

pub fn run(args: &[String], _out: &mut dyn Write) -> Result<()> {
    anyhow::ensure!(args.len() <= 1, "cd: Too many args for cd command");

    let raw = args.first().map(String::as_str).unwrap_or("~");
    let target = expand_tilde(raw)?;

    if let Err(e) = set_current_dir(&target) {
        let msg = match e.kind() {
            io::ErrorKind::NotFound => {
                format!("cd: The directory '{}' does not exist", target.display())
            }
            io::ErrorKind::NotADirectory => {
                format!("cd: '{}' is not a directory", target.display())
            }
            io::ErrorKind::PermissionDenied => {
                format!("cd: Permission denied for '{}'", target.display())
            }
            _ => format!("cd: Could not change to '{}': {e}", target.display()),
        };

        anyhow::bail!(msg);
    }

    Ok(())
}
