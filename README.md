# Find
A command line tool written in Rust that searches for a file in a given directory.

## Usage
There are two to run the program:
- Cargo:
```bash
cargo run -- --help # For further information of usage
```
- Binary:
```bash
# First build the binary
cargo build --release
# Then run the binary
./target/release/find --help # For further information of usage
```

> [!NOTE]
> - Even though it is supposed to work in both Unix like systems and Windows it hasn't been tested thoroughly.
> - New features will continue being addded and this is an early version of the program.

## Features
- Searches for a file in a given directory.
