use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub mod ops;

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
    pub fn line(&self) -> usize {
        self.position.0
    }

    /// Accessor method to retrieve the column on which a given valid
    /// instruction originates.
    pub fn column(&self) -> usize {
        self.position.1
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
