///
pub fn diffq(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let n = a.len();
    if n == 2 {
        return (b[0] - b[1]) / (a[0] - a[1]);
    } else if n == 1 {
        return b[0] / a[0];
    }
    panic!("uncompleted implement");
}
