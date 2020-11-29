use std::slice;

#[test]
fn test() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("after mut");
        *r2 += 1;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    unsafe {
        COUNTER += 3;
    }
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

#[allow(dead_code)]
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

//    (&mut slice[..mid],
//     &mut slice[mid..])
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

// call rust fn in other language
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

#[allow(dead_code)]
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

