use std::collections::HashSet;
use std::hash::Hash;

pub fn vec_diff<T: Hash + Eq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let b_set: HashSet<_> = b.into_iter().collect();
    a.into_iter().filter(|item| !b_set.contains(item)).collect()
}

pub fn vec_diff_ref<'a, T: Hash + Eq>(a: &'a [T], b: &[T]) -> Vec<&'a T> {
    let b_set: HashSet<_> = b.iter().collect();
    a.iter().filter(|item| !b_set.contains(item)).collect()
}
