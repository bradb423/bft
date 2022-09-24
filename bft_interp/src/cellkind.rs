#![deny(missing_docs)]

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
    fn to_u8(&self) -> u8;
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

    fn to_u8(&self) -> u8 {
        *self
    }
}
#[cfg(test)]
mod tests {
    use super::CellKind;

    #[test]
    fn test_increment() {
        let t = 0u8;
        assert_eq!(t.increment(), 1u8);
    }

    #[test]
    fn test_increment_wrapping() {
        let t = 255u8;
        assert_eq!(t.increment(), 0u8);
    }

    #[test]
    fn test_decrement() {
        let t = 255u8;
        assert_eq!(t.decrement(), 254u8);
    }
    #[test]
    fn test_decrement_wrapping() {
        let t = 0u8;
        assert_eq!(t.decrement(), 255u8);
    }
}
