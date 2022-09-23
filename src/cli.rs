#![deny(missing_docs)]

use clap::Parser;
use std::path::PathBuf;

/// A Brainfuck Interpreter, written in Rust. This is the final homework of
/// Daniel Silverstone's rust workshop at Codethink.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// The filename of the program to interpret.
    #[clap(name = "filename", short, long, parse(from_os_str))]
    pub(crate) filename: PathBuf,

    /// The number of cells in the tape of the Virtual Machine.
    #[clap(name = "cell", short, long, value_parser, default_value_t = 30000)]
    pub(crate) cells: usize,

    /// Whether or not the tape of the Virtual Machine can be extensible.
    #[clap(
        name = "extensible",
        short,
        long,
        value_parser,
        default_value_t = false
    )]
    pub(crate) extensible: bool,
}
