mod argparse;

pub(crate) mod util;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(FromArgs, attributes(arg_parser))]
pub fn derive_arg_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = argparse::argparse_expand(&input);
    to_token_stream(output)
}

// function proc macro
#[proc_macro]
pub fn run_with_args_tree(input: TokenStream) -> TokenStream {
    argparse::command::run_with_args_tree(input)
}

fn to_token_stream(tokens: syn::Result<proc_macro2::TokenStream>) -> TokenStream {
    match tokens {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
