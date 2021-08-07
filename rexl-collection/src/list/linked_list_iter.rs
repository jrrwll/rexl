use crate::linked_list::*;
use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Clone)]
pub struct Iter<'a, T: 'a> {
    pub(crate) head:   Option<NonNull<Node<T>>>,
    pub(crate) tail:   Option<NonNull<Node<T>>>,
    pub(crate) len:    usize,
    pub(crate) marker: PhantomData<&'a Node<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.len).finish()
    }
}

pub struct IterMut<'a, T: 'a> {
    // We do *not* exclusively own the entire list here, references to node's `element`
    // have been handed out by the iterator! So be careful when using this; the methods
    // called must be aware that there can be aliasing pointers to `element`.
    pub(crate) list: &'a mut LinkedList<T>,
    pub(crate) head: Option<NonNull<Node<T>>>,
    pub(crate) tail: Option<NonNull<Node<T>>>,
    pub(crate) len:  usize,
}

impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut").field(&self.list).field(&self.len).finish()
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.list).finish()
    }
}
