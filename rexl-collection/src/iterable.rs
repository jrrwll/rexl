pub trait Iterable<E>
where E: Sized + PartialEq + Clone {
    fn iter(&self) -> Box<dyn Iterator<Item = &E>>;
}
