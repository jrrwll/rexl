use std::time::Duration;
use std::thread;

#[test]
#[allow(unused_variables)]
#[allow(dead_code)]
fn some_closure() {
    // trait Fn、FnMut or FnOnce
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: i32| { x + 1 };
    let add_one_v4 = |x: i128| x + 1;

    generate_workout(20, 2);
    generate_workout(20, 3);
    generate_workout(25, 2);
    generate_workout(25, 3);
    generate_workout(30, 2);
    generate_workout(30, 3);

}

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(50));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} sit-ups!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
