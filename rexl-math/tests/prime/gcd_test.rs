extern crate rexl_math;

use rexl_math::prime::*;

#[test]
fn test_gcd() {
    assert_gcd(1, 1, 1);
    assert_gcd(1, 2, 1);
    assert_gcd(2, 3, 1);

    assert_gcd(10, 10, 10);
    assert_gcd(10, 5, 5);
    assert_gcd(10, 2, 2);

    assert_gcd(13, 7, 1);
    assert_gcd(111, 9, 3);
    assert_gcd(2 * 3 * 7, 2 * 7 * 11, 2 * 7);
}

fn assert_gcd(a: i64, b: i64, expected: i64) {
    let c = gcd(a, b);
    println!("gcd({}, {}) = {}\n", a, b, c);
    assert_eq!(c, expected);
}
