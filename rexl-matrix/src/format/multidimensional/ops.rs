use crate::format::Multidimensional;
use crate::Element;
use std::ops::*;

impl<T: Element> Index<(usize, usize)> for Multidimensional<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (rows, columns) = index;
        &self.values[columns][rows]
    }
}

impl<T: Element> IndexMut<(usize, usize)> for Multidimensional<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (rows, columns) = index;
        &mut self.values[columns][rows]
    }
}
