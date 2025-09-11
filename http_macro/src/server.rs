use proc_macro2::{Span, TokenStream};
use syn::parse::{Parse, ParseStream};
use quote::quote;

pub(crate) struct ServerArguments {
    //default = None
    config: Option<String>,
    //default = 5000
    port: u16,
    //default = "127.0.0.1"
    hostname: String,
}

impl Parse for ServerArguments {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        let map = super::util::InputParser::new(input)?;

        let config = map.get_string("config").ok();
        let port = map.get_u16("port").unwrap_or(5000);
        let hostname = map.get_string("hostname").unwrap_or(String::from("127.0.0.1"));

        Ok(
            Self { config, port, hostname }
        )
    }
}

pub(crate) struct ServerAttributes {
    public: bool,
    name: syn::Ident,
    routers: Vec<syn::Ident>,
    err_handler: Option<syn::Ident>
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

fn snake_case(name:&syn::Ident) -> syn::Ident {
    let string = name.to_string().chars().flat_map(|c|{
        if c.is_ascii_uppercase() {
            vec!['_', c.to_ascii_lowercase()]
        } else {
            vec![c]
        }
    }).collect::<String>();
    syn::Ident::new(
        if let Some(index) = string.find("_") && index == 0 {
            &string[1..]
        } else {
            &string
        },
        Span::call_site(),
    )
}

fn is_snake_case(name:&syn::Ident) -> bool {
    for c in name.to_string().chars() {
        if c.is_uppercase() {
            return false;
        }
    }

    return true;
}

fn build_new_function(args:&ServerArguments, routers:&Vec<syn::Ident>) -> (TokenStream, Vec<syn::Ident>) {
    let mut names = quote!();
    let mut list = Vec::new();
    let mut arguments = quote!{
        let (mut port, mut hostname, config_file) = http::get_arguments();
    };

    if args.config.is_some() {
        let file_name = args.config.as_ref().unwrap();
        arguments.extend(quote!{
            let config_file = config_file.or(Some(String::from(#file_name)));
        });
    }

    let port = args.port;
    let hostname = &args.hostname;
    arguments.extend(quote!{
        if config_file.is_some() {
            let (config_port, config_hostname) = http::load_settings(&config_file.unwrap());
            port = port.or(config_port);
            hostname = hostname.or(config_hostname); 
        }

        let port = port.unwrap_or(#port);
        let hostname = hostname.unwrap_or(
            String::from(#hostname)
        );
    });

    for name in routers {
        let struct_name = snake_case(name);
        list.push(struct_name.clone());

        names.extend(quote!{#struct_name: #name::new(),});
    }

    (
        quote!{
            fn new() -> Self {
                #arguments
                Self {
                    port, hostname,
                    #names
                }
            }
        },
        list
    )
}

fn build_error_handler(func:&Option<syn::Ident>) -> TokenStream {
    let block = match func {
        Some(func) => quote!(#func(req).await),
        None => quote!(http::types::Response::from_error(req.param))
    };

    quote!{
        async fn error_handler(&self, req:http::types::ErrorRequest) -> http::types::Response {
            #block
        }
    }
}

fn build_handler(routers:&Vec<syn::Ident>) -> TokenStream {
    let mut handle_router = quote!();

    for r in routers {
        handle_router.extend(quote!{
            if let Some(resp) = self.#r.handle(builder).await? {
                return Ok(resp)
            }
        });
    }

    quote!{
        async fn handle(&self, builder:&mut http::builder::RequestBuilder<http::async_net::TcpStream>) -> http::Result {
            use http::Router;
            #handle_router

            Err(http::HttpErrorKind::NotFound.into())
        }
    }
}

fn build_server_layers(args:&ServerArguments, att:&ServerAttributes) -> (TokenStream, syn::Ident, syn::Ident) {
    let (new_func, names) = build_new_function(&args, &att.routers);
    let mut att_names = quote!();
    for i in 0..names.len() {
        let name = &names[i];
        let ident = &att.routers[i];
        att_names.extend(quote!{
            #name: #ident,
        });
    }

    let struct_name = &att.name;
    let layers_name = syn::Ident::new(
        &(struct_name.to_string() + "Parts"),
        Span::call_site()
    );

    let handler = build_handler(&names);
    let error_handler = build_error_handler(&att.err_handler);

    (quote!{
        struct #layers_name {
            port: u16,
            hostname: String,
            #att_names
        }

        impl #layers_name {
            #handler

            #error_handler
        }

        impl http::builder::ServerParts for #layers_name {
            #new_func

            fn hostname(&self) -> &str {
                &self.hostname
            }

            fn port(&self) -> &u16 {
                &self.port
            }

            async fn handle_request(&self, req:&mut http::builder::RequestBuilder<http::async_net::TcpStream>) -> http::types::Response {
                match self.handle(req).await {
                    Ok(resp) => resp,
                    Err(e) => self.error_handler(req.error(e)).await
                }
            }
        }
    },
    struct_name.clone(),
    layers_name
    )
}

pub fn build_server(args:ServerArguments, att:ServerAttributes) -> TokenStream {
    let (parts, name, parts_name) = build_server_layers(&args, &att);

    let struct_start = if att.public {
        quote!{pub type}
    } else {
        quote!{type}
    };

    quote!{
        #parts
        #struct_start #name = http::builder::Server<#parts_name>;
    }
    
}