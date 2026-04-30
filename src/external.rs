use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn run_external(program: &Path, args: &[&str]) -> Result<()> {
    std::process::Command::new(program).args(args).status()?;
    Ok(())
}

pub fn path_lookup(name: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    env::split_paths(&path)
        .map(|path| path.join(name))
        .find(|candidate| is_executable(candidate))
}

fn is_executable(path: &Path) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !metadata.is_file() {
        return false;
    }

    let mode = metadata.permissions().mode();
    mode & 0o111 != 0
}
