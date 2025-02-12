# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

```
cargo install create-tauri-app --locked
```

Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
```
  cd smart-trade
  cargo tauri android init
```

For Desktop development, run:
```
  cargo tauri dev
```

For Android development, run:
```
  cargo tauri android dev
```


## Tailwindcss

```bash

./tailwindcss -i ./src/input.css -o ./css/output.css --watch

```


## Leptos format

### Examples

**Single file**

Format a specific file by name

`leptosfmt ./examples/counter/src/lib.rs`

**Current directory**

Format all .rs files within the current directory

`leptosfmt .`

**Directory**

Format all .rs files within the examples directory

`leptosfmt ./examples`

**Glob**

Format all .rs files ending with `_test.rs` within the examples directory

`leptosfmt ./examples/**/*_test.rs`

## Rust format check

`cargo fmt --all -- --check`

## Rust format

`cargo fmt --all`

## Format a file

`rustfmt src/main.rs `
