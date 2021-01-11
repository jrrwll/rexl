use std::cmp::{Ord, Ordering};
use std::mem;

#[test]
fn test_swap() {
    let mut v = [1, 2, 3, 4];
    println!("{:?}", &v);
    swap(&mut v, 1, 3);
    println!("{:?}", &v);
}

/// just copied from https://stackoverflow.com/questions/28294735/how-to-swap-elements-of-array
#[allow(dead_code)]
fn swap<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        // no swapping necessary
        Ordering::Equal => return,
        // get the smallest and largest of the two indices
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
    };

    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}
