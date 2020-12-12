extern crate rexl;

use rexl::time::*;

#[test]
fn test() {
    let ts = Timeit::new()
        .add_unary_action(|| {1}, |_|{
            // println!("invoke 1");
            let _ = vec![0; 100_000]; // must be 0 if you wanna effective
        } )
        .add_action(|| {
            // println!("invoke 2");
            let _: Vec<i32> = Vec::with_capacity(100_000);
        })
        .add_action(|| {
            // println!("invoke 3");
            for _ in 0..100_000 {
            }
        })
        .add_unary_action(|| {1}, |_|{
            // println!("invoke 4");
            let _ = vec![1; 100_000];
        } )
        .add_unary_action(|| {1}, |_|{
            // println!("invoke 4");
            let _ = vec![0u64; 100_000];
        } )
        .repeat(10).count(20).skip(5)
        .run_and_format_us("\t");
    println!("{}", ts);
}