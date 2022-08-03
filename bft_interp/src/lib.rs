use std::io::Read;
use std::io::Write;

use bft_types::instruction_description;
use bft_types::BfProgram;
use cellkind::CellKind;
use vm_error::VirtualMachineError;

mod cellkind;
mod vm_error;

/// A "Virtual Machine" for the Brainfuck program to be interpreted in.
/// This struct consists of a Tape (an array of numbers) and a Head (a pointer
/// to the a position in the array).
///
/// Classical Brainfuck programs have byte size numbers (0 to 255) and the size
/// of the array is by default set at 30,000.
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
    T: cellkind::CellKind + std::default::Default + std::clone::Clone,
{
    /// New implementation for the VirtualMachine struct.
    pub fn new(program: &'a BfProgram, mut tape_length: usize, growable: bool) -> Self {
        if tape_length == 0 {
            tape_length = 30000
        }
        Self {
            program,
            tape: vec![Default::default(); tape_length],
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
    pub fn check_head_location(&self) -> Result<usize, VirtualMachineError> {
        if self.tape_head > self.tape.len() {
            return Err(VirtualMachineError::InvalidHeadPosition {
                instruction_info: self.program.instructions()[self.program_position],
                filename: self.program.filename().display().to_string(),
                position: self.tape_head,
                end_position: self.tape.len(),
            });
        }
        Ok(self.program_position)
    }

    /// Increments the value in the cell at the head of the tape
    pub fn increment_cell_at_head(&mut self) {
        self.tape[self.tape_head].increment();
    }

    /// Decrements the value in the cell at the head of the tape
    pub fn decrement_cell_at_head(&mut self) {
        self.tape[self.tape_head].decrement();
    }

    /// Reads into the cell at the head of the tape, will return a
    /// VirtualMachineError if there is a failure to read
    pub fn read_into_cell(&mut self, mut reader: impl Read) -> Result<usize, VirtualMachineError> {
        let mut buffer: [u8; 1] = [0; 1];
        match reader.read_exact(&mut buffer) {
            Ok(()) => {
                println!(
                    "self.tape_head = {}, tape_length = {}",
                    self.tape_head,
                    self.tape.len()
                );
                self.tape[self.tape_head] = CellKind::from_u8(buffer[0]);
                Ok(self.program_position + 1)
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
    pub fn move_right(&mut self) -> Result<usize, VirtualMachineError> {
        // Check in case it has already moved into an invalid location.
        self.check_head_location()?;
        // Increment the head position.
        self.tape_head += 1;
        // Check to see if it has moved into an invalid location now.
        self.check_head_location()?;
        Ok(self.program_position + 1)
    }

    /// Moves the head of the tape to the left
    pub fn move_left(&mut self) -> Result<usize, VirtualMachineError> {
        self.check_head_location()?;
        self.tape_head -= 1;
        self.check_head_location()?;
        Ok(self.program_position + 1)
    }
}

#[cfg(test)]
mod tests {
    use bft_types::ops::Operation;
    use bft_types::BfProgram;

    use crate::VirtualMachine;

    use std::io::Cursor;

    /// A function to mock a program with instructions for associated tests.
    fn mock_working_program() -> BfProgram {
        let contents = String::from(
            "+-this
            is not a
            []>< brainfuck
                program! .,",
        );
        let filename = "test.bf";
        BfProgram::new(contents, filename)
    }

    /// A check that the program is properly filtered, with any invalid
    /// Brainfuck instructions ignored.
    #[test]
    fn correct_filtering() {
        let bf_program = mock_working_program();
        let instructions = bf_program.instructions();

        // Here there are 106 individual brainfuck commands
        assert_eq!(instructions.len(), 8);
    }

    /// A check that the program has stored the instruction in the right order.
    #[test]
    fn correct_instructions() {
        let bf_program = mock_working_program();
        let instructions = bf_program.instructions();

        assert_eq!(instructions[0].operation(), &Operation::IncrementByte);
        assert_eq!(instructions[1].operation(), &Operation::DecrementByte);
        assert_eq!(instructions[2].operation(), &Operation::StartLoop);
        assert_eq!(instructions[3].operation(), &Operation::EndLoop);
        assert_eq!(instructions[4].operation(), &Operation::IncrementPointer);
        assert_eq!(instructions[5].operation(), &Operation::DecrementPointer);
        assert_eq!(instructions[6].operation(), &Operation::OutputByte);
        assert_eq!(instructions[7].operation(), &Operation::InputByte);
    }

    /// Test that the program has the right line numbers for each command in the
    /// file.
    #[test]
    fn test_lines() {
        let bf_program = mock_working_program();
        let instructions = bf_program.instructions();

        assert_eq!(instructions[0].line(), 1);
        assert_eq!(instructions[1].line(), 1);
        assert_eq!(instructions[2].line(), 3);
        assert_eq!(instructions[3].line(), 3);
        assert_eq!(instructions[4].line(), 3);
        assert_eq!(instructions[5].line(), 3);
        assert_eq!(instructions[6].line(), 4);
        assert_eq!(instructions[7].line(), 4);
    }

    /// Test that the program has the right column numbers for each command in
    /// the file.
    #[test]
    fn test_columns() {
        let bf_program = mock_working_program();
        let instructions = bf_program.instructions();

        assert_eq!(instructions[0].column(), 1);
        assert_eq!(instructions[1].column(), 2);
        assert_eq!(instructions[2].column(), 13);
        assert_eq!(instructions[3].column(), 14);
        assert_eq!(instructions[4].column(), 15);
        assert_eq!(instructions[5].column(), 16);
        assert_eq!(instructions[6].column(), 26);
        assert_eq!(instructions[7].column(), 27);
    }

    /// A function to mock a program which has too few closing square brackets.
    fn too_few_closings() -> BfProgram {
        let contents = String::from("[[]");
        let filename = "test.bf";
        BfProgram::new(contents, filename)
    }

    /// A function to mock a program which has too many closing square brackets,
    /// leading to a loop being ended when there is no loop to be ended.
    fn unexpected_closing() -> BfProgram {
        let contents = String::from("[[][]]]");
        let filename = "test.bf";
        BfProgram::new(contents, filename)
    }

    /// A test to check that the bracket checker can correctly identify the
    /// problems in each of the bad programs, while not reporting errors in the
    /// case where the brackets are balanced.
    #[test]
    fn test_bracket_matcher() {
        let good_program = mock_working_program();
        let bad_program_1 = too_few_closings();
        let bad_program_2 = unexpected_closing();

        assert!(good_program.bracket_check().is_ok());
        assert!(bad_program_1.bracket_check().is_err());
        assert!(bad_program_2.bracket_check().is_err());
    }

    /// A test to check that the read method works properly
    #[test]
    fn test_read() {
        let good_program = mock_working_program();
        let mut vm = VirtualMachine::<u8>::new(&good_program, 0, false);

        let reader = Cursor::new(vec![1u8, 2u8]);

        assert!(vm.read_into_cell(reader).is_ok());
        assert_eq!(vm.tape[vm.tape_head], 1u8);
    }

    /// A test to check that the write method works properly
    #[test]
    fn test_write() {
        let good_program = mock_working_program();
        let mut vm = VirtualMachine::<u8>::new(&good_program, 0, false);

        let mut writer = Cursor::new(vec![1u8, 2u8]);

        assert!(vm.write_out_of_cell(&mut writer).is_ok());
        assert_eq!(vm.tape[vm.tape_head], 0u8);
    }
}
