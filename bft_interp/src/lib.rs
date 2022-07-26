use thiserror::Error;

use bft_types::instruction_description;
use bft_types::BfProgram;
use bft_types::InstructionInfo;

/// A "Virtual Machine" for the Brainfuck program to be interpreted in.
/// This struct consists of a Tape (an array of numbers) and a Head (a pointer
/// to the a position in the array).
///
/// Classical Brainfuck programs have byte size numbers (0 to 255) and the size
/// of the array is by default set at 30,000.
#[derive(Debug)]
pub struct VirtualMachine<'a, T = u8> {
    /// The Brainfuck program
    program: &'a BfProgram,
    /// The tape of the virtual machine interpreting the program
    tape: Vec<T>,
    /// The position of the head location of the tape
    tape_head: usize,
    /// The position of the interpreter in the program
    tape_position: usize,
    /// Bool to indicate whether the tape can grow
    growable: bool,
}

/// Trait to define extra methods for incrementing and decrementing the values
/// in the cells of the Brainfuck program.
pub trait CellKind {
    /// Wrapped incrementation of the value in a given cell
    fn increment(&self) -> Self;
    /// Wrapped decrementation of the value in a given cell
    fn decrement(&self) -> Self;
}

impl CellKind for u8 {
    fn increment(&self) -> Self {
        self.wrapping_add(1)
    }

    fn decrement(&self) -> Self {
        self.wrapping_sub(1)
    }
}

impl<'a, T: CellKind> VirtualMachine<'a, T> where T: CellKind {
    /// New implementation for the VirtualMachine struct.
    pub fn new(program: &'a BfProgram, mut tape_length: usize, growable: bool) -> Self {
        if tape_length == 0 {
            tape_length = 30000
        }
        Self {
            program,
            tape: Vec::with_capacity(tape_length),
            tape_head: 0,
            tape_position: 0,
            growable,
        }
    }
    /// Interpreter function for interpreting the program. Currently, this
    /// just prints out the commands of the program
    pub fn interpret(&self, program: &BfProgram) {
        let filename = program.filename();
        for instruction in program.instructions() {
            println!(
                "[{} : {} : {} ] {}",
                filename.display(),
                instruction.line(),
                instruction.column(),
                instruction_description(instruction.operation())
            );
        }
    }

    /// Checks that the head of the tape has not moved into an invalid location.
    /// If it has, then it will throw a `VirtualMachineError` back out.
    pub fn check_head_location(&self) -> Result<(), VirtualMachineError> {
        if self.tape_head > self.tape.len() {
            return Err(VirtualMachineError::InvalidHeadPosition {
                instruction_info: self.program.instructions()[self.tape_position],
                filename: self.program.filename().display().to_string(),
                position: self.tape_head,
                end_position: self.tape.len(),
            });
        }
        Ok(())
    }

    pub fn increment_cell_at_head(&mut self) {
        self.tape[self.tape_head].increment();
    }

    pub fn decrement_cell_at_head(&mut self) {
        self.tape[self.tape_head].decrement();
    }

    pub fn move_right(&mut self) -> Result<(), VirtualMachineError> {
        // Check in case it has already moved into an invalid location.
        self.check_head_location()?;
        // Increment the head position.
        self.tape_head += 1;
        // Check to see if it has moved into an invalid location now.
        self.check_head_location()?;
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), VirtualMachineError> {
        self.check_head_location()?;
        self.tape_head -=1;
        self.check_head_location()?;
        Ok(())
    }
}

/// An enum to represent the types of errors that the VirtualMachine may
/// encounter when interpreting the program.
#[derive(Debug, Error)]
pub enum VirtualMachineError {
    /// The head of the tape has been moved to an invalid position.
    #[error(
        "In {filename}: line {{instruction_info.line()}}, column \
        {{instruction_info.column}} the head is moved to an invalid position \
        by the command: {{instruction_info.operation}}. The current position \
        is {position}, while it should be within 0 and {end_position}."
    )]
    InvalidHeadPosition {
        instruction_info: InstructionInfo,
        filename: String,
        position: usize,
        end_position: usize,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
