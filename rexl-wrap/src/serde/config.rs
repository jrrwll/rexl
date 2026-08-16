use config::{Map, Value, ValueKind};
use serde::{Deserialize, Deserializer};

#[derive(Default)]
pub struct UnderscoreFlattenKey<T: Default>(pub T);

impl<'de, T: Deserialize<'de> + Default> Deserialize<'de> for UnderscoreFlattenKey<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let converted = flatten_value_keys(value, '_', 1);
        let inner = T::deserialize(converted).map_err(serde::de::Error::custom)?;
        Ok(Self(inner))
    }
}

/// flatten_value_keys(Value(...), '_', 1)
/// treat a.b.c.d as a.b_c_d
pub fn flatten_value_keys(config_value: Value, sep: char, keep_depth: u32) -> Value {
    let ValueKind::Table(m) = config_value.kind else {
        return config_value;
    };

    if keep_depth > 0 {
        let nest_keep_depth = keep_depth - 1;
        let mut new_map = Map::new();
        for (k, v) in m.to_owned() {
            let new_value = flatten_value_keys(v, sep, nest_keep_depth);
            new_map.insert(k, new_value);
        }
        Value::new(None, new_map)
    } else {
        let mut dest = Map::new();
        flatten_value_map_recursive(&mut dest, m.to_owned(), String::new(), sep);
        Value::new(None, dest)
    }
}

fn flatten_value_map_recursive(
    dest: &mut Map<String, Value>, src: Map<String, Value>, prefix: String, sep: char,
) {
    for (key, value) in src {
        let new_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{}{}{}", prefix, sep, key)
        };

        match value.kind {
            ValueKind::Table(obj) => {
                flatten_value_map_recursive(dest, obj, new_key, sep);
            }
            _ => {
                dest.insert(new_key, value);
            }
        }
    }
}
