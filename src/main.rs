use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use std::env::args;
use std::error::Error;

/// Main entry point of the program
fn main() -> Result<(), Box<dyn Error>> {
    let filename = args().nth(1).ok_or("Please give a valid filename")?;
    let bf_program = BfProgram::from_file(filename)?;
    let interpreter = VirtualMachine::<u8>::new(0, false);
    interpreter.interpret(&bf_program);
    Ok(())
}
