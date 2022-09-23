//! A representation of the possible errors that may arise within the Virtual
//! Machine, either at runtime, or during the bracket analysis phase prior to
//! runtime.
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
        /// Current line of the errored program
        line: usize,
        /// Current column of the errored program
        column: usize,
        /// The operation in question which causes the error
        operation: Operation,
        /// The filename of the program
        filename: String,
        /// The current position of the tape prior to the new operation
        position: usize,
        /// The current tape length, to show the range of valid values
        tape_length: usize,
    },

    /// An error corresponding to the failure to read into a cell
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("unmatched {bracket} on line {line} column {column}")]
    /// Corresponds to the case in which there are unpaired brackets in the
    /// Brainfuck Program, which would lead to problems at runtime.
    UnmatchedBracket {
        /// The bracket in question which does not have a corresponding matching
        /// bracket.
        bracket: char,
        /// The current line of the unmatched bracket
        line: usize,
        /// The current column of the unmatched bracket
        column: usize,
    },

    #[error("Failure to find the brackets")]
    /// A specific failure in the case that the bracket checker does not find a
    /// matching bracket, yet still allows the program to run. If this were to
    /// happen, the program would fail, and this error will indicate a failure
    /// in the aforementioned bracket checker.
    BracketFailure,
}
