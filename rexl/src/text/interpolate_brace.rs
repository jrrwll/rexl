use std::collections::HashMap;
use crate::text::*;
use crate::text::interpolate::*;

pub fn brace(
    s: &str, named: &HashMap<String, String>, positional: &Vec<String>, default_value: Option<&str>,
) -> Result<String, InterpolationError> {
    let size = s.len();
    if size <= 1 {
        return Ok(s.to_string())
    }

    let mut result = String::with_capacity(size_grow_up(size));
    let mut enter_brace = false; // in case like {...}
    let mut enter_positional = false; // in case like {1}
    let mut left_index = 0;
    let mut colon_index = 0; // the index of char `:`
    let mut default_index = 0;
    let mut chars = s.char_indices();
    loop {
        let c;
        let i;
        if let Some((ind, chr)) = chars.next() {
            c = chr;
            i = ind;
        } else {
            break
        }

        if enter_brace {
            if c == '\\' {
                if i == size - 1 {
                    return Err(invalid_char_err(c, i))
                }
                chars.next(); // treat the next char as a normal char
                continue
            }
            if c == ':' && colon_index == 0 {
                colon_index = i;
                continue
            }
            if c != '}' {
                continue
            }
            // extract xxx from ${xxx}
            let right_index = if colon_index != 0 { colon_index } else { i };
            let variable = s
                .get(left_index..right_index)
                .ok_or_else(|| invalid_string_err(left_index, right_index))?;
            // remove escape character
            let variable = from_backslash(variable);
            if let Some(value) = named.get(&variable) {
                result.push_str(value);
            } else {
                // has no default value
                if colon_index == 0 {
                    add_default_value(&mut result, default_value, variable)?;
                } else {
                    // extract ### from ${@@@:###}
                    let value = s
                        .get((colon_index + 1)..i)
                        .ok_or_else(|| invalid_string_err(colon_index + 1, i))?;
                    let value = from_backslash(value);
                    result.push_str(&value);
                }
            }
            enter_brace = false;
            colon_index = 0;
            continue
        }
        if enter_positional {
            if is_number_char(c) {
                continue
            }
            if c != '}' {
                return Err(invalid_char_err(c, i))
            }
            let variable = s
                .get(left_index..i)
                .ok_or_else(|| invalid_string_err(left_index, i))?;
            let n = variable
                .parse::<usize>()
                .map_err(|e| number_parse_err(left_index, variable.to_string(), e.to_string()))?;
            if let Some(argument) = positional.get(n) {
                result.push_str(argument);
            } else {
                add_default_value(&mut result, default_value, variable.to_string())?;
            }
            enter_positional = false;
            continue
        }
        if i == size - 1 {
            result.push(c);
            break
        }
        // escape char
        if c == '\\' {
            let (_, next) = chars.next().ok_or_else(|| invalid_char_err(c, i + 1))?;
            result.push(next);
            continue
        }
        if c != '{' {
            result.push(c);
            continue
        }
        // then c is {
        let (i, next) = chars.next().ok_or_else(|| invalid_char_err(c, i + 1))?;
        if next == '}' {
            if let Some(argument) = positional.get(default_index) {
                result.push_str(argument);
            } else {
                add_default_value_positional(&mut result, default_value, default_index)?;
            }
            default_index += 1;
            continue
        }
        if is_number_char(next) {
            left_index = i;
            enter_positional = true;
            continue
        }
        if is_first_variable_char(next) {
            left_index = i;
            enter_brace = true;
            continue
        }
        return Err(invalid_char_err(next, i))
    }
    Ok(result)
}

pub fn brace_named(
    s: &str, named: &HashMap<String, String>, default_value: Option<&str>,
) -> Result<String, InterpolationError> {
    brace(s, named, &Vec::new(), default_value)
}

pub fn brace_positional(
    s: &str, positional: &Vec<String>, default_value: Option<&str>,
) -> Result<String, InterpolationError> {
    brace(s, &HashMap::new(), positional, default_value)
}

pub fn brace_named_unwrap(
    s: &str, named: &HashMap<String, String>, default_value: Option<&str>,
) -> String {
    brace_named(s, named, default_value).unwrap()
}

pub fn brace_positional_unwrap(
    s: &str, positional: &Vec<String>, default_value: Option<&str>,
) -> String {
    brace_positional(s, positional, default_value).unwrap()
}
