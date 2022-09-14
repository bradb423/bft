use thiserror::Error;

use crate::ops::Operation;

/// An enum to represent the types of errors that the VirtualMachine may
/// encounter when interpreting the program.
#[derive(Debug, Error)]
pub enum VirtualMachineError {
    /// The head of the tape has been moved to an invalid position.
    #[error(
        "In {filename}: line {line}, column \
        {column} the head is moved to an invalid position \
        by the command: {operation}. The current position \
        is {position}, while it should be within 0 and {tape_length}."
    )]
    InvalidHeadPosition {
        line: usize,
        column: usize,
        operation: Operation,
        filename: String,
        position: usize,
        tape_length: usize,
    },

    /// An error corresponding to the failure to read into a cell
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("unmatched {bracket} on line {line} column {column}")]
    UnmatchedBracket {
        bracket: char,
        line: usize,
        column: usize,
    },

    #[error("Failure to find the brackets")]
    BracketFailure,
}
