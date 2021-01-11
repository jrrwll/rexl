pub use self::iter::*;
pub use self::ops::*;

mod iter;
mod ops;

use crate::{Element, Matrix, Size};
use std::{cmp, ptr};

#[derive(Clone, Debug, PartialEq)]
pub struct Conventional<T: Element> {
    /// The number of rows.
    pub rows:    usize,
    /// The number of columns.
    pub columns: usize,
    /// The values stored in the column-major order.
    pub values:  Vec<T>,
}

impl<T: Element> Conventional<T> {
    /// Create a zero matrix.
    pub fn new<S: Size>(size: S) -> Self {
        let (rows, columns) = size.dimensions();
        Conventional {
            rows,
            columns,
            values: vec![T::zero(); columns * rows],
        }
    }

    /// Create a matrix from a vector.
    pub fn from_vec<S: Size>(size: S, values: Vec<T>) -> Self {
        let (rows, columns) = size.dimensions();
        Conventional {
            rows,
            columns,
            values,
        }
    }

    /// Zero out the content.
    ///
    /// The function should only be used when it is safe to overwrite `T` with
    /// zero bytes.
    #[inline]
    pub unsafe fn erase(&mut self) {
        let values = &mut self.values;
        ptr::write_bytes(values.as_mut_ptr(), 0, values.len());
    }

    /// Resize.
    pub fn resize<S: Size>(&mut self, size: S) {
        let (rows, columns) = size.dimensions();
        let (old_rows, old_columns) = (self.rows, self.columns);
        self.rows = rows;
        self.columns = columns;

        if old_rows == rows {
            if old_columns > columns {
                self.values.truncate((old_columns - columns) * rows);
            } else {
                self.values
                    .extend(vec![T::zero(); (columns - old_columns) * rows]);
            }
            return
        }

        let mut matrix = Self::zero(size);
        let min_rows = cmp::min(old_rows, rows);
        let min_columns = cmp::min(old_columns, columns);
        for j in 0..min_columns {
            for i in 0..min_rows {
                matrix[(i, j)] = self[(i, j)];
            }
        }
        *self = matrix;
    }
}

impl<T: Element> Matrix for Conventional<T> {
    type Element = T;

    fn nonzeros(&self) -> usize {
        self.values
            .iter()
            .fold(0, |sum, &elem| if elem.is_zero() { sum } else { sum + 1 })
    }

    fn zero<S: Size>(size: S) -> Self {
        Self::new(size)
    }

    fn augment_vec_assign(&mut self, vector: &Vec<Self::Element>) {
        self.values.extend(vector);
    }
}
