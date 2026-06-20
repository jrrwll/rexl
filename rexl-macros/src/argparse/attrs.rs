use std::convert::TryFrom;
use syn::{Attribute, Meta};

const ARG_PARSER_ATTR: &str = "arg_parser";

#[derive(Default)]
pub(crate) struct ArgParserStructAttrs {
    // #[default = true]
    pub first_char: bool,
}

#[derive(Default)]
pub(crate) struct ArgParserFieldAttrs {
    pub names: Vec<String>,
    pub position: Option<u8>,
    pub positions: bool,
    pub from_str: bool,
}

impl TryFrom<&Vec<Attribute>> for ArgParserStructAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut res = Self::default();
        for attr in attrs {
            if !attr.path().is_ident(ARG_PARSER_ATTR) {
                continue;
            }
            let list = match &attr.meta {
                Meta::List(ml) => ml,
                _ => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        format!("expect #[{}(...)]", ARG_PARSER_ATTR),
                    ));
                }
            };
            list.parse_nested_meta(|meta| {
                let path = &meta.path;
                if path.is_ident("first_char") {
                    res.first_char = true;
                    Ok(())
                } else {
                    let ident = path
                        .get_ident()
                        .map_or_else(|| "".to_string(), |ident| ident.to_string());
                    Err(meta.error(format!("unknown {} key {}", ARG_PARSER_ATTR, ident)))
                }
            })?;
        }
        Ok(res)
    }
}

impl TryFrom<&Vec<Attribute>> for ArgParserFieldAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut res = Self::default();
        for attr in attrs {
            if !attr.path().is_ident(ARG_PARSER_ATTR) {
                continue;
            }
            let list = match &attr.meta {
                Meta::List(ml) => ml,
                _ => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        format!("expect #[{}(...)]", ARG_PARSER_ATTR),
                    ));
                }
            };
            list.parse_nested_meta(|meta| {
                let path = &meta.path;
                if path.is_ident("name") {
                    // name = "a,b,c",
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    res.names = lit
                        .value()
                        .split(",")
                        .map(|part| part.trim().to_string())
                        .collect();
                    Ok(())
                } else if path.is_ident("position") {
                    // pos = 1,
                    let v: syn::LitInt = meta.value()?.parse()?;
                    res.position = Some(v.base10_parse()?);
                    Ok(())
                } else if path.is_ident("positions") {
                    // positions,
                    res.positions = true;
                    Ok(())
                } else if path.is_ident("from_str") {
                    // from_str,
                    res.from_str = true;
                    Ok(())
                } else {
                    Err(meta.error("unknown arg_parser key"))
                }
            })?;
        }
        Ok(res)
    }
}

impl ArgParserFieldAttrs {
    pub fn is_position(&self) -> bool {
        self.positions || self.position.is_some()
    }
}
