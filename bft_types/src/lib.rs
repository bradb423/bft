use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod ops;

// Thanks to Kiran for the idea of using this crate
use line_col::LineColLookup;

/// A struct containing the main information surrounding a Brainfuck instruction
///
/// This includes the raw instruction itself, along with the line and column
/// number of the instruction.
#[derive(Debug, Clone, Copy)]
pub struct InstructionInfo {
    operation: ops::Operation,
    position: (usize, usize),
}

impl InstructionInfo {
    fn new(operation: ops::Operation, position: (usize, usize)) -> Self {
        Self {
            operation,
            position,
        }
    }

    /// Accessor method to retrieve the instruction out of the overall
    /// InstructionInfo structure.
    pub fn operation(&self) -> &ops::Operation {
        &self.operation
    }

    /// Accessor method to retrieve the line on which a given valid instruction
    /// originates.
    pub fn line(&self) -> &usize {
        &self.position.0
    }

    /// Accessor method to retrieve the column on which a given valid
    /// instruction originates.
    pub fn column(&self) -> &usize {
        &self.position.1
    }
}

/// Produces a description of a Brainfuck instruction
pub fn instruction_description(instruction: &ops::Operation) -> &str {
    match instruction {
        ops::Operation::IncrementPointer => "Increment the Data Pointer",
        ops::Operation::DecrementPointer => "Decrement the Data Pointer",
        ops::Operation::IncrementByte => "Increment the byte at the current pointer",
        ops::Operation::DecrementByte => "Decrement the byte at the current pointer",
        ops::Operation::OutputByte => "Output the byte at the current pointer",
        ops::Operation::InputByte => "Accept one byte of input at the current pointer",
        ops::Operation::StartLoop => "Start a loop",
        ops::Operation::EndLoop => "End a loop",
    }
}

/// A struct representing a Brainfuck program, with the set of instructions, and
/// the filename of the program.
#[derive(Debug)]
pub struct BfProgram {
    instructions: Vec<InstructionInfo>,
    filename: PathBuf,
}

impl BfProgram {
    /// Creates a new Brainfuck program, from a given string of contents and a
    /// filename.
    pub fn new<P>(contents: String, filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        // Once again, thanks to Kiran for the idea of using this crate
        let lookup = LineColLookup::new(&contents);

        let instructions = contents
            .chars()
            .enumerate()
            .filter_map(|(n, c)| {
                ops::Operation::char_to_operation(c)
                    .map(|instruction| InstructionInfo::new(instruction, lookup.get(n)))
            })
            .collect();
        Self {
            instructions,
            filename: filename.as_ref().to_path_buf(),
        }
    }

    /// Reads directly from a file, to produce a Brainfuck program.
    pub fn from_file<P>(filename: P) -> Result<BfProgram, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(&filename)?;
        Ok(BfProgram::new(contents, filename))
    }

    /// Accessor method to retrieve the instructions from a program.
    pub fn instructions(&self) -> &Vec<InstructionInfo> {
        &self.instructions
    }

    /// Accessor method to retrieve the filename from a program.
    pub fn filename(&self) -> &Path {
        &self.filename
    }

    pub fn bracket_check(&self) -> Result<(), String> {
        let mut opening_loops = Vec::new();
        for instruction in self.instructions() {
            if matches!(instruction.operation(), ops::Operation::StartLoop) {
                // If there is an opening bracket, add it to the Vector.
                opening_loops.push(instruction);
            } else if matches!(instruction.operation(), ops::Operation::EndLoop) {
                // If there is an already existing opening bracket, then it will
                // remove the last opening bracket from the Vector.
                // However, if there is no opening bracket, then there is one
                // too many closing brackets, and so this is not a valid program
                match opening_loops.pop() {
                    Some(_) => (),
                    None => {
                        return Err(format!(
                            "Unexpected ']' in the file {:?} at line: {} \
                            and column: {}",
                            self.filename(),
                            instruction.line(),
                            instruction.column(),
                        ))
                    }
                }
            }
        }
        // If there is another opening bracket left, then there are too many,
        // and so the program is not valid.
        match opening_loops.pop() {
            Some(instruction) => Err(format!(
                "Too few ']' in the file {:?} with the first unclosed bracket \
                at line: {} and column: {}",
                self.filename(),
                instruction.line(),
                instruction.column()
            )),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ops::Operation, BfProgram};

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

        assert_eq!(instructions[0].operation, Operation::IncrementByte);
        assert_eq!(instructions[1].operation, Operation::DecrementByte);
        assert_eq!(instructions[2].operation, Operation::StartLoop);
        assert_eq!(instructions[3].operation, Operation::EndLoop);
        assert_eq!(instructions[4].operation, Operation::IncrementPointer);
        assert_eq!(instructions[5].operation, Operation::DecrementPointer);
        assert_eq!(instructions[6].operation, Operation::OutputByte);
        assert_eq!(instructions[7].operation, Operation::InputByte);
    }

    /// Test that the program has the right line numbers for each command in the
    /// file.
    #[test]
    fn test_lines() {
        let bf_program = mock_working_program();
        let instructions = bf_program.instructions();

        assert_eq!(instructions[0].position.0, 1);
        assert_eq!(instructions[1].position.0, 1);
        assert_eq!(instructions[2].position.0, 3);
        assert_eq!(instructions[3].position.0, 3);
        assert_eq!(instructions[4].position.0, 3);
        assert_eq!(instructions[5].position.0, 3);
        assert_eq!(instructions[6].position.0, 4);
        assert_eq!(instructions[7].position.0, 4);
    }

    /// Test that the program has the right column numbers for each command in
    /// the file.
    #[test]
    fn test_columns() {
        let bf_program = mock_working_program();
        let instructions = bf_program.instructions();

        assert_eq!(instructions[0].position.1, 1);
        assert_eq!(instructions[1].position.1, 2);
        assert_eq!(instructions[2].position.1, 13);
        assert_eq!(instructions[3].position.1, 14);
        assert_eq!(instructions[4].position.1, 15);
        assert_eq!(instructions[5].position.1, 16);
        assert_eq!(instructions[6].position.1, 26);
        assert_eq!(instructions[7].position.1, 27);
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
}
