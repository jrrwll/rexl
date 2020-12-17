use std::ops::*;
use std::cmp;
use crate::{Element, Vector};

impl<T: Element> Index<usize> for Vector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Element> IndexMut<usize> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

macro_rules! implement (
    ($name:ident, $method:ident, $name_assign:ident, $method_assign:ident) => (

        impl<T> $name for &Vector<T>
            where T: Element + $name<Output=T> {
            type Output = Vector<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                let mut result = self.clone();
                result.$method_assign(rhs);
                result
            }
        }

        impl<T> $name_assign<&Vector<T>> for Vector<T>
            where T: Element +  $name<Output=T> {
            fn $method_assign(&mut self, rhs: &Vector<T>) {
                let size = cmp::min(self.len(), rhs.len());
                for i in 0..size {
                    self[i] = self[i].$method(rhs[i]);
                }
            }
        }
    );
);

implement!(Add, add, AddAssign, add_assign);
implement!(Sub, sub, SubAssign, sub_assign);
implement!(Mul, mul, MulAssign, mul_assign);
implement!(Div, div, DivAssign, div_assign);
