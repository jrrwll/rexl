use std::collections::HashMap;
use crate::cli::{Argument, ArgumentKind};

/// I will not give the implement about CLI usage since I believe less is more
#[derive(Debug)]
pub struct ArgParser {
    /// multi argparse-names to one key
    pub names_key_map: HashMap<String, String>,
    /// key, argument
    pub key_argument_map: HashMap<String, Argument>,
    /// key, value
    pub string_map: HashMap<String, String>,
    pub bool_map: HashMap<String, bool>,
    pub integer_map: HashMap<String, i64>,
    pub float_map: HashMap<String, f64>,
    /// multiple values
    pub strings_map: HashMap<String, Vec<String>>,
    pub integers_map: HashMap<String, Vec<i64>>,
    pub floats_map: HashMap<String, Vec<f64>>,
    pub properties_map: HashMap<String, HashMap<String, String>>,
    /// extra values
    pub extra_values: Vec<String>,
}

impl ArgParser {

    pub fn new() -> Self {
        Self {
            names_key_map: HashMap::new(),
            key_argument_map: HashMap::new(),

            string_map: HashMap::new(),
            bool_map: HashMap::new(),
            integer_map: HashMap::new(),
            float_map: HashMap::new(),

            strings_map: HashMap::new(),
            integers_map: HashMap::new(),
            floats_map: HashMap::new(),

            properties_map: HashMap::new(),
            extra_values: Vec::new(),
        }
    }

    #[inline]
    pub fn get_string(&self, key: &str) -> &String {
        &self.string_map[key]
    }

    #[inline]
    pub fn get_bool(&self, key: &str) -> bool {
        self.bool_map[key]
    }

    #[inline]
    pub fn get_integer(&self, key: &str) -> i64 {
        self.integer_map[key]
    }

    #[inline]
    pub fn get_float(&self, key: &str) -> f64 {
        self.float_map[key]
    }

    #[inline]
    pub fn get_strings(&self, key: &str) -> &Vec<String> {
        &self.strings_map[key]
    }

    #[inline]
    pub fn get_integers(&self, key: &str) -> &Vec<i64> {
        &self.integers_map[key]
    }

    #[inline]
    pub fn get_floats(&self, key: &str) -> &Vec<f64> {
        &self.floats_map[key]
    }

    #[inline]
    pub fn get_properties(&self, key: &str) -> &HashMap<String, String> {
        &self.properties_map[key]
    }

    #[inline]
    pub fn get_extra_values(&self) -> &Vec<String> {
        &self.extra_values
    }

    // -a 1.ts -a 2.ts
    #[inline]
    pub fn add(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::String, false)
    }

    #[inline]
    pub fn add_multiple(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::String, true)
    }

    #[inline]
    pub fn add_integer(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Integer, false)
    }

    #[inline]
    pub fn add_integer_multiple(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Integer, true)
    }

    #[inline]
    pub fn add_float(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Float, false)
    }

    #[inline]
    pub fn add_float_multiple(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Float, true)
    }

    /// aux, lah, cvzf, -ef
    /// only allow single char key and bool value
    #[inline]
    pub fn add_bool(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Bool, false)
    }

    /// only support single char argparse
    /// -Dfile.encoding=utf8 -Dspring.active.profile=dev
    #[inline]
    pub fn add_property(&mut self, key: &str, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Property, true)
    }

    /// -L 3, -L=3, -L3, --level 3, --level=3
    /// -o yaml, -o=yaml, -oyaml, --output yaml, --output=yaml
    /// -r, -r=true, -r true, --rm, --rm=true, --rm true
    pub fn add_by_kind(&mut self, key: &str, names: Vec<&str>,
                   kind: ArgumentKind, multiple: bool) -> &mut Self {
        let names = names.iter().map(|s| s.to_string()).collect();
        self.key_argument_map.insert(key.to_string(), Argument {
            names, kind, multiple
        });
        return self
    }
}