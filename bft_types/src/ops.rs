//! The possible operations within a Brainfuck Program, and their methods.
//! This includes the generation of Operation enums via the parsing of specific
//! characters which are valid Brainfuck commands, and the display method of
//! this enum.

use std::fmt;

/// Raw Brainfuck Instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
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

impl Operation {
    /// Converts a character in a Brainfuck program into a raw instruction.
    /// Returns None if the character is not a valid Brainfuck instruction.
    pub fn char_to_operation(c: char) -> Option<Operation> {
        match c {
            '>' => Some(Operation::IncrementPointer),
            '<' => Some(Operation::DecrementPointer),
            '+' => Some(Operation::IncrementByte),
            '-' => Some(Operation::DecrementByte),
            '.' => Some(Operation::OutputByte),
            ',' => Some(Operation::InputByte),
            '[' => Some(Operation::StartLoop),
            ']' => Some(Operation::EndLoop),
            _ => None,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::IncrementPointer => write!(f, ">"),
            Operation::DecrementPointer => write!(f, "<"),
            Operation::IncrementByte => write!(f, "+"),
            Operation::DecrementByte => write!(f, "-"),
            Operation::OutputByte => write!(f, "."),
            Operation::InputByte => write!(f, ","),
            Operation::StartLoop => write!(f, "["),
            Operation::EndLoop => write!(f, "]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Operation;

    #[test]
    fn test_display_increment_pointer() {
        let increment_pointer: Operation = Operation::IncrementPointer;
        assert_eq!(increment_pointer.to_string(), ">");
    }

    #[test]
    fn test_display_decrement_pointer() {
        let decrement_pointer: Operation = Operation::DecrementPointer;
        assert_eq!(decrement_pointer.to_string(), "<");
    }

    #[test]
    fn test_display_increment_byte() {
        let increment_byte: Operation = Operation::IncrementByte;
        assert_eq!(increment_byte.to_string(), "+");
    }

    #[test]
    fn test_display_decrement_byte() {
        let decrement_byte: Operation = Operation::DecrementByte;
        assert_eq!(decrement_byte.to_string(), "-");
    }

    #[test]
    fn test_display_output_byte() {
        let output_byte: Operation = Operation::OutputByte;
        assert_eq!(output_byte.to_string(), ".");
    }

    #[test]
    fn test_display_input_byte() {
        let input_byte: Operation = Operation::InputByte;
        assert_eq!(input_byte.to_string(), ",");
    }

    #[test]
    fn test_display_start_loop() {
        let start_loop: Operation = Operation::StartLoop;
        assert_eq!(start_loop.to_string(), "[");
    }

    #[test]
    fn test_display_end_loop() {
        let end_loop: Operation = Operation::EndLoop;
        assert_eq!(end_loop.to_string(), "]");
    }
}
