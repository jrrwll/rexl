use crate::format::Multidimensional;
use crate::{Element, Variant};
use std::mem;

pub struct MultidimensionalIterator<'a, T: 'a + Element> {
    pub matrix: &'a Multidimensional<T>,
    pub column_offset: usize,
    pub row_offset: usize,
    pub column_size: usize,
    pub row_size: usize,
    pub variant: Variant,
}

pub struct MultidimensionalIteratorMut<'a, T: 'a + Element> {
    pub matrix: &'a mut Multidimensional<T>,
    pub column_offset: usize,
    pub row_offset: usize,
    pub column_size: usize,
    pub row_size: usize,
    pub variant: Variant,
}

impl<'a, T: Element> Iterator for MultidimensionalIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut MultidimensionalIterator {
            ref matrix,
            ref mut column_offset,
            ref mut row_offset,
            ..
        } = self;

        let rows = *row_offset;
        let columns = *column_offset;
        if rows == self.row_size || columns == self.column_size {
            return None;
        }

        let item = unsafe { mem::transmute(&matrix.values[columns][rows]) };
        match self.variant {
            Variant::Column => {
                if rows == self.row_size - 1 {
                    *row_offset = 0;
                    *column_offset = columns + 1;
                } else {
                    *row_offset = rows + 1;
                }
            }
            Variant::Row => {
                if columns == self.column_size - 1 {
                    *column_offset = 0;
                    *row_offset = rows + 1;
                } else {
                    *column_offset = columns + 1;
                }
            }
        }
        Some(item)
    }
}

impl<'a, T: Element> Iterator for MultidimensionalIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut MultidimensionalIteratorMut {
            ref mut matrix,
            ref mut column_offset,
            ref mut row_offset,
            ..
        } = self;

        let rows = *row_offset;
        let columns = *column_offset;
        if rows == self.row_size || columns == self.column_size {
            return None;
        }

        let item = unsafe { mem::transmute(&mut matrix.values[columns][rows]) };
        match self.variant {
            Variant::Column => {
                if rows == self.row_size - 1 {
                    *row_offset = 0;
                    *column_offset = columns + 1;
                } else {
                    *row_offset = rows + 1;
                }
            }
            Variant::Row => {
                if columns == self.column_size - 1 {
                    *column_offset = 0;
                    *row_offset = rows + 1;
                } else {
                    *column_offset = columns + 1;
                }
            }
        }
        Some(item)
    }
}
