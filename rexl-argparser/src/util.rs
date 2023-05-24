use syn::{AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, PathArguments, Token, Type, TypePath};
use syn::punctuated::Punctuated;

pub(crate) type StructFields = Punctuated<Field, Token!(,)>;

// get fields from a struct
pub(crate) fn get_fields_from_derive_input(input: &DeriveInput) -> syn::Result<&StructFields> {
    if let Data::Struct(
        DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) = input.data {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(input, "Must define on a Struct, not Enum".to_string()))
}

// get T of Vec<T> or Option<T>
pub(crate) fn get_generic_inner_type<'a>(ty: &'a Type, outer_ident_name: &str) -> Option<&'a Type> {
    if let Type::Path(TypePath { ref path, .. }) = ty {
        if let Some(seg) = path.segments.last() {
            if seg.ident == outer_ident_name {
                if let PathArguments::AngleBracketed(
                    AngleBracketedGenericArguments {
                        ref args, .. }) = seg.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
