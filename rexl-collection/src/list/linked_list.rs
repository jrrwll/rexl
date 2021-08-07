pub use super::linked_list_iter::*;
pub use super::linked_list_impl_private::*;

use std::collections::LinkedList as _linked_list;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    pub (crate) head:   Option<NonNull<Node<T>>>,
    pub (crate) tail:   Option<NonNull<Node<T>>>,
    pub (crate) len:    usize,
    pub (crate) marker: PhantomData<Box<Node<T>>>,
}

pub struct Node<T> {
    pub (crate) next:    Option<NonNull<Node<T>>>,
    pub (crate) prev:    Option<NonNull<Node<T>>>,
    pub (crate) element: T,
}

impl<T> Node<T> {
    pub(crate) fn new(element: T) -> Self {
        Node { next: None, prev: None, element }
    }

    pub(crate) fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> Default for LinkedList<T> {
    /// Creates an empty `LinkedList<T>`.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

