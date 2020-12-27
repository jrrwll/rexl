use std::collections::{LinkedList, HashMap};
use std::hash::Hash;

use crate::cli::*;
use std::fmt::Debug;

static MULTIPLE_VALUE_SIZE: usize = 16;

impl<K: Hash + Eq + Debug + Clone> ArgParser<K> {

    #[inline]
    pub fn parse(&mut self, args: Vec<String>) -> Result<(), ArgParserError<K>> {
        self._parse(args, false)
    }

    #[inline]
    pub fn parse_bsd(&mut self, args: Vec<String>) -> Result<(), ArgParserError<K>> {
        self._parse(args, true)
    }

    fn _parse(&mut self, args: Vec<String>, bsd: bool) -> Result<(), ArgParserError<K>> {
        let mut shift = LinkedList::new();
        for arg in args {
            shift.push_front(arg)
        }
        if shift.is_empty() {
            // no any args is passed
            return Err(ArgParserError::NoArgs)
        }

        self._init_name_key_map();

        // handle bsd part
        // like ps -ef ... or ps aux ...
        if bsd {
            if let Some(name) = shift.pop_back() {
                self._parse_bsd_args(&name)?
            }
        }

        // handle no-bsd part
        // -n default -oyaml --type pom --rm=true --force
        // -Dspring.active.profile=dev -Dlog4j.debug=true
        // -i 1 2 3 --property:key=value extra1 extra2
        while !shift.is_empty() {
            let arg_name = shift.pop_back().unwrap();

            // name case
            // --rm = true - -type pom
            if arg_name.starts_with("--") {
                self._parse_double_minus(&arg_name, &mut shift)?;
            }
            // -P3306, -o yml, -f=%h-%m-%s, -Dlogger.level=debug
            else if arg_name.starts_with("-") {
                self._parse_minus(&arg_name, &mut shift)?;
            }
            // extra values
            else {
                self.extra_values.push(arg_name);
            }
        }
        Ok(())
    }

    fn _init_name_key_map(&mut self) {
        for (key, arg) in self.key_argument_map.iter() {
            for name in arg.names.iter() {
                self.names_key_map.insert(name.to_string(), key.clone());
            }
        }
    }

    fn _get_key(&self, name: &str) -> Result<(K, Argument<K>), ArgParserError<K>>{
        if let Some(key) = self.names_key_map.get(name) {
            if let Some(argument) = self.key_argument_map.get(key) {
                return Ok((key.clone(), argument.clone()))
            }
        }
        Err(ArgParserError::UnexpectedArg(name.to_string()))
    }

    fn _parse_bsd_args(&mut self, arg_names: &String) -> Result<(), ArgParserError<K>> {
        let start_index = if arg_names.starts_with("-") { 1 } else { 0 };
        let size = arg_names.len();
        for i in start_index..size {
            let arg_name = &arg_names[i..(i+1)];
            let (key, _) = self._get_key(arg_name)?;
            self.bool_map.insert(key, true);
        }
        Ok(())
    }

    fn _parse_double_minus(&mut self, arg_name: &String, shift: &mut LinkedList<String>)
        -> Result<(), ArgParserError<K>> {
        let arg_name_len = arg_name.len();
        // treat argument behind `--` as extra values
        if arg_name_len == 2 {
            self._add_values(shift);
            return Ok(())
        }
        // extract 'xxx' from '--xxx'
        let new_arg_name = &arg_name[2..arg_name_len];
        // the case, not `--rm=true` but `--type pom`
        if !new_arg_name.contains("=") {
            let (key, argument) = self._get_key(new_arg_name)?;
            return self._add_all_forward(key, argument, shift)
        }
        // the case, not `--type pom` but `--rm=true`
        let ind = new_arg_name.find("=").unwrap();
        let prefix = &new_arg_name[0..ind];
        let (key, argument) = self._get_key(prefix)?;

        // the case `--rm=`
        if  ind == new_arg_name.len() - 1 {
            argument.check_kind(ArgumentKind::Bool)?;
            self.bool_map.insert(key, true);
            return Ok(())
        }
        // the case `--rm=true`
        let value = &new_arg_name[(ind + 1)..];
        self._add_all(key, argument, value.to_string())
    }

    fn _parse_minus(&mut self, arg_name: &String, shift: &mut LinkedList<String>)
        -> Result<(), ArgParserError<K>> {
        let arg_name_len = arg_name.len();
        // only a `-`
        if arg_name_len == 1 {
            self.extra_values.push(arg_name.to_string());
            return Ok(())
        }
        // extract 'xxx' from '-xxx'
        let new_arg_name = &arg_name[1..arg_name_len];
        let name = &new_arg_name[..1];
        let (key, argument) = self._get_key(name)?;
        // the case `-o yaml`
        if new_arg_name.len() == 1 {
            return self._add_all_forward(key, argument, shift)
        }
        // the case `-P3306 -f=%h-%m-%s, -Dlogger.level=debug`
        let value = &new_arg_name[1..];
        // the case `-P3306`
        if !new_arg_name.contains("=") {
            return self._add_all(key, argument, value.to_string())
        }
        // the case `-f=%h-%m-%s`
        let mut ind = new_arg_name.find("=").unwrap();
        if ind == 0 {
            // the case, `-o=`
            if value.len() == 1 {
                argument.check_kind(ArgumentKind::Bool)?;
                self.bool_map.insert(key, true);
                return Ok(())
            }
            let value = &value[1..];
            return self._add_all(key, argument, value.to_string())
        }
        // the case, `-Dlogger.level=debug`
        argument.check_kind(ArgumentKind::Property)?;
        ind -= 1;
        let hkey = &value[..ind];
        // the case, `-Dlogger.level=`
        let hval = if value.len() == ind {
            ""
        } else {
            &value[(ind + 1)..]
        }.to_string();
        self._add_property(key, hkey.to_string(), hval);
        return Ok(())
    }

