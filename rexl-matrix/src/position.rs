/// A position.
pub trait Position {
    /// Return the row.
    fn row(&self) -> usize;

    /// Return the column.
    fn column(&self) -> usize;

    /// Return the row and column.
    #[inline]
    fn coordinates(&self) -> (usize, usize) {
        (self.row(), self.column())
    }
}

impl Position for (usize, usize) {
    #[inline]
    fn row(&self) -> usize {
        self.0
    }

    #[inline]
    fn column(&self) -> usize {
        self.1
    }
}

impl Position for usize {
    #[inline]
    fn row(&self) -> usize {
        *self
    }

    #[inline]
    fn column(&self) -> usize {
        *self
    }
}
