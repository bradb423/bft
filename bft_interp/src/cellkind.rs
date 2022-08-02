/// Trait to define extra methods for incrementing and decrementing the values
/// in the cells of the Brainfuck program.
pub trait CellKind {
    /// Wrapped incrementation of the value in a given cell
    fn increment(&self) -> Self;
    /// Wrapped decrementation of the value in a given cell
    fn decrement(&self) -> Self;

    /// Converts from u8 for IO
    fn from_u8(value: u8) -> Self;

    /// Converts to u8 for IO
    fn into_u8(&self) -> u8;

    fn from_usize(value: usize) -> Self;
}

impl CellKind for u8 {
    fn increment(&self) -> Self {
        self.wrapping_add(1)
    }

    fn decrement(&self) -> Self {
        self.wrapping_sub(1)
    }

    fn from_u8(value: u8) -> Self {
        value
    }

    fn into_u8(&self) -> u8 {
        *self
    }

    fn from_usize(value: usize) -> Self {
        value as u8
    }
}
