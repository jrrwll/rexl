mod singly_linked_list_impl;
mod singly_linked_list_impl_private;

use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct SinglyLinkedList<T> {
    pub (crate) head:   Option<NonNull<Node<T>>>,
    pub (crate) tail:   Option<NonNull<Node<T>>>,
    pub (crate) len:    usize,
    pub (crate) marker: PhantomData<Box<Node<T>>>,
}

pub struct Node<T> {
    pub (crate) next:    Option<NonNull<Node<T>>>,
    pub (crate) element: T,
}

impl<T> Node<T> {
    pub(crate) fn new(element: T) -> Self {
        Node { next: None, element }
    }

    pub(crate) fn into_element(self: Box<Self>) -> T {
        self.element
    }
}