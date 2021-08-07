/// cos(x) = x
pub fn cosxeqx(x: f64) -> f64 {
    let y = x.cos();
    if y == x {
        return y;
    }
    return cosxeqx(y);
}
