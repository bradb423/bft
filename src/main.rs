use bft_types::BfProgram;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = args()
    .nth(1)
    .ok_or("Please give a valid filename")?;
    let bf_program = BfProgram::from_file(filename)?;
    Ok(())
}
