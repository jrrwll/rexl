use chrono::NaiveDateTime;
use rexl::time::parse_datetime;

#[test]
fn test_parse() {
    test_parse_assert("2000-01-02 03:10:59", "2000-01-02 03:10:59");
    test_parse_assert("2000-01-02 03:10", "2000-01-02 03:10:00");
    test_parse_assert("2000-01-02", "2000-01-02 00:00:00");

    test_parse_assert("2000/01/02 03:10:59", "2000-01-02 03:10:59");
    test_parse_assert("2000/01/02 03:10", "2000-01-02 03:10:00");
    test_parse_assert("2000/01/02", "2000-01-02 00:00:00");
}

fn test_parse_assert(parse_str: &str, expect_str: &str) {
    let got = parse_datetime(parse_str);
    if let Some(v) = got {
        println!("{} => {}", parse_str, v)
    } else {
        println!("{} => None", parse_str);
    }

    if expect_str.len() ==0 {
        assert_eq!(got, None);
        return;
    }

    let expect = NaiveDateTime::parse_from_str(
        expect_str, "%Y-%m-%d %H:%M:%S").unwrap();

    assert_eq!(got, Some(expect));
}
