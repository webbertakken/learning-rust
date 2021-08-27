# Learning Rust

Nothing much to see here, just me learning Rust :)

Each folder contains a Rust project.
The basics folder contains examples of most of the language's basics.

![Build & Test](https://github.com/webbertakken/learning-rust/actions/workflows/main.yml/badge.svg)

## Usage

### Prerequisites

- [cargo-watch](https://crates.io/crates/cargo-watch)

### Build & Run

Usage example

```ps
cd basics
cargo run
```

Alternatively, to run from the root directory

```ps
cargo run --manifest-path=basics/Cargo.toml
``` 

### Watch

Usage example

```ps
cd basics
cargo watch -x run
```

Alternatively, to run from the root directory

```
cargo watch -x 'run -- --manifest-path=basics/Cargo.toml'
```

## License

[MIT](./LICENSE) license
