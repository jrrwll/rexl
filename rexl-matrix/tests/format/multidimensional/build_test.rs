use rexl::time::Timeit;
use rexl_matrix::format::Multidimensional;
use rexl_matrix::NumericMatrix;

#[test]
pub fn test_build() {
    let m: Multidimensional<f64> = Multidimensional::pascal(4);
    println!("{}\n", m.to_string());
    let m: Multidimensional<f64> = Multidimensional::pascal((4, 5));
    println!("{}\n", m.to_string());

    let m: Multidimensional<f64> = Multidimensional::vander(4, &vec![1.0, 2.0, 3.0]);
    println!("{}\n", m.to_string());

    let m: Multidimensional<f64> = Multidimensional::hilb(5);
    println!("{}\n", m.to_string());

    let m: Multidimensional<f64> =
        Multidimensional::hankel(&vec![1.0, 2.0, 3.0], &vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    println!("{}\n", m.to_string());
}

#[test]
pub fn test_det() {
    let mut m: Multidimensional<f64> = Multidimensional::pascal(6);
    println!("{}", m.to_string());
    println!("{}", m.det());
    println!("{}\n", m.to_string());

    let n = 36;
    for i in 1..n + 1 {
        let mut m: Multidimensional<f64> = Multidimensional::pascal(i);
        println!("%{:3}-level det {:?}", i, m.det());
    }
}

#[test]
pub fn test_det_speed() {
    let n = 36;
    for i in 1..n + 1 {
        let dimension = i as usize;
        let ts = Timeit::new()
            .add_action(move || {
                let mut m: Multidimensional<f64> = Multidimensional::pascal(dimension);
                m.det();
            })
            .repeat(3)
            .count(10)
            .skip(2)
            .run_and_format_us("\t");

        let mut m: Multidimensional<f64> = Multidimensional::pascal(dimension);
        println!("%{:3}-level det cost {}\t\t\t{:?}", dimension, ts, m.det());
    }
}
