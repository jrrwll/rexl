/// Greatest Common Divisor, use Euclidean Algorithm
/// gcd(a, b) = gcd(b, a mod b)
pub fn gcd(a: i64, b: i64) -> i64 {
    if a > b {
        return gcd(b, a);
    }

    let (mut min, mut max) = (a, b);
    let mut r: i64;
    loop {
        r = max % min;
        if r == 0 { break; }
        max = min;
        min = r;
    }
    return min;
}

