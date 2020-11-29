/*
cargo test
cargo test -- --test-threads=1

// not capture the output in passed test, just print it
cargo test -- --nocapture
// only test ignored fn
cargo test -- --ignored

// src/tests is default tagged `#[cfg(test)]`
cargo test --test <filename>

*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        // T: PartialEq + Debug
        // #[derive(PartialEq, Debug)]
        assert_eq!(2 + 2, 4);
        assert_ne!(0, 1);
    }

    #[test]
    #[allow(unconditional_panic)]
    #[allow(unused_variables)]
    #[should_panic(expected = "index out of bounds")]
    fn it_panics() {
        let v = [1, 2];
        let v = v[3];
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


