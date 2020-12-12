use crate::{Size, Element};
use std::ops::*;

/// A matrix.
pub trait Matrix:
IndexMut<(usize, usize), Output=<Self as Matrix>::Element> +
Size + Sized {
    /// The element type.
    type Element: Element;

    /// Count nonzero elements.
    fn nonzeros(&self) -> usize;

    /// Create a zero matrix.
    fn zero<S: Size>(size: S) -> Self;
}