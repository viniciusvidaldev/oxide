# oxide

A tiny POSIX-ish shell written in Rust, built as a learning project.

## Builtins

- `cd [dir]` — change directory; defaults to `~`, supports `~/...` expansion
- `echo [args...]` — print arguments separated by spaces
- `exit [code]` — exit with the given status (default `0`)
- `pwd` — print the current working directory
- `type <name>...` — report whether a name is a shell builtin or an external on `PATH`

## External commands

Anything not a builtin is looked up on `$PATH` and executed. The child's stdout, stderr, and exit status pass through unchanged.

## Config

Optional TOML config at `$XDG_CONFIG_HOME/oxide/config.toml` (falls back to `~/.config/oxide/config.toml`). Missing file → defaults; malformed file → defaults plus a warning on stderr.

```toml
prompt = "λ "
```

## Run

```sh
cargo run
```

```
/home/me λ echo hello world
hello world
/home/me λ type echo
echo is a shell builtin
/home/me λ type ls
ls is /bin/ls
/home/me λ cd ~/dev
/home/me/dev λ pwd
/home/me/dev
/home/me/dev λ exit
```
