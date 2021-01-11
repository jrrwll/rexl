use crate::{Element, Size, Vector};
use std::cmp;
use std::ops::*;

/// A matrix.
pub trait Matrix:
    IndexMut<(usize, usize), Output = <Self as Matrix>::Element> + Size + Sized + Clone {
    /// basic functions

    /// The element type.
    type Element: Element;

    /// Count nonzero elements.
    fn nonzeros(&self) -> usize;

    /// Create a zero matrix.
    fn zero<S: Size>(size: S) -> Self;

    /// build

    fn one<S: Size>(size: S) -> Self {
        let mut this = Self::zero(size);
        let (rows, columns) = this.dimensions();

        for i in 0..rows {
            for j in 0..columns {
                this[(i, j)] = <Self as Matrix>::Element::one();
            }
        }
        this
    }

    fn eye<S: Size>(size: S) -> Self {
        let mut this = Self::zero(size);
        let dimension = this.min_dimension();

        for i in 0..dimension {
            this[(i, i)] = <Self as Matrix>::Element::one();
        }
        this
    }

    #[inline]
    fn diag(diag: &Vector<<Self as Matrix>::Element>) -> Self {
        Self::diag_vec(&diag.data)
    }

    fn diag_vec(diag: &Vec<<Self as Matrix>::Element>) -> Self {
        let mut this = Self::zero(diag.len());
        let dimension = this.min_dimension();
        for i in 0..dimension {
            this[(i, i)] = diag[i];
        }
        this
    }

    /// getters & setter

    #[inline]
    fn get_row(&self, i: usize) -> Vector<Self::Element> {
        Vector::from(self.get_row_vec(i))
    }

    #[inline]
    fn set_row(&mut self, i: usize, vector: &Vector<Self::Element>) {
        self.set_row_vec(i, &vector.data);
    }

    #[inline]
    fn get_column(&self, j: usize) -> Vector<Self::Element> {
        Vector::from(self.get_column_vec(j))
    }

    #[inline]
    fn set_column(&mut self, j: usize, vector: &Vector<Self::Element>) {
        self.set_column_vec(j, &vector.data);
    }

    fn get_row_vec(&self, i: usize) -> Vec<Self::Element> {
        let columns = self.columns();
        let mut result = Vec::with_capacity(columns);
        for j in 0..columns {
            result.push(self[(i, j)]);
        }
        result
    }

    fn set_row_vec(&mut self, i: usize, vector: &Vec<Self::Element>) {
        let columns = cmp::min(self.columns(), vector.len());
        for j in 0..columns {
            self[(i, j)] = vector[j];
        }
    }

    fn get_column_vec(&self, j: usize) -> Vec<Self::Element> {
        let rows = self.rows();
        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            result.push(self[(i, j)]);
        }
        result
    }

    fn set_column_vec(&mut self, j: usize, vector: &Vec<Self::Element>) {
        let rows = cmp::min(self.rows(), vector.len());
        for i in 0..rows {
            self[(i, j)] = vector[j];
        }
    }

    /// matrix functions

    fn sub_matrix(&self, i1: usize, i2: usize, j1: usize, j2: usize) -> Self {
        let new_rows = i2 - i1 + 1;
        let new_columns = j2 - j1 + 1;
        let mut result = Self::zero((new_rows, new_columns));
        for i in 0..new_rows {
            for j in 0..new_columns {
                result[(i, j)] = self[(i1 + i, j1 + j)];
            }
        }
        result
    }

    fn augment(&self, vector: &Vector<Self::Element>) -> Self {
        let mut result = self.clone();
        result.augment_assign(vector);
        result
    }

    #[inline]
    fn augment_assign(&mut self, vector: &Vector<Self::Element>) {
        self.augment_vec_assign(&vector.data);
    }

    fn augment_vec(&self, vector: &Vec<Self::Element>) -> Self {
        let mut result = self.clone();
        result.augment_vec_assign(vector);
        result
    }

    fn augment_vec_assign(&mut self, vector: &Vec<Self::Element>);

    #[inline]
    fn complement(&self, i: usize, j: usize) -> Self {
        self.complement_bulk(i, i, j, j)
    }

    fn complement_bulk(&self, i1: usize, j1: usize, i2: usize, j2: usize) -> Self {
        let (rows, columns) = self.dimensions();
        let di = i2 - i1 + 1;
        let dj = j2 - j1 + 1;
        let new_rows = rows - di;
        let new_columns = columns - dj;
        let mut result = Self::zero((new_rows, new_columns));
        for i in 0..new_rows {
            for j in 0..new_columns {
                result[(i, j)] = if i < i1 {
                    if j < j1 {
                        self[(i, j)]
                    } else {
                        self[(i, j + dj + 1)]
                    }
                } else {
                    if j < j1 {
                        self[(i + di + 1, j)]
                    } else {
                        self[(i + di + 1, j + dj + 1)]
                    }
                };
            }
        }
        result
    }

    fn transpose(&self) -> Self {
        let (rows, columns) = self.dimensions();
        let mut result = Self::zero((columns, rows));
        for i in 0..rows {
            for j in 0..columns {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }

    #[inline]
    fn transpose_assign(&mut self) {
        *self = self.transpose();
    }

    fn triu(&self) -> Self {
        let rows = self.rows();
        let mut result = self.clone();
        for i in 0..rows {
            for j in 0..=i {
                result[(i, j)] = self[(i, j)];
            }
        }
        result
    }

    fn tril(&self) -> Self {
        let columns = self.columns();
        let mut result = self.clone();
        for j in 0..columns {
            for i in 0..=j {
                result[(i, j)] = self[(i, j)];
            }
        }
        result
    }
}
