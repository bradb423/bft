//! `bft_interp`, containing the Virtual machine used for the interpretation of
//! Brainfuck Programs, along with its methods.

#![deny(missing_docs)]

use std::io::Read;
use std::io::Write;

use bft_types::BfProgram;
use bft_types::{ops::Operation, vm_error::VirtualMachineError};

mod cellkind;
use cellkind::CellKind;

const DEFAULT_TAPE_LENGTH: usize = 30_000;

/// A "Virtual Machine" for the Brainfuck program to be interpreted in.
/// This struct consists of a Tape (an array of numbers) and a Head (a pointer
/// to the a position in the array).
///
/// Classical Brainfuck programs have byte size numbers (0 to 255) and the size
/// of the array is by default set at 30,000.
pub struct VirtualMachine<'a, T> {
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

impl<'a, T> VirtualMachine<'a, T>
where
    T: CellKind
        + std::default::Default
        + std::clone::Clone
        + Copy
        + std::cmp::PartialEq,
{
    /// New implementation for the VirtualMachine struct.
    pub fn new(
        program: &'a BfProgram,
        mut tape_length: usize,
        growable: bool,
    ) -> Self {
        if tape_length == 0 {
            tape_length = DEFAULT_TAPE_LENGTH;
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
    pub fn interpret(
        &mut self,
        mut input: &mut impl Read,
        mut output: &mut impl Write,
    ) -> Result<(), VirtualMachineError> {
        let instructions = self.program.instructions();
        let last_position = instructions.len() - 1;
        while self.program_position <= last_position {
            let instruction = instructions[self.program_position];
            self.program_position = match instruction.operation() {
                Operation::IncrementByte => self.increment_cell_at_head(),
                Operation::DecrementByte => self.decrement_cell_at_head(),
                Operation::IncrementPointer => self.move_right(),
                Operation::DecrementPointer => self.move_left(),
                Operation::OutputByte => self.write_out_of_cell(&mut output),
                Operation::InputByte => self.read_into_cell(&mut input),
                Operation::StartLoop => self.start_loop(),
                Operation::EndLoop => self.end_loop(),
            }?;
        }
        Ok(())
    }

    /// Checks that the head of the tape has not moved into an invalid location.
    /// If it has, then it will throw a `VirtualMachineError` back out.
    pub fn check_head_location(
        &mut self,
    ) -> Result<usize, VirtualMachineError> {
        // This needs the `- 1` due to the fact that the tape_head is an integer
        // and the tape itself is being indexed from 0.
        // This should return an error if the head of the tape has moved to an
        // invalid location, and the tape is not allowed to grow.
        if self.tape_head > self.tape.len() - 1 {
            // If the tape is growable, increase the length of the tape
            if self.growable {
                self.tape.push(Default::default());
            } else {
                return Err(VirtualMachineError::InvalidHeadPosition {
                    line: self.program.instructions()[self.program_position]
                        .line(),
                    column: self.program.instructions()[self.program_position]
                        .column(),
                    operation: self.program.instructions()
                        [self.program_position]
                        .operation(),
                    filename: self.program.filename().display().to_string(),
                    position: self.tape_head,
                    tape_length: self.tape.len(),
                });
            }
        }
        Ok(self.program_position)
    }

    /// Increments the value in the cell at the head of the tape
    pub fn increment_cell_at_head(
        &mut self,
    ) -> Result<usize, VirtualMachineError> {
        self.tape[self.tape_head] = self.tape[self.tape_head].increment();
        Ok(self.program_position + 1)
    }

    /// Decrements the value in the cell at the head of the tape
    pub fn decrement_cell_at_head(
        &mut self,
    ) -> Result<usize, VirtualMachineError> {
        self.tape[self.tape_head] = self.tape[self.tape_head].decrement();
        Ok(self.program_position + 1)
    }

    /// Reads into the cell at the head of the tape, will return a
    /// VirtualMachineError if there is a failure to read
    pub fn read_into_cell(
        &mut self,
        mut reader: impl Read,
    ) -> Result<usize, VirtualMachineError> {
        let mut buffer: [u8; 1] = [0; 1];
        match reader.read_exact(&mut buffer) {
            Ok(()) => {
                println!(
                    "self.tape_head = {}, tape_length = {}",
                    self.tape_head,
                    self.tape.len()
                );
                self.tape[self.tape_head] = T::from_u8(buffer[0]);
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
    ) -> Result<usize, VirtualMachineError> {
        let mut buffer: [u8; 1] = [0; 1];
        buffer[0] = self.tape[self.tape_head].to_u8();

        writer.write_all(&buffer)?;

        Ok(self.program_position + 1)
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
        if self.tape_head == 0 {
            return Err(VirtualMachineError::InvalidHeadPosition {
                line: self.program.instructions()[self.program_position].line(),
                column: self.program.instructions()[self.program_position]
                    .column(),
                operation: self.program.instructions()[self.program_position]
                    .operation(),
                filename: self.program.filename().display().to_string(),
                position: self.tape_head,
                tape_length: self.tape.len(),
            });
        } else {
            self.tape_head -= 1;
            self.check_head_location()?;
            Ok(self.program_position + 1)
        }
    }

    /// Performs the unconditional jump forwards to the closing ']'.
    pub fn start_loop(&mut self) -> Result<usize, VirtualMachineError> {
        if self
            .program
            .bracket_matching_positions()
            .contains_key(&self.program_position)
        {
            Ok(self.program.bracket_matching_positions()
                [&self.program_position])
        } else {
            Err(VirtualMachineError::BracketFailure)
        }
    }

    /// If the value of the cell at the head of the tape is non-zero, then this
    /// function will find the instruction after the corresponding opening
    /// bracket.
    pub fn end_loop(&mut self) -> Result<usize, VirtualMachineError> {
        let zero_value = T::from_u8(0u8);
        if self.tape[self.tape_head] != zero_value {
            for (key, value) in self.program.bracket_matching_positions().iter()
            {
                if *value == self.program_position {
                    return Ok(*key + 1);
                }
            }
        }
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
        BfProgram::new(contents, filename).unwrap()
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

        assert_eq!(instructions[0].operation(), Operation::IncrementByte);
        assert_eq!(instructions[1].operation(), Operation::DecrementByte);
        assert_eq!(instructions[2].operation(), Operation::StartLoop);
        assert_eq!(instructions[3].operation(), Operation::EndLoop);
        assert_eq!(instructions[4].operation(), Operation::IncrementPointer);
        assert_eq!(instructions[5].operation(), Operation::DecrementPointer);
        assert_eq!(instructions[6].operation(), Operation::OutputByte);
        assert_eq!(instructions[7].operation(), Operation::InputByte);
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

    /// A test which mocks the failure of the program to be created if the
    /// brackets are not balanced. In this case, this is due to there being
    /// too few closing brackets.
    #[test]
    fn too_few_closings() {
        let contents = String::from("[[]");
        let filename = "test.bf";
        assert!(BfProgram::new(contents, filename).is_err());
    }

    /// A test which mocks the failure of the program to be created if the
    /// brackets are not balanced. In this case, this is due to there being
    /// too many closing brackets.
    #[test]
    fn unexpected_closing() {
        let contents = String::from("[[][]]]");
        let filename = "test.bf";
        assert!(BfProgram::new(contents, filename).is_err());
    }

    /// A test to check that the bracket checker can correctly identify the
    /// problems in each of the bad programs, while not reporting errors in the
    /// case where the brackets are balanced.
    #[test]
    fn test_bracket_matcher() {
        let good_program = mock_working_program();

        assert!(good_program.bracket_check().is_ok());
    }

    /// A test to check that the program head can move backwards, it should
    /// pass an error back if it moves backwards too many times.
    #[test]
    fn test_moving_backwards() {
        let program =
            BfProgram::new(String::from("dklsjf.,<>;ahg"), "filename.bf")
                .expect("Something went wrong with this test");
        let mut vm = VirtualMachine::<u8>::new(&program, 2, false);

        // If the tape head moves forwards once, then moves backwards twice,
        // and error should be generated.

        assert!(vm.move_right().is_ok());
        assert!(vm.move_left().is_ok());
        assert!(vm.move_left().is_err());
    }

    /// A test to check that with a tape of length 1, the program cannot move
    /// right
    #[test]
    fn test_failed_move_forwards() {
        let program =
            BfProgram::new(String::from("dklsj,.<>f;ahg"), "filename.bf")
                .expect("Something went wrong with this test");
        let mut vm = VirtualMachine::<u8>::new(&program, 1, false);

        // If the tape head moves forwards too much, it will fall off the tape
        // which is set to a length of 1.

        assert!(vm.check_head_location().is_ok());
        assert!(vm.move_right().is_err());
        assert!(vm.check_head_location().is_err());
    }

    /// A test to check that with a program of length 2, the program can move
    /// right exactly once
    #[test]
    fn test_moving_forwards() {
        let program =
            BfProgram::new(String::from("dkl.,<>sjf;ahg"), "filename.bf")
                .expect("Something went wrong with this test");
        let mut vm = VirtualMachine::<u8>::new(&program, 2, false);

        // If the tape has length of 2, and starts at the first position, then
        // it should be able to move just once

        assert!(vm.check_head_location().is_ok());
        assert!(vm.move_right().is_ok());
        assert!(vm.check_head_location().is_ok());
        assert!(vm.move_right().is_err());
        assert!(vm.check_head_location().is_err())
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

    #[test]
    fn test_increment_cell_at_head() {
        let contents = String::from("dx.knks");
        let filename = "test.bf";
        let program = BfProgram::new(contents, filename).unwrap();
        let mut virtual_machine = VirtualMachine::<u8>::new(&program, 2, false);

        // The virtual machine should start with a program position of 0 (the
        // first instruction in the list of instructions)
        assert_eq!(virtual_machine.program_position, 0);
        // The machine should start with the tape head at 0
        assert_eq!(virtual_machine.tape_head, 0);
        // With a value of 0 in that cell
        assert_eq!(virtual_machine.tape[virtual_machine.tape_head], 0);
        // When incremented, it should return the next program position, which
        // should be 1
        assert_eq!(virtual_machine.increment_cell_at_head().unwrap(), 1);
        // The value at the tape head should be now be 1
        assert_eq!(virtual_machine.tape[virtual_machine.tape_head], 1);
    }

    #[test]
    fn test_increment_cell_at_head_wrapping() {
        let contents = String::from("dx.knks");
        let filename = "test.bf";
        let program = BfProgram::new(contents, filename).unwrap();
        let mut virtual_machine = VirtualMachine::<u8>::new(&program, 2, false);

        assert_eq!(virtual_machine.program_position, 0);
        assert_eq!(virtual_machine.tape_head, 0);
        // Set the value in the tape to 255
        virtual_machine.tape[virtual_machine.tape_head] = 255;
        assert_eq!(virtual_machine.increment_cell_at_head().unwrap(), 1);
        // Upon incrementation, it should wrap around to 0
        assert_eq!(virtual_machine.tape[virtual_machine.tape_head], 0);
    }

    #[test]
    fn test_decrement_cell_at_head_wrapping() {
        let contents = String::from("dx.knks");
        let filename = "test.bf";
        let program = BfProgram::new(contents, filename).unwrap();
        let mut virtual_machine = VirtualMachine::<u8>::new(&program, 2, false);

        assert_eq!(virtual_machine.program_position, 0);
        assert_eq!(virtual_machine.tape_head, 0);
        assert_eq!(virtual_machine.tape[virtual_machine.tape_head], 0);
        assert_eq!(virtual_machine.decrement_cell_at_head().unwrap(), 1);
        // In the case of decrementation, with the value originally at 0, it
        // should wrap around to 255
        assert_eq!(virtual_machine.tape[virtual_machine.tape_head], 255);
    }

    #[test]
    fn test_decrement_cell_at_head() {
        let contents = String::from("dx.knks");
        let filename = "test.bf";
        let program = BfProgram::new(contents, filename).unwrap();
        let mut virtual_machine = VirtualMachine::<u8>::new(&program, 2, false);

        assert_eq!(virtual_machine.program_position, 0);
        assert_eq!(virtual_machine.tape_head, 0);
        // Set the value in the tape to 1
        virtual_machine.tape[virtual_machine.tape_head] = 1;
        assert_eq!(virtual_machine.decrement_cell_at_head().unwrap(), 1);
        assert_eq!(virtual_machine.tape[virtual_machine.tape_head], 0);
    }

    #[test]
    fn test_start_loop() {
        let contents = String::from("[some,.],.program");
        let filename = "test.bf";
        let program = BfProgram::new(contents, filename).unwrap();
        let mut virtual_machine =
            VirtualMachine::<u8>::new(&program, 10, false);

        assert_eq!(virtual_machine.start_loop().unwrap(), 3);
    }

    #[test]
    fn test_end_loop() {
        let contents = String::from("[some,.],.program");
        let filename = "test.bf";
        let program = BfProgram::new(contents, filename).unwrap();
        let mut virtual_machine =
            VirtualMachine::<u8>::new(&program, 10, false);

        // Set the head of the tape to 3 so it is at the closing loop
        virtual_machine.tape_head = 3;
        // In this case, it end_loop() should return the position of the
        // instruction after the corresponding opening bracket, for this program
        // it is at position 1.
        assert_eq!(virtual_machine.end_loop().unwrap(), 1);
    }
}
