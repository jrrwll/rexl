use std::str::CharIndices;

#[test]
fn test_str_utf8() {
    let s = "En中文🤣😆";
    let mut chars = s.char_indices();
    show_and_assert(&s[0..1], "E", &mut chars);
    show_and_assert(&s[1..2], "n", &mut chars);
    show_and_assert(&s[2..5], "中", &mut chars);
    show_and_assert(&s[5..8], "文", &mut chars);
    show_and_assert(&s[8..12], "🤣", &mut chars);
    show_and_assert(&s[12..16], "😆", &mut chars);
    // show_and_assert(&s[..], "", &mut chars);

    let s = true;
}

fn show_and_assert(s: &str, expect: &str, chars: &mut CharIndices) {
    assert_eq!(s, expect);
    let (i, c) = chars.next().unwrap();
    print!("{}, {}", c, i);
    println!();
    assert_eq!(c.to_string(), expect)
}
