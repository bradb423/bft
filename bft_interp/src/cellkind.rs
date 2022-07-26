/// Trait to define extra methods for incrementing and decrementing the values
/// in the cells of the Brainfuck program.
pub trait CellKind {
    /// Wrapped incrementation of the value in a given cell
    fn increment(&self) -> Self;
    /// Wrapped decrementation of the value in a given cell
    fn decrement(&self) -> Self;
}

impl CellKind for u8 {
    fn increment(&self) -> Self {
        self.wrapping_add(1)
    }

    fn decrement(&self) -> Self {
        self.wrapping_sub(1)
    }
}
