# oxide

A tiny POSIX-ish shell written in Rust, built as a learning project.

## Builtins

- `echo [args...]` — print arguments separated by spaces
- `exit [code]` — exit with the given status (default `0`)
- `type <name>...` — report whether a name is a shell builtin

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
λ echo hello world
hello world
λ type echo
echo is a shell builtin
λ exit
```
