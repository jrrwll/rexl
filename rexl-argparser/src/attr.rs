use proc_macro2::Ident;
use syn::{Expr, ExprAssign, Field, Lit, Meta, MetaList};

#[derive(Default)]
pub(crate) struct ArgParserAttrs {
    value: Vec<String>,
    required: bool,
    ignored: bool,
    position: Option<u8>,
    positions: bool,
}

pub(crate) fn get_field_attr(field: &Field) -> syn::Result<Option<ArgParserAttrs>> {
    let attrs = ArgParserAttrs::default();
    for attr in &field.attrs {
        if !attr.path().is_ident("arg_parser") { continue }


        // let expr: Expr = attr.parse_args()?;
        // if let Expr::Assign(ExprAssign {ref a: b, ..}) = expr {
        //
        // }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("value") {
                return Ok(())
            }
            return Ok(())
        })?;
        if let Meta::List(list) = &attr.meta {
            let tokens = &list.tokens;

                    // if let Some(NestedMeta::Meta(Meta::NameValue(kv))) = nested.first() {
                    //     if kv.path.is_ident("each") {
                    //         if let Lit::Str(ref ident_str) = kv.lit {
                    //
                    //         }
                    //     } else {
                    //         if let Ok(Meta::List(ref list)) = attr.parse_meta() {
                    //             let msg = format!("unexpected `arg_parser({:?} = ...)`", kv.path.get_ident());
                    //             return Err(syn::Error::new_spanned(list, msg));
                    //         }
                    //     }
                    // }


        }
    }
    Ok(Some(attrs))
}
