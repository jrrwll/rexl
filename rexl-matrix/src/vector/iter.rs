use crate::{Element, Vector};
use std::mem;

pub struct VectorIterator<'a, T: 'a + Element> {
    pub vector: &'a Vector<T>,
    pub taken:  usize,
    pub size:   usize,
}

pub struct VectorIteratorMut<'a, T: 'a + Element> {
    pub vector: &'a mut Vector<T>,
    pub taken:  usize,
    pub size:   usize,
}

impl<'a, T: Element> Iterator for VectorIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut VectorIterator {
            ref vector,
            ref mut taken,
            ..
        } = self;

        let k = *taken;
        if k == self.size {
            return None
        }
        *taken += 1;
        let item = unsafe { mem::transmute(&vector.data[k]) };
        Some(item)
    }
}

impl<'a, T: Element> Iterator for VectorIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut VectorIteratorMut {
            ref mut vector,
            ref mut taken,
            ..
        } = self;

        let k = *taken;
        if k == self.size {
            return None
        }
        *taken += 1;
        let item = unsafe { mem::transmute(&mut vector.data[k]) };
        Some(item)
    }
}
