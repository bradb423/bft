#![deny(missing_docs)]

use clap::Parser;
use std::path::PathBuf;

/// A Brainfuck Interpreter, written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// The filename of the program to interpret.
    pub(crate) filename: PathBuf,

    /// The number of cells in the tape of the Virtual Machine.
    // #[clap(name = "cell", short, long, value_parser, default_value_t = 30000)]
    #[arg(short, long, default_value_t = 30_000)]
    pub(crate) cells: usize,

    /// Whether or not the tape of the Virtual Machine can be extensible.
    #[arg(short, long, default_value_t = false)]
    pub(crate) extensible: bool,
}
