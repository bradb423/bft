# Rust Brainfuck Interpreter (bft)

This is a Brainfuck interpreter, built during a series of internal Codethink
Rust sessions. This has been incredibly fun, and has taught me a lot about Rust.

## How to run

Using cargo:

```console
cargo run -- hello-world.bf
```

And here is the help menu for the program.

```console
$ cargo run -- --help

bft 0.1.0
Bradley Burns <bradleyburns00@gmail.com>
A Brainfuck Interpreter, written in Rust.

USAGE:
    bft [OPTIONS] <FILENAME>

ARGS:
    <FILENAME>    The filename of the program to interpret

OPTIONS:
    -c, --cell <cell>    The number of cells in the tape of the Virtual Machine [default: 30000]
    -e, --extensible     Whether or not the tape of the Virtual Machine can be extensible
    -h, --help           Print help information
    -V, --version        Print version information
```
