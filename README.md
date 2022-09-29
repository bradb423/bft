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

A Brainfuck Interpreter, written in Rust

Usage: bft [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  The filename of the program to interpret

Options:
  -c, --cells <CELLS>  The number of cells in the tape of the Virtual Machine [default: 30000]
  -e, --extensible     Whether or not the tape of the Virtual Machine can be extensible
  -h, --help           Print help information
  -V, --version        Print version information
```
