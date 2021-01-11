use crate::format::Conventional;
use crate::Element;
use std::ops::*;

impl<T: Element> Index<(usize, usize)> for Conventional<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (rows, columns) = index;
        &self.values[columns * self.rows + rows]
    }
}

impl<T: Element> IndexMut<(usize, usize)> for Conventional<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (rows, columns) = index;
        &mut self.values[columns * self.rows + rows]
    }
}
