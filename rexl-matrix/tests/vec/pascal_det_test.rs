extern crate rexl_matrix;

use std::time::Instant;
// or use crate::*;
use rexl_matrix::vec::*;

#[test]
fn test_pascal6_det() {
    let n = 6;

    let mut m = VecMatrix::pascal(n as usize);
    m.print_data();
    println!("{:?} pascal det is {:?}", n, m.det());
    m.print_data();
}

#[test]
fn test_pascal_det() {
    for n in 1..33 {
        let mut m = VecMatrix::pascal(n as usize);
        println!("{:?} pascal det is {:?}", n, m.det());
    }
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn test_pascal_det_speed() {
    let verbose = true;
    let n = 64;
    let count = 10;
    let skip = 2;

    for i in 1..n + 1 {
        let mut tss = speed(i, count, verbose);
        let ts = avg(&mut tss, skip);

        let mut m = VecMatrix::pascal(i as usize);
        println!("%{:3}-level det cost {:.3} us\t{:?}", i, ts, m.det());
    }
}

#[test]
#[ignore]
#[allow(unused_variables)]
fn test_pascal_det_speed_one() {
    let verbose = true;
    let n = 64;
    let count = 10;
    let skip = 2;

    for i in 1..n + 1 {
        let ts = speed_one(i) as f64 / 1000.;
        let mut m = VecMatrix::pascal(i as usize);
        println!("%{:3}-level det cost {:.3} us\t{:?}", i, ts, m.det());
    }
}

fn speed(dimension: usize, count: usize, verbose: bool) -> Vec<u128> {
    let mut tss = vec![0; count];
    for i in 0..count {
        let mut m = VecMatrix::pascal(dimension as usize);
        let now = Instant::now();
        m.det();
        tss[i] = now.elapsed().as_nanos();
    }
    if verbose {
        println!("{:?}", tss)
    }
    return tss;
}

fn speed_one(dimension: usize) -> u128 {
    let mut m = VecMatrix::pascal(dimension as usize);
    let now = Instant::now();
    m.det();
    return now.elapsed().as_nanos();
}

fn avg(tss: &mut Vec<u128>, skip: usize) -> f64 {
    tss.sort();
    let len = tss.len();
    let mut count = 0;
    let mut avg = 0.;
    for i in 0..len {
        // if skip = 10, then
        // skip 0, 1, ..., 9 or len-10, ..., len - 1
        if i < skip || i >= len - skip {
            continue;
        }
        count += 1;
        avg += tss[i] as f64 / 1000.;
    }
    return avg / count as f64;
}
