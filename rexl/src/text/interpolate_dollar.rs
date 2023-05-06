use std::collections::HashMap;
use crate::text::*;
use crate::text::interpolate::*;

///! string interpolation by dollar sign
/// `[$][a-zA-Z0-9]+` and `[$]{[a-zA-Z0-9]+:.*}` is supported
/// Example:
/// $x_y_z_1, the variable is x_y_z_1, and has no default value
/// ${x_y_z_1:the_default_value}, the variable is x_y_z_1, and default is: the_default_value
/// ${{\:\}\\:{\:\}\\}, the variable is {:}\, and default is {:}\
pub fn dollar(
    s: &str, named: &HashMap<String, String>, positional: &Vec<String>, default_value: Option<&str>,
) -> Result<String, InterpolationError> {
    let size = s.len();
    let mut result = String::with_capacity(size_grow_up(size));
    let mut enter_dollar = false; // in case like $...
    let mut enter_positional = false; // in case like $1
    let mut enter_dollar_brace = false; // in case like ${...}
    let mut left_index = 0;
    let mut colon_index = 0; // the index of char `:`
    let mut chars = s.char_indices();
    let mut previous: Option<(usize, char)> = None;
    loop {
        let c;
        let i;
        if let Some((ind, chr)) = previous {
            c = chr;
            i = ind;
            previous = None;
        } else if let Some((ind, chr)) = chars.next() {
            c = chr;
            i = ind;
        } else {
            break
        }

        if enter_dollar_brace {
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
            enter_dollar_brace = false;
            colon_index = 0;
            continue
        }
        if enter_dollar {
            if is_variable_char(c) {
                continue
            }
            let variable = s
                .get(left_index..i)
                .ok_or_else(|| invalid_string_err(left_index, i))?;
            if let Some(value) = named.get(variable) {
                result.push_str(value);
            } else {
                add_default_value(&mut result, default_value, variable.to_string())?;
            }

            previous = Some((i - 1, c));
            enter_dollar = false;
            continue
        }
        if enter_positional {
            if is_number_char(c) {
                continue
            }
            let variable = s
                .get(left_index..i)
                .ok_or_else(|| invalid_string_err(left_index, i))?;
            let mut n = variable.parse::<usize>().map_err(|e| {
                InterpolationError::NumberParse(NumberParseValue {
                    offset: left_index,
                    source: variable.to_string(),
                    error:  e.to_string(),
                })
            })?;
            n -= 1;
            if let Some(argument) = positional.get(n) {
                result.push_str(argument);
            } else {
                add_default_value_positional(&mut result, default_value, n)?;
            }
            previous = Some((i - 1, c));
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
        if c != '$' {
            result.push(c);
            continue
        }
        // then c is $
        let (i, next) = chars.next().ok_or_else(|| invalid_char_err(c, i + 1))?;
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
            continue
        }
        result.push(c);
        result.push(next);
    }
    Ok(result)
}

pub fn dollar_named(
    s: &str, named: &HashMap<String, String>, default_value: Option<&str>,
) -> Result<String, InterpolationError> {
    dollar(s, named, &Vec::new(), default_value)
}

pub fn dollar_positional(
    s: &str, positional: &Vec<String>, default_value: Option<&str>,
) -> Result<String, InterpolationError> {
    dollar(s, &HashMap::new(), positional, default_value)
}

pub fn dollar_named_unwrap(
    s: &str, named: &HashMap<String, String>, default_value: Option<&str>,
) -> String {
    dollar_named(s, &named, default_value).unwrap()
}

pub fn dollar_positional_unwrap(
    s: &str, positional: Vec<String>, default_value: Option<&str>,
) -> String {
    dollar_positional(s, &positional, default_value).unwrap()
}
