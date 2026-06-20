use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) fn run_with_args_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let root = parse_macro_input!(input as CommandNode);

    let expanded = expand_run_with_args_tree(&root);

    proc_macro::TokenStream::from(expanded)
}

fn expand_run_with_args_tree(node: &CommandNode) -> TokenStream {
    match node {
        CommandNode::Leaf(type_name) => {
            quote! {
                impl rexl::argparse::RunWithArgs for #type_name {
                    fn run_with_args(args: Vec<String>) -> Result<(), rexl::argparse::ArgParserError> {
                        let mut v = <#type_name as rexl::argparse::FromArgs>::from_args(args)?;
                        <#type_name as rexl::argparse::ArgParserRunnable>::run(v);
                        Ok(())
                    }
                }
            }
        }
        CommandNode::Node { type_name, subcommands } => {
            let child_impls = subcommands
                .iter()
                .map(|(_, child)| expand_run_with_args_tree(child));

            let match_arms = subcommands.iter().map(|(cmd, child)| {
                let child_type = extract_type_name(child);
                let cmd_parts: Vec<&str> = cmd.split(',').collect();

                quote! {
                    #( #cmd_parts )|* => {
                        <#child_type as rexl::argparse::RunWithArgs>::run_with_args(remaining_args)
                    }
                }
            });

            quote! {
                #( #child_impls )*

                impl rexl::argparse::RunWithArgs for #type_name {
                    fn run_with_args(args: Vec<String>) -> Result<(), rexl::argparse::ArgParserError> {
                        if args.is_empty() {
                            let mut v = <#type_name as rexl::argparse::FromArgs>::from_args(args)?;
                            <#type_name as rexl::argparse::ArgParserRunnable>::run(v);
                            return Ok(());
                        }

                        let cmd = args[0].as_str();
                        let remaining_args = args[1..].to_vec();
                        match cmd {
                            #( #match_arms, )*
                            _ => {
                                let mut v = <#type_name as rexl::argparse::FromArgs>::from_args(args)?;
                                <#type_name as rexl::argparse::ArgParserRunnable>::run(v);
                                Ok(())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn extract_type_name(node: &CommandNode) -> &Ident {
    match node {
        CommandNode::Leaf(type_name) => type_name,
        CommandNode::Node { type_name, .. } => type_name,
    }
}

enum CommandNode {
    Leaf(Ident),
    Node { type_name: Ident, subcommands: Vec<(String, CommandNode)> },
}

impl Parse for CommandNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_name: Ident = input.parse()?;

        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);

            let subcommands: Punctuated<_, Comma> =
                content.parse_terminated(Subcommand::parse, Comma)?;

            Ok(CommandNode::Node {
                type_name,
                subcommands: subcommands
                    .into_iter()
                    .map(|s| (s.cmd, s.node))
                    .collect(),
            })
        } else {
            Ok(CommandNode::Leaf(type_name))
        }
    }
}

// cmd => node
struct Subcommand {
    cmd: String,
    node: CommandNode,
}

impl Parse for Subcommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let cmd_lit: LitStr = input.parse()?;
        let cmd = cmd_lit.value();

        input.parse::<Token![=>]>()?;

        let node = input.parse()?;

        Ok(Subcommand { cmd, node })
    }
}
