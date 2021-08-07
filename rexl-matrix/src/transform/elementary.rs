use crate::format::*;
use crate::Element;
use std::ops::*;

/// The elementary transformation of a matrix
pub trait ElementaryTransformation<T: Element> {
    /// row
    fn multiply_row(&mut self, dest: usize, multiply: T);

    fn add_other_row(&mut self, dest: usize, src: usize, multiply: T);

    fn swap_row(&mut self, i1: usize, i2: usize);

    /// col
    fn multiply_col(&mut self, dest: usize, multiply: T);

    fn add_other_col(&mut self, dest: usize, src: usize, multiply: T);

    fn swap_col(&mut self, j1: usize, j2: usize);
}

macro_rules! implement_elementary_transformation {
    ($name:ident) => {
        impl<T> ElementaryTransformation<T> for $name<T>
        where
            T: Element + Add<Output = T> + Mul<Output = T>,
        {
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
    };
}

implement_elementary_transformation!(Multidimensional);
implement_elementary_transformation!(Conventional);
