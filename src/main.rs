use bft_interp::VirtualMachine;
use bft_types::BfProgram;
use clap::{crate_name, Parser};
use std::error::Error;
use std::process::exit;

mod cli;

/// Main entry point of the program
fn run_bft(arguments: &cli::Args) -> Result<(), Box<dyn Error>> {
    let bf_program = BfProgram::from_file(&arguments.filename)?;
    bf_program.bracket_check()?;
    let interpreter = VirtualMachine::<u8>::new(&bf_program, arguments.cells, arguments.extensible);
    interpreter.interpret(&bf_program);
    Ok(())
}

fn main() {
    let arguments = cli::Args::parse();

    // Deal with the error that could arise from executing the program
    exit(match run_bft(&arguments) {
        Ok(_) => 0,
        Err(err) => {
            println!("{}: {}", crate_name!(), err);
            1
        }
    })
}
