use core::fmt;

use proc_macro2::{TokenStream, Span};
use syn::parse::{Parse, ParseStream, Result};
use quote::quote;
use crate::util::*;

pub(crate) struct Path {
    pub(crate) pattern: String,
    //default = true
    pub(crate) end:bool,
    //defailt = false
    pub(crate) trailing: bool,
    //default = true
    pub(crate) insensitive: bool
}

pub(crate) struct RouterAttributes {
    pub(crate) path: Path,
    //default = "All"
    pub(crate) methods:String
    
}

impl Parse for RouterAttributes {
    fn parse(input:ParseStream) -> Result<Self> {
        let map = InputParser::new(input)?;

        let pattern = match map.get_string("path") {
            Ok(str) => str,
            Err(_) => match map.get_string("path_pattern") {
                Ok(str) => str,
                Err(_) => panic!("This is bad!")
            }
        };

        let end = map.get_bool("path_end").unwrap_or(true);
        let trailing = map.get_bool("path_trailing").unwrap_or(false);
        let insensitive = map.get_bool("path_insensitive").unwrap_or(true);
        let methods = map.get_string("methods").unwrap_or(String::from("ALL"));

        return Ok(Self{
            path: Path {
                pattern,
                end,
                trailing,
                insensitive
            },
            methods
        })
    }
}

impl Path {
    pub fn build_pattern<Name:fmt::Display>(&self, name:Name) -> (syn::Ident, TokenStream, TokenStream) {
        let pattern_name = syn::Ident::new(
            &format!("{}Pattern", name),
            Span::call_site()
        );

        let param_name = syn::Ident::new(
            &format!("{}PathParam", name),
            Span::call_site()
        );

        let handler_name = syn::Ident::new(
            &format!("{}_handler", snake_case(name)),
            Span::call_site()
        );

        let i = &self.insensitive;

        let (regex, keys) = crate::path::compile(
            &self.pattern,
            self.trailing,
            self.end
        );

        let (path_struct, path_size) = crate::path::build_path_types(
            &param_name,
            &keys
        );

        (
            handler_name.clone(),
            quote! {
                static #pattern_name:std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(||{
                    regex::RegexBuilder::new(#regex)
                        .case_insensitive(#i).build().unwrap()
                });

                #path_struct
            },
            quote! {
            if let Some(caps) = #pattern_name.captures(&req.url.pathname()) {
                let (_, list) = caps.extract() as (&str, [&str; #path_size]);
                let param = #param_name::new(list);

                return #handler_name(req.build(param)).await.map(|rsp|Some(rsp));
            }
        }
        )
    }
}