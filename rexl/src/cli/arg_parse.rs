use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::cli::{Argument, ArgumentKind};

/// I will not give the implement about CLI i18n since I believe less is more
#[derive(Debug)]
pub struct ArgParser<K: Hash + Eq + Debug + Clone> {
    /// multi argparse-names to one key
    pub(crate) names_key_map:    HashMap<String, K>,
    /// key, argument
    pub(crate) key_argument_map: HashMap<K, Argument<K>>,
    /// key, value
    pub(crate) string_map:       HashMap<K, String>,
    pub(crate) bool_map:         HashMap<K, bool>,
    pub(crate) integer_map:      HashMap<K, i64>,
    pub(crate) float_map:        HashMap<K, f64>,
    /// multiple values
    pub(crate) strings_map:      HashMap<K, Vec<String>>,
    pub(crate) integers_map:     HashMap<K, Vec<i64>>,
    pub(crate) floats_map:       HashMap<K, Vec<f64>>,
    pub(crate) properties_map:   HashMap<K, HashMap<String, String>>,
    /// extra values
    pub(crate) extra_values:     Vec<String>,
}

impl<K> ArgParser<K>
where K: Hash + Eq + Debug + Clone
{
    pub fn new() -> Self {
        Self {
            names_key_map:    HashMap::new(),
            key_argument_map: HashMap::new(),

            string_map:  HashMap::new(),
            bool_map:    HashMap::new(),
            integer_map: HashMap::new(),
            float_map:   HashMap::new(),

            strings_map:  HashMap::new(),
            integers_map: HashMap::new(),
            floats_map:   HashMap::new(),

            properties_map: HashMap::new(),
            extra_values:   Vec::new(),
        }
    }

    #[inline]
    pub fn get_bool<T>(&self, key: T) -> bool
    where T: Into<K> {
        if let Some(v) = self.bool_map.get(&key.into()) {
            *v
        } else {
            false
        }
    }

    #[inline]
    pub fn get_string<T>(&self, key: T) -> Option<&String>
    where T: Into<K> {
        self.string_map.get(&key.into())
    }

    #[inline]
    pub fn get_string_or_else<T, F>(&self, key: T, f: F) -> String
    where
        T: Into<K>,
        F: FnOnce() -> String, {
        if let Some(v) = self.get_string(key) {
            v.clone()
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_integer<T>(&self, key: T) -> Option<&i64>
    where T: Into<K> {
        self.integer_map.get(&key.into())
    }

    #[inline]
    pub fn get_integer_or_else<T, F>(&self, key: T, f: F) -> i64
    where
        T: Into<K>,
        F: FnOnce() -> i64, {
        if let Some(v) = self.get_integer(key) {
            *v
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_float<T>(&self, key: T) -> Option<&f64>
    where T: Into<K> {
        self.float_map.get(&key.into())
    }

    #[inline]
    pub fn get_float_or_else<T, F>(&self, key: T, f: F) -> f64
    where
        T: Into<K>,
        F: FnOnce() -> f64, {
        if let Some(v) = self.get_float(key) {
            *v
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_strings<T>(&self, key: T) -> Option<&Vec<String>>
    where T: Into<K> {
        self.strings_map.get(&key.into())
    }

    #[inline]
    pub fn get_strings_or_else<T, F>(&self, key: T, f: F) -> Vec<String>
    where
        T: Into<K>,
        F: FnOnce() -> Vec<String>, {
        if let Some(v) = self.get_strings(key) {
            v.clone()
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_strings_or_empty<T>(&self, key: T) -> Vec<String>
    where T: Into<K> {
        self.get_strings_or_else(
            key,
            #[inline]
            || Vec::new(),
        )
    }

    #[inline]
    pub fn get_integers<T>(&self, key: T) -> Option<&Vec<i64>>
    where T: Into<K> {
        self.integers_map.get(&key.into())
    }

    #[inline]
    pub fn get_integers_or_else<T, F>(&self, key: T, f: F) -> Vec<i64>
    where
        T: Into<K>,
        F: FnOnce() -> Vec<i64>, {
        if let Some(v) = self.get_integers(key) {
            v.clone()
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_integers_or_empty<T>(&self, key: T) -> Vec<i64>
    where T: Into<K> {
        self.get_integers_or_else(
            key,
            #[inline]
            || Vec::new(),
        )
    }

    #[inline]
    pub fn get_floats<T>(&self, key: T) -> Option<&Vec<f64>>
    where T: Into<K> {
        self.floats_map.get(&key.into())
    }

    #[inline]
    pub fn get_floats_or_else<T, F>(&self, key: T, f: F) -> Vec<f64>
    where
        T: Into<K>,
        F: FnOnce() -> Vec<f64>, {
        if let Some(v) = self.get_floats(key) {
            v.clone()
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_floats_or_empty<T>(&self, key: T) -> Vec<f64>
    where T: Into<K> {
        self.get_floats_or_else(
            key,
            #[inline]
            || Vec::new(),
        )
    }

    #[inline]
    pub fn get_properties<T: Into<K>>(&self, key: T) -> Option<&HashMap<String, String>> {
        self.properties_map.get(&key.into())
    }

    #[inline]
    pub fn get_properties_or_else<T, F>(&self, key: T, f: F) -> HashMap<String, String>
    where
        T: Into<K>,
        F: FnOnce() -> HashMap<String, String>, {
        if let Some(v) = self.get_properties(key) {
            v.clone()
        } else {
            f()
        }
    }

    #[inline]
    pub fn get_properties_or_empty<T>(&self, key: T) -> HashMap<String, String>
    where T: Into<K> {
        self.get_properties_or_else(
            key,
            #[inline]
            || HashMap::new(),
        )
    }

    #[inline]
    pub fn get_extra_values(&self) -> &Vec<String> {
        &self.extra_values
    }

    // -a 1.ts -a 2.ts
    #[inline]
    pub fn add<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::String, false)
    }

    #[inline]
    pub fn add_multiple<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::String, true)
    }

    #[inline]
    pub fn add_integer<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Integer, false)
    }

    #[inline]
    pub fn add_integer_multiple<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Integer, true)
    }

    #[inline]
    pub fn add_float<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Float, false)
    }

    #[inline]
    pub fn add_float_multiple<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Float, true)
    }

    /// aux, lah, cvzf, -ef
    /// only allow single char key and bool value
    #[inline]
    pub fn add_bool<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Bool, false)
    }

    /// only support single char argparse
    /// -Dfile.encoding=utf8 -Dspring.active.profile=dev
    #[inline]
    pub fn add_property<T: Into<K>>(&mut self, key: T, names: Vec<&str>) -> &mut Self {
        return self.add_by_kind(key, names, ArgumentKind::Property, true)
    }

    /// -L 3, -L=3, -L3, --level 3, --level=3
    /// -o yaml, -o=yaml, -oyaml, --output yaml, --output=yaml
    /// -r, -r=true, -r true, --rm, --rm=true, --rm true
    pub fn add_by_kind<T>(
        &mut self, key: T, names: Vec<&str>, kind: ArgumentKind, multiple: bool,
    ) -> &mut Self
    where T: Into<K> {
        let key = key.into();
        let names = names.iter().map(|s| s.to_string()).collect();
        self.key_argument_map.insert(key.clone(), Argument {
            key,
            names,
            kind,
            multiple,
        });
        return self
    }
}
