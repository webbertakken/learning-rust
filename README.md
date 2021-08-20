# Learning Rust

Nothing much to see here, just me learning Rust :)

Each folder contains a Rust project.

## Usage

### Prerequisites

- [cargo-watch](https://crates.io/crates/cargo-watch)

### Build & Run

Usage example

```ps
cd hello
cargo run
```

Alternatively, to run from the root directory

```ps
cargo run --manifest-path=hello/Cargo.toml
``` 

### Watch

Usage example

```ps
cd hello
cargo watch -x run
```

Alternatively, to run from the root directory

```
cargo watch -x 'run -- --manifest-path=hello/Cargo.toml'
```

## License

[MIT](./LICENSE) license
