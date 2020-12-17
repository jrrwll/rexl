#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Column,
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
