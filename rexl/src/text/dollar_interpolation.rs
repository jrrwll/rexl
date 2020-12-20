use crate::text::{from_backslash, is_variable_char};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum InterpolationError {
    MismatchedEscape,
    MissingVariable(String),
}

///! string interpolation by dollar sign
/// `[$][a-zA-Z0-9]+` and `[$]{[a-zA-Z0-9]+:.*}` is supported
/// Example:
/// $x_y_z_1, the variable is x_y_z_1, and has no default value
/// ${x_y_z_1:the_default_value}, the variable is x_y_z_1, and default is: the_default_value
/// ${{\:\}\\:{\:\}\\}, the variable is {:}\, and default is {:}\
pub fn dollar_interpolate(s: &str, context: &HashMap<String, String>, default_value: Option<&str>)
    -> Result<String, InterpolationError>{
    let size = s.len();
    let mut result = String::with_capacity(size + size >> 1);
    let mut enter_dollar = false; // in case like $...
    let mut enter_dollar_brace = false; // in case like ${...}
    let mut left_index = 0;
    let mut colon_index = 0; // the index of char `:`
    let mut k = -1;
    loop {
        k += 1;
        let mut i = k as usize;
        if i >= size { break }
        let c = s.get(i..(i+1)).unwrap().chars().next().unwrap();
        if enter_dollar_brace {
            if c == '\\' {
                if i == size - 1 {
                    return Err(InterpolationError::MismatchedEscape)
                }
                k += 1;
                continue
            }
            if c == ':' && colon_index == 0 {
                colon_index = i;
                continue
            }
            if c != '}' { continue }
            // extract xxx from ${xxx}
            let right_index = if colon_index != 0 { colon_index } else { i };
            let variable = s.get(left_index..right_index).unwrap();
            // remove escape character
            let variable = from_backslash(variable);
            if let Some(value) = context.get(&variable) {
                result.push_str(value);
            } else {
                // has no default value
                if colon_index == 0 {
                    if let Some(some_default_value) = default_value {
                        result.push_str(some_default_value);
                    } else {
                        return Err(InterpolationError::MissingVariable(variable))
                    }
                } else {
                    // extract ### from ${@@@:###}
                    let value = s.get((colon_index + 1)..i).unwrap();
                    let value = from_backslash(value);
                    result.push_str(&value);
                }
            }
            enter_dollar_brace = false;
            colon_index = 0;
            continue;
        }
        if enter_dollar {
            if is_variable_char(c) { continue }
            let variable = s.get(left_index..i).unwrap();
            if let Some(value) = context.get(variable) {
                result.push_str(value);
            } else {
                if let Some(some_default_value) = default_value {
                    result.push_str(some_default_value);
                } else {
                    return Err(InterpolationError::MissingVariable(variable.to_string()))
                }
            }
            k -= 1;
            enter_dollar = false;
            continue
        }
        if c != '$' {
            result.push(c);
            continue
        }
        // then c is $
        if i == size - 1 {
            result.push(c);
            break
        }
        k += 1;
        i = k as usize;
        let next = s.get(i..(i+1)).unwrap().chars().next().unwrap();
        if is_variable_char(next) {
            left_index = i;
            enter_dollar = true;
            continue
        }
        if next == '{' {
            left_index = i + 1;
            enter_dollar_brace = true;
            continue;
        }
        result.push(c);
        result.push(next);
    }
    Ok(result)
}