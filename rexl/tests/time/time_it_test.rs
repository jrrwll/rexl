use rexl::time::*;

#[ignore]
#[test]
fn test_time_it() {
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
    // output like: 85.131us	7.026us	30633.678us	33903.761us	164.901us
    println!("{}", ts);
}