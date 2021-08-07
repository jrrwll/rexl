pub use self::compressed::*;
pub use self::conventional::*;
pub use self::multidimensional::*;

mod compressed;
mod conventional;
mod multidimensional;

use crate::*;
use std::cmp;
use std::ops::*;

macro_rules! implement_size {
    ($name:ident) => {
        impl<T: Element> Size for $name<T> {
            #[inline]
            fn rows(&self) -> usize {
                self.rows
            }

            #[inline]
            fn columns(&self) -> usize {
                self.columns
            }
        }
    };
}

macro_rules! implement_numeric_matrix {
    ($name:ident) => {
        impl<T> NumericMatrix for $name<T>
        where
            T: NumericElement
                + Add<Output = T>
                + Sub<Output = T>
                + Mul<Output = T>
                + Div<Output = T>
                + Neg<Output = T>,
        {
            type Element = T;
        }
    };
}

macro_rules! implement_ops (
    ($format:ident, $name:ident, $method:ident, $name_assign:ident, $method_assign:ident) => (
        impl<T> $name for &$format<T>
            where T: NumericElement + $name<Output=T> {
            type Output = $format<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                let mut result = self.clone();
                result.$method_assign(rhs);
                result
            }
        }

        impl<T> $name_assign<&$format<T>> for $format<T>
            where T: NumericElement + $name<Output=T> {
            fn $method_assign(&mut self, rhs: &$format<T>) {
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

implement_size!(Multidimensional);
implement_size!(Conventional);

implement_numeric_matrix!(Multidimensional);
implement_numeric_matrix!(Conventional);

implement_ops!(Multidimensional, Add, add, AddAssign, add_assign);
implement_ops!(Multidimensional, Sub, sub, SubAssign, sub_assign);
implement_ops!(Multidimensional, Mul, mul, MulAssign, mul_assign);
implement_ops!(Multidimensional, Div, div, DivAssign, div_assign);
implement_ops!(Conventional, Add, add, AddAssign, add_assign);
implement_ops!(Conventional, Sub, sub, SubAssign, sub_assign);
implement_ops!(Conventional, Mul, mul, MulAssign, mul_assign);
implement_ops!(Conventional, Div, div, DivAssign, div_assign);
