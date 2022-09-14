use std::fmt;

/// Raw Brainfuck Instruction
#[derive(Debug, PartialEq, Clone, Copy)]
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
