pub use self::ops::*;
pub use self::iter::*;

mod ops;
mod iter;

use std::ptr;
use crate::{Size, Element, Matrix, Variant};

/// A conventional matrix based on a multidimensional array
#[derive(Clone, Debug, PartialEq)]
pub struct Multidimensional<T: Element> {
    /// The number of rows.
    pub rows: usize,
    /// The number of columns.
    pub columns: usize,
    /// The values stored in the column-major order.
    pub values: Vec<Vec<T>>,
}

impl<T: Element> Multidimensional<T> {

    /// Create a zero matrix.
    pub fn new<S: Size>(size: S) -> Self {
        let (rows, columns) = size.dimensions();
        Multidimensional {
            rows,
            columns,
            values: vec![vec![T::zero(); rows]; columns]
        }
    }

    /// Create a matrix from a vector.
    pub fn from_vec<S: Size>(size: S, values: Vec<Vec<T>>) -> Self {
        let (rows, columns) = size.dimensions();
        Multidimensional {
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
        self.each_column(#[inline] |value|{
            ptr::write_bytes(value.as_mut_ptr(), 0, value.len());
        });
    }

    /// Resize.
    pub fn resize<S: Size>(&mut self, size: S) {
        let (rows, columns) = size.dimensions();
        let (old_rows, old_columns) = (self.rows, self.columns);

        if old_columns > columns {
            self.values.truncate(columns);
        } else {
            self.values.extend(vec![vec![T::zero(); rows]; columns - old_columns]);
        }

        if old_rows > rows {
            self.each_column(#[inline] |value|{
                value.truncate(rows);
            });
        } else {
            self.each_column(#[inline] |value|{
                value.extend(vec![T::zero(); rows - old_rows]);
            });
        }
        self.rows = rows;
        self.columns = columns;
    }

    fn each_column<F: Fn(&mut Vec<T>) -> ()>(&mut self, f: F) {
        let values = &mut self.values;
        for value in values {
            f(value);
        }
    }

    pub fn to_string(&self) -> String {
        self.values.iter().map(|value|
            value.iter()
                .map(|elem| format!("{:?}", elem))
                .collect::<Vec<String>>().join(", "))
            .collect::<Vec<String>>().join("\n")
    }

    pub fn iter(&self, variant: Variant) -> MultidimensionalIterator<T> {
        let (rows, columns) = self.dimensions();
        MultidimensionalIterator {
            matrix: self,
            row_size: rows,
            column_size: columns,
            row_offset: 0,
            column_offset: 0,
            variant,
        }
    }

    pub fn iter_mut(&mut self, variant: Variant) -> MultidimensionalIteratorMut<T> {
        let (rows, columns) = self.dimensions();
        MultidimensionalIteratorMut {
            matrix: self,
            row_size: rows,
            column_size: columns,
            row_offset: 0,
            column_offset: 0,
            variant,
        }
    }
}

impl<T: Element> Matrix for Multidimensional<T> {
    type Element = T;

    fn nonzeros(&self) -> usize {
        self.values.iter().map(|value| {
            value.iter().fold(0, |sum, &elem|
                if elem.is_zero() { sum } else { sum + 1 })
        }).sum()
    }

    #[inline]
    fn zero<S: Size>(size: S) -> Self {
        Self::new(size)
    }

    fn augment_vec_assign(&mut self, vector: &Vec<Self::Element>) {
        self.values.push(vector.clone());
    }
}