pub(crate) mod attrs;
pub(crate) mod command;
pub(crate) mod types;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Index};

use crate::util::{StructFields, get_field_name, get_fields_from_derive_input};
use attrs::{ArgParserFieldAttrs, ArgParserStructAttrs};
use types::{ArgParserType, ArgumentKind};

pub(crate) fn argparse_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let fields = get_fields_from_derive_input(input)?;
    let struct_attrs = ArgParserStructAttrs::try_from(&input.attrs)?;

    let parser_ident = format_ident!("parser");
    // parser.add_by_kind(...)
    let argparse_arg_defs = argparse_arg_defs(fields, &parser_ident, &struct_attrs)?;

    // field: parser.get_value(...)
    let argparse_arg_assign = argparse_arg_assign(fields, &parser_ident)?;

    let tokens = quote! {
        impl rexl::argparse::FromArgs for #ident {
            type Output = #ident;

            fn from_args(
                args: Vec<String>
            ) -> Result<Self, rexl::argparse::ArgParserError> {
                let mut #parser_ident = rexl::argparse::ArgParser::default();
                #argparse_arg_defs
                #parser_ident.parse(args)?;
                Ok(Self{
                    #(#argparse_arg_assign),*
                })
            }
        }
    };
    Ok(tokens)
}

fn argparse_arg_defs(
    fields: &StructFields, parser_ident: &Ident, struct_attrs: &ArgParserStructAttrs,
) -> syn::Result<TokenStream> {
    let mut tokens = TokenStream::new();
    for field in fields.iter() {
        let key = get_field_name(field)?;
        let field_attrs = ArgParserFieldAttrs::try_from(&field.attrs)?;
        let field_type = ArgParserType::parse(field, &field_attrs)?;

        if field_attrs.is_position() {
            argparse_position_type_check(&field_type, &field_attrs, field)?;
            continue;
        }

        let key_token = quote!(#key);

        let mut names = field_attrs.names;
        if struct_attrs.first_char {
            if let Some(c) = key.chars().next() {
                if names.is_empty() {
                    names.insert(0, c.to_string());
                };
            }
        }
        if !names.contains(&key) {
            names.push(key.replace("_", "-"));
        }
        names = names
            .iter()
            .filter(|part| !part.is_empty())
            .map(|v| v.clone())
            .collect::<Vec<_>>();

        let kind_ident = Ident::new(&field_type.kind.to_string(), Span::call_site());
        let multiple_ident = format_ident!("{}", &field_type.multiple);
        tokens.extend(quote! {
            #parser_ident.add_by_kind(
                #key_token,
                [ #(#names),* ].to_vec(),
                rexl::argparse::ArgumentKind::#kind_ident,
                #multiple_ident
            );
        });
    }
    Ok(tokens)
}

fn argparse_arg_assign(
    fields: &StructFields, parser_ident: &Ident,
) -> syn::Result<Vec<TokenStream>> {
    let mut tokens = vec![];
    for field in fields.iter() {
        let key = get_field_name(field)?;
        let field_attrs = ArgParserFieldAttrs::try_from(&field.attrs)?;
        let field_type = ArgParserType::parse(field, &field_attrs)?;

        let key_ident = format_ident!("{}", key);
        // position args
        if field_attrs.is_position() {
            let position_tokens =
                argparse_arg_assign_position(&field_type, &field_attrs, &key_ident, parser_ident)?;
            if let Some(t) = position_tokens {
                tokens.push(t);
            }
            continue;
        }

        let kind = field_type.kind;
        let key_str_token = quote!(#key);
        // properties
        if kind == ArgumentKind::Property {
            tokens.push(quote! {
                #key_ident: #parser_ident.get_properties(#key_str_token).as_ref().clone()
            });
            continue;
        }
        // multiple
        if field_type.multiple {
            let multiple_tokens = argparse_arg_assign_multiple(
                &field_type,
                &field_attrs,
                &key_ident,
                &key_str_token,
                parser_ident,
            )?;
            if let Some(t) = multiple_tokens {
                tokens.push(t);
            }
            continue;
        }

        // value
        let Some(value_type) = field_type.value_type else {
            return Ok(tokens);
        };
        let value_type_ident = quote!(#value_type);

        if kind == ArgumentKind::Bool {
            if field_type.optional {
                tokens.push(quote! {
                    #key_ident: #parser_ident.get_bool_option(#key_str_token)
                });
            } else {
                tokens.push(quote! {
                    #key_ident: #parser_ident.get_bool(#key_str_token)
                });
            }
        } else if kind == ArgumentKind::String {
            if field_attrs.from_str {
                if field_type.optional {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_from_str(
                            #key_str_token,
                            |v|<#value_type_ident as std::str::FromStr>::from_str(v)
                        )?
                    });
                } else {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_from_str_or_err(
                            #key_str_token,
                            |v|<#value_type_ident as std::str::FromStr>::from_str(v),
                        )?
                    });
                }
            } else {
                if field_type.optional {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_string(#key_str_token)
                    });
                } else {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_string_or_default(#key_str_token)
                    });
                }
            }
        }

        if kind == ArgumentKind::Integer {
            let no_cast = value_type_ident.to_string() == "i64";
            if field_type.optional {
                if no_cast {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_integer(#key_str_token)
                    });
                } else {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_integer(#key_str_token)
                            .map(|v| v as #value_type_ident)
                    });
                }
            } else {
                if no_cast {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_integer_or_default(#key_str_token)
                    });
                } else {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_integer_or_default(#key_str_token)
                            as #value_type_ident
                    });
                }
            }
        } else if kind == ArgumentKind::Float {
            let no_cast = value_type_ident.to_string() == "f64";
            if field_type.optional {
                if no_cast {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_float(#key_str_token)
                    });
                } else {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_float(#key_str_token)
                            .map(|v| v as #value_type_ident)
                    });
                }
            } else {
                if no_cast {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_float_or_default(#key_str_token)
                    });
                } else {
                    tokens.push(quote! {
                        #key_ident: #parser_ident.get_float_or_default(#key_str_token)
                            as #value_type_ident
                    });
                }
            }
        }
    }
    Ok(tokens)
}

