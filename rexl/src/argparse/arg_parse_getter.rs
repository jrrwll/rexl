use crate::argparse::{ArgParser, ArgParserError, Argument, ParseErrorValue};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

macro_rules! impl_getters {
    ($getter_name:ident, $or_default:ident, $field:ident, $value_type:ty) => {
        #[inline]
        pub fn $getter_name(&self, key: &str) -> Option<$value_type> {
            self.$field.get(key).map(|v| v.clone())
        }

        #[inline]
        pub fn $or_default(&self, key: &str) -> $value_type {
            self.$getter_name(key).unwrap_or_default()
        }
    };
}

macro_rules! impl_multiple_getters {
    ($getter_name:ident, $field:ident, $value_type:ty) => {
        #[inline]
        pub fn $getter_name(&self, key: &str) -> Cow<'_, Vec<$value_type>> {
            self.$field
                .get(key)
                .map_or_else(|| Cow::Owned(Vec::new()), |v| Cow::Borrowed(v))
        }
    };
}

impl ArgParser {
    #[inline]
    pub fn get_bool(&self, key: &str) -> bool {
        self.bool_map
            .get(key)
            .map_or(false, |v| v.clone())
    }

    #[inline]
    pub fn get_bool_option(&self, key: &str) -> Option<bool> {
        self.bool_map.get(key).map(|v| v.clone())
    }

    impl_getters!(get_string, get_string_or_default, string_map, String);
    impl_getters!(get_integer, get_integer_or_default, integer_map, i64);
    impl_getters!(get_float, get_float_or_default, float_map, f64);

    impl_multiple_getters!(get_strings, strings_map, String);
    impl_multiple_getters!(get_integers, integers_map, i64);
    impl_multiple_getters!(get_floats, floats_map, f64);

    pub fn get_from_str<F, T, E>(
        &self, key: &str, from_str: F,
    ) -> Result<Option<T>, ArgParserError>
    where
        F: Fn(&str) -> Result<T, E>,
        E: Debug, {
        let Some(argument) = self.key_argument_map.get(key) else {
            return Err(ArgParserError::UnexpectedArg(key.to_string()));
        };

        let Some(value) = self.string_map.get(key) else {
            return Ok(None);
        };

        from_str(value)
            .map_err(|e| value_parse_error(e, value, argument))
            .map(|v| Some(v))
    }

    pub fn get_from_str_or_err<F, T, E>(
        &self, key: &str, from_str: F,
    ) -> Result<T, ArgParserError>
    where
        F: Fn(&str) -> Result<T, E>,
        E: Debug, {
        let Some(argument) = self.key_argument_map.get(key) else {
            return Err(ArgParserError::UnexpectedArg(key.to_string()));
        };

        let Some(value) = self.string_map.get(key) else {
            return Err(ArgParserError::NoArgPassed(argument.clone()));
        };

        from_str(value).map_err(|e| value_parse_error(e, value, argument))
    }

    pub fn get_multiple_from_str<F, T, E>(
        &self, key: &str, from_str: F,
    ) -> Result<Vec<T>, ArgParserError>
    where
        F: Fn(&str) -> Result<T, E>,
        E: Debug, {
        let Some(argument) = self.key_argument_map.get(key) else {
            return Err(ArgParserError::UnexpectedArg(key.to_string()));
        };

        let Some(value) = self.strings_map.get(key) else { return Ok(Vec::new()) };

        value
            .iter()
            .map(|v| from_str(v).map_err(|e| value_parse_error(e, v, argument)))
            .collect::<Result<Vec<T>, ArgParserError>>()
    }

    #[inline]
    pub fn get_properties(&self, key: &str) -> Cow<'_, HashMap<String, String>> {
        self.properties_map
            .get(key)
            .map_or_else(|| Cow::Owned(HashMap::new()), |v| Cow::Borrowed(v))
    }

    #[inline]
    pub fn get_position_values(&self) -> &Vec<String> {
        &self.position_values
    }
}

fn value_parse_error<E: Debug>(e: E, value: &str, argument: &Argument) -> ArgParserError {
    let source =
        if value.len() <= 100 { value.to_string() } else { format!("{}...", &value[0..100]) };
    let error = format!("{:?}", e);
    ArgParserError::ValueParse(ParseErrorValue { argument: argument.clone(), source, error })
}
