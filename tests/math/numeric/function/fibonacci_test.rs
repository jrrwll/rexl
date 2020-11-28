extern crate rexl;

use rexl::math::numeric::function::*;

#[test]
fn test_fibonacci() {
    for n in 1..20 {
        println!("fibonacci {:?} is:\t{:?}", n, fibonacci(n));
    }
}
