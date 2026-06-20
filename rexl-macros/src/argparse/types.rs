use crate::argparse::attrs::ArgParserFieldAttrs;
use crate::util::{
    get_map_type, get_option_type, get_vec_type, is_bool_type, is_float_type, is_integer_type,
    is_string_type,
};
use quote::quote;
use std::any::Any;
use std::fmt;
use std::fmt::{Display, Formatter};
use syn::{Field, Type};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum ArgumentKind {
    String,
    Bool,
    Integer,
    Float,
    Property,
}

impl Display for ArgumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(crate) struct ArgParserType {
    pub kind: ArgumentKind,
    pub multiple: bool,
    pub optional: bool,
    pub value_type: Option<Type>,
}

impl ArgParserType {
    pub fn parse(field: &Field, field_attrs: &ArgParserFieldAttrs) -> Result<Self, syn::Error> {
        let mut multiple = false;
        let mut optional = false;

        let field_type = &field.ty;
        let option_kind = get_kind(field_type);
        if let Some(kind) = option_kind {
            return Ok(Self { kind, multiple, optional, value_type: Some(field_type.clone()) });
        }

        let option_type = get_option_type(field_type);
        if let Some(ty) = option_type {
            optional = true;
            let option_kind = get_kind(ty);
            if let Some(kind) = option_kind {
                return Ok(Self { kind, multiple, optional, value_type: Some(ty.clone()) });
            } else if field_attrs.from_str {
                return Ok(Self {
                    kind: ArgumentKind::String,
                    multiple,
                    optional,
                    value_type: Some(ty.clone()),
                });
            } else {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("Option inner type {:?} not supported", ty.type_id()),
                ));
            }
        }

        let vec_type = get_vec_type(field_type);
        if let Some(ty) = vec_type {
            multiple = true;
            let option_kind = get_kind(ty);
            if let Some(kind) = option_kind {
                return Ok(Self { kind, multiple, optional, value_type: Some(ty.clone()) });
            } else if field_attrs.from_str {
                return Ok(Self {
                    kind: ArgumentKind::String,
                    multiple,
                    optional,
                    value_type: Some(ty.clone()),
                });
            } else {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("Vec inner type {:?} not supported", ty.type_id()),
                ));
            }
        }

        let map_type = get_map_type(field_type);
        if let Some((ty1, ty2)) = map_type {
            if !is_string_type(ty1) {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("Only String is supported for Map inner type1"),
                ));
            }
            if !is_string_type(ty2) {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("Only String is supported for Map inner type2"),
                ));
            }
            return Ok(Self { kind: ArgumentKind::Property, multiple, optional, value_type: None });
        }

        if field_attrs.from_str {
            return Ok(Self {
                kind: ArgumentKind::String,
                multiple,
                optional,
                value_type: Some(field_type.clone()),
            });
        }

        let type_str = quote!(#field_type).to_string();
        Err(syn::Error::new_spanned(
            field,
            format!("field type {:?} not supported", type_str),
        ))
    }
}

fn get_kind(ty: &Type) -> Option<ArgumentKind> {
    if is_bool_type(ty) {
        Some(ArgumentKind::Bool)
    } else if is_string_type(ty) {
        Some(ArgumentKind::String)
    } else if is_integer_type(ty) {
        Some(ArgumentKind::Integer)
    } else if is_float_type(ty) {
        Some(ArgumentKind::Float)
    } else {
        None
    }
}
