use rexl::interpolate::dollar_named_unwrap;
use std::collections::HashMap;

pub const TEMPLATE: &'static str = include_str!("./template.html");

pub fn format_html(title: &str, path: &str, lis: &str) -> String {
    let mut named: HashMap<String, String> = HashMap::new();
    named.insert("title".to_owned(), title.to_owned());
    named.insert("path".to_owned(), path.to_owned());
    named.insert("lis".to_owned(), lis.to_owned());
    return dollar_named_unwrap(TEMPLATE, &named, None)
}

pub fn format_li(name: &str, path: &str, clazz: &str) -> String {
    format!(
        "<li><a href=\"{}\" title=\"{}\" class=\"{}\">{}</a></li>",
        path, name, clazz, name
    )
}
