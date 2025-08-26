use proc_macro2::{Span, TokenStream};
use syn::parse::{Parse, ParseStream};
use quote::quote;

pub(crate) enum ServerArguments {
    CmdLineArgs,
    ConfigFile(String),
    // defualt(port, hostname),
    // hostname = "127.0.0.1"
    HardCode(u16, String)
}

impl Parse for ServerArguments {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitInt) {
            let port: syn::LitInt = input.parse()?;
            let hostname = if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
                input.parse::<syn::LitStr>()?.value()
            } else {
                String::from("127.0.0.1")
            };

            Ok (
                Self::HardCode(port.base10_parse()?, hostname)
            )
        } else if input.peek(syn::LitStr) {
            let filename: syn::LitStr = input.parse()?;
            Ok (
                Self::ConfigFile(filename.value())
            )
        } else {
            input.parse::<syn::parse::Nothing>()?;
            Ok (
                Self::CmdLineArgs
            )
        }
    }
}

pub(crate) struct ServerAttributes {
    public: bool,
    name: syn::Ident,
    routers: Vec<syn::Ident>,
    err_handler: Option<syn::ItemFn>
}

impl Parse for ServerAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let public = if input.peek(syn::token::Pub) {
            input.parse::<syn::token::Pub>()?;
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
            if fields.peek(syn::Ident) {
                routers.push(fields.parse()?)
            } else if fields.peek(syn::token::Fn) {
                err_handler = Some(fields.parse()?);
                break;
            } else {
                return Err(syn::Error::new(
                    fields.span(),
                    "Expected either a router identifier or error handler function!"
                ));
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

fn build_new_function(args:&ServerArguments, routers:&Vec<syn::Ident>) -> (TokenStream, Vec<syn::Ident>) {
    let mut names = quote!();
    let mut list = Vec::new();
    let arguments = match args {
        ServerArguments::CmdLineArgs => quote!{
            let args = http::server::get_arguments();
            let port = args.0.unwrap();
            let hostname = args.1.unwrap();
        },
        ServerArguments::ConfigFile(filename) => quote! {
            let (port, hostname) = http::server::load_settings(#filename);
        },
        ServerArguments::HardCode(port, hostname) => quote! {
            let port:u16 = #port;
            let hostname = String::from(#hostname);
        }
    };

    for name in routers {
        let struct_name = snake_case(name);
        list.push(struct_name.clone());

        names.extend(quote!{#struct_name: #name::new(),});
    }

    (
        quote!{
            pub fn new() -> Self {
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

fn build_error_handler(func:Option<syn::ItemFn>) -> TokenStream {
    match func {
        Some(func) => {
            let error_args = func.sig.inputs;
            let error_block = func.block;
            let error_return = func.sig.output;

            quote! {
                pub fn error_handler(&self, #error_args) #error_return {
                    #error_block
                }
            }
        },
        None => quote!{
            pub fn error_handler<'body>(&self, _: http::ErrorRequest<'body>, err: http::HttpError) -> http::Response {
                http::Response::from_error(err)
            }
        }
    }
}

fn build_handler(routers:&Vec<syn::Ident>) -> TokenStream {
    let mut handle_router = quote!();

    for r in routers {
        handle_router.extend(quote!{
            if let Some(resp) = self.#r.handle(req_builder)? {
                return Ok(resp)
            }
        });
    }

    quote!{
        pub fn handle<'body, 'a>(&'body self, req_builder: &'a mut http::RequestBuilder<'body>) -> Result<http::Response, http::HttpError> {
            use http::Router;
            #handle_router

            Err(http::HttpErrorKind::NotFound.into())
        }
    }
}

pub fn build_server(args:ServerArguments, att:ServerAttributes) -> TokenStream {
    let (new_func, names) = build_new_function(&args, &att.routers);
    let mut att_names = quote!();
    for i in 0..names.len() {
        let name = &names[i];
        let ident = &att.routers[i];
        att_names.extend(quote!{
            #name: #ident,
        });
    }

    let struct_start = if att.public {
        quote!{pub struct}
    } else {
        quote!{struct}
    };

    let struct_name = att.name;
    let handler = build_handler(&names);
    let error_handler = build_error_handler(att.err_handler);

    quote!{
        #struct_start #struct_name {
            port: u16,
            hostname: String,
            #att_names
        }

        impl #struct_name {
            #new_func



            pub fn hostname(&self) -> &str {
                &self.hostname
            }

            pub fn port(&self) -> u16 {
                self.port
            }
        }
    }
}