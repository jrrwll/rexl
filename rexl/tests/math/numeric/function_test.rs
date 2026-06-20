use rexl::math::numeric::function::fibonacci;

#[test]
fn test_fibonacci() {
    for n in 1..20 {
        println!("fibonacci {:?} is:\t{:?}", n, fibonacci(n));
    }
}
