use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

pub mod ops;
use ops::Operation;

pub mod vm_error;

// Thanks to Kiran for the idea of using this crate
use line_col::LineColLookup;

/// A struct containing the main information surrounding a Brainfuck instruction
///
/// This includes the raw instruction itself, along with the line and column
/// number of the instruction.
#[derive(Debug, Clone, Copy)]
pub struct InstructionInfo {
    operation: Operation,
    line: usize,
    column: usize,
}

impl InstructionInfo {
    fn new(operation: Operation, line: usize, column: usize) -> Self {
        Self {
            operation,
            line,
            column,
        }
    }

    /// Accessor method to retrieve the instruction out of the overall
    /// InstructionInfo structure.
    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    /// Accessor method to retrieve the line on which a given valid instruction
    /// originates.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Accessor method to retrieve the column on which a given valid
    /// instruction originates.
    pub fn column(&self) -> usize {
        self.column
    }
}

/// Produces a description of a Brainfuck instruction
pub fn instruction_description(instruction: &Operation) -> &str {
    match instruction {
        Operation::IncrementPointer => "Increment the Data Pointer",
        Operation::DecrementPointer => "Decrement the Data Pointer",
        Operation::IncrementByte => "Increment the byte at the current pointer",
        Operation::DecrementByte => "Decrement the byte at the current pointer",
        Operation::OutputByte => "Output the byte at the current pointer",
        Operation::InputByte => {
            "Accept one byte of input at the current pointer"
        }
        Operation::StartLoop => "Start a loop",
        Operation::EndLoop => "End a loop",
    }
}

/// A struct representing a Brainfuck program, with the set of instructions, and
/// the filename of the program.
#[derive(Debug)]
pub struct BfProgram {
    /// Vector of instructions that are contained in the program.
    instructions: Vec<InstructionInfo>,
    /// The filename of the program.
    filename: PathBuf,
    // The pairs of brackets that are present in the program.
    // bracket_pairs: (usize, usize),
    bracket_matching_positions: HashMap<usize, usize>,
}

impl BfProgram {
    /// Creates a new Brainfuck program, from a given string of contents and a
    /// filename.
    pub fn new<P>(
        contents: String,
        filename: P,
    ) -> Result<Self, vm_error::VirtualMachineError>
    where
        P: AsRef<Path>,
    {
        // Once again, thanks to Kiran for the idea of using this crate
        let lookup = LineColLookup::new(&contents);

        let instructions: Vec<InstructionInfo> = contents
            .chars()
            .enumerate()
            .filter_map(|(n, c)| {
                Operation::char_to_operation(c).map(|instruction| {
                    InstructionInfo::new(instruction, lookup.get(n).0, lookup.get(n).1)
                })
            })
            .collect();
        let mut program = Self {
            instructions,
            filename: filename.as_ref().to_path_buf(),
            bracket_matching_positions: HashMap::new(),
        };
        let new_matching_positions: HashMap<usize, usize> =
            program.bracket_check()?;
        program.bracket_matching_positions = new_matching_positions;
        Ok(program)
    }

    /// Reads directly from a file, to produce a Brainfuck program.
    pub fn from_file<P>(filename: P) -> Result<BfProgram, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(&filename)?;
        Ok(BfProgram::new(contents, filename)?)
    }

    /// Accessor method to retrieve the instructions from a program.
    pub fn instructions(&self) -> &Vec<InstructionInfo> {
        &self.instructions
    }

    /// Accessor method to retrieve the filename from a program.
    pub fn filename(&self) -> &Path {
        &self.filename
    }

    pub fn bracket_matching_positions(&self) -> &HashMap<usize, usize> {
        &self.bracket_matching_positions
    }

    pub fn bracket_check(
        &self,
    ) -> Result<HashMap<usize, usize>, vm_error::VirtualMachineError> {
        let mut bracket_stack: Vec<usize> = Vec::new();
        let mut matching_bracket_positions: HashMap<usize, usize> =
            HashMap::new();

        // Line number of the most recent opening bracket.
        let mut latest_line: usize = 0;
        // Column number of the most recent opening bracket.
        let mut latest_column: usize = 0;
        for (position, instruction) in self.instructions().iter().enumerate() {
            match instruction.operation() {
                &Operation::StartLoop => {
                    // If we have an opening bracket, then we should add it to
                    // the stack
                    bracket_stack.push(position);
                    latest_line = instruction.line();
                    latest_column = instruction.column();
                }
                Operation::EndLoop => {
                    if let Some(p) = bracket_stack.last() {
                        matching_bracket_positions.insert(*p, position);
                    }
                    // If there are too many closing brackets, then popping
                    // will cause an error which we should percolate up.
                    bracket_stack.pop().ok_or(
                        vm_error::VirtualMachineError::UnmatchedBracket {
                            bracket: ']',
                            line: instruction.line(),
                            column: instruction.column(),
                        },
                    )?;
                }
                _ => {}
            }
        }

        // If the bracket stack is not empty after the full loop, then this is
        // due to there being too many opening brackets
        if !bracket_stack.is_empty() {
            return Err(vm_error::VirtualMachineError::UnmatchedBracket {
                bracket: '[',
                line: latest_line,
                column: latest_column,
            });
        }
        Ok(matching_bracket_positions)
    }
}
