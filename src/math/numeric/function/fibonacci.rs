/// 1, 1, 2, 3, 5, 8, 13, 21
pub fn fibonacci(n: i64) -> i64 {
    if n < 1 {
        panic!("not implement error");
    } else if n == 1 {
        return 1;
    }

    let (mut fib_prev, mut fib) = (1, 1);
    for _ in 2..n {
        let next = fib + fib_prev;
        fib_prev = fib;
        fib = next;
    }
    return fib;
}