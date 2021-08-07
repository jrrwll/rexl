use crate::Iterable;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub trait Collection<E>: Iterable<E>
where E: Sized + PartialEq + Clone {
    fn size(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn contains(&self, e: &E) -> bool {
        for i in self.iter() {
            if i == e {
                return true
            }
        }
        false
    }

    fn add(&mut self, e: E) -> bool {
        false
    }

    fn remove(&mut self, e: &E) -> bool {
        false
    }

    fn contains_all<C>(&self, c: C) -> bool
    where C: Collection<E> {
        for e in c.iter() {
            if !self.contains(e) {
                return false
            }
        }
        true
    }

    fn add_all<C>(&mut self, c: C) -> bool
    where C: Collection<E> {
        let mut modified = false;
        for e in c.iter() {
            if (self.add(e.clone())) {
                modified = true;
            }
        }
        modified
    }

    fn remove_all<C>(&mut self, c: C) -> bool
    where C: Collection<E> {
        let mut modified = false;
        for e in c.iter() {
            if (self.remove(e)) {
                modified = true;
            }
        }
        modified
    }

    fn remove_if<F>(&mut self, f: F) -> bool
    where F: FnOnce(&E) -> bool;

    fn retain_all<C>(&mut self, c: C) -> bool
    where C: Collection<E> {
        let mut modified = false;
        for e in c.iter() {
            if (!self.contains(e)) {
                self.remove(e);
                modified = true;
            }
        }
        modified
    }

    fn clear(&mut self);
}
