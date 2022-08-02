use std::io::Read;
use std::io::Write;

use bft_types::instruction_description;
use bft_types::BfProgram;
use vm_error::VirtualMachineError;
use cellkind::CellKind;

mod cellkind;
mod vm_error;

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
    program_position: usize,
    /// Bool to indicate whether the tape can grow
    growable: bool,
}

impl<'a, T: cellkind::CellKind> VirtualMachine<'a, T>
where
    T: cellkind::CellKind,
{
    /// New implementation for the VirtualMachine struct.
    pub fn new(program: &'a BfProgram, mut tape_length: usize, growable: bool) -> Self {
        if tape_length == 0 {
            tape_length = 30000
        }
        Self {
            program,
            tape: Vec::with_capacity(tape_length),
            tape_head: 0,
            program_position: 0,
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
                instruction_info: self.program.instructions()[self.program_position],
                filename: self.program.filename().display().to_string(),
                position: self.tape_head,
                end_position: self.tape.len(),
            });
        }
        Ok(())
    }

    /// Increments the value in the cell at the head of the tape
    pub fn increment_cell_at_head(&mut self) {
        self.tape[self.tape_head].increment();
    }

    /// Decrements the value in the cell at the head of the tape
    pub fn decrement_cell_at_head(&mut self) {
        self.tape[self.tape_head].decrement();
    }

    /// Reads into the call at the head of the tape, will return a
    /// VirtualMachineError if there is a failure to read
    pub fn read_into_cell(
        &mut self,
        mut reader: impl Read,
    ) -> Result<(), VirtualMachineError> {
        let mut buffer: [u8; 1] = [0; 1];
        match reader.read_exact(&mut buffer) {
            Ok(()) => {
                self.tape[self.tape_head] = CellKind::from_u8(buffer[0]);
                Ok(())
            }
            Err(e) => Err(VirtualMachineError::IOError(e)),
        }
    }

    /// Writes out of the cell at the head of the tape, will return a
    /// VirtualMachineError if there is a failure to write
    pub fn write_out_of_cell(
        &mut self,
        writer: &mut impl Write,
    ) -> Result<(), VirtualMachineError> {
        let mut buffer: [u8; 1] = [0; 1];
        buffer[0] = self.tape[self.tape_head].into_u8();

        writer.write_all(&buffer)?;

        Ok(())
    }

    /// Moves the head of the tape to the right
    pub fn move_right(&mut self) -> Result<(), VirtualMachineError> {
        // Check in case it has already moved into an invalid location.
        self.check_head_location()?;
        // Increment the head position.
        self.tape_head += 1;
        // Check to see if it has moved into an invalid location now.
        self.check_head_location()?;
        Ok(())
    }

    /// Moves the head of the tape to the left
    pub fn move_left(&mut self) -> Result<(), VirtualMachineError> {
        self.check_head_location()?;
        self.tape_head -= 1;
        self.check_head_location()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
