use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Raw Brainfuck Instruction
#[derive(Debug)]
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
    instructions: Vec<Instruction>,
    filename: PathBuf,
}

impl BfProgram {
    /// Creates a new Brainfuck program, from a given string of contents and a
    /// filename.
    pub fn new<P>(contents: String, filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        let instructions = contents
            .chars()
            .filter_map(Instruction::char_to_instruction)
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

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn filename(&self) -> &Path {
        &self.filename
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
