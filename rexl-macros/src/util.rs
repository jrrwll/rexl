use syn::punctuated::Punctuated;
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, PathArguments,
    Token, Type, TypePath,
};

pub(crate) type StructFields = Punctuated<Field, Token!(,)>;

// get fields from a struct
pub(crate) fn get_fields_from_derive_input(input: &DeriveInput) -> syn::Result<&StructFields> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }), ..
    }) = input.data
    {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(input, "Must define on a Struct, not Enum".to_string()))
}

pub(crate) fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        return path.is_ident("bool");
    }
    false
}

pub(crate) fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        return path.is_ident("String")
            || (path.segments.len() == 1 && path.segments[0].ident == "String");
    }
    false
}

pub(crate) fn is_integer_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        let type_name = path.get_ident().map(|i| i.to_string());
        return matches!(
            type_name.as_deref(),
            Some(
                "i8" | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
            )
        );
    }
    false
}

pub(crate) fn is_float_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        let type_name = path.get_ident().map(|i| i.to_string());
        return matches!(type_name.as_deref(), Some("f32" | "f64"));
    }
    false
}

pub(crate) fn get_option_type(ty: &Type) -> Option<&Type> {
    get_inner_type(ty, "Option")
}

pub(crate) fn get_vec_type(ty: &Type) -> Option<&Type> {
    get_inner_type(ty, "Vec")
}

pub(crate) fn get_inner_type<'a>(ty: &'a Type, type_name: &str) -> Option<&'a Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        let segments = &path.segments;
        if let Some(last) = segments.last() {
            let ident_name = last.ident.to_string();
            if ident_name != type_name {
                return None;
            }
            if let PathArguments::AngleBracketed(args) = &last.arguments {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    return Some(inner_type);
                }
            }
        }
    }
    None
}

pub(crate) fn get_map_type(ty: &Type) -> Option<(&Type, &Type)> {
    get_inner_type2(ty, "HashMap").or_else(|| get_inner_type2(ty, "BTreeMap"))
}

pub(crate) fn get_inner_type2<'a>(ty: &'a Type, type_name: &str) -> Option<(&'a Type, &'a Type)> {
    if let Type::Path(TypePath { path, .. }) = ty {
        let segments = &path.segments;
        if let Some(last) = segments.last() {
            let ident_name = last.ident.to_string();
            if ident_name != type_name {
                return None;
            }
            if let PathArguments::AngleBracketed(args) = &last.arguments {
                if args.args.len() != 2 {
                    return None;
                }

                let inner_type1: &Type;
                if let Some(GenericArgument::Type(ty)) = args.args.first() {
                    inner_type1 = ty;
                } else {
                    return None;
                }
                let inner_type2: &Type;
                if let Some(GenericArgument::Type(ty)) = args.args.last() {
                    inner_type2 = ty;
                } else {
                    return None;
                }
                return Some((inner_type1, inner_type2));
            }
        }
    }
    None
}

pub(crate) fn get_field_name(field: &Field) -> syn::Result<String> {
    field
        .ident
        .as_ref()
        .cloned()
        .map(|ident| ident.to_string())
        .ok_or_else(|| syn::Error::new_spanned(field, "tuple struct not supported"))
}
