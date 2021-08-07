#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[test]
fn test() {
    let black = Color(0, 0, 0);
    println!("{:?}", black);

    let width = 30;
    let sq = Rectangle::square(width);
    println!("{:?}", sq);

    let rect1 = Rectangle { width, height: 50 };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let mut rect3 = Rectangle { width: 60, ..rect1 };
    rect3.height = 45;

    // Debug, simple {:?}, or pretty {:#?}
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);
    // This loop prints: 0 1 2
    for x in &array {
        print!("{} ", x);
    }
    println!();
    for x in array.iter() {
        print!("{} ", x);
    }
}
