use crate::transform::ElementaryTransformation;
use crate::{Element, Matrix, NumericElement, Size};
use std::ops::*;

pub trait NumericMatrix:
    Matrix<Element = <Self as NumericMatrix>::Element>
    + ElementaryTransformation<<Self as NumericMatrix>::Element> {
    /// The element type.
    type Element: NumericElement
        + Add<Output = <Self as NumericMatrix>::Element>
        + Sub<Output = <Self as NumericMatrix>::Element>
        + Mul<Output = <Self as NumericMatrix>::Element>
        + Div<Output = <Self as NumericMatrix>::Element>
        + Neg<Output = <Self as NumericMatrix>::Element>;

    /// build

    fn pascal<S: Size>(size: S) -> Self {
        let mut this = Self::zero(size);
        let (rows, columns) = this.dimensions();

        for i in 0..rows {
            for j in 0..columns {
                this[(i, j)] = if i == 0 || j == 0 {
                    <Self as NumericMatrix>::Element::one()
                } else {
                    this[(i - 1, j)] + this[(i, j - 1)]
                };
            }
        }
        this
    }

    fn vander(rows: usize, vector: &Vec<<Self as NumericMatrix>::Element>) -> Self {
        let columns = vector.len();
        let mut this = Self::zero((rows, columns));

        for i in 0..rows {
            for j in 0..columns {
                if i == 0 {
                    this[(i, j)] = <Self as NumericMatrix>::Element::one();
                } else {
                    this[(i, j)] = this[(i - 1, j)] * vector[j];
                }
            }
        }
        this
    }

    fn hilb<S: Size>(size: S) -> Self {
        let mut this = Self::zero(size);
        let (rows, columns) = this.dimensions();

        for i in 0..rows {
            for j in 0..columns {
                this[(i, j)] = <Self as NumericMatrix>::Element::one()
                    / <Self as NumericMatrix>::Element::from_usize(i + j + 1);
            }
        }
        this
    }

    fn hankel(
        a: &Vec<<Self as NumericMatrix>::Element>, b: &Vec<<Self as NumericMatrix>::Element>,
    ) -> Self {
        let (rows, columns) = (a.len(), b.len());
        let mut this = Self::zero((rows, columns));

        for i in 0..rows {
            for j in 0..columns {
                if i + j < rows {
                    this[(i, j)] = a[i + j];
                } else {
                    this[(i, j)] = b[i + j - rows + 1];
                }
            }
        }
        this
    }

    /// elementary transformation

    ///
    /// a00  a01  a02
    /// a10  a11  a12
    /// a20  a21  a22
    ///
    /// a00                  a01                 a02
    /// a10-a00*(a10/a00)    a11-a01*(a10/a00)   a12-a02(a10/a00)
    /// a20-a00*(a20/a00)    a21-a01*(a20/a00)   a21-a01*(a20/a00)
    fn diagonalize_triu(&mut self) -> Option<bool> {
        let dimension = self.min_dimension();
        let mut sign_reversed = false;
        for k in 0..dimension - 1 {
            let kk = self[(k, k)];
            if kk.is_zero() {
                let mut swapped = false;
                for i in (k + 1)..dimension {
                    if !self[(i, k)].is_zero() {
                        self.swap_row(i, k);
                        swapped = true;
                        // reverse sign
                        sign_reversed = !sign_reversed;
                        break
                    }
                }
                // if not swapped
                if !swapped {
                    return None
                }
            }

            // kk will not equal zero in this area
            for i in (k + 1)..dimension {
                let ik = self[(i, k)];
                self.add_other_row(i, k, -ik / kk);
            }
        }
        Some(sign_reversed)
    }

    fn det(&mut self) -> <Self as NumericMatrix>::Element {
        let sign_reversed = match self.diagonalize_triu() {
            None => return <Self as NumericMatrix>::Element::zero(),
            Some(sign_reversed) => sign_reversed,
        };

        let dimension = self.min_dimension();
        let mut result = <Self as NumericMatrix>::Element::one();
        for k in 0..dimension {
            result = result * self[(k, k)];
        }
        if sign_reversed {
            -result
        } else {
            result
        }
    }
}
