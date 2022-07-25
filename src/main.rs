use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use std::error::Error;
use clap::Parser;

mod cli;

/// Main entry point of the program
fn main() -> Result<(), Box<dyn Error>> {
    let arguments = cli::Args::parse();
    let bf_program = BfProgram::from_file(arguments.filename)?;
    let interpreter = VirtualMachine::<u8>::new(arguments.cells, false);
    interpreter.interpret(&bf_program);
    Ok(())
}
