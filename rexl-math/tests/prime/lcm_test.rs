extern crate rexl_math;

use rexl_math::prime::*;

#[test]
fn test_lcm() {
    assert_lcm(1, 1, 1);
    assert_lcm(1, 2, 2);
    assert_lcm(2, 3, 6);

    assert_lcm(2 * 3 * 7, 2 * 7 * 11, 2 * 7 * 3 * 11);
    assert_lcm(2 * 3 * 7, 7 * 11, 2 * 7 * 3 * 11);
    assert_lcm(2 * 3 * 3 * 3 * 7, 3 * 3 * 7 * 7 * 11, 2 * 3 * 3 * 3 * 7 * 7 * 11);
}

fn assert_lcm(a: i64, b: i64, expected: i64) {
    let c = lcm(a, b);
    println!("lcm({}, {}) = {}\n", a, b, c);
    assert_eq!(c, expected);
}