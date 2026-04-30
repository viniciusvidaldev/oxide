use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub struct External {
    bin_path: PathBuf,
    args: Vec<String>,
}

impl External {
    pub fn from_name(name: &str, args: &[&str]) -> anyhow::Result<Self> {
        let bin_path =
            path_lookup(name).ok_or_else(|| anyhow::anyhow!("command not found: {}", name))?;

        Ok(Self {
            bin_path,
            args: args.iter().map(|&s| s.to_string()).collect(),
        })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let status = std::process::Command::new(&self.bin_path)
            .args(&self.args)
            .status()?;

        if !status.success() {
            anyhow::bail!("command failed with status: {}", status);
        }

        Ok(())
    }
}

pub fn path_lookup(name: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    env::split_paths(&path)
        .map(|path| path.join(name))
        .find(|candidate| is_executable(candidate))
}

fn is_executable(path: &Path) -> bool {
    let metadata = match fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !metadata.is_file() {
        return false;
    }

    let mode = metadata.permissions().mode();
    mode & 0o111 != 0
}
