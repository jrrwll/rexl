#[allow(dead_code)]
const SQRT_TWO: f64 = 1.414;

#[test]
#[allow(unused_variables)]
fn data_type() {
    //u8, i16, i32, u64
    // isize, usize: size is decided by arch
    let num = 98_222;
    let num: u16 = 0xff;
    let num = 0o77i64;
    let num = 0b1111_0000u8;
    let num = b'A';

    let num = 3.;
    let num: f32 = 3.14E-2;
    let num = 1.25f32;

    let v = true == false;

    let z = 'ℤ';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // destructure a tuple
    let (x, y, z) = tup;

    // array, stack type
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let january = months[0];
}

#[test]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn function() {
    // return from a sentence
    let mut y = {
        let x = 3;
        x + 1
    };
    if y == 4 {
        y += 1;
    }

    fn plus_one(x: i32) -> i32 {
        // return from a fn
        x + 1
    }
    let x = plus_one(5);
}

#[test]
#[allow(unused_variables)]
fn sentence() {
    let num = 1;

    let sign = if num > 0 {
        1
    } else if num < 0 {
        -1
    } else {
        0
    };

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    while num == 1 {
        loop {
            let a = [10, 20, 30, 40, 50];
            for element in a.iter() {
                println!("the value is: {}", element);
                continue;
            }
            break;
        }
        break;
    }
}

#[test]
#[allow(unused_variables)]
fn test() {
    let s = String::from("hello world");
    let world = &s[..5];
    let world = &s[6..];
    let word = first_word(&s);
    println!("{}", word)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
