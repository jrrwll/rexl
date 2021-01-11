extern crate rexl_math;

use rexl_math::numeric::function::*;

#[test]
fn test_fibonacci() {
    for n in 1..20 {
        println!("fibonacci {:?} is:\t{:?}", n, fibonacci(n));
    }
}
