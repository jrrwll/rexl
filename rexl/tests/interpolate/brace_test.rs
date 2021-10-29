use rexl::interpolate::{brace_named, brace_positional, brace_positional_unwrap};
use std::collections::HashMap;

#[test]
fn test_brace() {
    let mut context = HashMap::new();
    context.insert("Eric".to_string(), "Clapton".to_string());
    context.insert("John".to_string(), "Lennon".to_string());
    context.insert("Bob".to_string(), "Dylan".to_string());
    context.insert(":\\:".to_string(), "colon_backslash_colon".to_string());
    let template = "$$ :: {Eric} is a friend of {John}, but not a friend of {Bob}! ::$$";
    show_named(
        template,
        &context,
        "$$ :: Clapton is a friend of Lennon, but not a friend of Dylan! ::$$",
    );

    let arguments = vec!["Clapton".to_string(), "Lennon".to_string()];
    let template = "$$ :: {2} is a friend of {1}, but not a friend of {}! ::$$";
    show_positional(
        template,
        &arguments,
        "$$ :: NULL is a friend of Lennon, but not a friend of Clapton! ::$$",
    );

    let arguments = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ];
    let template = "{} {1} {0} {2} {} {4} {} \\{\\} {}";
    show_positional(template, &arguments, "A B A C B NULL C {} D");
}

#[test]
fn test_brace_named() {
    let mut context = HashMap::new();
    let template = " {a} {a_Y_0:val} {_b_Z} \\{{_val}\\} {_val} \\} \\} {} {} {0}";
    show_named(
        template,
        &context,
        " NULL val NULL {NULL} NULL } } NULL NULL NULL",
    );

    context.insert("a".to_string(), "a_val".to_string());
    context.insert("a_Y_0".to_string(), "a_Y_0_val".to_string());
    context.insert("_b_Z".to_string(), "_b_Z_val".to_string());
    context.insert(":".to_string(), "_val".to_string());
    show_named(
        template,
        &context,
        " a_val a_Y_0_val _b_Z_val {NULL} NULL } } NULL NULL NULL",
    );
}

#[test]
fn test_brace_positional_unwrap_utf8() {
    let s = brace_positional_unwrap("English, 简体中文, 🤣😆😁", &Vec::new(), None);
    println!("{}", s)
}

fn show_named(template: &str, context: &HashMap<String, String>, expect: &str) {
    println!("{}", template);
    match brace_named(template, &context, Some("NULL")) {
        Ok(formatted) => {
            println!("{}\n", formatted);
            assert_eq!(formatted, expect)
        }
        Err(err) => {
            eprintln!("{:?}\n", err);
        }
    }
}

fn show_positional(template: &str, arguments: &Vec<String>, expect: &str) {
    println!("{}", template);
    match brace_positional(template, arguments, Some("NULL")) {
        Ok(formatted) => {
            println!("{}\n", formatted);
            assert_eq!(formatted, expect)
        }
        Err(err) => {
            eprintln!("{:?}\n", err);
        }
    }
}
