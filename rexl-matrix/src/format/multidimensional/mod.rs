pub use self::multidimensional::*;
pub use self::ops::*;

mod multidimensional;
mod ops;

use std::ptr;
use crate::{Size, Element};

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
        debug_assert!(values.len() == columns && values.len() > 0);
        debug_assert_eq!(values.len() * values[0].len(), rows * columns);
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
        // let values = &mut self.values;
        // for value in values {
        //     ptr::write_bytes(value.as_mut_ptr(), 0, value.len());
        // }
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
}
