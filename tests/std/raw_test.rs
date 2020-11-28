#[test]
fn ptr_wrapping_add() {
    let data = [1u8, 2, 3, 4, 5];
    let mut ptr: *const u8 = data.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_add(6);
    print!("{:?}, ", ptr);

    // This loop prints "1, 3, 5, "
    while ptr != end_rounded_up {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_add(step);
    }
    println!("{:?}", end_rounded_up);

    // zero size
    let data = [(), (), ()];
    let mut ptr = data.as_ptr() as *const u8;
    let end_rounded_up = ptr.wrapping_add(4);
    print!("{:?}, {:?}:\t", ptr, end_rounded_up);
    while ptr != end_rounded_up {
        unsafe {
            print!("{:?}, ", *ptr);
        }
        ptr = ptr.wrapping_add(1);
    }
}
