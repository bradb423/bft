use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// Thanks to Kiran for the idea of using this crate
use line_col::LineColLookup;

/// Raw Brainfuck Instruction
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Represents the `>` character
    IncrementPointer,
    /// Represents the `<` character
    DecrementPointer,
    /// Represents the `+` character
    IncrementByte,
    /// Represents the `-` character
    DecrementByte,
    /// Represents the `.` character
    OutputByte,
    /// Represents the `,` character
    InputByte,
    /// Represents the `[` character
    StartLoop,
    /// Represents the `]` character
    EndLoop,
}

impl Instruction {
    /// Converts a character in a Brainfuck program into a raw instruction.
    /// Returns None if the character is not a valid Brainfuck instruction.
    fn char_to_instruction(c: char) -> Option<Instruction> {
        match c {
            '>' => Some(Instruction::IncrementPointer),
            '<' => Some(Instruction::DecrementPointer),
            '+' => Some(Instruction::IncrementByte),
            '-' => Some(Instruction::DecrementByte),
            '.' => Some(Instruction::OutputByte),
            ',' => Some(Instruction::InputByte),
            '[' => Some(Instruction::StartLoop),
            ']' => Some(Instruction::EndLoop),
            _ => None,
        }
    }
}

/// A struct containing the main information surrounding a Brainfuck instruction
///
/// This includes the raw instruction itself, along with the line and column
/// number of the instruction.
#[derive(Debug)]
pub struct InstructionInfo {
    instruction: Instruction,
    position: (usize, usize),
}

impl InstructionInfo {
    fn new(instruction: Instruction, position: (usize, usize)) -> Self {
        Self {
            instruction,
            position,
        }
    }

    /// Accessor method to retrieve the instruction out of the overall
    /// InstructionInfo structure.
    pub fn instruction(&self) -> &Instruction {
        &self.instruction
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
pub fn instruction_description(instruction: &Instruction) -> &str {
    match instruction {
        Instruction::IncrementPointer => "Increment the Data Pointer",
        Instruction::DecrementPointer => "Decrement the Data Pointer",
        Instruction::IncrementByte => "Increment the byte at the current pointer",
        Instruction::DecrementByte => "Decrement the byte at the current pointer",
        Instruction::OutputByte => "Output the byte at the current pointer",
        Instruction::InputByte => "Accept one byte of input at the current pointer",
        Instruction::StartLoop => "Start a loop",
        Instruction::EndLoop => "End a loop",
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
                Instruction::char_to_instruction(c)
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
            if matches!(instruction.instruction(), Instruction::StartLoop) {
                // If there is an opening bracket, add it to the Vector.
                opening_loops.push(instruction);
            } else if matches!(instruction.instruction(), Instruction::EndLoop) {
                // If there is an already existing opening bracket, then it will
                // remove the last opening bracket from the Vector.
                // However, if there is no opening bracket, then there is one
                // too many closing brackets, and so this is not a valid program
                match opening_loops.pop() {
                    Some(_) => (),
                    None => {
                        return Err(format!(
                            "Unexpected ']' in the file {:?} at line: {} and column: {}",
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
                "Too few ']' in the file {:?} with the last opening bracket at line: {} and column: {}",
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
    use crate::{BfProgram, Instruction};

    /// A function to mock a program with instructions for associated tests.
    fn mock_instructions() -> BfProgram {
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
        let bf_program = mock_instructions();
        let instructions = bf_program.instructions();

        assert_eq!(instructions.len(), 8);
    }

    /// A check that the program has stored the instruction in the right order.
    #[test]
    fn correct_instructions() {
        let bf_program = mock_instructions();
        let instructions = bf_program.instructions();

        // assert_eq!(instructions[0].instruction, Instruction::IncrementByte);
    }

    /// Test that the program has the right line numbers for each command in the
    /// file.
    #[test]
    fn test_lines() {
        let bf_program = mock_instructions();
        let instructions = bf_program.instructions();
    }

    /// Test that the program has the right column numbers for each command in
    /// the file.
    #[test]
    fn test_columns() {
        let bf_program = mock_instructions();
        let instructions = bf_program.instructions();
    }
}
