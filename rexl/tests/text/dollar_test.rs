use rexl::text::{dollar_named, dollar_positional};
use std::collections::HashMap;

#[test]
fn test_dollar() {
    let mut context = HashMap::new();
    context.insert("Eric".to_string(), "Clapton".to_string());
    context.insert("John".to_string(), "Lennon".to_string());
    context.insert("Bob".to_string(), "Dylan".to_string());
    context.insert(":\\:".to_string(), "colon_backslash_colon".to_string());
    let template = "$$ :: $Eric is a friend of $John, but not a friend of $Bob! ::$$";
    show_named(
        template,
        &context,
        "$$ :: Clapton is a friend of Lennon, but not a friend of Dylan! ::$$",
    );

    let arguments = vec!["Clapton".to_string(), "Lennon".to_string()];
    let template = "$$ :: $2 is a friend of $1, but not a friend of $3! ::$$";
    show_positional(
        template,
        &arguments,
        "$$ :: Lennon is a friend of Clapton, but not a friend of NULL! ::$$",
    );
}

#[test]
fn test_dollar_named() {
    let mut context = HashMap::new();
    // $$ $ $a $a_Y_0 $_b_Z ${10:01} ${10:01:02} ${:::} ${{\:\}\\:{\:\}\\}
    let template = "$$ $ $a $a_Y_0 $_b_Z ${10:01} ${10:01:02} ${:::} ${{\\:\\}\\\\:{\\:\\}\\\\}";
    show_named(template, &context, "$$ $ NULL NULL NULL 01 01:02 :: {:}\\");

    context.insert("a".to_string(), "a_val".to_string());
    context.insert("a_Y_0".to_string(), "a_Y_0_val".to_string());
    context.insert("_b_Z".to_string(), "_b_Z_val".to_string());
    context.insert(":".to_string(), "_val".to_string());
    // $$ $ a_val a_Y_0_val _b_Z_val 01 01:02 :: {:}\
    show_named(
        template,
        &context,
        "$$ $ a_val a_Y_0_val _b_Z_val 01 01:02 :: {:}\\",
    );

    // ${{\:\}\\:{\:\}\\}
    let template = "${{\\:\\}\\\\:{\\:\\}\\\\}";
    context.clear();
    context.insert("{:}\\".to_string(), "xxx".to_string());
    show_named(template, &context, "xxx");
}

#[test]
fn test_dollar_utf8() {
    let mut context = HashMap::new();
    context.insert("English".to_string(), "".to_string());

    let template = "English, 简体中文, 🤣😆😁";
    let expect = "English, 简体中文, 🤣😆😁";
    show_named(template, &context, expect);

    let template = "$English, ${简体:中文}, ${\\🤣:😆:\\\\😁}";
    let expect = ", 中文, 😆:\\😁";
    show_named(template, &context, expect);

    let template = "English, 简体中文, 🤣😆😁 $English, ${简体:中文}, ${\\🤣:😆:\\\\😁}";
    let expect = "English, 简体中文, 🤣😆😁 , 中文, 😆:\\😁";
    show_named(template, &context, expect);
}

fn show_named(template: &str, context: &HashMap<String, String>, expect: &str) {
    println!("{}", template);
    match dollar_named(template, context, Some("NULL")) {
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
    match dollar_positional(template, arguments, Some("NULL")) {
        Ok(formatted) => {
            println!("{}\n", formatted);
            assert_eq!(formatted, expect)
        }
        Err(err) => {
            eprintln!("{:?}\n", err);
        }
    }
}
