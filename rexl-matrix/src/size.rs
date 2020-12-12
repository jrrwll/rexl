use std::cmp;

/// A size.
pub trait Size {
    /// Return the number of rows.
    fn rows(&self) -> usize;

    /// Return the number of columns.
    fn columns(&self) -> usize;

    /// Return the number of rows and columns.
    #[inline]
    fn dimensions(&self) -> (usize, usize) {
        (self.rows(), self.columns())
    }

    #[inline]
    fn min_dimension(&self) -> usize {
        cmp::min(self.rows(), self.columns())
    }
}

impl Size for (usize, usize) {
    #[inline]
    fn rows(&self) -> usize {
        self.0
    }

    #[inline]
    fn columns(&self) -> usize {
        self.1
    }
}

impl Size for usize {
    #[inline]
    fn rows(&self) -> usize {
        *self
    }

    #[inline]
    fn columns(&self) -> usize {
        *self
    }
}
