#[cfg(test)]
mod build_test;

use rexl_matrix::format::Multidimensional;
use rexl_matrix::{Matrix, NumericMatrix, Variant, Vector};

#[test]
pub fn test() {
    let mut m: Multidimensional<f64> = Multidimensional::new((3, 4));
    m[(0, 0)] = 1.0;
    m[(2, 3)] = 1.0;
    println!("{}\n", m.to_string());

    m.resize(3);
    println!("{}\n", m.to_string());

    unsafe {
        m.erase();
    }
    println!("{}\n", m.to_string());

    let m: Multidimensional<f64> = Multidimensional::zero((4, 3));
    println!("{}\n", m.to_string());
}

#[test]
pub fn test_matrix() {
    let mut m: Multidimensional<f64> = Multidimensional::pascal((3, 4));
    println!("{}\n", m.to_string());
    assert_eq!(vec![1.0, 1.0, 1.0, 1.0], m.get_row_vec(0));
    assert_eq!(vec![1.0, 1.0, 1.0], m.get_column_vec(0));

    m.set_row(0, &Vector::from(vec![1.0, 2.0, 3.0, 4.0]));
    assert_eq!(vec![1.0, 2.0, 3.0, 4.0], m.get_row_vec(0));

    println!("{}\n", m.to_string());
    for i in m.iter(Variant::Column) {
        print!("{:?}, ", i);
    }

    for i in m.iter_mut(Variant::Column) {
        *i = *i * *i;
    }
    println!("\n{}\n", m.to_string());
}
