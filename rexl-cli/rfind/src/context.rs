use crate::i18n::load_i18n_config;
use rexl::io::load_properties_from_str;
use rexl::text::brace_positional_unwrap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub usage: &'static str,
    message: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        let (usage, message) = unsafe { load_i18n_config() };
        Self { usage, message: load_properties_from_str(message) }
    }

    pub fn format(&self, key: &str, args: Vec<String>) -> String {
        brace_positional_unwrap(&self.message[key], &args, None)
    }
}
