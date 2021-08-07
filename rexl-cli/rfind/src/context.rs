use rexl::interpolate::brace_positional_unwrap;
use rexl::text::load_properties_from_str;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub usage: &'static str,
    message: HashMap<String, String>,
}

impl Context {
    pub fn new(usage: &'static str, message: &'static str) -> Self {
        Self {
            usage,
            message: load_properties_from_str(message),
        }
    }

    pub fn format(&self, key: &'static str, args: Vec<String>) -> String {
        brace_positional_unwrap(&self.message[key], &args, None)
    }
}
