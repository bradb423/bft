//! Main crate for bft, this is where the magic happens!

#![deny(missing_docs)]

use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::{crate_name, Parser};
use std::error::Error;
use std::io::stdin;
use std::io::stdout;
use std::process::ExitCode;

mod cli;

/// Main entry point of the program
fn run_bft(arguments: &cli::Args) -> Result<(), Box<dyn Error>> {
    let bf_program = BfProgram::from_file(&arguments.filename)?;
    let mut interpreter = VirtualMachine::<u8>::new(
        &bf_program,
        arguments.cells,
        arguments.extensible,
    );
    interpreter.interpret(&mut stdin(), &mut stdout())?;
    Ok(())
}

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
