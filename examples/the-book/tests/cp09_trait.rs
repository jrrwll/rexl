use std::fmt::{Debug, Display};
use std::ops::Add;
use std::fmt;

#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summarizable {
    fn author_summary(&self) -> String;

    // impl Summarizable for NewsArticle {}
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

#[allow(dead_code)]
struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn author_summary(&self) -> String {
        format!("@{}", self.chance_of_precipitation)
    }
}

#[allow(unused_variables)]
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    return 0;
}

// lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(0);
    }
}

trait IteratorT<T> {
    fn next(&mut self) -> Option<T>;
}

impl IteratorT<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        return Some(0);
    }
}


struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);

    fn some_static_method();
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }

    fn some_static_method() {}
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[test]
fn human() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // same as Human::fly(&person)
    <Human as Wizard>::some_static_method();
}

#[test]
fn test() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    some_function(1, 2);
    longest("a", "b");
}

//

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Dot {
    x: i32,
    y: i32,
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Dot {}

#[test]
fn show_dot() {
    let dot = Dot {x: 1, y: 2};
    dot.outline_print();
}