    fn _add_values(&mut self, shift: &mut LinkedList<String>) {
        while !shift.is_empty() {
            let arg_name = shift.pop_back().unwrap();
            self.extra_values.push(arg_name)
        }
    }

    fn _add_all(&mut self, key: K, argument: Argument<K>, value: String)
        -> Result<(), ArgParserError<K>>{
        match argument.kind {
            ArgumentKind::Bool => {
                let value = argument.parse_bool(value)?;
                self.bool_map.insert(key, value);
            }
            ArgumentKind::String => {
                self.string_map.insert(key, value);
            }
            ArgumentKind::Integer => {
                let value = argument.parse_i64(value)?;
                self.integer_map.insert(key, value);
            }
            ArgumentKind::Float => {
                let value = argument.parse_f64(value)?;
                self.float_map.insert(key, value);
            }
            ArgumentKind::Property => {
                return Err(ArgParserError::NoProperties(argument))
            }
        }
        Ok(())
    }


    fn _add_all_forward(&mut self, key: K, argument: Argument<K>, shift: &mut LinkedList<String>)
        -> Result<(), ArgParserError<K>> {
        match argument.kind {
            ArgumentKind::Bool => {
                self.bool_map.insert(key, true);
            }
            ArgumentKind::String => {
                if shift.is_empty() {
                    return Err(ArgParserError::MissingValue(argument))
                }
                if !argument.multiple {
                    // unwrap since we check `shift` is not empty
                    self.string_map.insert(key, shift.pop_back().unwrap());
                } else {
                    while let Some(value) = shift.pop_back() {
                        // reach another argument, then go back
                        if value.starts_with("-") {
                            shift.push_back(value);
                            return Ok(())
                        }
                        // clone key in a loop
                        self._add_string_list(key.clone(), value);
                    }
                }
            }
            ArgumentKind::Integer => {
                if shift.is_empty() {
                    return Err(ArgParserError::MissingValue(argument))
                }
                if !argument.multiple {
                    // unwrap since we check `shift` is not empty
                    let value = argument.parse_i64(shift.pop_back().unwrap())?;
                    self.integer_map.insert(key.clone(), value);
                } else {
                    while let Some(value) = shift.pop_back() {
                        // reach another argument, then go back
                        if value.starts_with("-") {
                            shift.push_back(value);
                            return Ok(())
                        }

                        let value = argument.parse_i64(value)?;
                        self._add_integer_list(key.clone(), value);
                    }
                }
            }
            ArgumentKind::Float => {
                if shift.is_empty() {
                    return Err(ArgParserError::MissingValue(argument.clone()))
                }
                if !argument.multiple {
                    // unwrap since we check `shift` is not empty
                    let value = argument.parse_f64(shift.pop_back().unwrap())?;
                    self.float_map.insert(key.clone(), value);
                } else {
                    while let Some(value) = shift.pop_back() {
                        // reach another argument, then go back
                        if value.starts_with("-") {
                            shift.push_back(value);
                            return Ok(())
                        }

                        let value = argument.parse_f64(value)?;
                        self._add_float_list(key.clone(), value);
                    }
                }
            }
            ArgumentKind::Property => {
                return Err(ArgParserError::NoProperties(argument.clone()))
            }
        }
        Ok(())
    }

    fn _add_string_list(&mut self, key: K, value: String) {
        if let Some(vec) = self.strings_map.get_mut(&key) {
            vec.push(value);
        } else {
            let mut vec = Vec::with_capacity(MULTIPLE_VALUE_SIZE);
            vec.push(value);
            self.strings_map.insert(key, vec);
        }
    }

    fn _add_integer_list(&mut self, key: K, value: i64) {
        if let Some(vec) = self.integers_map.get_mut(&key) {
            vec.push(value);
        } else {
            let mut vec = Vec::with_capacity(MULTIPLE_VALUE_SIZE);
            vec.push(value);
            self.integers_map.insert(key, vec);
        }
    }

    fn _add_float_list(&mut self, key: K, value: f64) {
        if let Some(vec) = self.floats_map.get_mut(&key) {
            vec.push(value);
        } else {
            let mut vec = Vec::with_capacity(MULTIPLE_VALUE_SIZE);
            vec.push(value);
            self.floats_map.insert(key, vec);
        }
    }

    fn _add_property(&mut self, key: K, hkey: String, hval: String) {
        if let Some(props) = self.properties_map.get_mut(&key) {
            props.insert(hkey, hval);
        } else {
            let mut props = HashMap::new();
            props.insert(hkey, hval);
            self.properties_map.insert(key, props);
        }
    }

}