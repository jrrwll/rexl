use std::collections::HashMap;
use rexl::text::{dollar_interpolate, InterpolationError};

#[test]
fn test() {
    let mut context = HashMap::new();
    // $$ $ $a $a_Y_0 $_b_Z ${10:01} ${10:01:02} ${:::} ${{\:\}\\:{\:\}\\}
    let template = "$$ $ $a $a_Y_0 $_b_Z ${10:01} ${10:01:02} ${:::} ${{\\:\\}\\\\:{\\:\\}\\\\}";
    show_dollar_interpolate(template, &context, "$$ $ NULL NULL NULL 01 01:02 :: {:}\\");

    context.insert("a".to_string(), "a_val".to_string());
    context.insert("a_Y_0".to_string(), "a_Y_0_val".to_string());
    context.insert("_b_Z".to_string(), "_b_Z_val".to_string());
    context.insert(":".to_string(), "_val".to_string());
    // $$ $ a_val a_Y_0_val _b_Z_val 01 01:02 :: {:}\
    show_dollar_interpolate(
        template, &context, "$$ $ a_val a_Y_0_val _b_Z_val 01 01:02 :: {:}\\");

    // ${{\:\}\\:{\:\}\\}
    let template = "${{\\:\\}\\\\:{\\:\\}\\\\}";
    context.clear();
    context.insert("{:}\\".to_string(), "xxx".to_string());
    show_dollar_interpolate(template, &context, "xxx");
}

fn show_dollar_interpolate(template: &str, context: &HashMap<String, String>, expect: &str) {
    println!("{}", template);
    match dollar_interpolate(template, &context, Some("NULL")) {
        Ok(formatted) => {
            println!("{}\n", formatted);
            assert_eq!(formatted, expect)
        }
        Err(err) => {
            eprintln!("{:?}\n", err);
        }
    }
}