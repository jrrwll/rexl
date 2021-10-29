#[cfg(test)]
mod code;

use std::mem::size_of;

#[test]
fn per_char() {
    println!("{}", size_of::<char>());
    let s = "123abcABC */?";
    for c in s.chars() {
        println!("{}", c)
    }
    println!();
    for b in s.bytes() {
        println!("{}", b);
    }
}

#[test]
fn byte_shl() {
    let n = 0b1010_0101_1010_0101;
    let high = n >> 8;
    let low = n & 0xff;
    println!("{}, {}, {}", n, high, low);
    println!("{}, {}, {}", (high << 8) | low, high << 0, low >> 0);
    println!("{}, {}", 0 << 8, n & ((1 << 0) - 1));
}
