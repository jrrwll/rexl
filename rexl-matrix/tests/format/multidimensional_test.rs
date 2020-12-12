use rexl_matrix::format::Multidimensional;
use rexl_matrix::{Matrix};

#[test]
pub fn test() {
    let mut m: Multidimensional<f64> = Multidimensional::new((3, 4));
    m[(0, 0)] = 1.0;
    m[(2, 3)] = 1.0;
    println!("{}\n", m.to_string());

    m.resize(3);
    println!("{}\n", m.to_string());

    unsafe { m.erase(); }
    println!("{}\n", m.to_string());

    let m: Multidimensional<f64> = Multidimensional::zero((4, 3));
    println!("{}\n", m.to_string());
}
