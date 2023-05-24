pub(crate) mod util;
pub(crate) mod attr;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Fields, Data, Field};
use crate::util::{get_fields_from_derive_input, StructFields};


#[proc_macro_derive(ArgParser, attributes(arg_parser_value, arg_parser_position))]
pub fn derive_arg_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match do_expand(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let parser_ident = format_ident!("parser");
    let fields = get_fields_from_derive_input(input)?;

    let argparse_arg_defs = argparse_arg_defs(fields, &parser_ident)?;
    let ident_init_clauses = ident_init_clauses(fields, &parser_ident)?;

    let tokens = quote! {
        impl #ident {
            pub fn parse_from_arg(args: Vec<String>) -> Result<#ident, rexl::argparse::ArgParserError<String>> {
                // let mut #parser_ident = rexl::argparse::ArgParser::new();
                // #argparse_arg_defs
                // #parser_ident.parse(args)?
                // #ident{
                //     #(#ident_init_clauses),*
                // }
                Ok(#ident::default())
            }
        }
    };
    Ok(tokens)
}

fn argparse_arg_defs(fields: &StructFields, parser_ident: &Ident) -> syn::Result<TokenStream> {
    // fields.iter().map(|f| {
    //     ()
    // }).collect();
    Ok(quote!{

    })
}

fn ident_init_clauses(fields: &StructFields, parser_ident: &Ident) -> syn::Result<Vec<TokenStream>> {
    Ok(vec!{

    })
}