fn argparse_arg_assign_multiple(
    field_type: &ArgParserType, field_attrs: &ArgParserFieldAttrs, key_ident: &Ident,
    key_str_token: &TokenStream, parser_ident: &Ident,
) -> syn::Result<Option<TokenStream>> {
    let Some(value_type) = &field_type.value_type else {
        return Ok(None);
    };
    let value_type_ident = quote!(#value_type);

    let kind = field_type.kind;
    if kind == ArgumentKind::String {
        if field_attrs.from_str {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_multiple_from_str(
                    #key_str_token,
                    |v|<#value_type_ident as std::str::FromStr>::from_str(v),
                )?
            }));
        } else {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_strings(#key_str_token).as_ref().clone()
            }));
        }
    }

    if kind == ArgumentKind::Integer {
        let no_cast = value_type_ident.to_string() == "i64";
        if no_cast {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_integers(#key_str_token)
                    .as_ref().clone()
            }));
        } else {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_integers(#key_str_token).as_ref().clone()
                    .iter().map(|v| v.clone() as #value_type_ident).collect::<Vec<_>>()
            }));
        }
    } else if kind == ArgumentKind::Float {
        let no_cast = value_type_ident.to_string() == "f64";
        if no_cast {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_floats(#key_str_token).as_ref().clone()
            }));
        } else {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_floats(#key_str_token).as_ref().clone()
                    .iter().map(|v| v.clone() as #value_type_ident).collect::<Vec<_>>()
            }));
        }
    }
    Ok(None)
}

fn argparse_position_type_check(
    field_type: &ArgParserType, field_attrs: &ArgParserFieldAttrs, field: &Field,
) -> syn::Result<()> {
    if field_attrs.positions {
        if !field_type.multiple || field_type.optional {
            return Err(syn::Error::new_spanned(
                field,
                "the type of positions-arg must be a Vec<T>",
            ));
        }
        if field_type.kind == ArgumentKind::Bool {
            return Err(syn::Error::new_spanned(
                field,
                "the inner type of positions-arg can only be a string|integer|float",
            ));
        }
    } else {
        if field_type.multiple || field_type.kind == ArgumentKind::Bool {
            return Err(syn::Error::new_spanned(
                field,
                "the type of position-arg can only be a string|integer|float",
            ));
        }
    }
    Ok(())
}

fn argparse_arg_assign_position(
    field_type: &ArgParserType, field_attrs: &ArgParserFieldAttrs, key_ident: &Ident,
    parser_ident: &Ident,
) -> syn::Result<Option<TokenStream>> {
    if field_attrs.positions {
        return Ok(Some(quote! {
            #key_ident: #parser_ident.get_position_values().clone()
        }));
    }

    let kind = field_type.kind;
    let Some(position) = field_attrs.position else {
        return Ok(None);
    };

    let index = Index::from(position as usize);
    if kind == ArgumentKind::String {
        if field_type.optional {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_position_values().get(#index)
                    .map(|v| v.clone())
            }));
        } else {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_position_values().get(#index)
                    .map_or_else(|| String::default(), |v| v.clone())
            }));
        }
    } else if kind == ArgumentKind::Integer || kind == ArgumentKind::Float {
        let Some(value_type) = &field_type.value_type else {
            return Ok(None);
        };
        let value_type_ident = quote!(#value_type);
        if field_type.optional {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_position_values().get(#index)
                    .and_then(|v|v.parse::<#value_type_ident>().ok())
            }));
        } else {
            return Ok(Some(quote! {
                #key_ident: #parser_ident.get_position_values().get(#index)
                    .and_then(|v|v.parse::<#value_type_ident>().ok()).unwrap_or_default()
            }));
        }
    }
    Ok(None)
}
