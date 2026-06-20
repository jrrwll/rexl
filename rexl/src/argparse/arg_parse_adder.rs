use crate::argparse::{ArgParser, Argument, ArgumentKind};

macro_rules! impl_adders {
    ($adder_name:ident, $multiple_adder_name:ident, $kind:ident) => {
        #[inline]
        pub fn $adder_name<K: Into<String>>(&mut self, key: K, names: Vec<&str>) -> &mut Self {
            self.add_by_kind(key, names, ArgumentKind::$kind, false)
        }

        #[inline]
        pub fn $multiple_adder_name<K: Into<String>>(
            &mut self, key: K, names: Vec<&str>,
        ) -> &mut Self {
            self.add_by_kind(key, names, ArgumentKind::$kind, true)
        }
    };
}

impl ArgParser {
    // -a 1.ts -a 2.ts
    impl_adders!(add, add_multiple, String);
    impl_adders!(add_integer, add_integer_multiple, Integer);
    impl_adders!(add_float, add_float_multiple, Float);

    // aux, lah, cvzf, -ef
    // only allow single char key and bool value
    #[inline]
    pub fn add_bool<K: Into<String>>(&mut self, key: K, names: Vec<&str>) -> &mut Self {
        self.add_by_kind(key, names, ArgumentKind::Bool, false)
    }

    // only support single char argparse
    // -Dfile.encoding=utf8 -Dspring.active.profile=dev
    #[inline]
    pub fn add_property<K: Into<String>>(&mut self, key: K, names: Vec<&str>) -> &mut Self {
        self.add_by_kind(key, names, ArgumentKind::Property, true)
    }

    // -L 3, -L=3, -L3, --level 3, --level=3
    // -o yaml, -o=yaml, -oyaml, --output yaml, --output=yaml
    // -r, -r=true, -r true, --rm, --rm=true, --rm true
    pub fn add_by_kind<K: Into<String>>(
        &mut self, key: K, names: Vec<&str>, kind: ArgumentKind, multiple: bool,
    ) -> &mut Self {
        let key = key.into();
        let names = names.iter().map(|s| s.to_string()).collect();
        self.key_argument_map
            .insert(key.clone(), Argument { key, names, kind, multiple });
        self
    }
}
