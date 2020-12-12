use std::ops::*;
use crate::{Element, NumericElement};
use crate::format::Multidimensional;
use std::cmp;

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

macro_rules! implement (
    ($name:ident, $method:ident, $name_assign:ident, $method_assign:ident) => (
        impl<T> $name for &Multidimensional<T>
            where T: NumericElement + $name<Output=T> {
            type Output = Multidimensional<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                let mut result = self.clone();
                result.$method_assign(rhs);
                result
            }
        }

        impl<T> $name_assign<&Multidimensional<T>> for Multidimensional<T>
            where T: NumericElement + $name<Output=T> {
            fn $method_assign(&mut self, rhs: &Multidimensional<T>) {
                let rows = cmp::min(self.rows, rhs.rows);
                let columns = cmp::min(self.columns, rhs.columns);
                for i in 0..rows {
                    for j in 0..columns {
                        self[(i, j)] = self[(i, j)].$method(rhs[(i, j)]);
                    }
                }
            }
        }
    );
);

implement!(Add, add, AddAssign, add_assign);
implement!(Sub, sub, SubAssign, sub_assign);
implement!(Mul, mul, MulAssign, mul_assign);
implement!(Div, div, DivAssign, div_assign);
