use rexl_matrix::{Vector};
use std::f64::consts::PI;

#[test]
fn test_eq_xxx() {
    let v = Vector::eq_diff(0.0, 3.0, 10);
    println!("{:?}", v);
    println!("{}\n", v);

    println!("{}", Vector::eq_diff_at(1.0, 5, 1.5, 9));
    println!("{}", Vector::eq_prop(1.0, 2.0, 9));
    println!("{}\n", Vector::eq_prop_at(4.0, 4, 2.0, 9));

    println!("{}", Vector::line_sq(1, 100, 9));
    println!("{}\n", Vector::line_sq(1.0, 100.0, 9));

    println!("{}", Vector::line_sq_weight(
        1, 10, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0],
        #[inline] |it| it as i32));
    println!("{}", Vector::line_sq_weight(
        1, 22, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        #[inline] |it| it as i32));
    println!("{}", Vector::line_sq_weight(
        1, 43, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        #[inline] |it| it as i32));

    println!("{}", Vector::line_sq_weight(
        1.0, 10.0, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0],
        #[inline] |it| it));
    println!("{}", Vector::line_sq_weight(
        1.0, 22.0, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        #[inline] |it| it));
    println!("{}\n", Vector::line_sq_weight(
        1.0, 43.0, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        #[inline] |it| it));

    println!("{}", Vector::line_sq_weight(
        1, 80, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0],
        #[inline] |it| it as i32));
    println!("{}", Vector::line_sq_weight(
        1, 80, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        #[inline] |it| it as i32));
    println!("{}", Vector::line_sq_weight(
        1.0, 80.0, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0],
        #[inline] |it| it));
    println!("{}\n", Vector::line_sq_weight(
        1.0, 80.0, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        #[inline] |it| it));
}

#[test]
fn test_ops() {
    let mut v = Vector::from(vec![1.0; 10]);
    let u = Vector::eq_diff(1.0, 1.0, 10);
    println!("{}", v);
    println!("{}\n", u);
    println!("{}", &v + &u);
    println!("{}", &v - &u);
    println!("{}", &v * &u);
    println!("{}\n", &v / &u);

    v[0] = v[1] + v[2];
    v[1] = v[2] / v[3] + v[0] * v[3];
    println!("{}\n", v);

    let mut v: Vector<f64> = Vector::zero(5);
    v[3] = PI;
    println!("{}", v);
    let v = Vector::from_vec(v.into());
    println!("{}", v);
    let v = Vector::from_slice(vec![1.0; 10].as_slice());
    println!("{}", v);
}

#[test]
fn test_iter() {
    let mut v = Vector::eq_diff(1.0, 1.0, 10);
    println!("{}", v);

    for i in v.iter() {
        print!("{}, ", i)
    }
    println!("\n");

    for i in v.iter_mut() {
        let num = *i as f64;
        *i = num.powf(2.0) - 2.0 * num;
    }
    println!("{}", v);
}

#[test]
fn test_vector() {
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    let u = Vector::new(vec![-3.0, 1.0, 2.0]);
    println!("{}", v.dot_mul(&u));
    println!("{}", v.dot_mul(&u));
    assert_eq!(v.dot_mul(&u), 5.0);

    let norm: f64 = (1.0f64.powf(2.0) + 2.0f64.powf(2.0) + 3.0f64.powf(2.0)).powf(0.5);
    println!("sum={}, product={}, average={}, variance={}",
             v.sum(), v.product(), v.average(), v.variance());
    println!("norm={}, {}", v.norm(2.0), norm);
    assert_eq!(v.norm(2.0), norm)
}