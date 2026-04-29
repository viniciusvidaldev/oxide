# oxide

A tiny POSIX-ish shell written in Rust, built as a learning project.

## Builtins

- `echo [args...]` — print arguments separated by spaces
- `exit [code]` — exit with the given status (default `0`)
- `type <name>...` — report whether a name is a shell builtin

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
