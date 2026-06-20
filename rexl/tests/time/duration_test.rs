use rexl::time::HumanDuration;

#[test]
fn test_parse() {
    let duration = HumanDuration::parse("1y2mo3w4d5h6m7s8ms9us10ns").unwrap();
    println!("duration {}", duration);

    let zero = vec![
        "0y", "0mo", "0w",
        "0d", "0h", "0m", "0s",
        "0ms", "0us", "0ns"
    ];
    for s in zero {
        _test_parse(s);
    }
}

fn _test_parse(s: &str) {
    match HumanDuration::try_from(s.to_string()) {
        Ok(v) => {
            println!("{} = {}", s, v)
        }
        Err(e) => {
            println!("failed to parse duration: {}", e)
        },
    }
}

#[test]
fn test_chrono() {
    let d = chrono::Duration::hours(2) + chrono::Duration::minutes(40) +
        chrono::Duration::seconds(93) + chrono::Duration::nanoseconds(123);

    println!("{}", d);
}
