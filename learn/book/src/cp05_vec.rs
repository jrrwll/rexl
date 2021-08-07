use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[test]
#[allow(unused_variables)]
fn test() {
    let v: Vec<i32> = Vec::new();
    // mut type can auto-detected
    let mut v = Vec::new();
    v.push(1);
    println!("{:?}", v.pop());

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    //let does_not_exist = &v[100]; // panic!
    let does_not_exist = v.get(100); // None

    let mut v = vec![1, 4, 7];
    for i in &mut v {
        *i += 1;
    }
    println!("{:?}", v);

    //String
    let s = "initial contents";
    let s = String::from("initial contents");
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    for b in "foobar".bytes() {
        println!("{}", b);
    }
    for c in "foobar".chars() {
        println!("{}", c);
    }

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    // Some
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Red")).or_insert(100);

    let text = "hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // &mut V
        *count += 1;
    }
    println!("{:?}", map);
}

#[test]
fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[allow(unused_variables)]
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let expensive_closure = simulated_expensive_calculation;

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_millis(100));
    intensity
}
