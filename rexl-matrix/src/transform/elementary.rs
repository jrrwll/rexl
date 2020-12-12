use crate::Element;

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