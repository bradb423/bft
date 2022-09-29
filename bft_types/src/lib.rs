//! bft_types, handling the types of operations. And creating the Brainfuck
//! Program.

#![deny(missing_docs)]

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
    /// The raw instruction.
    operation: Operation,
    /// The line on which the instruction is found.
    line: usize,
    /// The column on which the instruction is found.
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

    /// Retrieves the raw instruction.
    pub fn operation(&self) -> Operation {
        self.operation
    }

    /// Retrieves the line on which a given instruction is found.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Retrieves the column on which a given instruction is found.
    pub fn column(&self) -> usize {
        self.column
    }
}

/// A Brainfuck program, with the set of instructions, the filename of the
/// program, and the pairs of opening and closing brackets representing the
/// loops of the program.
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
    ///
    /// For example:
    /// Given a Brainfuck program, named 'hello.bf', with the contents:
    /// 'f[+-hello]+-,.'
    /// The the program could be loaded from the following:
    /// ```
    /// use bft_types::{BfProgram, InstructionInfo, ops::Operation};
    /// let contents: String = "f[+-hello]+-,.".to_string();
    /// let filename = "hello.bf";
    /// let new_program = BfProgram::new(contents, filename).unwrap();
    ///
    /// // We can check that the first instruction of the program is a'['
    /// // by looking at the first element of the instructions vector and
    /// // making sure that it is the start of a loop. We can also check its
    /// // position, which should be line 1, column 2 (As any characters which
    /// // are not valid Brainfuck characters are treated as comments, thus the
    /// // 'f' at the beginning of the program is ignored.)
    /// let first_instruction: InstructionInfo = new_program.instructions()[0];
    /// assert_eq!(first_instruction.operation(), Operation::StartLoop);
    /// assert_eq!(first_instruction.line(), 1);
    /// assert_eq!(first_instruction.column(), 2);
    /// ```
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
                    InstructionInfo::new(
                        instruction,
                        lookup.get(n).0,
                        lookup.get(n).1,
                    )
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
    /// Given a program file named 'path/to/program.bf', we can load the
    /// program from the file as follows:
    /// ```
    /// use bft_types::BfProgram;
    /// let new_program = BfProgram::from_file("path/to/program.bf");
    /// ```
    pub fn from_file<P>(filename: P) -> Result<BfProgram, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(&filename)?;
        Ok(BfProgram::new(contents, filename)?)
    }

    /// Retrieves the list of instructions present in a given program.
    pub fn instructions(&self) -> &Vec<InstructionInfo> {
        &self.instructions
    }

    /// Retrieves the filename of the program.
    pub fn filename(&self) -> &Path {
        &self.filename
    }

    /// A hashmap describing the positions of pairs of matching brackets
    pub fn bracket_matching_positions(&self) -> &HashMap<usize, usize> {
        &self.bracket_matching_positions
    }

    /// Checks the program for brackets which can be paired, these will later
    /// signify the loops within the Brainfuck Program. In the case of unmatched
    /// brackets, this method will return an error detailing the position of the
    /// unmatched bracket, along with its type. Furthermore, upon finding
    /// unmatched brackets, `bft` will stop and no interpreting will happen from
    /// this point onwards.
    /// For example:
    /// ```
    /// // Given a program named 'test.bf', with contents '[]', the bracket
    /// // should give the hashmap of positions, and produce no error.
    /// # use std::collections::HashMap;
    /// # use bft_types::BfProgram;
    /// let filename = "test.bf";
    /// let contents = "[]".to_string();
    /// let balanced_program: BfProgram = BfProgram::new(contents, filename).unwrap();
    ///
    /// assert!(balanced_program.bracket_check().is_ok());
    /// let bracket_positions: HashMap<usize,usize> = balanced_program.bracket_check().unwrap();
    /// // We can then check that the first and second brackets are paired
    /// // correctly. The first bracket is at the 0th position in a list of brackets, and the second
    /// // bracket is at the 1st position.
    /// assert_eq!(bracket_positions.get(&0).unwrap(), &1);
    /// ```
    /// In the case of an unbalanced program, the bracket_check() will return an
    /// error as follows:
    /// ```
    /// # use bft_types::BfProgram;
    /// let filename = "test.bf";
    /// let contents = "[]]".to_string();
    /// // As the bracket_check is called within the new() method, this should
    /// // return an error from the get-go.
    /// let unbalanced_program = BfProgram::new(contents, filename);
    /// assert!(unbalanced_program.is_err());
    /// ```
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
                Operation::StartLoop => {
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
