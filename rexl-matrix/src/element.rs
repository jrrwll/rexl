use std::fmt::Debug;

/// An element.
pub trait Element: Copy + PartialEq + Debug {
    /// Return the zero element.
    fn zero() -> Self;

    /// Return the one element.
    fn one() -> Self;

    /// Check if the element is zero.
    #[inline]
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

/// An numeric element
pub trait NumericElement: Element {
    /// Convert usize as a `Element`
    fn from_usize(n: usize) -> Self;
}

macro_rules! implement (
    ($name:ty, $zero:expr, $one:expr) => (
        impl Element for $name {
            #[inline]
            fn zero() -> Self {
                $zero
            }

            #[inline]
            fn one() -> Self {
                $one
            }
        }

        impl NumericElement for $name {
            #[inline]
            fn from_usize(n: usize) -> Self {
                n as $name
            }
        }
    );

    ($name:ty) => (
        implement!($name, 0, 1);
    );
);

implement!(u8);
implement!(u16);
implement!(u32);
implement!(u64);

implement!(i8);
implement!(i16);
implement!(i32);
implement!(i64);

implement!(f32, 0.0, 1.0);
implement!(f64, 0.0, 1.0);

implement!(isize);
implement!(usize);

impl Element for bool {
    #[inline]
    fn zero() -> Self {
        false
    }

    #[inline]
    fn one() -> Self {
        true
    }
}
