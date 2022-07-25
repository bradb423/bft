use bft_types::instruction_description;
use bft_types::BfProgram;

/// A "Virtual Machine" for the Brainfuck program to be interpreted in.
/// This struct consists of a Tape (an array of numbers) and a Head (a pointer
/// to the a position in the array).
///
/// Classical Brainfuck programs have byte size numbers (0 to 255) and the size
/// of the array is by default set at 30,000.
#[derive(Debug)]
pub struct VirtualMachine<T> {
    tape: Vec<T>,
    tape_length: usize,
    tape_head: usize,
    growable: bool,
}

impl<T> VirtualMachine<T> {
    pub fn new(mut tape_length: usize, growable: bool) -> Self {
        if tape_length == 0 {
            tape_length = 30000
        }
        Self {
            tape: Vec::with_capacity(tape_length),
            tape_length,
            tape_head: 0,
            growable,
        }
    }
    /// Interpreter function for interpreting the program. Currently, this
    /// just prints out the commands of the program
    pub fn interpret(&self, program: &BfProgram) {
        let filename = program.filename();
        for instruction in program.instructions() {
            println!(
                "[{} : {} : {} ] {}",
                filename.display(),
                instruction.line(),
                instruction.column(),
                instruction_description(instruction.instruction())
            );
        }
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
