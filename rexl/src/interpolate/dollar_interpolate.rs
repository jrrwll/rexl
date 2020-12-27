use std::collections::HashMap;

use crate::text::*;
use crate::interpolate::*;

///! string interpolation by dollar sign
/// `[$][a-zA-Z0-9]+` and `[$]{[a-zA-Z0-9]+:.*}` is supported
/// Example:
/// $x_y_z_1, the variable is x_y_z_1, and has no default value
/// ${x_y_z_1:the_default_value}, the variable is x_y_z_1, and default is: the_default_value
/// ${{\:\}\\:{\:\}\\}, the variable is {:}\, and default is {:}\
pub fn dollar(s: &str, named: &HashMap<String, String>, positional: &Vec<String>,
              default_value: Option<&str>) -> Result<String, InterpolationError>{
    let size = s.len();
    let mut result = String::with_capacity(size_grow_up(size));
    let mut enter_dollar = false; // in case like $...
    let mut enter_positional = false; // in case like $1
    let mut enter_dollar_brace = false; // in case like ${...}
    let mut left_index = 0;
    let mut colon_index = 0; // the index of char `:`
    let mut k: isize = -1;
    loop {
        k += 1;
        let mut i = k as usize;
        if i >= size { break }
        let c = s.get(i..(i+1)).ok_or(unexpected_err(s, i, "s.get(i..(i+1))"))?
            .chars().next().ok_or(unexpected_err(s, i, "chars().next()"))?;
        if enter_dollar_brace {
            if c == '\\' {
                if i == size - 1 {
                    return Err(invalid_char_err(c, i))
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
            if let Some(value) = named.get(&variable) {
                result.push_str(value);
            } else {
                // has no default value
                if colon_index == 0 {
                    add_default_value(&mut result, default_value, variable)?;
                } else {
                    // extract ### from ${@@@:###}
                    let value = s.get((colon_index + 1)..i).unwrap();
                    let value = from_backslash(value);
                    result.push_str(&value);
                }
            }
            enter_dollar_brace = false;
            colon_index = 0;
            continue
        }
        if enter_dollar {
            if is_variable_char(c) { continue }
            let variable = s.get(left_index..i).unwrap();
            if let Some(value) = named.get(variable) {
                result.push_str(value);
            } else {
                add_default_value(&mut result, default_value, variable.to_string())?;
            }
            k -= 1;
            enter_dollar = false;
            continue
        }
        if enter_positional {
            if is_number_char(c) { continue }
            let variable = s.get(left_index..i).unwrap();
            let mut n = variable.parse::<usize>().map_err(
                |e| InterpolationError::NumberParse(NumberParseValue{
                    offset: left_index,
                    source: variable.to_string(),
                    error: e.to_string()
                }))?;
            n -= 1;
            if let Some(argument) = positional.get(n) {
                result.push_str(argument);
            } else {
                add_default_value_positional(&mut result, default_value, n)?;
            }
            k -= 1;
            enter_positional = false;
            continue
        }
        if i == size - 1 {
            result.push(c);
            break
        }
        // escape char
        if c == '\\' {
            k += 1;
            i = k as usize;
            let next = s.get(i..(i+1)).unwrap().chars().next().unwrap();
            result.push(next);
            continue;
        }
        if c != '$' {
            result.push(c);
            continue
        }
        // then c is $
        k += 1;
        i = k as usize;
        let next = s.get(i..(i+1)).unwrap().chars().next().unwrap();
        if is_number_char(next) {
            left_index = i;
            enter_positional = true;
            continue
        }
        if is_first_variable_char(next) {
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

pub fn dollar_named(s: &str, named: &HashMap<String, String>, default_value: Option<&str>)
    -> Result<String, InterpolationError>{
    dollar(s, named, &Vec::new(), default_value)
}

pub fn dollar_positional(s: &str, positional: &Vec<String>, default_value: Option<&str>)
    -> Result<String, InterpolationError>{
    dollar(s, &HashMap::new(), positional, default_value)
}

pub fn dollar_unwrap(s: &str, positional: Vec<String>) -> String {
    dollar_positional(s, &positional, None).unwrap()
}