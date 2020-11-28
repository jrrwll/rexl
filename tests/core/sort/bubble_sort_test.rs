extern crate rexl;

use rexl::*;

#[test]
fn test_bubble_sort() {
    let mut arr = [0, 3, 6, 2, 6, 13, 9];
    println!("bubble sort on:\t{:?}", &arr);
    core::sort::bubble_sort(&mut arr);
    println!("result:\t{:?}", &arr);
}