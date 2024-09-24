# Rust

## Compilation avec rustc

```
rustc basics.rs
rustc --edition 2015 -o basics2015 basics.rs 
rustc --edition 2018 -o basics2018 basics.rs 
rustc --edition 2021 -o basics2021 basics.rs 
```

## Cargo
### Init Project
```
cargo new my_new_project
cargo new --name projectname directory_name
```
### Build and Run program
```
cargo run
cargo run -r
```

### Build only
```
cargo build
cargo build -r 
```

### Clean target directry
```
cargo clean
```

### Update dependency with lib in development
Refresh git reference to last commit
```
cargo update
```

### Cargo.lock
Discussion about putting or leaving Cargo.lock in your SCM (git or alt.)

https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html