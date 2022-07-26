use thiserror::Error;

use bft_types::InstructionInfo;

/// An enum to represent the types of errors that the VirtualMachine may
/// encounter when interpreting the program.
#[derive(Debug, Error)]
pub enum VirtualMachineError {
    /// The head of the tape has been moved to an invalid position.
    #[error(
        "In {filename}: line {{instruction_info.line()}}, column \
        {{instruction_info.column}} the head is moved to an invalid position \
        by the command: {{instruction_info.operation}}. The current position \
        is {position}, while it should be within 0 and {end_position}."
    )]
    InvalidHeadPosition {
        instruction_info: InstructionInfo,
        filename: String,
        position: usize,
        end_position: usize,
    },
}
