use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use crate::util::*;

pub(crate) struct ServerArguments {
    //default = None
    pub(crate) config: Option<String>,
    //default = 5000
    pub(crate) port: u16,
    //default = "127.0.0.1"
    pub(crate) hostname: String,
    //default = 3
    pub(crate) threads: usize
}

const DEFAULT_PORT:u16 = 5000;
const DEFAULT_HOSTNAME:&'static str = "127.0.0.1";
const DEFAULT_THREADS:usize = 3;

impl Parse for ServerArguments {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        let map = InputParser::new(input)?;

        let config = map.get_string("config")
            .ok();
        let port = map.get_u16("port")
            .unwrap_or(DEFAULT_PORT);
        let hostname = map.get_string("hostname")
            .unwrap_or(String::from(DEFAULT_HOSTNAME));
        let threads = map.get_usize("threads")
            .unwrap_or(DEFAULT_THREADS);

        Ok(
            Self { config, port, hostname, threads }
        )
    }
}

pub(crate) fn parse_server_arguments(input:TokenStream) -> ServerArguments {
    if input.is_empty() {
        ServerArguments {
            config: None,
            port: DEFAULT_PORT,
            hostname: DEFAULT_HOSTNAME.to_string(),
            threads: DEFAULT_THREADS
        }
    } else {
        syn::parse::<ServerArguments>(input).unwrap()
    }
}

pub(crate) struct ServerAttributes {
    pub(crate) public: bool,
    pub(crate) name: syn::Ident,
    pub(crate) routers: Vec<syn::Ident>,
    pub(crate) err_handler: Option<syn::Ident>
}

impl Parse for ServerAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let public = if input.peek(syn::token::Pub) {
            input.parse::<syn::token::Pub>()?;();
            true
        } else {
            false
        };

        input.parse::<syn::token::Struct>()?;

        let name: syn::Ident = input.parse()?;
        let mut routers = Vec::new();
        let mut err_handler = None;

        let fields;
        syn::parenthesized!(fields in input);
        input.parse::<syn::Token![;]>()?;

        while !fields.peek(syn::parse::End) {

            let name = fields.parse::<syn::Ident>()?;

            if is_snake_case(&name) {
                err_handler = Some(name);
                break;
            } else {
                routers.push(name);
            }

            if !fields.peek(syn::parse::End) {
                fields.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self{
            public,
            name,
            routers,
            err_handler
        })
    }
}

pub(crate) fn parse_server_attributes(input:TokenStream) -> ServerAttributes {
    syn::parse::<ServerAttributes>(input).unwrap()
}