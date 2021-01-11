///
pub fn ploy(v: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    let n = v.len();
    for k in 0..n {
        sum += v[k].powi((n - k) as i32);
    }
    sum
}
