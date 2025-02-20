# Smart Trade

Smart Trade is built on the [Ref Finance API](https://www.ref.finance/) using near-api-rs, Tauri, Leptos, and Ollama.

It uses ref-finance.testnet. 

## Using the Smart Trade App

Install ollama.

<https://ollama.com/>

Install deepseek model

```bash
ollama run deepseek-r1:1.5b
```

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

"hello.near"


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
