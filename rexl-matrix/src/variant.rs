/// A variant of a compressed matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    /// The compressed-column variant.
    Column,
    /// The compressed-row variant.
    Row,
}

impl Variant {
    /// Return the other variant.
    #[inline]
    pub fn flip(&self) -> Self {
        match *self {
            Variant::Column => Variant::Row,
            Variant::Row => Variant::Column,
        }
    }
}
