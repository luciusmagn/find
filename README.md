# finder_rs
A simple command line tool to find files in your system by name.

## Usage
Either run the binary directly or use cargo to run it directly:
```
cargo run -- [options] <filename>
```
```
./finder_rs [options] <filename>
```

If you want to use the binary directly, you must build the project first:
```
cargo build
```

### Options:
> [!NOTE]
> The options are case insensitive.
- -h, --help: Print help message
- -t, --time: Show time elapsed during search

## Examples:
```
./finder_rs -t main.rs
```
```
./finder_rs --help
```

> [!WARNING]
> This is a work in progress.
> This is a preliminary version.
> It is not yet feature complete, and it will be updated in the near future.
