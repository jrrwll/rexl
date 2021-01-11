use super::*;

/// Least Common Multiple
/// a * b = gcd(a, b) * lcm(a, b)
pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
