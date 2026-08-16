use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

pub fn convert_directly<T: Serialize, U: DeserializeOwned>(
    input: &T,
) -> Result<U, Box<dyn std::error::Error>> {
    let bytes = serde_json::to_vec(input)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn merge_struct<T: Serialize + DeserializeOwned>(dest: &mut T, from: &T) {
    let dest_val = serde_json::to_value(&mut *dest).unwrap();
    let from_val = serde_json::to_value(from).unwrap();
    let merged = merge_json(dest_val, from_val);
    *dest = serde_json::from_value(merged).unwrap();
}

fn merge_json(dest: Value, from: Value) -> Value {
    match (dest, &from) {
        (Value::Object(mut dest_map), Value::Object(from_map)) => {
            for (k, v) in from_map {
                if v.is_string() && v.as_str().unwrap().is_empty() {
                    continue;
                }
                match dest_map.get_mut(k) {
                    Some(dest_v) if dest_v.is_object() && v.is_object() => {
                        *dest_v = merge_json(dest_v.take(), v.to_owned());
                    }
                    _ => {
                        dest_map.insert(k.to_owned(), v.to_owned());
                    }
                }
            }
            Value::Object(dest_map)
        }
        _ => from,
    }
}
