use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::Parser;
use std::error::Error;

mod cli;

/// Main entry point of the program
fn main() -> Result<(), Box<dyn Error>> {
    let arguments = cli::Args::parse();
    let bf_program = BfProgram::from_file(arguments.filename)?;
    bf_program.bracket_check()?;
    let interpreter = VirtualMachine::<u8>::new(arguments.cells, false);
    interpreter.interpret(&bf_program);
    Ok(())
}
