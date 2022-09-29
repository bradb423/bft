//! Main crate for bft, this is where the magic happens!

#![deny(missing_docs)]
#![cfg(not(tarpaulin_include))]

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::{crate_name, Parser};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::process::ExitCode;

mod cli;

/// A wrapper around Write to ensure that a new line is written.
struct WriterWrapper<T> {
    writer: T,
    last_byte: u8,
}

impl<T> Write for WriterWrapper<T>
where
    T: Write,
{
    /// Wrapped write command which keeps aa eye on the last byte.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(b) = buf.last() {
            self.last_byte = *b;
        }
        self.writer.write(buf)
    }

    /// Wrapped flush method, no real difference from the original flush method.
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<T> Drop for WriterWrapper<T> {
    /// When the wrapper ends, a new line is added if there is not one already.
    fn drop(&mut self) {
        if self.last_byte != b'\n' {
            println!()
        }
    }
}

/// Main entry point of the program. This takes the arguments passed in via the
/// CLI and interprets the program.
fn run_bft(arguments: &cli::Args) -> Result<(), Box<dyn Error>> {
    let bf_program = BfProgram::from_file(&arguments.filename)?;
    let mut interpreter = VirtualMachine::<u8>::new(
        &bf_program,
        arguments.cells,
        arguments.extensible,
    );
    let mut writer_wrapper = WriterWrapper {
        writer: stdout(),
        last_byte: 0u8,
    };
    interpreter.interpret(&mut stdin(), &mut writer_wrapper)?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
/// The main program for the interpreter
fn main() -> ExitCode {
    let arguments = cli::Args::parse();

    // Deal with the error that could arise from executing the program
    match run_bft(&arguments) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("{}: {}", crate_name!(), err);
            ExitCode::FAILURE
        }
    }
}
