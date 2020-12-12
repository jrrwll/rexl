use std::ops::*;
use crate::{Element, Matrix, Size, NumericElement, NumericMatrix};
use crate::format::Multidimensional;
use crate::transform::ElementaryTransformation;

impl<T: Element> Size for Multidimensional<T> {
    #[inline]
    fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    fn columns(&self) -> usize {
        self.columns
    }
}

impl<T: Element> Matrix for Multidimensional<T> {
    type Element = T;

    fn nonzeros(&self) -> usize {
        self.values
            .iter()
            .map(|value| {
                value.iter().fold(0, |sum, &elem|
                    if elem.is_zero() { sum } else { sum + 1 })
            }).sum()
    }

    #[inline]
    fn zero<S: Size>(size: S) -> Self {
        Self::new(size)
    }
}

impl<T> NumericMatrix for Multidimensional<T>
where T: NumericElement + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Neg<Output=T> {
    type Element = T;
}

impl<T> ElementaryTransformation<T> for Multidimensional<T>
    where T: NumericElement + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Neg<Output=T> {

    fn multiply_row(&mut self, dest: usize, multiply: T) {
        for j in 0..self.columns {
            self[(dest, j)] = self[(dest, j)] * multiply;
        }
    }

    fn add_other_row(&mut self, dest: usize, src: usize, multiply: T) {
        for j in 0..self.columns {
            self[(dest, j)] = self[(dest, j)] + self[(src, j)] * multiply;
        }
    }

    fn swap_row(&mut self, i1: usize, i2: usize) {
        for j in 0..self.columns {
            let tmp = self[(i1, j)];
            self[(i1, j)] = self[(i2, j)];
            self[(i2, j)] = tmp;
        }
    }

    fn multiply_col(&mut self, dest: usize, multiply: T) {
        for i in 0..self.rows {
            self[(i, dest)] = self[(i, dest)] * multiply;
        }
    }

    fn add_other_col(&mut self, dest: usize, src: usize, multiply: T) {
        for i in 0..self.rows {
            self[(i, dest)] = self[(i, dest)] + self[(i, src)] * multiply;
        }
    }

    fn swap_col(&mut self, j1: usize, j2: usize) {
        for i in 0..self.rows {
            let tmp = self[(i, j1)];
            self[(i, j1)] = self[(i, j2)];
            self[(i, j2)] = tmp;
        }
    }
}