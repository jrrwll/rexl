use rexl::text::to_snake;

#[test]
fn test_to_snake() {
    let s = to_snake("XMLHttpRequest");
    println!("{}", s);
    assert_eq!(to_snake(&s), "xml_http_request");
}
