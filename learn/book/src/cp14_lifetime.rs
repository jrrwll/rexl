#[allow(dead_code)]
struct Context<'s>(&'s str);

// 's: 'c, s > =c
#[allow(dead_code)]
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

#[allow(dead_code)]
impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

#[allow(dead_code)]
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

#[allow(dead_code)]
struct Ref<'a, T: 'a>(&'a T);

#[allow(dead_code)]
fn bar() -> ! {
    // never type, diverging functions
    loop {}
}

// fn pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn test() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
